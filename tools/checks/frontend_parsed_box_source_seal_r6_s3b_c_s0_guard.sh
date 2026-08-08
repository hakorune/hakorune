#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-c-s0"
AUTHORITY="$ROOT/src/parser/source_authority.rs"
BODY="$ROOT/src/parser/declarations/box_def/body.rs"
SEAL="$ROOT/src/parser/source_seal.rs"
SOURCE_TESTS="$ROOT/src/parser/delegate_source_tests.rs"
SEAL_TESTS="$ROOT/src/parser/source_seal_delegate_tests.rs"
TASK="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-s0-implementation-task-2026-08-09.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$AUTHORITY" "$BODY" "$SEAL" "$SOURCE_TESTS" "$SEAL_TESTS" "$TASK" "$SSOT" "$REFERENCE" "$INDEX"

python3 - "$AUTHORITY" "$BODY" "$SEAL" "$SOURCE_TESTS" "$SEAL_TESTS" "$TASK" "$SSOT" "$REFERENCE" "$INDEX" <<'PY'
import sys
from pathlib import Path

authority, body, seal, source_tests, seal_tests, task, ssot, reference, index = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[1:]
]

for needle in (
    "DelegateSourceDeclarationV1",
    "record_delegate_source_at_current",
    "delegate_source_declarations",
    "DelegateCompatibilityOnly",
    "prepend_selected_gate",
):
    if needle not in authority:
        raise SystemExit(f"missing parser-private delegate source transport: {needle}")

if "record_delegate_source_at_current(&delegate)" not in body:
    raise SystemExit("delegate parser must issue source rows immediately after parsing")

for needle in (
    "delegate_source_declarations: Box<[DelegateSourceDeclarationV1]>",
    "delegate_source_declarations: Box::new([])",
):
    if needle not in seal:
        raise SystemExit(f"missing prepared/final seal boundary: {needle}")

for needle in (
    "transaction_records_one_delegate_source_row_per_expose",
    "selected_gate_rebases_delegate_source_member_path",
    "compatibility_delegate_cannot_enter_source_transport",
):
    if needle not in source_tests:
        raise SystemExit(f"missing parser transport test: {needle}")
if "r6_s3b_c_s0_transports_delegate_rows_but_keeps_them_out_of_final_seal" not in seal_tests:
    raise SystemExit("missing final-seal isolation test")

for document, label in ((task, "task"), (ssot, "SSOT"), (reference, "reference")):
    for needle in ("R6-S3B-C-S0", "one row per expose", "resolver-visible"):
        if needle not in document:
            raise SystemExit(f"{label} missing S0 receipt: {needle}")

for forbidden in (
    "target lookup",
    "GeneratedDelegateSourceRelation",
    "ParserBoxSourceSealV1",
    "HashMap",
):
    if forbidden in source_tests:
        raise SystemExit(f"focused source transport tests must not open later authority: {forbidden}")

if "frontend_parsed_box_source_seal_r6_s3b_c_s0_guard.sh" not in index:
    raise SystemExit("check index must list the S0 guard")

for path in (Path(sys.argv[1]), Path(sys.argv[3])):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

print("delegate_source_rows=1")
print("selected_gate_rebase=1")
print("prepared_only_transport=1")
print("final_seal_isolation=1")
print("focused_tests=1")
print("landed_docs=1")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
