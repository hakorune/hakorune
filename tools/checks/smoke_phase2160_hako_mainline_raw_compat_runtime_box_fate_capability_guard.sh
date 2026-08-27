#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="smoke-phase2160-hako-mainline-raw-compat-runtime-box-fate-capability"
CARD="$ROOT_DIR/docs/development/current/main/investigations/smoke-phase2160-hako-mainline-raw-compat-runtime-box-fate-capability-i0-2026-08-27.toml"
REGISTRY="$ROOT_DIR/tools/checks/guard_rows.toml"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

for path in "$CARD" "$REGISTRY" \
  "$ROOT_DIR/src/mir/builder.rs" \
  "$ROOT_DIR/src/mir/builder/raw_compat_runtime_box_fate.rs" \
  "$ROOT_DIR/src/mir/builder/recursive_child_lowering.rs" \
  "$ROOT_DIR/src/mir/builder/program_root_lowering.rs" \
  "$ROOT_DIR/src/mir/builder/raw_expression_dispatch/nonmain_static_box_lifecycle.rs" \
  "$ROOT_DIR/src/mir/builder/instance_box_declaration_lifecycle.rs" \
  "$ROOT_DIR/src/mir/builder/runtime_box_fate_tests.rs" \
  "$ROOT_DIR/src/mir/builder/module_lowering_invocation_reentrant_tests.rs"; do
  [[ -e "$path" ]] || fail "missing owner"
done

python3 - "$ROOT_DIR" "$CARD" "$REGISTRY" <<'PY'
from pathlib import Path
import sys
import tomllib

root, card_path, registry_path = map(Path, sys.argv[1:])
card = tomllib.loads(card_path.read_text())
registry = tomllib.loads(registry_path.read_text())

status = card.get("status")
if status not in ("selected_fast", "in_progress", "landed_bounded_child_row"):
    raise SystemExit(f"I0 card has unexpected lifecycle status: {status!r}")
if card.get("implementation_permission") is not True:
    raise SystemExit("I0 implementation permission is not open")
if card.get("selected_fast") is not True:
    raise SystemExit("I0 is not marked as the selected fast row")
if status == "landed_bounded_child_row":
    if not card.get("landed_commit"):
        raise SystemExit("landed I0 card is missing landed_commit evidence")
    evidence = card.get("evidence")
    if not isinstance(evidence, dict) or not evidence.get("implementation"):
        raise SystemExit("landed I0 card is missing implementation evidence")

guard_id = "smoke-phase2160-hako-mainline-raw-compat-runtime-box-fate-capability"
guard_script = "tools/checks/smoke_phase2160_hako_mainline_raw_compat_runtime_box_fate_capability_guard.sh"
rows = registry.get("rows")
if not isinstance(rows, list):
    raise SystemExit("guard registry rows are missing")
matches = [row for row in rows if row.get("id") == guard_id]
if len(matches) != 1:
    raise SystemExit(f"expected exactly one registry row for {guard_id}, found {len(matches)}")
row = matches[0]
if row.get("profiles") != ["pilot", "quick-static"]:
    raise SystemExit("I0 guard profiles drifted")
if row.get("cmd") != ["bash", guard_script]:
    raise SystemExit("I0 guard command drifted")

def read(rel: str) -> str:
    return (root / rel).read_text()

builder = read("src/mir/builder.rs")
fate = read("src/mir/builder/raw_compat_runtime_box_fate.rs")
recursive = read("src/mir/builder/recursive_child_lowering.rs")
lowering = read("src/mir/builder/program_root_lowering.rs")
static = read("src/mir/builder/raw_expression_dispatch/nonmain_static_box_lifecycle.rs")
instance = read("src/mir/builder/instance_box_declaration_lifecycle.rs")
tests = read("src/mir/builder/runtime_box_fate_tests.rs")
old_tests = read("src/mir/builder/module_lowering_invocation_reentrant_tests.rs")

