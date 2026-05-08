#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-inline-required-vocab"
cd "$ROOT_DIR"

echo "[$TAG] running M11c-required-vocab guard"

cargo test -q parser_accepts_canonical_rune_control_plane_surface_and_roundtrips_ast_json
cargo test -q parser_rejects_invalid_lowering_rune_value
cargo test -q mir_preserves_rune_lowering_inline_required_as_inline_plan_metadata

rg -F -q 'Lowering(inline_required)' src/ast/attrs.rs
rg -F -q 'Lowering(inline_required)' lang/src/compiler/parser/rune/rune_contract_box.hako
rg -F -q 'InlineRequest::Required' src/mir/inline_plan.rs
rg -F -q 'source: "rune_lowering"' src/mir/inline_plan.rs
rg -F -q 'InlineRequest::Required => {}' src/mir/passes/inline_soft_leaf.rs
rg -F -q 'M11c-required-vocab live schema' docs/development/current/main/design/inline-plan-ssot.md
rg -F -q 'M11c-required-vocab is live as a vocabulary/preservation row.' \
  docs/development/current/main/phases/phase-293x/293x-056-M11C-REQUIRED-INLINE-VOCAB.md

if rg -F -q 'inline_required' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume inline_required in M11c-required-vocab" >&2
  exit 1
fi

if rg -F -q 'inline_plans' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume inline_plans in M11c-required-vocab" >&2
  exit 1
fi

echo "[$TAG] ok"
