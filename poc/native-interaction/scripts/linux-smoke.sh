#!/usr/bin/env bash
# Smoke Linux (Xvfb + openbox + xdotool): atalho -> overlay -> arraste X11 entre processos -> drop -> Escape.
# NÃO substitui os testes em Nemo/Explorer/Finder nem em Linux Mint real. Requer: xvfb xdotool openbox python3-gi
# e as libs do Tauri. BIN=caminho do executável (padrão: debug; em CI: target/release/hf-poc-app).
#
# Por que reiniciar o app entre tentativas: a prova mediu instâncias "ruins" do overlay que não recebem
# o arraste em NENHUMA tentativa (agrupadas por instância; ver docs/poc-native-interaction.md, 4.1).
# Por que mover a janela de diagnóstico: com ela no caminho do arraste a falha foi ~8%; fora, ~0,6%.
set -euo pipefail
cd "$(dirname "$0")/.."
BIN=${BIN:-target/debug/hf-poc-app}
export DISPLAY=${DISPLAY:-:99}
export HOME=$(mktemp -d)               # isola o log do app
FIX=$(mktemp -d)
pids=(); APP_PID=""
cleanup() { for p in "${pids[@]:-}" "${APP_PID:-}"; do [ -n "$p" ] && kill "$p" 2>/dev/null || true; done; }
trap cleanup EXIT

[ -x "$BIN" ] || cargo build -p hf-poc-app
(Xvfb "$DISPLAY" -screen 0 1920x1080x24 >/dev/null 2>&1 & echo $! > "$HOME/xvfb.pid") ; pids+=("$(cat "$HOME/xvfb.pid")")
sleep 1
(command -v openbox >/dev/null && openbox >/dev/null 2>&1 & echo $! > "$HOME/wm.pid") || true; [ -f "$HOME/wm.pid" ] && pids+=("$(cat "$HOME/wm.pid")"); sleep 1

printf x > "$FIX/foto café 😀.png"; printf x > "$FIX/日本語 b'c\".txt"
LOG=""
check() { grep -q "$1" "$LOG" && echo "OK   $2" || { echo "FALHA $2"; cat "$LOG"; exit 1; }; }

start_app() {
  [ -n "$APP_PID" ] && { kill "$APP_PID" 2>/dev/null || true; sleep 1; }
  "$BIN" >"$HOME/app.out" 2>&1 & APP_PID=$!
  sleep 5
  LOG=$(find "$HOME" -name poc-log.jsonl | head -1)
  [ -n "$LOG" ] || { echo "FALHA: log não criado"; cat "$HOME/app.out"; exit 1; }
  # tira a janela de diagnóstico do caminho do arraste (continua aberta, fora da área do overlay)
  for w in $(xdotool search --name "" 2>/dev/null); do
    case "$(xdotool getwindowname "$w" 2>/dev/null)" in *diagn*) xdotool windowmove "$w" 0 1000;; esac
  done
  sleep 0.5
}

show_overlay() {
  xdotool mousemove 960 540
  xdotool key ctrl+alt+shift+d; sleep 1
  RECT=$(grep overlay_shown "$LOG" | tail -1 | python3 -c 'import sys,json;print(*json.loads(sys.stdin.read())["detail"]["rect"])')
  read -r X Y W H <<<"$RECT"
}

# A fonte de arraste existe ANTES do overlay, como o gerenciador de arquivos no uso real.
/usr/bin/python3.12 scripts/gtk_drag_source.py "$FIX/foto café 😀.png" "$FIX/日本語 b'c\".txt" & pids+=($!)
sleep 2
start_app

# 1) Atalho + posicionamento (sem arraste).
show_overlay
check overlay_shown "atalho mostra overlay"
[ "$X" = 976 ] && [ "$Y" = 556 ] && echo "OK   overlay em 976,556 (cursor 960,540 + deslocamento 16)" || { echo "FALHA posição do overlay: $RECT"; exit 1; }
xdotool key Escape; sleep 0.5
check 'cancelado: Escape' "Escape global cancela o overlay"

# 2) FLUXO REAL: botão pressionado e arraste em andamento -> atalho -> overlay aparece -> soltar nele.
#    Lote de 2 arquivos com Unicode. Até 3 instâncias do app (ver nota no topo).
ok=0
for attempt in 1 2 3; do
  if [ "$attempt" -gt 1 ]; then start_app; fi
  before=$(grep -c '"kind":"drop_accepted"' "$LOG" || true)
  shown_before=$(grep -c '"kind":"overlay_shown"' "$LOG" || true)
  xdotool mousemove 150 130 mousedown 1
  xdotool mousemove 190 150; sleep 0.3; xdotool mousemove 230 170; sleep 0.3
  xdotool key ctrl+alt+shift+d; sleep 0.8                      # atalho DURANTE o arraste
  [ "$(grep -c '"kind":"overlay_shown"' "$LOG")" -gt "$shown_before" ] || { echo "FALHA atalho durante o arraste não mostrou o overlay"; cat "$LOG"; exit 1; }
  RECT=$(grep overlay_shown "$LOG" | tail -1 | python3 -c 'import sys,json;print(*json.loads(sys.stdin.read())["detail"]["rect"])')
  read -r X Y W H <<<"$RECT"
  for i in 1 2 3 4 5 6 7 8; do xdotool mousemove $((230 + (X + W/2 - 230) * i / 8)) $((170 + (Y + H/2 - 170) * i / 8)); sleep 0.15; done
  for j in 1 2 3 4 5 6; do xdotool mousemove $((X + W/2 + j*8)) $((Y + H/2 + j*4)); sleep 0.15; done
  sleep 1; xdotool mouseup 1; sleep 1
  after=$(grep -c '"kind":"drop_accepted"' "$LOG" || true)
  if [ "$after" -gt "$before" ]; then ok=1; echo "OK   atalho durante o arraste + drop no overlay (instância $attempt de 3)"; break; fi
  echo "INFO instância $attempt do app sem drop registrado"
  xdotool key Escape; sleep 0.5
done
[ "$ok" = 1 ] || { echo "FALHA arraste externo não chegou em 3 instâncias"; cat "$LOG"; exit 1; }
check drag_enter "drag_enter registrado"
check 'foto café 😀.png' "nome Unicode/emoji preservado"
check '"count":2' "lote de 2 arquivos recebido"
echo "Smoke Linux concluído (log: $LOG)"
