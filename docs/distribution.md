# Distribuição e manutenção

## Matriz inicial proposta

| Destino | Artefato | Condição |
| --- | --- | --- |
| Windows x64 | Instalador suportado pela stack escolhida | Versão mínima a fixar; validar dependência WebView se Tauri |
| macOS Apple silicon | Aplicativo e DMG | Versão mínima a fixar; assinatura/notarização para distribuição pública conforme canal |
| macOS Intel | Artefato separado ou universal | Somente após validar engines e build |
| Windows ARM | Artefato específico | Investigação posterior |

## CI

Build por sistema/arquitetura, dependências fixas, caches controlados e testes adequados. Pull requests executam checks sem credenciais de assinatura. Releases usam jobs protegidos, artefatos identificados e segredos restritos.

Não baixar binários de URL mutável sem hash e procedência. Embarcar engines e dependências de runtime necessárias ao funcionamento offline; quando um instalador depender de download, declarar claramente e testar o cenário sem internet.

## Release

- [ ] Versão e changelog consistentes.
- [ ] Builds e testes dos sistemas incluídos concluídos.
- [ ] Catálogo só anuncia operações verificadas.
- [ ] Licenças, avisos, inventário de dependências e fontes exigidas presentes.
- [ ] Permissões e original preservado revisados.
- [ ] Instalador testado em conta comum e máquina limpa.
- [ ] Assinatura válida e notarização do Mac verificada conforme canal.
- [ ] Checksums e retenção de versão anterior definidos.
- [ ] Política de privacidade descreve processamento e acessos reais.
- [ ] Responsável autorizou publicação externa.

## Updates

Separar assinatura do instalador da assinatura do mecanismo de atualização. Validar integridade/autenticidade antes de instalar; nunca executar pacote só porque veio de HTTPS. Testar falha de download, versão inválida e aplicativo em conversão. Pedir para concluir/cancelar jobs antes de reiniciar. Manter distribuição manual até o mecanismo automático estar validado.

## Custos e serviços

Processamento local dispensa VPS para converter arquivos. Site de divulgação e downloads são componentes separados. Não criar serviços externos, contas pagas, login ou licenciamento por suposição. Certificados, conta Apple, runners e distribuição podem ter custos; levantar valores oficiais quando a decisão for necessária, sem valores presumidos neste plano.

Publicar em lojas pode mudar permissões e arquitetura; fazer avaliação própria antes de escolher esse canal. Não prometer que um build de distribuição direta satisfaz automaticamente as regras das lojas.
