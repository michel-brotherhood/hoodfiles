#!/usr/bin/env bash
# Mostra o `state` REAL que o X11 entrega para o atalho e aponta bits que o global-hotkey 0.8.0 não cobre.
# Somente leitura: abre uma janela do xev (pacote x11-utils) por alguns segundos; nada é alterado.
# Uso: ./linux-key-probe.sh [segundos]   (padrão 25). Clique na janela do xev e pressione o atalho
#      (Ctrl+Alt+Shift+D) algumas vezes; repita com Scroll Lock ligado e depois desligado.
set -u
dur=${1:-25}
command -v xev >/dev/null || { echo "xev não encontrado (instale o pacote x11-utils)"; exit 1; }
tmp=$(mktemp); trap 'rm -f "$tmp"' EXIT

echo "== Teclado (somente leitura)"
if command -v setxkbmap >/dev/null; then setxkbmap -query | grep -E "layout|variant|options" || true; fi
if command -v xmodmap >/dev/null; then echo "-- modificadores:"; xmodmap -pm | grep -E "mod[0-9]|lock|shift|control" ; fi
if command -v xset >/dev/null; then echo "-- LEDs:"; xset q | grep -E "LED|Caps Lock|Scroll" || true; fi

echo
echo "== Abrindo o xev por ${dur}s. CLIQUE na janela e pressione o atalho algumas vezes."
timeout "$dur" xev -geometry 360x140+100+100 -event keyboard >"$tmp" 2>&1 || true

# bits do `state` (X11 core): 0x1 Shift, 0x2 Lock(Caps), 0x4 Control, 0x8 Mod1(Alt), 0x10 Mod2(NumLock),
# 0x20 Mod3, 0x40 Mod4(Super), 0x80 Mod5(AltGr/ISO_Level3), 0x2000/0x4000 = índice do grupo de layout (XKB).
# Cobertos pelos grabs do global-hotkey 0.8.0: o conjunto do atalho (Shift/Control/Mod1/Mod4) + ignorados Lock e Mod2.
covered=$(( 0x1 | 0x4 | 0x8 | 0x40 | 0x2 | 0x10 ))
names=(Shift Lock Control Mod1/Alt Mod2/NumLock Mod3 Mod4/Super Mod5/AltGr)

echo
echo "== KeyPress de teclas que não são modificadoras"
n=0
while IFS='|' read -r st kc; do
  n=$((n+1)); st=$((st))      # "0x2d" -> número (bash aceita prefixo 0x; mawk não tem strtonum)
  extra=$(( st & 0xff & ~covered ))
  group=$(( (st >> 13) & 3 ))
  bits=""
  for i in 0 1 2 3 4 5 6 7; do [ $(( (st >> i) & 1 )) -eq 1 ] && bits="$bits ${names[$i]}"; done
  flag="OK (todos os bits cobertos)"
  [ "$extra" -ne 0 ] && flag="ATENÇÃO: bits NÃO cobertos pelo global-hotkey: $(printf '0x%x' "$extra")"
  printf 'state=0x%-6x teclas:%s | grupo=%d | keycode %s -> %s\n' "$st" "${bits:- (nenhum)}" "$group" "$kc" "$flag"
done < <(awk '
  /^KeyPress event/ {inpress=1}
  inpress && /state 0x/ {match($0,/state 0x[0-9a-f]+/); st=substr($0,RSTART+6,RLENGTH-6)}
  inpress && /keycode/ {match($0,/keycode [0-9]+ \(keysym 0x[0-9a-f]+, [^)]*\)/); kc=substr($0,RSTART+8,RLENGTH-8)
     if (kc !~ /Shift|Control|Alt|Meta|Super|Hyper|Lock|ISO_/) { printf "%s|%s\n", st, kc }; inpress=0}
' "$tmp" 2>/dev/null)
[ "$n" -eq 0 ] && echo "(nenhuma tecla comum capturada: clique na janela do xev e pressione o atalho)"
echo
echo "Envie esta saída inteira no PR. 'ATENÇÃO' indica um modificador extra (ex.: Scroll Lock em Mod3) que impede o grab do atalho."
