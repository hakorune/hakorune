#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-a"
PARSER="$ROOT/src/parser/mod.rs"
SEAL_MOD="$ROOT/src/parser/source_seal/mod.rs"
SEAL_MODEL="$ROOT/src/parser/source_seal/model.rs"
SEAL_GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
SEAL_FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
TASK="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$PARSER" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$TASK"

python3 - "$PARSER" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$TASK" <<'PY'
import sys
from pathlib import Path

parser_path = Path(sys.argv[1])
seal_paths = list(map(Path, sys.argv[2:6]))
task_path = Path(sys.argv[6])
parser = parser_path.read_text(encoding="utf-8")
seal = "\n".join(path.read_text(encoding="utf-8") for path in seal_paths)
task = task_path.read_text(encoding="utf-8")

for needle in (
    "OpenParserPostpassProductV1::new",
    ".prune_build_gates(&parser)?",
    ".lower_delegates()?",
    "parse_from_string_with_source_seal_ast",
    "parser.take_metadata()",
):
    if needle not in parser:
        raise SystemExit(f"missing S3B-A rich/product handoff: {needle}")

for needle in (
    "pub(in crate::parser) struct OpenParserPostpassProductV1",
    "pub(in crate::parser) struct ParserSourceSessionV1",
    "pub(in crate::parser) fn prune_build_gates",
    "pub(in crate::parser) fn lower_delegates",
    "pub(in crate::parser) fn finalize(",
    "metadata: ParserMetadata",
    "pub(in crate::parser) fn metadata(&self)",
):
    if needle not in seal:
        raise SystemExit(f"missing S3B-A product contract: {needle}")

for needle in (
    "R6-S3B-A  ParserPostpassProductV1 and AST-only projection parity",
    "Every AST-only public parser API calls the canonical rich path exactly once",
):
    if needle not in task:
        raise SystemExit(f"missing S3B-A SSOT receipt: {needle}")

for forbidden in (
    "method_source_member_ordinals",
    "record_new_methods_since",
    "crate::mir",
    "crate::resolver",
):
    if forbidden in (parser + seal):
        raise SystemExit(f"S3B-A forbidden legacy/semantic coupling remains: {forbidden}")

for path in (parser_path, *seal_paths):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

print("single_postpass_product=1")
print("metadata_is_diagnostic_only=1")
print("bounded_ast_projection=1")
print("gate_delegate_expansion_not_reopened=1")
print("legacy_sidecars=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
