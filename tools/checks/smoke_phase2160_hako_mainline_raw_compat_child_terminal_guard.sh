#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="smoke-phase2160-hako-mainline-raw-compat-child-terminal"
CARD="$ROOT_DIR/docs/development/current/main/investigations/smoke-phase2160-hako-mainline-raw-compat-child-terminal-i1-2026-08-27.toml"
REGISTRY="$ROOT_DIR/tools/checks/guard_rows.toml"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

for path in "$CARD" "$REGISTRY" \
  "$ROOT_DIR/src/mir/builder/raw_compatibility_child_terminal.rs" \
  "$ROOT_DIR/src/mir/builder/raw_compat_child_terminal_tests.rs" \
  "$ROOT_DIR/src/mir/builder/program_root_work_plan/raw_compatibility.rs" \
  "$ROOT_DIR/src/mir/builder/program_root_work_plan.rs" \
  "$ROOT_DIR/src/mir/builder/program_root_lowering.rs" \
  "$ROOT_DIR/src/mir/builder/decls.rs" \
  "$ROOT_DIR/src/mir/builder/instance_box_constructor_batch.rs" \
  "$ROOT_DIR/src/mir/builder/instance_box_declaration_lifecycle.rs" \
  "$ROOT_DIR/src/mir/builder/nonmain_static_box_method_batch.rs" \
  "$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs" \
  "$ROOT_DIR/src/mir/builder.rs"; do
  [[ -e "$path" ]] || fail "missing owner ${path#$ROOT_DIR/}"
done

python3 - "$ROOT_DIR" "$CARD" "$REGISTRY" <<'PY'
from pathlib import Path
import sys
import tomllib

root, card_path, registry_path = map(Path, sys.argv[1:])
card = tomllib.loads(card_path.read_text())
registry = tomllib.loads(registry_path.read_text())

if card.get("status") != "in_progress":
    raise SystemExit("I1 card is not the active in-progress row")
if card.get("implementation_permission") is not True:
    raise SystemExit("I1 implementation permission is not open")
if card.get("selected_fast") is not True:
    raise SystemExit("I1 is not marked as the selected fast row")

guard_id = "smoke-phase2160-hako-mainline-raw-compat-child-terminal"
guard_script = "tools/checks/smoke_phase2160_hako_mainline_raw_compat_child_terminal_guard.sh"
rows = registry.get("rows")
if not isinstance(rows, list):
    raise SystemExit("guard registry rows are missing")
matches = [row for row in rows if row.get("id") == guard_id]
if len(matches) != 1:
    raise SystemExit(f"expected exactly one registry row for {guard_id}, found {len(matches)}")
row = matches[0]
if row.get("profiles") != ["pilot", "quick-static"]:
    raise SystemExit("I1 guard profiles drifted")
if row.get("cmd") != ["bash", guard_script]:
    raise SystemExit("I1 guard command drifted")

def read(rel: str) -> str:
    return (root / rel).read_text()

terminal = read("src/mir/builder/raw_compatibility_child_terminal.rs")
workplan_child = read("src/mir/builder/program_root_work_plan/raw_compatibility.rs")
workplan = read("src/mir/builder/program_root_work_plan.rs")
lowering = read("src/mir/builder/program_root_lowering.rs")
decls = read("src/mir/builder/decls.rs")
constructors = read("src/mir/builder/instance_box_constructor_batch.rs")
lifecycle = read("src/mir/builder/instance_box_declaration_lifecycle.rs")
methods = read("src/mir/builder/nonmain_static_box_method_batch.rs")
root_lifecycle = read("src/mir/builder/normal_default_root_catalog_lifecycle.rs")
tests = read("src/mir/builder/raw_compat_child_terminal_tests.rs")
builder = read("src/mir/builder.rs")

if terminal.count("trait RawCompatibilityChildTerminalPortV1") != 1:
    raise SystemExit("dedicated RawCompatibility terminal trait is not unique")
if terminal.count("impl RawCompatibilityChildTerminalPortV1 for RawInvocationChildPortV1") != 1:
    raise SystemExit("RawInvocation production terminal implementation is not unique")
for method in (
    "lower_raw_compat_static_child",
    "lower_raw_compat_instance_child",
    "lower_raw_compat_app_main_static_child",
    "lower_raw_compat_main_materialization",
):
    if terminal.count(f"fn {method}(") != 2:
        raise SystemExit(f"terminal method declaration/implementation drifted: {method}")
if "struct RawCompatibilityCallableShapeV1" not in terminal:
    raise SystemExit("private compatibility shape disappeared")
shape_start = terminal.index("struct RawCompatibilityCallableShapeV1")
shape_end = terminal.index("impl RawCompatibilityCallableShapeV1", shape_start)
shape = terminal[shape_start:shape_end]
if "Box<str>" not in shape or "physical_arity: usize" not in shape:
    raise SystemExit("shape no longer owns symbol and physical arity")
