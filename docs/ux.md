# Experiência e interface

Identidade própria HoodFiles. Aparência proposta: limpa, compacta, tema claro/escuro e tipografia de sistema. Não existe identidade visual final aprovada; não copiar ícones, textos ou ativos do Tangerine.

## Menu contextual

Três grupos: Converter, Ferramentas e Ações. O menu radial é uma opção visual a provar, não uma obrigação quando prejudicar legibilidade e acessibilidade. Até seis escolhas visíveis por nível é uma hipótese inicial de design; testar com usuários. Opções avançadas abrem configuração breve.

Posicionar perto do cursor sem sair da área útil; respeitar monitores, escala e bordas. Não mover foco desnecessariamente nem prender arraste. Escape cancela sem produzir arquivo. Atalhos e mouse devem funcionar com o mesmo catálogo.

## Estados

- Vazio: explicar um gesto e oferecer seletor de arquivos.
- Inspeção: indicar que o arquivo está sendo identificado.
- Incompatível: explicar o tipo e oferecer apenas ações realmente disponíveis.
- Lote misto: mostrar quantos itens serão processados e quais não serão, antes de executar.
- Em execução: nome legível, operação, progresso real/indeterminado e cancelar.
- Sucesso: resultado e revelar na pasta; resumo discreto em lotes.
- Falha: causa útil e ação possível, preservando o original. Detalhes técnicos ficam em diagnóstico.

## Acessibilidade

Navegação por Tab e setas, foco visível, rótulos semânticos e uso sem depender da cor. Respeitar contraste, escala de texto e movimento reduzido. Testar com Narrator e VoiceOver nas máquinas-alvo. Não impedir acesso por teclado ao privilegiar apenas o gesto de arraste.

## Preferências

Destino, atalhos por grupo, qualidade padrão, tema e política de metadados. Inicialização com o sistema exige escolha explícita. Histórico desligado inicialmente. Permissão extra deve ter explicação, caminho para revogar e modo alternativo quando possível.

## Critérios visuais

Validar telas reais com textos longos, Unicode, 200% de escala e lotes com falhas. Não usar percentual artificial, notificação por arquivo em lote ou labels de implementação como mensagens principais. Primeiro uso demonstra o fluxo sem converter automaticamente arquivos do usuário.
