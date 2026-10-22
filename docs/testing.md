# Plano de testes

Este pacote foi revisado como documentação; nenhum app foi implementado ou testado. A matriz abaixo será preenchida pelo Claude e pelo responsável nas máquinas de destino.

## Camadas de verificação

1. Unidade: catálogo, parâmetros, estados de jobs, nomes de saída e erros.
2. Integração real: engines, filesystem temporário, publicação sem sobrescrita e cancelamento de processos.
3. UI: foco, estados, teclado, mensagens e configuração; mocks limitados ao contrato.
4. Nativo: arraste externo, atalhos, permissões, tray/menu bar, escala e múltiplos monitores.
5. Distribuição: instalar, abrir, converter offline, atualizar quando implementado e desinstalar em máquina limpa.

## Casos mínimos

| Área | Casos | Resultado esperado |
| --- | --- | --- |
| Caminhos | Espaços, acentos, emoji, nomes longos, rede e somente leitura | Processar ou informar limitação sem corrupção |
| Tipos | Extensão incorreta, arquivo vazio, truncado, tipo incompatível | Inspeção confiável e falha útil |
| Saída | Nome ocupado, duas conversões simultâneas, falta de espaço | Não sobrescrever; limpar incompletos |
| Imagem | Alfa, orientação EXIF, ICC e imagem animada | Política explícita, sem perda silenciosa |
| Lote | Mistura de formatos e um arquivo inválido | Falha individual e resumo correto |
| Cancelamento | Antes, durante e próximo da publicação | Estado final único e original intacto |
| Crash | Fechar worker/app durante conversão | Originais intactos e temporários recuperáveis |
| Segurança | IPC inválido, metacaracteres e links | Nenhuma ampliação de acesso |
| Offline | Rede bloqueada, engines presentes | Conversão local completa |
| UX nativa | Arraste em Explorer/Finder, Escape e atalhos conflitantes | Controle devolvido corretamente ao usuário |

## Fixtures e oráculos

Usar material próprio ou redistribuível, pequeno e com licença conhecida. Comparar hash do original antes/depois. Validar formato por assinatura/decoder, dimensões e propriedades por leitor independente. Compressão com perdas deve usar comparação visual e tolerância justificada; hash idêntico da saída não é requisito.

PDF exige renderizar páginas resultantes e revisar ordem/layout. Mídia exige inspeção e reprodução. Planilhas exigem conferir valores, fórmulas e abas conforme a operação.

## Ambiente

Registrar SO/build, CPU/arquitetura, RAM, engine e versão do aplicativo. Windows x64 e macOS Apple silicon são alvos iniciais propostos; versões mínimas serão fixadas na prova. Intel Mac e Windows ARM só entram após pipeline e engine comprovados. Linux pode testar lógica portátil; não comprova compatibilidade desktop nos alvos.

Não pressupor que Playwright no navegador cobre a WebView nativa. Escolher automação suportada pelo framework e plataforma; quando não existir, documentar teste manual reproduzível.

## Performance

Medir antes de fixar orçamento: memória ociosa/pico, CPU ociosa, tempo para abrir menu e tempo de conversão com fixtures documentadas. Reportar mediana e variação, hardware e número de repetições. Nenhum benchmark é fornecido neste pacote.

## Evidência por tarefa

Registrar comando, exit code, resumo, SO e limitações. Se teste não executou, escrever `NÃO EXECUTADO` e motivo. Teste mockado, build e teste nativo têm resultados separados. Exigir execução dos checks pertinentes, sem criar testes que apenas reproduzam detalhes triviais da implementação.
