#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

MANIFEST="docs/development/current/main/design/static-data-manifest-v0.toml"
GENERATOR="tools/backend_static_data_manifest_codegen.py"
GENERATED="lang/src/shared/backend/ll_emit/generated/static_data_defaults.hako"
REGISTRY="lang/src/shared/backend/ll_emit/static_data_registry_box.hako"
EMITTER="lang/src/shared/backend/ll_emit/ll_text_emit_box.hako"
MODULES="lang/src/shared/hako_module.toml"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
MANUAL="docs/reference/runtime/substrate-capabilities.md"

echo "[k2-wide-static-data-first-row] running static readonly data first-row guard"

python3 "$GENERATOR" --check

rg -F -q 'symbol = ".hako_size_class_u16_v0"' "$MANIFEST"
rg -F -q 'element = "u16"' "$MANIFEST"
rg -F -q 'purpose = "mimalloc-size-class-fixture"' "$MANIFEST"
rg -F -q 'row_0.set("symbol", ".hako_size_class_u16_v0")' "$GENERATED"
rg -F -q 'values_0.push("8")' "$GENERATED"
rg -F -q 'values_0.push("65535")' "$GENERATED"

rg -F -q 'static box StaticDataRegistryBox' "$REGISTRY"
rg -F -q 'build_global_line(row)' "$REGISTRY"
rg -F -q 'emit_globals()' "$REGISTRY"
rg -F -q 'emit_globals_for_root(root)' "$REGISTRY"
rg -F -q 'using selfhost.shared.backend.ll_emit.static_data_registry as StaticDataRegistryBox' "$EMITTER"
rg -F -q 'StaticDataRegistryBox.emit_globals_for_root(root)' "$EMITTER"
rg -F -q 'backend.ll_emit.static_data_registry = "backend/ll_emit/static_data_registry_box.hako"' "$MODULES"
rg -F -q 'backend.ll_emit.generated.static_data_defaults = "backend/ll_emit/generated/static_data_defaults.hako"' "$MODULES"

python3 - <<'PY'
import pathlib
import tomllib

root = pathlib.Path.cwd()
data = tomllib.loads((root / "docs/development/current/main/design/static-data-manifest-v0.toml").read_text())
rows = data.get("rows", [])
assert len(rows) == 1, rows
row = rows[0]
values = row["values"]
assert len(values) == 64, len(values)
assert all(isinstance(v, int) for v in values), values
assert all(0 <= v <= 65535 for v in values), values
ty = {"u8": "i8", "u16": "i16", "u32": "i32", "u64": "i64"}[row["element"]]
line = (
    f"@{row['symbol']} = {row['linkage']} unnamed_addr "
    f"constant [{len(values)} x {ty}] ["
    + ", ".join(f"{ty} {v}" for v in values)
    + f"], align {row['align']}"
)
assert "@.hako_size_class_u16_v0 = private unnamed_addr constant [64 x i16]" in line
assert "i16 8, i16 16" in line
assert line.endswith("align 2")
print("[k2-wide-static-data-first-row] rendered global line ok")
PY

rg -F -q '`M10c-pre pointer/handle return proof vocabulary`' "$TASKBOARD"
rg -F -q '`M11a static readonly data segment`' "$TASKBOARD"
rg -F -q '`M11b const eval/static table syntax`' "$TASKBOARD"
rg -F -q 'Static Readonly Data Segment Row' "$MANUAL"

echo "[k2-wide-static-data-first-row] ok"
