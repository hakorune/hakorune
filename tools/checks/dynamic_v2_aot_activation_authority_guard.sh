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
LEASE_HEADER="$ROOT_DIR/include/nyrt_dynamic_v2_lease_v1.h"
NYRT_HEADER="$ROOT_DIR/include/nyrt.h"
RUST="$ROOT_DIR/src/abi/text_scan_aot_export_facts.rs"
PYTHON="$ROOT_DIR/src/llvm_py/builders/dynamic_v2_text_scan_export_facts.py"
CODEGEN_TEST="$ROOT_DIR/tools/checks/lib/provider_slot_contract_codegen_tests.py"
PROJECTION_TEST="$ROOT_DIR/tools/checks/lib/text_scan_export_projection_tests.py"
STRICT_LEAF="$ROOT_DIR/crates/nyash_kernel/src/exports/dynamic_v2_text_scan.rs"
LEASE="$ROOT_DIR/src/runtime/dynamic_v2_lease.rs"
LEASE_ADAPTER="$ROOT_DIR/crates/nyash_kernel/src/ffi/dynamic_v2_lease.rs"
LEASE_FFI_MOD="$ROOT_DIR/crates/nyash_kernel/src/ffi/mod.rs"
METADATA="$ROOT_DIR/src/llvm_py/builders/dynamic_v2_aot_admission.py"
HOOK="$ROOT_DIR/src/llvm_py/instructions/mir_call/selected_dynamic_v2.py"
METADATA_TEST="$ROOT_DIR/src/llvm_py/tests/test_dynamic_v2_aot_admission.py"
CALLOUT_TRANSPORT="$ROOT_DIR/src/llvm_py/builders/checked_callout_transport.py"
CALLOUT_TRANSPORT_TEST="$ROOT_DIR/src/llvm_py/tests/test_checked_callout_transport.py"
CALLOUT_TEST_PLAN="$ROOT_DIR/src/llvm_py/builders/checked_callout_test_plan.py"
CALLOUT_TEST_PLAN_TEST="$ROOT_DIR/src/llvm_py/tests/test_checked_callout_test_plan.py"
RUST_METADATA="$ROOT_DIR/src/box_callable/provider_admission/call_metadata.rs"
RUST_METADATA_TEST="$ROOT_DIR/src/box_callable/provider_admission/call_metadata_tests.rs"
JSON_METADATA="$ROOT_DIR/src/runner/mir_json_emit/dynamic_v2_aot_admission.rs"
LINK_DRIVER="$ROOT_DIR/crates/nyash-llvm-compiler/src/link_driver.rs"
PLAN_OWNER="$ROOT_DIR/crates/nyash-llvm-compiler/src/runtime_executable_plan.rs"
CALLOUT_OWNER="$ROOT_DIR/src/mir/checked_callout/site_plan.rs"
CALLOUT_CENSUS="$ROOT_DIR/src/mir/checked_callout/census.rs"
CALLOUT_FACADE="$ROOT_DIR/src/mir/checked_callout.rs"
CALLOUT_TESTS="$ROOT_DIR/src/mir/checked_callout/tests.rs"
CALLOUT_CFG="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_cfg/session.rs"
CALLOUT_SSA="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/session.rs"
SELECTED_CAPABILITY="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_capability.rs"
SELECTED_EMITTER="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/mod.rs"
CALLOUT_CORRIDOR="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/callout_corridor/mod.rs"
CALLOUT_CORRIDOR_EMISSION="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/callout_corridor/emission.rs"
SELECTED_LIFECYCLE="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/lifecycle_terminal.rs"
CATALOGED_HANDOFF="$ROOT_DIR/src/mir/builder/cataloged_box_method_collector_handoff.rs"
CATALOGED_HANDOFF_TESTS="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/tests.rs"
PACKAGE_INSTALL="$ROOT_DIR/src/mir/normal_callable_semantic_package/install.rs"
PACKAGE_ADAPTER="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port.rs"
C1A_ROUTE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_route.inc"
C1A_LOWERING="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc"
C1A_HEADER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_selected_dynamic_entry_header.inc"
C1_OWNER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_checked_callout_lowering.inc"
C1_DISPATCH="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch.inc"
C1_PRESCAN="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc"
C1_SHIM="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi.c"
C1_SMOKE="$ROOT_DIR/tools/checks/dynamic_v2_checked_callout_physicalizer_smoke.sh"
ARTIFACT_DESCRIPTOR_HEADER="$ROOT_DIR/include/hako_dynamic_v2_artifact_descriptor_v1.h"
ARTIFACT_DESCRIPTOR_EMITTER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_dynamic_v2_artifact_descriptor.inc"
ARTIFACT_DESCRIPTOR_OPEN="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_ir_open.inc"
ARTIFACT_DESCRIPTOR_RUST="$ROOT_DIR/crates/nyash-llvm-compiler/src/link_driver/static_artifact_descriptor.rs"
ARTIFACT_PUBLICATION_RUST="$ROOT_DIR/crates/nyash-llvm-compiler/src/link_driver/static_artifact_publication.rs"
ARTIFACT_PUBLICATION_TESTS="$ROOT_DIR/crates/nyash-llvm-compiler/src/link_driver/static_artifact_publication/tests.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_command "$TAG" llvm-nm
guard_require_files "$TAG" "$SOURCE" "$MODULE" "$CODEGEN" "$MANIFEST" "$HEADER" "$LEASE_HEADER" "$NYRT_HEADER" "$RUST" "$PYTHON" "$CODEGEN_TEST" "$PROJECTION_TEST" "$STRICT_LEAF" "$LEASE" "$LEASE_ADAPTER" "$LEASE_FFI_MOD" "$METADATA" "$HOOK" "$METADATA_TEST" "$CALLOUT_TRANSPORT" "$CALLOUT_TRANSPORT_TEST" "$CALLOUT_TEST_PLAN" "$CALLOUT_TEST_PLAN_TEST" "$RUST_METADATA" "$RUST_METADATA_TEST" "$JSON_METADATA" "$LINK_DRIVER" "$PLAN_OWNER" "$CALLOUT_FACADE" "$CALLOUT_OWNER" "$CALLOUT_CENSUS" "$CALLOUT_TESTS" "$CALLOUT_CFG" "$CALLOUT_SSA" "$SELECTED_CAPABILITY" "$SELECTED_EMITTER" "$CALLOUT_CORRIDOR" "$CALLOUT_CORRIDOR_EMISSION" "$SELECTED_LIFECYCLE" "$CATALOGED_HANDOFF" "$CATALOGED_HANDOFF_TESTS" "$PACKAGE_INSTALL" "$PACKAGE_ADAPTER"
guard_require_files "$TAG" "$C1A_ROUTE" "$C1A_LOWERING" "$C1A_HEADER"
guard_require_files "$TAG" "$C1_OWNER" "$C1_DISPATCH" "$C1_PRESCAN" "$C1_SHIM" "$C1_SMOKE"
guard_require_files "$TAG" "$ARTIFACT_DESCRIPTOR_HEADER" "$ARTIFACT_DESCRIPTOR_EMITTER" "$ARTIFACT_DESCRIPTOR_OPEN"
guard_require_files "$TAG" "$ARTIFACT_DESCRIPTOR_RUST" "$ARTIFACT_PUBLICATION_RUST" "$ARTIFACT_PUBLICATION_TESTS"

