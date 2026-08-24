#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-canonical-corridor-guard"
LLVM="$ROOT_DIR/src/runner/product/llvm/mod.rs"
OPTIMIZER="$ROOT_DIR/src/mir/optimizer/core.rs"
SCHEDULE="$ROOT_DIR/src/mir/passes/callsite_canonicalize/schedule.rs"
CSE="$ROOT_DIR/src/mir/passes/cse.rs"
DIAGNOSTICS="$ROOT_DIR/src/mir/optimizer_passes/diagnostics.rs"
INTERPRETER_CALLS="$ROOT_DIR/src/backend/mir_interpreter/handlers/calls/mod.rs"
REJECT="$ROOT_DIR/src/mir/contracts/backend_core_ops/allowlists.rs"
JSON="$ROOT_DIR/src/runner/json_v0_bridge/core.rs"
PROGRAM_LOWERING="$ROOT_DIR/src/runner/json_v0_bridge/lowering/program.rs"
EXEC="$ROOT_DIR/src/runner/modes/common_util/exec.rs"
CALL_OPS="$ROOT_DIR/src/runner/json_v0_bridge/lowering/expr/call_ops.rs"
PROGRAM_CALL_TARGETS="$ROOT_DIR/src/runner/json_v0_bridge/lowering/program_call_targets.rs"
METHODS="$ROOT_DIR/src/mir/instruction/methods.rs"
MIR_V0_CALL="$ROOT_DIR/src/runner/mir_json_v0/call.rs"
MIR_V0_CATALOG="$ROOT_DIR/src/runner/mir_json_v0/catalog.rs"
MIR_V0_MODULE="$ROOT_DIR/src/runner/mir_json_v0/module.rs"
CALLEE_DEFS="$ROOT_DIR/crates/hakorune_mir_defs/src/call_unified.rs"
SIMPLIFY_FLOW="$ROOT_DIR/src/mir/passes/simplify_cfg/flow.rs"
VALUE_CONSUMER="$ROOT_DIR/src/mir/value_consumer.rs"
ESCAPE_BARRIER="$ROOT_DIR/src/mir/escape_barrier.rs"
OWNERSHIP_VERIFY="$ROOT_DIR/src/mir/ownership_ssa/verify.rs"
OWNERSHIP_TESTS="$ROOT_DIR/src/mir/ownership_ssa/tests.rs"
QUERY="$ROOT_DIR/src/mir/query.rs"
PRINTER_HELPERS="$ROOT_DIR/src/mir/printer_helpers.rs"
PRINTER_DISPLAY="$ROOT_DIR/src/mir/instruction/display.rs"
PRINTER_TESTS="$ROOT_DIR/src/mir/printer/tests.rs"
JSON_CALLS="$ROOT_DIR/src/runner/mir_json_emit/emitters/calls.rs"
JSON_ROOT="$ROOT_DIR/src/runner/mir_json_emit/root.rs"
JSON_EMITTERS="$ROOT_DIR/src/runner/mir_json_emit/emitters/mod.rs"
JSON_HELPERS="$ROOT_DIR/src/runner/mir_json_emit/helpers.rs"
BACKEND_SHAPE="$ROOT_DIR/src/host_providers/mir_builder/backend_shape.rs"
MIR_BUILDER="$ROOT_DIR/src/host_providers/mir_builder.rs"
HANDOFF="$ROOT_DIR/src/host_providers/mir_builder/handoff.rs"
LLVM_GENERIC_CALLS="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch_calls.inc"
LLVM_MIR_CALL_DISPATCH="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_mir_call_dispatch.inc"
LLVM_MIR_CALL_SURFACE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_mir_call_surface_policy.inc"
LLVM_MIR_CALL_EXTERN="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell_extern_emit.inc"
LLVM_MIR_CALL_EXTERN_RULES="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell_extern_rules.inc"
LLVM_MIR_CALL_EXTERN_BODY="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell_extern_emit_body.inc"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

