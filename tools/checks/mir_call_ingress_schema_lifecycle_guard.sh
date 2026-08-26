#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-ingress-schema-lifecycle-guard"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
WORKSTREAM="$ROOT_DIR/docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md"
MANIFEST="$ROOT_DIR/tools/checks/guard_rows.toml"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

fail() {
  echo "[$TAG] result_class=current-change failure status=fail: $*" >&2
  exit 1
}

[[ $# -le 1 ]] || fail "usage: $0 [wpre_readiness]"
PHASE="${1:-wpre_readiness}"
case "$PHASE" in
  wpre_readiness) ;;
  wpre_i0|typed_global_b1|r7_closeout)
    fail "recognized future phase is not landed: $PHASE"
    ;;
  *)
    fail "unknown phase: $PHASE"
    ;;
esac

for file in "$CARD" "$STATE" "$WORKSTREAM" "$MANIFEST" "$INDEX"; do
  [[ -f "$file" ]] || fail "required owner missing: ${file#$ROOT_DIR/}"
done

python3 - "$ROOT_DIR" "$CARD" "$STATE" "$WORKSTREAM" "$MANIFEST" "$INDEX" "$PHASE" <<'PY'
from pathlib import Path
import sys
import tomllib

root, card_path, state_path, workstream_path, manifest_path, index_path = map(
    Path, sys.argv[1:7]
)
phase = sys.argv[7]


def fail(message: str) -> None:
    raise SystemExit(message)


with card_path.open("rb") as stream:
    card = tomllib.load(stream)
with state_path.open("rb") as stream:
    state = tomllib.load(stream)
with manifest_path.open("rb") as stream:
    manifest = tomllib.load(stream)
workstream = workstream_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

guard_id = "mir-call-ingress-schema-lifecycle-guard"
guard_script = "tools/checks/mir_call_ingress_schema_lifecycle_guard.sh"
execution_row = "MIR-CALL-INGRESS-SCHEMA-LIFECYCLE-GUARD-S0"

rows = manifest.get("rows")
if not isinstance(rows, list):
    fail("guard_rows.toml rows table is missing")
matches = [row for row in rows if isinstance(row, dict) and row.get("id") == guard_id]
if len(matches) != 1:
    fail(f"expected one registry row for {guard_id}, found {len(matches)}")
row = matches[0]
if row.get("profiles") != ["pilot", "quick-static"]:
    fail("ingress lifecycle guard profiles drifted")
if row.get("cmd") != ["bash", guard_script]:
    fail("ingress lifecycle guard command drifted")
if sum(1 for item in rows if isinstance(item, dict) and item.get("id") == guard_id) != 1:
    fail("ingress lifecycle guard id is duplicated")

if state.get("work_mode") not in {"fast", "design_stop", "closeout"}:
    fail("guard-only row requires CURRENT_STATE work_mode=fast, design_stop, or closeout")
if state.get("latest_card_path") != str(card_path.relative_to(root)):
    fail("CURRENT_STATE latest_card_path no longer names the active card")
if state.get("implementation_permission") is True:
    fail("CURRENT_STATE must not expose semantic implementation permission")

guard_row = card.get("ingress_schema_guard_row")
if not isinstance(guard_row, dict):
    fail("active card ingress_schema_guard_row is missing")
if guard_row.get("execution_row") != execution_row:
    fail("active card guard execution row drifted")
if guard_row.get("phase") != phase:
    fail(f"active card phase is not {phase}")
if guard_row.get("status") not in {"selected_fast_guard_only", "landed_guard_only"}:
    fail("active card guard-only status drifted")
if card.get("implementation_permission") is not False:
    fail("semantic implementation permission opened during guard-only row")
allowed_files = set(guard_row.get("allowed_files") or [])
expected_files = {
    guard_script,
    "tools/checks/guard_rows.toml",
    "docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml",
    "docs/development/current/main/CURRENT_STATE.toml",
    "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    "docs/tools/check-scripts-index.md",
}
if allowed_files != expected_files:
    fail("guard-only allowed file boundary drifted")
if guard_row.get("change", "").find("reusable fail-closed guard") < 0:
    fail("active card no longer describes the reusable fail-closed guard")
if "No parser" not in guard_row.get("contract", ""):
    fail("active card guard contract permits parser changes")

