//! Prova HF-00: atalho global + overlay perto do cursor + recebimento de caminhos de um
//! arraste externo. Não converte nada. Cada evento relevante vai para um log JSONL local
//! e para a janela de diagnóstico, para o testador copiar como evidência.
#![cfg_attr(all(not(debug_assertions), windows), windows_subsystem = "windows")]

mod platform;

use hf_poc_core::geometry::{place_overlay, Monitor, Rect};
use hf_poc_core::registry::{Kind, Registry};
use hf_poc_core::session::{Action, Event, Session, ShortcutMode, State};
use serde::Serialize;
use serde_json::{json, Value};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;
use tauri::{
    DragDropEvent, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

const OVERLAY: &str = "overlay";
const OVERLAY_LOGICAL: (u32, u32) = (320, 180);
const OVERLAY_OFFSET: (i32, i32) = (16, 16);
const DEFAULT_SHORTCUT: &str = "CommandOrControl+Alt+Shift+D";
/// Rede de segurança: o overlay não fica preso se nada acontecer.
const OVERLAY_TIMEOUT_SECS: u64 = 15;

#[derive(Serialize, Clone)]
struct LogEntry {
    t_ms: u128,
    kind: String,
    detail: Value,
}

struct Config {
    shortcut: Shortcut,
    shortcut_text: String,
    mode: ShortcutMode,
    escape: EscapeStrategy,
    focusable: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EscapeStrategy {
    /// Registra Escape como atalho global só enquanto o overlay está visível.
    Global,
    /// Escape só chega pela webview do overlay (exige foco nela).
    Webview,
}

struct Shared {
    session: Session,
    registry: Registry,
    generation: u64,
    log: Vec<LogEntry>,
    log_file: Option<PathBuf>,
    t0: Instant,
    shortcut_t: Option<Instant>,
    /// Escape global só é desregistrado se o registro deu certo.
    escape_registered: bool,
}

type State_ = Mutex<Shared>;

impl Shared {
    fn record(&mut self, app: &tauri::AppHandle, kind: &str, detail: Value) {
        let e = LogEntry {
            t_ms: self.t0.elapsed().as_millis(),
            kind: kind.into(),
            detail,
        };
        if let Some(p) = &self.log_file {
            if let Ok(mut f) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(p)
            {
                let _ = writeln!(f, "{}", serde_json::to_string(&e).unwrap_or_default());
            }
        }
        let _ = app.emit_to("main", "poc:event", &e);
        self.log.push(e);
    }
}

/// Registra um evento se o estado compartilhado já existe (o handler do atalho pode disparar cedo);
/// sem estado, cai no stderr para não perder a pista.
fn rec(app: &tauri::AppHandle, kind: &str, detail: Value) {
    match app.try_state::<State_>() {
        Some(st) => st.lock().unwrap().record(app, kind, detail),
        None => eprintln!("[poc] {kind}: {detail}"),
    }
}

/// Operações de janela do Tauri devolvem `Result`; ignorá-las esconde falhas do gerenciador de janelas.
fn win_op(app: &tauri::AppHandle, op: &str, r: tauri::Result<()>) {
    if let Err(e) = r {
        rec(
            app,
            "window_error",
            json!({"op": op, "error": e.to_string()}),
        );
    }
}

fn parse_config() -> Result<Config, String> {
    let text = std::env::var("HF_POC_SHORTCUT").unwrap_or_else(|_| DEFAULT_SHORTCUT.into());
    let shortcut: Shortcut = text
        .parse()
        .map_err(|e| format!("HF_POC_SHORTCUT inválido ({text}): {e}"))?;
    let mode = match std::env::var("HF_POC_MODE").as_deref() {
        Ok("hold") => ShortcutMode::HoldToShow,
        Ok("toggle") | Err(_) => ShortcutMode::Toggle,
        Ok(o) => return Err(format!("HF_POC_MODE inválido: {o} (use toggle|hold)")),
    };
    let escape = match std::env::var("HF_POC_ESCAPE").as_deref() {
        Ok("webview") => EscapeStrategy::Webview,
        Ok("global") | Err(_) => EscapeStrategy::Global,
        Ok(o) => return Err(format!("HF_POC_ESCAPE inválido: {o} (use global|webview)")),
    };
    let focusable = std::env::var("HF_POC_FOCUSABLE").as_deref() != Ok("0");
    Ok(Config {
        shortcut,
        shortcut_text: text,
        mode,
        escape,
        focusable,
    })
}

fn monitors_core(w: &WebviewWindow) -> (Vec<Monitor>, Value, Option<String>) {
    let (ms, err) = match w.available_monitors() {
        Ok(ms) => (ms, None),
        Err(e) => (Vec::new(), Some(e.to_string())),
    };
    let core: Vec<Monitor> = ms
        .iter()
        .map(|m| {
            let wa = m.work_area();
            Monitor {
                work_area: Rect {
                    x: wa.position.x,
                    y: wa.position.y,
                    w: wa.size.width,
                    h: wa.size.height,
                },
                scale_factor: m.scale_factor(),
            }
        })
        .collect();
    let desc = json!(ms
        .iter()
        .map(|m| {
            let wa = m.work_area();
            json!({
            "position": [m.position().x, m.position().y], "size": [m.size().width, m.size().height],
            "work_area": [wa.position.x, wa.position.y, wa.size.width, wa.size.height],
            "scale": m.scale_factor() })
        })
        .collect::<Vec<_>>());
    (core, desc, err)
}

/// Executa as ações da máquina de estados. Chamado com o lock já liberado quando possível.
fn run_actions(app: &tauri::AppHandle, actions: Vec<Action>, dropped: Option<Vec<PathBuf>>) {
    for a in actions {
        match a {
            // Janelas e NSWindow só na thread principal; o handler de atalho pode rodar fora dela.
            Action::ShowOverlay => {
                let a = app.clone();
                if let Err(e) = app.run_on_main_thread(move || show_overlay(&a)) {
                    rec(
                        app,
                        "window_error",
                        json!({"op": "run_on_main_thread(show_overlay)", "error": e.to_string()}),
                    );
                }
            }
            Action::HideOverlay => {
                let a = app.clone();
                if let Err(e) = app.run_on_main_thread(move || hide_overlay(&a)) {
                    rec(
                        app,
                        "window_error",
                        json!({"op": "run_on_main_thread(hide_overlay)", "error": e.to_string()}),
                    );
                }
            }
            Action::AcceptDrop => {
                let paths = dropped.clone().unwrap_or_default();
                let state = app.state::<State_>();
                let mut s = state.lock().unwrap();
                let out = s.registry.add(paths);
                let views: Vec<Value> = out.accepted.iter().map(|v| json!({
                    "id": v.id, "name": v.display_name, "dir": v.kind == Kind::Directory, "symlink": v.was_symlink })).collect();
                let rej: Vec<Value> = out
                    .rejected
                    .iter()
                    .map(|r| json!({"name": r.display_name, "reason": format!("{:?}", r.reason)}))
                    .collect();
                s.record(
                    app,
                    "drop_accepted",
                    json!({"accepted": views, "rejected": rej, "duplicates": out.duplicates}),
                );
            }
            Action::Log(msg) => {
                let state = app.state::<State_>();
                state
                    .lock()
                    .unwrap()
                    .record(app, "session", json!({ "msg": msg }));
            }
        }
    }
}

fn show_overlay(app: &tauri::AppHandle) {
    let Some(w) = app.get_webview_window(OVERLAY) else {
        rec(
            app,
            "error",
            json!({"msg": "janela do overlay não encontrada"}),
        );
        return;
    };
    let cfg = app.state::<Config>();
    let started = app.state::<State_>().lock().unwrap().shortcut_t;
    let cursor_res = w.cursor_position();
    let cursor_err = cursor_res.as_ref().err().map(|e| e.to_string());
    let cursor = cursor_res.ok();
    let (mons, mons_desc, mons_err) = monitors_core(&w);
    let placed = cursor.and_then(|c| {
        place_overlay(
            (c.x.round() as i32, c.y.round() as i32),
            OVERLAY_LOGICAL,
            OVERLAY_OFFSET,
            &mons,
        )
    });
    let Some(r) = placed else {
        app.state::<State_>().lock().unwrap().record(app, "error", json!({"msg": "sem cursor/monitores; overlay não posicionado", "cursor": cursor.map(|c| [c.x, c.y]),
            "cursor_error": cursor_err, "monitors_error": mons_err, "monitors_found": mons.len()}));
        return;
    };
    win_op(app, "set_size", w.set_size(PhysicalSize::new(r.w, r.h)));
    win_op(
        app,
        "set_position",
        w.set_position(PhysicalPosition::new(r.x, r.y)),
    );
    win_op(app, "show", w.show());
    win_op(app, "set_always_on_top", w.set_always_on_top(true));
    // Nunca pedir foco: roubar o foco da origem pode encerrar o arraste (hipótese a medir).
    let ms = started.map(|t| t.elapsed().as_secs_f64() * 1000.0);
    if cfg.escape == EscapeStrategy::Global {
        match app.global_shortcut().register("Escape") {
            Ok(()) => app.state::<State_>().lock().unwrap().escape_registered = true,
            Err(e) => rec(
                app,
                "hotkey_error",
                json!({"op": "register(Escape)", "error": e.to_string()}),
            ),
        }
    }
    // O que o gerenciador de janelas realmente fez (pode ignorar posição/tamanho, ex.: Wayland).
    // Lido 300 ms depois: logo após show() o servidor gráfico ainda não mapeou a janela (valores enganosos).
    let probe_app = app.clone();
    let expected = [r.x, r.y, r.w as i32, r.h as i32];
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));
        let Some(w) = probe_app.get_webview_window(OVERLAY) else {
            return;
        };
        rec(
            &probe_app,
            "overlay_probe",
            json!({
                "expected_rect": expected,
                "visible": w.is_visible().map_err(|e| e.to_string()).ok(),
                "outer_position": w.outer_position().ok().map(|p| [p.x, p.y]),
                "outer_size": w.outer_size().ok().map(|s| [s.width, s.height]),
            }),
        );
    });
    let gen = {
        let state = app.state::<State_>();
        let mut s = state.lock().unwrap();
        s.generation += 1;
        s.record(
            app,
            "overlay_shown",
            json!({
            "cursor": cursor.map(|c| [c.x, c.y]), "rect": [r.x, r.y, r.w, r.h],
            "monitors": mons_desc, "monitors_error": mons_err, "cursor_error": cursor_err,
            "shortcut_to_shown_ms": ms, "focusable": cfg.focusable }),
        );
        s.generation
    };
    // Timeout vinculado à geração, para não esconder um overlay posterior.
    let app2 = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(OVERLAY_TIMEOUT_SECS));
        let actions = {
            let state = app2.state::<State_>();
            let mut s = state.lock().unwrap();
            if s.generation != gen || s.session.state == State::Idle {
                return;
            }
            s.session.handle(Event::Timeout)
        };
        run_actions(&app2, actions, None);
    });
}

