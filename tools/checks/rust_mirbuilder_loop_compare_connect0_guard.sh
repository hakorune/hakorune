#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-loop-compare-connect0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

I9_CONTROL="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/i8_i9_control.rs"
VALUE_LEDGER="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/value_ledger.rs"
BRAND="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/mod.rs"
WRITER="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_compare_writer.rs"
OPERAND="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/session/same_block_operand.rs"
OPERATION_CURSOR="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/operation_cursor.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-compare-connect0-d0-2026-08-22.md"
README="$ROOT_DIR/src/mir/builder/resolved_lowering/README.md"
REFERENCE="$ROOT_DIR/docs/reference/mir/canonical-loop-compare-same-block.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_loop_compare_connect0_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$I9_CONTROL" "$VALUE_LEDGER" "$BRAND" "$WRITER" \
  "$OPERAND" "$OPERATION_CURSOR" "$CARD" "$README" "$REFERENCE" "$INDEX"

guard_expect_fixed_in_file "$TAG" "CanonicalLoopCompareI64WriterV1::prepare(" "$I9_CONTROL" \
  "selected Dynamic I9 must call the named strict writer preparation"
guard_expect_fixed_in_file "$TAG" "SelectedDynamicI9CompareHandoffIssuerV1" "$I9_CONTROL" \
  "I9 handoff must have one named private issuer"
guard_expect_fixed_in_file "$TAG" "prepare_existing_same_block_integer" "$I9_CONTROL" \
  "I9 operands must be rebound through canonical same-block witnesses"
guard_expect_fixed_in_file "$TAG" "prepare_branch(canonical" "$I9_CONTROL" \
  "I9 Branch must be prepared before result reservation"
guard_expect_fixed_in_file "$TAG" "reserve_result(" "$I9_CONTROL" \
  "Dynamic V13 must reserve after all strict preparation"
guard_expect_fixed_in_file "$TAG" "Consume the existing physical census before the first I8 ValueId" "$I9_CONTROL" \
  "I8/I9/If claims must be explicitly pre-effect"
guard_expect_fixed_in_file "$TAG" "pending.commit()" "$I9_CONTROL" \
  "Dynamic V13 must commit only from the private prepared aggregate"
guard_expect_fixed_in_file "$TAG" "for_owner(demand.identity().owner())" "$BRAND" \
  "Dynamic session brand must bind the canonical function owner"
guard_expect_fixed_in_file "$TAG" "PendingDynamicV2PhysicalValuePublishV1" "$VALUE_LEDGER" \
  "Dynamic result publication must use a private pending token"
guard_expect_fixed_in_file "$TAG" "ResultPoisoned" "$VALUE_LEDGER" \
  "dropped Dynamic reservations must poison their result slot"
guard_expect_fixed_in_file "$TAG" "CanonicalCompareDefinitionSourceV1" "$WRITER" \
  "strict writer must return the canonical definition source"
guard_expect_fixed_in_file "$TAG" "unique physical MIR definition" "$OPERAND" \
  "same-block issuer must document its physical-definition proof"
guard_expect_fixed_in_file "$TAG" "Selected Dynamic I9 direct canonical handoff" "$README" \
  "resolved-lowering README must document the selected handoff"
guard_expect_fixed_in_file "$TAG" "Dynamic I9 direct handoff" "$REFERENCE" \
  "Compare reference must document the selected handoff"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the CONNECT0 guard"

production_callers=()
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  case "$file" in
    *_tests.rs) continue ;;
  esac
  production_callers+=("$file")
done < <(rg -l --glob '*.rs' -F 'CanonicalLoopCompareI64WriterV1::prepare(' "$ROOT_DIR/src" || true)

if [[ "${#production_callers[@]}" -ne 1 || "${production_callers[0]:-}" != "$I9_CONTROL" ]]; then
  guard_fail "$TAG" "expected exactly one selected I9 production writer caller; found ${production_callers[*]:-none}"
fi

python3 - "$I9_CONTROL" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
first_effect = text.find("issue_physical_value_id(")
if first_effect < 0:
    raise SystemExit("missing first I9 physical ValueId effect")
for claim in ("claim_operation(I8)", "claim_operation(I9)", "claim_if()"):
    positions = [index for index in range(len(text)) if text.startswith(claim, index)]
    if len(positions) != 1:
        raise SystemExit(f"expected exactly one {claim}, found {len(positions)}")
    if positions[0] > first_effect:
        raise SystemExit(f"{claim} occurs after the first I9 physical effect")

start = text.index("impl SelectedDynamicI9CompareHandoffIssuerV1")
end = text.index("\npub(super) fn emit", start)
body = text[start:end]
compare = body.find("CanonicalLoopCompareI64WriterV1::prepare(")
branch = body.find("let branch = prepare_branch(canonical")
reserve = body.find("let pending = values")
commit = body.find(".commit(outer.builder_view_mut_for_lowering())")
if min(compare, branch, reserve, commit) < 0:
    raise SystemExit("missing I9 prepare/branch/reserve/commit sequence")
if not compare < branch < reserve < commit:
    raise SystemExit("I9 fallible preparation/reservation order is not strict")
pending_end = body.find(";", reserve)
if pending_end < 0 or body[pending_end + 1 :].count("?") != 0:
    raise SystemExit("I9 has a Result path after V13 reservation")
PY

# The selected I9 row must not fall back to the generic Loop ledger or legacy
# Compare leaf. Other canary/compatibility rows remain outside this guard.
for forbidden in \
  'emit_compare_i64_at' \
  'CanonicalLoopCompareI64WriterV1::emit(' \
  '.emit_branch(' \
  'loop_operation' \
  'LoopOperationValueLedger' \
  'values.publish(' \
  'state.get('
do
  if rg -n -F -- "$forbidden" "$I9_CONTROL" >/dev/null 2>&1; then
    guard_fail "$TAG" "I9 handoff reaches a forbidden legacy/second-authority route: $forbidden"
  fi
done

python3 - "$VALUE_LEDGER" <<'PY'
from pathlib import Path
import sys

text = Path(sys.argv[1]).read_text()
production = text.split("#[cfg(test)]", 1)[0]
if "assert_eq!" in production:
    raise SystemExit("Dynamic V13 production commit must not use assert-based definition pairing")
PY

for file in "$I9_CONTROL" "$VALUE_LEDGER" "$BRAND" "$WRITER" "$OPERAND" "$OPERATION_CURSOR"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "CONNECT0 source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
  if (( lines >= 760 )); then
    guard_fail "$TAG" "CONNECT0 source reached the 760-line split trigger: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (one selected I9 writer caller, I8/I9/If pre-effect claims, Dynamic-only V13 commit, no legacy fallback)"
