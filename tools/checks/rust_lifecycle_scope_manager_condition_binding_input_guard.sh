#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from pathlib import Path

root = Path(".")
scope = (root / "src/mir/join_ir/lowering/scope_manager.rs").read_text()
adapter_card = (root / "docs/development/current/main/phases/phase-296x/296x-1452-SCOPE-MANAGER-CONDITION-BINDING-INPUT-PROBE-001.md").read_text()

assert "use super::condition_env::{ConditionBinding, ConditionEnv};" in scope
assert "pub condition_bindings: &'a [ConditionBinding]" in scope
assert ".resolve_promoted_condition_binding_identity(name, self.condition_bindings)" in scope
assert "self.carrier_info.resolve_promoted_join_id(name)" in scope

assert "test_loop_break_scope_manager_condition_binding_adapter" in scope
assert "test_loop_break_scope_manager_condition_env_wins_over_adapter" in scope
assert 'condition_env.insert("ch".to_string(), ValueId(111));' in scope
assert "assert_eq!(scope.lookup(\"ch\"), Some(ValueId(111)))" in scope

assert "condition_bindings_input_added=1" in adapter_card
assert "lookup_uses_condition_binding_adapter=1" in adapter_card
assert "trim_route_lowering_emitted=0" in adapter_card
assert "do_not_emit_trim_route_lowering=1" in adapter_card
PY

cat <<'REPORT'
output_contract=rust-lifecycle-scope-manager-condition-binding-input-v0
condition_bindings_input_added=1
lookup_uses_condition_binding_adapter=1
lookup_order_preserves_condition_env_priority=1
legacy_resolve_promoted_join_id_kept=1
trim_route_lowering_emitted=0
backend_behavior_changed=0
summary=ok
REPORT
