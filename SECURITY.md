# Política de segurança

O HoodFiles lida com arquivos do usuário, então falhas de segurança importam mesmo na fase de pré-lançamento.

## Como relatar

Use **[Report a vulnerability](https://github.com/michel-brotherhood/hoodfiles/security/advisories/new)** (aba *Security* do repositório). O relato fica privado até haver uma correção. Não abra issue pública para isso.

Inclua, se puder: o que acontece, como reproduzir, em qual sistema e versão, e o impacto que você enxerga.

## O que consideramos grave

Perda do arquivo original, sobrescrita silenciosa, escrita fora do destino autorizado, execução de comando arbitrário e envio de conteúdo, nomes ou caminhos de arquivos para fora da máquina. A lista completa de riscos e controles está em [`docs/security.md`](docs/security.md).

## Versões

Ainda não há versão publicada do aplicativo. Os pacotes de teste gerados pelo CI não são assinados e não devem ser usados com arquivos importantes.
