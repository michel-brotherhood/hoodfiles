# Segurança e integridade

## Fronteiras de confiança

Arquivos, metadados, nomes, caminhos e IPC são entradas não confiáveis. O processo local tem acesso aos arquivos do usuário: minimizar privilégio e superfície é essencial. Processamento local não elimina vulnerabilidades de parsers.

| Risco | Controle necessário | Evidência |
| --- | --- | --- |
| Comando injetado via nome | Executável fixo e argumentos estruturados; sem shell | Fixture com espaços, aspas e caracteres especiais |
| Sobrescrita e colisão | Criação exclusiva, reserva e publicação sem replace | Dois jobs disputam mesmo nome |
| Links e troca de caminhos | Política explícita de links e validação próxima à abertura; APIs seguras | Symlink/reparse point e corrida controlada |
| Parser vulnerável | Engines atualizadas, limites e isolamento quando possível | Entradas truncadas e inválidas |
| Consumo excessivo | Limites de memória, dimensões, tempo, páginas e concorrência | Imagem de dimensões excessivas e timeout |
| Travessia em compactados | Validar caminho final, links e cada entrada | ZIP com `../`, absoluto e links |
| IPC privilegiado | Comandos específicos, validação no núcleo e escopo por janela | Chamada inválida não cria processo nem arquivo |
| Exfiltração | Sem upload no fluxo, recursos locais e rede separada | Conversão em ambiente com rede bloqueada e inspeção de tráfego |
| Update comprometido | Assinatura, canal confiável e verificação antes de executar | Pacote adulterado recusado |

## Regras de implementação

- Preferir usuário comum. Não exigir execução como administrador para operar arquivos comuns.
- Não conceder filesystem/shell irrestritos à interface. No Tauri, capacidades mínimas por janela; comandos personalizados também validam autorização. No Electron, revisar isolamento de contexto, sandbox, preload mínimo, IPC e conteúdo remoto conforme documentação oficial.
- Nunca carregar conteúdo ativo de documentos dentro da UI privilegiada. SVG/HTML devem ser sanitizados/renderizados sem rede e scripts, ou ficar indisponíveis até haver tratamento seguro.
- Subprocessos recebem ambiente mínimo necessário, diretório de trabalho controlado e executável conhecido. Não procurar executável arbitrário no diretório do arquivo.
- Temporários por job em diretório controlado; limpeza não segue links nem remove caminhos fornecidos pelo frontend. Crash recovery limpa somente itens comprovadamente pertencentes ao app.
- Operações destrutivas de mover/remover não entram como efeito implícito de conversão.
- Logs não contêm conteúdos, senhas de documentos ou caminhos completos por padrão. Secret de assinatura é restrito ao CI de release e nunca disponível em builds de pull requests externos.

## Dependências e licenças

Fixar versões, manter lockfiles e inventário de bibliotecas/binários, hashes e fontes de download. Verificar advisories atuais antes do release e registrar o que foi analisado. Scanner sem achados não substitui revisão.

FFmpeg muda de obrigações conforme a configuração da build. Registrar flags, codecs, licença efetiva, atribuições e fontes correspondentes antes de distribuir. Não assumir que executar por subprocesso resolve todas as obrigações. Revisar também engines PDF/Office e codecs adicionados.

## Definição de bloqueador

Perda de original, sobrescrita silenciosa, execução de comando arbitrário, escrita fora do destino autorizado e vazamento de conteúdos bloqueiam o release. Registrar severidade, reprodução e correção; não reclassificar sem evidência para avançar uma etapa.
