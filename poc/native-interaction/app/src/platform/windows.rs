//! Windows. NÃO TESTADO em Windows real nesta etapa (só compilação cruzada, se registrada).
use super::EnvInfo;
use tauri::WebviewWindow;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
use windows_sys::Win32::Security::{
    GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Elevação do próprio processo. Relevante porque o UIPI impede arraste/atalho entre
/// processos de integridades diferentes (ADR-0001: "níveis de privilégio diferentes").
fn is_elevated() -> Option<bool> {
    unsafe {
        let mut token: HANDLE = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return None;
        }
        let mut elev = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut ret = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            &mut elev as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut ret,
        );
        CloseHandle(token);
        (ok != 0).then_some(elev.TokenIsElevated != 0)
    }
}

pub fn env_info() -> EnvInfo {
    EnvInfo {
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        elevated: is_elevated(),
        notes: vec![
            "Teste manual exigido: arrastar de um Explorer elevado para este processo não elevado (UIPI)".into(),
        ],
    }
}

pub fn configure_overlay(_w: &WebviewWindow) -> Vec<String> {
    // Hipótese a validar: focusable(false) (WS_EX_NOACTIVATE) mantém o foco na origem
    // do arraste. Nenhum ajuste Win32 adicional aplicado nesta versão.
    vec![
        "windows: sem ajustes Win32 extras; depende de focusable(false)/always_on_top do Tauri"
            .into(),
    ]
}
pub fn configure_app(_app: &mut tauri::App) -> Vec<String> {
    vec![]
}
