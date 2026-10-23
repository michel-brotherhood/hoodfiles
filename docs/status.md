# Estado do HoodFiles

## Situação atual

Planejamento inicial criado. Código do aplicativo, prova técnica, engines, instaladores e testes nativos ainda não existem neste pacote.

| Etapa | Estado |
| --- | --- |
| HF-00 | Em andamento — prova implementada e testada só na lógica portátil e no Linux; **gate BLOQUEADO**: testes nativos Windows, macOS e Linux Mint PENDENTES (o gate exige os três) (`docs/poc-native-interaction.md`, seção 5) |
| HF-01 a HF-11 | Planejadas; dependências não concluídas |

Próxima tarefa: o responsável executa a matriz N01–N20 em Windows x64 e macOS Apple silicon (`poc/native-interaction/README.md`) e anexa as evidências; só então fechar HF-00.

## Questões que precisam de evidência

- Há acesso a Windows, macOS e Linux Mint para a prova? Registrar máquinas, versão do Mint e tipo de sessão (X11/Wayland).
- Qual mecanismo permite o fluxo global sem quebrar o arraste e sem privilégios excessivos?
- Quais versões mínimas e arquiteturas terão engines e testes completos?

## Registro de execução

Preencher a cada tarefa, mantendo o resumo atual no início:

```text
Tarefa:
Estado: em andamento / concluída / bloqueada
Alterações:
Decisões e evidências:
Comandos executados e resultados:
Testes nativos por SO:
Não executado e motivo:
Riscos pendentes:
Próxima tarefa elegível:
```

Não marcar uma etapa como concluída somente porque gerou código. Não usar o resultado dos checks deste pacote documental como resultado de teste do software.

### 2026-10-22 — HF-00 (parcial)

```text
Tarefa: HF-00 prova de integração nativa
Estado: em andamento (gate bloqueado por falta de testes nativos)
Alterações: tarball extraído na raiz; poc/native-interaction/ (core + app Tauri 2.12.5), docs/poc-native-interaction.md, docs/evidence/hf-00/
Decisões e evidências: ADR-0001 permanece "proposta"; sem bloqueio para Tauri demonstrado nem evidência de que o fluxo funciona. Hipótese central (atalho durante arraste + overlay sem cancelar a origem) NÃO validada.
Comandos executados e resultados (Linux x86_64, container):
  cargo test --workspace -> 18 passaram
  cargo fmt --check; cargo clippy -D warnings -> limpos
  cargo build -p hf-poc-app -> ok
  cargo check --target x86_64-pc-windows-msvc -> ok (type-check apenas)
  cargo check --target aarch64-apple-darwin -> FALHOU no build script de objc2-exception-helper (sem toolchain Apple); platform/macos.rs nunca compilado
  scripts/linux-smoke.sh -> atalho/overlay/posição/Escape ok; arraste X11 externo NÃO chegou ao overlay (causa não resolvida; Linux não é alvo)
Testes nativos por SO: Windows PENDENTE; macOS PENDENTE
Não executado e motivo: matriz N01-N20 (sem máquinas Windows/macOS); compilação macOS (sem SDK); comparação com Electron (sem motivo técnico)
Riscos pendentes: R1-R6 em docs/poc-native-interaction.md; Escape global pode conflitar com o cancelamento do arraste; arquivos virtuais sem caminho
Próxima tarefa elegível: executar a matriz nos dois SOs; HF-01 depende disso
```

### 2026-10-23 — HF-00: CI e builds de teste

```text
Tarefa: CI nativo + artifacts + roteiro N03-N06
Estado: em andamento (gate continua BLOQUEADO)
Alterações: .github/workflows/hf-00-poc.yml; scripts/package-ci.sh; ROTEIRO-N03-N06.md; atalho padrão Ctrl/Cmd+Alt+Shift+D (Cmd+Option+D esconde o Dock no macOS); correções de testes do núcleo no Windows
Comandos e resultados: PR #1, run 3 (commit 1a3dd6d): Windows x64 (windows-2025) verde; macOS arm64 (macos-15) verde — fmt, clippy -D warnings, testes, build release, empacotamento. Runs 1-2 falharam no Windows por problemas reais nos testes (unused_mut; nome com aspas e caminho "/x" não absoluto), corrigidos.
Artifacts: hf-poc-windows-x64 e hf-poc-macos-arm64 (SEM ASSINATURA; 14 dias)
Testes nativos por SO: Windows PENDENTE; macOS PENDENTE (nenhum executável foi executado; CI não substitui)
Não executado e motivo: N01-N20 e N03-N06 (dependem de máquina real e pessoa arrastando)
Riscos pendentes: inalterados (R1-R6 em docs/poc-native-interaction.md); conflitos do novo atalho não verificados (N11)
Próxima tarefa elegível: responsável executa ROTEIRO-N03-N06.md em ambos os sistemas
```

### 2026-10-23 — HF-00: Linux Mint como plataforma oficial; investigação do arraste

```text
Tarefa: incluir Linux Mint; investigar arraste externo falho no Linux; build/CI Linux
Estado: em andamento (gate BLOQUEADO; exige Windows, macOS e Linux Mint)
Alterações: docs (spec, arquitetura, roadmap, testes, distribuição, ADR, README, CLAUDE.md) com Linux Mint oficial; platform/linux.rs registra distro/sessão/desktop; job Linux no CI (ubuntu-24.04) com smoke Xvfb; pacote linux-x64 com DEPENDENCIAS-RUNTIME.txt e linux-run-checks.sh; roteiro com seção Mint
Decisões e evidências: causa do defeito original CONFIRMADA = bug do app (on_webview_event em WebviewWindow nunca dispara; correto é on_window_event), não do wry/WebKitGTK; hipótese info==2 REFUTADA com wry instrumentado. Vale também para Windows/macOS: builds anteriores tinham o defeito. Intermitência residual (~8% com outra janela do app no caminho; ~0,6% fora) SEM causa isolada; GTK puro 0/40 não reproduziu. Nenhuma dependência trocada.
Comandos e resultados (container Ubuntu 24.04, Xvfb): ver docs/poc-native-interaction.md seção 4.1
Testes nativos por SO: Windows PENDENTE; macOS PENDENTE; Linux Mint PENDENTE (o container Ubuntu NÃO é Mint)
Versão do Mint e sessão gráfica usadas no teste: NÃO REGISTRADAS (nenhum teste em Mint foi feito); o app e o linux-run-checks.sh registram esses valores quando executados
Riscos pendentes: R1-R6; intermitência do overlay no Linux; Wayland não avaliado; Mint 21.x/LMDE não cobertos pelo build Ubuntu 24.04
Próxima tarefa elegível: responsável executa ROTEIRO-N03-N06.md nos três sistemas
```
