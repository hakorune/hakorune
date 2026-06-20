#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from pathlib import Path

root = Path(".")
doc = (root / "docs/development/current/main/design/condition-binding-resolution-rewrite-design.md").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1446-CONDITION-BINDING-RESOLUTION-REWRITE-DESIGN-001.md").read_text()
proof = (root / "docs/development/current/main/design/condition-binding-promoted-identity-proof-probe.md").read_text()
carrier_impl = (root / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs").read_text()
condition_env = (root / "src/mir/join_ir/lowering/condition_env.rs").read_text()
boundary_types = (root / "src/mir/join_ir/lowering/inline_boundary/types.rs").read_text()

assert "rewrite_design_documented=1" in doc
assert "rewrite_shape=additive_adapter" in doc
assert "new_adapter=resolve_promoted_condition_binding_identity" in doc
assert "legacy_resolve_promoted_join_id_kept=1" in doc
assert "implementation_started=0" in doc
assert "do not change scope_manager lookup in this design row" in doc
assert "rewrite_shape=additive_adapter" in card
assert "legacy_resolve_promoted_join_id_kept=1" in card
assert "do_not_modify_resolution_code=1" in card
assert "AllowIdentityCandidate(ConditionBinding.join_value)" in doc
assert "allow_identity_candidate=1" in proof

assert "pub fn resolve_promoted_join_id" in carrier_impl
assert "pub struct ConditionBinding" in condition_env
assert "pub join_value: ValueId" in condition_env
assert "pub condition_bindings: Vec<ConditionBinding>" in boundary_types

# This is a design row; the future adapter must not already exist in code.
assert "resolve_promoted_condition_binding_identity" not in carrier_impl
PY

cat <<'REPORT'
output_contract=rust-lifecycle-condition-binding-resolution-design-v0
rewrite_design_documented=1
rewrite_shape=additive_adapter
legacy_resolve_promoted_join_id_kept=1
condition_binding_identity_input_named=1
implementation_started=0
backend_behavior_changed=0
summary=ok
REPORT
