#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL_DIR="$ROOT_DIR/tools/rust_lifecycle/rustc_semir_adapter"

test -f "$TOOL_DIR/Cargo.toml"
test -f "$TOOL_DIR/rust-toolchain.toml"

if rg -n "rustc_(driver|interface|hir|middle|mir|span|hir_analysis)|rustc-private|rustc_private" \
  "$ROOT_DIR/Cargo.toml" "$ROOT_DIR/crates" "$ROOT_DIR/src" >/tmp/rustc_semir_product_rustc_private_hits 2>/dev/null; then
  cat /tmp/rustc_semir_product_rustc_private_hits >&2
  exit 1
fi

bash "$ROOT_DIR/tools/checks/rustc_semir_adapter_hir_inventory_contract_guard.sh" >/dev/null

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

NIGHTLY_TARGET="$TMP_DIR/nightly-target"
CARGO_TARGET_DIR="$NIGHTLY_TARGET" cargo +nightly-2026-06-20 build -q -p hakorune-mir-core
CORE_RLIB="$(ls "$NIGHTLY_TARGET"/debug/deps/libhakorune_mir_core-*.rlib | head -1)"
REPORT_JSON="$TMP_DIR/binding-context-thir.json"

(
  cd "$TOOL_DIR"
  SYSROOT="$(rustc --print sysroot)"
  HOST="$(rustc -Vv | awk '/^host:/ { print $2 }')"
  LIB_DIR="$SYSROOT/lib/rustlib/$HOST/lib"
  export LD_LIBRARY_PATH="$LIB_DIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  cargo run --quiet --features rustc-private -- --binding-context-thir-body-inventory \
    "$ROOT_DIR/crates/hakorune_mir_builder/src/lib.rs" \
    --extern "hakorune_mir_core=$CORE_RLIB"
) >"$REPORT_JSON"

python3 - "$REPORT_JSON" <<'PY'
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text())
assert report["output_contract"] == "rustc-semir-adapter-binding-context-thir-body-inventory-v0"
assert report["schema_version"] == 0
assert report["kind"] == "RustcSemirBindingContextThirBodyInventory"
assert report["family"] == "BindingContext"

bodies = {body["hir_owner_reference"]: body for body in report["bodies"]}
required = {
    "body:crate::binding_context::BindingContext::new",
    "body:crate::binding_context::BindingContext::is_empty",
    "body:crate::binding_context::BindingContext::len",
    "body:crate::binding_context::BindingContext::contains",
    "body:crate::binding_context::BindingContext::lookup",
    "body:crate::binding_context::BindingContext::insert",
    "body:crate::binding_context::BindingContext::clear_for_function_entry",
}
assert required.issubset(bodies)

for body in bodies.values():
    assert body["module_id"] == "crate::binding_context"
    assert body["root_expr_kind"] == "Scope"
    assert body["expr_count"] > 0
    assert body["block_count"] > 0
    assert not body["source"]["path"].startswith("/")
    assert "Scope" in body["expr_kind_counts"]

assert bodies["body:crate::binding_context::BindingContext::insert"]["stmt_count"] > 0
assert "Call" in bodies["body:crate::binding_context::BindingContext::lookup"]["expr_kind_counts"]

coverage = report["coverage"]
assert coverage["binding_context_family_selected"] == 1
assert coverage["hir_owner_reference_used"] == 1
assert coverage["selected_definition_count"] >= len(required)

claims = report["claims"]
assert claims["MIR_or_borrowck_extracted"] == 0
assert claims["drop_elaboration_extracted"] == 0
assert claims["RustLifecycleAdapterFacts_generated"] == 0
assert claims["hako_plan_emitted"] == 0
assert claims["hako_source_emitted"] == 0
assert claims["backend_behavior_changed"] == 0
PY

cat <<'REPORT'
output_contract=rustc-semir-adapter-binding-context-thir-body-inventory-guard-v0
binding_context_family_selected=1
thir_body_inventory_green=1
hir_owner_reference_used=1
selected_definition_count_positive=1
MIR_or_borrowck_extracted=0
drop_elaboration_extracted=0
RustLifecycleAdapterFacts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
summary=ok
REPORT
