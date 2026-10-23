//! Contrato por plataforma. A lógica comum fica em `main.rs`/`hf-poc-core`; aqui só o
//! que depende do SO. Cada implementação declara o que NÃO conseguiu garantir.

use serde::Serialize;
use tauri::WebviewWindow;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as imp;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as imp;

#[cfg(not(any(windows, target_os = "macos")))]
mod linux;
#[cfg(not(any(windows, target_os = "macos")))]
use linux as imp;

#[derive(Debug, Clone, Serialize)]
pub struct EnvInfo {
    pub os: &'static str,
    pub arch: &'static str,
    /// `Some(true)` se o processo roda elevado (Windows). `None` = não aplicável/desconhecido.
    pub elevated: Option<bool>,
    /// Pares chave/valor do ambiente (no Linux: distribuição, sessão gráfica, desktop...).
    pub details: std::collections::BTreeMap<String, String>,
    pub notes: Vec<String>,
}

/// Informações do ambiente (sem consultar o usuário).
pub fn env_info() -> EnvInfo {
    imp::env_info()
}

/// Ajustes nativos do overlay após criação (nível da janela, comportamento de espaços...).
/// Devolve notas descrevendo o que foi aplicado ou não.
pub fn configure_overlay(w: &WebviewWindow) -> Vec<String> {
    imp::configure_overlay(w)
}

/// Chamado em `setup`, antes das janelas: política de ativação do app (macOS).
pub fn configure_app(app: &mut tauri::App) -> Vec<String> {
    imp::configure_app(app)
}
