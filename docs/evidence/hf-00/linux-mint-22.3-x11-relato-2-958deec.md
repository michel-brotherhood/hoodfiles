# Relato 2 — Linux Mint 22.3 (Cinnamon, X11), build 958deec

Evidência preservada: **não editar**; correções entram em arquivos novos. Origem: relato textual do responsável (2026-10-23).
O JSON do relatório e a saída de `linux-run-checks.sh` **não foram anexados** a esta sessão.

> No Mint 22.3 Cinnamon/X11, build 958deec, o botão abre o overlay corretamente. A webview registra KeyD com ctrl=true, alt=true e shift=true, mas não há hotkey_raw.

| Dado | Valor |
| --- | --- |
| Sistema | Linux Mint 22.3, Cinnamon, sessão X11 |
| Build | 958deec (artifact Linux sem assinatura) |
| Botão "Testar overlay" | abre o overlay corretamente: **exibição do painel funciona** |
| `webview_key` para o atalho | `code: KeyD`, `ctrl: true`, `alt: true`, `shift: true` |
| `hotkey_raw` | **nenhum**: o handler do atalho global não foi chamado |
| Layouts mencionados pelo responsável para teste | `br,us` |
| Não informado | `register_ms`, opções do `setxkbmap`, estado de Scroll Lock, `state` real do X para a tecla |

Restrição do responsável: não substituir o atalho global por evento local da webview; o atalho precisa funcionar **durante o arraste no Nemo**. HF-00 permanece bloqueada.
