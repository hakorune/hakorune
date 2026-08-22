#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-dynamic-fault-cleanup-preclaim-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FAULT="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/fault_terminals.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-compare-hardening-d0-2026-08-22.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_dynamic_fault_cleanup_preclaim_i0_guard.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$FAULT" "$CARD" "$INDEX"

guard_expect_fixed_in_file "$TAG" "Consume both existing cleanup rows before any Fault or CallOut-End MIR" "$FAULT" \
  "Fault cleanup claims must document the pre-effect boundary"
guard_expect_fixed_in_file "$TAG" "MIR-DYNAMIC-FAULT-CLEANUP-PRECLAIM-I0" "$CARD" \
  "active card must record the Fault cleanup preclaim I0"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the Fault cleanup preclaim guard"

python3 - "$FAULT" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
start = text.find("pub(super) fn emit(")
if start < 0:
    raise SystemExit("missing Fault terminal emitter")
body = text[start:]

claims = {
    "I6Fault": "claim(DynamicV2PhysicalCleanupCutPointV1::I6Fault)",
    "I7Fault": "claim(DynamicV2PhysicalCleanupCutPointV1::I7Fault)",
}
positions = {}
for name, marker in claims.items():
    found = [index for index in range(len(body)) if body.startswith(marker, index)]
    if len(found) != 1:
        raise SystemExit(f"expected exactly one {name} cleanup claim, found {len(found)}")
    positions[name] = found[0]

first_i6_effect = body.find("corridor.with_i6_fault")
first_i7_effect = body.find("corridor.with_i7_fault")
if first_i6_effect < 0 or first_i7_effect < 0:
    raise SystemExit("missing Fault/CallOut-End effect closures")
first_effect = min(first_i6_effect, first_i7_effect)
for name, position in positions.items():
    if position > first_effect:
        raise SystemExit(f"{name} cleanup claim occurs after the first physical effect")

if sum(body.count(marker) for marker in claims.values()) != 2:
    raise SystemExit("Fault emitter must have exactly two cleanup claims")
PY

lines="$(wc -l < "$FAULT" | tr -d '[:space:]')"
if (( lines >= 800 )); then
  guard_fail "$TAG" "Fault terminal source reached the 800-line hard boundary: $lines"
fi
if (( lines >= 760 )); then
  guard_fail "$TAG" "Fault terminal source reached the 760-line split trigger: $lines"
fi

echo "[$TAG] ok (I6/I7 cleanup claims once before Fault/CallOut-End effects; existing cursor/discard authority preserved)"
