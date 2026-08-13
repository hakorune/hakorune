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
STRICT_LEAF="$ROOT_DIR/crates/nyash_kernel/src/exports/dynamic_v2_text_scan.rs"
LEASE="$ROOT_DIR/src/runtime/dynamic_v2_lease.rs"
METADATA="$ROOT_DIR/src/llvm_py/builders/dynamic_v2_aot_admission.py"
HOOK="$ROOT_DIR/src/llvm_py/instructions/mir_call/selected_dynamic_v2.py"
METADATA_TEST="$ROOT_DIR/src/llvm_py/tests/test_dynamic_v2_aot_admission.py"
CALLOUT_TRANSPORT="$ROOT_DIR/src/llvm_py/builders/checked_callout_transport.py"
CALLOUT_TRANSPORT_TEST="$ROOT_DIR/src/llvm_py/tests/test_checked_callout_transport.py"
RUST_METADATA="$ROOT_DIR/src/box_callable/provider_admission/call_metadata.rs"
JSON_METADATA="$ROOT_DIR/src/runner/mir_json_emit/dynamic_v2_aot_admission.rs"
LINK_DRIVER="$ROOT_DIR/crates/nyash-llvm-compiler/src/link_driver.rs"
PLAN_OWNER="$ROOT_DIR/crates/nyash-llvm-compiler/src/runtime_executable_plan.rs"
CALLOUT_OWNER="$ROOT_DIR/src/mir/checked_callout.rs"
CALLOUT_CFG="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_cfg/session.rs"
CALLOUT_SSA="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/session.rs"
SELECTED_CAPABILITY="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_capability.rs"
SELECTED_EMITTER="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/mod.rs"
CALLOUT_CORRIDOR="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/callout_corridor.rs"
SELECTED_LIFECYCLE="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/lifecycle_terminal.rs"
CATALOGED_HANDOFF="$ROOT_DIR/src/mir/builder/cataloged_box_method_collector_handoff.rs"
CATALOGED_HANDOFF_TESTS="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/tests.rs"
PACKAGE_INSTALL="$ROOT_DIR/src/mir/normal_callable_semantic_package/install.rs"
PACKAGE_ADAPTER="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$SOURCE" "$MODULE" "$CODEGEN" "$MANIFEST" "$HEADER" "$RUST" "$PYTHON" "$CODEGEN_TEST" "$PROJECTION_TEST" "$STRICT_LEAF" "$LEASE" "$METADATA" "$HOOK" "$METADATA_TEST" "$CALLOUT_TRANSPORT" "$CALLOUT_TRANSPORT_TEST" "$RUST_METADATA" "$JSON_METADATA" "$LINK_DRIVER" "$PLAN_OWNER" "$CALLOUT_OWNER" "$CALLOUT_CFG" "$CALLOUT_SSA" "$SELECTED_CAPABILITY" "$SELECTED_EMITTER" "$CALLOUT_CORRIDOR" "$SELECTED_LIFECYCLE" "$CATALOGED_HANDOFF" "$CATALOGED_HANDOFF_TESTS" "$PACKAGE_INSTALL" "$PACKAGE_ADAPTER"

python3 "$CODEGEN_TEST"
python3 "$CODEGEN" --check
python3 "$PROJECTION_TEST"

for file in "$SOURCE" "$CODEGEN" "$MANIFEST" "$HEADER" "$RUST" "$PYTHON"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "I0-B artifact reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done
for file in "$METADATA" "$HOOK" "$METADATA_TEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "I0-D1 metadata file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done
for file in "$CALLOUT_TRANSPORT" "$CALLOUT_TRANSPORT_TEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "CheckedCallOut transport view reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done
for file in "$RUST_METADATA" "$JSON_METADATA"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "I0-D1b metadata file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done
if (( $(wc -l < "$LINK_DRIVER" | tr -d '[:space:]') >= 800 )); then
  guard_fail "$TAG" "link boundary reached hard 800-line boundary"
fi
if (( $(wc -l < "$PLAN_OWNER" | tr -d '[:space:]') >= 800 )); then
  guard_fail "$TAG" "post-link plan owner reached hard 800-line boundary"
fi

