# HoodFiles

Aplicativo desktop para Windows, macOS e Linux Mint, inspirado no fluxo de conversão local do Tangerine. O aplicativo ainda não está implementado: este repositório reúne a especificação, o plano e a prova de interação nativa (HF-00).

## Comece aqui

1. Leia `docs/spec.md` (produto), `docs/roadmap.md` (etapas) e `docs/status.md` (estado real).
2. Execute primeiro a etapa HF-00 de `docs/roadmap.md`: a prova de interação nativa está em `poc/native-interaction/` e o relatório em `docs/poc-native-interaction.md`. A escolha final da stack depende dela.
3. Ao terminar uma tarefa, registre evidências em `docs/status.md` e retome pelo próximo item elegível. Regras de trabalho em `CONTRIBUTING.md`.
4. Site do projeto (pré-lançamento): pasta `site/`, publicado com GitHub Pages (`docs/site.md`).

Linux Mint passou a ser plataforma oficial junto com Windows e macOS. Instaladores, arraste e atalho precisam de execução e validação nos três sistemas; CI e container Linux (Ubuntu/Xvfb) não substituem o teste de arraste por uma pessoa em cada sistema de destino.

## Documentos

| Arquivo | Uso |
| --- | --- |
| `CONTRIBUTING.md` | Regras permanentes do projeto e como trabalhar |
| `docs/spec.md` | Produto, fluxos e limites |
| `docs/architecture.md` | Componentes e contratos propostos |
| `docs/roadmap.md` | Sequência executável e critérios de aceite |
| `docs/formats.md` | Catálogo planejado de operações |
| `docs/ux.md` | Comportamento e qualidade da interface |
| `docs/security.md` | Proteção dos arquivos e do computador |
| `docs/testing.md` | Testes e evidências exigidas |
| `docs/distribution.md` | Builds, instaladores e atualizações |
| `docs/status.md` | Estado real e retomada |
| `docs/decisions/0001-stack.md` | Decisão técnica ainda proposta |
| `docs/sources.md` | Referências e adaptação do guia |
| `docs/poc-native-interaction.md` | Relatório e matriz de testes da prova HF-00 |
| `site/` | Site estático do projeto (GitHub Pages); ver `docs/site.md` |
| `design/` | Arte de identidade original e script que gera os assets do site |

## Decisões e premissas

- Confirmado: nome HoodFiles; Windows, macOS e Linux Mint; experiência de referência Tangerine.
- Direção adotada: processamento local, identidade própria e desenvolvimento incremental até a cobertura ampliada de formatos.
- Proposto: Tauri 2 + Rust + interface HTML/CSS/TypeScript sem framework; comparar com Electron na prova técnica quando houver impedimento concreto.
- Ainda em aberto: versões mínimas dos sistemas, suporte a Intel Mac e Windows ARM, monetização, distribuição em lojas e política de licenciamento do próprio produto.
- Não incluir autenticação, nuvem ou cobrança no núcleo inicial por pressuposição.

Não confunda planejamento com implementação: nenhum conversor foi implementado e os testes nativos seguem pendentes (ver `docs/status.md`). Todos os formatos listados são metas, até haver resultado registrado por plataforma.
