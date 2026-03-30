Param()
$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$bin = Join-Path $root 'target\release\hakorune.exe'
if (-not (Test-Path $bin)) {
  $bin = Join-Path $root 'target\release\hakorune'
}
if (-not (Test-Path $bin)) {
  $bin = Join-Path $root 'target\release\nyash.exe'
}
if (-not (Test-Path $bin)) {
  $bin = Join-Path $root 'target\release\nyash'
}

if (-not (Test-Path $bin)) {
  Write-Host 'Building hakorune (release)...'
  cargo build --release --features cranelift-jit | Out-Null
  $bin = Join-Path $root 'target\release\hakorune.exe'
  if (-not (Test-Path $bin)) { $bin = Join-Path $root 'target\release\hakorune' }
  if (-not (Test-Path $bin)) { $bin = Join-Path $root 'target\release\nyash.exe' }
  if (-not (Test-Path $bin)) { $bin = Join-Path $root 'target\release\nyash' }
}

Write-Host '[Smoke] Parser v0 JSON pipe → MIR-Interp'
$json = '{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Binary","op":"*","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}}}]}'
$pipeOut = $json | & $bin --ny-parser-pipe
$pipeRc = $LASTEXITCODE
if ($pipeRc -eq 7) { Write-Host 'PASS: pipe path' } else { Write-Host "FAIL: pipe path (rc=$pipeRc)"; Write-Output $pipeOut; exit 1 }

Write-Host '[Smoke] --json-file path'
# archive-only evidence: keep this as a compat loader monitor, not a current-facing direct-MIR route
$tmp = New-TemporaryFile
@'{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Binary","op":"*","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}}}]}'@ | Set-Content -Path $tmp -NoNewline
$fileOut = & $bin --json-file $tmp
$fileRc = $LASTEXITCODE
if ($fileRc -eq 7) { Write-Host 'PASS: json-file path' } else { Write-Host "FAIL: json-file path (rc=$fileRc)"; Write-Output $fileOut; exit 1 }
Write-Host 'All PASS'
