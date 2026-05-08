#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/mimalloc-raw-page-proof/main.hako"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_mimalloc_raw_page_proof_$$"
VERIFY_OUT="$TMP_ROOT.verify.out"
VERIFY_ERR="$TMP_ROOT.verify.err"
JSON_OUT="$TMP_ROOT.mir.json"
EMIT_OUT="$TMP_ROOT.emit.out"
EMIT_ERR="$TMP_ROOT.emit.err"

mkdir -p "$TMP_ROOT"
trap 'rm -rf "$TMP_ROOT"' EXIT

if [ -n "${HAKORUNE_BIN:-}" ]; then
  HAKO_CMD=("$HAKORUNE_BIN")
else
  HAKO_CMD=(cargo run -q --bin hakorune --)
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "${HAKO_CMD[@]}" --backend mir --verify "$APP" >"$VERIFY_OUT" 2>"$VERIFY_ERR"

grep -q 'MIR verification passed' "$VERIFY_OUT"

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "${HAKO_CMD[@]}" --emit-mir-json "$JSON_OUT" "$APP" >"$EMIT_OUT" 2>"$EMIT_ERR"

python3 - "$JSON_OUT" <<'PY'
import json
import sys

root = json.load(open(sys.argv[1], encoding="utf-8"))
functions = {fn.get("name"): fn for fn in root.get("functions", [])}

for name in ["MiRawPageProof.acquireBlock/1", "MiRawPageProof.releaseBlock/1"]:
    metadata = functions[name]["metadata"]
    plans = metadata.get("effect_plans", [])
    assert len(plans) == 1, (name, plans)
    assert plans[0]["requires"] == ["no_alloc", "no_safepoint"], (name, plans)
    assert plans[0]["source"] == "rune_contract", (name, plans)

all_calls = []
for fn in functions.values():
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            call = inst.get("mir_call")
            if call:
                callee = call.get("callee", {})
                all_calls.append((fn.get("name"), callee.get("type"), callee.get("box_name"), callee.get("name")))

required_calls = {
    ("RawBufCoreBox", "alloc_bytes_i64"),
    ("RawBufCoreBox", "free_bytes_i64"),
    ("RawArrayCoreBox", "slot_append_any"),
    ("RawArrayCoreBox", "slot_load_i64"),
    ("RawArrayCoreBox", "slot_store_i64"),
}

seen = set()
for _, callee_type, box_name, name in all_calls:
    if callee_type == "Method":
        seen.add((box_name, name))
    if callee_type == "Global" and isinstance(name, str) and "." in name:
        owner, rest = name.split(".", 1)
        method = rest.split("/", 1)[0]
        seen.add((owner, method))
missing = sorted(required_calls - seen)
assert not missing, missing
PY

printf '%s\n' \
  'mimalloc-raw-page-proof' \
  'mir_verify=ok' \
  'effect_plans=ok' \
  'capability_calls=ok' \
  'summary=ok'
