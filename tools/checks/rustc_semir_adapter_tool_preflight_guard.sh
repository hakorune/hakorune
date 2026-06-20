#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

MANIFEST="tools/rust_lifecycle/rustc_semir_adapter/Cargo.toml"
MAIN="tools/rust_lifecycle/rustc_semir_adapter/src/main.rs"

test -f "$MANIFEST"
test -f "$MAIN"

if rg -n "rustc_(driver|interface|hir|middle|mir|span)|rustc-private|rustc_private" \
  Cargo.toml crates src >/tmp/rustc_semir_product_rustc_private_hits 2>/dev/null; then
  cat /tmp/rustc_semir_product_rustc_private_hits >&2
  exit 1
fi

OUTPUT="$(cargo run --quiet --manifest-path "$MANIFEST" -- --preflight)"

grep -q '^output_contract=rustc-semir-adapter-tool-preflight-v0$' <<<"$OUTPUT"
grep -q '^adapter_tool_preflight_green=1$' <<<"$OUTPUT"
grep -q '^standalone_tool_manifest_exists=1$' <<<"$OUTPUT"
grep -q '^rustc_private_dependency_enabled=0$' <<<"$OUTPUT"
grep -q '^facts_generated=0$' <<<"$OUTPUT"
grep -q '^hako_plan_emitted=0$' <<<"$OUTPUT"
grep -q '^hako_source_emitted=0$' <<<"$OUTPUT"
grep -q '^backend_behavior_changed=0$' <<<"$OUTPUT"
grep -q '^summary=ok$' <<<"$OUTPUT"

cat <<'REPORT'
output_contract=rustc-semir-adapter-tool-preflight-guard-v0
adapter_tool_preflight_green=1
standalone_tool_manifest_exists=1
root_Cargo_rustc_private_dependency=0
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
summary=ok
REPORT
