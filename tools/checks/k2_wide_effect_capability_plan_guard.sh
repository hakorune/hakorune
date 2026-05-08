#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-effect-capability-plan"
cd "$ROOT_DIR"

echo "[$TAG] running M11d EffectPlan/CapabilityPlan guard"

cargo test -q effect_capability_plan
cargo test -q mir_preserves_rune_contracts_as_effect_plan_metadata
cargo test -q build_mir_json_root_emits_effect_and_capability_plans
cargo test -q rune_contract_verifier_consumes_effect_plan_metadata

rg -F -q 'pub mod effect_capability_plan' src/mir/mod.rs
rg -F -q 'pub effect_plans: Vec<EffectPlan>' src/mir/function/types.rs
rg -F -q 'pub capability_plans: Vec<CapabilityPlan>' src/mir/function/types.rs
rg -F -q 'refresh_function_effect_capability_plans' src/mir/semantic_refresh.rs
rg -F -q 'metadata.effect_plans' docs/reference/mir/metadata-facts-ssot.md
rg -F -q 'metadata.capability_plans' docs/reference/mir/metadata-facts-ssot.md
rg -F -q 'M11d is live as a MIR-owned metadata boundary.' \
  docs/development/current/main/phases/phase-293x/293x-060-M11D-EFFECT-CAPABILITY-PLAN.md

if rg -F -q '"Profile"' src/ast/attrs.rs lang/src/compiler/parser/rune/rune_contract_box.hako; then
  echo "[$TAG] ERROR: Profile parser surface must stay disabled in M11d" >&2
  exit 1
fi

if rg -F -q '"Capability"' src/ast/attrs.rs lang/src/compiler/parser/rune/rune_contract_box.hako; then
  echo "[$TAG] ERROR: Capability parser surface must stay disabled in M11d" >&2
  exit 1
fi

if rg -F -q 'effect_plans' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume effect_plans in M11d" >&2
  exit 1
fi

if rg -F -q 'capability_plans' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume capability_plans in M11d" >&2
  exit 1
fi

echo "[$TAG] ok"
