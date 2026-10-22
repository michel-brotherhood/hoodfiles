# Prova HF-00 — interação nativa (não é o produto)

Tauri 2.12.5 + Rust. Mostra, sem converter nada, se este fluxo funciona: **arrastar arquivos no Explorer/Finder → pressionar atalho → overlay perto do cursor → soltar no overlay → app recebe os caminhos**. Relatório e matriz de resultados: `docs/poc-native-interaction.md`.

## Estrutura

| Pasta | Conteúdo |
| --- | --- |
| `core/` | Lógica portátil testável em qualquer SO: máquina de estados da sessão, posicionamento multi-monitor/DPI, registro de entradas com IDs opacos |
| `app/` | App Tauri da prova; contrato por plataforma em `app/src/platform/{windows,macos,linux}.rs` |
| `app/ui/` | Janela de diagnóstico (`index.html`) e overlay (`overlay.html`), JS puro, sem bundler |
| `scripts/` | Fixtures (`make-fixtures.sh`/`.ps1`) e smoke Linux (`linux-smoke.sh`) |

## Pré-requisitos

- **Windows 10/11 x64:** Rust estável (`rustup`), Visual Studio Build Tools com “Desenvolvimento para desktop com C++”, WebView2 Runtime (já presente no Windows 11).
- **macOS (Apple silicon):** Xcode Command Line Tools (`xcode-select --install`), Rust estável.
- Nenhum Node/npm: a UI é estática.

## Executar (Windows e macOS)

```text
cd poc/native-interaction
cargo test -p hf-poc-core                 # lógica portátil
cargo run -p hf-poc-app                   # abre a janela de diagnóstico
```

Gere as fixtures antes: macOS `scripts/make-fixtures.sh`; Windows `powershell -ExecutionPolicy Bypass -File scripts\make-fixtures.ps1` (cria `hf-poc-fixtures` na Área de Trabalho).

Variáveis (opcionais, defina antes do `cargo run`):

| Variável | Valores | Efeito |
| --- | --- | --- |
| `HF_POC_SHORTCUT` | ex. `CommandOrControl+Alt+Shift+D` (padrão) | atalho que mostra o overlay |
| `HF_POC_MODE` | `toggle` (padrão) / `hold` | alternar ou manter pressionado |
| `HF_POC_ESCAPE` | `global` (padrão) / `webview` | Escape registrado como atalho global só enquanto o overlay está visível, ou só pela webview |
| `HF_POC_FOCUSABLE` | `1` (padrão) / `0` | overlay pode receber foco; `0` = janela não ativável |
| `HF_POC_ACCESSORY` | `1` | macOS: política Accessory (sem Dock) |
| `HF_POC_TRACE` | `1` | registra eventos `drag_over` |

Windows (PowerShell): `$env:HF_POC_FOCUSABLE="0"; cargo run -p hf-poc-app`. macOS: `HF_POC_FOCUSABLE=0 cargo run -p hf-poc-app`.

## CI e builds de teste sem assinatura

`.github/workflows/hf-00-poc.yml` (PR e execução manual) roda em `windows-2025` (x64) e `macos-15` (Apple silicon/arm64): fmt, clippy `-D warnings`, testes do núcleo, build release e empacotamento. Falha se o host do Rust não for o esperado. Os artifacts (`...SEM-ASSINATURA...`, retenção 14 dias) **não são assinados nem notarizados**; no macOS o `.app` leva só assinatura ad-hoc (exigência do arm64). **CI verde só prova compilação e lógica portátil, não o fluxo nativo.** Roteiro curto de teste: `ROTEIRO-N03-N06.md`.

## Roteiro de teste manual

1. Anote ambiente (SO/build, CPU, RAM, monitores/escala) — o app já registra SO, arquitetura, monitores e elevação no evento `startup`.
2. Para cada caso `N01…N20` da matriz em `docs/poc-native-interaction.md`, execute e preencha o resultado (`PASSOU` / `FALHOU` / `PARCIAL` + observação). Variantes obrigatórias: `FOCUSABLE=0` e `ESCAPE=webview`; `MODE=hold`.
3. O fluxo central (N03–N06): abra o Explorer/Finder, **segure o botão do mouse sobre um arquivo e comece a arrastar**; com o botão ainda pressionado, aperte o atalho com a outra mão; leve o cursor até o overlay azul e solte. Conferir na janela de diagnóstico: `overlay_shown`, `drag_enter` (com os nomes), `drop_accepted`.
4. Clique em **Copiar relatório (JSON)** e cole em `docs/evidence/hf-00/<so>-<data>.json` (ou anexe o `poc-log.jsonl` indicado na barra superior). O log contém nomes de exibição, nunca caminhos completos.
5. Registre se algum diálogo de permissão apareceu (macOS: Acessibilidade/Monitoramento de Entrada; Windows: SmartScreen/firewall) e a ação exata.

## Smoke Linux (somente lógica comum)

`scripts/linux-smoke.sh` (precisa de `xvfb xdotool openbox python3-gi` e das libs do Tauri). Verifica atalho, posição do overlay e Escape. Não comprova nada sobre os alvos.

## Limitações conhecidas da prova

- Sem ícone na bandeja/barra de menus (HF-01).
- Sem detecção de tipo/validação de conteúdo — só caminhos e metadados de existência.
- Atalho/overlay em Wayland não é objetivo.
- O build de `release` e a assinatura não foram avaliados.