require() {
  local file="$1"
  local token="$2"
  rg -F -q -- "$token" "$file" || fail "missing '$token' in ${file#$ROOT_DIR/}"
}

for file in "$LLVM" "$OPTIMIZER" "$SCHEDULE" "$CSE" "$DIAGNOSTICS" "$INTERPRETER_CALLS" "$REJECT" "$JSON" "$PROGRAM_LOWERING" "$EXEC" "$CALL_OPS" "$PROGRAM_CALL_TARGETS" "$METHODS" "$MIR_V0_CALL" "$MIR_V0_CATALOG" "$MIR_V0_MODULE" "$CALLEE_DEFS" "$SIMPLIFY_FLOW" "$VALUE_CONSUMER" "$ESCAPE_BARRIER" "$OWNERSHIP_VERIFY" "$OWNERSHIP_TESTS" "$QUERY" "$PRINTER_HELPERS" "$PRINTER_DISPLAY" "$PRINTER_TESTS" "$JSON_CALLS" "$JSON_ROOT" "$JSON_EMITTERS" "$JSON_HELPERS" "$BACKEND_SHAPE" "$MIR_BUILDER" "$HANDOFF" "$LLVM_GENERIC_CALLS" "$LLVM_MIR_CALL_DISPATCH" "$LLVM_MIR_CALL_SURFACE" "$LLVM_MIR_CALL_EXTERN" "$LLVM_MIR_CALL_EXTERN_RULES" "$LLVM_MIR_CALL_EXTERN_BODY"; do
  [[ -f "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done

if rg -F -q "CallsiteCanonicalizeScheduleSite::MirOptimizerLateCallAndInline" "$OPTIMIZER" || rg -F -q "MirOptimizerLateCallAndInline" "$SCHEDULE"; then
  fail "optimizer retained the retired callsite-canonicalize schedule"
fi
require "$OPTIMIZER" "call_callee_{:?}_"
require "$CSE" "cse_call_key_uses_typed_callee_and_ignores_stale_func"
require "$CSE" "cse_closure_key_does_not_use_legacy_func"
require "$CSE" "cse_call_key_keeps_legacy_func_compatibility_distinct"
require "$DIAGNOSTICS" "diagnostics_observe_typed_method_without_legacy_func_const_scan"
require "$INTERPRETER_CALLS" "missing_callee_rejects_before_legacy_register_lookup"
require "$INTERPRETER_CALLS" "call-missing-callee: typed Callee required"
require "$REJECT" "Some(\"call-missing-callee\")"
require "$LLVM" "fn reject_selected_dynamic_legacy_callsites"
require "$LLVM" "legacy_callsite_reject_code"
require "$LLVM" "boundary_executor::BoundaryExecutorBox::try_execute_selected_dynamic"
require "$JSON" "CallsiteCanonicalizeScheduleSite::ProgramJsonV0Bridge"
require "$PROGRAM_LOWERING" "lower_defs_into_module"
if rg -F -q "maybe_resolve_calls" "$PROGRAM_LOWERING" || rg -F -q "func_map" "$PROGRAM_LOWERING"; then
  fail "Program lowering retained a late func_map/maybe_resolve_calls authority"
fi
require "$EXEC" "project_module_to_legacy_calls"
require "$METHODS" "pub(crate) fn call("
require "$PROGRAM_CALL_TARGETS" "ProgramCallTargetCatalog"
require "$PROGRAM_CALL_TARGETS" "ambiguous-name"
require "$SCHEDULE" "allow_legacy_target_rewrite"
require "$SCHEDULE" "ProgramJsonV0Bridge"
require "$MIR_V0_CALL" "enum JsonV0CallInput"
require "$MIR_V0_CALL" "struct JsonV0CallInputError"
require "$MIR_V0_CALL" "MirInstruction::call("
require "$MIR_V0_CATALOG" "JsonV0FunctionCatalog"
require "$MIR_V0_CATALOG" "ConstValue::String"
require "$MIR_V0_MODULE" "JsonV0FunctionCatalog::from_function"
require "$CALLEE_DEFS" "pub fn rewrite_value_operands"
require "$CALLEE_DEFS" "pub fn for_each_value_operand"
require "$SIMPLIFY_FLOW" "callee.rewrite_value_operands"
require "$SIMPLIFY_FLOW" "simplify_cfg_call_use_rewrite_preserves_typed_targets_and_args"
require "$SIMPLIFY_FLOW" "simplify_cfg_call_use_rewrite_keeps_targetless_callees_empty"
require "$SIMPLIFY_FLOW" "simplify_cfg_call_use_rewrite_preserves_legacy_func_parity"
require "$METHODS" "callee.for_each_value_operand"
require "$ROOT_DIR/src/mir/instruction/tests.rs" "typed_call_used_values_project_callee_operands_before_args"
require "$CALLEE_DEFS" "callee_for_each_value_operand_preserves_occurrence_order_and_duplicates"
require "$CALLEE_DEFS" "callee_for_each_value_operand_is_empty_for_targetless_and_missing_receiver_shapes"
require "$VALUE_CONSUMER" "MirInstruction::Call { .. } => inst.used_values()"
require "$VALUE_CONSUMER" "refresh_value_consumer_facts_counts_typed_callee_targets_as_other_uses"
require "$VALUE_CONSUMER" "refresh_value_consumer_facts_ignores_typed_func_and_dst_decoration"
require "$VALUE_CONSUMER" "refresh_value_consumer_facts_preserves_legacy_func_use"
require "$ESCAPE_BARRIER" "typed_value_target_is_a_call_barrier_and_stale_func_is_ignored"
require "$ESCAPE_BARRIER" "closure_target_marks_captures_as_capture_before_call_args"
require "$ESCAPE_BARRIER" "legacy_missing_callee_keeps_func_out_of_shared_barriers"
require "$ESCAPE_BARRIER" "targetless_typed_callees_add_no_target_barrier"
require "$ROOT_DIR/src/mir/verification/fastmem/tests.rs" "rejects_memop_value_escape_to_typed_value_target"
require "$ROOT_DIR/src/mir/verification/fastmem/tests.rs" "rejects_memop_value_escape_to_closure_capture"
require "$ROOT_DIR/src/mir/verification/fastmem/tests.rs" "keeps_legacy_func_as_fastmem_ordinary_use"
require "$OWNERSHIP_VERIFY" "fn verify_call_ownership"
require "$OWNERSHIP_VERIFY" "None => Some(func)"
require "$OWNERSHIP_VERIFY" "Callee::Method {"
require "$OWNERSHIP_VERIFY" "Callee::Value(receiver)"
require "$OWNERSHIP_VERIFY" "Callee::Closure { .. }"
require "$OWNERSHIP_VERIFY" "instruction.used_values()"
require "$OWNERSHIP_TESTS" "typed_targetless_call_ignores_legacy_func"
require "$OWNERSHIP_TESTS" "typed_method_and_value_targets_accept_known_trivial_values"
require "$OWNERSHIP_TESTS" "typed_method_and_value_targets_reject_managed_or_unknown_values"
require "$OWNERSHIP_TESTS" "typed_managed_target_fails_before_generic_liveness"
require "$OWNERSHIP_TESTS" "typed_closure_operands_use_generic_liveness_not_managed_call_policy"
require "$OWNERSHIP_TESTS" "legacy_call_still_requires_a_known_trivial_func"
require "$ROOT_DIR/src/mir/instruction/tests.rs" "call_kind_metadata_delegates_to_canonical_call_methods"
require "$QUERY" "Call { .. } => inst.used_values()"
require "$QUERY" "return inst.dst_value().into_iter().collect()"
require "$QUERY" "query_call_reads_match_canonical_used_values_for_every_shape"
require "$QUERY" "query_call_writes_match_canonical_dst_value_and_ignore_target_shape"
require "$PRINTER_HELPERS" "pub(crate) fn format_call_target"
require "$PRINTER_DISPLAY" "format_call_target(callee.as_ref(), *func, args)"
require "$PRINTER_TESTS" "typed_printer_projects_callee_and_ignores_stale_func"
require "$PRINTER_TESTS" "printer_preserves_explicit_legacy_call_rendering"
require "$JSON_CALLS" "fn emit_call_with_callee_v0"
require "$JSON_CALLS" "v0_typed_call_variants_ignore_stale_numeric_func_decoration"
require "$JSON_CALLS" "v0_legacy_call_preserves_explicit_numeric_func_decoration"
require "$JSON_CALLS" "method_none_keeps_legacy_receiver_func_until_r6"
require "$JSON_ROOT" "pub(crate) enum JsonEgressProfile"
require "$JSON_ROOT" "json_profile_selector_matrix_is_finite_and_root_owned"
require "$JSON_ROOT" "json_profile_selector_rejects_mixed_and_invalid_values"
require "$JSON_EMITTERS" "profile: JsonEgressProfile"
require "$BACKEND_SHAPE" "normalize_program_json_bridge_backend_shape"
require "$BACKEND_SHAPE" "rejects_console_externcall_with_defaultable_fields_missing"
require "$BACKEND_SHAPE" "rejects_console_externcall_with_malformed_values"
require "$BACKEND_SHAPE" "rejects_externcall_with_unowned_extra_fields"
require "$BACKEND_SHAPE" "bridge backend-shape missing functions array"
require "$HANDOFF" "with_phase0_mir_json_env(|| {"
require "$HANDOFF" "super::normalize_program_json_bridge_backend_shape(&mir_json)"
require "$LLVM_GENERIC_CALLS" "legacy_call_missing_structured_callee"
require "$LLVM_MIR_CALL_DISPATCH" 'strcmp(ctype, "Extern")'
require "$LLVM_MIR_CALL_SURFACE" "classify_mir_call_string_extern_surface"
require "$LLVM_MIR_CALL_EXTERN" "hako_llvmc_ffi_mir_call_shell_extern_rules.inc"
require "$LLVM_MIR_CALL_EXTERN" "hako_llvmc_ffi_mir_call_shell_extern_emit_body.inc"
require "$LLVM_MIR_CALL_EXTERN_RULES" "lowering_plan_extern_emit_rule_matches"
require "$LLVM_MIR_CALL_EXTERN_RULES" "struct MirCallExternEmitRule"
require "$LLVM_MIR_CALL_EXTERN_BODY" "emit_extern_call_lowering_plan_mir_call"

if rg -F -q "normalize_program_json_bridge_backend_module_shape" "$BACKEND_SHAPE" "$MIR_BUILDER"; then
  fail "typed backend_shape mutation retained a second semantic authority"
fi
if rg -F -q 'strcmp(op, "externcall")' "$LLVM_GENERIC_CALLS"; then
  fail "selected structured ny-llvmc dispatcher gained a raw externcall terminal"
fi

if rg -F -q "Option<Callee>" "$MIR_V0_CALL" || rg -F -q "callee: None" "$MIR_V0_CALL" || rg -F -q "parse_call_callee" "$MIR_V0_CALL"; then
  fail "MIR JSON-v0 call owner retained an optional/missing-callee target state"
fi

python3 - "$LLVM" "$ROOT_DIR" <<'PY'
from pathlib import Path
import sys

llvm = Path(sys.argv[1]).read_text()
root = Path(sys.argv[2])

start = llvm.index("let mut module = if selected_dynamic")
reject = llvm.index("if let Err(error) = reject_selected_dynamic_legacy_callsites", start)
backend = llvm.index("match execute_via_harness_or_fallback", reject)
window = llvm[start:backend]

if "into_verified_module" not in window:
    raise SystemExit("selected corridor does not verify the module")
if "project_module_to_legacy_calls" in window:
    raise SystemExit("compatibility legacy projection crossed selected corridor")
if not (window.index("into_verified_module") < window.index("reject_selected_dynamic_legacy_callsites")):
    raise SystemExit("selected verification/rejection order drifted")

definition_start = llvm.index("fn reject_selected_dynamic_legacy_callsites")
definition_end = llvm.index("struct LlvmExecutionOutcome", definition_start)
definition = llvm[definition_start:definition_end]
for token in ("block.instructions", "block.terminator", "legacy_callsite_reject_code"):
    if token not in definition:
        raise SystemExit(f"selected legacy scanner lost {token}")

for relative in (
    "src/mir/instruction/methods.rs",
    "src/mir/optimizer/core.rs",
    "src/mir/passes/callsite_canonicalize/schedule.rs",
    "src/mir/passes/cse.rs",
    "src/mir/optimizer_passes/diagnostics.rs",
    "src/backend/mir_interpreter/handlers/calls/mod.rs",
    "src/mir/contracts/backend_core_ops/allowlists.rs",
    "src/runner/product/llvm/mod.rs",
    "crates/hakorune_mir_defs/src/call_unified.rs",
    "src/mir/passes/simplify_cfg/flow.rs",
    "src/mir/value_consumer.rs",
    "src/mir/escape_barrier.rs",
    "src/mir/ownership_ssa/verify.rs",
    "src/mir/ownership_ssa/tests.rs",
    "src/mir/query.rs",
    "src/mir/instruction/tests.rs",
    "src/mir/printer_helpers.rs",
    "src/mir/instruction/display.rs",
    "src/mir/printer/tests.rs",
    "src/runner/mir_json_emit/emitters/calls.rs",
    "src/runner/mir_json_emit/emitters/mod.rs",
    "src/runner/mir_json_emit/helpers.rs",
    "src/runner/mir_json_emit/root.rs",
    "src/host_providers/mir_builder.rs",
    "src/host_providers/mir_builder/backend_shape.rs",
    "src/host_providers/mir_builder/handoff.rs",
    "tools/checks/mir_call_canonical_corridor_guard.sh",
    "lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell_extern_emit.inc",
    "lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell_extern_rules.inc",
    "lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell_extern_emit_body.inc",
):
    lines = (root / relative).read_text().splitlines()
    if len(lines) >= 800:
        raise SystemExit(f"800-line hard stop reached: {relative} ({len(lines)})")

call_ops = (root / "src/runner/json_v0_bridge/lowering/expr/call_ops.rs").read_text()
if call_ops.count("callee: None") != 0:
    raise SystemExit("Program JSON-v0 missing-callee producer was reintroduced")

generic_start = call_ops.index("pub(super) fn lower_call_expr")
generic_end = call_ops.index("pub(super) fn lower_array_values_expr", generic_start)
generic = call_ops[generic_start:generic_end]
for required in (
    "env.program_call_targets.resolve(name, args.len())?",
    "MirInstruction::call(",
    "EffectMask::READ",
):
    if required not in generic:
        raise SystemExit(f"generic Program call lost catalog issuer evidence: {required}")
if generic.index("program_call_targets.resolve") > generic.index("lower_args_with_scope"):
    raise SystemExit("generic Program call resolves target after argument lowering")
for forbidden in ("fun_value", "ConstValue::String(name", "callee: None"):
    if forbidden in generic:
        raise SystemExit(f"generic Program call retained legacy target carrier: {forbidden}")

for name, next_name in (
    ("lower_stageb_static_call_for_box", "lower_stageb_instance_call_for_box"),
    ("lower_stageb_instance_call_for_box", "lower_stageb_static_method_call"),
):
    start = call_ops.index(f"fn {name}")
    end = call_ops.index(f"fn {next_name}", start)
    window = call_ops[start:end]
    for forbidden in ("callee: None", "fun_val", "func:", "ConstValue::String(qualified)"):
        if forbidden in window:
            raise SystemExit(f"{name} retained legacy target edge: {forbidden}")
    for required in ("MirInstruction::call(", "Callee::Global(qualified)", "EffectMask::READ"):
        if required not in window:
            raise SystemExit(f"{name} lost canonical qualified Call evidence: {required}")

constructor = (root / "src/mir/instruction/methods.rs").read_text()
start = constructor.index("pub(crate) fn call(")
end = constructor.index("pub fn extern_name", start)
window = constructor[start:end]
for required in ("func: ValueId::INVALID", "callee: Some(callee)"):
    if required not in window:
        raise SystemExit(f"canonical Call constructor drifted: {required}")

callee = (root / "crates/hakorune_mir_defs/src/call_unified.rs").read_text()
projection_start = callee.index("pub fn for_each_value_operand")
projection_end = callee.index("/// Call flags", projection_start)
projection = callee[projection_start:projection_end]
for token in (
    "Callee::Global",
    "Callee::Extern",
    "Callee::Constructor",
    "Callee::Method",
    "Callee::Closure",
    "Callee::Value",
):
    if token not in projection:
        raise SystemExit(f"Callee projection lost explicit variant: {token}")
if "_ =>" in projection:
    raise SystemExit("Callee projection introduced a wildcard variant arm")
if "pub fn rewrite_value_operands" not in projection:
    raise SystemExit("Callee projection lost mutable rewrite facet")

flow = (root / "src/mir/passes/simplify_cfg/flow.rs").read_text()
rewrite_start = flow.index("fn rewrite_value_uses_in_instruction")
call_start = flow.index("MirInstruction::Call {", rewrite_start)
call_end = flow.index("MirInstruction::NewClosure", call_start)
call_window = flow[call_start:call_end]
if "Callee::" in call_window:
    raise SystemExit("SimplifyCFG retained a pass-local Callee match")
if call_window.count("rewrite_value_operands") != 1:
    raise SystemExit("SimplifyCFG Call arm does not delegate exactly once")

methods = (root / "src/mir/instruction/methods.rs").read_text()
used_start = methods.index("pub fn used_values")
used_window = methods[used_start:]
if used_window.count("callee.for_each_value_operand") != 1:
    raise SystemExit("used_values Call arm does not delegate exactly once")
if used_window.index("callee.for_each_value_operand") > used_window.index("used.extend(args"):
    raise SystemExit("used_values emits args before typed Callee operands")
if "match callee" in used_window:
    raise SystemExit("used_values retained a consumer-local Callee match")

value_consumer = (root / "src/mir/value_consumer.rs").read_text()
consumer_start = value_consumer.index("fn value_consumer_used_values")
consumer_end = value_consumer.index("pub fn refresh_function_value_consumer_facts", consumer_start)
consumer_window = value_consumer[consumer_start:consumer_end]
if consumer_window.count("MirInstruction::Call { .. } => inst.used_values()") != 1:
    raise SystemExit("value_consumer Call arm does not delegate exactly once")
if "MirInstruction::Call { func" in consumer_window:
    raise SystemExit("value_consumer retained a direct legacy func carrier in the Call arm")
for token in ("match_method_set_call", "record_direct_set_consumer_use", "record_other_uses"):
    if token not in value_consumer:
        raise SystemExit(f"value_consumer lost fact boundary helper: {token}")

escape = (root / "src/mir/escape_barrier.rs").read_text()
role_start = escape.index("MirInstruction::Call { callee, args, .. }")
role_end = escape.index("MirInstruction::Store", role_start)
role_window = escape[role_start:role_end]
if role_window.count("callee.for_each_value_operand") != 1:
    raise SystemExit("escape Call arm does not delegate Callee occurrence projection exactly once")
if "Callee::Method" in role_window or "receiver:" in role_window:
    raise SystemExit("escape Call arm retained a field-level Callee/receiver scan")
if "Callee::Closure { .. }" not in role_window:
    raise SystemExit("escape Call arm lost the Closure Capture role boundary")
if role_window.count("EscapeBarrier::Capture") != 1 or role_window.count("EscapeBarrier::Call") < 2:
    raise SystemExit("escape Call arm lost the accepted Call/Capture role matrix")
if "func" in role_window:
    raise SystemExit("escape Call arm reintroduced legacy func as a shared barrier")

ownership = (root / "src/mir/ownership_ssa/verify.rs").read_text()
ownership_start = ownership.index("fn verify_call_ownership")
ownership_end = ownership.index("fn process_instruction", ownership_start)
ownership_window = ownership[ownership_start:ownership_end]
for token in (
    "None => Some(func)",
    "Callee::Global",
    "Callee::Extern",
    "Callee::Constructor",
    "Callee::Method",
    "Callee::Closure",
    "Callee::Value",
    "MirOwnershipKindV1::None",
    "ManagedCallOwnershipUnsupported",
):
    if token not in ownership_window:
        raise SystemExit(f"ownership Call policy lost {token}")
typed_window = ownership_window[ownership_window.index("Some(Callee::"):]
if "func" in typed_window:
    raise SystemExit("typed ownership Call policy re-read legacy func")
if "instruction.used_values()" not in ownership[ownership.index("fn process_instruction"):]:
    raise SystemExit("ownership verifier lost generic used_values liveness")

instruction_kinds = (root / "src/mir/instruction_kinds/mod.rs").read_text()
if "CallLikeInst" in instruction_kinds:
    raise SystemExit("instruction_kinds retained the retired CallLike adapter")
if instruction_kinds.count("MirInstruction::Call") < 2:
    raise SystemExit("instruction_kinds lost direct Call metadata arms")
if "Some(i.used_values())" not in instruction_kinds:
    raise SystemExit("instruction_kinds Call use metadata lost canonical delegation")

optimizer = (root / "src/mir/optimizer/core.rs").read_text()
if "MirOptimizerLateCallAndInline" in optimizer:
    raise SystemExit("optimizer retained the retired callsite schedule")
if "call_callee_{:?}_" not in optimizer:
    raise SystemExit("optimizer key lost canonical Callee projection")

cse = (root / "src/mir/passes/cse.rs").read_text()
if "call_closure_{}_" in cse or "Use func as distinguisher" in cse:
    raise SystemExit("CSE retained a Closure key based on legacy func")
if "call_callee_{:?}_" not in cse:
    raise SystemExit("CSE lost canonical Callee key projection")

diagnostics = (root / "src/mir/optimizer_passes/diagnostics.rs").read_text()
diag_end = diagnostics.index("#[cfg(test)]")
diag_owner = diagnostics[:diag_end]
if (
    "def_map" in diag_owner
    or "ConstValue::String" in diag_owner
    or "MirInstruction::Call { func" in diag_owner
):
    raise SystemExit("optimizer diagnostics retained legacy func-to-Const target observation")

interpreter = (root / "src/backend/mir_interpreter/handlers/calls/mod.rs").read_text()
interpreter_end = interpreter.index("#[cfg(test)]")
interpreter_owner = interpreter[:interpreter_end]
if "reg_load(func)" in interpreter_owner or "functions.get(s)" in interpreter_owner:
    raise SystemExit("Rust interpreter retained the missing-Callee by-name execution edge")
if "call-missing-callee: typed Callee required" not in interpreter_owner:
    raise SystemExit("Rust interpreter lost the typed missing-Callee terminal reject")
if "self.execute_callee_call(callee_type, args)?" not in interpreter_owner:
    raise SystemExit("Rust interpreter lost the typed Callee execution path")

query = (root / "src/mir/query.rs").read_text()
query_impl = query.index("impl<'m> MirQuery for MirQueryBox")
reads_start = query.index("fn reads_of", query_impl)
writes_start = query.index("fn writes_of", reads_start)
reads_window = query[reads_start:writes_start]
if "Call { .. } => inst.used_values()" not in reads_window:
    raise SystemExit("MirQuery reads_of lost canonical Call delegation")
if "callee" in reads_window or "func" in reads_window:
    raise SystemExit("MirQuery reads_of retained local target/func reconstruction")
writes_end = query.index("#[cfg(test)]", writes_start)
writes_window = query[writes_start:writes_end]
if "return inst.dst_value().into_iter().collect()" not in writes_window:
    raise SystemExit("MirQuery writes_of lost canonical Call delegation")

printer_helpers = (root / "src/mir/printer_helpers.rs").read_text()
helper_start = printer_helpers.index("pub(crate) fn format_call_target")
helper_end = printer_helpers.index("pub fn format_instruction", helper_start)
helper_window = printer_helpers[helper_start:helper_end]
for token in (
    "Some(Callee::Global",
    "Some(Callee::Method",
    "Some(Callee::Constructor",
    "Some(Callee::Closure",
    "Some(Callee::Value",
    "Some(Callee::Extern",
    "None => format!(\"call_legacy",
):
    if token not in helper_window:
        raise SystemExit(f"printer target projection lost {token}")
if "resolve" in helper_window or "retry" in helper_window or "ConstValue" in helper_window:
    raise SystemExit("printer target projection retained semantic target reconstruction")

display = (root / "src/mir/instruction/display.rs").read_text()
call_start = display.index("MirInstruction::Call {")
call_end = display.index("MirInstruction::Return", call_start)
display_call = display[call_start:call_end]
if "callee: _" in display_call or "TODO: Use callee" in display_call:
    raise SystemExit("MIR Display retained the stale func-only Call observer")
if display_call.count("format_call_target(callee.as_ref(), *func, args)") != 1:
    raise SystemExit("MIR Display does not delegate Call rendering exactly once")

printer_tests = (root / "src/mir/printer/tests.rs").read_text()
for token in (
    '"%1 = call_value %7(%2)"',
    '"call_legacy %99(%2)"',
    '"%99"',
):
    if token not in printer_tests:
        raise SystemExit(f"printer parity tests lost {token}")

json_calls = (root / "src/runner/mir_json_emit/emitters/calls.rs").read_text()
typed_start = json_calls.index("fn emit_call_with_callee_v0")
typed_end = json_calls.index("fn emit_call_with_optional_func", typed_start)
typed_helper = json_calls[typed_start:typed_end]
if "func" in typed_helper or "emit_call_with_optional_func" in typed_helper:
    raise SystemExit("typed v0 JSON projection retained legacy func decoration")
if '"callee": callee' not in typed_helper:
    raise SystemExit("typed v0 JSON projection lost explicit callee emission")
if "receiver.unwrap_or(*func)" not in json_calls:
    raise SystemExit("Method(None) compatibility receiver projection was removed before R6")
import re
if re.search(r"emit_call_with_callee_v0\s*\(\s*dst\s*,\s*func\b", json_calls):
    raise SystemExit("typed v0 JSON call site still forwards legacy func")
if "fn emit_call_with_optional_func" not in json_calls:
    raise SystemExit("legacy v0 JSON call helper was removed before its compatibility row")

json_root = (root / "src/runner/mir_json_emit/root.rs").read_text()
if json_root.count("JsonEgressProfile::from_env()") != 1:
    raise SystemExit("JSON root does not select exactly one egress profile")
if json_root.count("let use_v1_schema = profile.is_canonical_v1()") != 1:
    raise SystemExit("JSON root schema kind is not projected from the selected profile")
root_emit_start = json_root.index("emitters::emit_non_phi_instructions")
if "profile," not in json_root[root_emit_start:]:
    raise SystemExit("JSON root does not pass the selected profile to emitters")
for relative in (
    "src/runner/mir_json_emit/emitters/calls.rs",
    "src/runner/mir_json_emit/emitters/mod.rs",
    "src/runner/mir_json_emit/helpers.rs",
):
    owner = (root / relative).read_text()
    if any(token in owner for token in (
        "NYASH_JSON_SCHEMA_V1",
        "NYASH_MIR_UNIFIED_CALL",
        "HAKO_MIR_BUILDER_METHODIZE",
    )):
        raise SystemExit(f"{relative} retained an independent JSON profile selector read")
if "calls::emit_call(dst, func, callee.as_ref(), args, effects, profile)" not in (root / "src/runner/mir_json_emit/emitters/mod.rs").read_text():
    raise SystemExit("Call emitter lost the root-selected profile argument")

print("[mir-call-canonical-corridor-guard] ok")
PY
