#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="core-method-contract-manifest-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" \
  "$ROOT_DIR/tools/core_method_contract_manifest_codegen.py" \
  "$ROOT_DIR/tools/checks/lib/core_method_contract_codegen_tests.py" \
  "$ROOT_DIR/lang/src/runtime/meta/core_method_contract_box.hako" \
  "$ROOT_DIR/lang/src/runtime/meta/generated/core_method_contract_manifest.json" \
  "$ROOT_DIR/src/mir/generated/core_method_contract_rows.rs"

python3 "$ROOT_DIR/tools/checks/lib/core_method_contract_codegen_tests.py"
python3 "$ROOT_DIR/tools/core_method_contract_manifest_codegen.py" --check

core_lookup_consumers="$({
  rg -l 'lookup_core_method_result_row_v1' "$ROOT_DIR/src/mir" \
    --glob '*.rs' --glob '!core_method_result_kind.rs' || true
} | sort -u)"
if [[ -z "$core_lookup_consumers" ]]; then
  guard_fail "$TAG" "S0b callable-result Core lookup consumer is missing"
fi
while IFS= read -r path; do
  [[ -z "$path" ]] && continue
  case "$path" in
    "$ROOT_DIR/src/mir/callable_result_representation/"*.rs) ;;
    *) guard_fail "$TAG" "unexpected Core result-kind lookup consumer: $path" ;;
  esac
done <<< "$core_lookup_consumers"

core_lookup_call_count="$({
  rg -o 'lookup_core_method_result_row_v1\(' \
    "$ROOT_DIR/src/mir/callable_result_representation" --glob '*.rs' \
      --glob '!**/tests/**' --glob '!*_tests.rs' || true
} | wc -l)"
if [[ "$core_lookup_call_count" -ne 1 ]]; then
  guard_fail "$TAG" \
    "expected exactly one disconnected callable-result Core lookup consumer, got $core_lookup_call_count"
fi

runtime_json_reads="$(
  rg -l 'core_method_contract_manifest\.json' "$ROOT_DIR/src" --glob '*.rs' || true
)"
while IFS= read -r path; do
  [[ -z "$path" ]] && continue
  case "$path" in
    "$ROOT_DIR/src/mir/core_method_op.rs"|"$ROOT_DIR/src/mir/core_method_result_kind.rs") ;;
    *) guard_fail "$TAG" "unexpected runtime manifest reader: $path" ;;
  esac
done <<< "$runtime_json_reads"
