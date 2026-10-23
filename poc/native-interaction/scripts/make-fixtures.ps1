# Gera fixtures pequenas e próprias para os testes manuais no Windows.
# Uso: powershell -ExecutionPolicy Bypass -File scripts\make-fixtures.ps1 [-Dir <pasta>]
param([string]$Dir = (Join-Path ([Environment]::GetFolderPath('Desktop')) 'hf-poc-fixtures'))
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force "$Dir\lote-25", "$Dir\pasta com espaço" | Out-Null
$enc = New-Object System.Text.UTF8Encoding($false)
function W($p) { [IO.File]::WriteAllText($p, 'hf-poc', $enc) }
W "$Dir\simples.txt"
W "$Dir\foto café 😀.png"
W "$Dir\日本語 b'c d;&.txt"        # aspas duplas são inválidas em nomes no Windows
W ("$Dir\nome-longo-" + ('x' * 150) + '.txt')
W "$Dir\pasta com espaço\dentro.txt"
1..25 | ForEach-Object { W ("$Dir\lote-25\arquivo-{0:D2}.txt" -f $_) }
Write-Host "Fixtures em: $Dir"