fn hide_overlay(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window(OVERLAY) {
        win_op(app, "hide", w.hide());
    }
    let was_registered =
        std::mem::take(&mut app.state::<State_>().lock().unwrap().escape_registered);
    if was_registered {
        if let Err(e) = app.global_shortcut().unregister("Escape") {
            rec(
                app,
                "hotkey_error",
                json!({"op": "unregister(Escape)", "error": e.to_string()}),
            );
        }
    }
    app.state::<State_>()
        .lock()
        .unwrap()
        .record(app, "overlay_hidden", json!({}));
}

fn dispatch(app: &tauri::AppHandle, ev: Event, dropped: Option<Vec<PathBuf>>) {
    let actions = {
        let state = app.state::<State_>();
        let mut s = state.lock().unwrap();
        let before = s.session.state.clone();
        if matches!(ev, Event::ShortcutPressed) {
            s.shortcut_t = Some(Instant::now());
        }
        let a = s.session.handle(ev.clone());
        let after = s.session.state.clone();
        if before != after || !a.is_empty() {
            s.record(app, "transition", json!({"event": format!("{ev:?}"), "from": format!("{before:?}"), "to": format!("{after:?}")}));
        }
        a
    };
    run_actions(app, actions, dropped);
}

#[tauri::command]
fn get_report(
    webview: tauri::Webview,
    state: tauri::State<State_>,
    cfg: tauri::State<Config>,
) -> Result<Value, String> {
    if webview.label() != "main" {
        return Err("negado".into());
    }
    let s = state.lock().unwrap();
    Ok(json!({
        "poc": "HF-00 native-interaction", "tauri": tauri::VERSION,
        "env": platform::env_info(),
        "config": {"shortcut": cfg.shortcut_text, "mode": format!("{:?}", cfg.mode),
                   "escape": format!("{:?}", cfg.escape), "focusable": cfg.focusable},
        "log": s.log, "log_file": s.log_file.as_ref().map(|p| p.display().to_string()),
    }))
}

