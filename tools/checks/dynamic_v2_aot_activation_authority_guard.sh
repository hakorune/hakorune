#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-aot-activation-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SOURCE="$ROOT_DIR/lang/src/runtime/meta/provider_slot_contract_box.hako"
MODULE="$ROOT_DIR/lang/src/runtime/meta/hako_module.toml"
CODEGEN="$ROOT_DIR/tools/provider_slot_contract_manifest_codegen.py"
MANIFEST="$ROOT_DIR/lang/src/runtime/meta/generated/provider_slot_contract_manifest.json"
HEADER="$ROOT_DIR/include/nyrt_dynamic_text_scan_v1.h"
RUST="$ROOT_DIR/src/abi/text_scan_aot_export_facts.rs"
PYTHON="$ROOT_DIR/src/llvm_py/builders/dynamic_v2_text_scan_export_facts.py"
CODEGEN_TEST="$ROOT_DIR/tools/checks/lib/provider_slot_contract_codegen_tests.py"
PROJECTION_TEST="$ROOT_DIR/tools/checks/lib/text_scan_export_projection_tests.py"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$SOURCE" "$MODULE" "$CODEGEN" "$MANIFEST" "$HEADER" "$RUST" "$PYTHON" "$CODEGEN_TEST" "$PROJECTION_TEST"

python3 "$CODEGEN_TEST"
python3 "$CODEGEN" --check
python3 "$PROJECTION_TEST"

for file in "$SOURCE" "$CODEGEN" "$MANIFEST" "$HEADER" "$RUST" "$PYTHON"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "I0-B artifact reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

guard_expect_fixed_in_file "$TAG" '"role_count": 2' "$MANIFEST" "TextScan contract must have exactly two roles"
guard_expect_fixed_in_file "$TAG" 'TextScanProviderSlotContract = "provider_slot_contract_box.hako"' "$MODULE" "Hako module must expose the TextScan contract source"
guard_expect_fixed_in_file "$TAG" 'StringSubstring' "$MANIFEST" "substring CoreMethod identity is missing"
guard_expect_fixed_in_file "$TAG" 'StringIndexOf' "$MANIFEST" "indexOf CoreMethod identity is missing"
guard_expect_fixed_in_file "$TAG" 'HAKO_TEXT_SCAN_ENTRY_COUNT UINT32_C(2)' "$HEADER" "neutral export must declare two entries"
guard_expect_fixed_in_file "$TAG" 'TextScanAotEntryIdV1' "$RUST" "Rust symbolic entry projection is missing"
guard_expect_fixed_in_file "$TAG" 'EXPORT_FACTS = (' "$PYTHON" "Python symbolic export projection is missing"

for pair in \
  "Substring = 1" "IndexOf = 2" \
  "HostHandle = 1" "ImmediateI64 = 2" \
  "None = 0" "EndAuthorized = 1" \
  'TEXT_SCAN_SYMBOL_SUBSTRING_V1' \
  'TEXT_SCAN_SYMBOL_INDEX_OF_V1' \
  'receiver_lane: TextScanValueLaneV1::HostHandle'; do
  guard_expect_fixed_in_file "$TAG" "$pair" "$RUST" "Rust export projection drifted: $pair"
done

if rg -n 'row\.set\("(result_kind|effect)"' "$SOURCE" || \
   rg -n '"(result_kind|effect)"[[:space:]]*:' "$MANIFEST"; then
  guard_fail "$TAG" "TextScan artifact must not reissue CoreMethod result/effect"
fi

# I0-B is intentionally closed before provider/runtime/LLVM production use.
for root in "$ROOT_DIR/src/mir" "$ROOT_DIR/src/llvm_py/instructions" "$ROOT_DIR/crates/nyash_kernel" "$ROOT_DIR/src/backend/mir_interpreter" "$ROOT_DIR/src/tests"; do
  if [[ -d "$root" ]] && rg -n \
    --glob '*.rs' --glob '*.py' --glob '*.hako' \
    --glob '!**/tests.rs' --glob '!**/*_tests.rs' --glob '!**/tests/**' \
    'text_scan_aot_export_facts|TextScanAotExportFactV1|hako\.text\.scan\.(substring|index_of)\.v1' \
    "$root"; then
    guard_fail "$TAG" "I0-B symbolic export has an early production/runtime/VM caller: ${root#"$ROOT_DIR/"}"
  fi
done

if rg -n '^[[:space:]]*(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?(struct|enum|fn|const)[[:space:]].*(ProviderAdmissionSeal|RuntimeExecutablePlan|BoxCallableRegistry|lower_method_call|DynamicV2PhysicalEmissionSession)' "$SOURCE" "$CODEGEN" "$HEADER" "$RUST" "$PYTHON"; then
  guard_fail "$TAG" "I0-B artifact illegally opens provider/session/runtime authority"
fi

echo "[$TAG] ok"
