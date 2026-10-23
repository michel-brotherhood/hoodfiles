# Relato 1 — Linux Mint 22.3 (Cinnamon, X11), build 818d708

Evidência preservada: **não editar**; correções entram em arquivos novos. Origem: relato textual do responsável (2026-10-23).
O JSON do relatório e a saída de `linux-run-checks.sh` **não foram anexados** a esta sessão.

> No Mint 22.3 Cinnamon/X11, build 818d708, o app abre e informa registered: true, mas pressionar o atalho sem arrastar não abre o overlay. Reproduzido com o padrão e com Control+Alt+H. Ambos os relatórios contêm somente startup.

| Dado | Valor |
| --- | --- |
| Sistema | Linux Mint 22.3, Cinnamon, sessão X11 |
| Build | 818d708 (artifact Linux sem assinatura) |
| `startup.registered` | `true` |
| Atalho padrão (`CommandOrControl+Alt+Shift+D`) | não abre o overlay |
| `Control+Alt+H` | não abre o overlay |
| Eventos nos relatórios | somente `startup` |
| Não informado | `register_ms` (build não tinha), layouts/opções de teclado, estado de Scroll Lock/Num Lock |
