//! Linux (Mint é plataforma oficial; sessão X11 é a referência inicial).
//! Registra distribuição, desktop e tipo de sessão gráfica para cada evidência de teste.
//! NÃO TESTADO em Linux Mint real nesta etapa.
use super::EnvInfo;
use std::collections::BTreeMap;
use tauri::WebviewWindow;

/// Lê `CHAVE=valor` de /etc/os-release (formato shell simples, valores opcionalmente entre aspas).
fn os_release() -> BTreeMap<String, String> {
    let text = std::fs::read_to_string("/etc/os-release")
        .or_else(|_| std::fs::read_to_string("/usr/lib/os-release"))
        .unwrap_or_default();
    text.lines()
        .filter_map(|l| l.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().trim_matches('"').to_string()))
        .collect()
}

pub fn env_info() -> EnvInfo {
    let rel = os_release();
    let mut details = BTreeMap::new();
    for k in [
        "PRETTY_NAME",
        "ID",
        "ID_LIKE",
        "VERSION_ID",
        "VERSION_CODENAME",
        "UBUNTU_CODENAME",
    ] {
        if let Some(v) = rel.get(k) {
            details.insert(format!("os_release.{k}"), v.clone());
        }
    }
    // Sessão gráfica: o dado que decide se atalho global e posicionamento são confiáveis.
    for k in [
        "XDG_SESSION_TYPE",
        "XDG_CURRENT_DESKTOP",
        "XDG_SESSION_DESKTOP",
        "DESKTOP_SESSION",
        "DISPLAY",
        "WAYLAND_DISPLAY",
        "GDK_BACKEND",
    ] {
        details.insert(
            k.to_string(),
            std::env::var(k).unwrap_or_else(|_| "(não definido)".into()),
        );
    }
    let session = std::env::var("XDG_SESSION_TYPE")
        .unwrap_or_default()
        .to_lowercase();
    let mut notes = vec![];
    let is_mint = rel.get("ID").map(|v| v == "linuxmint").unwrap_or(false);
    if !is_mint {
        notes.push(format!(
            "distribuição não é Linux Mint (ID={}): resultado NÃO vale como evidência do Mint",
            rel.get("ID").map(String::as_str).unwrap_or("?")
        ));
    }
    match session.as_str() {
        "x11" => notes.push("sessão X11 (referência inicial)".into()),
        "wayland" => notes.push(
            "sessão Wayland: atalho global e posicionamento de janela NÃO são garantidos; registrar o que ocorrer. Para comparar, teste também em sessão X11".into(),
        ),
        other => notes.push(format!("tipo de sessão '{other}' desconhecido; registrar manualmente")),
    }
    EnvInfo {
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        elevated: None,
        details,
        notes,
    }
}

pub fn configure_overlay(_w: &WebviewWindow) -> Vec<String> {
    vec!["linux: nenhum ajuste nativo; depende de focusable(false)/always_on_top do Tauri e do gerenciador de janelas".into()]
}

pub fn configure_app(_app: &mut tauri::App) -> Vec<String> {
    vec![]
}
