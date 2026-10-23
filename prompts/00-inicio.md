# Primeiro prompt para o Claude Code

Você vai desenvolver o HoodFiles, aplicativo desktop para Windows, macOS e Linux Mint inspirado no fluxo de conversão local e arraste do Tangerine. O código, a interface, os textos e os ativos devem ser próprios.

Leia README.md, CLAUDE.md, docs/spec.md, docs/architecture.md, docs/roadmap.md, docs/security.md, docs/testing.md e docs/status.md. Consulte docs/decisions/0001-stack.md antes de escolher a stack.

Nesta primeira execução, trabalhe somente na HF-00, prova de integração nativa. Não crie o produto completo nem substitua a experiência principal por uma página com upload.

1. Inspecione o diretório, Git e ferramentas sem apagar ou sobrescrever arquivos existentes. Identifique o sistema disponível e o que falta para testar Windows, macOS e Linux Mint (versão do Mint e tipo de sessão gráfica).
2. Resuma o produto e apresente um plano curto da prova, riscos e critérios de sucesso. Prossiga com investigação e trabalho local reversível autorizado por este pedido.
3. Consulte documentação oficial para o mecanismo de arraste externo e menu flutuante. Separe suporte confirmado de hipóteses. Tauri é candidato inicial; comparar Electron somente com motivo técnico.
4. Implemente a menor prova possível, numa pasta poc/native-interaction, com contratos separados por plataforma. Ainda sem engines de conversão.
5. Teste o que o ambiente realmente permite. Documente como reproduzir nos outros sistemas; não declare sucesso nativo com testes em navegador ou mocks.
6. Registre relatório em docs/poc-native-interaction.md, resultados em docs/status.md e atualize a ADR quando houver evidência suficiente.

A prova deve avaliar arraste desde Explorer/Finder, ativação por tecla, overlay perto do cursor, recebimento real de caminhos autorizados, Escape, lote, DPI e múltiplos monitores. Não exija permissões amplas sem demonstrar necessidade.

Se faltarem máquinas, avance em pesquisa e componentes portáveis, marque o gate como bloqueado e liste exatamente o teste que o responsável precisa executar. Se a experiência precisar mudar, apresente a decisão de produto necessária. Não invente equivalência funcional.

Ao encerrar, entregue: alterações, comandos reais e resultados, limitações, estado por plataforma e próximo passo. Não faça push, release, contrate serviços nem publique arquivos externamente.
