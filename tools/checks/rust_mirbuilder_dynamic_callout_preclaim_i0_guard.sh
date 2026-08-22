#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-dynamic-callout-preclaim-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CALLOUT="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/callout_corridor/emission.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-compare-hardening-d0-2026-08-22.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_dynamic_callout_preclaim_i0_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" python3
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$CALLOUT" "$CARD" "$INDEX"

guard_expect_fixed_in_file "$TAG" "Consume the existing operation census before the first MIR/ledger" "$CALLOUT" \
  "CallOut claims must document the pre-MIR/ledger boundary"
guard_expect_fixed_in_file "$TAG" "MIR-DYNAMIC-CALLOUT-PRECLAIM-I0" "$CARD" \
  "active card must record the CallOut preclaim I0"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the CallOut preclaim guard"

python3 - "$CALLOUT" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
first_effect = text.find("loop_operation::publish_i64_value")
if first_effect < 0:
    raise SystemExit("missing first CallOut MIR/ledger physical effect")
claim = "claim_operation(crate::mir::loop_recipe_contract::LoopItemKeyV1::new(raw))"
if text.count("for raw in 0..8") != 1:
    raise SystemExit("CallOut I0..I7 claim loop must occur exactly once")
if text.count(claim) != 1:
    raise SystemExit("CallOut I0..I7 claim operation must occur exactly once")
claim_position = text.find(claim)
if claim_position > first_effect:
    raise SystemExit("CallOut I0..I7 claims occur after the first physical effect")
PY

lines="$(wc -l < "$CALLOUT" | tr -d '[:space:]')"
if (( lines >= 800 )); then
  guard_fail "$TAG" "CallOut source reached the 800-line hard boundary: $lines"
fi
if (( lines >= 760 )); then
  guard_fail "$TAG" "CallOut source reached the 760-line split trigger: $lines"
fi

echo "[$TAG] ok (I0..I7 claims once before first MIR/ledger effect; no second authority)"
