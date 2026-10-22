# Arquitetura proposta

Status: desenho inicial sujeito à prova HF-00. A documentação não atesta suporte nativo já implementado.

## Camadas

| Componente | Responsabilidade | Limite |
| --- | --- | --- |
| UI | Menu, opções, fila, acessibilidade | Sem acesso arbitrário ao sistema |
| Serviço de aplicação | Validar operações, planejar jobs, comunicar progresso | Contratos tipados e versionados |
| Núcleo de arquivos | Identificação, destino, publicação segura | Originais sempre preservados |
| Engines | Conversões e inspeção | Processos/workers limitados |
| Adaptadores Windows/macOS | Drag/drop, atalhos, tray, revelar saída | APIs específicas por plataforma |
| Persistência | Preferências e registros mínimos | Local, sem conteúdos |

O núcleo deve ser reutilizável. A integração de arraste precisa de implementação e testes por sistema. Não assumir que um atalho global fornece acesso ao conteúdo do arraste de outra aplicação.

## Stack candidata

Tauri 2 com Rust para núcleo e integrações; Vite, TypeScript, HTML e CSS para interface sem framework. O backend aqui é o processo local do aplicativo, não um servidor remoto. Engines podem ser bibliotecas locais ou binários embarcados, escolhidos após testes de distribuição e licença.

Electron é alternativa de avaliação caso o candidato não atenda aos requisitos. Sua familiaridade com Node não elimina o trabalho de integração nativa. Qualquer escolha deve medir uso de recursos, tamanho final com engines e custo de manutenção.

## Estrutura depois da prova

```text
src/                    interface e contratos do cliente
src-tauri/src/app/      casos de uso e jobs
src-tauri/src/files/    identificação e destino seguro
src-tauri/src/engines/  adaptadores de conversão
src-tauri/src/platform/ windows e macos
src-tauri/capabilities/ permissões por janela
tests/fixtures/         arquivos pequenos próprios ou redistribuíveis
tests/integration/      engines e contratos
docs/                   especificação e evidências
```

Essa árvore é proposta; adaptar ao framework efetivamente escolhido, evitando módulos vazios para funcionalidades futuras.

## Contratos

- `inspectInputs`: recebe entradas autorizadas, retorna IDs opacos, tipo detectado e operações possíveis. Não confiar somente na extensão.
- `planOperation`: recebe IDs, operação permitida, parâmetros tipados e destino autorizado; retorna plano e avisos.
- `enqueue`: aceita plano validado, cria jobs e retorna IDs. Revalidar recursos no momento da execução.
- `cancelJob`: solicita cancelamento idempotente. Progresso e resultado via eventos por job.
- `revealOutput`: aceita ID de resultado existente; não um comando ou URL arbitrários.

Objetos de job: ID, ID do input, operação, opções validadas, destino reservado, estado, progresso opcional, erro estruturado e saída. Caminhos completos ficam no núcleo quando possível.

Estados: `queued`, `running`, `cancelling`, `cancelled`, `succeeded`, `failed`. Transições finais são únicas; eventos têm ID e sequência para evitar atualização fora de ordem. Sem percentual real, mostrar estado indeterminado.

## Execução e isolamento

Começar com concorrência 1 configurável internamente. Processamento pesado fora da thread da UI. Workers/processos separados para parsers complexos quando viável; processo separado não equivale a sandbox. Aplicar timeout, limite de dimensões/recursos, cancelamento de toda a árvore de processos e limpeza restrita ao job.

Para imagens, avaliar engine com cobertura comprovada dos três formatos iniciais. FFmpeg/ffprobe são candidatos para mídia; não presumir codecs disponíveis sem inspecionar a build distribuída. PDF e Office exigem decisão própria; não escolher motor só pela quantidade de extensões anunciadas.

## Configuração e diagnósticos

Configuração local com versão de schema e migrações pequenas. Histórico desligado por padrão. Logs locais rotativos com IDs de job e erros redigidos; relatório exportado pelo usuário deve passar por prévia. Sem telemetria de arquivos. Atualização por rede separada do processamento e desativável.
