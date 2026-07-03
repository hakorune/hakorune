#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-inline-required-vocab"
cd "$ROOT_DIR"
source tools/checks/lib/cargo_test_filter_group.sh
source tools/checks/lib/guard_common.sh

echo "[$TAG] running M11c-required-vocab guard"

run_cargo_test_filter_group "$TAG" "required inline vocabulary acceptance" \
  parser_accepts_canonical_rune_control_plane_surface_and_roundtrips_ast_json \
  parser_rejects_invalid_lowering_rune_value \
  mir_preserves_rune_lowering_inline_required_as_inline_plan_metadata

guard_require_files "$TAG" \
  crates/hakorune_frontend_ast/src/attrs.rs \
  lang/src/compiler/parser/rune/rune_contract_box.hako \
  src/mir/inline_plan.rs \
  src/mir/passes/inline_soft_leaf.rs \
  docs/development/current/main/design/inline-plan-ssot.md \
  docs/development/current/main/phases/phase-293x/293x-056-M11C-REQUIRED-INLINE-VOCAB.md

guard_expect_fixed_in_file "$TAG" \
  "Lowering(inline_required)" \
  "crates/hakorune_frontend_ast/src/attrs.rs" \
  "frontend AST rune attrs must accept only the inline_required lowering token"
guard_expect_fixed_in_file "$TAG" \
  "Lowering(inline_required)" \
  "lang/src/compiler/parser/rune/rune_contract_box.hako" \
  "Stage1 rune parser must mirror the inline_required token"
guard_expect_fixed_in_file "$TAG" \
  "InlineRequest::Required" \
  "src/mir/inline_plan.rs" \
  "MIR InlinePlan must carry the required inline request"
guard_expect_fixed_in_file "$TAG" \
  "source: \"rune_lowering\"" \
  "src/mir/inline_plan.rs" \
  "MIR InlinePlan must record rune_lowering as the source"
guard_expect_fixed_in_file "$TAG" \
  "InlineRequest::Required => {}" \
  "src/mir/passes/inline_soft_leaf.rs" \
  "soft leaf inliner must preserve required inline requests"
guard_expect_fixed_in_file "$TAG" \
  "M11c-required-vocab live schema" \
  "docs/development/current/main/design/inline-plan-ssot.md" \
  "InlinePlan SSOT must document the live required-vocab schema"
guard_expect_fixed_in_file "$TAG" \
  "M11c-required-vocab is live as a vocabulary/preservation row." \
  "docs/development/current/main/phases/phase-293x/293x-056-M11C-REQUIRED-INLINE-VOCAB.md" \
  "phase card must record the required-vocab row"

if rg -F -q 'inline_required' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume inline_required in M11c-required-vocab" >&2
  exit 1
fi

if rg -F -q 'inline_plans' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume inline_plans in M11c-required-vocab" >&2
  exit 1
fi

echo "[$TAG] ok"