if "Clone" in shape or "Copy" in shape:
    raise SystemExit("compatibility shape became copyable")
for token in (
    "capture_static_box_method_pending_v1(",
    "capture_normalized_instance_box_method_pending_v1(",
    "commit_legacy_nested_box_method_symbol_pending_v1(",
    "RawInvocationRootLineageV1::ScriptRoot",
    "unexpected root",
    "source-unlocated",
    "source-missing",
):
    if token not in terminal:
        raise SystemExit(f"terminal source/capture contract disappeared: {token}")
if any(token in terminal for token in (
    "RawLegacyChildLoweringPortV1",
    "raw_expression_dispatch",
    "instance_box_method_batch",
    "CalleeResolver",
    "CallTarget",
    "MirInstruction",
)):
    raise SystemExit("raw compatibility terminal entered an excluded runtime/target owner")

if workplan.count("ProgramRootWorkPlanAdmissionV1::RawCompatibility") < 2:
    raise SystemExit("explicit RawCompatibility admission branches disappeared")
if "admission: ProgramRootWorkPlanAdmissionV1" not in workplan:
    raise SystemExit("work-plan admission is not carried explicitly")
for token in (
    "lower_raw_compat_with_port_v1",
    "RawCompatibility(parts)",
    "raw-compat-admission",
):
    if token not in workplan_child:
        raise SystemExit(f"raw family work-plan dispatch disappeared: {token}")
if "lower_static_box_method(" in workplan_child or "lower_instance_box_method(" in workplan_child:
    raise SystemExit("raw family work-plan retained a loose success terminal")
if "params.len() + 1" not in constructors:
    raise SystemExit("instance constructor physical arity is not issued before the terminal")

for token in (
    "immediate.lower_raw_compat_with_port_v1",
    "deferred.lower_raw_compat_with_port_v1",
    "PreparedProgramRootRuntimeWorkV1::RawCompatibility",
    "build_verified_static_main_box_raw_compat_with_port_v1",
    "prepare_normal_collector_drain",
):
    if token not in lowering:
        raise SystemExit(f"root raw terminal/collector structure disappeared: {token}")
if lowering.count("prepare_normal_collector_drain") != 1:
    raise SystemExit("outer collector drain is no longer a single terminal")
if "RawEntryMaterializationSourceReceiptV1" not in root_lifecycle:
    raise SystemExit("raw materialization receipt is not sealed at compatibility ingress")
if root_lifecycle.count("NormalCallableSemanticPackageMode::Compatibility(") != 1:
    raise SystemExit("compatibility mode handoff is not explicit and unique")
if "lower_raw_compat_app_main_static_child" not in decls or "lower_raw_compat_main_materialization" not in decls:
    raise SystemExit("App Main child/materialization terminal handoff disappeared")
if "lower_cataloged_static_box_method" in decls:
    raw_helper_start = decls.index("build_verified_static_main_box_raw_compat_with_port_v1")
    raw_helper_end = decls.index("fn lower_verified_static_main_root_with_port_v1", raw_helper_start)
    if "lower_cataloged_static_box_method" in decls[raw_helper_start:raw_helper_end]:
        raise SystemExit("raw App Main helper entered cataloged publication")
if "lower_raw_compat_with_port_v1" not in lifecycle or "lower_raw_compat_with_port_v1" not in methods:
    raise SystemExit("instance/deferred raw family owner handoff disappeared")

for name in (
    "raw_compatibility_root_families_use_the_collector_terminal",
    "raw_compatibility_source_context_is_script_root_only",
):
    if f"fn {name}()" not in tests:
        raise SystemExit(f"focused I1 evidence is missing: {name}")
if "raw_compat_child_terminal_tests" not in builder:
    raise SystemExit("focused test sibling is not registered")

for rel in (
    "src/mir/builder.rs",
    "src/mir/builder/program_root_work_plan.rs",
    "src/mir/builder/raw_compatibility_child_terminal.rs",
    "src/mir/builder/raw_compat_child_terminal_tests.rs",
    "src/mir/builder/program_root_work_plan/raw_compatibility.rs",
    "src/mir/builder/program_root_lowering.rs",
    "src/mir/builder/decls.rs",
    "src/mir/builder/instance_box_constructor_batch.rs",
    "src/mir/builder/instance_box_declaration_lifecycle.rs",
    "src/mir/builder/nonmain_static_box_method_batch.rs",
    "src/mir/builder/normal_default_root_catalog_lifecycle.rs",
):
    lines = (root / rel).read_text().splitlines()
    if len(lines) >= 800:
        raise SystemExit(f"line budget hard stop reached: {rel}={len(lines)}")

print(f"[{guard_id}] structural I1 guard passed")
PY
