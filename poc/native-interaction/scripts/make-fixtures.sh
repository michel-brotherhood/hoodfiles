#!/usr/bin/env bash
# Gera fixtures vazias/pequenas e próprias para os testes manuais (macOS/Linux).
# Uso: scripts/make-fixtures.sh [DIRETÓRIO]   (padrão: ~/Desktop/hf-poc-fixtures)
set -euo pipefail
D="${1:-$HOME/Desktop/hf-poc-fixtures}"
mkdir -p "$D/lote-25" "$D/pasta com espaço"
printf 'hf-poc' > "$D/simples.txt"
printf 'hf-poc' > "$D/foto café 😀.png"
printf 'hf-poc' > "$D/日本語 b'c\"d;&.txt"
printf 'hf-poc' > "$D/nome-longo-$(printf 'x%.0s' $(seq 1 180)).txt"
printf 'hf-poc' > "$D/pasta com espaço/dentro.txt"
for i in $(seq -w 1 25); do printf 'hf-poc' > "$D/lote-25/arquivo-$i.txt"; done
ln -sf "$D/simples.txt" "$D/link-para-simples.txt"
echo "Fixtures em: $D"
