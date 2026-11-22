# Fontes e adaptação

Referências consultadas em 22/10/2026. São fontes sobre produto e capacidades dos frameworks; não comprovam o funcionamento do HoodFiles. Confirmar documentação e versões ao implementar.

## Guia fornecido

“Guia — Do zero ao ar com IA”, Rafael Dias, edição out/2026, enviado pelo responsável. Foram usados os princípios de especificação antes de implementar, contexto permanente em um documento de regras (`CONTRIBUTING.md`), tarefas pequenas, revisão, testes e preparação do lançamento. Este pacote é uma adaptação original; não redistribui o PDF.

| Tema do guia | Adaptação no HoodFiles |
| --- | --- |
| Ideia e validação | Público, hipótese e beta com operações reais |
| Spec e execução | Documentos, tarefas e roteiros de execução |
| Deploy web | Builds e instaladores por sistema |
| Banco/login | Configuração local; sem autenticação obrigatória |
| Segurança web | IPC, filesystem, engines e distribuição |
| Custos | Dependências, assinatura e distribuição |
| Lançamento | Beta e release com evidências |

Não herdar automaticamente Next.js, Vercel, Supabase, Firebase ou Resend do exemplo do guia: não são requisitos do produto desktop definido aqui.

## Documentação pública

- Tangerine: https://tangerineformac.com/ — referência de fluxo e categorias anunciadas; aplicativo não instalado ou testado nesta análise.
- Tauri, binários externos: https://v2.tauri.app/develop/sidecar/ — distribuição de sidecars por arquitetura; não equivale a prova de codecs ou licenças.
- Tauri, capacidades: https://v2.tauri.app/reference/acl/capability/ — escopo de acesso de janelas/webviews.
- Tauri, distribuição: https://v2.tauri.app/distribute/ — canais e empacotamento.
- Tauri, assinatura macOS: https://v2.tauri.app/distribute/sign/macos/ — assinatura/notarização conforme canal.
- Tauri, testes: https://v2.tauri.app/develop/tests/ — pontos de partida para testes; validar suporte por plataforma.
- Electron, native drag/drop: https://www.electronjs.org/docs/latest/tutorial/native-file-drag-drop — iniciar arraste de arquivos pelo app; não demonstra captura global do arraste do Finder/Explorer.
- Electron, segurança: https://www.electronjs.org/docs/latest/tutorial/security — isolamento e superfície privilegiada.
- FFmpeg, licença: https://ffmpeg.org/legal.html — obrigações variam com a configuração da build.

## Pendências de pesquisa na implementação

Ler APIs oficiais da Apple e Microsoft para o mecanismo nativo escolhido; reproduzir com sessão de arraste real. Selecionar e documentar motores de imagem/PDF/Office com fontes oficiais, versões, licenças e compatibilidade. Não usar este pacote como parecer jurídico de distribuição.
