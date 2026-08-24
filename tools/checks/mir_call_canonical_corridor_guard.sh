#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-canonical-corridor-guard"
LLVM="$ROOT_DIR/src/runner/product/llvm/mod.rs"
OPTIMIZER="$ROOT_DIR/src/mir/optimizer/core.rs"
SCHEDULE="$ROOT_DIR/src/mir/passes/callsite_canonicalize/schedule.rs"
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

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

require() {
  local file="$1"
  local token="$2"
  rg -F -q -- "$token" "$file" || fail "missing '$token' in ${file#$ROOT_DIR/}"
}

for file in "$LLVM" "$OPTIMIZER" "$SCHEDULE" "$REJECT" "$JSON" "$PROGRAM_LOWERING" "$EXEC" "$CALL_OPS" "$PROGRAM_CALL_TARGETS" "$METHODS" "$MIR_V0_CALL" "$MIR_V0_CATALOG" "$MIR_V0_MODULE"; do
  [[ -f "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done

require "$OPTIMIZER" "CallsiteCanonicalizeScheduleSite::MirOptimizerLateCallAndInline"
require "$SCHEDULE" "MirOptimizerLateCallAndInline"
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
    "src/mir/contracts/backend_core_ops/allowlists.rs",
    "src/runner/product/llvm/mod.rs",
    "tools/checks/mir_call_canonical_corridor_guard.sh",
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

print("[mir-call-canonical-corridor-guard] ok")
PY
