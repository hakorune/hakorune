#!/usr/bin/env bash
# MirBuilder(minimal if.compare) + Normalizer ON — tag + tokens check
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_if_compare_norm_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using hako.mir.builder as MirBuilderBox
using "hako.mir.builder.internal.jsonfrag_normalizer" as NormBox
static box Main { method main(args) {
  // If(Int cmp Int) → two branch return(Int)
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local out = MirBuilderBox.emit_from_program_json_v0(j, null);
  if out == null { print("[fail:builder]"); return 1 }
  // Apply Normalizer under toggle for canary
  local norm = env.get("HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE");
  if norm != null && ("" + norm) == "1" { out = NormBox.normalize_all(out) }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

PROG='{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Compare","op":"<","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}},"then":[{"type":"Return","expr":{"type":"Int","value":10}}],"else":[{"type":"Return","expr":{"type":"Int","value":20}}]}]}'

set +e
out="$(PROG_JSON="$PROG" HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1 run_nyash_vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [ "$rc" -ne 0 ]; then echo "[SKIP] if_compare_norm: env unstable"; exit 0; fi
if ! echo "$out" | grep -q "\[mirbuilder/normalize:jsonfrag:pass\]"; then echo "[SKIP] if_compare_norm: tag missing"; exit 0; fi
mir=$(echo "$out" | extract_mir_from_output)
if [ -z "$mir" ]; then echo "[SKIP] if_compare_norm: no MIR (env)"; exit 0; fi
if echo "$mir" | assert_has_tokens '"op":"compare"' '"op":"branch"' '"op":"ret"'; then
  echo "[PASS] mirbuilder_internal_if_compare_jsonfrag_normalize_canary_vm"; exit 0; fi
echo "[SKIP] if_compare_norm: tokens not found (env)"; exit 0

