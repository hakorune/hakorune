#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-effect-capability-plan"
cd "$ROOT_DIR"
source tools/checks/lib/cargo_test_filter_group.sh
source tools/checks/lib/guard_common.sh

echo "[$TAG] running M11d EffectPlan/CapabilityPlan guard"

run_cargo_test_filter_group "$TAG" "effect/capability plan acceptance" \
  effect_capability_plan \
  mir_preserves_rune_contracts_as_effect_plan_metadata \
  build_mir_json_root_emits_effect_and_capability_plans \
  rune_contract_verifier_consumes_effect_plan_metadata

guard_require_files "$TAG" \
  src/mir/mod.rs \
  src/mir/rune_plan_refresh.rs \
  src/mir/function/metadata.rs \
  src/mir/semantic_refresh.rs \
  src/runner/mir_json_emit/plan_metadata.rs \
  src/runner/mir_json_emit/root.rs \
  src/runner/mir_json_emit/metadata.rs \
  crates/hakorune_frontend_ast/src/attrs.rs \
  lang/src/compiler/parser/rune/rune_contract_box.hako \
  docs/reference/mir/metadata-facts-ssot.md \
  docs/development/current/main/phases/phase-293x/293x-060-M11D-EFFECT-CAPABILITY-PLAN.md \
  docs/development/current/main/phases/phase-293x/293x-061-M11D-RUNE-PLAN-REFRESH-SSOT.md

guard_expect_fixed_in_file "$TAG" \
  "pub mod effect_capability_plan" \
  "src/mir/mod.rs" \
  "MIR root must expose the effect/capability plan module"
guard_expect_fixed_in_file "$TAG" \
  "pub mod rune_plan_refresh" \
  "src/mir/mod.rs" \
  "MIR root must expose the rune plan refresh module"
guard_expect_fixed_in_file "$TAG" \
  "pub fn refresh_function_rune_plans" \
  "src/mir/rune_plan_refresh.rs" \
  "rune plan refresh must keep a function-level SSOT entry"
guard_expect_fixed_in_file "$TAG" \
  "pub effect_plans: Vec<EffectPlan>" \
  "src/mir/function/metadata.rs" \
  "function metadata must carry effect plans"
guard_expect_fixed_in_file "$TAG" \
  "pub capability_plans: Vec<CapabilityPlan>" \
  "src/mir/function/metadata.rs" \
  "function metadata must carry capability plans"
guard_expect_fixed_in_file "$TAG" \
  "refresh_function_rune_plans(function)" \
  "src/mir/semantic_refresh.rs" \
  "semantic refresh must call the rune plan SSOT"
guard_expect_fixed_in_file "$TAG" \
  "insert_plan_metadata_json" \
  "src/runner/mir_json_emit/plan_metadata.rs" \
  "MIR JSON emitter must keep plan metadata insertion helper"
guard_expect_fixed_in_file "$TAG" \
  "build_function_metadata_json(f)" \
  "src/runner/mir_json_emit/root.rs" \
  "MIR JSON root must build function metadata JSON"
guard_expect_fixed_in_file "$TAG" \
  "insert_plan_metadata_json(obj, metadata)" \
  "src/runner/mir_json_emit/metadata.rs" \
  "MIR JSON metadata must insert plan metadata"
guard_expect_fixed_in_file "$TAG" \
  "metadata.effect_plans" \
  "docs/reference/mir/metadata-facts-ssot.md" \
  "MIR metadata facts SSOT must document effect plans"
guard_expect_fixed_in_file "$TAG" \
  "metadata.capability_plans" \
  "docs/reference/mir/metadata-facts-ssot.md" \
  "MIR metadata facts SSOT must document capability plans"
guard_expect_fixed_in_file "$TAG" \
  "M11d is live as a MIR-owned metadata boundary." \
  "docs/development/current/main/phases/phase-293x/293x-060-M11D-EFFECT-CAPABILITY-PLAN.md" \
  "M11d effect/capability phase card must record the live boundary"
guard_expect_fixed_in_file "$TAG" \
  "Rune-derived plan refresh has one SSOT entry." \
  "docs/development/current/main/phases/phase-293x/293x-061-M11D-RUNE-PLAN-REFRESH-SSOT.md" \
  "M11d rune refresh phase card must record the SSOT entry"

if rg -F -q 'refresh_function_effect_capability_plans(' src/mir/builder src/runner/json_v0_bridge -g '*.rs'; then
  echo "[$TAG] ERROR: bridge/builder must use refresh_function_rune_plans" >&2
  exit 1
fi

if rg -F -q 'refresh_function_inline_plans(' src/mir/builder src/runner/json_v0_bridge -g '*.rs'; then
  echo "[$TAG] ERROR: bridge/builder must use refresh_function_rune_plans" >&2
  exit 1
fi

if rg -F -q '"Capability"' crates/hakorune_frontend_ast/src/attrs.rs lang/src/compiler/parser/rune/rune_contract_box.hako; then
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
