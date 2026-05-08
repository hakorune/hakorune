#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-inline-plan-preserve"
cd "$ROOT_DIR"

echo "[$TAG] running M11c-preserve guard"

cargo test -q mir_preserves_rune_hint
cargo test -q build_mir_json_root_emits_inline_plans_from_hint_runes

rg -F -q 'pub struct InlinePlan' src/mir/inline_plan.rs
rg -F -q 'inline_plans_from_runes' src/mir/inline_plan.rs
rg -F -q 'inline_plans' src/mir/function/types.rs
rg -F -q '"inline_plans"' src/runner/mir_json_emit/root.rs
rg -F -q 'refresh_function_inline_plans' src/mir/semantic_refresh.rs
rg -F -q 'metadata.inline_plans' docs/reference/mir/hints.md
rg -F -q 'inline_plans' docs/reference/mir/metadata-facts-ssot.md
rg -F -q 'M11c-preserve' docs/development/current/main/design/inline-plan-ssot.md
rg -F -q '293x-047-M11C-INLINE-PLAN-PRESERVE' docs/development/current/main/CURRENT_STATE.toml

if rg -F -q 'inline_plans' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume inline_plans in M11c-preserve" >&2
  exit 1
fi

echo "[$TAG] ok"
