#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s1"
SOURCE="$ROOT/src/parser/source_authority.rs"
PARSER_MOD="$ROOT/src/parser/mod.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SOURCE" "$PARSER_MOD"

python3 - "$ROOT" "$SOURCE" "$PARSER_MOD" <<'PY'
import re
import sys
from pathlib import Path

root, source_path, parser_mod_path = map(Path, sys.argv[1:])
source = source_path.read_text(encoding="utf-8")
parser_mod = parser_mod_path.read_text(encoding="utf-8")

if len(source.splitlines()) >= 800:
    raise SystemExit(f"source must remain below 800 lines: {source_path}")
if "mod source_authority;" not in parser_mod:
    raise SystemExit("parser-private source authority module is not declared")

required = (
    "ParserInvocationBrandV1",
    "SourceBoxDeclarationSiteV1",
    "SourceBoxMemberSiteV1",
    "SourceBoxGateSelectionV1",
    "SourceBoxMethodSiteV1",
    "OpenBoxMethodSourceTransactionV1",
    "PreparedBoxSourceSealV1",
    "ParserBoxSourceSealV1",
    "ForeignBoxSite",
    "MemberOrdinalOverflow",
    "BoxMethodInventoryErrorV1",
    "try_push_explicit_source",
    "finish(self) -> PreparedBoxSourceSealV1",
    "StaleMemberSite",
)
for needle in required:
    if needle not in source:
        raise SystemExit(f"missing R6-S1 contract: {needle}")

if "impl Clone for ParserBoxSourceSealV1" in source:
    raise SystemExit("final parser source seal must remain non-Clone")
if re.search(r"impl\s+ParserBoxSourceSealV1\s*\{", source):
    raise SystemExit("final parser source seal must not have a constructor in R6-S1")
if "pub struct ParserBoxSourceSealV1" in source:
    raise SystemExit("final parser source seal must remain parser-private")

# S1 is only a disconnected substrate. The parser branch, resolver, and
# postpass must not consume the new types until the later rich-output row.
parser_root = root / "src/parser"
for path in parser_root.rglob("*.rs"):
    if path == source_path or path == parser_mod_path:
        continue
    text = path.read_text(encoding="utf-8")
    for needle in (
        "source_authority",
        "ParserInvocationBrandV1",
        "OpenBoxMethodSourceTransactionV1",
        "ParserBoxSourceSealV1",
    ):
        if needle in text:
            raise SystemExit(f"R6-S1 source authority connected early: {path}: {needle}")

for forbidden in (
    "lower_delegate_exposes",
    "prune_build_gate_program",
    "Resolver",
    "Recipe",
    "MirBuilder",
    "source_slice",
    "MapBox",
):
    if forbidden in source:
        raise SystemExit(f"forbidden R6-S1 dependency: {forbidden}")

print("parser_invocation_brand=1")
print("source_site_types=1")
print("transaction_prepared_seal=1")
print("final_seal_constructor=0")
print("parser_postpass_connection=0")
print("resolver_connection=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
