# Plano de execução

Execute na ordem das dependências. Cada etapa termina com diff revisado, checks relevantes e atualização de status. Etapas de expansão podem ser planejadas em paralelo conceitualmente, mas não devem criar complexidade antecipada no núcleo.

## HF-00 — Viabilidade da experiência nativa

Dependência: nenhuma. Criar prova mínima, separada do código de produção, para Windows, macOS e Linux Mint.

- Inventariar ambiente e acesso às máquinas dos três sistemas (no Mint: versão e tipo de sessão gráfica, X11 ou Wayland).
- Capturar entrada por arraste normal e investigar ativação durante arraste externo.
- Validar overlay, drop target, foco, cancelamento e retorno ao aplicativo de origem.
- Testar atalho sem disparar ações indesejadas do sistema.
- Testar seleção múltipla, nomes Unicode, múltiplos monitores, DPI/Retina e saída pelo Escape.
- Não executar conversão ainda; mostrar somente os arquivos realmente recebidos.

Aceite: evidência gravada ou roteiro reproduzível com resultados nos três sistemas (Windows, macOS e Linux Mint), código da prova e relatório de limitações. Comparar Tauri/Electron apenas se necessário; fechar ADR-0001 com decisão fundamentada. Um sistema não disponível ou não testado fica bloqueado, não aprovado; o gate só fecha com os três validados. Pode continuar investigação e lógica portátil, sem declarar o gate concluído.

## HF-01 — Fundação

Depende de HF-00 aprovado. Criar projeto, scripts reais, lockfiles, lint, checks de tipos, testes e build. Implementar tray/menu bar, instância única, janela principal e configuração local. Definir versões mínimas e arquiteturas suportadas no ADR.

Aceite: app abre e fecha corretamente nos três sistemas; scripts documentados executam; sair encerra workers. Não iniciar no login sem consentimento nas preferências.

## HF-02 — Núcleo seguro de arquivos

Depende de HF-01. Implementar inspeção, IDs autorizados, catálogo de operações, planejamento de saída, temporários e publicação sem sobrescrita. Usar arquivos de teste pequenos.

Aceite: colisões concorrentes não sobrescrevem arquivos; falta de espaço/permissão produz erro útil; links e troca de caminhos não ampliam acesso. Hash do original permanece igual.

## HF-03 — Conversão de imagens

Depende de HF-02. Escolher engine após testar JPEG/PNG/WebP nas arquiteturas-alvo. Implementar conversão, redimensionamento e compressão. Definir transparência, orientação, perfil de cor e metadados.

Aceite: saídas decodificam em leitor independente, dimensões e formato são verificados, imagem com alfa recebe política explícita ao virar JPEG; arquivos inválidos falham sem encerrar o app.

## HF-04 — Fila e lotes

Depende de HF-03. Implementar estados, progresso real quando disponível, cancelamento por item/lote e processamento limitado. Lote misto exibe itens compatíveis e exige escolha explícita antes de pular os demais.

Aceite: erro em um job não cancela outros por acidente; cancelar impede publicação de resultado incompleto; fechar app com jobs ativos pede uma decisão; processo filho não fica órfão.

## HF-05 — Experiência rápida integrada

Depende de HF-04 e mecanismo aprovado em HF-00. Integrar menu contextual com catálogo real, atalhos remapeáveis, teclado e preferências. Testar foco, escala e desempenho com UI aberta/oculta.

Aceite: repetir fluxo real desde Explorer/Finder/Nemo até saída; menu não contém opções falsas; cancelamento devolve controle; leitor de tela e teclado têm percurso utilizável.

## HF-06 — Beta de imagens

Depende de HF-05. Empacotar builds internos e executar matriz de testes em máquinas limpas. Revisar superfície IPC, permissões, licenças e original preservado. Conduzir teste com usuários proposto na spec.

Aceite: evidências dos sistemas, riscos residuais registrados e nenhum bloqueador aberto de perda de dados, execução arbitrária ou travamento recorrente. Artefato interno sem assinatura deve ser identificado como tal; não é release público aprovado.

## HF-07 — Áudio e vídeo

Depende de HF-06. Provar distribuição de FFmpeg/ffprobe ou alternativa, codecs e licença da build. Adicionar transcodificação, extração de áudio, corte e compressão como operações distintas. Corte por cópia e corte com recodificação têm limitações diferentes, que a UI deve explicar.

Aceite: inspecionar codec/container/duração, reproduzir em leitor independente, cancelar processo completo, medir CPU/memória e testar offline sem downloads ocultos.

## HF-08 — PDF

Depende de HF-06. Selecionar motor. Implementar primeiro juntar, dividir e renderizar páginas. PDF para formatos editáveis e OCR exigem prova separada de fidelidade e licença; não bloquear os utilitários básicos por essas funções.

Aceite: contagem de páginas correta, ordem preservada e revisão visual de saídas; documento protegido recebe fluxo explícito, sem tentativa de contornar proteção.

## HF-09 — Documentos, planilhas e slides

Depende de HF-06; utilizar integração PDF quando necessário. Investigar conversão local sem presumir Microsoft Office instalado. Motor Office headless, se escolhido, requer teste de fontes, processos, tamanho de distribuição e licenças.

Aceite: matriz por operação com fontes/tabelas/fórmulas/slides reais; informar diferenças e perda de editabilidade. Não prometer fórmulas em CSV ou reconstrução perfeita de PDF.

## HF-10 — Legendas, arquivos e ações

Depende de HF-06. Legendas SRT/VTT; ZIP/TAR/GZIP após proteção contra travessia de caminhos e expansão excessiva. Ações de copiar caminho e revelar saída primeiro; mover/copiar e integrações adicionais depois de definir conflitos, permissões e comportamento entre volumes.

Aceite: extração não escreve fora do destino; limites interrompem expansão; ações usam APIs seguras e confirmam efeitos materiais. AirDrop é específico de macOS; não apresentar suporte equivalente no Windows sem implementação real.

## HF-11 — Release ampliado

Depende dos módulos efetivamente incluídos na versão. Consolidar catálogo comprovado, auditoria, instaladores assinados, notarização do Mac quando aplicável, checksum, licenças e documentação. Validar update assinado e recuperação antes de ativar atualização automática.

Aceite: checklist de distribuição completo. Publicação externa e despesas dependem de instrução explícita do responsável. Marcar módulos não entregues como planejados, sem adiar indefinidamente uma versão utilizável já validada.
