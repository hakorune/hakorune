#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TOOL_DIR="$ROOT_DIR/tools/rust_lifecycle/rustc_semir_adapter"

test -f "$TOOL_DIR/Cargo.toml"
test -f "$TOOL_DIR/rust-toolchain.toml"
grep -q 'channel = "nightly-[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}"' "$TOOL_DIR/rust-toolchain.toml"
grep -q 'rustc-dev' "$TOOL_DIR/rust-toolchain.toml"
grep -q 'llvm-tools-preview' "$TOOL_DIR/rust-toolchain.toml"

if rg -n "rustc_(driver|interface|hir|middle|mir|span)|rustc-private|rustc_private" \
  "$ROOT_DIR/Cargo.toml" "$ROOT_DIR/crates" "$ROOT_DIR/src" >/tmp/rustc_semir_product_rustc_private_hits 2>/dev/null; then
  cat /tmp/rustc_semir_product_rustc_private_hits >&2
  exit 1
fi

if [[ -n "${RUSTC_BOOTSTRAP:-}" ]]; then
  echo "RUSTC_BOOTSTRAP must be unset for the formal pinned-nightly guard" >&2
  exit 1
fi

OUTPUT="$(
  cd "$TOOL_DIR"
  SYSROOT="$(rustc --print sysroot)"
  HOST="$(rustc -Vv | awk '/^host:/ { print $2 }')"
  LIB_DIR="$SYSROOT/lib/rustlib/$HOST/lib"
  export LD_LIBRARY_PATH="$LIB_DIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  cargo run --quiet --features rustc-private -- --rustc-private-probe
)"

grep -q '^output_contract=rustc-semir-adapter-rustc-private-probe-v0$' <<<"$OUTPUT"
grep -q '^pinned_toolchain_active=1$' <<<"$OUTPUT"
grep -q '^rustc_release_reported=1$' <<<"$OUTPUT"
grep -q '^rustc_release=.*nightly' <<<"$OUTPUT"
grep -q '^rustc_commit_hash_reported=1$' <<<"$OUTPUT"
grep -q '^rustc_commit_hash=[0-9a-f]\{40\}$' <<<"$OUTPUT"
grep -q '^rustc_sysroot_reported=1$' <<<"$OUTPUT"
grep -q '^rustc_dev_component_installed=1$' <<<"$OUTPUT"
grep -q '^llvm_tools_component_installed=1$' <<<"$OUTPUT"
grep -q '^rustc_private_probe_compiled=1$' <<<"$OUTPUT"
grep -q '^rustc_private_probe_linked=1$' <<<"$OUTPUT"
grep -q '^rustc_private_probe_executed=1$' <<<"$OUTPUT"
grep -q '^rustc_private_readiness=verified$' <<<"$OUTPUT"
grep -q '^canonical_bootstrap_override=0$' <<<"$OUTPUT"
grep -q '^bootstrap_facts_accepted=0$' <<<"$OUTPUT"
grep -q '^facts_generated=0$' <<<"$OUTPUT"
grep -q '^hako_plan_emitted=0$' <<<"$OUTPUT"
grep -q '^hako_source_emitted=0$' <<<"$OUTPUT"
grep -q '^backend_behavior_changed=0$' <<<"$OUTPUT"
grep -q '^summary=ok$' <<<"$OUTPUT"

cat <<'REPORT'
output_contract=rustc-semir-adapter-pinned-nightly-preflight-guard-v0
pinned_nightly_route_documented=1
adapter_local_toolchain_file=1
moving_nightly_alias_used=0
pinned_toolchain_active=1
rustc_release_reported=1
rustc_commit_hash_reported=1
rustc_sysroot_reported=1
rustc_dev_component_installed=1
llvm_tools_component_installed=1
rustc_private_probe_compiled=1
rustc_private_probe_linked=1
rustc_private_probe_executed=1
rustc_private_readiness=verified
canonical_bootstrap_override=0
bootstrap_facts_accepted=0
product_crates_rustc_private_dependency=0
root_product_toolchain_changed=0
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
summary=ok
REPORT
