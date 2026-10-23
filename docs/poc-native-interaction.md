# HF-00 — Prova de interação nativa

Estado do gate: **BLOQUEADO — testes nativos em Windows, macOS e Linux Mint PENDENTES.** O gate exige a interação validada nos **três** sistemas. Nada aqui declara a etapa aprovada.

Código: `poc/native-interaction/` (instruções em seu `README.md`). Data deste relatório: 2026-10-22.

## 1. Plano e critérios

Pergunta central: é possível, com Tauri 2, receber os caminhos de um arraste **iniciado em outro aplicativo** (Explorer/Finder) depois de acionar um atalho global durante o arraste, mostrando um alvo perto do cursor?

Critérios de sucesso (todos medidos nos três SOs): overlay visível perto do cursor em < 200 ms depois do atalho (meta provisória a calibrar, não é requisito da spec); o arraste de origem **não** é cancelado pelo atalho nem pelo overlay; caminhos chegam ao app (na entrada ou no drop); lote ≥ 25 itens e nomes Unicode corretos; Escape devolve o controle; nenhuma permissão ampla sem necessidade comprovada.

Riscos principais: (R1) atalho não disparar, ou disparar mas cancelar o arraste, enquanto o botão do mouse está pressionado; (R2) overlay mostrar-se sem roubar foco/ativação da origem; (R3) Escape global conflitar com o cancelamento do próprio arraste; (R4) UIPI no Windows (origem elevada); (R5) macOS exigir Acessibilidade/Monitoramento de Entrada; (R6) arquivos virtuais (zip, nuvem, navegador) sem caminho real.

## 2. Pesquisa: o que está confirmado e o que é hipótese

A busca em documentação web não foi concluída (a consulta às páginas oficiais do Tauri falhou neste ambiente). As afirmações abaixo vêm da **leitura do código-fonte das versões fixadas** no `Cargo.lock` (tauri 2.12.5, wry 0.57.0, global-hotkey 0.8.0) e são confirmações *estáticas*: dizem o que o código faz, não que funciona em uma sessão real de arraste.

| Afirmação | Base | Estado |
| --- | --- | --- |
| Tauri expõe `DragDropEvent::{Enter{paths,position}, Over, Drop{paths,position}, Leave}`. Em uma `WebviewWindow` o evento chega por `on_window_event` (`WindowEvent::DragDrop`); `on_webview_event` **não** dispara | `tauri-runtime/src/window.rs`; `tauri-runtime-wry/src/lib.rs` (`WebviewKind::WindowContent` → `SynthesizedWindowEvent`) | Confirmado (fonte) e medido no Linux (antes/depois da correção) |
| Windows: wry registra um `IDropTarget` (OLE `RegisterDragDrop`); `DragEnter` entrega `Enter` com caminhos de `CF_HDROP` | `wry/src/webview2/drag_drop.rs` | Confirmado (fonte). Itens sem `CF_HDROP` (arquivos virtuais) não têm caminho: hipótese a testar (N20) |
| macOS: wry implementa `draggingEntered:` e emite `Enter` com caminhos | `wry/src/wkwebview/drag_drop.rs` | Confirmado (fonte) |
| Atalho global: Windows usa `RegisterHotKey` + `WM_HOTKEY` numa janela de mensagens; macOS usa `RegisterEventHotKey` (Carbon). `CGEventTap` só para teclas de mídia | `global-hotkey/src/platform_impl/{windows,macos}` | Confirmado (fonte). Que isso dispare com o botão do mouse pressionado no meio de um arraste e sem diálogo de permissão: **hipótese** (N02, N03, N17) |
| Tauri tem `focusable(false)`, `focused(false)`, `always_on_top`, `visible_on_all_workspaces`, `cursor_position`, `available_monitors` (com área útil e escala) | `tauri/src/window`, `webview_window.rs` | Confirmado (fonte) |
| Mostrar o overlay sem ativar o app evita cancelar o arraste de origem | — | **Hipótese** (N04, N15) |
| Escape registrado como atalho global não interfere no cancelamento do arraste | — | **Hipótese** contrária provável (R3); por isso há `HF_POC_ESCAPE=webview` (N09) |
| Origem elevada → destino não elevado bloqueado por UIPI | conhecimento geral do Windows, não verificado aqui | **Hipótese** (N14) |

