# Distribuição e manutenção

## Matriz inicial proposta

| Destino | Artefato | Condição |
| --- | --- | --- |
| Windows x64 | Instalador suportado pela stack escolhida | Versão mínima a fixar; validar dependência WebView se Tauri |
| macOS Apple silicon | Aplicativo e DMG | Versão mínima a fixar; assinatura/notarização para distribuição pública conforme canal |
| Linux Mint x86_64 | Pacote `.deb` (candidato principal; instala dependências via apt) ou AppImage; teste atual: `.tar.gz` sem instalação | Versão do Mint, sessão X11/Wayland e bibliotecas a fixar com testes reais |
| macOS Intel | Artefato separado ou universal | Somente após validar engines e build |
| Windows ARM | Artefato específico | Investigação posterior |

## Linux Mint: dependências e integridade

O app Tauri usa GTK3 e WebKitGTK 4.1 do sistema, não embarcados. O build de teste é feito em Ubuntu 24.04 (base do Mint 22.x) e depende de glibc compatível; Mint 21.x e LMDE não estão cobertos até haver build e teste próprios. Pacotes de runtime (Ubuntu 24.04 / Mint 22.x): `libwebkit2gtk-4.1-0`, `libjavascriptcoregtk-4.1-0`, `libgtk-3-0t64`, `libsoup-3.0-0`, `libdbus-1-3` (as demais vêm por dependência). Um `.deb` deve declarar essas dependências em `Depends`. Distribuição pública exige assinatura/checksum verificável (repositório apt assinado com GPG ou checksum publicado por canal confiável); decisão pendente. Não prometer que o build de uma versão do Mint funciona em outra.

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
- [ ] Linux Mint: pacote instalado e desinstalado em conta comum e máquina limpa; dependências declaradas; checksum/assinatura verificáveis.
- [ ] Checksums e retenção de versão anterior definidos.
- [ ] Política de privacidade descreve processamento e acessos reais.
- [ ] Responsável autorizou publicação externa.

## Updates

Separar assinatura do instalador da assinatura do mecanismo de atualização. Validar integridade/autenticidade antes de instalar; nunca executar pacote só porque veio de HTTPS. Testar falha de download, versão inválida e aplicativo em conversão. Pedir para concluir/cancelar jobs antes de reiniciar. Manter distribuição manual até o mecanismo automático estar validado.

## Custos e serviços

Processamento local dispensa VPS para converter arquivos. Site de divulgação e downloads são componentes separados. Não criar serviços externos, contas pagas, login ou licenciamento por suposição. Certificados, conta Apple, runners e distribuição podem ter custos; levantar valores oficiais quando a decisão for necessária, sem valores presumidos neste plano.

Publicar em lojas pode mudar permissões e arquitetura; fazer avaliação própria antes de escolher esse canal. Não prometer que um build de distribuição direta satisfaz automaticamente as regras das lojas.
