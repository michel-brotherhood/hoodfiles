const { invoke } = window.__TAURI__.core;
// Só funciona se a webview tiver foco; com HF_POC_ESCAPE=global o Escape vem do atalho global.
window.addEventListener('keydown', (e) => { if (e.key === 'Escape') invoke('overlay_cancel'); });
