#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-public-ast-postpass-final-retire-s0"
TASK="$ROOT/docs/development/current/main/investigations/parser-public-ast-postpass-final-retire-s0-implementation-task-2026-08-09.md"
NOELSE="$ROOT/docs/development/current/main/investigations/parser-public-ast-postpass-final-no-else-receipt-d0-design-task-2026-08-09.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
MOD="$ROOT/src/parser/mod.rs"
PRED="$ROOT/src/parser/build_cfg/predicate.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TASK" "$NOELSE" "$STATE" "$MOD" "$PRED"

python3 - "$TASK" "$NOELSE" "$STATE" "$MOD" "$PRED" <<'PY'
import sys
from pathlib import Path

task, noelse, state, mod, pred = [
    Path(path).read_text(encoding="utf-8") for path in sys.argv[1:]
]

if "Status: accepted execution task; implementation not landed" not in task:
    raise SystemExit("retirement task must remain the active execution contract")
for needle in ("source_gate_prune.rs", "explain_build_gate_program", "NoElse receipt implementation"):
    if needle not in task:
        raise SystemExit(f"retirement task missing boundary: {needle}")
if "FINAL-NOELSE-RECEIPT-D0" not in noelse:
    raise SystemExit("NoElse D0 must remain the next separate design row")
for needle in (
    'work_mode = "fast"',
    'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-RETIRE-S0"',
):
    if needle not in state:
        raise SystemExit(f"CURRENT_STATE missing retirement pointer: {needle}")
if "source_gate_prune" in mod:
    raise SystemExit("caller-zero source_gate_prune module is not retired")
if "explain_build_gate_program" in pred:
    raise SystemExit("caller-zero explain helper is not retired")
for path in (
    Path(mod),
    Path(pred),
    Path(sys.argv[0]) if False else None,
):
    if path is not None and len(path.read_text(encoding="utf-8").splitlines()) >= 760:
        raise SystemExit(f"{path} reached the 760-line split trigger")
print("caller_zero_source_gate_prune=1")
print("caller_zero_explain_helper=1")
print("no_noelse_implementation=1")
print("summary=ok")
PY

echo "[$TAG] ok"
