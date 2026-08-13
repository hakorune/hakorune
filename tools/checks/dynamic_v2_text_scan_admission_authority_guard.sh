#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-text-scan-admission-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" \
  "$ROOT_DIR/src/box_callable/provider_admission/mod.rs" \
  "$ROOT_DIR/src/box_callable/provider_admission/seal.rs" \
  "$ROOT_DIR/src/box_callable/provider_admission/admitted_registry.rs" \
  "$ROOT_DIR/src/box_callable/provider_admission/aot_admission.rs" \
  "$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_capability.rs" \
  "$ROOT_DIR/docs/development/current/main/investigations/dynamic-fault-exit-transaction-d0-design-task-2026-08-10.md"

BRAND_PORT="$ROOT_DIR/src/mir/builder/module_lowering_invocation.rs"
RAW_CHILD="$ROOT_DIR/src/mir/builder/recursive_child_lowering.rs"
COLLECTOR="$ROOT_DIR/src/mir/builder/module_draft_collector.rs"
EMITTER_TESTS="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/tests.rs"
guard_require_files "$TAG" "$BRAND_PORT" "$RAW_CHILD" "$COLLECTOR" "$EMITTER_TESTS"

ADMISSION_DIR="$ROOT_DIR/src/box_callable/provider_admission"

