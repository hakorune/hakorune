#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-dynamic-inner-return-preclaim-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

INNER_RETURN="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/inner_return_then.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-compare-hardening-d0-2026-08-22.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_dynamic_inner_return_preclaim_i0_guard.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$INNER_RETURN" "$CARD" "$INDEX"

guard_expect_fixed_in_file "$TAG" "Consume the existing cleanup and operation rows before select_block()" "$INNER_RETURN" \
  "InnerReturn claims must document the pre-effect boundary"
guard_expect_fixed_in_file "$TAG" "MIR-DYNAMIC-INNER-RETURN-PRECLAIM-I0" "$CARD" \
  "active card must record the InnerReturn preclaim I0"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the InnerReturn preclaim guard"

python3 - "$INNER_RETURN" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
start = text.find("fn emit_program(")
if start < 0:
    raise SystemExit("missing InnerReturn program emitter")
body = text[start:]

markers = {
    "cleanup": "claim(DynamicV2PhysicalCleanupCutPointV1::InnerReturn)",
    "I11": "claim_operation(I11)",
    "Exit": "claim_exit()",
}
positions = {}
for name, marker in markers.items():
    found = [index for index in range(len(body)) if body.startswith(marker, index)]
    if len(found) != 1:
        raise SystemExit(f"expected exactly one {name} claim, found {len(found)}")
    positions[name] = found[0]

effects = [
    body.find(".select_block("),
    body.find("read_entry_receipt("),
    body.find("emit_checked_callout_end("),
    body.find("values.publish("),
]
effects = [position for position in effects if position >= 0]
if not effects:
    raise SystemExit("missing InnerReturn physical effect")
if max(positions.values()) > min(effects):
    raise SystemExit("InnerReturn cleanup/operation claims occur after a physical effect")

if body.count("claim_operation(I11)") != 1 or body.count("claim_exit()") != 1:
    raise SystemExit("InnerReturn operation claims must not retain post-effect edges")
PY

lines="$(wc -l < "$INNER_RETURN" | tr -d '[:space:]')"
if (( lines >= 800 )); then
  guard_fail "$TAG" "InnerReturn source reached the 800-line hard boundary: $lines"
fi
if (( lines >= 760 )); then
  guard_fail "$TAG" "InnerReturn source reached the 760-line split trigger: $lines"
fi

echo "[$TAG] ok (InnerReturn cleanup/I11/Exit claims once before select_block and physical effects)"
