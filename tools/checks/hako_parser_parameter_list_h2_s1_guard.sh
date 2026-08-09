#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-parser-parameter-list-h2-s1"
DIR="$ROOT/lang/src/compiler/parser/source_carrier_v1"
FIXTURE="$ROOT/tools/checks/fixtures/parser_parameter_list_h2_s1_v1.hako"
BIN="$ROOT/target/release/hakorune"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" \
  "$DIR/parameter_syntax_records_v1.hako" \
  "$DIR/parameter_list_sealer_v1.hako" \
  "$DIR/parameter_list_builder_v1.hako" \
  "$FIXTURE"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune

vm_log=/tmp/hakorune-parser-parameter-list-h2-s1.vm.log
timeout 10s env NYASH_DISABLE_PLUGINS=1 "$BIN" --backend vm "$FIXTURE" >"$vm_log" 2>&1 || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "H2-S1 fixture failed or exceeded 10 seconds"
}
rg -q 'RC: 0' "$vm_log" || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "H2-S1 fixture returned nonzero"
}

python3 - "$ROOT" "$DIR" "$FIXTURE" <<'PY'
import sys
from pathlib import Path

root, source_dir, fixture = map(Path, sys.argv[1:])
sources = [
    source_dir / "parameter_syntax_records_v1.hako",
    source_dir / "parameter_list_sealer_v1.hako",
    source_dir / "parameter_list_builder_v1.hako",
]
paths = sources + [fixture]
for path in paths:
    lines = len(path.read_text(encoding="utf-8").splitlines())
    if lines >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path} has {lines}")

joined = "\n".join(path.read_text(encoding="utf-8") for path in sources)
required = (
    "ParserParameterSyntaxIssuerSealV1", "ParserParameterTransferKindV1",
    "ParserParameterTransferSyntaxV1",
    "ParserDeclaredParameterTypeKindV1", "ParserDeclaredParameterTypeSyntaxV1",
    "ParserParameterSourceSiteV1",
    "ParserParameterSyntaxRowV1", "ParserNeutralParameterDeclV1",
    "ParserParameterListProductV1", "ParserParameterListBuilderV1",
    "ParserParameterListSealerV1Box", "Ordinary", "Take", "Absent", "Explicit",
    "accepts_ordinary", "is_absent",
    "duplicate_name", "double_finish", "mutation_after_close",
)
for needle in required:
    if needle not in joined:
        raise SystemExit(f"missing H2-S1 contract: {needle}")
for forbidden in (
    "HomeDemand", "HomeAbi", "Resolver", "Recipe", "MIR",
    "CallableContract", "FuncScanner", "JsonParser", "ParamDecl",
    " clone", "clone_",
):
    if forbidden in joined:
        raise SystemExit(f"forbidden H2-S1 semantic or reconstruction dependency: {forbidden}")

construction_allow = {
    "ParserParameterSyntaxIssuerSealV1": sources[0],
    "ParserParameterTransferSyntaxV1": sources[0],
    "ParserDeclaredParameterTypeSyntaxV1": sources[0],
    "ParserParameterSourceSiteV1": sources[0],
    "ParserParameterSyntaxRowV1": sources[0],
    "ParserNeutralParameterDeclV1": sources[0],
    "ParserParameterListProductV1": sources[0],
    "ParserParameterListFinishV1": sources[0],
    "ParserParameterListBuilderV1": sources[2],
}
for token, owner in construction_allow.items():
    spelling = f"new {token}"
    for path in paths:
        if path != owner and spelling in path.read_text(encoding="utf-8"):
            raise SystemExit(f"factory-only parameter construction escaped: {path}: {spelling}")

parser_root = root / "lang/src/compiler/parser"
for path in parser_root.rglob("*.hako"):
    if source_dir in path.parents:
        continue
    text = path.read_text(encoding="utf-8")
    if "parameter_list_builder_v1" in text or "ParserParameterListProductV1" in text:
        raise SystemExit(f"H2-S1 parameter product connected to parser branch: {path}")

fixture_text = fixture.read_text(encoding="utf-8")
for needle in (
    "empty_exact_list", "ordered_ordinary_rows", "duplicate_and_empty_name_reject",
    "untyped_ordinary_is_explicit_absence",
    "foreign_and_ordinal_reject", "closed_state_rejects",
):
    if needle not in fixture_text:
        raise SystemExit(f"missing H2-S1 lifecycle fixture: {needle}")

records_text = sources[0].read_text(encoding="utf-8")
for path in paths:
    text = path.read_text(encoding="utf-8")
    if path != sources[0] and "ParserParameterTransferKindV1::" in text:
        raise SystemExit(f"closed transfer vocabulary escaped its issuer: {path}")
for forbidden in (
    '.kind()', '== "Ordinary"', '!= "Ordinary"',
    '== "Take"', '!= "Take"', 'take_transfer()',
):
    if forbidden in joined:
        raise SystemExit(f"raw or prematurely active transfer authority: {forbidden}")
if "ParserParameterTransferKindV1::Take(" in records_text:
    raise SystemExit("Take vocabulary exists but must have no R0a issuer")

print("parser_branch_connection=0")
print("take_syntax_construction=0")
print("raw_transfer_string_authority=0")
print("untyped_parameter_explicit_absence=1")
print("resolver_home_semantics=0")
print("parameter_ordinals_source_ordered=1")
print("neutral_projection_one_way=1")
print("partial_product_publication=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
