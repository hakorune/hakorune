#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-parser-box-declaration-h1"
DIR="$ROOT/lang/src/compiler/parser/source_carrier_v1"
FIXTURE="$ROOT/tools/checks/fixtures/parser_box_declaration_h1_v1.hako"
BIN="$ROOT/target/release/hakorune"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" \
  "$DIR/source_declaration_refs_v1.hako" \
  "$DIR/parser_source_session_v1.hako" \
  "$DIR/source_declaration_records_v1.hako" \
  "$DIR/source_declaration_sealer_v1.hako" \
  "$DIR/source_declaration_builder_v1.hako" \
  "$FIXTURE"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune

vm_log=/tmp/hakorune-parser-box-declaration-h1.vm.log
timeout 10s env NYASH_DISABLE_PLUGINS=1 "$BIN" --backend vm "$FIXTURE" >"$vm_log" 2>&1 || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "H1 fixture failed or exceeded 10 seconds"
}
rg -q 'RC: 0' "$vm_log" || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "H1 fixture returned nonzero"
}

python3 - "$ROOT" "$DIR" "$FIXTURE" <<'PY'
import sys
from pathlib import Path

root, source_dir, fixture = map(Path, sys.argv[1:])
paths = [
    source_dir / "source_declaration_refs_v1.hako",
    source_dir / "parser_source_session_v1.hako",
    source_dir / "source_declaration_records_v1.hako",
    source_dir / "source_declaration_sealer_v1.hako",
    source_dir / "source_declaration_builder_v1.hako",
    fixture,
]
for path in paths:
    lines = len(path.read_text(encoding="utf-8").splitlines())
    if lines >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path} has {lines}")

joined = "\n".join(path.read_text(encoding="utf-8") for path in paths)
required = (
    "ParserCarrierBrandV1", "ParserSourceUnitRefV1",
    "ParserProgramStatementSiteV1", "ParserBoxDeclarationSiteV1", "ParserBoxMemberSiteV1",
    "ParserBoxMethodSourceSiteV1", "ParserBoxInventoryOrdinalV1",
    "ParserProgramSourceSessionV1", "ParserBoxMemberSourceCursorV1",
    "ParserBoxDeclarationBuilderV1", "SealedParserBoxDeclarationV1",
    "duplicate_method", "double_finish", "SelectedBuildGate",
    "PropertyGetter", "positive_two_box_session", "foreign_session_and_box_reject",
    "finish_with_live_cursor", "foreign_cursor", "mutation_after_close",
)
for needle in required:
    if needle not in joined:
        raise SystemExit(f"missing H1 contract: {needle}")
for forbidden in (
    "CallableContract", "Resolver", "Recipe", "MIR",
    "FuncScannerBox", "StageBRuneBox", "source_slice", "MapBox",
    "JsonParser",
    "ParserDeclarationSiteIssuerV1", "_next_member: Integer\n  birth(token)",
):
    if forbidden in joined:
        raise SystemExit(f"forbidden H1 dependency or reserved spelling: {forbidden}")

fixture_text = fixture.read_text(encoding="utf-8")
for forbidden in (
    "ParserDeclarationRefsV1Box.carrier",
    "ParserDeclarationRefsV1Box.source_unit",
    "ParserDeclarationRefsV1Box.program_site",
    "ParserDeclarationRefsV1Box.box_site",
    "ParserDeclarationRefsV1Box.direct_member",
    "ParserDeclarationRefsV1Box.selected_member",
):
    if forbidden in fixture_text:
        raise SystemExit(f"fixture bypasses program-owned source session: {forbidden}")

parser_root = root / "lang/src/compiler/parser"
session_path = source_dir / "parser_source_session_v1.hako"
raw_session_factories = (
    "ParserDeclarationRefsV1Box.carrier",
    "ParserDeclarationRefsV1Box.source_unit",
    "ParserDeclarationRefsV1Box.program_site",
    "ParserDeclarationRefsV1Box.box_site",
    "ParserDeclarationRefsV1Box.direct_member",
    "ParserDeclarationRefsV1Box.selected_member",
)
for path in parser_root.rglob("*.hako"):
    text = path.read_text(encoding="utf-8")
    if path != session_path:
        for needle in raw_session_factories:
            if needle in text:
                raise SystemExit(f"raw source-site factory bypasses program session: {path}: {needle}")
    if source_dir in path.parents:
        continue
    if "source_declaration_builder_v1" in text or "SealedParserBoxDeclarationV1" in text:
        raise SystemExit(f"H1 declaration carrier connected to parser branch: {path}")

print("parser_branch_connection=0")
print("resolver_semantic_publication=0")
print("inventory_ordinal_source_identity=0")
print("program_owned_source_session=1")
print("box_scoped_member_cursor=1")
print("duplicate_partial_publication=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
