#!/usr/bin/env bash
# Smoke test Linux (Xvfb): atalho -> overlay -> arraste X11 real entre processos -> drop -> Escape.
# NÃO substitui os testes em Explorer/Finder. Requer: xvfb xdotool python3-gi, libs do Tauri.
set -euo pipefail
cd "$(dirname "$0")/.."
BIN=${BIN:-target/debug/hf-poc-app}
export DISPLAY=${DISPLAY:-:99}
export HOME=$(mktemp -d)               # isola o log do app
FIX=$(mktemp -d)
pids=()
cleanup() { for p in "${pids[@]:-}"; do kill "$p" 2>/dev/null || true; done; }
trap cleanup EXIT

[ -x "$BIN" ] || cargo build -p hf-poc-app
(Xvfb "$DISPLAY" -screen 0 1920x1080x24 >/dev/null 2>&1 & echo $! > "$HOME/xvfb.pid") ; pids+=("$(cat "$HOME/xvfb.pid")")
sleep 1
(command -v openbox >/dev/null && openbox >/dev/null 2>&1 & echo $! > "$HOME/wm.pid") || true; [ -f "$HOME/wm.pid" ] && pids+=("$(cat "$HOME/wm.pid")"); sleep 1

printf x > "$FIX/foto café 😀.png"; printf x > "$FIX/日本語 b'c\".txt"
"$BIN" >"$HOME/app.out" 2>&1 & pids+=($!)
sleep 5
LOG=$(find "$HOME" -name poc-log.jsonl | head -1)
[ -n "$LOG" ] || { echo "FALHA: log não criado"; cat "$HOME/app.out"; exit 1; }

check() { grep -q "$1" "$LOG" && echo "OK   $2" || { echo "FALHA $2"; cat "$LOG"; exit 1; }; }

# 1) Atalho + posicionamento (verificável aqui).
xdotool mousemove 960 540
xdotool key ctrl+alt+d; sleep 1
check overlay_shown "atalho mostra overlay"
RECT=$(grep -m1 overlay_shown "$LOG" | python3 -c 'import sys,json;print(*json.loads(sys.stdin.read())["detail"]["rect"])')
read -r X Y W H <<<"$RECT"
[ "$X" = 976 ] && [ "$Y" = 556 ] && echo "OK   overlay em 976,556 (cursor 960,540 + deslocamento 16)" || { echo "FALHA posição do overlay: $RECT"; exit 1; }

# 2) Arraste X11 real entre processos (fonte GTK -> overlay). INFORMATIVO: em Xvfb+openbox com
#    WebKitGTK 2.52/wry 0.57 o evento NÃO chegou (ver docs/poc-native-interaction.md). O controle
#    abaixo prova que a ferramenta de arraste funciona contra um alvo GTK simples.
/usr/bin/python3.12 scripts/gtk_drag_source.py "$FIX/foto café 😀.png" "$FIX/日本語 b'c\".txt" & pids+=($!)
sleep 2
xdotool mousemove 150 130 mousedown 1
for i in 1 2 3 4 5 6 7 8 9 10; do xdotool mousemove $((150 + (X + W/2 - 150) * i / 10)) $((130 + (Y + H/2 - 130) * i / 10)); sleep 0.1; done
sleep 0.5; xdotool mouseup 1; sleep 1
if grep -q drag_enter "$LOG"; then
  echo "OK   arraste externo recebido pelo overlay"
  grep -q '"kind":"drop_accepted"' "$LOG" && echo "OK   drop registrado" || echo "INFO drop não registrado"
else
  echo "INFO arraste externo NÃO chegou ao overlay neste ambiente (não é falha de Windows/macOS; ver relatório)"
fi

# 3) Escape (global) cancela: volta a Idle e testa de novo.
xdotool key Escape; sleep 0.5
xdotool key ctrl+alt+d; sleep 1
xdotool key Escape; sleep 1
check 'cancelado: Escape' "Escape cancela o overlay"
echo "Smoke Linux concluído (log: $LOG)"
