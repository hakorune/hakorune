#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-phi-binding-boundary"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1021-COREPLAN-PHI-BINDING-SSOT-001.md"
INDEX="docs/tools/check-scripts-index.md"
LOCAL_PATCH_SSOT="docs/development/current/main/design/local-patch-prevention-ssot.md"
DEV_GATE_STEPS="tools/checks/lib/dev_gate_quick_steps.sh"
SELF_SCRIPT="tools/checks/coreplan_phi_binding_boundary_guard.sh"
PREHEADER="src/mir/builder/control_flow/plan/features/nested_loop_depth1_preheader.rs"
LOOP_COND="src/mir/builder/control_flow/plan/features/loop_cond_bc.rs"

echo "[$TAG] checking PHI / BindingState / RecipeOnly boundary"

guard_require_files \
  "$TAG" \
  "$TASKBOARD" \
  "$CARD" \
  "$INDEX" \
  "$LOCAL_PATCH_SSOT" \
  "$DEV_GATE_STEPS" \
  "$SELF_SCRIPT" \
  "$PREHEADER" \
  "$LOOP_COND"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-PHI-BINDING-SSOT-001" \
  "$TASKBOARD" \
  "taskboard must record PHI/BINDING stop-the-line row"
guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-PHI-BINDING-SSOT-001" \
  "$CARD" \
  "phase card must record PHI/BINDING stop-the-line row"
guard_expect_fixed_in_file "$TAG" \
  "pub(in crate::mir::builder) struct PhiTxn" \
  "src/mir/builder/emission/phi_lifecycle.rs" \
  "phi_lifecycle must expose a transaction wrapper"
guard_expect_fixed_in_file "$TAG" \
  "[freeze:contract][phi_lifecycle/provisional_left_unpatched]" \
  "src/mir/builder/emission/phi_lifecycle.rs" \
  "PhiTxn commit must fail-fast on unpatched provisional PHIs"
guard_expect_fixed_in_file "$TAG" \
  "[freeze:contract][phi_lifecycle/txn_abort]" \
  "src/mir/builder/emission/phi_lifecycle.rs" \
  "PhiTxn abort must fail-fast after rollback"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$INDEX" \
  "check index must list this guard"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$DEV_GATE_STEPS" \
  "dev_gate quick must include this guard"
guard_expect_fixed_in_file "$TAG" \
  "same failure class + 2 local patches" \
  "$LOCAL_PATCH_SSOT" \
  "local patch prevention SSOT must define the two-strike stop line"
guard_expect_fixed_in_file "$TAG" \
  "$LOCAL_PATCH_SSOT" \
  "$CARD" \
  "phase card must point to local patch prevention SSOT"
guard_expect_fixed_in_file "$TAG" \
  "$LOCAL_PATCH_SSOT" \
  "docs/development/current/main/design/compiler-expressivity-first-policy.md" \
  "compiler expressivity policy must point to local patch prevention SSOT"

python3 - <<'PY'
from pathlib import Path

path = Path("src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs")
text = path.read_text()
for forbidden in (
    "instructions.push(MirInstruction::Phi",
    "add_instruction(MirInstruction::Phi",
    "emit_instruction(MirInstruction::Phi",
    "MirInstruction::Phi {",
):
    if forbidden in text:
        raise SystemExit(
            "[coreplan-phi-binding-boundary] ERROR: exit_phi_builder must construct PHIs through phi_lifecycle/PhiTxn: "
            f"{forbidden}"
        )
PY

python3 - <<'PY'
from pathlib import Path

path = Path("src/mir/builder/control_flow/plan/features/nested_loop_depth1_preheader.rs")
text = path.read_text()
prod = text.split("#[cfg(test)]", 1)[0]
for forbidden in (
    "capture_external_values",
    "collect_used_values",
    "collect_defined_values",
    "remap_loop_uses",
    "remap_plan_uses",
    "CoreEffectPlan",
    "MirType",
    "BTreeMap",
    "BTreeSet",
    "alloc_typed",
    "CoreEffectPlan::Copy",
):
    if forbidden in prod:
        raise SystemExit(
            "[coreplan-phi-binding-boundary] ERROR: nested_loop_depth1 "
            f"preheader freshness must not allocate/copy/remap arbitrary values: {forbidden}"
        )
PY

python3 - <<'PY'
from pathlib import Path

