#!/usr/bin/env bash
# Empacota o build de teste SEM assinatura em dist/. Uso: package-ci.sh windows-x64|macos-arm64 <sha7>
set -euo pipefail
cd "$(dirname "$0")/.."
OS="$1"; SHA="$2"
OUT="dist/hf-poc-$OS-$SHA"
rm -rf dist && mkdir -p "$OUT"
cp ROTEIRO-N03-N06.md "$OUT/LEIA-ME.md"
cp scripts/make-fixtures.sh scripts/make-fixtures.ps1 "$OUT/"
printf 'Build de TESTE SEM ASSINATURA/NOTARIZACAO (HF-00)\ncommit: %s\nplataforma: %s\n' "$SHA" "$OS" > "$OUT/BUILD-INFO.txt"

case "$OS" in
  windows-x64)
    cp target/release/hf-poc-app.exe "$OUT/"
    (cd "$OUT" && sha256sum hf-poc-app.exe > SHA256SUMS.txt)
    # zip preserva tudo no Windows; o artifact do Actions já é um .zip, então entrega a pasta.
    ;;
  macos-arm64)
    APP="$OUT/HoodFilesPoC.app"
    mkdir -p "$APP/Contents/MacOS"
    cp target/release/hf-poc-app "$APP/Contents/MacOS/hf-poc-app"
    chmod +x "$APP/Contents/MacOS/hf-poc-app"
    cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleIdentifier</key><string>dev.hoodfiles.poc.native-interaction</string>
  <key>CFBundleName</key><string>HoodFiles PoC</string>
  <key>CFBundleExecutable</key><string>hf-poc-app</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>0.0.1</string>
  <key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
    # Assinatura ad-hoc (sem identidade de desenvolvedor): exigida pelo arm64 para executar; NÃO é assinatura nem notarização.
    codesign --force --deep -s - "$APP"
    codesign -dv "$APP" 2>&1 | head -5 || true
    # tar.gz preserva permissão de execução (o zip do artifact não preserva).
    tar -C "$OUT" -czf "$OUT/HoodFilesPoC-macos-arm64.tar.gz" HoodFilesPoC.app
    rm -rf "$APP"
    (cd "$OUT" && shasum -a 256 HoodFilesPoC-macos-arm64.tar.gz > SHA256SUMS.txt)
    ;;
  *) echo "plataforma desconhecida: $OS" >&2; exit 2 ;;
esac
ls -la "$OUT"
