#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-public-ast-postpass-final-d0"
TASK="$ROOT/docs/development/current/main/investigations/parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md"
TASKMAP="$ROOT/docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
MOD="$ROOT/src/parser/mod.rs"
SEAL_MOD="$ROOT/src/parser/source_seal/mod.rs"
SEAL_MODEL="$ROOT/src/parser/source_seal/model.rs"
SEAL_GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
SEAL_FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
ENTRY="$ROOT/src/parser/string_postpass_entry.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TASK" "$TASKMAP" "$SSOT" "$STATE" "$MOD" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$ENTRY"

python3 - "$ROOT" "$TASK" "$TASKMAP" "$SSOT" "$STATE" "$MOD" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$ENTRY" <<'PY'
import sys
from pathlib import Path

root = Path(sys.argv[1])
task, taskmap, ssot, state, mod = [Path(path).read_text(encoding="utf-8") for path in sys.argv[2:7]]
seal_paths = list(map(Path, sys.argv[7:11]))
seal = "\n".join(path.read_text(encoding="utf-8") for path in seal_paths)
entry = Path(sys.argv[11]).read_text(encoding="utf-8")

if not any(
    status in task
    for status in (
        "Status: accepted design; implementation not opened",
        "Status: closed design audit",
    )
):
    raise SystemExit("FINAL D0 must remain design-only or carry its closeout receipt")
for text, label in ((taskmap, "task map"), (ssot, "SSOT")):
    if "PARSER-PUBLIC-AST-POSTPASS-FINAL" not in text:
        raise SystemExit(f"{label} missing FINAL boundary")
for needle in (
    "retirement proof",
    "grammar-evidence",
    "postpass_compatibility::lower",
    "source_gate_prune.rs",
    "NoElse",
    "no retry",
):
    if needle not in task:
        raise SystemExit(f"FINAL task missing contract: {needle}")
final_design_stop = 'work_mode = "design_stop"' in state
cleanup_successor = all(
    needle in state
    for needle in (
        'work_mode = "fast"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-GUARD-CLEANUP-S0"',
    )
)
if not (final_design_stop or cleanup_successor):
    raise SystemExit("FINAL D0 must remain a design stop or explicit cleanup successor")
if not any(
    needle in state
    for needle in (
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-D0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-NOELSE-RECEIPT-D0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-CLOSEOUT-D0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-GUARD-CLEANUP-S0"',
    )
):
    raise SystemExit("CURRENT_STATE missing FINAL/NoElse design-stop pointer")
if "parse_postpass_s0" not in entry or "parse_postpass_with_demand" not in entry:
    raise SystemExit("public parser entry must remain on the shared postpass owner")
if "postpass_compatibility::lower" not in seal:
    raise SystemExit("explicit compatibility arm must remain visible in the coordinator")
if "parse_from_string_with_source_seal" not in mod:
    raise SystemExit("resolver-grade source-seal entry must remain available")
for path in (
    root / "src/parser/mod.rs",
    *seal_paths,
    root / "src/parser/postpass_envelope.rs",
    root / "src/parser/build_cfg/projection.rs",
):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 760:
        raise SystemExit(f"{path} reached the 760-line split trigger")
print("final_retirement_proof_design=1")
print("separate_grammar_and_compat=1")
print("no_forced_switch=1")
print("summary=ok")
PY

echo "[$TAG] ok"
