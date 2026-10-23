//! macOS. NÃO TESTADO em macOS real nesta etapa (só compilação cruzada, se registrada).
use super::EnvInfo;
use objc2_app_kit::{NSWindow, NSWindowCollectionBehavior};
use tauri::WebviewWindow;

/// NSPopUpMenuWindowLevel: acima de janelas flutuantes comuns e do Dock.
const POPUP_MENU_LEVEL: isize = 101;

pub fn env_info() -> EnvInfo {
    EnvInfo {
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        elevated: None,
        details: Default::default(),
        notes: vec![
            "Atalho via Tauri global-shortcut (RegisterEventHotKey): hipótese de não exigir Acessibilidade/Monitoramento de Entrada — confirmar manualmente e registrar se algum diálogo de permissão surgir".into(),
        ],
    }
}

pub fn configure_overlay(w: &WebviewWindow) -> Vec<String> {
    let mut notes = Vec::new();
    match w.ns_window() {
        Ok(ptr) if !ptr.is_null() => {
            // SAFETY: ponteiro NSWindow válido devolvido pelo Tauri, usado na thread principal
            // (setup/run_on_main_thread) durante a vida da janela.
            let win: &NSWindow = unsafe { &*(ptr as *const NSWindow) };
            win.setLevel(POPUP_MENU_LEVEL);
            win.setCollectionBehavior(
                NSWindowCollectionBehavior::CanJoinAllSpaces
                    | NSWindowCollectionBehavior::FullScreenAuxiliary,
            );
            notes.push(
                "macos: level=101 (popup menu), CanJoinAllSpaces|FullScreenAuxiliary aplicados"
                    .into(),
            );
        }
        _ => notes.push("macos: NSWindow indisponível; nível não ajustado".into()),
    }
    notes
}

pub fn configure_app(app: &mut tauri::App) -> Vec<String> {
    // Accessory: sem ícone no Dock e sem roubar ativação ao mostrar o overlay.
    // Hipótese a validar: não interfere na sessão de arraste do Finder. Opt-in
    // (HF_POC_ACCESSORY=1) para comparar os dois modos no teste manual.
    if std::env::var("HF_POC_ACCESSORY").as_deref() == Ok("1") {
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);
        vec!["macos: ActivationPolicy::Accessory (HF_POC_ACCESSORY=1)".into()]
    } else {
        vec!["macos: ActivationPolicy::Regular (padrão)".into()]
    }
}
