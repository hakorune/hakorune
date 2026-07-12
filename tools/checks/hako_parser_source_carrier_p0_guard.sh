#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-parser-source-carrier-p0"
DIR="$ROOT/lang/src/compiler/parser/source_carrier_v1"
FIXTURE="$ROOT/tools/checks/fixtures/parser_source_carrier_p0_v1.hako"
BIN="$ROOT/target/release/hakorune"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$DIR/README.md" "$DIR/source_vocabulary_v1.hako" \
  "$DIR/source_refs_v1.hako" "$DIR/source_records_v1.hako" \
  "$DIR/parser_node_product_v1.hako" "$DIR/source_carrier_builder_v1.hako" \
  "$DIR/source_carrier_outcome_v1.hako" "$DIR/source_carrier_sealer_v1.hako" "$FIXTURE"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune

compile_log=/tmp/hakorune-parser-source-carrier-p0.compile.log
mir_out=/tmp/hakorune-parser-source-carrier-p0.mir
timeout 10s env NYASH_DISABLE_PLUGINS=1 NYASH_MIR_COMPILE_TRACE=1 \
  "$BIN" --dump-mir --no-optimize "$FIXTURE" >"$mir_out" 2>"$compile_log" || {
  tail -n 100 "$compile_log" >&2
  guard_fail "$TAG" "release compile fixture failed or exceeded 10 seconds"
}
rg -q 'stage=build_module' "$compile_log" || guard_fail "$TAG" "build_module trace missing"
rg -q 'stage=semantic_refresh' "$compile_log" || guard_fail "$TAG" "semantic_refresh trace missing"

vm_log=/tmp/hakorune-parser-source-carrier-p0.vm.log
timeout 10s env NYASH_DISABLE_PLUGINS=1 "$BIN" --backend vm "$FIXTURE" >"$vm_log" 2>&1 || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "VM fixture failed or exceeded 10 seconds"
}
rg -q 'RC: 0' "$vm_log" || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "VM fixture returned nonzero"
}

python3 - "$ROOT" "$DIR" "$FIXTURE" <<'PY'
import sys
from pathlib import Path

root, source_dir, fixture = map(Path, sys.argv[1:])
sources = sorted(source_dir.glob("*.hako"))
paths = sources + [fixture]
if len(sources) != 7:
    raise SystemExit(f"expected seven P0 Hako modules, got {len(sources)}")
for path in paths:
    lines = len(path.read_text(encoding="utf-8").splitlines())
    if lines >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path} has {lines}")

joined = "\n".join(path.read_text(encoding="utf-8") for path in sources)
required = (
    "ParserNodeProductV1", "SourceNodeRefV1", "SourceNodeListRefV1",
    "SourceCarrierBuilderV1", "HakoSourceTreeV1", "Open", "Poisoned", "Sealed",
    "InternalCarrierContractViolation", "ProgramV0CompatProjectionViolation",
    "internal.carrier.unreachable_node", "internal.carrier.double_finish",
    "internal.carrier.list_mutation_after_seal", "internal.carrier.node_ref_invalid",
)
for needle in required:
    if needle not in joined:
        raise SystemExit(f"missing P0 carrier contract: {needle}")
for forbidden in ("MapBox", "JsonParser", "indexOf", "substring(", "MIRBuilder", "planner.", "route."):
    if forbidden in joined:
        raise SystemExit(f"forbidden P0 carrier dependency: {forbidden}")

product = (source_dir / "parser_node_product_v1.hako").read_text(encoding="utf-8")
for path in sources:
    if path.name != "parser_node_product_v1.hako" and "compat_fragment" in path.read_text(encoding="utf-8"):
        raise SystemExit(f"compat fragment escaped explicit parser product: {path}")

construction_allow = {
    "SourceNodeRefV1": source_dir / "source_refs_v1.hako",
    "SourceNodeListRefV1": source_dir / "source_refs_v1.hako",
    "SourceNodeRecordV1": source_dir / "source_records_v1.hako",
    "SourceNodeListRecordV1": source_dir / "source_records_v1.hako",
    "HakoSourceTreeV1": source_dir / "source_records_v1.hako",
}
for token, owner in construction_allow.items():
    spelling = f"new {token}"
    for path in sources + [fixture]:
        if path != owner and spelling in path.read_text(encoding="utf-8"):
            raise SystemExit(f"factory-only construction escaped: {path}: {spelling}")

parser_root = root / "lang/src/compiler/parser"
for path in parser_root.rglob("*.hako"):
    if source_dir in path.parents:
        continue
    text = path.read_text(encoding="utf-8")
    if "source_carrier_v1" in text or "ParserNodeProductV1" in text:
        raise SystemExit(f"P0 carrier connected to parser branch: {path}")

fixture_text = fixture.read_text(encoding="utf-8")
for needle in (
    "success_tree", "products", "invalid_ref_poison", "wrong_family_poison",
    "mutation_after_list_seal", "incomplete_root", "unreachable_node",
    "cycle_attempt", "double_finish_and_closed_mutation",
):
    if needle not in fixture_text:
        raise SystemExit(f"missing lifecycle fixture: {needle}")

print("parser_branch_connection=0")
print("program_v0_behavior_change=0")
print("typed_branch_count=0")
print("partial_tree_publication=0")
print("raw_map_json_carrier=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
