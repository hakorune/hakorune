#!/usr/bin/env bash
#!/usr/bin/env bash
# TLV round‑trip smoke (Phase 22.1)
# Always runs a focused test against the nyash-tlv crate only.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
echo "[info] Building nyash-tlv (c-shim) ..." >&2
(
  cd "$ROOT" && cargo build -p nyash-tlv --features c-shim --release >/dev/null
)

python3 - "$ROOT" << 'PY'
import sys, importlib.util, pathlib, subprocess, json
root = pathlib.Path(sys.argv[1])
print("[info] TLV roundtrip (identity)")
# Since nyash-tlv is a lib crate, we exec `cargo test -p nyash-tlv` as a quick proof.
rc = subprocess.call(["cargo","test","-p","nyash-tlv","--release","--","identity_roundtrip"], cwd=root)
sys.exit(0 if rc == 0 else 1)
PY

echo "[PASS] tlv_roundtrip_smoke"
exit 0
