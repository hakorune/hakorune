#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-dynamic-backedge-preclaim-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

BACKEDGE="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/continuation_backedge.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-compare-hardening-d0-2026-08-22.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_dynamic_backedge_preclaim_i0_guard.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$BACKEDGE" "$CARD" "$INDEX"

guard_expect_fixed_in_file "$TAG" "Consume the existing cleanup and operation rows before select_block()" "$BACKEDGE" \
  "Backedge claims must document the pre-effect boundary"
guard_expect_fixed_in_file "$TAG" "MIR-DYNAMIC-BACKEDGE-PRECLAIM-I0" "$CARD" \
  "active card must record the Backedge preclaim I0"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the Backedge preclaim guard"

python3 - "$BACKEDGE" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
start = text.find("fn emit_program(")
if start < 0:
    raise SystemExit("missing Backedge program emitter")
body = text[start:]

cleanup_claim = "claim(DynamicV2PhysicalCleanupCutPointV1::Backedge)"
if body.count(cleanup_claim) != 1:
    raise SystemExit("Backedge cleanup claim must occur exactly once")

operation_batch = "for item in [I13, I14, I15, I16]"
if body.count(operation_batch) != 1:
    raise SystemExit("I13..I16 operation claims must occur in exactly one batch")

claim_positions = [body.find(cleanup_claim), body.find(operation_batch)]
if min(claim_positions) < 0:
    raise SystemExit("missing Backedge preclaim")
last_claim = max(claim_positions)

header_check = body.find("Header has a foreign session brand")
if header_check < 0 or header_check > min(claim_positions):
    raise SystemExit("Header brand must be validated before Backedge claims")

effects = [
    body.find(".select_block("),
    body.find("values.publish("),
    body.find("issue_physical_value_id("),
    body.find("emit_checked_callout_end("),
]
effects = [position for position in effects if position >= 0]
if not effects:
    raise SystemExit("missing Backedge physical effect")
first_effect = min(effects)
if last_claim > first_effect:
    raise SystemExit("Backedge cleanup/operation claims occur after a physical effect")

if body.count("claim_operation(item)") != 1:
    raise SystemExit("Backedge operation claim call must occur exactly once")
PY

lines="$(wc -l < "$BACKEDGE" | tr -d '[:space:]')"
if (( lines >= 800 )); then
  guard_fail "$TAG" "Backedge source reached the 800-line hard boundary: $lines"
fi
if (( lines >= 760 )); then
  guard_fail "$TAG" "Backedge source reached the 760-line split trigger: $lines"
fi

echo "[$TAG] ok (Backedge cleanup/I13..I16 claims once before select_block and physical continuation effects)"
