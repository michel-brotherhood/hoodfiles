#!/usr/bin/env bash
# Bateria X11 do atalho global (Xvfb + openbox + xdotool): para cada configuração de teclado, o atalho chega
# ao handler (hotkey_raw)? e qual é o `state` que o X entrega (xev)? NÃO é o Mint real; reproduz o mecanismo.
# Requer: xvfb xdotool openbox x11-utils(xev) x11-xkb-utils(setxkbmap) x11-xserver-utils(xmodmap,xset).
# Uso: BIN=target/debug/hf-poc-app scripts/x11-hotkey-matrix.sh
set -uo pipefail
cd "$(dirname "$0")/.."
BIN=${BIN:-target/debug/hf-poc-app}
SC_KEY=${SC_KEY:-d}                     # tecla do atalho (padrão do app: Ctrl+Alt+Shift+D)
export DISPLAY=${DISPLAY:-:99}
export HOME=$(mktemp -d)
pkill -x Xvfb 2>/dev/null; sleep 0.5
(Xvfb "$DISPLAY" -screen 0 1920x1080x24 >/dev/null 2>&1 &); sleep 1
(openbox >/dev/null 2>&1 &); sleep 1
setxkbmap -layout us -option "" 
APP=""
start_app() {   # reinicia o app: o grab X11 é registrado na inicialização, sob o layout atual
  [ -n "$APP" ] && { kill "$APP" 2>/dev/null; sleep 1; }
  rm -rf "$HOME/.local"; LOG=""
  "$BIN" >"$HOME/app.out" 2>&1 & APP=$!
  for _ in $(seq 1 40); do LOG=$(find "$HOME" -name poc-log.jsonl | head -1); [ -n "$LOG" ] && break; sleep 0.5; done
  sleep 2
}
trap 'kill $APP 2>/dev/null; pkill -x xev 2>/dev/null' EXIT
start_app
raw() { grep -c '"kind":"hotkey_raw"' "$LOG" || true; }
leds() { xset q | grep -E "Caps Lock" | sed 's/  */ /g'; }

# janela com foco para o xev ver o `state` do mesmo KeyPress
xev -geometry 200x100+1500+800 -event keyboard >"$HOME/xev.out" 2>&1 & XEV=$!
sleep 1
XW=$(xdotool search --name "Event Tester" | head -1)

case_run() {   # $1 = descrição
  local before after xmark
  before=$(raw); xmark=$(wc -l < "$HOME/xev.out")
  xdotool windowfocus "$XW" 2>/dev/null; sleep 0.2
  xdotool key ctrl+alt+shift+$SC_KEY; sleep 0.7
  after=$(raw)
  # `state` do KeyPress da tecla do atalho (última linha KeyPress com o keysym) no xev
  # `state` do KeyPress da TECLA do atalho (não da modificadora). Se o grab do global-hotkey pegou o evento,
  # o xev não o vê (o grab o consome): isso é o caso saudável e aparece como "(consumido pelo grab)".
  local kcode st
  kcode=$(xmodmap -pke | grep -m1 -E " = $SC_KEY( |$)" | awk '{print $2}')
  st=$(tail -n +$((xmark+1)) "$HOME/xev.out" | grep -A4 "KeyPress event" | grep -A2 "state" | grep -B2 "keycode $kcode " | grep -oE "state 0x[0-9a-f]+" | tail -1 | cut -d' ' -f2)
  [ -z "$st" ] && [ $((after-before)) -gt 0 ] && st="(consumido pelo grab)"
  printf '%-52s hotkey_raw=%s  state(tecla)=%s\n' "$1" "$((after-before))" "${st:-?}"
}
toggle() { xdotool key "$1"; sleep 0.3; }

echo "# $(date -u +%FT%TZ)  BIN=$BIN  keyboard: $(setxkbmap -query | tr '\n' ' ')"
case_run "1. us, sem locks"
toggle Num_Lock;    case_run "2. us, Num Lock LIGADO ($(xset q | grep -o 'Num Lock: *[a-z]*'))"
toggle Num_Lock
toggle Caps_Lock;   case_run "3. us, Caps Lock LIGADO"
toggle Caps_Lock
toggle Scroll_Lock; case_run "4. us, Scroll_Lock pressionado (keymap padrão: LED $(xset q | grep -o 'Scroll Lock: *[a-z]*' | tr -s ' '), sem modificador)"
toggle Scroll_Lock; case_run "4b. us, Scroll Lock DESLIGADO de novo"
toggle Num_Lock; toggle Caps_Lock; toggle Scroll_Lock
case_run "5. us, Num+Caps+Scroll LIGADOS"
toggle Num_Lock; toggle Caps_Lock; toggle Scroll_Lock

# Scroll Lock ligado a Mod3 (configuração possível em teclados/layouts reais; aqui só no servidor de teste)
xmodmap -e "remove mod3 = ISO_Level5_Shift" -e "add mod3 = Scroll_Lock" 2>/dev/null
toggle Scroll_Lock; case_run "6. Scroll_Lock mapeado em Mod3, LIGADO"
toggle Scroll_Lock; case_run "6b. Scroll_Lock mapeado em Mod3, DESLIGADO"
setxkbmap -layout us -option ""


cfg() {   # $1 = descrição; demais = argumentos do setxkbmap; reinicia o app sob esse layout
  local d="$1"; shift
  setxkbmap "$@"; sleep 0.5; start_app
  XW=$(xdotool search --name "Event Tester" | head -1)
  echo "# $d -> $(setxkbmap -query | grep -E 'layout|variant|options' | tr -s ' ' | tr '\n' ' ')"
}
# --- Layouts. ATENÇÃO: o que quebra o atalho nos casos 7-11 é a OPÇÃO de troca de grupo (grp:alt_shift_toggle),
# --- que faz o Shift desaparecer do `state` (0xc em vez de 0xd); o layout br,us em si não é o problema (caso 7c).
cfg "br,us SEM opção de troca de grupo (controle)" -layout br,us -variant "" -option ""
case_run "7c. br,us sem grp:* — registro e teste sob br,us"
cfg "br,us COM grp:alt_shift_toggle" -layout br,us -variant "" -option grp:alt_shift_toggle
case_run "7. br,us + alt_shift_toggle — grupo 1"
xdotool key ISO_Next_Group; sleep 0.3
case_run "8. br,us + alt_shift_toggle — grupo 2"
cfg "br(abnt2),us COM grp:alt_shift_toggle" -layout br,us -variant abnt2, -option grp:alt_shift_toggle
case_run "9. br(abnt2),us + alt_shift_toggle — grupo 1"
cfg "registro sob us; depois liga alt_shift_toggle" -layout us -option ""
setxkbmap -layout br,us -option grp:alt_shift_toggle; sleep 0.5
case_run "11. app iniciado sob us; depois br,us + alt_shift_toggle"
