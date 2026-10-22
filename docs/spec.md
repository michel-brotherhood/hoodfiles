# Especificação do produto

## Problema e público

Profissionais que lidam com imagens, PDFs, mídias e documentos perdem tempo abrindo aplicações ou enviando arquivos para conversores online. HoodFiles permite operações locais rápidas a partir do gerenciamento cotidiano dos arquivos.

Público inicial proposto: designers, desenvolvedores, profissionais de marketing e usuários que convertem arquivos com frequência. A hipótese deve ser testada com usuários reais.

## Experiência pretendida

1. HoodFiles permanece disponível na bandeja do Windows ou barra de menus do Mac.
2. O usuário começa a arrastar um arquivo no Explorer ou Finder e aciona o atalho configurado.
3. Um menu flutuante surge próximo ao cursor, limitado às operações compatíveis. A prova técnica deve definir como receber os arquivos preservando a sessão de arraste.
4. O usuário escolhe uma operação e solta no alvo correspondente. Se necessário, uma tela pede opções adicionais antes de iniciar.
5. O resultado é salvo sem modificar o original, com aviso discreto e ação para revelar a saída.

Atalhos sugeridos para conversões, ferramentas e ações devem ser remapeáveis por sistema. Não capturar Shift, Control ou Command globalmente sem avaliar conflitos com cópia, movimento e seleção do próprio sistema. A semântica final depende de HF-00.

Um modo por janela, seleção de arquivos e uso de teclado também deve existir. Ele oferece acessibilidade e recuperação quando a integração global estiver indisponível; não comprova paridade com a experiência principal.

## Primeira versão funcional

1. Importar imagens por seletor ou arraste, identificar formatos suportados e listar operações reais.
2. Converter JPEG/PNG/WebP entre si, com comportamento de transparência e qualidade explícito.
3. Redimensionar proporcionalmente e comprimir com opções coerentes com o formato.
4. Processar lotes em fila, com progresso, cancelamento, falha por item e destinos seguros.
5. Disponibilizar acesso rápido e menu contextual conforme a integração validada em HF-00, além de preferências locais.

Essa entrega inicial não substitui a meta de expansão: vídeo, áudio, PDF, documentos, planilhas, apresentações, legendas e compactados serão adicionados por módulos.

## Telas e superfícies

| Superfície | Responsabilidade |
| --- | --- |
| Primeiro uso | Explicar fluxo, permissões justificadas e atalho |
| Menu flutuante | Conversões, ferramentas e ações compatíveis |
| Janela principal | Entrada, opções, fila e resultados |
| Configurações | Atalhos, destino, qualidade, tema e histórico |
| Sobre | Versão, licenças, diagnóstico e privacidade |

## Saídas

- Padrão proposto: salvar na pasta de origem com nome derivado e sufixo de operação; se não houver escrita, pedir outro destino.
- Em colisão, reservar novo nome de forma exclusiva. Exemplo: `foto.webp`, `foto (1).webp`.
- Sobrescrita não faz parte da primeira versão. Não apagar nem mover os originais após conversão.
- Gerar arquivo temporário no destino, validar e publicar por operação segura sem substituir arquivo existente.
- Cancelar limpa temporários pertencentes ao job e preserva resultados anteriores completos.

## Dados

Preferências em configuração local versionada. Fila em memória na primeira versão; ao reiniciar, avisar sobre tarefas interrompidas se houver journal mínimo, sem retomada automática prometida. Histórico opcional e desativado por padrão; não armazenar conteúdo dos arquivos. SQLite somente se consultas, persistência da fila ou migrações justificarem sua adoção.

## Fora da primeira entrega

Login, nuvem, colaboração, editor completo, conversões por IA, OCR, cobrança, loja de plugins, monitoramento de pastas, promessa de 275 operações, atualização silenciosa e suporte a Linux como plataforma de produto.

## Sucesso

Proposta para beta: cinco usuários concluem dez operações reais cada, sem perda de originais; pelo menos quatro conseguem repetir o fluxo principal sem ajuda. Registrar tempo para concluir tarefa e erros observados. Não tratar esses números como resultados já obtidos.