Electron não foi avaliado: não há bloqueio concreto (o ADR só pede comparação mediante motivo técnico).

## 3. O que foi implementado

- `core` (Rust, sem dependências): máquina de estados da sessão (`Idle → OverlayVisible → DragOver → Idle`, modos *toggle* e *hold*, Escape, timeout), posicionamento do overlay por monitor com escala mista/coordenadas negativas/inversão nas bordas, e registro de entradas com **IDs opacos** (caminhos completos nunca saem do núcleo; rejeita relativos/inexistentes, deduplica por caminho canônico inclusive via symlink, limite de 1000).
- `app` (Tauri): atalho global configurável, overlay sem decoração/sempre no topo/fora da barra de tarefas/não focado, receptor de arraste no overlay, registro de latência e eventos (`startup`, `transition`, `overlay_shown`, `drag_enter`, `drop_accepted`, `overlay_hidden`) em JSONL local e na janela de diagnóstico, janela de controle que também registra arrastes.
- Contratos por plataforma (`platform/`): Windows — elevação do processo (`GetTokenInformation`); macOS — nível de janela 101 + `CanJoinAllSpaces|FullScreenAuxiliary`, política Accessory opcional; Linux — apenas stub de smoke test.
- Segurança: a UI só tem `core:event:default`; os dois comandos próprios checam o rótulo da webview; a UI recebe IDs/nomes, não caminhos; não há shell nem filesystem exposto.
- Lição de engenharia registrada: chamar APIs de janela direto do handler do atalho **travou** o app no Linux (a thread do atalho não é a principal). Toda ação de janela/NSWindow roda via `run_on_main_thread`.

## 4. Verificações executadas neste ambiente (Linux x86_64, Ubuntu 24.04, container)