path = Path("src/mir/builder/control_flow/plan/features/loop_cond_bc.rs")
text = path.read_text()
needle = "BodyLoweringPolicy::RecipeOnly"
start = text.find(needle)
if start < 0:
    raise SystemExit(f"[coreplan-phi-binding-boundary] ERROR: {needle} arm not found")

end_marker = "let body_entry_bindings = current_bindings.clone();"
end = text.find(end_marker, start)
if end < 0:
    raise SystemExit("[coreplan-phi-binding-boundary] ERROR: RecipeOnly arm boundary not found")

section = text[start:end]
for forbidden in (
    "try_build_exit_allowed_block_recipe",
    ".or_else(|err|",
    ".body_exit_allowed.clone",
):
    if forbidden in section:
        raise SystemExit(
            "[coreplan-phi-binding-boundary] ERROR: RecipeOnly arm must not "
            f"perform route-level whole-body fallback: found {forbidden}"
        )
PY

python3 - <<'PY'
import subprocess
from pathlib import Path

allowed_low_level_phi_call_prefixes = (
    "src/mir/builder/emission/phi_lifecycle.rs:",
)

low_level_patterns = (
    r"cf_common::insert_phi_at_head",
    r"insert_phi_at_head_spanned\(",
    r"insert_phi_at_head\(",
    r"\.update_phi_instruction\(",
)

cmd = [
    "rg",
    "-n",
    "|".join(low_level_patterns),
    "src/mir/builder",
    "-g",
    "*.rs",
]
result = subprocess.run(cmd, text=True, capture_output=True)
lines = result.stdout.splitlines() if result.returncode in (0, 1) else []
violations = []
for line in lines:
    if any(line.startswith(prefix) for prefix in allowed_low_level_phi_call_prefixes):
        continue
    # Comments documenting the old failure mode are not callsites.
    if line.lstrip().startswith("//") or line.lstrip().startswith("//!"):
        continue
    violations.append(line)

if violations:
    print("[coreplan-phi-binding-boundary] ERROR: low-level PHI lifecycle calls outside phi_lifecycle")
    print("\n".join(violations))
    raise SystemExit(1)

allowed_prefixes = (
    "src/mir/ssot/cf_common.rs:",
    "src/mir/builder/emission/phi_lifecycle.rs:",
    "src/mir/builder/ssa/phi_input_materializer.rs:",
    "src/mir/builder/record_helper_args.rs:",
    "src/mir/builder/control_flow/joinir/merge/loop_header_phi_builder.rs:",
    "src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs:",
    "src/mir/builder/control_flow/joinir/merge/phi_block_remapper.rs:",
    "src/mir/builder/control_flow/joinir/merge/rewriter/stages/plan/instruction_rewrite.rs:",
    "src/mir/builder/control_flow/plan/lowerer/effect_emission.rs:",
    "src/mir/builder/control_flow/plan/features/if_join.rs:",
    "src/mir/builder/control_flow/edgecfg/api/emit/tests.rs:",
)

patterns = (
    r"emit_instruction\(MirInstruction::Phi",
    r"add_instruction\(MirInstruction::Phi",
    r"instructions\.push\(MirInstruction::Phi",
    r"MirInstruction::Phi \{",
)

cmd = [
    "rg",
    "-n",
    "|".join(patterns),
    "src/mir/builder",
    "src/mir/ssot",
    "-g",
    "*.rs",
]
result = subprocess.run(cmd, text=True, capture_output=True)
lines = result.stdout.splitlines() if result.returncode in (0, 1) else []
violations = []
for line in lines:
    if any(line.startswith(prefix) for prefix in allowed_prefixes):
        continue
    # Pattern matching / read-only inspection is allowed outside PHI builders.
    if (
        "matches!(" in line
        or "if let" in line
        or "let Some(" in line
        or "=>" in line
        or "| MirInstruction::Phi" in line
    ):
        continue
    violations.append(line)

if violations:
    print("[coreplan-phi-binding-boundary] ERROR: direct PHI construction outside allowlisted owners")
    print("\n".join(violations))
    raise SystemExit(1)
PY

echo "[$TAG] nested_loop_preheader_hidden_value_capture=0"
echo "[$TAG] recipe_only_whole_body_fallback=0"
echo "[$TAG] phi_low_level_callsite_owner=phi_lifecycle"
echo "[$TAG] phi_transaction_boundary_defined=1"
echo "[$TAG] joinir_exit_phi_builder_direct_phi_construction=0"
echo "[$TAG] phi_direct_emit_no_growth=1"
echo "[$TAG] ok"
