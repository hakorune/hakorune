#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-text-scan-contract"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CONTRACT="$ROOT_DIR/src/box_callable/text_scan.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$CONTRACT"

lines="$(wc -l < "$CONTRACT" | tr -d '[:space:]')"
if (( lines >= 800 )); then
  guard_fail "$TAG" "TextScan contract reached the hard 800-line boundary: $lines"
fi

for required in \
  'HAKO_TEXT_SCAN_CONTRACT_ID_V1: &str = "hako.text.scan@1"' \
  'HAKO_TEXT_SCAN_PROFILE_V1: &str = "utf8-codepoint-clamped-v1"' \
  'TextSliceRange' \
  'TextFindNeedle' \
  'immediate_i64_no_lease' \
  'TextScanProviderAdmissionV1' \
  'complete_two_role_contract_admits_one_provider_profile' \
  'contract_rejects_partial_role_or_alias_set' \
  'contract_rejects_foreign_requirement'; do
  guard_expect_fixed_in_file "$TAG" "$required" "$CONTRACT" \
    "TextScan contract evidence is missing: $required"
done

for forbidden in \
  'index_mode_from_env' \
  'StringBox::invoke_surface' \
  'nyash.integer.get_h' \
  'runtime registry' \
  'lower_method_call' \
  'fallback' \
  'retry'; do
  if rg -n -i -F -- "$forbidden" "$CONTRACT"; then
    guard_fail "$TAG" "TextScan contract contains compatibility/runtime authority: $forbidden"
  fi
done

# The contract is cold until the complete AOT activation cell is available.
if rg -n --glob '*.rs' \
  'TextScanProviderAdmissionV1::admit|AdmittedTextScanProviderV1' \
  "$ROOT_DIR/src" --glob '!src/box_callable/text_scan.rs' --glob '!src/box_callable/mod.rs'; then
  guard_fail "$TAG" "TextScan contract has an unscoped production consumer before activation"
fi

echo "[$TAG] ok"
