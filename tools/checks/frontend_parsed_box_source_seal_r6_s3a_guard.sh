#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3a"
PARSER="$ROOT/src/parser/mod.rs"
AUTHORITY="$ROOT/src/parser/source_authority.rs"
SEAL_MOD="$ROOT/src/parser/source_seal/mod.rs"
SEAL_MODEL="$ROOT/src/parser/source_seal/model.rs"
SEAL_GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
SEAL_FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
BOX="$ROOT/src/parser/declarations/box_def/mod.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$PARSER" "$AUTHORITY" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$BOX"

python3 - "$PARSER" "$AUTHORITY" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$BOX" <<'PY'
import re
import sys
from pathlib import Path

parser_path = Path(sys.argv[1])
authority_path = Path(sys.argv[2])
seal_paths = list(map(Path, sys.argv[3:7]))
box_path = Path(sys.argv[7])
parser = parser_path.read_text(encoding="utf-8")
authority = authority_path.read_text(encoding="utf-8")
seal = "\n".join(path.read_text(encoding="utf-8") for path in seal_paths)
box = box_path.read_text(encoding="utf-8")

for needle in (
    "mod source_seal;",
    "parse_from_string_with_source_seal",
    "std::mem::take(&mut parser.prepared_source_seals)",
    "OpenParserPostpassProductV1::new",
    ".finalize()",
):
    if needle not in parser:
        raise SystemExit(f"missing R6-S3A rich parse entry/finalizer: {needle}")

for needle in (
    "pub(in crate::parser) struct ParserBoxSourceSealV1",
    "pub(in crate::parser) struct ParsedProgramWithSourceV1",
    "fn finalize_program",
    "InventoryPrefixMismatch",
):
    if needle not in seal:
        raise SystemExit(f"missing R6-S3A final source-seal contract: {needle}")
if "lower_delegate_exposes" not in parser:
    raise SystemExit("rich parse path must run delegate postpass before final seal")

if re.search(r"derive\([^)]*Clone[^)]*\)\s*pub\(super\) struct ParserBoxSourceSealV1", seal):
    raise SystemExit("ParserBoxSourceSealV1 must remain non-Clone")
if re.search(
    r"impl\s+ParserBoxSourceSealV1\s*\{.*?pub(?:\(crate\)|\(super\)|\(in\s+crate::parser\))?\s+fn\s+new\s*\(",
    seal,
    re.S,
):
    raise SystemExit("R6-S3A final source seal must not expose a constructor")

if "register_prepared_source_seal" not in box or "state.source_tx.finish()" not in box:
    raise SystemExit("ordinary Box close must register one prepared source seal")

for forbidden in (
    "method_source_member_ordinals",
    "record_new_methods_since",
    "try_merge_selected_gate(selected, &[u32]",
    "crate::mir",
    "crate::resolver",
):
    if forbidden in (parser + authority + seal + box):
        raise SystemExit(f"R6-S3A forbidden legacy/semantic coupling remains: {forbidden}")

for path in (parser_path, authority_path, *seal_paths, box_path):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

print("rich_parse_entry=1")
print("finalizer_after_delegate_postpass=1")
print("non_clone_final_seal=1")
print("seal_constructor=0")
print("ordinary_transaction_registration=1")
print("legacy_sidecars=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
