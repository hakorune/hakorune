#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-bounded-body-snapshot-schema-v0"
source "$ROOT/tools/checks/lib/guard_common.sh"

RUST_ROOT="$ROOT/src/analysis/bounded_body_snapshot_v0"
HAKO_SCHEMA="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot_schema_v0.hako"
INVENTORY="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/program-v0-wire-contract-inventory-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" \
  "$RUST_ROOT/schema.rs" "$RUST_ROOT/path.rs" "$RUST_ROOT/budget.rs" \
  "$RUST_ROOT/outcome.rs" "$RUST_ROOT/snapshot.rs" "$HAKO_SCHEMA" "$INVENTORY"

cargo test -q analysis::bounded_body_snapshot_v0

python3 - "$RUST_ROOT" "$HAKO_SCHEMA" "$INVENTORY" <<'PY'
import json
import re
import sys
from pathlib import Path

rust_root = Path(sys.argv[1])
hako = Path(sys.argv[2]).read_text(encoding="utf-8")
inventory = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
rust = "\n".join(path.read_text(encoding="utf-8") for path in rust_root.glob("*.rs"))

limits = {
    "max_depth": 64,
    "max_node_count": 32768,
    "max_children_per_body": 2048,
    "max_arguments": 128,
    "max_literal_bytes": 65536,
    "max_atom_bytes": 1024,
    "max_total_text_bytes": 4194304,
}
for name, value in limits.items():
    rust_value = f"{value:_}"
    if not re.search(rf"\b{name}:\s*(?:{value}|{re.escape(rust_value)})\b", rust):
        raise SystemExit(f"Rust limit drift: {name}")
    if f"{name}() {{ return {value} }}" not in hako:
        raise SystemExit(f"Hako limit drift: {name}")

rows = inventory.get("rows") or []
for row in rows:
    tag = row["tag"]
    classification = row["classification"]
    needle = f'if kind == "{tag}" {{ return "{classification}" }}'
    if needle not in hako:
        raise SystemExit(f"Hako classification missing: {row['domain']}:{tag}")

atom_rows = {
    ("stmt", "Local"): "name:Text:Atom",
    ("stmt", "Expr"): "",
    ("stmt", "If"): "",
    ("stmt", "Loop"): "",
    ("stmt", "LoopRange"): "var_name:Text:Atom",
    ("stmt", "Return"): "",
    ("stmt", "Break"): "",
    ("stmt", "Continue"): "",
    ("expr", "Int"): "value:I64:-",
    ("expr", "Str"): "value:Text:Literal",
    ("expr", "Bool"): "value:Bool:-",
    ("expr", "Null"): "value:Null:-",
    ("expr", "Var"): "name:Text:Atom",
    ("expr", "Binary"): "op:Text:Atom",
    ("expr", "Compare"): "op:Text:Atom",
    ("expr", "Logical"): "op:Text:Atom",
    ("expr", "Call"): "name:Text:Atom",
    ("expr", "Method"): "method:Text:Atom",
    ("expr", "Field"): "field:Text:Atom",
}
child_rows = {
    ("stmt", "Local"): "expr:One",
    ("stmt", "Expr"): "expr:One",
    ("stmt", "If"): "cond:One,then:List,else:OptionalList",
    ("stmt", "Loop"): "cond:One,body:List",
    ("stmt", "LoopRange"): "start:One,end:One,body:List",
    ("stmt", "Return"): "expr:One",
    ("stmt", "Break"): "",
    ("stmt", "Continue"): "",
    ("expr", "Int"): "",
    ("expr", "Str"): "",
    ("expr", "Bool"): "",
    ("expr", "Null"): "",
    ("expr", "Var"): "",
    ("expr", "Binary"): "lhs:One,rhs:One",
    ("expr", "Compare"): "lhs:One,rhs:One",
    ("expr", "Logical"): "lhs:One,rhs:One",
    ("expr", "Call"): "args:List",
    ("expr", "Method"): "recv:One,args:List",
    ("expr", "Field"): "recv:One",
}
for (_, kind), encoding in atom_rows.items():
    if encoding and f'kind == "{kind}"' not in hako:
        raise SystemExit(f"Hako atom kind missing: {kind}")
    if encoding and f'return "{encoding}"' not in hako:
        raise SystemExit(f"Hako atom schema missing: {kind}={encoding}")
for (_, kind), encoding in child_rows.items():
    if encoding and f'return "{encoding}"' not in hako:
        raise SystemExit(f"Hako child schema missing: {kind}={encoding}")
if 'root_body_depth() { return 0 }' not in hako or 'top_level_node_depth() { return 1 }' not in hako:
    raise SystemExit("Hako depth convention drift")
for field in ("body", "type", "expr", "cond", "then", "else", "start", "end", "lhs", "rhs", "recv", "args", "name", "method", "field", "var_name", "op", "value"):
    if f'field == "{field}"' not in hako:
        raise SystemExit(f"Hako path field missing: {field}")

for forbidden in ("crate::ast", "crate::mir", "crate::runner", "crate::stage1", "planner", "backend", "runtime"):
    if forbidden in rust:
        raise SystemExit(f"Rust dependency boundary violated: {forbidden}")
if "indexOf" in hako or "substring(" in hako:
    raise SystemExit("Hako raw scanner dependency detected")
using_lines = "\n".join(
    line.strip().lower() for line in hako.splitlines() if line.strip().startswith("using ")
)
for forbidden in ("mirbuilder", "route", "runtime"):
    if forbidden in using_lines:
        raise SystemExit(f"Hako dependency boundary violated: {forbidden}")

for path in rust_root.glob("*.rs"):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"Rust source exceeds 800 lines: {path}")
if len(hako.splitlines()) > 800:
    raise SystemExit("Hako schema exceeds 800 lines")

print("output_contract=BoundedBodySnapshotSchemaV0")
print("rust_hako_schema_parity=1")
print("ordered_atom_schema_parity=1")
print("ordered_child_schema_parity=1")
print("closed_path_depth_parity=1")
print("analysis_only_dependency_boundary=1")
print("source_files_under_800=1")
print("summary=ok")
PY
