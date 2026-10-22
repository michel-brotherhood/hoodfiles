# HoodFiles

Responda em português. Leia `docs/spec.md`, `docs/roadmap.md` e `docs/status.md` antes de trabalhar. Consulte os demais documentos conforme a tarefa. Este arquivo contém instruções; não significa que a stack ou os comandos já existam.

## Objetivo

Criar um utilitário desktop de conversão e manipulação local de arquivos para Windows e macOS, com menu contextual rápido durante o arraste. Interface, marca, textos e ativos próprios. O Tangerine é uma referência funcional pública.

## Execução

- Comece por HF-00. Não implemente o produto inteiro num único pedido.
- Antes de cada tarefa, apresente um plano curto, arquivos envolvidos e validação. Prossiga nas tarefas autorizadas; peça decisão somente quando uma ambiguidade mudar o produto, houver ação destrutiva ou gasto/serviço externo novo.
- Não reduza a experiência global para um simples dropzone sem registrar a limitação e obter decisão do responsável.
- Escolha rotinas pequenas, contratos explícitos e dependências justificadas. Pesquise documentação oficial e fixe versões compatíveis nos manifests e lockfiles.
- Não invente APIs, resultados de comandos, métricas, suporte a formatos ou testes em sistemas indisponíveis.
- Ao finalizar: revisar o diff, executar checks relevantes e atualizar `docs/status.md` com evidências e próximo passo.
- Faça commits locais pequenos se houver repositório; não publique, faça push ou release sem instrução do responsável.

## Arquitetura candidata

Tauri 2, Rust, Vite e TypeScript sem framework. Decisão provisória em `docs/decisions/0001-stack.md`. Adaptadores nativos separados por sistema. Engines de conversão isoladas da interface. Não adicione servidor web, banco remoto, login ou API de IA ao fluxo de arquivos.

## Regras inegociáveis

- Preserve os originais. Colisões nunca causam sobrescrita silenciosa.
- Arquivos locais são dados não confiáveis. Valide no núcleo, mesmo se a interface já validou.
- Não exponha execução arbitrária de shell, caminhos ou argumentos livres por IPC.
- Não envie conteúdos, nomes ou caminhos completos de arquivos para serviços externos.
- Não peça administrador ou acessibilidade ampla sem necessidade comprovada e explicação específica.
- Separe operações rápidas e avançadas. Exiba apenas opções disponíveis para o arquivo e engine instalados.
- Sem comandos de instalação remota opacos ou dependências baixadas automaticamente em runtime sem política definida.
- Recursos de rede, assinatura e publicação exigem configuração explícita; segredo nunca entra no Git.

## Verificação

No bootstrap, crie e documente scripts reais para dev, lint, typecheck, test e build. Se Tauri for escolhido, inclua formatação, lint e testes Rust. Rode os comandos existentes; registre indisponibilidades. Testes de interface com mocks não comprovam Explorer, Finder, assinatura ou engines reais.

## Aprendizados

Adicione aqui somente regras curtas derivadas de falhas reais e recorrentes. Estado e logs ficam em `docs/status.md`.
