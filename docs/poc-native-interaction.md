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

### 4.2 Relato do Mint 22.3 (Cinnamon, X11, build `818d708`): atalho não abre o overlay

Fato reportado: o app abre e o `startup` mostra `registered: true`; pressionar o atalho (padrão e `Control+Alt+H`) sem arrastar não abre o overlay; os relatórios só têm `startup`. Como nenhum `transition` aparece, o evento **não chegou à máquina de estados**: ou o handler não foi chamado, ou a comparação do atalho falhou.

O que a leitura do código (`global-hotkey` 0.8.0, X11) mostra, sem executar no Mint:

| Ponto | Fato lido no código | Por que importa |
| --- | --- | --- |
| `register()` | Se a thread interna de eventos tiver morrido, o erro só aparece com a feature opcional `tracing` e `register()` devolve `Ok(())` | `registered: true` **não prova** que o atalho está ativo (hipótese H1) |
| Modificadores | Compara o estado X11 exatamente com `Alt→Mod1`, `Ctrl`, `Shift`, `Super→Mod4`, ignorando Num/Caps Lock | Se o `Alt` do teclado não for `Mod1` (opção de layout), o atalho nunca casa (H2) |
| Keycode | Usa o **primeiro** keycode cujo mapa contenha o keysym `d`/`h` em qualquer coluna | Layouts múltiplos/incomuns podem escolher outro keycode (H3) |
| Conflito | Se outro cliente já tem o mesmo grab, o `register` falha com `AlreadyRegistered` | Não é o caso (o registro devolveu `Ok`), mas só vale se a thread estiver viva |

Hipóteses abertas, **nenhuma confirmada**: H1 thread de eventos morta; H2 mapeamento de modificadores; H3 escolha de keycode; H4 outra causa no Cinnamon/Muffin. Nada foi alterado nas configurações do Cinnamon nem nas dependências.

**Instrumentação adicionada (build seguinte):**

- `hotkey_raw`: gravado na **entrada** do handler global, antes de qualquer comparação: `event_id`, `state` (`Pressed`/`Released`), atalho resolvido, `shortcut_id`, `is_main`/`is_escape` e nome da thread.
- `window_error` e `hotkey_error`: erros de `set_size`, `set_position`, `show`, `hide`, `set_always_on_top`, `run_on_main_thread`, cursor/monitores e registro/remoção do Escape, antes ignorados. `overlay_shown` agora inclui `cursor_error`/`monitors_error`; `overlay_probe` (300 ms depois) mostra `visible`, posição e tamanho reais.
- `startup`: `shortcut_id`, atalho resolvido e `register_ms` (tempo próximo de 0 ms é indício de thread morta, não prova; no teste sintético deu ≈ 20 ms).
- Botões **Testar overlay (abre/fecha)** e **Fechar overlay** na janela de diagnóstico: usam o mesmo fluxo do atalho (máquina de estados → thread principal → `show_overlay`), sem teclado. Separam “captura do teclado” de “exibição do painel”.
- `webview_key`: teclas vistas pela **webview** da janela de diagnóstico (só combinações com Ctrl/Alt/Meta e as próprias modificadoras), com `code`, `alt`, `meta`, `altGraph`: mostra o que o GTK/WebKit entende como Alt.
- `linux-run-checks.sh`: leituras somente-leitura de `setxkbmap -query`, `xmodmap -pm` e das teclas d/h/modificadores.

**Como ler o próximo relatório:**

| Observação | Leitura |
| --- | --- |
| `hotkey_raw` Pressed aparece, mas não há `overlay_shown` | falha entre handler e exibição: ver `transition`, `window_error` e `overlay_probe` |
| `overlay_probe` com `visible:false` ou posição diferente do esperado | o gerenciador de janelas ignorou o painel (display), não o teclado |
| Botão abre o overlay e o `overlay_probe` confere | exibição OK; o problema é só a captura do atalho |
| Botão **não** abre: `window_error`/`error` | o problema é de janela (a mensagem diz qual operação) |
| Sem `hotkey_raw`, mas `webview_key` mostra `ctrl`+`alt`+`shift` e `code: KeyD` | os modificadores são os esperados para o WebKit: suspeitar de H1 (ver `register_ms`) ou H3 (`xmodmap`) |
| `webview_key` mostra `alt:false`/`meta:true` para a tecla Alt | Alt mapeado de outro modo (H2): testar `HF_POC_SHORTCUT` sem Alt |

Variantes inofensivas para repetir (variável de ambiente do app, nada do Cinnamon muda): `HF_POC_SHORTCUT=F9`, `HF_POC_SHORTCUT=Control+Shift+F9`, `HF_POC_SHORTCUT=Alt+Shift+KeyD`. Verificação sintética (Xvfb): `hotkey_raw`, `overlay_probe` e os dois botões funcionam; isso **não** reproduz nem explica o relato do Mint.

### 4.3 Relato 2 do Mint (build `958deec`): o botão abre o overlay, o atalho global não chega

Relato preservado em `docs/evidence/hf-00/linux-mint-22.3-x11-relato-2-958deec.md` (e o relato 1 em `...relato-1-818d708.md`). Fatos: o botão "Testar overlay" abre o overlay (**a exibição funciona**); a webview vê `KeyD` com `ctrl`, `alt` e `shift` verdadeiros; **não há `hotkey_raw`**, ou seja, o handler do atalho global não é chamado. Restrição: o atalho **tem de ser global** (acionado durante o arraste no Nemo); substituí-lo por evento da webview não é opção.