# C1-A is the only selected-entry signature owner.  The legacy no-parameter
# header remains available for old seeds, but selected Dynamic must pass the
# exact four-formal gate before the C1 physicalizer can be enabled.
if [[ "$(rg -n '^static int hako_llvmc_selected_dynamic_parameter_signature_valid\(' "$C1A_ROUTE" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "C1-A selected Dynamic parameter-signature validator must have one owner"
fi
if [[ "$(rg -n '^static int hako_llvmc_emit_selected_dynamic_entry_header\(' "$C1A_ROUTE" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "C1-A parameterized selected-entry header issuer must have one owner"
fi
if [[ "$(grep -F -c '#include "hako_llvmc_ffi_selected_dynamic_entry_header.inc"' "$C1A_LOWERING")" != 1 ]]; then
  guard_fail "$TAG" "C1-A selected-entry header include must be exactly once"
fi
guard_expect_fixed_in_file "$TAG" "dynamic_v2_aot_call_admission_v2" "$C1A_ROUTE" \
  "C1-A selected gate must be tied to the existing candidate admission metadata"
guard_expect_fixed_in_file "$TAG" "count != 4" "$C1A_ROUTE" \
  "C1-A selected Dynamic signature must require exactly four formal values"
guard_expect_fixed_in_file "$TAG" "selected_dynamic_parameter_signature_mismatch" "$C1A_HEADER" \
  "C1-A mismatch must fail before C1 physicalization"
if rg -n 'hako_llvmc_emit_entry_header\(f, &selection\)' "$C1A_LOWERING"; then
  guard_fail "$TAG" "generic lowering must not directly choose the legacy header for selected Dynamic"
fi
for file in "$C1A_ROUTE" "$C1A_LOWERING" "$C1A_HEADER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "C1-A source reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

for file in "$CALLOUT_FACADE" "$CALLOUT_OWNER" "$CALLOUT_CENSUS" "$CALLOUT_TESTS" "$CALLOUT_CORRIDOR" "$CALLOUT_CORRIDOR_EMISSION"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 700 )); then
    guard_fail "$TAG" "W6-S source split file reached the mandatory 700-line gate: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

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
for file in "$CALLOUT_TEST_PLAN" "$CALLOUT_TEST_PLAN_TEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 650 )); then
    guard_fail "$TAG" "test-only CheckedCallOut planner reached its 650-line cap: ${file#"$ROOT_DIR/"} has $lines"
  fi
done
for file in "$RUST_METADATA" "$RUST_METADATA_TEST" "$JSON_METADATA"; do
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
  --glob '*.py' --glob '!checked_callout_transport.py' --glob '!checked_callout_test_plan.py'; then
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
if [[ "$(rg -n 'fn verify_checked_callout_function_v1\(' "$CALLOUT_CENSUS" | wc -l | tr -d '[:space:]')" != 1 ]]; then
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
    --glob '!src/llvm_py/tests/test_checked_callout_transport.py' \
    --glob '!src/llvm_py/builders/checked_callout_test_plan.py' \
    --glob '!src/llvm_py/tests/test_checked_callout_test_plan.py' "$root"; then
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

# W6-E selected execution is a physical projection of the already-sealed
# metadata pair.  Keep the census and Boundary caller single-owned; ordinary
# modules remain on the existing compatibility path and selected failures do
# not enter its fallback branch.
SELECTED_RUNNER="$ROOT_DIR/src/runner/product/llvm/mod.rs"
SELECTED_CENSUS_CALLERS="$(rg -l --glob '*.rs' \
  'exec::selected_dynamic_aot_metadata_present' \
  "$ROOT_DIR/src/runner/product/llvm" || true)"
if [[ "$SELECTED_CENSUS_CALLERS" != "$SELECTED_RUNNER" ]]; then
  guard_fail "$TAG" "selected Dynamic metadata census must have exactly one runner caller"
fi
if [[ "$(rg -n 'HarnessExecutorBox::try_execute_selected_dynamic\(module\)' \
  "$SELECTED_RUNNER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "selected Dynamic Boundary execution caller must be unique"
fi
guard_expect_fixed_in_file "$TAG" 'backend: "ny_llvmc_selected_dynamic_exe"' "$SELECTED_RUNNER" \
  "selected Dynamic execution must report the dedicated Boundary backend"
guard_expect_fixed_in_file "$TAG" 'if selected_dynamic {' "$SELECTED_RUNNER" \
  "selected Dynamic metadata must branch before ordinary fallback"
guard_expect_fixed_in_file "$TAG" \
  'StaticArtifactReceiptConsumedFenceV1, String>' \
  "$ROOT_DIR/src/runner/modes/common_util/exec.rs" \
  "selected Boundary emitter must return the consumed receipt fence"
guard_expect_fixed_in_file "$TAG" \
  'pub(crate) fn selected_dynamic_nyrt_dir()' \
  "$ROOT_DIR/src/runner/modes/common_util/exec.rs" \
  "selected Boundary must resolve one explicit NyRT archive directory"
guard_expect_fixed_in_file "$TAG" \
  'Some(nyrt_dir.as_str())' \
  "$ROOT_DIR/src/runner/product/llvm/harness_executor.rs" \
  "selected Boundary must pass the explicit NyRT archive directory"
if rg -n -A16 'pub fn ny_llvmc_emit_exe_selected_dynamic_bin\(' \
  "$ROOT_DIR/src/runner/modes/common_util/exec.rs" | rg -q 'nyrt_dir\.ok_or_else'; then
  :
else
  guard_fail "$TAG" "selected Boundary emitter must reject an omitted explicit NyRT directory"
fi
if [[ "$(rg -n 'let _receipt_fence' \
  "$ROOT_DIR/src/runner/product/llvm/harness_executor.rs" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "selected runner must consume one artifact receipt fence before execution"
fi
if [[ "$(rg -n 'run_selected_dynamic_after_receipt\(' \
  "$ROOT_DIR/src/runner/product/llvm/harness_executor.rs" | wc -l | tr -d '[:space:]')" != 2 ]]; then
  guard_fail "$TAG" "selected execution must have one receipt-gated helper and one caller"
fi
if rg -n 'lookup_core_method|into_parts|\.clone\(|RuntimeExecutablePlanV1::clone' "$PLAN_OWNER"; then
  guard_fail "$TAG" "W3 post-link plan owner must not re-resolve or clone semantic facts"
fi
if rg -n \
  --glob '*.py' \
  --glob '!**/tests/**' \
  --glob '!**/dynamic_v2_aot_admission.py' \
  --glob '!**/selected_dynamic_v2.py' \
  --glob '!**/checked_callout_test_plan.py' \
  'selected_dynamic_v2|dynamic_v2_aot_admission' \
  "$ROOT_DIR/src/llvm_py/builders" "$ROOT_DIR/src/llvm_py/instructions"; then
  guard_fail "$TAG" "I0-D1 metadata/hook must have zero production Python callers"
fi
projection_callers=()
while IFS= read -r file; do
  case "$file" in
    "$SELECTED_EMITTER") projection_callers+=("$file") ;;
    *_tests.rs|*/tests.rs) ;;
    *) guard_fail "$TAG" "C0-B Rust metadata projection gained an unapproved caller: ${file#"$ROOT_DIR/"}" ;;
  esac
done < <(rg -l --glob '*.rs' --glob '!call_metadata.rs' \
  'project_dynamic_v2_aot_call_metadata\(' "$ROOT_DIR/src" || true)
if [[ "${#projection_callers[@]}" -ne 1 ]]; then
  guard_fail "$TAG" "C0-B Rust metadata projection must have exactly one selected-session caller"
fi
json_callers=()
while IFS= read -r file; do
  case "$file" in
    "$ROOT_DIR/src/runner/mir_json_emit/metadata.rs") json_callers+=("$file") ;;
    *) guard_fail "$TAG" "C0-B JSON metadata emitter gained an unapproved caller: ${file#"$ROOT_DIR/"}" ;;
  esac
done < <(rg -l --glob '*.rs' --glob '!dynamic_v2_aot_admission.rs' \
  'insert_dynamic_v2_aot_call_admission_json\(' "$ROOT_DIR/src" || true)
if [[ "${#json_callers[@]}" -ne 1 ]]; then
  guard_fail "$TAG" "C0-B JSON metadata emitter must have exactly one metadata consumer"
fi
if rg -n 'lookup_core_method|selector|PreparedAotExecutableAdmissionV1::|into_parts|clone\(' "$RUST_METADATA" "$JSON_METADATA"; then
  guard_fail "$TAG" "I0-D1b projection must borrow retained facts without reseal, lookup, selector, or clone"
fi
guard_expect_fixed_in_file "$TAG" 'DynamicV2AotCallMetadataProjectionV1' "$RUST_METADATA" "Rust typed metadata projection is missing"
guard_expect_fixed_in_file "$TAG" 'DynamicV2AotFormalProjectionV1' "$RUST_METADATA" "formal ValueId/lane transport projection is missing"
guard_expect_fixed_in_file "$TAG" 'normal_result_dst' "$RUST_METADATA" "Normal-result destination transport is missing"
guard_expect_fixed_in_file "$TAG" 'function_effects' "$RUST_METADATA" "verified function effect transport is missing"
guard_expect_fixed_in_file "$TAG" 'normal_shape' "$RUST_METADATA" "per-site Normal shape transport is missing"
guard_expect_fixed_in_file "$TAG" 'dynamic_v2_aot_call_admission_v2' "$JSON_METADATA" "JSON metadata key projection is missing"
guard_expect_fixed_in_file "$TAG" 'formal_parameters' "$JSON_METADATA" "JSON formal parameter transport is missing"
guard_expect_fixed_in_file "$TAG" 'normal_result_dst' "$JSON_METADATA" "JSON Normal-result destination transport is missing"
guard_expect_fixed_in_file "$TAG" 'site_id' "$RUST_METADATA" "AOT metadata projection must use canonical CheckedCallOut site identity"
guard_expect_fixed_in_file "$TAG" 'site_id' "$METADATA" "Python AOT metadata loader must require canonical site identity"
guard_expect_fixed_in_file "$TAG" 'formal_parameters' "$METADATA" "Python AOT metadata loader must require formal transport"
guard_expect_fixed_in_file "$TAG" 'normal_result_dst' "$METADATA" "Python AOT metadata loader must require Normal-result transport"
guard_expect_fixed_in_file "$TAG" '_required_u16' "$METADATA" "Python AOT metadata loader must bound typed effect fields"
RUST_METADATA_BODY="$(sed '/^#\[cfg(test)\]/,$d' "$RUST_METADATA")"
if printf '%s\n' "$RUST_METADATA_BODY" | rg -n 'instruction_index|(^|[[:space:]])block[[:space:]]*:' || \
   rg -n 'instruction_index|(^|[[:space:]])block[[:space:]]*:' "$JSON_METADATA"; then
  guard_fail "$TAG" "AOT downstream metadata must not expose the old block/instruction locator"
fi
if rg -n 'require_call_edge|instruction_index|\["block"\]' "$METADATA"; then
  guard_fail "$TAG" "Python AOT metadata loader must not locate selected calls by the old block/index pair"
fi
if rg -n 'llvmlite|IRBuilder|lower_instruction|RuntimeExecutablePlan|dynamic_v2_text_scan|mir_call' "$CALLOUT_TEST_PLAN" "$CALLOUT_TEST_PLAN_TEST"; then
  guard_fail "$TAG" "test-only CheckedCallOut planner must remain detached from LLVM/runtime/dispatcher"
fi

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
if [[ "$(rg -n '#\[export_name = \"nyrt_dynamic_v2_lease_consume_end_authorized_v1\"\]' "$LEASE_ADAPTER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "neutral lease C ABI adapter symbol must be defined exactly once"
fi
if [[ "$(rg -n 'dynamic_v2_lease::consume_end_authorized' "$LEASE_ADAPTER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "neutral lease adapter must call the Rust owner exactly once"
fi
if rg -n 'drop_handle|release_h|HostHandle' "$LEASE_ADAPTER"; then
  guard_fail "$TAG" "neutral lease adapter must not own raw handle lifecycle"
fi
for pair in \
  'NYRT_DYNAMIC_V2_LEASE_ABI_REVISION_V1 UINT32_C(1)' \
  'NYRT_DYNAMIC_V2_LEASE_CONSUME_OK UINT32_C(0)' \
  'NYRT_DYNAMIC_V2_LEASE_CONSUME_INVALID_TOKEN UINT32_C(1)' \
  'NYRT_DYNAMIC_V2_LEASE_CONSUME_UNKNOWN_OR_ALREADY_CONSUMED UINT32_C(2)' \
  'NYRT_DYNAMIC_V2_LEASE_CONSUME_STALE_HANDLE_IDENTITY UINT32_C(3)'; do
  guard_expect_fixed_in_file "$TAG" "$pair" "$LEASE_HEADER" "lease C ABI vocabulary drifted: $pair"
done
guard_expect_fixed_in_file "$TAG" 'nyrt_dynamic_v2_lease_consume_end_authorized_v1' "$LEASE_HEADER" \
  "lease C ABI declaration is missing"
guard_expect_fixed_in_file "$TAG" '#include "nyrt_dynamic_v2_lease_v1.h"' "$NYRT_HEADER" \
  "core NyRT header must reference the versioned lease ABI fragment"
guard_expect_fixed_in_file "$TAG" 'pub mod dynamic_v2_lease;' "$LEASE_FFI_MOD" \
  "kernel FFI module must wire the neutral lease adapter"
LEASE_ARCHIVE="${CARGO_TARGET_DIR_EFFECTIVE:-$ROOT_DIR/target}/release/libnyash_kernel.a"
if [[ -f "$LEASE_ARCHIVE" ]]; then
  lease_symbol_count="$(llvm-nm -g --defined-only "$LEASE_ARCHIVE" 2>/dev/null | awk '$NF == "nyrt_dynamic_v2_lease_consume_end_authorized_v1" {count++} END {print count + 0}')"
  if [[ "$lease_symbol_count" != 1 ]]; then
    guard_fail "$TAG" "static NyRT archive must define the lease End symbol exactly once (got $lease_symbol_count)"
  fi
else
  echo "[$TAG] informational: static NyRT archive not built; source/export checks still apply"
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

# C1 Boundary physicalization is one transport-only owner.  It consumes the
# typed site-id projection once, emits the two direct entries and lowers the
# three canonical End cutpoints through the sole Rust lease ABI consumer.
if [[ "$(grep -F -c '#include "hako_llvmc_ffi_checked_callout_lowering.inc"' "$C1_SHIM")" != 1 ]]; then
  guard_fail "$TAG" "C1 checked-callout lowering include must be exactly once"
fi
if [[ "$(rg -n '^static int hako_llvmc_c1_validate_projection\(' "$C1_OWNER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "C1 projection validator must have one owner"
fi
if [[ "$(rg -n '^static int hako_llvmc_c1_emit_callout\(' "$C1_OWNER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "C1 CheckedCallOut emitter must have one owner"
fi
if [[ "$(rg -n '^static int hako_llvmc_c1_emit_end\(' "$C1_OWNER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "C1 End emitter must have one owner"
fi
if [[ "$(rg -n 'hako_llvmc_c1_emit_callout\(' "$C1_DISPATCH" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "C1 dispatch consumer must be exactly once"
fi
if [[ "$(rg -n 'hako_llvmc_c1_emit_declarations\(' "$C1_PRESCAN" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "C1 declaration prescan consumer must be exactly once"
fi
if [[ "$(rg -n 'hako_llvmc_c1_emit_end\(' "$C1_OWNER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "C1 End issuer definition must be unique"
fi
guard_expect_fixed_in_file "$TAG" 'nyrt_dynamic_v2_lease_consume_end_authorized_v1' "$C1_OWNER" \
  "C1 End must use the sole neutral lease C ABI"
guard_expect_fixed_in_file "$TAG" 'HAKO_TEXT_SCAN_SYMBOL_SUBSTRING' "$C1_OWNER" \
  "C1 must use the existing direct substring symbol"
guard_expect_fixed_in_file "$TAG" 'HAKO_TEXT_SCAN_SYMBOL_INDEX_OF' "$C1_OWNER" \
  "C1 must use the existing direct indexOf symbol"
if rg -n 'require_call_site|lookup_core_method|drop_handle|release_h|mir_call|fallback|retry' "$C1_OWNER"; then
  guard_fail "$TAG" "C1 owner must not use selector lookup, generic calls, raw release, fallback, or retry"
fi
if rg -n 'Provider|Registry|RuntimeExecutablePlan|VM|Interpreter|selector|name lookup' "$C1_OWNER"; then
  guard_fail "$TAG" "C1 owner must not open provider, registry, executable-plan, or VM authority"
fi
if (( $(wc -l < "$C1_OWNER" | tr -d '[:space:]') >= 700 )); then
  guard_fail "$TAG" "C1 owner reached its 700-line design boundary"
fi

# W6-D-I0: the selected Boundary link path receives one explicit archive
# argument. Legacy hako_llvmc_link_obj remains compatibility-only; the Rust
# Boundary driver must not rediscover the archive through an environment.
LINK_FFI="$ROOT_DIR/crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs"
LINK_C_HEADER="$ROOT_DIR/lang/c-abi/include/hako_aot.h"
LINK_C_IMPL="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc"
LINK_ROUTE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_route.inc"
LINK_SMOKE="$ROOT_DIR/tools/checks/dynamic_v2_w6_explicit_link_abi_smoke.sh"
guard_require_files "$TAG" "$LINK_FFI" "$LINK_C_HEADER" "$LINK_C_IMPL" "$LINK_ROUTE" "$LINK_SMOKE"
if [[ ! -x "$LINK_SMOKE" ]]; then
  guard_fail "$TAG" "W6-D explicit-link smoke must be executable"
fi
if [[ "$(rg -n '\.get\(b\"hako_llvmc_link_obj_v2\\0' "$LINK_FFI" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "selected Rust Boundary driver must load hako_llvmc_link_obj_v2 exactly once"
fi
if [[ "$(rg -n '^int hako_llvmc_link_obj_v2\(' "$LINK_C_IMPL" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "versioned C link export must have one issuer"
fi
if [[ "$(rg -n '^static int forward_link_obj_to_aot_v2\(' "$LINK_ROUTE" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "versioned link forwarder must have one issuer"
fi
guard_expect_fixed_in_file "$TAG" 'runtime_archive_path' "$LINK_C_HEADER" \
  "explicit archive path must be part of the C ABI"
if rg -n -F 'hako_llvmc_link_obj\0' "$LINK_FFI" || \
   rg -n -F 'NYASH_EMIT_EXE_NYRT' "$LINK_FFI"; then
  guard_fail "$TAG" "selected Rust Boundary link path must not use legacy symbol or archive env override"
fi
for file in "$LINK_FFI" "$LINK_C_HEADER" "$LINK_C_IMPL" "$LINK_ROUTE"; do
  lines=$(wc -l < "$file" | tr -d '[:space:]')
  if (( lines >= 800 )); then
    guard_fail "$TAG" "W6-D link boundary reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

# W6-D-I1: the selected object owns one fixed descriptor projection and the
# Rust transaction issues one move-only receipt from actual object/archive/
# executable observations.  The selected Boundary runner consumes that
# receipt; ordinary compatibility remains a separate explicit edge.
if [[ "$(grep -F -c '#include "hako_llvmc_ffi_dynamic_v2_artifact_descriptor.inc"' "$C1_SHIM")" != 1 ]]; then
  guard_fail "$TAG" "artifact descriptor emitter include must be exactly once"
fi
if [[ "$(grep -F -c '#include "hako_llvmc_ffi_pure_compile_ir_open.inc"' "$C1_PRESCAN")" != 1 ]]; then
  guard_fail "$TAG" "artifact descriptor emission must have one IR-open consumer"
fi
if [[ "$(rg -n '^static int hako_llvmc_c1_emit_artifact_descriptor\(' "$ARTIFACT_DESCRIPTOR_EMITTER" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "artifact descriptor emitter must have one owner"
fi
for token in \
  'HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SCHEMA UINT32_C(1)' \
  'HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SIZE UINT32_C(192)' \
  'HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_COUNT UINT32_C(2)' \
  'HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SYMBOL' \
  'HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SECTION'; do
  guard_expect_fixed_in_file "$TAG" "$token" "$ARTIFACT_DESCRIPTOR_HEADER" \
    "artifact descriptor layout drifted: $token"
done
guard_expect_fixed_in_file "$TAG" 'HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_OFFSET_SITE_ID' "$ARTIFACT_DESCRIPTOR_HEADER" \
  "artifact descriptor entries must retain canonical CheckedCallOut site identity"
if [[ "$(rg -n '^pub\(super\) struct StaticLinkedAotArtifactReceiptV1' "$ARTIFACT_PUBLICATION_RUST" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "static linked artifact receipt owner must be unique"
fi
if [[ "$(rg -n '^pub\(super\) struct StaticAotArtifactPublicationTxnV1' "$ARTIFACT_PUBLICATION_RUST" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "static artifact publication transaction owner must be unique"
fi
guard_expect_fixed_in_file "$TAG" 'expected_descriptor_from_json(input_json)' "$ARTIFACT_PUBLICATION_RUST" \
  "static transaction must co-check the final candidate metadata"
guard_expect_fixed_in_file "$TAG" 'observe_descriptor(object_path)' "$ARTIFACT_PUBLICATION_RUST" \
  "static transaction must observe the real object descriptor"
guard_expect_fixed_in_file "$TAG" 'require_archive_call_symbols(runtime_archive' "$ARTIFACT_PUBLICATION_RUST" \
  "static transaction must observe exact archive symbols"
guard_expect_fixed_in_file "$TAG" 'observe_descriptor(&candidate_path)' "$ARTIFACT_PUBLICATION_RUST" \
  "static transaction must observe the linked candidate descriptor"
guard_expect_fixed_in_file "$TAG" 'object_path: PathBuf' "$ARTIFACT_PUBLICATION_RUST" \
  "static receipt must retain exact object identity"
guard_expect_fixed_in_file "$TAG" 'runtime_archive_path: PathBuf' "$ARTIFACT_PUBLICATION_RUST" \
  "static receipt must retain exact runtime archive identity"
guard_expect_fixed_in_file "$TAG" 'candidate_path: PathBuf' "$ARTIFACT_PUBLICATION_RUST" \
  "static receipt must retain the observed temporary executable identity"
guard_expect_fixed_in_file "$TAG" 'symbol_census: StaticArtifactSymbolCensusV1' "$ARTIFACT_PUBLICATION_RUST" \
  "static receipt must retain exact symbol census evidence"
guard_expect_fixed_in_file "$TAG" 'rust_projection_matches_the_neutral_header_layout' "$ARTIFACT_DESCRIPTOR_RUST" \
  "Rust descriptor mirror must be checked against the neutral header"
guard_expect_fixed_in_file "$TAG" 'descriptor_symbol.section_index() != Some(section.index())' "$ARTIFACT_DESCRIPTOR_RUST" \
  "descriptor symbol must belong to the exact descriptor section"
guard_expect_fixed_in_file "$TAG" 'boundary_generated_object_survives_exact_link_and_receipt_observation' "$ARTIFACT_PUBLICATION_TESTS" \
  "Boundary-generated object must have an actual object-to-executable receipt test"
if rg -n -U '#\[derive\([^]]*Clone[^]]*\)\]\npub\(super\) struct (StaticAotArtifact|StaticLinked|PreparedStatic)' \
  "$ARTIFACT_DESCRIPTOR_RUST" "$ARTIFACT_PUBLICATION_RUST" || \
  rg -n 'impl[[:space:]]+Clone|into_parts|dlsym|RuntimeExecutablePlan|fallback|retry|selector|lookup_core_method' \
  "$ARTIFACT_DESCRIPTOR_RUST" "$ARTIFACT_PUBLICATION_RUST"; then
  guard_fail "$TAG" "static artifact owner must not clone/split/reselect or open another execution route"
fi
PREPARE_FILES="$(rg -l 'StaticAotArtifactPublicationTxnV1::prepare\(' \
  "$ROOT_DIR/crates/nyash-llvm-compiler/src" --glob '*.rs' --glob '!**/tests.rs' || true)"
if [[ "$PREPARE_FILES" != "$ROOT_DIR/crates/nyash-llvm-compiler/src/link_driver.rs" ]]; then
  guard_fail "$TAG" "static artifact preparation must have exactly one named child owner"
fi
if [[ "$(rg -n 'StaticAotArtifactPublicationTxnV1::prepare\(' \
  "$ROOT_DIR/crates/nyash-llvm-compiler/src/link_driver.rs" | wc -l | tr -d '[:space:]')" != 1 ]]; then
  guard_fail "$TAG" "static artifact preparation must have one child call site"
fi
COMMIT_FILES="$(rg -l 'prepared\.commit\(\)' "$ROOT_DIR/crates/nyash-llvm-compiler/src" \
  --glob '*.rs' --glob '!**/tests.rs' | sort -u)"
if [[ "$COMMIT_FILES" != "$ROOT_DIR/crates/nyash-llvm-compiler/src/link_driver.rs" ]]; then
  guard_fail "$TAG" "static artifact publication must have exactly one named child owner"
fi
for file in \
  "$ARTIFACT_DESCRIPTOR_HEADER" "$ARTIFACT_DESCRIPTOR_EMITTER" "$ARTIFACT_DESCRIPTOR_OPEN" \
  "$ARTIFACT_DESCRIPTOR_RUST" "$ARTIFACT_PUBLICATION_RUST" "$ARTIFACT_PUBLICATION_TESTS" "$C1_PRESCAN"; do
  lines=$(wc -l < "$file" | tr -d '[:space:]')
  if (( lines >= 760 )); then
    guard_fail "$TAG" "W6-D-I1 touched source reached the 760-line split gate: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

echo "[$TAG] ok"
