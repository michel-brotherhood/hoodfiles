# HoodFiles — guia de desenvolvimento

Aplicativo desktop para Windows e macOS, inspirado no fluxo de conversão local do Tangerine. Este pacote orienta o Claude Code; ainda não contém o aplicativo implementado.

## Comece aqui

1. Extraia esta pasta no computador de desenvolvimento.
2. Abra o Claude Code dentro dela.
3. Cole o conteúdo de `prompts/00-inicio.md`.
4. Execute primeiro a etapa HF-00 de `docs/roadmap.md`. A escolha final da stack depende da prova de integração nativa.
5. Ao terminar uma tarefa, registre evidências em `docs/status.md` e retome pelo próximo item elegível.

O desenvolvimento pode começar no Linux Mint, mas a experiência nativa e os instaladores precisam de execução e validação no Windows e no macOS. CI não substitui o teste de arraste por uma pessoa nos sistemas de destino.

## Documentos

| Arquivo | Uso |
| --- | --- |
| `CLAUDE.md` | Regras permanentes para o agente |
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
| `prompts/00-inicio.md` | Primeiro comando para o Claude Code |
| `prompts/01-execucao.md` | Continuação de cada etapa |

## Decisões e premissas

- Confirmado: nome HoodFiles; Windows e macOS; experiência de referência Tangerine; execução pelo Claude Code.
- Direção adotada: processamento local, identidade própria e desenvolvimento incremental até a cobertura ampliada de formatos.
- Proposto: Tauri 2 + Rust + interface HTML/CSS/TypeScript sem framework; comparar com Electron na prova técnica quando houver impedimento concreto.
- Ainda em aberto: versões mínimas dos sistemas, suporte a Intel Mac e Windows ARM, monetização, distribuição em lojas e política de licenciamento do próprio produto.
- Não incluir autenticação, nuvem ou cobrança no núcleo inicial por pressuposição.

Não confunda planejamento com implementação: nenhum teste nativo ou conversor foi executado neste pacote. Todos os formatos listados são metas, até haver resultado registrado por plataforma.
