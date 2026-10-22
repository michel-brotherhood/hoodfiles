//! Linux NÃO é plataforma de produto (docs/spec.md). Existe só para smoke test da
//! lógica comum em CI/containers; não prova nada sobre Windows ou macOS.
use super::EnvInfo;
use tauri::WebviewWindow;

pub fn env_info() -> EnvInfo {
    EnvInfo {
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        elevated: None,
        notes: vec!["Linux: apenas smoke test; resultados não valem para os alvos".into()],
    }
}
pub fn configure_overlay(_w: &WebviewWindow) -> Vec<String> {
    vec!["linux: nenhum ajuste nativo".into()]
}
pub fn configure_app(_app: &mut tauri::App) -> Vec<String> {
    vec![]
}