| Verificação | Comando | Resultado |
| --- | --- | --- |
| Testes unitários do núcleo | `cargo test --workspace` (em `poc/native-interaction`) | **18 passaram, 0 falharam** |
| Formatação / lint | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings` | sem diferenças / sem avisos |
| Build do app (Linux/WebKitGTK 2.52.6) | `cargo build -p hf-poc-app` | OK |
| Compilação para Windows | `cargo check -p hf-poc-app --target x86_64-pc-windows-msvc` | **OK (só type-check; não executa)** |
| Compilação para macOS (local) | `cargo check -p hf-poc-app --target aarch64-apple-darwin` | **NÃO EXECUTADO com sucesso aqui** (compilou no CI macOS, ver abaixo): o build script de `objc2-exception-helper` precisa de compilador Objective-C/SDK da Apple. `platform/macos.rs` foi conferido contra a fonte de `objc2-app-kit 0.3.2` |
| Smoke Linux (Xvfb + openbox + xdotool) | `poc/native-interaction/scripts/linux-smoke.sh` | atalho mostra overlay: OK; overlay em 976,556 para cursor 960,540: OK; latência atalho→overlay ≈ 1 ms (medida no processo, sem repintura da webview); Escape global cancela: OK |
| Arraste X11 real entre processos (fonte GTK → overlay) | mesmo script | **Falhou 100% antes da correção** (bug no app, ver 4.1); depois, recebido em 92–100% das instâncias conforme a condição (intermitência residual, ver 4.1). Smoke tenta até 3 vezes |

### CI (GitHub Actions, PR #1, run 3 no commit `1a3dd6d`)

Workflow `.github/workflows/hf-00-poc.yml`; runners `windows-2025` (x64) e `macos-15` (Apple silicon/arm64), confirmados na documentação da GitHub e verificados no job (`rustc -vV` host = `x86_64-pc-windows-msvc` / `aarch64-apple-darwin`). Repositório privado: usa a franquia de minutos da conta, com macOS mais caro por minuto.

| Job | Resultado | Observação |
| --- | --- | --- |
| Windows x64 | **verde** (fmt, clippy `-D warnings`, testes, build release, empacotamento) | 2 falhas reais encontradas e corrigidas antes: `unused_mut` em teste e dois testes que assumiam caminhos/nomes de unix |
| macOS Apple silicon | **verde** (idem) | primeira compilação real de `platform/macos.rs`: passou |

Artifacts `hf-poc-<plataforma>-SEM-ASSINATURA-<sha>` (14 dias): Windows `.exe`; macOS `.app` em `.tar.gz` com assinatura ad-hoc (exigência do arm64; **sem** assinatura de desenvolvedor nem notarização). **Isto prova compilação, lint e lógica portátil. Nenhum executável foi aberto, nem o fluxo de arraste testado, em Windows ou macOS.**

### 4.1 Investigação do arraste externo no Linux

Ambiente: container Ubuntu 24.04.5, Xvfb 1920×1080, openbox, fonte de arraste GTK3 (`text/uri-list`, Python/gi) e ponteiro sintético (`xdotool`/XTEST). **Não é Linux Mint, nem Nemo, nem sessão real.**

| # | Experimento | Resultado |
| --- | --- | --- |
| 1 | Cópia local do `wry` 0.57.0 com `eprintln` nos sinais GTK de arraste (a cópia fica fora do repositório) | `drag-motion`, `drag-data-received` (`info=2`, URI correta), `drag-drop` e `drag-leave` **disparam**. A hipótese “`info == 2` não casa com WebKitGTK 2.52” está **refutada**; `wry`/WebKitGTK entregam o `Enter` |
| 2 | Leitura de `tauri-runtime-wry` | Para `WebviewKind::WindowContent` o arraste vira `WindowEvent::DragDrop`; o app registrava `on_webview_event`, que **nunca** é chamado nesse caso |
| 3 | Antes/depois de trocar para `on_window_event` | Antes: nenhum evento no overlay nem na janela principal. Depois: `drag_enter` (com nomes), `drag_over`, `drop_accepted` com IDs e `main_drag_*` aparecem |

**Causa do defeito original (confirmada):** erro de uso da API do Tauri no código da prova, determinístico e **não específico do Linux**. Os builds de Windows e macOS anteriores à correção teriam o mesmo defeito em N05/N06. Nenhuma dependência foi trocada nem é justificada por esse achado.

**Intermitência residual (causa NÃO isolada).** Após a correção, algumas instâncias do app nunca entregam o arraste ao overlay (o overlay não recebe nenhum `drag-motion`; instâncias ruins falham em todas as tentativas). Contagens (instâncias independentes, app reiniciado a cada uma):

| Condição | Falhas / instâncias |
| --- | --- |
| Janela de diagnóstico no caminho do arraste, poucos eventos de ponteiro dentro do overlay | 7 / 86 |
| Idem, com 6 movimentos extras dentro do overlay | 2 / 30 |
| Janela de diagnóstico fora do caminho, poucos eventos | 1 / 60 |
| Janela de diagnóstico fora do caminho, com movimentos extras | 0 / 94 |
| GTK puro (Python, 2 janelas no mesmo processo, sem WebKit/Tauri), caminho equivalente | 0 / 40 |
| Fonte de arraste iniciada **depois** de o overlay já estar visível, janela de diagnóstico fora do caminho (smoke antigo; 1ª instância) | debug 1 / 10; release 5 / 16. A 2ª instância (fonte já existente quando o overlay aparece) passou em todas |
| **Fluxo real**: botão pressionado e arraste em andamento → atalho → overlay aparece → soltar nele (diagnóstico fora do caminho) | debug 1 / 20; release **0 / 20** (39 de 40 recebidos) |

Leitura honesta: (a) a falha está associada a o arraste cruzar outra janela do mesmo processo antes do overlay (9/116 ≈ 8% contra 1/154 ≈ 0,6%); (b) ela também aparece quando a fonte de arraste é iniciada depois de o overlay já estar mapeado, ordem que não ocorre no uso real (o gerenciador de arquivos existe antes); (c) `XdndAware` do overlay é idêntico em instâncias boas e ruins; (d) GTK puro não reproduziu (chance de 0/40 com taxa de 8% ≈ 4%, indício, não prova). O mecanismo não foi isolado e **não** se sabe se ocorre no Mint real. Hipóteses abertas: interação WebKit/Tauri entre janelas do mesmo processo; estado do cache de janelas do GDK da fonte; artefato do harness sintético. **No fluxo que importa para o produto (atalho durante o arraste, nada mais no caminho) o resultado sintético foi 39/40.** Mitigações: o roteiro manda manter a janela de diagnóstico fora do caminho; o smoke usa o fluxo real e reinicia o app entre tentativas (até 3 instâncias), pois as falhas agrupam por instância. Decisão de produto/engenharia a tomar em HF-05 se o Mint real reproduzir: não manter outra janela do app sob o caminho do arraste enquanto o overlay está aberto.

Consequência: **ainda não há evidência de recebimento de caminhos de um arraste externo em Windows, macOS ou Linux Mint reais.** Provados: lógica portátil, compilação (CI) nos três sistemas e, em Linux X11 sintético (container Ubuntu, não Mint), o fluxo atalho-durante-o-arraste→overlay→drop→Escape em 39/40 repetições.

## 5. Matriz de testes nativos (preencher na máquina de destino)

`PENDENTE` em todas as linhas. Variantes a repetir nos casos N03–N06 e N09: padrão; `HF_POC_FOCUSABLE=0`; `HF_POC_ESCAPE=webview`; `HF_POC_MODE=hold`.

| ID | Caso | Esperado | Windows | macOS | Linux Mint |
| --- | --- | --- | --- | --- | --- |
| N01 | Inventário: SO/build, CPU/arquitetura, RAM, monitores/escala, elevação | Registrado no `startup` | PENDENTE | PENDENTE | PENDENTE |
| N02 | Atalho registra sem erro e sem diálogo de permissão | `registered:true`; nenhum diálogo | PENDENTE | PENDENTE | PENDENTE |
| N03 | Atalho **com o botão do mouse pressionado** no meio de um arraste de Explorer/Finder | `overlay_shown` ocorre; origem continua arrastando | PENDENTE | PENDENTE | PENDENTE |
| N04 | Overlay visível não cancela o arraste nem tira o foco da origem | Arraste prossegue; origem segue ativa | PENDENTE | PENDENTE | PENDENTE |
| N05 | Caminhos na entrada do arraste | `drag_enter` com contagem e nomes corretos | PENDENTE | PENDENTE | PENDENTE |
| N06 | Soltar no overlay | `drop_accepted` com IDs; original intacto (hash igual) | PENDENTE | PENDENTE | PENDENTE |
| N07 | Lote de 25 itens (`lote-25`) | 25 aceitos, sem travar | PENDENTE | PENDENTE | PENDENTE |
| N08 | Nomes: espaços, acentos, emoji, aspas, 180 caracteres | Nomes idênticos; caminho completo ausente do log | PENDENTE | PENDENTE | PENDENTE |
| N09 | Escape durante o arraste (global e webview) | Overlay fecha; registrar o que acontece com o arraste de origem | PENDENTE | PENDENTE | PENDENTE |
| N10 | Modo `hold`: soltar o atalho antes e durante o arraste | Antes: fecha; durante: mantém o alvo | PENDENTE | PENDENTE | PENDENTE |
| N11 | Atalho em conflito com apps comuns (editor, navegador, IME) | Informar conflitos; atalho remapeável via env | PENDENTE | PENDENTE | PENDENTE |
| N12 | DPI/Retina: 100%/150%/200% (Windows), Retina (macOS) | Overlay perto do cursor, 320×180 lógicos | PENDENTE | PENDENTE | PENDENTE |
| N13 | Segundo monitor, inclusive à esquerda/acima do principal e escala diferente | Overlay no monitor do cursor, dentro da área útil | PENDENTE | PENDENTE | PENDENTE |
| N14 | Origem elevada (Explorer “como administrador”/processo elevado) → app comum | Registrar bloqueio ou sucesso | PENDENTE | n/a | n/a |
| N15 | Overlay não ativa o app (barra de tarefas/Dock sem mudança de foco) | Foco permanece na origem | PENDENTE | PENDENTE | PENDENTE |
| N16 | App em tela cheia / outro Space / sobre a barra de menus | Overlay aparece onde está o cursor | PENDENTE | PENDENTE | PENDENTE |
| N17 | Permissões: Acessibilidade, Monitoramento de Entrada, firewall/SmartScreen | Nenhuma necessária, ou documentar qual e por quê | PENDENTE | PENDENTE | PENDENTE |
| N18 | Arrastar para fora do overlay e voltar; soltar fora | `drag_leave`; nada registrado ao soltar fora | PENDENTE | PENDENTE | PENDENTE |
| N19 | Pastas de rede, OneDrive/iCloud (placeholders), mídia removível | Registrar o que chega | PENDENTE | PENDENTE | PENDENTE |
| N20 | Outras origens: navegador (download), Área de Trabalho, busca, zip aberto no Explorer/Finder/Nemo | Registrar se há caminhos reais; itens virtuais sem caminho | PENDENTE | PENDENTE | PENDENTE |
| N21 | Linux: registrar versão do Mint (`/etc/os-release`), `XDG_SESSION_TYPE` (x11/wayland) e desktop (Cinnamon); repetir N03–N06 em X11 e, se disponível, Wayland | Valores aparecem no evento `startup` e no `linux-run-checks.sh` | n/a | n/a | PENDENTE |

Observações a anotar sempre: latência `shortcut_to_shown_ms`, impressão subjetiva de atraso, falhas visuais (piscar, janela atrás de outras), uso de CPU/memória ociosa se possível (processo `hf-poc-app`).

## 6. Decisões de produto se a hipótese central falhar

Só aplicável se N03/N04 falharem num SO. Não é decisão tomada.

| Opção | Experiência | Custo/risco |
| --- | --- | --- |
| A. Alvo pré-armado | Atalho/tray abre o alvo **antes** de arrastar; usuário arrasta arquivos até ele | Perde o “durante o arraste”; simples e portável |
| B. Zona de borda | Janela fina na borda da tela que se expande ao entrar um arraste | Requer receber arraste sem atalho; mais UX para acertar |
| C. Integração com o shell | Menu de contexto/Quick Action (Windows Shell/Finder Sync) | Código nativo por SO; fora do arraste |
| E. Restringir o Linux a X11 | Se atalho global ou posicionamento falharem em Wayland, declarar só sessões X11 como suportadas | Exclui usuários de Wayland; decisão de produto |
| D. Adaptador nativo (NSPanel/Win32 `WS_EX_NOACTIVATE`/hook de baixo nível) | Mantém a experiência-alvo com mais código nativo | Maior manutenção; pode exigir permissões no macOS |

## 7. Próximo passo

1. Responsável executa a matriz no Windows x64, no macOS Apple silicon e no Linux Mint (anotando versão e sessão) e anexa os JSONs em `docs/evidence/hf-00/`.
2. Roteiro curto N03–N06: `poc/native-interaction/ROTEIRO-N03-N06.md` (vai dentro dos artifacts como `LEIA-ME.md`). `platform/macos.rs` compila no CI, mas nunca foi executado.
3. Com N03–N06 aprovados nos três, atualizar este relatório e a ADR-0001 e então liberar HF-01. Se falhar, escolher uma opção da seção 6.
