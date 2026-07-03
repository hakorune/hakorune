#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")"/../../../.. && pwd)
source "$root/tools/test/golden/macro/lib/resolve_hakorune.sh"
bin="$(resolve_hakorune_golden_bin "$root")"
src="apps/tests/macro/loopform/foreach_basic.hako"
golden="$root/tools/test/golden/macro/foreach_basic.expanded.json"


normalize_json() { python3 -c 'import sys,json; print(json.dumps(json.loads(sys.stdin.read()), sort_keys=True, separators=(",", ":")))'; }

out_raw=$("$bin" --dump-expanded-ast-json "$src")
out_norm=$(printf '%s' "$out_raw" | normalize_json)
gold_norm=$(normalize_json < "$golden")

if [ "$out_norm" != "$gold_norm" ]; then
  echo "[FAIL] foreach_basic expanded JSON mismatch" >&2
  diff -u <(echo "$out_norm") <(echo "$gold_norm") || true
  exit 2
fi

echo "[OK] golden foreach_basic expansion matched"
