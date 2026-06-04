#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-vm-exact-numeric-helper-field-mutation"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/vm-helper-exact-numeric-field-mutation-proof/main.hako"
APP_README="apps/vm-helper-exact-numeric-field-mutation-proof/README.md"
ARTIFACT_DIR="$ROOT_DIR/target/checks/$TAG"
TMP_DIR="$ARTIFACT_DIR/tmp"
VM_LOG="$TMP_DIR/vm.log"
MIR_JSON="$TMP_DIR/app.mir.json"
EXE_OUT="$TMP_DIR/app.exe"
BUILD_LOG="$TMP_DIR/build.log"
RUN_LOG="$TMP_DIR/exe.log"

mkdir -p "$TMP_DIR"

echo "[$TAG] checking VM exact-numeric helper field mutation"

guard_require_files "$TAG" "$APP" "$APP_README"

guard_expect_fixed_in_file "$TAG" 'box HelperFieldMutationCounter' "$APP" "fixture owner box missing"
guard_expect_fixed_in_file "$TAG" 'addViaHelper' "$APP" "caller-visible helper route missing"
guard_expect_fixed_in_file "$TAG" 'applyAdd' "$APP" "mutating helper missing"
guard_expect_fixed_in_file "$TAG" 'me.value = me.value + delta' "$APP" "helper must mutate receiver field"
guard_expect_fixed_in_file "$TAG" 'me.write_count = me.write_count + 1' "$APP" "helper must mutate second receiver field"

PURE_FIRST_VM_BIN="${PURE_FIRST_VM_BIN:-debug}" pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$VM_LOG"

proof_output_assert_fixed_lines "$TAG" "$VM_LOG" \
  'vm-helper-exact-numeric-field-mutation-proof' \
  'values=12,23,23,2' \
  'summary=ok'

PURE_FIRST_MIR_EMIT_BIN="${PURE_FIRST_MIR_EMIT_BIN:-debug}" pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$MIR_JSON"

python3 - "$MIR_JSON" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
for name in (
    "main",
    "HelperFieldMutationCounter.birth/1",
    "HelperFieldMutationCounter.addViaHelper/1",
    "HelperFieldMutationCounter.applyAdd/1",
):
    if name not in functions:
        raise SystemExit(f"missing function: {name}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
counter = plans.get("HelperFieldMutationCounter")
if counter is None:
    raise SystemExit("missing typed object plan: HelperFieldMutationCounter")
fields = {field.get("name"): field for field in counter.get("fields", [])}
if fields.get("value", {}).get("declared_type") != "usize":
    raise SystemExit(f"value field must stay usize: {fields.get('value')}")
if fields.get("write_count", {}).get("declared_type") != "i64":
    raise SystemExit(f"write_count field must stay i64: {fields.get('write_count')}")

checked_value_sets = []
for fn in data.get("functions", []):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if (
                inst.get("op") == "field_set"
                and inst.get("field") == "value"
                and inst.get("exact_numeric_runtime_check", {}).get("kind")
                    == "dynamic_integer_range"
                and inst.get("exact_numeric_runtime_check", {}).get("declared_type")
                    == "usize"
            ):
                checked_value_sets.append((fn.get("name"), block.get("id")))
if not checked_value_sets:
    raise SystemExit("missing usize dynamic integer range contract on value field_set")
print("[helper-field-mutation-mir] ok")
PY

pure_first_guard_build_toolchain
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$MIR_JSON" "$EXE_OUT" "$BUILD_LOG"
pure_first_guard_assert_clean_build_log "$TAG" "$BUILD_LOG"
pure_first_guard_run_exe "$TAG" "$EXE_OUT" "$RUN_LOG"

proof_output_assert_fixed_lines "$TAG" "$RUN_LOG" \
  'vm-helper-exact-numeric-field-mutation-proof' \
  'values=12,23,23,2' \
  'summary=ok'

echo "[$TAG] ok"
