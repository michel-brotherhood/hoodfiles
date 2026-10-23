#!/usr/bin/env bash
# Opções de troca de layout do XKB (grp:*) x atalhos: o atalho global chega ao handler (hotkey_raw)?
# Xvfb + openbox + xdotool + setxkbmap; NÃO é o Mint real. Uso: BIN=target/debug/hf-poc-app scripts/x11-hotkey-xkb-options.sh
set -uo pipefail
cd "$(dirname "$0")/.."
BIN=${BIN:-target/debug/hf-poc-app}
export DISPLAY=${DISPLAY:-:99}
pkill -x Xvfb 2>/dev/null; sleep 0.5
(Xvfb "$DISPLAY" -screen 0 1920x1080x24 >/dev/null 2>&1 &); sleep 1
(openbox >/dev/null 2>&1 &); sleep 1
probe() {   # $1 opção xkb; $2 HF_POC_SHORTCUT; $3 combinação do xdotool
  setxkbmap -layout br,us -variant "" -option "" ${1:+-option "$1"}; sleep 0.4
  local H L n; H=$(mktemp -d)
  (HF_POC_SHORTCUT="$2" HOME=$H "$BIN" >/dev/null 2>&1 &)
  for _ in $(seq 1 40); do L=$(ls "$H"/.local/share/*/poc-log.jsonl 2>/dev/null | head -1); [ -n "$L" ] && break; sleep 0.4; done
  sleep 1.5; xdotool key "$3"; sleep 0.8
  n=$(grep -c '"kind":"hotkey_raw"' "$L"); pkill -x hf-poc-app; sleep 0.7
  printf '%-24s %-24s hotkey_raw=%s\n' "${1:-(nenhuma)}" "$2" "$n"
}
echo "# layouts br,us; $(date -u +%FT%TZ)"
for opt in "" grp:alt_shift_toggle grp:ctrl_shift_toggle grp:ctrl_alt_toggle grp:alt_space_toggle grp:win_space_toggle grp:shifts_toggle grp:caps_toggle; do
  probe "$opt" "Control+Alt+Shift+KeyD" ctrl+alt+shift+d
  probe "$opt" "Control+Alt+KeyH" ctrl+alt+h
  probe "$opt" "F9" F9
done
