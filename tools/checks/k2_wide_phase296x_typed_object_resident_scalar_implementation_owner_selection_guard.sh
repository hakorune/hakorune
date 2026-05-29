#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-303-TYPED-OBJECT-RESIDENT-SCALAR-IMPLEMENTATION-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-302-TYPED-OBJECT-RESIDENT-SCALAR-SELECTED-METHOD-PLAN.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row303-typed-object-resident-owner-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-resident-scalar-implementation-owner-selection-v0"
require_line "$DOC" "input_contract=typed-object-resident-scalar-selected-method-plan-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_plan_helper_ops=21"
require_line "$DOC" "selected_plan_net_helper_delta=21"
require_line "$DOC" "selected_owner=llvm_py_typed_object_resident_scalar_lowering"
require_line "$DOC" "selected_owner_file=src/llvm_py/instructions/typed_object_resident_scalar.py"
require_line "$DOC" "thin_hook_file=src/llvm_py/instructions/field_access.py"
require_line "$DOC" "new_env_var_required=0"
require_line "$DOC" "activation_gate=HAKO_TYPED_OBJECT_STORE=single_thread_exact,HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1,selected_method_plan"
require_line "$DOC" "default_emission_unchanged=1"
require_line "$DOC" "generic_residence_rewrite=0"
require_line "$DOC" "runtime_helper_abi_unchanged=1"
require_line "$DOC" "mirbuilder_change_required=0"
require_line "$DOC" "hako_source_change_required=0"
require_line "$DOC" "selected_next=typed_object_resident_scalar_lowering_pilot"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

echo "[row303-typed-object-resident-owner-selection] ok"
