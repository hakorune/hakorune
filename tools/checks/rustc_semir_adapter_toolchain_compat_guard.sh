#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

MANIFEST="tools/rust_lifecycle/rustc_semir_adapter/Cargo.toml"
test -f "$MANIFEST"

if rg -n "rustc_(driver|interface|hir|middle|mir|span)|rustc-private|rustc_private" \
  Cargo.toml crates src >/tmp/rustc_semir_product_rustc_private_hits 2>/dev/null; then
  cat /tmp/rustc_semir_product_rustc_private_hits >&2
  exit 1
fi

OUTPUT="$(cargo run --quiet --manifest-path "$MANIFEST" -- --toolchain-preflight)"

grep -q '^output_contract=rustc-semir-adapter-toolchain-compat-v0$' <<<"$OUTPUT"
grep -q '^toolchain_compat_preflight_green=1$' <<<"$OUTPUT"
grep -q '^rustc_version_reported=1$' <<<"$OUTPUT"
grep -q '^rustc_channel=' <<<"$OUTPUT"
grep -q '^rustc_sysroot=' <<<"$OUTPUT"
grep -q '^rustc_private_readiness=' <<<"$OUTPUT"
grep -q '^rustc_private_readiness_reported=1$' <<<"$OUTPUT"
grep -q '^facts_generated=0$' <<<"$OUTPUT"
grep -q '^hako_plan_emitted=0$' <<<"$OUTPUT"
grep -q '^hako_source_emitted=0$' <<<"$OUTPUT"
grep -q '^source_shape_fallback=0$' <<<"$OUTPUT"
grep -q '^backend_behavior_changed=0$' <<<"$OUTPUT"
grep -q '^summary=ok$' <<<"$OUTPUT"

cat <<'REPORT'
output_contract=rustc-semir-adapter-toolchain-compat-guard-v0
toolchain_compat_preflight_green=1
rustc_version_reported=1
rustc_channel_classified=1
rustc_private_readiness_reported=1
root_Cargo_rustc_private_dependency=0
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
source_shape_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
