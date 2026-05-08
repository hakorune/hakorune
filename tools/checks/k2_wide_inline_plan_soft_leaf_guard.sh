#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-inline-plan-soft-leaf"
cd "$ROOT_DIR"

echo "[$TAG] running M11c-soft-leaf guard"

cargo test -q inline_soft_leaf

rg -F -q 'pub mod inline_soft_leaf' src/mir/passes/mod.rs
rg -F -q 'pub fn apply(module: &mut MirModule) -> usize' src/mir/passes/inline_soft_leaf.rs
rg -F -q 'InlineRequest::Prefer' src/mir/passes/inline_soft_leaf.rs
rg -F -q 'SOFT_LEAF_MAX_INSTRUCTIONS' src/mir/passes/inline_soft_leaf.rs
rg -F -q 'passes::inline_soft_leaf::apply' src/mir/optimizer/core.rs
rg -F -q 'M11c-soft-leaf' docs/development/current/main/design/inline-plan-ssot.md
rg -F -q '293x-048 M11c soft leaf inline landed' docs/development/current/main/CURRENT_STATE.toml

if rg -F -q 'inline_soft_leaf' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not own M11c-soft-leaf inline decisions" >&2
  exit 1
fi

echo "[$TAG] ok"
