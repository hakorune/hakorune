#!/bin/bash
# Return(Method set with String value) → const string + mir_call structure
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_method_set_str_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder" as MirBuilderBox
static box Main { method main(args) {
  // Local m = new MapBox(); return m.set(1, "x");
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[" +
    "{\"type\":\"Local\",\"name\":\"m\",\"expr\":{\"type\":\"New\",\"class\":\"MapBox\",\"args\":[]}}," +
    "{\"type\":\"Return\",\"expr\":{\"type\":\"Method\",\"recv\":{\"type\":\"Var\",\"name\":\"m\"},\"method\":\"set\",\"args\":[{\"type\":\"Int\",\"value\":1},{\"type\":\"String\",\"value\":\"x\"}]}}]}";
  local out = MirBuilderBox.emit_from_program_json_v0(j, null);
  if out == null { return 0 }
  local s = "" + out
  if s.indexOf("\"type\":\"string\"") >= 0 && s.indexOf("\"op\":\"mir_call\"") >= 0 { return 1 }
  return 0
} }
HAKO

set +e
out="$(HAKO_MIR_BUILDER_INTERNAL=1 "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [ "$rc" -eq 1 ]; then echo "[PASS] mirbuilder_internal_method_set_string_canary_vm"; exit 0; fi
echo "[FAIL] mirbuilder_internal_method_set_string_canary_vm (rc=$rc)" >&2; exit 1
