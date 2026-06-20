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
cat >"$TMP_DIR/sample.rs" <<'RS'
pub mod model {
    pub struct Point {
        pub x: i64,
        pub y: i64,
    }
}

pub fn add(a: i64, b: i64) -> i64 {
    a + b
}
RS

OUTPUT="$(
  cd "$TOOL_DIR"
  SYSROOT="$(rustc --print sysroot)"
  HOST="$(rustc -Vv | awk '/^host:/ { print $2 }')"
  LIB_DIR="$SYSROOT/lib/rustlib/$HOST/lib"
  export LD_LIBRARY_PATH="$LIB_DIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  cargo run --quiet --features rustc-private -- --hir-item-provenance-inventory "$TMP_DIR/sample.rs"
)"

grep -q '^output_contract=rustc-semir-adapter-hir-item-provenance-inventory-v0$' <<<"$OUTPUT"
grep -q '^hir_item_provenance_inventory_green=1$' <<<"$OUTPUT"
grep -q '^crate_name=sample$' <<<"$OUTPUT"
grep -q '^crate_identity_reported=1$' <<<"$OUTPUT"
grep -q '^module_identity_reported=1$' <<<"$OUTPUT"
grep -q '^module_0_path=crate$' <<<"$OUTPUT"
grep -q '_path=model::Point$' <<<"$OUTPUT"
grep -q '_kind=Struct$' <<<"$OUTPUT"
grep -q '_path=add$' <<<"$OUTPUT"
grep -q '_kind=Fn$' <<<"$OUTPUT"
grep -q '_source=.*/sample.rs:' <<<"$OUTPUT"
grep -q '^item_identity_reported=1$' <<<"$OUTPUT"
grep -q '^source_provenance_reported=1$' <<<"$OUTPUT"
grep -q '^RustLifecycleAdapterFacts_generated=0$' <<<"$OUTPUT"
grep -q '^hako_plan_emitted=0$' <<<"$OUTPUT"
grep -q '^hako_source_emitted=0$' <<<"$OUTPUT"
grep -q '^backend_behavior_changed=0$' <<<"$OUTPUT"
grep -q '^summary=ok$' <<<"$OUTPUT"

cat <<'REPORT'
output_contract=rustc-semir-adapter-hir-item-provenance-inventory-guard-v0
pinned_nightly_preflight_guard_green=1
hir_item_provenance_inventory_green=1
crate_identity_reported=1
module_identity_reported=1
item_identity_reported=1
source_provenance_reported=1
RustLifecycleAdapterFacts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
summary=ok
REPORT
