# Catálogo planejado

Estado inicial de todas as linhas: PLANEJADO, NÃO TESTADO. Não significa que todo par de extensões será convertível. O catálogo do aplicativo deve ser gerado por operações implementadas, engines disponíveis e resultados por plataforma.

| Fase | Entradas candidatas | Saídas/operações | Validação especial |
| --- | --- | --- | --- |
| Imagens inicial | JPEG, PNG, WebP estáticos | Converter entre os três, redimensionar, comprimir | Alfa, orientação, ICC e metadados |
| Imagens ampliado | HEIC, AVIF, TIFF, BMP | Pares aprovados após teste | Codecs, múltiplas páginas, profundidade e licença |
| Vetor | SVG | Renderização para imagem após prova | Bloquear scripts, rede e referências externas |
| Áudio | MP3, WAV, FLAC, M4A, OGG/Opus | Pares conforme codecs da build | Codec não é container; preservar duração |
| Vídeo | MP4, MOV, MKV, WebM, AVI | Transcodificar, comprimir, cortar, extrair áudio | Codecs presentes, sincronização e keyframes |
| Animação | GIF e WebP animado | Somente operações explicitamente implementadas | Não descartar quadros silenciosamente |
| PDF básico | PDF | Juntar, dividir, páginas para PNG/JPEG | Número/ordem de páginas e DPI |
| PDF avançado | PDF digitalizado ou digital | Texto, OCR e formatos editáveis após PoC | Fidelidade, idioma e layout |
| Documentos | DOCX, ODT, TXT, Markdown, EPUB | Pares e exportação PDF após PoC | Fontes, mídia embutida e estilos |
| Planilhas | XLSX, ODS, CSV, TSV | Pares e PDF após PoC | Fórmulas, múltiplas abas, delimitador e locale |
| Apresentações | PPTX; PPT legado após prova | PDF e imagens de slides | Fontes, notas, animações e proporção |
| Legendas | SRT, VTT | Converter e exportar texto | Encoding, timestamps e perda de estilos |
| Compactados | ZIP, TAR, GZIP | Criar/extrair conforme engine | Travessia, links e expansão excessiva |

PAGES, RTF/RTFD, XLS antigo, RAR, WMA e WMV permanecem investigação posterior. Compatibilidade do Tangerine não implica compatibilidade do HoodFiles.

## Registro por operação

Manter identificador, entradas, saídas, parâmetros, engine/versão, SO/arquitetura, preservação e perda de dados, arquivos de teste, limites e status. Use `planned`, `experimental`, `verified` e `unavailable`. Só `verified` aparece como suporte estável.

## Políticas iniciais

- Não alegar compressão sempre menor. Se a saída crescer, informar tamanhos reais.
- Redimensionar preserva proporção; ampliar exige opção explícita.
- Conversão para JPEG solicita ou mostra a cor de fundo usada para transparência.
- Normalizar orientação visual antes de remover EXIF de orientação.
- Tratar cor de forma explícita: preservar perfil quando possível ou converter para espaço definido; não descartar ICC sem considerar efeito visual.
- Metadados sensíveis: remover por padrão na exportação inicial, informando isso; política por engine deve registrar exatamente o que é removido ou preservado.
- Limites iniciais de dimensões, páginas, tamanho e tempo são configurados após medição, com valores registrados; não aceitar entradas ilimitadas.