#[tauri::command]
fn overlay_cancel(webview: tauri::Webview, app: tauri::AppHandle) -> Result<(), String> {
    if webview.label() != OVERLAY {
        return Err("negado".into());
    }
    dispatch(&app, Event::EscapePressed, None);
    Ok(())
}

/// Abre o overlay pelo MESMO caminho do atalho (máquina de estados -> run_on_main_thread -> show_overlay),
/// sem passar pelo teclado: separa "captura do atalho" de "exibição do painel".
/// `async` para rodar fora da thread principal, como o handler do atalho global.
#[tauri::command]
async fn test_overlay(webview: tauri::Webview, app: tauri::AppHandle) -> Result<(), String> {
    if webview.label() != "main" {
        return Err("negado".into());
    }
    rec(&app, "ui_test_overlay", json!({"action": "open"}));
    dispatch(&app, Event::ShortcutPressed, None);
    Ok(())
}

/// Fecha o overlay sem depender do teclado (útil quando a captura de teclas é justamente o problema).
#[tauri::command]
async fn test_overlay_close(webview: tauri::Webview, app: tauri::AppHandle) -> Result<(), String> {
    if webview.label() != "main" {
        return Err("negado".into());
    }
    rec(&app, "ui_test_overlay", json!({"action": "close"}));
    dispatch(&app, Event::EscapePressed, None);
    Ok(())
}