if builder.count("mod raw_compat_runtime_box_fate;") != 1:
    raise SystemExit("runtime Box fate module is not registered exactly once")
if builder.count("mod runtime_box_fate_tests;") != 1:
    raise SystemExit("runtime Box fate test sibling is not registered exactly once")

struct_start = fate.index("struct RawCompatibilityRuntimeBoxFateV1")
struct_end = fate.index("\n}\n", struct_start) + 3
struct = fate[struct_start:struct_end]
if struct.count("state: RawCompatibilityRuntimeBoxFateStateV1") != 1:
    raise SystemExit("fate capability does not have the single state field")
for token in (
    "Callee", "ValueId", "AST", "RawInvocationRootLineageV1", "EffectMask",
    "Rc<", "Arc<", "RefCell", "into_parts", "Option<",
):
    if token in struct:
        raise SystemExit(f"forbidden fate capability payload or escape: {token}")
derive_start = fate.rfind("#[derive", 0, struct_start)
if "Clone" in fate[derive_start:struct_start] or "Copy" in fate[derive_start:struct_start]:
    raise SystemExit("fate capability became Clone/Copy")

scope_start = fate.index("enum RuntimeBoxFateScopeV1")
scope_end = fate.index("\n}\n", scope_start) + 3
scope = fate[scope_start:scope_end]
if "Unarmed" not in scope or "Phase2160(&'scope mut RawCompatibilityRuntimeBoxFateV1)" not in scope:
    raise SystemExit("scope is not the explicit Unarmed/Phase2160 borrow sum")
if "Option<" in scope or "Clone" in scope or "Copy" in scope:
    raise SystemExit("scope reintroduced option/copy authority")
if "fn reborrow<'short>" not in fate or "&mut **fate" not in fate:
    raise SystemExit("short scoped fate reborrow is missing")
if "#[must_use" not in fate:
    raise SystemExit("fate capability is not must-use")

if lowering.count("with_phase2160_raw_compat_runtime_box_fate_v1(") != 1:
    raise SystemExit("phase2160 fate issuer/scope is not unique")
script_runtime = lowering.index("ProgramRootTerminalScheduleV1::ScriptRuntime")
scope_call = lowering.index("with_phase2160_raw_compat_runtime_box_fate_v1(", script_runtime)
if not script_runtime < scope_call:
    raise SystemExit("fate scope is not inside ScriptRuntime arm")
if "immediate.lower_raw_compat_with_port_v1" not in lowering or "deferred.lower_raw_compat_with_port_v1" not in lowering:
    raise SystemExit("immediate/deferred RawCompatibility routes disappeared")

if recursive.count("runtime_box_fate: RuntimeBoxFateScopeV1<'port>") != 1:
    raise SystemExit("RawInvocation does not carry the scoped fate field")
if recursive.count("runtime_box_fate: self.runtime_box_fate.reborrow()") != 1:
    raise SystemExit("RawInvocation recursive reborrow does not carry fate")
if recursive.count("fn take_runtime_box_fate_v1") < 3:
    raise SystemExit("default, inherent, and production fate consumers are not all present")
for rel in (
    "src/mir/builder/recursive_child_lowering/legacy_port.rs",
    "src/mir/builder/normal_callable_semantic_loan_port.rs",
    "src/mir/builder/program_root_work_plan/raw_compatibility.rs",
    "src/mir/builder/raw_compatibility_child_terminal.rs",
):
    text = read(rel)
    if "with_phase2160_raw_compat_runtime_box_fate_v1(" in text:
        raise SystemExit(f"excluded owner opened phase2160 fate scope: {rel}")

static_app = static.index("if prepared_root_app_mode_v1(builder)?")
static_take = static.index("take_runtime_box_fate_v1()", static_app)
static_register = static.index("register_user_box", static_take)
if not static_app < static_take < static_register:
    raise SystemExit("F fate is not consumed after App no-op and before registration")