proof_plan = card.get("proof_plan")
if not isinstance(proof_plan, dict) or guard_id not in str(proof_plan.get("future_guard_rows", "")):
    fail("active card proof plan does not name the ingress guard")
if guard_script not in index:
    fail("check-script index does not list the ingress lifecycle guard")
if guard_id not in workstream:
    fail("active workstream does not name the ingress guard row")

# Wpre is a boundary census, not a parser implementation.  The inventory is
# deliberately explicit: removing one edge during this guard-only row is a
# current-change failure, while the later Wpre row will change this contract.
edge_markers = {
    "src/main.rs": ("core_executor::execute_json_artifact",),
    "src/runner/pipe_io.rs": ("try_run_json_v0_pipe", "execute_json_artifact"),
    "src/runner/mod.rs": ("execute_mir_json_text", "try_run_json_v0_pipe"),
    "src/runner/dispatch.rs": ("try_parse_v1_to_module", "parse_mir_v0_to_module", "text.contains"),
    "src/runner/core_executor.rs": (
        "maybe_try_core_direct_for_mir_json",
        "parse_direct_mir_json_text_with_v0_fallback",
        "falling back to VM interpreter",
    ),
    "src/runner/json_artifact/mir_loader.rs": (
        "try_parse_v1_to_module",
        "parse_direct_mir_json_text_with_v0_fallback",
        "parse_mir_v0_to_module",
    ),
    "src/runner/json_artifact/mod.rs": (
        "canonicalize_module_json",
        "load_mir_json_to_module",
        "load_program_json_v0_to_module",
    ),
    "src/runner/modes/common_util/core_bridge.rs": (
        "canonicalize_module_json",
        "methodize_calls",
    ),
    "src/runner/json_v1_bridge/parse/mod.rs": ("try_parse_v1_to_module",),
    "src/runner/mir_json_v0.rs": ("parse_mir_v0_to_module",),
    "src/runner/json_artifact/program_json_v0_loader.rs": (
        "load_program_json_v0_to_module",
        "maybe_merge_program_json_v0_imports",
    ),
    "src/runner/modes/common_util/selfhost/json.rs": (
        "parse_mir_json_v0_line",
        "parse_json_v0_line",
    ),
    "src/runner/modes/common_util/selfhost/stage_a_route.rs": (
        "run_captured_json_v0_command",
        "parse_mir_json_v0_line",
    ),
    "src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs": (
        "parse_mir_json_v0_line",
        "parse_json_v0_line",
        "enforce_stage_a_rust_json_bridge_guard_or_exit",
    ),
    "src/runner/reference/vm_hako/payload_normalize.rs": (
        "normalize_instruction_aliases_in_root",
        "normalize_global_mir_calls",
    ),
    "src/host_providers/mir_builder/backend_shape.rs": (
        "normalize_console_print_externcall",
        "nyash.console.log",
        '"type": "Global"',
    ),
}
for relative, markers in edge_markers.items():
    path = root / relative
    if not path.is_file():
        fail(f"Wpre entrance owner missing: {relative}")
    text = path.read_text(encoding="utf-8")
    missing = [marker for marker in markers if marker not in text]
    if missing:
        fail(f"raw selector/retry edge drifted in {relative}: {', '.join(missing)}")

# A guard-only readiness phase must not accidentally become a partial schema
# implementation.  Scan compiled Rust/inc surfaces, but not docs or this
# guard, so the check is about executable authority rather than prose.
source_files = [
    path
    for tree in (root / "src", root / "crates")
    if tree.is_dir()
    for path in tree.rglob("*")
    if path.suffix in {".rs", ".inc"} and path.is_file()
]
source_text = "\n".join(path.read_text(encoding="utf-8") for path in source_files)
for marker in (
    "CanonicalGlobalTargetV1",
    "CanonicalSameModuleGlobalTargetV1",
    "CanonicalBuiltinGlobalV1",
    "GlobalTargetV2",
    'schema_version: "2.0"',
    'schema_version = "2.0"',
    '"schema_version":"2.0"',
    '"schema_version": "2.0"',
):
    if marker in source_text:
        fail(f"partial v2/typed-Global surface appeared before Wpre: {marker}")

print(
    f"[{guard_id}] result_class=current-change failure status=pass "
    f"phase={phase} raw_selector_retry_edges=frozen typed_v2_surface=absent"
)
PY
