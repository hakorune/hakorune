#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-allocation-policy-native-source-owner-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-1761-MIRBUILDER-ALLOCATION-POLICY-HAKO-NATIVE-SOURCE-OWNER-001.md"
NATIVE_OWNER="$ROOT_DIR/lang/src/compiler/lib/next_value_id_prepared_state_kernel.hako"
GENERATED_OWNER="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.hako"
TOML="$ROOT_DIR/lang/src/compiler/hako_module.toml"
README="$ROOT_DIR/lang/src/compiler/lib/README.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-native-source-owner-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$STATE" "$CARD" "$NATIVE_OWNER" "$GENERATED_OWNER" "$TOML" "$README" "$FIXTURE"

python3 - <<'PY'
import json
from pathlib import Path

native_owner = Path("lang/src/compiler/lib/next_value_id_prepared_state_kernel.hako").read_text()
generated_owner = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.hako").read_text()
assert "manual-edit: forbidden" not in native_owner
assert "manual-edit: forbidden" in generated_owner
assert "static box MirBuilderAllocationPolicyApi" in native_owner
assert "next_value_id(current_function_present, function_state, core_context, reserved_membership)" in native_owner
assert "lib.next_value_id_prepared_state_kernel = \"lib/next_value_id_prepared_state_kernel.hako\"" in Path("lang/src/compiler/hako_module.toml").read_text()
assert "first native source owner candidate" in Path("lang/src/compiler/lib/README.md").read_text()

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-native-source-owner-v0.json").read_text())
assert fixture["decision"] == "Adopt"
assert fixture["input_evidence"]["native_hako_source_owner_present"] == 1
assert fixture["input_evidence"]["generator_overwrite_guard"] == 1
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-allocation-policy-native-source-owner-v0
family_id=hakorune_mir_builder::next_value_id_prepared_state_kernel
native_hako_source_owner_present=1
generator_overwrite_guard=1
decision=Adopt
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
manual_next_owner_selection=0
summary=ok
REPORT
