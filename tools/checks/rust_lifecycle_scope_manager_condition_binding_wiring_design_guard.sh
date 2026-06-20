#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from pathlib import Path

root = Path(".")
doc = (root / "docs/development/current/main/design/scope-manager-condition-binding-adapter-wiring-design.md").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1450-SCOPE-MANAGER-CONDITION-BINDING-ADAPTER-WIRING-DESIGN-001.md").read_text()
scope = (root / "src/mir/join_ir/lowering/scope_manager.rs").read_text()

assert "wiring_design_documented=1" in doc
assert "wiring_shape=explicit_scope_manager_condition_bindings_input" in doc
assert "condition_bindings: &'a [ConditionBinding]" in doc
assert "condition-binding adapter" in doc
assert "legacy resolve_promoted_join_id" in doc
assert "legacy_resolve_promoted_join_id_kept=1" in doc
assert "implementation_started=0" in doc

assert "wiring_shape=explicit_scope_manager_condition_bindings_input" in card
assert "do_not_change_scope_manager_code=1" in card
assert "do_not_emit_trim_route_lowering=1" in card

# The design guarantees the legacy lookup remains available even after later
# implementation rows add the condition-binding adapter path.
assert "self.carrier_info.resolve_promoted_join_id(name)" in scope
PY

cat <<'REPORT'
output_contract=rust-lifecycle-scope-manager-condition-binding-wiring-design-v0
wiring_design_documented=1
condition_bindings_input_named=1
lookup_order_documented=1
legacy_path_preserved=1
implementation_started=0
backend_behavior_changed=0
summary=ok
REPORT
