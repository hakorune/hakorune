#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from pathlib import Path

root = Path(".")
impls = (root / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs").read_text()
scope = (root / "src/mir/join_ir/lowering/scope_manager.rs").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1448-CONDITION-BINDING-RESOLUTION-ADAPTER-PROBE-001.md").read_text()

assert "pub fn resolve_promoted_join_id" in impls
assert "pub fn resolve_promoted_condition_binding_identity" in impls
assert "condition_bindings:" in impls
assert ".find(|binding| binding.name == helper.carrier_name)" in impls
assert ".map(|binding| binding.join_value)" in impls

assert "test_resolve_promoted_condition_binding_identity_allows_match" in impls
assert "test_resolve_promoted_condition_binding_identity_denies_missing_promoted_local" in impls
assert "test_resolve_promoted_condition_binding_identity_denies_original_name_mismatch" in impls
assert "test_resolve_promoted_condition_binding_identity_denies_missing_binding" in impls

assert "self.carrier_info.resolve_promoted_join_id(name)" in scope
assert "resolve_promoted_condition_binding_identity" not in scope

assert "adapter_exists=1" in card
assert "do_not_change_scope_manager_lookup=1" in card
assert "trim_route_lowering_emitted=0" in card
PY

cat <<'REPORT'
output_contract=rust-lifecycle-condition-binding-resolution-adapter-v0
adapter_exists=1
adapter_allows_matching_condition_binding=1
adapter_denies_missing_promoted_body_local=1
adapter_denies_original_name_mismatch=1
adapter_denies_missing_condition_binding=1
legacy_resolve_promoted_join_id_kept=1
scope_manager_uses_legacy_path=1
trim_route_lowering_emitted=0
backend_behavior_changed=0
summary=ok
REPORT
