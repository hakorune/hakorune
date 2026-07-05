#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-v0-scanner-aot-blocker-inventory-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_files "$TAG" "$SCANNER_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-programjson-scanner-aot-inventory.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

BLOCKED_COUNT=0

bash "$HAKO_BIN" --backend mir --verify "$SCANNER_IMPL" >/dev/null

make_probe() {
  local name="$1"
  local body="$2"
  local app="$TMP_DIR/${name}.hako"
  cat >"$app" <<EOF
using lang.compiler.mirbuilder.program_json_v0_scanner_box as ProgramJsonV0ScannerBox

static box Main {
  main() {
${body}
    return 0
  }
}
EOF
  printf '%s\n' "$app"
}

run_probe() {
  local name="$1"
  local body="$2"
  local expected_output="$3"
  local expected_symbol="$4"
  local app
  app="$(make_probe "$name" "$body")"
  local exe="$TMP_DIR/${name}.exe"
  local log="$TMP_DIR/${name}.log"
  local run_log="$TMP_DIR/${name}.run.log"

  if ! bash "$HAKO_BIN" --backend mir --verify "$app" >"$TMP_DIR/${name}.verify.log" 2>&1; then
    guard_fail "$TAG" "MIR verify failed for probe=$name"
  fi

  set +e
  NYASH_LLVM_ROUTE_TRACE=1 bash "$HAKO_BIN" --backend mir --emit-exe "$exe" "$app" >"$log" 2>&1
  local rc=$?
  set -e

  if [ "$rc" -eq 0 ]; then
    if ! "$exe" >"$run_log" 2>&1; then
      tail -n 80 "$run_log" || true
      guard_fail "$TAG" "AOT runtime failed for probe=$name"
    fi
    local actual
    actual="$(grep -v '^Result:' "$run_log" | sed '/^$/d' | tail -n 1 || true)"
    if [ "$actual" != "$expected_output" ]; then
      printf 'expected: %s\nactual: %s\n' "$expected_output" "$actual" >&2
      tail -n 80 "$run_log" || true
      guard_fail "$TAG" "AOT runtime output mismatch for probe=$name"
    fi
    echo "probe=$name mir_verify=green aot_emit=green aot_run=green blocker_symbol=-"
    return 0
  fi

  if ! grep -Fq "unsupported pure shape for current backend recipe" "$log"; then
    tail -n 80 "$log" || true
    guard_fail "$TAG" "unexpected AOT failure for probe=$name"
  fi
  if ! grep -Fq "callee_symbol=${expected_symbol}" "$log"; then
    tail -n 120 "$log" || true
    guard_fail "$TAG" "AOT blocker did not match expected symbol for probe=$name"
  fi
  if ! grep -Fq "reason=module_generic_prepass_failed" "$log"; then
    tail -n 120 "$log" || true
    guard_fail "$TAG" "AOT blocker reason did not match for probe=$name"
  fi

  BLOCKED_COUNT=$((BLOCKED_COUNT + 1))
  echo "probe=$name mir_verify=green aot_emit=blocked reason=module_generic_prepass_failed blocker_symbol=${expected_symbol}"
}

run_probe \
  "read_char" \
  '    local s = "{\"body\":[]}"
    local p = ProgramJsonV0ScannerBox._read_char(s, 0)
    print(p)' \
  '{' \
  'ProgramJsonV0ScannerBox._read_char/2'

run_probe \
  "read_char_nonzero_cursor" \
  '    local s = "{\"body\":[]}"
    local p = ProgramJsonV0ScannerBox._read_char(s, 8)
    print(p)' \
  '[' \
  'ProgramJsonV0ScannerBox._read_char/2'

run_probe \
  "seek_after" \
  '    local s = "{\"body\":[]}"
    local p = ProgramJsonV0ScannerBox.seek_after(s, "\"body\":", 0)
    print("" + p)' \
  '8' \
  'StringHelpers.to_i64/1'

run_probe \
  "seek_obj_end" \
  '    local s = "{\"body\":[]}"
    local p = ProgramJsonV0ScannerBox.seek_obj_end_unescaped(s, 0)
    print("" + p)' \
  '11' \
  'ProgramJsonV0ScannerBox.seek_obj_end_unescaped/2'

run_probe \
  "seek_obj_field_value_start" \
  '    local s = "{\"body\":[]}"
    local p = ProgramJsonV0ScannerBox.seek_obj_field_value_start(s, "body", 0)
    print("" + p)' \
  '8' \
  'ProgramJsonV0ScannerBox.seek_obj_field_value_start/3'

run_probe \
  "seek_obj_field_obj_start" \
  '    local s = "{\"cond\":{\"type\":\"Compare\"}}"
    local p = ProgramJsonV0ScannerBox.seek_obj_field_obj_start(s, "cond", 0)
    print("" + p)' \
  '8' \
  'ProgramJsonV0ScannerBox.seek_obj_field_obj_start/3'

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-v0-scanner-aot-blocker-inventory-guard-v0
owner=ProgramJsonV0ScannerBox
inventory_status=green
mir_verify_status=green
source_selfhost_claim=0
programjson_snapshot_parity_claim=0
backend_lowering_migration=0
REPORT
if [ "$BLOCKED_COUNT" -eq 0 ]; then
  cat <<'REPORT'
aot_execution_status=green
reason=none
summary=ok
REPORT
else
  cat <<REPORT
aot_execution_status=blocked
reason=module_generic_prepass_failed
blocked_probe_count=$BLOCKED_COUNT
summary=ok
REPORT
fi