/// Teclas vistas PELA WEBVIEW da janela de diagnóstico (não pelo atalho global). Só combinações com
/// Ctrl/Alt/Meta e as próprias teclas modificadoras: texto digitado normalmente não é registrado.
#[tauri::command]
fn log_key(webview: tauri::Webview, app: tauri::AppHandle, key: Value) -> Result<(), String> {
    if webview.label() != "main" {
        return Err("negado".into());
    }
    rec(&app, "webview_key", key);
    Ok(())
}

fn main() {
    let cfg = match parse_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    let main_shortcut = cfg.shortcut;
    let mode = cfg.mode;
    let focusable = cfg.focusable;
    let escape_shortcut: Shortcut = "Escape".parse().expect("Escape válido");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, sc, ev| {
                    // Primeiro de tudo: prova de que o handler foi chamado, antes de qualquer comparação.
                    let state_txt = match ev.state() {
                        ShortcutState::Pressed => "Pressed",
                        ShortcutState::Released => "Released",
                    };
                    let th = std::thread::current();
                    rec(
                        app,
                        "hotkey_raw",
                        json!({
                            "event_id": ev.id(), "state": state_txt,
                            "shortcut": format!("{sc:?}"), "shortcut_id": sc.id(),
                            "is_main": *sc == main_shortcut, "is_escape": *sc == escape_shortcut,
                            "thread": th.name().unwrap_or("(sem nome)"),
                        }),
                    );
                    let ev_kind = if *sc == main_shortcut {
                        match ev.state() {
                            ShortcutState::Pressed => Event::ShortcutPressed,
                            ShortcutState::Released => Event::ShortcutReleased,
                        }
                    } else if *sc == escape_shortcut && ev.state() == ShortcutState::Pressed {
                        Event::EscapePressed
                    } else {
                        return;
                    };
                    dispatch(app, ev_kind, None);
                })
                .build(),
        )
        .manage(cfg)
        .setup(move |app| {
            let platform_notes = platform::configure_app(app);
            let log_file = app.path().app_local_data_dir().ok().map(|d| {
                let _ = std::fs::create_dir_all(&d);
                d.join("poc-log.jsonl")
            });
            app.manage(Mutex::new(Shared {
                session: Session::new(mode),
                registry: Registry::new(),
                generation: 0,
                log: vec![],
                log_file,
                t0: Instant::now(),
                shortcut_t: None,
                escape_registered: false,
            }));
            let overlay =
                WebviewWindowBuilder::new(app, OVERLAY, WebviewUrl::App("overlay.html".into()))
                    .title("HoodFiles PoC overlay")
                    .inner_size(OVERLAY_LOGICAL.0 as f64, OVERLAY_LOGICAL.1 as f64)
                    .decorations(false)
                    .resizable(false)
                    .skip_taskbar(true)
                    .always_on_top(true)
                    .visible(false)
                    .focused(false)
                    .focusable(focusable)
                    .visible_on_all_workspaces(true)
                    .build()?;
            let mut notes = platform_notes;
            notes.extend(platform::configure_overlay(&overlay));

            let handle = app.handle().clone();
            // WebviewWindow: o arraste chega como WindowEvent::DragDrop (on_webview_event NÃO dispara).
            overlay.on_window_event(move |ev| {
                let WindowEvent::DragDrop(d) = ev else { return };
                match d {
                    DragDropEvent::Enter { paths, .. } => {
                        // Evidência-chave: o SO entrega os caminhos já na entrada do arraste?
                        let names: Vec<String> = paths
                            .iter()
                            .map(|p| hf_poc_core::registry::display_name(p))
                            .collect();
                        handle.state::<State_>().lock().unwrap().record(
                            &handle,
                            "drag_enter",
                            json!({"count": paths.len(), "names": names}),
                        );
                        dispatch(&handle, Event::DragEntered { count: paths.len() }, None);
                    }
                    DragDropEvent::Over { position } => {
                        if std::env::var("HF_POC_TRACE").as_deref() == Ok("1") {
                            handle.state::<State_>().lock().unwrap().record(
                                &handle,
                                "drag_over",
                                json!({"pos": [position.x, position.y]}),
                            );
                        }
                    }
                    DragDropEvent::Drop { paths, .. } => dispatch(
                        &handle,
                        Event::Dropped { count: paths.len() },
                        Some(paths.clone()),
                    ),
                    DragDropEvent::Leave => {
                        handle.state::<State_>().lock().unwrap().record(
                            &handle,
                            "drag_leave",
                            json!({}),
                        );
                        dispatch(&handle, Event::DragLeft, None);
                    }
                    _ => {}
                }
            });

            // Controle: a janela de diagnóstico também registra arrastes. Serve para separar
            // "arraste externo não chega a nenhuma webview" de "o overlay específico falha".
            if let Some(main) = app.get_webview_window("main") {
                let h = app.handle().clone();
                main.on_window_event(move |ev| {
                    let WindowEvent::DragDrop(d) = ev else { return };
                    let (kind, n) = match d {
                        DragDropEvent::Enter { paths, .. } => ("main_drag_enter", paths.len()),
                        DragDropEvent::Drop { paths, .. } => ("main_drop", paths.len()),
                        DragDropEvent::Leave => ("main_drag_leave", 0),
                        _ => return,
                    };
                    h.state::<State_>()
                        .lock()
                        .unwrap()
                        .record(&h, kind, json!({"count": n}));
                });
            }

            let sc = app.state::<Config>().shortcut;
            let text = app.state::<Config>().shortcut_text.clone();
            let reg_t = Instant::now();
            let reg = app.global_shortcut().register(sc);
            let register_ms = reg_t.elapsed().as_secs_f64() * 1000.0;
            let state = app.state::<State_>();
            let mut s = state.lock().unwrap();
            let h = app.handle().clone();
            s.record(
                &h,
                "startup",
                json!({
                "shortcut": text, "shortcut_id": sc.id(), "shortcut_parsed": format!("{sc:?}"),
                "registered": reg.is_ok(), "register_error": reg.err().map(|e| e.to_string()),
                // No X11 o register() devolve Ok mesmo se a thread de eventos do global-hotkey tiver morrido
                // (fonte lida: erro só aparece com a feature `tracing`). Tempo ~0 ms é indício, não prova.
                "register_ms": register_ms,
                "env": platform::env_info(), "platform_notes": notes, "tauri": tauri::VERSION }),
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_report,
            overlay_cancel,
            test_overlay,
            test_overlay_close,
            log_key
        ])
        .run(tauri::generate_context!())
        .expect("erro ao executar a prova HF-00");
}
