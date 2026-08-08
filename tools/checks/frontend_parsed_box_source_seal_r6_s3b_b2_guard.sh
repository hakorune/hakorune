#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-b2"
PARSER="$ROOT/src/parser/mod.rs"
LEDGER="$ROOT/src/parser/source_gate_ledger.rs"
PATHS="$ROOT/src/parser/source_path.rs"
PROJECTION="$ROOT/src/parser/build_cfg/prune.rs"
SEAL="$ROOT/src/parser/source_seal.rs"
TESTS="$ROOT/src/parser/source_session_tests.rs"
SEAL_TESTS="$ROOT/src/parser/source_seal_finalizer_tests.rs"
TASK="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$PARSER" "$LEDGER" "$PATHS" "$PROJECTION" "$SEAL" "$TESTS" "$SEAL_TESTS" "$TASK"

python3 - "$PARSER" "$LEDGER" "$PATHS" "$PROJECTION" "$SEAL" "$TESTS" "$SEAL_TESTS" "$TASK" <<'PY'
import sys
from pathlib import Path

parser, ledger, paths, projection, seal, tests, seal_tests, task = [Path(p).read_text(encoding="utf-8") for p in sys.argv[1:]]

for needle in ("source_build_gate_scope", "prepared_source_build_gate_records", "take_source_build_gate_records"):
    if needle not in parser:
        raise SystemExit(f"missing parser gate ledger transport: {needle}")

for needle in ("PreparedBuildGateSourceRecordV1", "TopLevelItem", "register_source_build_gate"):
    if needle not in ledger:
        raise SystemExit(f"missing typed gate ledger owner: {needle}")

for needle in ("SourceBuildGatePathV1", "RootTopLevel", "BranchChild"):
    if needle not in paths:
        raise SystemExit(f"missing distinct gate path grammar: {needle}")

for needle in ("project_build_gate_program", "BuildGateProjectionSelector", "BuildGateSelectionReceiptV1"):
    if needle not in projection + seal:
        raise SystemExit(f"missing source-aware gate prune contract: {needle}")

for needle in ("prepare_prune", "commit_prune", "source_seal_survives"):
    if needle not in seal:
        raise SystemExit(f"missing atomic source-session boundary: {needle}")

for needle in ("b2_top_level_gate_ledger", "b2_method_body_gate"):
    if needle not in tests:
        raise SystemExit(f"missing B2 focused test: {needle}")
for needle in ("r6_s3b_b2_prunes_nested_top_level_gate_once", "empty_gate"):
    if needle not in seal_tests:
        raise SystemExit(f"missing B2 focused test: {needle}")

for needle in ("R6-S3B-B2", "SourceBuildGatePathV1", "one selection receipt", "method/body"):
    if needle not in task:
        raise SystemExit(f"missing B2 SSOT boundary: {needle}")

for path in map(Path, sys.argv[1:8]):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

print("parser_gate_ledger=1")
print("distinct_gate_path=1")
print("source_aware_preorder_prune=1")
print("one_receipt_per_opened_gate=1")
print("atomic_source_session_prune=1")
print("method_body_scope_closed=1")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
