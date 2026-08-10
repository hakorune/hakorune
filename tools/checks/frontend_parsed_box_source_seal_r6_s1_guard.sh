#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s1"
SOURCE="$ROOT/src/parser/source_authority.rs"
SEAL_MOD="$ROOT/src/parser/source_seal/mod.rs"
SEAL_MODEL="$ROOT/src/parser/source_seal/model.rs"
SEAL_GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
SEAL_FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
PARSER_MOD="$ROOT/src/parser/mod.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SOURCE" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$PARSER_MOD"

python3 - "$ROOT" "$SOURCE" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$PARSER_MOD" <<'PY'
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
source_path = Path(sys.argv[2])
seal_paths = list(map(Path, sys.argv[3:7]))
parser_mod_path = Path(sys.argv[7])
source = source_path.read_text(encoding="utf-8")
seal = "\n".join(path.read_text(encoding="utf-8") for path in seal_paths)
parser_mod = parser_mod_path.read_text(encoding="utf-8")

if len(source.splitlines()) >= 800:
    raise SystemExit(f"source must remain below 800 lines: {source_path}")
for seal_path in seal_paths:
    if len(seal_path.read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {seal_path}")
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
    if needle not in (source + seal):
        raise SystemExit(f"missing R6-S1 contract: {needle}")

if "impl Clone for ParserBoxSourceSealV1" in seal:
    raise SystemExit("final parser source seal must remain non-Clone")
if re.search(r"impl\s+ParserBoxSourceSealV1\s*\{[^}]*\bfn\s+new\s*\(", seal, re.S):
    raise SystemExit("final parser source seal must not have a constructor")
if "pub struct ParserBoxSourceSealV1" in seal:
    raise SystemExit("final parser source seal must remain parser-private")

for forbidden in (
    "lower_delegate_exposes",
    "prune_build_gate_program",
    "Resolver",
    "Recipe",
    "MirBuilder",
    "source_slice",
    "MapBox",
):
    if forbidden in (source + seal):
        raise SystemExit(f"forbidden R6-S1 dependency: {forbidden}")

print("parser_invocation_brand=1")
print("source_site_types=1")
print("transaction_prepared_seal=1")
print("final_seal_constructor=0")
print("parser_postpass_connection=historical-later-rows-allowed")
print("resolver_connection=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
