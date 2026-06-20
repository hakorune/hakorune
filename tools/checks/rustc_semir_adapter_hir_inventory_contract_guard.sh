#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL_DIR="$ROOT_DIR/tools/rust_lifecycle/rustc_semir_adapter"

test -f "$TOOL_DIR/Cargo.toml"
test -f "$TOOL_DIR/rust-toolchain.toml"

if rg -n "rustc_(driver|interface|hir|middle|mir|span)|rustc-private|rustc_private" \
  "$ROOT_DIR/Cargo.toml" "$ROOT_DIR/crates" "$ROOT_DIR/src" >/tmp/rustc_semir_product_rustc_private_hits 2>/dev/null; then
  cat /tmp/rustc_semir_product_rustc_private_hits >&2
  exit 1
fi

bash "$ROOT_DIR/tools/checks/rustc_semir_adapter_pinned_nightly_preflight_guard.sh" >/dev/null

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$TMP_DIR/src"
cat >"$TMP_DIR/src/lib.rs" <<'RS'
pub mod model;

fn private_helper(value: i64) -> i64 {
    value + 1
}

pub fn add(a: i64, b: i64) -> i64 {
    private_helper(a) + b
}
RS

cat >"$TMP_DIR/src/model.rs" <<'RS'
pub struct Point {
    pub x: i64,
    pub y: i64,
}
RS

OUTPUT_JSON="$TMP_DIR/hir-inventory.json"
(
  cd "$TOOL_DIR"
  SYSROOT="$(rustc --print sysroot)"
  HOST="$(rustc -Vv | awk '/^host:/ { print $2 }')"
  LIB_DIR="$SYSROOT/lib/rustlib/$HOST/lib"
  export LD_LIBRARY_PATH="$LIB_DIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  cargo run --quiet --features rustc-private -- --hir-inventory-contract "$TMP_DIR/src/lib.rs"
) >"$OUTPUT_JSON"

python3 - "$OUTPUT_JSON" <<'PY'
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text())
assert report["output_contract"] == "rustc-semir-adapter-hir-inventory-contract-v0"
assert report["schema_version"] == 0
assert report["kind"] == "RustcSemirHirInventory"
assert report["id_policy"] == "canonical-rust-path-v0"
assert report["ordering_policy"] == "module-id-and-source-order-v0"
assert report["crate"]["root_module_id"] == "crate"
assert report["crate"]["root_source_path"] == "lib.rs"

modules = {module["module_id"]: module for module in report["modules"]}
assert "crate" in modules
assert "crate::model" in modules
assert modules["crate"]["parent_module_id"] is None
assert modules["crate::model"]["parent_module_id"] == "crate"
assert modules["crate::model"]["path_segments"] == ["crate", "model"]

definitions = {definition["semantic_id"]: definition for definition in report["definitions"] if definition["semantic_id"]}
assert "type:crate::model::Point" in definitions
assert "value:crate::add" in definitions
assert "value:crate::private_helper" in definitions

point = definitions["type:crate::model::Point"]
assert point["kind"] == "Struct"
assert point["namespace"] == "type"
assert point["module_id"] == "crate::model"
assert point["declared_visibility"]["kind"] == "public"
assert point["source"]["path"] == "model.rs"

add = definitions["value:crate::add"]
assert add["kind"] == "Fn"
assert add["module_id"] == "crate"
assert add["declared_visibility"]["kind"] == "public"

private_helper = definitions["value:crate::private_helper"]
assert private_helper["declared_visibility"]["kind"] == "crate"
assert private_helper["declared_visibility"]["scope_module_id"] == "crate"

for source in [module["source"] for module in report["modules"]]:
    assert not source["path"].startswith("/")
for definition in report["definitions"]:
    assert not definition["source"]["path"].startswith("/")

assert report["coverage"]["module_count"] >= 2
assert report["coverage"]["definition_count"] >= 3
assert report["coverage"]["semantic_id_missing_count"] == 0
assert report["coverage"]["absolute_source_paths"] == 0

claims = report["claims"]
assert claims["THIR_extracted"] == 0
assert claims["MIR_or_borrowck_extracted"] == 0
assert claims["drop_elaboration_extracted"] == 0
assert claims["RustLifecycleAdapterFacts_generated"] == 0
assert claims["hako_plan_emitted"] == 0
assert claims["hako_source_emitted"] == 0
assert claims["backend_behavior_changed"] == 0
PY

NIGHTLY_TARGET="$TMP_DIR/nightly-target"
CARGO_TARGET_DIR="$NIGHTLY_TARGET" cargo +nightly-2026-06-20 build -q -p hakorune-mir-core
CORE_RLIB="$(ls "$NIGHTLY_TARGET"/debug/deps/libhakorune_mir_core-*.rlib | head -1)"
REAL_JSON="$TMP_DIR/hakorune-mir-builder-hir-inventory.json"
(
  cd "$TOOL_DIR"
  SYSROOT="$(rustc --print sysroot)"
  HOST="$(rustc -Vv | awk '/^host:/ { print $2 }')"
  LIB_DIR="$SYSROOT/lib/rustlib/$HOST/lib"
  export LD_LIBRARY_PATH="$LIB_DIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  cargo run --quiet --features rustc-private -- --hir-inventory-contract \
    "$ROOT_DIR/crates/hakorune_mir_builder/src/lib.rs" \
    --extern "hakorune_mir_core=$CORE_RLIB"
) >"$REAL_JSON"

python3 - "$REAL_JSON" <<'PY'
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text())
modules = {module["module_id"]: module for module in report["modules"]}
expected_modules = {
    "crate",
    "crate::binding_context",
    "crate::context",
    "crate::core_context",
    "crate::metadata_context",
    "crate::type_context",
    "crate::variable_context",
}
assert expected_modules.issubset(modules)
definitions = {
    definition["semantic_id"]
    for definition in report["definitions"]
    if definition["semantic_id"]
}
assert "type:crate::binding_context::BindingContext" in definitions
assert "type:crate::core_context::CoreContext" in definitions
assert report["coverage"]["module_count"] == 7
assert report["coverage"]["absolute_source_paths"] == 0
claims = report["claims"]
assert claims["THIR_extracted"] == 0
assert claims["MIR_or_borrowck_extracted"] == 0
assert claims["RustLifecycleAdapterFacts_generated"] == 0
assert claims["hako_plan_emitted"] == 0
assert claims["hako_source_emitted"] == 0
assert claims["backend_behavior_changed"] == 0
PY

cat <<'REPORT'
output_contract=rustc-semir-adapter-hir-inventory-contract-guard-v0
pinned_nightly_preflight_guard_green=1
hir_inventory_json_contract_v0=1
schema_version=0
kind=RustcSemirHirInventory
module_hierarchy_truthful=1
definition_owner_relation=1
declared_visibility_normalized=1
source_paths_crate_relative=1
absolute_source_paths=0
deterministic_ordering=1
synthetic_golden_green=1
hakorune_mir_builder_smoke_green=1
THIR_extracted=0
MIR_or_borrowck_extracted=0
drop_elaboration_extracted=0
RustLifecycleAdapterFacts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
summary=ok
REPORT
