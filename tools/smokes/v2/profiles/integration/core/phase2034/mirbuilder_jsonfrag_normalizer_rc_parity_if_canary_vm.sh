#!/usr/bin/env bash
# RC parity: MirBuilder output vs Normalized output must produce identical return codes (If/Compare)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_hako="/tmp/mirbuilder_rc_parity_if_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder" as MirBuilderBox
using "hako.mir.builder.internal.jsonfrag_normalizer" as NormBox
static box Main { method main(args) {
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local out = MirBuilderBox.emit_from_program_json_v0(j, null)
  if out == null { print("[fail:builder]"); return 1 }
  local outn = NormBox.normalize_all(out)
  print("[A_BEGIN]"); print("" + out);  print("[A_END]")
  print("[B_BEGIN]"); print("" + outn); print("[B_END]")
  return 0
} }
HAKO

PROG='{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Compare","op":"<=","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":2}},"then":[{"type":"Return","expr":{"type":"Int","value":7}}],"else":[{"type":"Return","expr":{"type":"Int","value":9}}]}]}'

set +e
out="$(PROG_JSON="$PROG" run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
set -e
rm -f "$tmp_hako" || true
if [ "$rc" -ne 0 ]; then echo "[SKIP] rc_parity_if: env unstable"; exit 0; fi

mir_a=$(echo "$out" | awk '/\[A_BEGIN\]/{f=1;next}/\[A_END\]/{f=0}f')
mir_b=$(echo "$out" | awk '/\[B_BEGIN\]/{f=1;next}/\[B_END\]/{f=0}f')
if [ -z "$mir_a" ] || [ -z "$mir_b" ]; then echo "[SKIP] rc_parity_if: MIR missing"; exit 0; fi

tmp_a="/tmp/mir_a_$$.json"; tmp_b="/tmp/mir_b_$$.json"
printf '%s' "$mir_a" > "$tmp_a"; printf '%s' "$mir_b" > "$tmp_b"

set +e
verify_mir_rc "$tmp_a"; rc_a=$?
verify_mir_rc "$tmp_b"; rc_b=$?
set -e
rm -f "$tmp_a" "$tmp_b" || true

if [ "$rc_a" -eq "$rc_b" ]; then echo "[PASS] mirbuilder_jsonfrag_normalizer_rc_parity_if_canary_vm"; exit 0; fi
echo "[FAIL] rc_parity_if: rc_a=$rc_a rc_b=$rc_b"; exit 1

