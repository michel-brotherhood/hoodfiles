# Roteiro HF-00 — N03 a N06 (build de teste SEM assinatura) — Windows, macOS e Linux Mint

Este build **não é assinado nem notarizado** e não é o produto. Serve só para verificar, num computador real, se o fluxo central funciona. Compilar no CI **não** prova isso: só você, arrastando de verdade, prova.

## 1. Baixar
1. Abra o PR #1 do repositório → aba **Checks** → workflow **HF-00 PoC** → seção **Artifacts** (precisa estar logado com acesso ao repositório privado).
2. Baixe o artifact da sua plataforma (`...windows-x64-SEM-ASSINATURA...`, `...macos-arm64-SEM-ASSINATURA...` ou `...linux-x64-SEM-ASSINATURA...`). **Use sempre os artifacts mais recentes do PR**: builds anteriores tinham um defeito que impedia o recebimento do arraste (N05/N06). Vem em `.zip`; descompacte.
3. Confira o hash em `SHA256SUMS.txt` (opcional).

## 2. Executar
- **Windows 10/11 x64:** execute `hf-poc-app.exe`. O SmartScreen avisará (não assinado): *Mais informações → Executar assim mesmo*. Windows 10 precisa do WebView2 Runtime.
- **macOS (Apple silicon):** dentro do `.zip` há `HoodFilesPoC-macos-arm64.tar.gz`. No Terminal: `tar -xzf HoodFilesPoC-macos-arm64.tar.gz && xattr -dr com.apple.quarantine HoodFilesPoC.app && open HoodFilesPoC.app`. Se o macOS bloquear, clique com o botão direito no app → *Abrir*. Anote qualquer pedido de permissão.

- **Linux Mint (x86_64):** o artifact traz `hf-poc-linux-x64-<sha>.tar.gz`. No Terminal:
  ```
  tar -xzf hf-poc-linux-x64-*.tar.gz && cd hf-poc-linux-x64-*/
  ./linux-run-checks.sh      # mostra versão do Mint, sessão gráfica e bibliotecas que faltam
  ./hf-poc-app
  ```
  Dependências de runtime (Mint 22.x, base Ubuntu 24.04): `sudo apt install libwebkit2gtk-4.1-0 libjavascriptcoregtk-4.1-0 libgtk-3-0t64 libsoup-3.0-0 libdbus-1-3`. Lista completa por biblioteca em `DEPENDENCIAS-RUNTIME.txt`. O build foi feito em Ubuntu 24.04; **Mint 21.x e LMDE não são cobertos** (se tentar mesmo assim, anote o resultado). **Anote:** versão do Mint (`grep PRETTY_NAME /etc/os-release`) e tipo de sessão (`echo $XDG_SESSION_TYPE`; o app também registra isso no evento `startup`, campos `os_release.*` e `XDG_SESSION_TYPE`). A referência é sessão **X11**; se a sua for Wayland, registre e, se possível, repita em X11 (tela de login → ícone de sessão).

Abre a janela **diagnóstico**. Deixe-a visível, mas **fora do caminho do arraste e longe do cursor** (o overlay surge perto do cursor; no Linux a janela de diagnóstico no caminho do arraste aumentou a chance de falha nos testes automáticos).
Atalho: **Ctrl+Alt+Shift+D** (Windows e Linux) / **Cmd+Option+Shift+D** (macOS). Confirme na tabela que o evento `startup` mostra `"registered": true`; se for `false`, anote `register_error` e pare: N03 falha.

## 3. Arquivos de teste
Rode `make-fixtures.ps1` (Windows: `powershell -ExecutionPolicy Bypass -File make-fixtures.ps1`) ou `./make-fixtures.sh` (macOS e Linux). Cria `hf-poc-fixtures` na Área de Trabalho.

## 4. Testes (repita cada um 3 vezes; anote uma linha por tentativa)
**Antes de começar:** anote SO/versão, CPU, monitores/escala e, no Linux, versão do Mint e tipo de sessão (X11/Wayland). No Linux o gerenciador de arquivos é o **Nemo**.

| ID | Faça | Observe | Resultado |
| --- | --- | --- | --- |
| N03 | No Explorer/Finder, **segure o botão do mouse** sobre `simples.txt` e mova ~30 px (o arraste começou). Com o botão ainda pressionado, aperte o atalho. | Aparece o overlay azul “Solte aqui” perto do cursor? Surge `overlay_shown` na tabela? | |
| N04 | Sem soltar o botão, leve o cursor até o overlay. | O ícone do arquivo continua preso ao cursor (arraste não foi cancelado)? A janela ativa do Explorer/Finder continua sendo a de origem? | |
| N05 | Ainda sem soltar, passe sobre o overlay. | Surge `drag_enter` com a **quantidade e os nomes** corretos? (Repita com `lote-25` selecionado inteiro: 25.) | |
| N06 | Solte sobre o overlay. | Surge `drop_accepted` com `accepted` e IDs? O overlay fecha? O arquivo original continua na pasta, intacto? | |

Pare e anote exatamente o ponto se algo falhar (ex.: “atalho não disparou com o botão pressionado”, “arraste cancelou ao aparecer o overlay”, “overlay apareceu mas `drag_enter` não veio”).

## 4b. Se o atalho não abrir o overlay (diagnóstico, sem alterar nada do sistema)
1. Clique em **Testar overlay (abre/fecha)** na janela de diagnóstico: abre o overlay pelo mesmo fluxo, sem teclado. **Fechar overlay** fecha. Isso separa captura do teclado de exibição do painel.
2. Rode `./linux-run-checks.sh` (Linux) e guarde a saída: mostra mapa de modificadores e teclas.
3. Com o atalho, olhe na tabela: `hotkey_raw` (o atalho global chegou?), `webview_key` (clique na janela de diagnóstico e pressione o atalho: o que o WebKit vê), `window_error`/`hotkey_error` e `overlay_probe`.
4. Variantes do atalho (só variável de ambiente do app): `HF_POC_SHORTCUT=F9 ./hf-poc-app`, depois `Control+Shift+F9` e `Alt+Shift+KeyD`.
5. Copie o relatório JSON e a saída do `linux-run-checks.sh` para o PR. Interpretação em `docs/poc-native-interaction.md`, seção 4.2.

## 5. Variantes (só se N03 ou N04 falharem)
No Terminal/PowerShell, defina antes de abrir o executável: `HF_POC_FOCUSABLE=0` (Linux: `HF_POC_FOCUSABLE=0 ./hf-poc-app`). Windows (PowerShell): `$env:HF_POC_FOCUSABLE="0"; .\hf-poc-app.exe`. macOS: `HF_POC_FOCUSABLE=0 HoodFilesPoC.app/Contents/MacOS/hf-poc-app`. Repita N03–N06 e anote a diferença.

## 6. Enviar o resultado
Na janela de diagnóstico clique **Copiar relatório (JSON)** e cole, junto com a tabela acima preenchida, num comentário do PR #1 (ou em `docs/evidence/hf-00/<so>-<data>.json`). O relatório contém nomes de arquivos de teste, nunca caminhos completos.

**HF-00 permanece bloqueada** até estes testes serem executados e registrados nos **três** sistemas (Windows, macOS e Linux Mint).
