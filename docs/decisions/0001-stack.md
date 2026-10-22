# ADR-0001 — Stack desktop

Status: proposta, aguardando HF-00 (prova implementada; testes nativos Windows/macOS pendentes — ver `docs/poc-native-interaction.md`). Nenhuma evidência ainda a favor ou contra Tauri.

## Contexto

HoodFiles precisa operar localmente em Windows e macOS e oferecer integração durante o arraste. A experiência global é requisito central, cuja viabilidade deve ser demonstrada antes da fundação completa.

## Hipótese de decisão

Avaliar primeiro Tauri 2 + Rust + Vite/TypeScript sem framework. Motivos de projeto: núcleo local tipado, fronteira de permissões explícita e possibilidade de adaptadores nativos. Não existe neste pacote benchmark que comprove menor uso de recursos para este aplicativo.

## Alternativas

| Alternativa | Vantagem potencial | Custo a investigar |
| --- | --- | --- |
| Tauri | Núcleo Rust e permissões por superfície | FFI, WebView e ferramentas Rust |
| Electron | Ecossistema Node e UI Chromium | Distribuição, isolamento e recursos consumidos |
| Interfaces nativas separadas | Integração direta com cada SO | Duplicação de UI, builds e manutenção |

## Gate

Registrar arquitetura da máquina, versões do SO, mecanismo de drag/drop, permissões, conflitos, múltiplos monitores e evidências. Verificar interoperabilidade entre processos de níveis de privilégio diferentes no Windows. No Mac, documentar a necessidade real de permissões adicionais.

Comparar outra stack somente quando houver bloqueio ou vantagem demonstrável. Se nenhuma solução entregar a interação global, apresentar as alternativas e pedir decisão de produto; nunca marcar uma janela de drop como requisito global atendido.

## Consequências

Possível necessidade de código Swift/Objective-C ou C/C++/Rust via APIs nativas. Builds e testes por sistema são necessários. Versões mínimas e arquiteturas devem ser fixadas a partir das dependências e das máquinas efetivamente testadas.
