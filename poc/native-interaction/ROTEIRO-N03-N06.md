# Roteiro HF-00 — N03 a N06 (build de teste SEM assinatura)

Este build **não é assinado nem notarizado** e não é o produto. Serve só para verificar, num computador real, se o fluxo central funciona. Compilar no CI **não** prova isso: só você, arrastando de verdade, prova.

## 1. Baixar
1. Abra o PR #1 do repositório → aba **Checks** → workflow **HF-00 PoC** → seção **Artifacts** (precisa estar logado com acesso ao repositório privado).
2. Baixe o artifact da sua plataforma (`...windows-x64-SEM-ASSINATURA...` ou `...macos-arm64-SEM-ASSINATURA...`). Vem em `.zip`; descompacte.
3. Confira o hash em `SHA256SUMS.txt` (opcional).

## 2. Executar
- **Windows 10/11 x64:** execute `hf-poc-app.exe`. O SmartScreen avisará (não assinado): *Mais informações → Executar assim mesmo*. Windows 10 precisa do WebView2 Runtime.
- **macOS (Apple silicon):** dentro do `.zip` há `HoodFilesPoC-macos-arm64.tar.gz`. No Terminal: `tar -xzf HoodFilesPoC-macos-arm64.tar.gz && xattr -dr com.apple.quarantine HoodFilesPoC.app && open HoodFilesPoC.app`. Se o macOS bloquear, clique com o botão direito no app → *Abrir*. Anote qualquer pedido de permissão.

Abre a janela **diagnóstico**. Deixe-a visível, fora da área onde o overlay aparecerá (ele surge perto do cursor).
Atalho: **Ctrl+Alt+Shift+D** (Windows) / **Cmd+Option+Shift+D** (macOS). Confirme na tabela que o evento `startup` mostra `"registered": true`; se for `false`, anote `register_error` e pare: N03 falha.

## 3. Arquivos de teste
Rode `make-fixtures.ps1` (Windows: `powershell -ExecutionPolicy Bypass -File make-fixtures.ps1`) ou `./make-fixtures.sh` (macOS). Cria `hf-poc-fixtures` na Área de Trabalho.

## 4. Testes (repita cada um 3 vezes; anote uma linha por tentativa)
**Antes de começar:** anote SO/versão, CPU, monitores/escala.

| ID | Faça | Observe | Resultado |
| --- | --- | --- | --- |
| N03 | No Explorer/Finder, **segure o botão do mouse** sobre `simples.txt` e mova ~30 px (o arraste começou). Com o botão ainda pressionado, aperte o atalho. | Aparece o overlay azul “Solte aqui” perto do cursor? Surge `overlay_shown` na tabela? | |
| N04 | Sem soltar o botão, leve o cursor até o overlay. | O ícone do arquivo continua preso ao cursor (arraste não foi cancelado)? A janela ativa do Explorer/Finder continua sendo a de origem? | |
| N05 | Ainda sem soltar, passe sobre o overlay. | Surge `drag_enter` com a **quantidade e os nomes** corretos? (Repita com `lote-25` selecionado inteiro: 25.) | |
| N06 | Solte sobre o overlay. | Surge `drop_accepted` com `accepted` e IDs? O overlay fecha? O arquivo original continua na pasta, intacto? | |

Pare e anote exatamente o ponto se algo falhar (ex.: “atalho não disparou com o botão pressionado”, “arraste cancelou ao aparecer o overlay”, “overlay apareceu mas `drag_enter` não veio”).

## 5. Variantes (só se N03 ou N04 falharem)
No Terminal/PowerShell, defina antes de abrir o executável: `HF_POC_FOCUSABLE=0`. Windows (PowerShell): `$env:HF_POC_FOCUSABLE="0"; .\hf-poc-app.exe`. macOS: `HF_POC_FOCUSABLE=0 HoodFilesPoC.app/Contents/MacOS/hf-poc-app`. Repita N03–N06 e anote a diferença.

## 6. Enviar o resultado
Na janela de diagnóstico clique **Copiar relatório (JSON)** e cole, junto com a tabela acima preenchida, num comentário do PR #1 (ou em `docs/evidence/hf-00/<so>-<data>.json`). O relatório contém nomes de arquivos de teste, nunca caminhos completos.

**HF-00 permanece bloqueada** até estes testes serem executados e registrados nos dois sistemas.
