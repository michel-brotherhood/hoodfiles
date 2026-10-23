#!/usr/bin/env bash
# Verificações rápidas no Linux Mint ANTES de abrir o app: versão, sessão gráfica, arquitetura e bibliotecas.
set -u
cd "$(dirname "$0")"
echo "== Sistema"
grep -E '^(PRETTY_NAME|VERSION_ID|VERSION_CODENAME|UBUNTU_CODENAME|ID)=' /etc/os-release
echo "arch: $(uname -m)   kernel: $(uname -r)"
echo "== Sessão gráfica (ANOTE)"
echo "XDG_SESSION_TYPE=${XDG_SESSION_TYPE:-?}  XDG_CURRENT_DESKTOP=${XDG_CURRENT_DESKTOP:-?}  DISPLAY=${DISPLAY:-}  WAYLAND_DISPLAY=${WAYLAND_DISPLAY:-}"
[ "${XDG_SESSION_TYPE:-}" = "x11" ] || echo "AVISO: sessão não é X11. Registre como está; para o teste de referência, use uma sessão X11."
echo "== Teclado (somente leitura; nada é alterado)"
if command -v setxkbmap >/dev/null; then setxkbmap -query; else echo "(setxkbmap indisponível)"; fi
if command -v xmodmap >/dev/null; then
  echo "-- mapa de modificadores (xmodmap -pm)"; xmodmap -pm
  echo "-- teclas d/h e modificadores no mapa de teclado"
  xmodmap -pke | grep -E " = (d|D|h|H)( |$)|Alt_L|Alt_R|Control_L|Shift_L|Super_L|Meta_L|ISO_Level3_Shift|Mode_switch" | head -20
else echo "(xmodmap indisponível)"; fi
if command -v xset >/dev/null; then xset q | grep -iE "Num Lock|Caps Lock" || true; fi
echo "== Bibliotecas do executável"
missing=$(ldd ./hf-poc-app 2>&1 | grep "not found" || true)
if [ -n "$missing" ]; then
  echo "FALTAM bibliotecas:"; echo "$missing"
  echo "Instale (Mint 22.x): sudo apt install libwebkit2gtk-4.1-0 libjavascriptcoregtk-4.1-0 libgtk-3-0t64 libsoup-3.0-0 libdbus-1-3"
  exit 1
fi
echo "OK: todas as bibliotecas dinâmicas encontradas."