for file in "$ADMISSION_DIR"/*.rs; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "I0-C admission file reached the hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

if [[ "$(rg -n 'pub\(crate\) struct PreparedAotExecutableAdmissionV1' "$ADMISSION_DIR/aot_admission.rs" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "symbolic AOT admission must have exactly one owner"
fi
if [[ "$(rg -n 'pub\(crate\) struct AdmittedTextScanRegistryV1' "$ADMISSION_DIR/admitted_registry.rs" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "immutable admitted registry must have exactly one owner"
fi
if [[ "$(rg -n 'pub\(crate\) struct ProviderAdmissionSealV1' "$ADMISSION_DIR/seal.rs" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "ProviderAdmissionSeal must have exactly one issuer"
fi
if [[ "$(rg -n 'ProviderAdmissionSealV1::consume_text_scan' "$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_capability.rs" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "selected physical capability must consume the admission exactly once"
fi
if rg -n 'NonZeroU64|registry_generation[[:space:]]*:' \
  "$ADMISSION_DIR/seal.rs" \
  "$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_capability.rs"; then
  guard_fail "$TAG" "admission must not accept an independent raw registry generation"
fi
if [[ "$(rg -n 'AdmittedTextScanRegistryV1::new' "$ADMISSION_DIR/seal.rs" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "the admitted registry must be constructed by the seal exactly once"
fi
if [[ "$(rg -n 'with_invocation_brand<R>' "$BRAND_PORT" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "module port must expose exactly one invocation-brand callback"
fi
if [[ "$(rg -n 'with_invocation_brand<R>' "$RAW_CHILD" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "raw child port must delegate exactly one invocation-brand callback"
fi
if [[ "$(rg -n 'receipt_brand\(\)' "$BRAND_PORT" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "module brand callback must source the collector receipt brand"
fi
if rg -n 'ModuleInvocationBrandV1::(legacy_test|test_with_ordinal)' \
  "$BRAND_PORT" "$RAW_CHILD" "$COLLECTOR"; then
  guard_fail "$TAG" "production brand transport must not mint a test brand"
fi
guard_expect_fixed_in_file "$TAG" ".with_invocation_brand(|brand|" "$EMITTER_TESTS" \
  "the unpublished W6 canary must consume the collector-backed brand callback"
guard_expect_fixed_in_file "$TAG" "capability.aot_admission().plan_stamp(), brand" "$EMITTER_TESTS" \
  "the W6 canary must compare admission PlanStamp with the collector brand"
guard_expect_fixed_in_file "$TAG" "commit_cataloged_box_method_completed(completed)" "$EMITTER_TESTS" \
  "the W6 canary must carry the completed draft to the branded collector terminal"

if rg -n 'lookup_core_method|lookup_core_method_result_row|selector|lower_method_call|RuntimeExecutablePlan|function_address|image_digest|Vm|Interpreter' \
  "$ADMISSION_DIR"; then
  guard_fail "$TAG" "I0-C admission must not re-search generated rows, selectors, runtime plans, addresses, images, or VM lanes"
fi
if rg -n -U '#\[derive\([^\n]*Clone[^\n]*\)\][[:space:]]*\n[[:space:]]*(pub\([^)]*\)[[:space:]]+)?struct[[:space:]]+(PreparedAotExecutableAdmissionV1|AdmittedTextScanRegistryV1)' \
  "$ADMISSION_DIR" || \
  rg -n 'Clone[[:space:]]*for[[:space:]]+(PreparedAotExecutableAdmissionV1|AdmittedTextScanRegistryV1)|into_parts|raw registry' \
  "$ADMISSION_DIR"; then
  guard_fail "$TAG" "I0-C admission must remain move-only without raw registry escape"
fi

guard_expect_fixed_in_file "$TAG" 'receiver_lane: TextScanValueLaneV1::HostHandle' \
  "$ROOT_DIR/src/abi/text_scan_aot_export_facts.rs" \
  "Rust export facts must declare the HostHandle receiver lane"
guard_expect_fixed_in_file "$TAG" '"receiver_lane": VALUE_HOST_HANDLE' \
  "$ROOT_DIR/src/llvm_py/builders/dynamic_v2_text_scan_export_facts.py" \
  "Python export facts must declare the HostHandle receiver lane"
guard_expect_fixed_in_file "$TAG" 'HAKO_TEXT_SCAN_SUBSTRING_RECEIVER_LANE' \
  "$ROOT_DIR/include/nyrt_dynamic_text_scan_v1.h" \
  "C export facts must declare the substring receiver lane"
guard_expect_fixed_in_file "$TAG" 'HAKO_TEXT_SCAN_INDEX_OF_RECEIVER_LANE' \
  "$ROOT_DIR/include/nyrt_dynamic_text_scan_v1.h" \
  "C export facts must declare the indexOf receiver lane"
guard_expect_fixed_in_file "$TAG" 'TextScanCallAbiFactV1' \
  "$ROOT_DIR/src/abi/text_scan_aot_export_facts.rs" \
  "Rust projection must retain the checked call ABI facts"
guard_expect_fixed_in_file "$TAG" 'TextScanCallTransportReturnV1::U32' \
  "$ROOT_DIR/src/abi/text_scan_aot_export_facts.rs" \
  "Rust projection must use a transport-only u32 return"
guard_expect_fixed_in_file "$TAG" 'parameter_types' \
  "$ROOT_DIR/src/abi/text_scan_aot_export_facts.rs" \
  "Rust projection must retain exact parameter order and signedness"
guard_expect_fixed_in_file "$TAG" '"parameter_types"' \
  "$ROOT_DIR/src/llvm_py/builders/dynamic_v2_text_scan_export_facts.py" \
  "Python projection must retain exact parameter order and signedness"
guard_expect_fixed_in_file "$TAG" 'HakoDynamicV2CallOutV1 *out' \
  "$ROOT_DIR/include/nyrt_dynamic_text_scan_v1.h" \
  "C projection must require the neutral checked out wire"
guard_expect_fixed_in_file "$TAG" '"call_abi"' \
  "$ROOT_DIR/src/llvm_py/builders/dynamic_v2_text_scan_export_facts.py" \
  "Python projection must retain the checked call ABI facts"
guard_expect_fixed_in_file "$TAG" 'HAKO_TEXT_SCAN_CALL_OUT_WIRE_REVISION' \
  "$ROOT_DIR/include/nyrt_dynamic_text_scan_v1.h" \
  "C projection must pin the neutral output wire revision"

guard_expect_fixed_in_file "$TAG" 'ModuleInvocationBrandV1' \
  "$ADMISSION_DIR/aot_admission.rs" \
  "symbolic admission must carry the existing compile-session PlanStamp"
guard_expect_fixed_in_file "$TAG" "symbol: &'static str" \
  "$ADMISSION_DIR/aot_admission.rs" \
  "symbolic admission must retain the neutral export symbol"
guard_expect_fixed_in_file "$TAG" 'RejectBeforeEffect' \
  "$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_capability.rs" \
  "pre-link admission must remain non-production/reject-only"

echo "[$TAG] ok"
