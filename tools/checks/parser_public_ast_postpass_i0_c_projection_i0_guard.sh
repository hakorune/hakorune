#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-public-ast-postpass-i0-c-projection-i0"
TASK="$ROOT/docs/development/current/main/investigations/parser-public-ast-postpass-i0-c-projection-i0-implementation-task-2026-08-09.md"
DESIGN="$ROOT/docs/development/current/main/investigations/parser-public-ast-postpass-i0-c-projection-d0-design-task-2026-08-09.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
README="$ROOT/src/parser/README.md"
REFERENCE="$ROOT/docs/reference/language/build-conditional-gate.md"
CALLABLE_REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
PROJECTION="$ROOT/src/parser/build_cfg/projection.rs"
PROJECTION_TESTS="$ROOT/src/parser/build_cfg/projection_tests.rs"
PRUNE="$ROOT/src/parser/build_cfg/prune.rs"
SEAL_MOD="$ROOT/src/parser/source_seal/mod.rs"
SEAL_MODEL="$ROOT/src/parser/source_seal/model.rs"
SEAL_GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
SEAL_FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
ENTRY="$ROOT/src/parser/string_postpass_entry.rs"
ENVELOPE="$ROOT/src/parser/postpass_envelope.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TASK" "$DESIGN" "$SSOT" "$README" "$REFERENCE" "$CALLABLE_REFERENCE" "$PROJECTION" "$PROJECTION_TESTS" "$PRUNE" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$ENTRY" "$ENVELOPE"

python3 - "$TASK" "$DESIGN" "$SSOT" "$README" "$REFERENCE" "$CALLABLE_REFERENCE" "$PROJECTION" "$PROJECTION_TESTS" "$PRUNE" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$ENTRY" "$ENVELOPE" <<'PY'
import sys
from pathlib import Path

task, design, ssot, readme, reference, callable_reference, projection, projection_tests, prune = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[1:10]
]
seal_paths = list(map(Path, sys.argv[10:14]))
seal = "\n".join(path.read_text(encoding="utf-8") for path in seal_paths)
entry, envelope = [Path(p).read_text(encoding="utf-8") for p in sys.argv[14:16]]

if "Status: closed" not in task:
    raise SystemExit("I0-C projection implementation task must be closed")
for text, label in ((design, "design"), (ssot, "SSOT"), (readme, "README"), (reference, "reference"), (callable_reference, "callable reference")):
    if "I0-C projection" not in text and "Projection I0 receipt" not in text and "I0-C-PROJECTION-I0" not in text:
        raise SystemExit(f"{label} missing I0-C projection receipt")
for needle in (
    "BuildGateProjectionOutputV1",
    "PreparedBuildGateDecisionSetV1",
    "source receipts",
    "explain",
    "`eval_build_predicate` call",
    "old generic prune",
):
    if needle not in task:
        raise SystemExit(f"I0-C task missing receipt: {needle}")
if "eval_build_predicate" in projection:
    raise SystemExit("projection must consume decisions and never evaluate predicates")
for needle in ("source_gate_prune", "prune_build_gate_program", "explain_build_gate_program"):
    if needle in seal:
        raise SystemExit(f"shared source seal still references retired postpass path: {needle}")
if "parse_postpass_with_demand" not in entry or "ExplainDemandV1::Capture" not in entry:
    raise SystemExit("public explain entry must use the shared postpass coordinator")
if "into_ast_and_explain" not in envelope:
    raise SystemExit("completed postpass must expose consuming AST/explain projection")
if "BuildGateProjectionSelector" not in prune:
    raise SystemExit("generic BuildGate walker must expose the projection selector boundary")
if "shared_projection_consumes_one_selected_source_gate" not in projection_tests:
    raise SystemExit("projection focused test is missing")
for path, text in ((Path(sys.argv[7]), projection), (Path(sys.argv[9]), prune), *((path, path.read_text(encoding="utf-8")) for path in seal_paths)):
    lines = len(text.splitlines())
    if lines >= 760:
        raise SystemExit(f"{path} reached the 760-line split trigger: {lines}")
print("shared_projection=1")
print("single_decision_consumer=1")
print("explain_parity_route=1")
print("summary=ok")
PY

echo "[$TAG] ok"