normal_start = static.index("pub(in crate::mir::builder) fn lower_normal_with_port_v1")
if "take_runtime_box_fate_v1" in static[normal_start:]:
    raise SystemExit("SelectedNormal/static normal path inherited the phase2160 fate")

instance_take = instance.index("take_runtime_box_fate_v1()")
instance_prefix = instance.index("lower_declaration_prefix_v1", instance_take)
if not instance_take < instance_prefix:
    raise SystemExit("I/H fate is not consumed before declaration prefix")
raw_compat_start = instance.index("pub(in crate::mir::builder) fn lower_raw_compat_with_port_v1")
if "take_runtime_box_fate_v1" in instance[raw_compat_start:]:
    raise SystemExit("root I1 raw compatibility path inherited the phase2160 fate")

for name in (
    "phase2160_raw_compat_static_nested_static_box_is_typed_retire",
    "phase2160_raw_compat_static_nested_instance_box_is_typed_retire",
    "phase2160_raw_compat_nested_instance_constructor_is_typed_retire",
    "phase2160_raw_compat_nested_depth_three_is_typed_retire",
):
    if tests.count(f"fn {name}(") != 1:
        raise SystemExit(f"focused typed-negative evidence is missing: {name}")
if tests.count("raw-compat/runtime-box-fate-retired/") < 1:
    raise SystemExit("typed retirement assertion is missing")
if tests.count("assert_zero_delta") < 2 or "snapshot(invocation)" not in tests:
    raise SystemExit("snapshot zero-delta evidence is missing")
for token in (
    "module_functions",
    "module_metadata",
    "compilation_context",
    "metadata_context",
    "recursion_depth",
):
    if token not in tests:
        raise SystemExit(f"zero-delta snapshot does not cover {token}")
if "RawInvocationSourceTransportV1::script_root(())" not in tests:
    raise SystemExit("phase2160 negative evidence is not rooted in ScriptRoot transport")
if tests.count("generic_raw_invocation_keeps_unarmed_nested_box_success()") != 1:
    raise SystemExit("generic unarmed RawInvocation success parity is missing")
for old_name in (
    "port_aware_static_body_collects_nested_static_child_before_outer_commit",
    "port_aware_static_body_collects_nested_instance_child_before_outer_commit",
    "port_aware_nested_instance_constructor_uses_the_same_child_terminal",
    "raw_capture_commit_reaches_static_instance_constructor_depth_three",
):
    if f"fn {old_name}(" in old_tests:
        raise SystemExit(f"old positive success assertion remains in parent test file: {old_name}")
for symbol in ("NestedStatic.run/0", "NestedInstance.run/0", "NestedCtor.birth/0", "Leaf.birth/0"):
    if symbol in old_tests:
        raise SystemExit(f"moved success fixture remains in parent test file: {symbol}")

for rel in (
    "src/mir/builder.rs",
    "src/mir/builder/raw_compat_runtime_box_fate.rs",
    "src/mir/builder/recursive_child_lowering.rs",
    "src/mir/builder/program_root_lowering.rs",
    "src/mir/builder/raw_expression_dispatch/nonmain_static_box_lifecycle.rs",
    "src/mir/builder/instance_box_declaration_lifecycle.rs",
    "src/mir/builder/runtime_box_fate_tests.rs",
    "src/mir/builder/module_lowering_invocation_reentrant_tests.rs",
):
    lines = (root / rel).read_text().splitlines()
    if len(lines) >= 800:
        raise SystemExit(f"line budget hard stop reached: {rel}={len(lines)}")
if len((root / "src/mir/builder/module_lowering_invocation_reentrant_tests.rs").read_text().splitlines()) >= 760:
    raise SystemExit("parent reentrant test file remains at/over split threshold")

print(f"[{guard_id}] structural I0 guard passed")
PY