**Método.** Servidor X virtual (Xvfb 1920×1080 + openbox) no container Ubuntu 24.04, `xdotool`, `setxkbmap`, `xmodmap` e `xev`, app `958deec`. **Não é o Mint real.** Scripts versionados: `scripts/x11-hotkey-matrix.sh`, `scripts/x11-hotkey-xkb-options.sh`; saídas em `docs/evidence/hf-00/container-x11-hotkey-*.txt`. São diagnósticos (alguns casos falham por definição), não gates de CI.

**Registro X11 e thread de eventos (`global-hotkey` 0.8.0).** O grab é `XGrabKey` na raiz com o modificador exato do atalho mais quatro variantes (`0`, `Mod2`=NumLock, `Lock`=CapsLock, `Mod2|Lock`); a thread de eventos compara `state & (Shift|Control|Mod1|Mod4)` com o atalho. Em todas as execuções do container a thread estava viva (`register_ms` entre 15 e 37 ms; eventos entregues quando o grab casava): **não observei thread morta**. Ela continua possível no Mint (H1) e é checada pelo `register_ms` do `startup` do responsável: ~0 ms indicaria thread morta, ~15–40 ms indica viva.

**Resultados (contagem de `hotkey_raw` para uma pressão; 2 = Pressed+Released chegaram):**

| Configuração do servidor de teste | Atalho `Ctrl+Alt+Shift+D` |
| --- | --- |
| `us`, sem locks / Num Lock / Caps Lock / Num+Caps ligados | **2** (grabs cobrem `Mod2` e `Lock`) |
| `us`, Scroll Lock pressionado (keymap padrão não liga Scroll Lock a nenhum modificador; LED fica desligado) | **2** |
| Scroll Lock mapeado em **`Mod3`** e ligado (`state` do X = `0x2d`) | **0**: a webview vê `ctrl/alt/shift = true`, o atalho global não chega. Desligado de volta: **2** |
| `br,us` **sem** opção de troca de grupo; grupo 2 ativo | **2** (o layout, o grupo ativo e o keycode escolhido não são o problema) |
| `br,us` **com** `grp:alt_shift_toggle` / `ctrl_shift_toggle` / `ctrl_alt_toggle` | **0** para o atalho padrão; `Control+Alt+H` falha **só** com `ctrl_alt_toggle` |
| `grp:alt_space`, `win_space`, `shifts`, `caps` | **2** |

Verificação causal do caso `Mod3` (fora do repositório, em cópia local de `global-hotkey` com `Mod3` acrescentado às variantes de grab): o atalho passou a chegar com Scroll Lock/`Mod3` ligado (0 → 1 evento) e o repositório **não foi alterado**.

**Dois mecanismos reproduzidos, nenhum confirmado como o do responsável:**

1. **Modificador extra no `state`** (ex.: `Mod3`): como o grab é por máscara exata, qualquer bit fora de `Shift/Control/Mod1/Mod4` + `Lock/Mod2` impede o casamento. Compatível com o relato (a webview vê os três modificadores, pois o GDK ignora `Mod3`; sem `hotkey_raw`). Não testei `Mod5` (AltGr) nem teclados reais.
2. **Opção de troca de layout do XKB** cujo acorde coincide com os modificadores do atalho: ao completar o acorde o `state` perde um modificador (`0xc` em vez de `0xd`) e a webview também passa a ver um a menos. Isso **não** bate com o relato (a webview viu os três verdadeiros), então só vale para o responsável se o GDK dele se comportar de outro modo.

**Hipóteses refutadas neste teste:** escolha do keycode (H3: o mesmo keycode 40/43 em `us`, `br,us` e `br(abnt2),us`); layout `br,us` por si só; grupo de layout ativo; Num/Caps Lock.

**Dados que decidem o caso do responsável** (nada disso foi enviado ainda): `register_ms` do `startup`; saída de `./linux-key-probe.sh` (mostra o `state` real do X para o atalho e marca bits não cobertos); `setxkbmap -query` (opções `grp:*`); `xmodmap -pm` (o que está em `Mod3`/`Mod5`); e se o atalho passa a funcionar com Scroll Lock desligado.

**Caminhos de correção, SEM aplicar (alteram dependências ou arquitetura e dependem de decisão):**

| Caminho | O que resolve | Custo/risco |
| --- | --- | --- |
| A. Adaptador X11 próprio em `platform/linux.rs` (grab com `AnyModifier` e filtro no handler, ou todas as combinações de bits ignoráveis) | Mecanismo 1 e, com filtro por keycode, também o 2 | Nova dependência direta (`x11rb`) e código nativo por manter |
| B. Cópia/patch do `global-hotkey` (variantes de grab com `Mod3`, opcionalmente `Mod5`) | Mecanismo 1 | Dependência modificada; manter ou subir upstream |
| C. Trocar o atalho padrão no Linux para algo que não use acordes de troca de layout | Parte do mecanismo 2 | Não resolve `Mod3`; depende da configuração de cada usuário |
| Descartado | Evento local da webview | Não funciona durante o arraste em outro aplicativo |

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
