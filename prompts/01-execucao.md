# Continuar uma etapa

Leia CLAUDE.md e docs/status.md. Identifique a próxima tarefa com dependências concluídas em docs/roadmap.md. Se a tarefa anterior estiver bloqueada, não pule seu gate silenciosamente.

Apresente em poucas linhas objetivo, arquivos e validação. Implemente apenas essa tarefa e seus requisitos de segurança. Escolhas internas reversíveis podem ser resolvidas com justificativa; ambiguidade que altere o produto ou adicione gastos deve ser apresentada ao responsável.

Use documentação oficial e dependências compatíveis com versões fixadas. Verifique entradas no núcleo, preserve originais e trate falhas por item. Não adicione recursos não previstos para aparentar completude.

Execute checks adequados ao diff e testes reais de integração quando a tarefa envolver engines ou filesystem. Testes por SO devem ter evidências separadas. Revise o diff final para segurança, perda de dados, condições de corrida e regressões.

Atualize docs/status.md com o resultado verdadeiro. Finalize informando o que mudou, como foi verificado, o que não pôde ser testado e a próxima tarefa elegível. Não faça push nem publicação externa.
