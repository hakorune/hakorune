#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-inline-required-verify"
cd "$ROOT_DIR"

echo "[$TAG] running M11c-required-verify guard"

cargo test -q required_inline_verifies_leaf_with_required_contracts
cargo test -q required_inline_rejects_missing_contracts
cargo test -q mir_verifier_runs_required_inline_check
cargo test -q required_inline_rejects_nested_call
cargo test -q mir_preserves_rune_lowering_inline_required_as_inline_plan_metadata

rg -F -q 'pub(crate) mod inline_leaf' src/mir/mod.rs
rg -F -q 'check_leaf_inline_shape' src/mir/inline_leaf.rs
rg -F -q 'check_required_inline_plans' src/mir/verification/inline_required.rs
rg -F -q 'verify_required_inline_plans' src/mir/verification.rs
rg -F -q 'VerificationError::InlinePlanViolation' src/mir/verification_types.rs
rg -F -q 'plan.verified = required_inline_plan_verified' src/mir/inline_plan.rs
rg -F -q 'M11c-required-verify live surface' docs/development/current/main/design/inline-plan-ssot.md
rg -F -q 'M11c-required-verify is live as a MIR verifier row.' \
  docs/development/current/main/phases/phase-293x/293x-059-M11C-REQUIRED-INLINE-VERIFY.md

if rg -F -q 'inline_required' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume inline_required in M11c-required-verify" >&2
  exit 1
fi

if rg -F -q 'inline_plans' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume inline_plans in M11c-required-verify" >&2
  exit 1
fi

echo "[$TAG] ok"
