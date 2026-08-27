#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rawlegacy-nested-main-retire"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/investigations/runtime-box-rawlegacy-nested-main-retire-i0-2026-08-27.toml"
ORIGIN_CARD="$ROOT_DIR/docs/development/current/main/investigations/runtime-box-rawlegacy-nested-main-fate-d0-2026-08-27.toml"
REGISTRY="$ROOT_DIR/tools/checks/guard_rows.toml"

guard_require_command "$TAG" python3
guard_require_files "$TAG" \
  "$CARD" \
  "$ORIGIN_CARD" \
  "$REGISTRY" \
  "$ROOT_DIR/src/mir/builder/recursive_child_lowering.rs" \
  "$ROOT_DIR/src/mir/builder/recursive_child_lowering/legacy_port.rs" \
  "$ROOT_DIR/src/mir/builder/raw_expression_dispatch/mod.rs" \
  "$ROOT_DIR/src/mir/builder/raw_expression_dispatch/tests.rs" \
  "$ROOT_DIR/src/mir/builder/recursive_child_lowering_rawport_tests.rs"

python3 - "$ROOT_DIR" "$CARD" "$ORIGIN_CARD" "$REGISTRY" <<'PY'
from pathlib import Path
import sys
import tomllib

root, card_path, origin_path, registry_path = map(Path, sys.argv[1:])
card = tomllib.loads(card_path.read_text())
origin = tomllib.loads(origin_path.read_text())
registry = tomllib.loads(registry_path.read_text())

def fail(message: str) -> None:
    raise SystemExit(f"[rawlegacy-nested-main-retire] ERROR: {message}")

rows = registry.get("rows")
matches = [row for row in rows or [] if row.get("id") == "rawlegacy-nested-main-retire"]
if len(matches) != 1:
    fail(f"guard registry row must be unique, found {len(matches)}")
if matches[0].get("profiles") != ["pilot", "quick-static"]:
    fail("guard profiles drifted")
if matches[0].get("cmd") != ["bash", "tools/checks/rawlegacy_nested_main_retire_guard.sh"]:
    fail("guard command drifted")

census = origin.get("census", {})
if census.get("production_direct_issuer_count") != 20:
    fail("D0a production issuer count drifted")
if census.get("production_root_main_direct_issuer_count") != 0:
    fail("D0a direct Root Main issuer count must remain zero")
issuers = census.get("issuer_classes")
if not isinstance(issuers, list) or len(issuers) != 20:
    fail("D0a issuer_classes must contain exactly 20 entries")
if any(not entry.startswith("LegacyMainOriginUnknown:") for entry in issuers):
    fail("D0a issuer classes must remain explicitly source-less/unknown")

def read(relative: str) -> str:
    return (root / relative).read_text()

port = read("src/mir/builder/recursive_child_lowering.rs")
legacy = read("src/mir/builder/recursive_child_lowering/legacy_port.rs")
dispatch = read("src/mir/builder/raw_expression_dispatch/mod.rs")
tests = read("src/mir/builder/raw_expression_dispatch/tests.rs")
raw_tests = read("src/mir/builder/recursive_child_lowering_rawport_tests.rs")

if port.count("enum RawNestedMainFateV1") != 1:
    fail("nested Main fate enum is not unique")
if port.count("ContinueExistingTerminal") != 2 or port.count("RetireRawLegacyBeforeEffects") != 1:
    fail("nested Main fate variants/default are not exact")
if port.count("fn nested_main_fate_v1(&mut self)") != 1:
    fail("default nested Main fate method is not unique")
if legacy.count("fn nested_main_fate_v1(&mut self)") != 1:
    fail("RawLegacy nested Main override is not unique")
if legacy.count("RawNestedMainFateV1::RetireRawLegacyBeforeEffects") != 1:
    fail("RawLegacy is not the sole retirement override")

marker = 'if is_static && name == "Main" {'
start = dispatch.find(marker)
end = dispatch.find("} else if is_static {", start)
if start < 0 or end < 0:
    fail("static Main dispatcher arm disappeared")
arm = dispatch[start:end]
if arm.count("port.nested_main_fate_v1()") != 1:
    fail("static Main dispatcher must query fate exactly once")
if arm.count("port.lower_static_main_box(") != 1:
    fail("static Main dispatcher must retain one existing terminal")
if arm.index("port.nested_main_fate_v1()") > arm.index("port.lower_static_main_box("):
    fail("nested Main fate must be queried before helper/prepare terminal")
if arm.count("RawNestedMainFateV1::RetireRawLegacyBeforeEffects") != 1:
    fail("static Main dispatcher retirement arm disappeared")
if dispatch.count("port.nested_main_fate_v1()") != 1:
    fail("nested Main fate query leaked into another dispatcher arm")

for excluded in (
    "src/mir/builder/program_root_lowering.rs",
    "src/mir/builder/program_root_work_plan.rs",
    "src/mir/builder/raw_compatibility_child_terminal.rs",
    "src/mir/builder/normal_script_runtime_work.rs",
):
    if "nested_main_fate_v1" in read(excluded):
        fail(f"nested Main fate leaked into excluded owner: {excluded}")

for name in (
    "raw_legacy_nested_static_main_retires_before_effects",
    "raw_legacy_nested_static_main_retires_from_body_statement_and_expression_facades",
    "raw_legacy_explicit_root_compatibility_keeps_existing_order",
):
    if tests.count(f"fn {name}(") != 1:
        fail(f"required RawLegacy test is missing or duplicated: {name}")
if raw_tests.count("fn raw_invocation_nested_main_keeps_existing_root_only_terminal(") != 1:
    fail("required RawInvocation parity test is missing or duplicated")
if tests.count("[freeze:contract][raw-legacy/nested-main-retired]") < 1:
    fail("typed retirement error tag is not asserted")

for relative in (
    "src/mir/builder/recursive_child_lowering.rs",
    "src/mir/builder/recursive_child_lowering/legacy_port.rs",
    "src/mir/builder/raw_expression_dispatch/mod.rs",
    "src/mir/builder/raw_expression_dispatch/tests.rs",
    "src/mir/builder/recursive_child_lowering_rawport_tests.rs",
):
    lines = (root / relative).read_text().count("\n")
    if lines >= 800:
        fail(f"changed source crossed 800-line hard stop: {relative}={lines}")

if card.get("implementation_permission") is not True:
    fail("I0 implementation permission is not open")
print("[rawlegacy-nested-main-retire] ok")
PY