if [[ "$(rg -n '^def load_selected_dynamic_v2_aot_admission\(' "$METADATA" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "I0-D1 metadata loader definition must be unique"
fi
guard_expect_fixed_in_file "$TAG" "def _required_u64(" "$METADATA" \
  "I0-D1 metadata loader must enforce the Rust u64 boundary"
if [[ "$(rg -n '^def inspect_selected_dynamic_v2_call\(' "$HOOK" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "I0-D1 selected hook definition must be unique"
fi
if [[ "$(rg -n '^def require_selected_dynamic_v2_call\(' "$HOOK" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "I0-D1 terminal test seam definition must be unique"
fi
if [[ "$(rg -n '^def parse_checked_callout_transport\(' "$CALLOUT_TRANSPORT" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "CheckedCallOut transport parser definition must be unique"
fi
if rg -n 'checked_callout_transport' "$ROOT_DIR/src/llvm_py/instructions" "$ROOT_DIR/src/llvm_py/builders" \
  --glob '*.py' --glob '!checked_callout_transport.py'; then
  guard_fail "$TAG" "transport-only CheckedCallOut view must have no production importer before W6"
fi
if [[ "$(rg -n '^pub\(crate\) fn project_dynamic_v2_aot_call_metadata\(' "$RUST_METADATA" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "I0-D1b Rust metadata issuer definition must be unique"
fi
if [[ "$(rg -n '^pub\(crate\) fn insert_dynamic_v2_aot_call_admission_json\(' "$JSON_METADATA" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "I0-D1b JSON metadata emitter definition must be unique"
fi
if [[ "$(rg -n '^fn require_explicit_nyrt_archive\(' "$LINK_DRIVER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "W3 explicit --nyrt artifact boundary must have one owner"
fi
if [[ "$(rg -n '^pub\(crate\) struct RuntimeExecutablePlanV1' "$PLAN_OWNER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "W3 post-link executable plan owner must be unique"
fi
if [[ "$(rg -n '^pub\(crate\) fn issue_runtime_executable_plan\(' "$PLAN_OWNER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "W3 post-link executable plan issuer must be unique"
fi

# CheckedCallOut R0 is neutral MIR plumbing only.  Keep the site-plan owner and
# the two canonical session issuers unique while production/AOT callers remain 0.
if [[ "$(rg -n '^pub\(crate\) struct CheckedCallOutSitePlanV1' "$CALLOUT_OWNER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "CheckedCallOut site-plan owner definition must be unique"
fi
if [[ "$(rg -n 'fn emit_checked_callout\(' "$CALLOUT_CFG" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "Canonical CFG CheckedCallOut issuer must be unique"
fi
if [[ "$(rg -n 'fn define_checked_callout_normal_result\(' "$CALLOUT_SSA" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "Canonical Normal-result issuer must be unique"
fi
if [[ "$(rg -n 'fn verify_checked_callout_function_v1\(' "$CALLOUT_OWNER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "CheckedCallOut function census owner must be unique"
fi
guard_expect_fixed_in_file "$TAG" "CheckedCallOutSitePlanPairV1" "$CALLOUT_OWNER" \
  "CheckedCallOut must own the exact two-site transport shape"
guard_expect_fixed_in_file "$TAG" "prepare_aot_activation" "$SELECTED_CAPABILITY" \
  "selected capability admission must issue the site-plan transport"
guard_expect_fixed_in_file "$TAG" "PreparedSelectedDynamicV2AotActivationV1" "$SELECTED_CAPABILITY" \
  "site-plan transport must remain one move-only activation aggregate"
guard_expect_fixed_in_file "$TAG" "consume_for_session" "$SELECTED_CAPABILITY" \
  "the selected session must consume the activation aggregate exactly once"
guard_expect_fixed_in_file "$TAG" "install_checked_callout_site_plans" "$SELECTED_EMITTER" \
  "the emitter must install admitted plans before corridor allocation"
guard_expect_fixed_in_file "$TAG" "i6_site: CheckedCallOutSiteIdV1" "$CALLOUT_CORRIDOR" \
  "the private corridor must retain the I6 site identity"
guard_expect_fixed_in_file "$TAG" "self.i6_normal.matches(brand)" "$CALLOUT_CORRIDOR" \
  "the private corridor must verify both I6 landing brands"
guard_expect_fixed_in_file "$TAG" "site_pair_matches" "$CALLOUT_CORRIDOR" \
  "the private corridor must reject lifecycle/site-pair drift"
guard_expect_fixed_in_file "$TAG" "CheckedCallOutEnd" "$SELECTED_LIFECYCLE" \
  "the selected lifecycle owner must project the typed End instruction"
guard_expect_fixed_in_file "$TAG" "CheckedCallOutFault" "$SELECTED_LIFECYCLE" \
  "the selected lifecycle owner must project the non-rejoining Fault terminal"
guard_expect_fixed_in_file "$TAG" "fn parse_effect_mask" "$ROOT_DIR/src/runner/mir_json_v0/checked_callout.rs" \
  "CheckedCallOut JSON transport must reject unknown effect bits"
guard_expect_fixed_in_file "$TAG" "lifecycle_terminal::DynamicV2PhysicalLifecycleTerminalPlanV1::issue" "$SELECTED_EMITTER" \
  "the selected session must consume cleanup/site facts before Builder open"
guard_expect_fixed_in_file "$TAG" "emit_checked_callout_fault" "$CALLOUT_CFG" \
  "canonical CFG must own the physical Fault terminal issuer"
guard_expect_fixed_in_file "$TAG" "emit_checked_callout_end" "$CALLOUT_SSA" \
  "canonical SSA must own the physical End issuer"
if [[ "$(rg -n 'commit_cataloged_box_method_completed\(' "$CATALOGED_HANDOFF" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "cataloged Box-method collector terminal definition must be unique"
fi
if [[ "$(rg -n 'commit_cataloged_box_method_completed\(' "$CATALOGED_HANDOFF_TESTS" | wc -l | tr -d '[:space:]')" -lt 1 ]] && \
   ! rg -q 'assemble_unpublished_selected_dynamic_w6' "$CATALOGED_HANDOFF_TESTS"; then
  guard_fail "$TAG" "cataloged Box-method collector terminal needs a focused test caller"
fi
if rg -n 'commit_cataloged_box_method_completed\(' "$ROOT_DIR/src/mir" \
  --glob '*.rs' --glob '!**/tests.rs' --glob '!**/*_tests.rs' \
  --glob '!**/cataloged_box_method_collector_handoff.rs' \
  --glob '!**/selected_dynamic_physical_emitter/mod.rs'; then
  guard_fail "$TAG" "cataloged Box-method collector terminal must have no production caller before selected cutover"
fi
guard_expect_fixed_in_file "$TAG" "assemble_unpublished_selected_dynamic_w6" "$CATALOGED_HANDOFF_TESTS" \
  "the focused canary must consume the private unpublished W6 orchestration"
if [[ "$(rg -n 'pub\(crate\) fn with_selected_and_admission<R>\(' "$PACKAGE_INSTALL" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "selected cataloged input/admission loan issuer must be unique"
fi
if [[ "$(rg -n 'input\.with_selected_and_admission\(' "$PACKAGE_ADAPTER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "selected package adapter must consume the bounded input/admission loan exactly once"
fi
guard_expect_fixed_in_file "$TAG" "into_lowering_and_admission" "$PACKAGE_ADAPTER" \
  "legacy route must consume the same wrapper only after the selected loan closes"
if rg -n 'with_selected_and_admission\([^)]*clone|selected_and_admission.*into_parts' "$PACKAGE_INSTALL" "$PACKAGE_ADAPTER"; then
  guard_fail "$TAG" "selected input/admission loan must not clone or expose split parts"
fi
for file in "$CATALOGED_HANDOFF" "$CATALOGED_HANDOFF_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "cataloged collector handoff file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done
if rg -n 'ReleaseStrong|Throw|After|drop_handle|consume_end_authorized' "$SELECTED_LIFECYCLE"; then
  guard_fail "$TAG" "selected lifecycle owner must not reclassify cleanup or execute runtime lease effects"
fi
if [[ "$(rg -n '\.verify_checked_callout_function\(function\)' "$CALLOUT_SSA" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "canonical finish must consume the CheckedCallOut function census exactly once"
fi
CALLOUT_NORMAL_ISSUER="$(sed -n '/fn define_checked_callout_normal_result(/,/^    }/p' "$CALLOUT_SSA")"
if printf '%s\n' "$CALLOUT_NORMAL_ISSUER" | rg -n 'dst:[[:space:]]*ValueId'; then
  guard_fail "$TAG" "CheckedCallOut Normal-result destination must be minted by canonical SSA"
fi
if rg -n 'lower_method_call|RuntimeExecutablePlan|dynamic_v2_text_scan|lookup_core_method' \
  "$CALLOUT_OWNER" "$CALLOUT_CFG" "$CALLOUT_SSA"; then
  guard_fail "$TAG" "neutral CheckedCallOut R0 must not resolve provider, runtime, or generic method routes"
fi
for root in "$ROOT_DIR/src/llvm_py" "$ROOT_DIR/crates/nyash_kernel" "$ROOT_DIR/src/backend" "$ROOT_DIR/src/runtime"; do
  if [[ -d "$root" ]] && rg -n 'MirInstruction::CheckedCallOut|CheckedCallOutNormalResult|CheckedCallOutEnd|CheckedCallOutFault' \
    --glob '*.rs' --glob '*.py' \
    --glob '!src/llvm_py/builders/checked_callout_transport.py' \
    --glob '!src/llvm_py/tests/test_checked_callout_transport.py' "$root"; then
    guard_fail "$TAG" "CheckedCallOut has an unapproved production/JSON/VM caller before R0 completion"
  fi
done
# MIR JSON v0 is the sole transport-only exception: it may parse/emit the
# neutral vocabulary for round-trip inspection, but it must not dispatch it to
# LLVM, VM, runtime, or a selected production caller.
for file in \
  "$ROOT_DIR/src/runner/mir_json_v0/checked_callout.rs" \
  "$ROOT_DIR/src/runner/mir_json_emit/emitters/control_flow.rs" \
  "$ROOT_DIR/src/runner/mir_json_emit/emitters/mod.rs" \
  "$ROOT_DIR/src/runner/mir_json_emit/tests/checked_callout_transport.rs"; do
  if [[ ! -f "$file" ]]; then
    guard_fail "$TAG" "CheckedCallOut transport owner is missing: ${file#"$ROOT_DIR/"}"
  fi
done
while IFS= read -r file; do
  case "$file" in
    "$ROOT_DIR/src/runner/mir_json_v0/checked_callout.rs"|\
    "$ROOT_DIR/src/runner/mir_json_emit/emitters/control_flow.rs"|\
    "$ROOT_DIR/src/runner/mir_json_emit/emitters/mod.rs"|\
    "$ROOT_DIR/src/runner/mir_json_emit/tests/checked_callout_transport.rs") ;;
    *) guard_fail "$TAG" "CheckedCallOut has an unapproved runner caller: ${file#"$ROOT_DIR/"}" ;;
  esac
done < <(rg -l 'MirInstruction::CheckedCallOut|CheckedCallOutNormalResult|CheckedCallOutEnd|CheckedCallOutFault' \
  "$ROOT_DIR/src/runner" --glob '*.rs' --glob '*.py' || true)
if rg -n 'RuntimeExecutablePlanV1|issue_runtime_executable_plan\(' \
  --glob '*.rs' \
  --glob '!runtime_executable_plan.rs' \
  "$ROOT_DIR/crates/nyash-llvm-compiler/src"; then
  guard_fail "$TAG" "W3 executable plan must remain disconnected from production callers"
fi
if rg -n 'lookup_core_method|into_parts|\.clone\(|RuntimeExecutablePlanV1::clone' "$PLAN_OWNER"; then
  guard_fail "$TAG" "W3 post-link plan owner must not re-resolve or clone semantic facts"
fi
if rg -n \
  --glob '*.py' \
  --glob '!**/tests/**' \
  --glob '!**/dynamic_v2_aot_admission.py' \
  --glob '!**/selected_dynamic_v2.py' \
  'selected_dynamic_v2|dynamic_v2_aot_admission' \
  "$ROOT_DIR/src/llvm_py/builders" "$ROOT_DIR/src/llvm_py/instructions"; then
  guard_fail "$TAG" "I0-D1 metadata/hook must have zero production Python callers"
fi
if rg -n \
  --glob '*.rs' \
  --glob '!call_metadata.rs' \
  'project_dynamic_v2_aot_call_metadata\(' \
  "$ROOT_DIR/src"; then
  guard_fail "$TAG" "I0-D1b Rust metadata issuer must have zero production callers"
fi
if rg -n \
  --glob '*.rs' \
  --glob '!dynamic_v2_aot_admission.rs' \
  'insert_dynamic_v2_aot_call_admission_json\(' \
  "$ROOT_DIR/src"; then
  guard_fail "$TAG" "I0-D1b JSON metadata emitter must have zero production callers"
fi
if rg -n 'lookup_core_method|selector|PreparedAotExecutableAdmissionV1::|into_parts|clone\(' "$RUST_METADATA" "$JSON_METADATA"; then
  guard_fail "$TAG" "I0-D1b projection must borrow retained facts without reseal, lookup, selector, or clone"
fi
guard_expect_fixed_in_file "$TAG" 'DynamicV2AotCallMetadataProjectionV1' "$RUST_METADATA" "Rust typed metadata projection is missing"
guard_expect_fixed_in_file "$TAG" 'dynamic_v2_aot_call_admission_v1' "$JSON_METADATA" "JSON metadata key projection is missing"

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
    --glob '!**/exports/dynamic_v2_text_scan.rs' \
    'text_scan_aot_export_facts|TextScanAotExportFactV1|hako\.text\.scan\.(substring|index_of)\.v1' \
    "$root"; then
    guard_fail "$TAG" "I0-B symbolic export has an early production/runtime/VM caller: ${root#"$ROOT_DIR/"}"
  fi
done

if rg -n '^[[:space:]]*(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?(struct|enum|fn|const)[[:space:]].*(ProviderAdmissionSeal|RuntimeExecutablePlan|BoxCallableRegistry|lower_method_call|DynamicV2PhysicalEmissionSession)' "$SOURCE" "$CODEGEN" "$HEADER" "$RUST" "$PYTHON"; then
  guard_fail "$TAG" "I0-B artifact illegally opens provider/session/runtime authority"
fi

# I0-D strict leaf/lease is a work-branch checkpoint: its two exported
# definitions are allowed here, but no LLVM/VM/production caller is opened.
if [[ "$(rg -n '#\[export_name = "hako\.text\.scan\.(substring|index_of)\.v1"\]' "$STRICT_LEAF" | wc -l | tr -d '[:space:]')" != 2 ]]; then
  guard_fail "$TAG" "strict CodePoint leaf must define exactly two declared entries"
fi
if [[ "$(rg -n '^fn issue_end_authorized\(' "$LEASE" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "neutral lease owner must keep the raw-handle issuer private"
fi
if [[ "$(rg -n '^pub fn publish_end_authorized_text\(' "$LEASE" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "neutral lease owner must expose one aggregate publisher"
fi
if [[ "$(rg -n '^pub fn consume_end_authorized' "$LEASE" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "neutral lease owner must expose exactly one End consumer"
fi
LEASE_BODY="$(sed '/^#\[cfg(test)\]/,$d' "$LEASE")"
if [[ "$(printf '%s\n' "$LEASE_BODY" | rg -n 'capture_text_lease_identity|drop_if_lease_identity_matches' | wc -l | tr -d '[:space:]')" -lt 3 ]]; then
  guard_fail "$TAG" "lease owner must use text identity capture and conditional drop"
fi
if printf '%s\n' "$LEASE_BODY" | rg -n 'capture_lease_identity|drop_handle\('; then
  guard_fail "$TAG" "lease owner must not use generic capture or raw handle drop"
fi
if printf '%s\n' "$LEASE_BODY" | rg -n 'table\.insert\(token'; then
  guard_fail "$TAG" "lease token collision must preserve the existing entry"
fi
STRICT_BODY="$(sed '/^#\[cfg(test)\]/,$d' "$STRICT_LEAF")"
if printf '%s\n' "$STRICT_BODY" | rg -n 'index_mode_from_env|compat_fallback_allowed|hako_forward|drop_handle'; then
  guard_fail "$TAG" "strict leaf must not use generic mode, fallback, forwarding, or raw handle drop"
fi
if printf '%s\n' "$STRICT_BODY" | rg -n 'A_Prime|a_prime|Vm|Interpreter|lower_method_call|RuntimeExecutablePlan'; then
  guard_fail "$TAG" "strict leaf must not open VM, MIR, LLVM, or executable-plan routes"
fi
for file in "$STRICT_LEAF" "$LEASE" "$SELECTED_LIFECYCLE"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "I0-D strict leaf/lease file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

echo "[$TAG] ok"
