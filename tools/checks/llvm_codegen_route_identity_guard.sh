#!/usr/bin/env bash
# Guard the observed LLVM/llvmlite route identity without changing behavior.
# This is a census guard: it pins the current source-backed selectors and
# requires the graduation card to classify every known ingress explicitly.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="llvm-codegen-route-identity-guard"
CARD="$ROOT/docs/development/current/main/investigations/llvm-native-library-llvmlite-graduation-task-2026-07-22.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
ROUTE_ENTRY="$ROOT/src/host_providers/llvm_codegen/mir_json_text_object.rs"
ROUTE="$ROOT/src/host_providers/llvm_codegen/route.rs"
BOUNDARY_FFI="$ROOT/crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs"
BOUNDARY_DEFAULTS="$ROOT/crates/nyash-llvm-compiler/src/boundary_driver_defaults.rs"
RUNNER_EXEC="$ROOT/src/runner/modes/common_util/exec.rs"
LLVM_RUNNER="$ROOT/src/runner/product/llvm/mod.rs"
HARNESS_EXECUTOR="$ROOT/src/runner/product/llvm/harness_executor.rs"
FALLBACK_EXECUTOR="$ROOT/src/runner/product/llvm/fallback_executor.rs"
LLVM_VM_FLAGS="$ROOT/src/config/env/vm_backend_flags.rs"
SELECTED_BUNDLE="$ROOT/src/runner/modes/common_util/selected_dynamic_artifact_bundle.rs"
CAPI="$ROOT/src/host_providers/llvm_codegen/capi_transport.rs"
PROVIDER="$ROOT/src/host_providers/llvm_codegen/provider_keep.rs"
PLUGIN="$ROOT/src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs"
AOT="$ROOT/lang/c-abi/shims/hako_aot_shared_impl.inc"
AOT_GENERIC="$ROOT/lang/c-abi/shims/hako_aot_generic_ffi_compile.inc"
AOT_CHILD="$ROOT/lang/c-abi/shims/hako_aot_child_process.inc"
AOT_SMOKE="$ROOT/tools/checks/llvm_hako_aot_ffi_admission_smoke.sh"
C_COMMON="$ROOT/lang/c-abi/shims/hako_llvmc_ffi_common.inc"
CAPI_ROUTE="$ROOT/lang/c-abi/shims/hako_llvmc_ffi_route.inc"
CAPI_INVOCATION="$ROOT/lang/c-abi/shims/hako_llvmc_ffi_invocation.inc"
PUBLISHED_ROWS="$ROOT/lang/c-abi/shims/published_mir/hako_llvmc_ffi_published_static_method.inc"
LEGACY_EMITTER="$ROOT/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_legacy_capi_emit.inc"
CAPI_CAPTURE_TEST="$ROOT/lang/c-abi/tests/legacy_capi_invocation_capture_test.c"
FAST_CAPTURE_TEST="$ROOT/lang/c-abi/tests/fast_invocation_capture_test.c"
NYLLVM_README="$ROOT/crates/nyash-llvm-compiler/README.md"
HARNESS_SCRIPT="$ROOT/tools/run_llvm_harness.sh"
FAST_SMOKE="$ROOT/.github/workflows/fast-smoke.yml"
CABI_README="$ROOT/lang/c-abi/README.md"
ENV_INVENTORY="$ROOT/docs/development/current/main/design/environment-variables-inventory-ssot.md"
STAGE1_BUILD="$ROOT/tools/selfhost/mainline/build_stage1.sh"
STAGE1_CONTRACT="$ROOT/tools/selfhost/lib/stage1_contract.sh"
SELFHOST_README="$ROOT/tools/selfhost/README.md"
ENV_REFERENCE="$ROOT/docs/reference/environment-variables.md"

fail() {
  echo "[$TAG] FAIL: $*" >&2
  exit 1
}

need_file() {
  [[ -f "$1" ]] || fail "missing source or owner: ${1#$ROOT/}"
}

need_fixed() {
  local file="$1" pattern="$2" label="$3"
  rg -Fq -- "$pattern" "$file" || fail "$label (${file#$ROOT/})"
}

for file in "$CARD" "$INDEX" "$ROUTE_ENTRY" "$ROUTE" "$CAPI" "$PROVIDER" "$PLUGIN" "$AOT" \
  "$RUNNER_EXEC" "$LLVM_RUNNER" "$HARNESS_EXECUTOR" "$FALLBACK_EXECUTOR" "$LLVM_VM_FLAGS" "$C_COMMON" "$CAPI_ROUTE" \
  "$CAPI_INVOCATION" "$PUBLISHED_ROWS" "$LEGACY_EMITTER" "$CAPI_CAPTURE_TEST" \
  "$FAST_CAPTURE_TEST" \
  "$NYLLVM_README" "$HARNESS_SCRIPT" "$FAST_SMOKE" "$CABI_README" "$ENV_INVENTORY"; do
  need_file "$file"
done
need_file "$SELECTED_BUNDLE"
need_file "$BOUNDARY_FFI"
need_file "$BOUNDARY_DEFAULTS"
need_file "$AOT_GENERIC"
need_file "$AOT_CHILD"
need_file "$AOT_SMOKE"
need_file "$STAGE1_BUILD"
need_file "$STAGE1_CONTRACT"
need_file "$SELFHOST_README"
need_file "$ENV_REFERENCE"

# The source-derived route owner must remain one ordered chokepoint.
need_fixed "$ROUTE_ENTRY" 'route::try_compile_via_capi_keep' "CAPI route entry missing"
need_fixed "$ROUTE_ENTRY" 'route::try_compile_via_explicit_provider_keep' "provider route entry missing"
need_fixed "$ROUTE_ENTRY" 'route::try_compile_via_boundary_default' "Boundary route entry missing"

capi_line="$(rg -n 'route::try_compile_via_capi_keep' "$ROUTE_ENTRY" | head -n1 | cut -d: -f1)"
provider_line="$(rg -n 'route::try_compile_via_explicit_provider_keep' "$ROUTE_ENTRY" | head -n1 | cut -d: -f1)"
boundary_line="$(rg -n 'route::try_compile_via_boundary_default' "$ROUTE_ENTRY" | head -n1 | cut -d: -f1)"
[[ "$capi_line" -lt "$provider_line" && "$provider_line" -lt "$boundary_line" ]] || \
  fail "route precedence drifted: expected CAPI -> explicit provider -> Boundary"

# Pin the typed request boundary, not descriptive names or ambient provider
# labels. The caller-zero ny-llvmc provider helper must not re-enter here.
need_fixed "$ROUTE" 'CodegenRouteRequestV1::ExplicitHarnessCompat => mir_json_to_object_llvmlite' \
  "explicit llvmlite request route missing"
need_fixed "$ROUTE" 'CodegenRouteRequestV1::BoundaryPureFirst => Ok(None)' \
  "Boundary request must not enter the explicit provider keep"
if rg -n 'mir_json_to_object_ny_llvmc|Some\("ny-llvmc"\) =>' "$ROUTE" "$PROVIDER"; then
  fail "caller-zero Rust ny-llvmc provider selector is still present"
fi
need_fixed "$PROVIDER" 'tools/llvmlite_harness.py' "Python harness owner missing"
need_fixed "$AOT" '--driver harness' "generic C -> hako_aot harness selector missing"

# OBSERVE0-R0 keeps route selection and actual child evidence on separate
# existing owners.  This is a static census only; it must not create a
# durable receipt or infer child reachability from a runner hint.
need_fixed "$ROUTE" '[llvm-route/select] owner={} recipe={} compat_replay={}' \
  "Hako route selection observation owner drifted"
need_fixed "$ROUTE" 'fn llvm_route_trace_enabled()' "Hako trace gate owner missing"
need_fixed "$ROUTE" 'Some("1" | "on" | "true" | "yes")' "Hako trace default-off gate drifted"
need_fixed "$BOUNDARY_FFI" '[llvm-route/select] owner=boundary recipe={}' \
  "Boundary route selection observation owner drifted"
need_fixed "$C_COMMON" 'static int hako_llvmc_route_trace_enabled(void)' \
  "C child trace gate owner missing"
need_fixed "$C_COMMON" 'if (!hako_llvmc_route_trace_enabled()) return;' \
  "C trace must remain diagnostic-only and default-off"
need_fixed "$CAPI_ROUTE" '\"%s\" --driver harness --in' \
  "CAPI child-command owner drifted"
need_fixed "$AOT" 'env_prefix ? env_prefix : ""' \
  "generic AOT child-command env seam drifted"
need_fixed "$AOT_GENERIC" 'hako_aot_build_direct_harness_env_prefix' \
  "generic AOT child environment owner missing"
need_fixed "$AOT" '#include "hako_aot_child_process.inc"' \
  "generic AOT process-boundary include missing"
need_fixed "$AOT_CHILD" 'CreateProcessA' \
  "Windows AOT child process boundary missing"
need_fixed "$AOT_CHILD" 'GetEnvironmentStringsA' \
  "Windows AOT inherited environment capture missing"
need_fixed "$AOT_CHILD" 'NAME= remains distinct' \
  "Windows AOT empty environment contract missing"
need_fixed "$AOT_SMOKE" 'MODE="${1:-all}"' \
  "AOT child-env standalone mode missing"
need_fixed "$AOT_SMOKE" 'run_child_env_checks(temp, out)' \
  "AOT child-env check must run before real compatibility lanes"
need_fixed "$AOT_SMOKE" 'mode == "child-env"' \
  "AOT child-env early-exit mode missing"
need_fixed "$AOT_SMOKE" 'aot_child_env_probe.c' \
  "Windows child observer must distinguish missing from empty"
need_fixed "$AOT_SMOKE" 'aot_windows_env_call.py' \
  "Windows DLL parent must receive environment before CRT initialization"
if rg -n 'hako_aot_ensure_default_opt_env|setenv\("HAKO_LLVM_OPT_LEVEL"|setenv\("NYASH_LLVM_OPT_LEVEL"|set "HAKO_LLVM_OPT_LEVEL=|set "NYASH_LLVM_OPT_LEVEL=' \
  "$AOT" "$AOT_GENERIC" "$AOT_CHILD"; then
  fail "direct AOT harness still mutates the parent opt-level environment"
fi
need_fixed "$CAPI_ROUTE" '"child"' "CAPI child observation producer missing"
need_fixed "$CAPI_ROUTE" '"driver=harness"' "CAPI child observation shape drifted"
need_fixed "$AOT" 'hako_aot_emit_child_trace(' \
  "generic AOT child observation producer missing"
need_fixed "$SELECTED_BUNDLE" 'exec::validate_selected_dynamic_boundary_route_request()?' \
  "selected Dynamic route-request gate missing"
need_fixed "$RUNNER_EXEC" 'expected pure-first or unset' \
  "selected Dynamic recipe boundary drifted"
need_fixed "$RUNNER_EXEC" 'expected none or unset' \
  "selected Dynamic replay boundary drifted"
need_fixed "$RUNNER_EXEC" 'rejects explicit HAKO_LLVM_EMIT_PROVIDER' \
  "selected Dynamic provider inheritance gate missing"
need_fixed "$LLVM_RUNNER" 'LlvmHarnessInvocationPolicyV1' \
  "ordinary LLVM harness invocation policy owner missing"
need_fixed "$LLVM_RUNNER" 'let policy = LlvmHarnessInvocationPolicyV1::capture();' \
  "ordinary LLVM harness policy must be captured at the route boundary"
need_fixed "$HARNESS_EXECUTOR" 'ensure_harness_requested(policy)?' \
  "harness executor must consume the captured policy"
need_fixed "$FALLBACK_EXECUTOR" 'if policy.primary_request_failfast' \
  "fallback executor must consume the captured primary gate"
need_fixed "$RUNNER_EXEC" 'ny_llvmc_emit_exe_lib_with_harness_policy' \
  "harness child emitter must receive the captured precheck policy"
need_fixed "$LLVM_VM_FLAGS" 'pub fn llvm_harness_primary_requested()' \
  "primary harness gate owner missing"
need_fixed "$LLVM_VM_FLAGS" 'pub fn llvm_harness_child_nyrt_precheck_bypass()' \
  "literal-1 child precheck owner missing"
if rg -n 'env_bool\("NYASH_LLVM_USE_HARNESS"\)' "$LLVM_RUNNER" "$HARNESS_EXECUTOR" "$FALLBACK_EXECUTOR"; then
  fail "ordinary LLVM runner still rereads the primary harness selector"
fi
if rg -n 'std::env::var\("NYASH_LLVM_USE_HARNESS"\)' "$RUNNER_EXEC"; then
  fail "common executable transport still reads the harness selector directly"
fi
need_fixed "$AOT" 'stage=child result=%s reason=ny-llvmc extra=driver=harness' \
  "generic AOT child observation shape drifted"
need_fixed "$CARD" 'LLVMLITE-ROUTE0-OBSERVE0-R0' "OBSERVE0-R0 row missing"
for field in request_id entry_family driver export recipe compat_replay python_child artifact_result; do
  need_fixed "$CARD" "\`$field\`" "OBSERVE0-R0 field missing: $field"
done

# F0 closes the Rust transport's caller-zero ambient branch. Boundary callers
# use the versioned options entry; named harness callers bypass CAPI. Public C
# exports and the AOT dlsym surface remain separate compatibility owners.
if rg -n 'compile_symbol|std::env::(set_var|remove_var)' "$CAPI"; then
  fail "caller-zero Rust CAPI symbol/env branch is still present"
fi
need_fixed "$CAPI" 'compile_via_capi_with_options' \
  "explicit Rust CAPI options transport missing"
need_fixed "$BOUNDARY_FFI" 'OwnedPhysicalCompileContract' \
  "Boundary invocation-owned options contract missing"
need_fixed "$BOUNDARY_FFI" 'hako_llvmc_compile_json_with_options_v1' \
  "Boundary versioned options entry missing"
need_fixed "$BOUNDARY_FFI" 'byte_size: std::mem::size_of::<PhysicalCompileContractV1>() as u32' \
  "Boundary C layout size pin missing"
need_fixed "$BOUNDARY_FFI" 'llvmc_path: std::ptr::null()' \
  "Boundary unsupported llvmc path must remain null"
if rg -n 'CompileFn|with_compile_symbol|call_compile_symbol|with_env_override|boundary_compile_symbol|boundary_codegen_request_defaults|std::env::(set_var|remove_var)\("HAKO_BACKEND_' \
  "$BOUNDARY_FFI" "$BOUNDARY_DEFAULTS"; then
  fail "Boundary Rust legacy symbol/env transport is still present"
fi
if rg -n 'hako_llvmc_compile_json\\0|hako_llvmc_compile_json_pure_first' "$BOUNDARY_FFI"; then
  fail "Boundary Rust legacy compile dlsym is still present"
fi
need_fixed "$CAPI_ROUTE" 'hako_llvmc_require_pure_first_recipe' \
  "generic C recipe gate missing"
need_fixed "$CAPI_ROUTE" 'generic-capi-recipe-required' \
  "generic C recipe failure missing"
need_fixed "$CAPI_ROUTE" 'compile_json_public_generic_options' \
  "public Generic options bridge missing"
need_fixed "$CAPI_ROUTE" 'HAKO_LLVMC_PHYSICAL_PROFILE_GENERIC_COMPAT' \
  "public Generic profile-0 bridge missing"
need_fixed "$CAPI_ROUTE" 'hako_llvmc_resolve_tool("NYASH_NY_LLVM_OPT_TOOL", "opt", "opt-18")' \
  "public Generic opt-tool capture missing"
need_fixed "$CAPI_ROUTE" 'hako_llvmc_llc_flags()' \
  "public Generic llc-flag capture missing"
need_fixed "$CAPI_INVOCATION" 'hako_llvmc_invocation_capture_legacy_capi' \
  "legacy CAPI invocation capture owner missing"

# Published-call rows are a private physical transport owned by one
# invocation. Keep the old process-global object and zero-argument production
# accessors out of the route while allowing the focused test's local owners.
python3 - "$PUBLISHED_ROWS" "$CAPI_INVOCATION" "$ROOT/lang/c-abi/shims" <<'PY'
import pathlib
import re
import sys

published_rows = pathlib.Path(sys.argv[1]).read_text()
invocation = pathlib.Path(sys.argv[2]).read_text()
shims = pathlib.Path(sys.argv[3])
assert "} hako_llvmc_published_call_rows;" not in published_rows
assert "struct HakoLlvmcPublishedCallRows published_call_rows;" in invocation
for path in shims.rglob("*.inc"):
    text = path.read_text()
    assert re.search(r"hako_llvmc_published_call_rows_active\(\s*\)", text) is None, path
    assert re.search(r"hako_llvmc_published_static_method_rows_(?:finish|end)\(\s*\)", text) is None, path
PY
need_fixed "$CAPI_INVOCATION" 'struct HakoLlvmcLegacyCapiState' \
  "legacy CAPI invocation state missing"
need_fixed "$CAPI_CAPTURE_TEST" 'mutation-after-capture' \
  "legacy CAPI capture mutation proof missing"
need_fixed "$CAPI_INVOCATION" 'hako_llvmc_invocation_capture_fast' \
  "FAST invocation capture owner missing"
need_fixed "$FAST_CAPTURE_TEST" 'fast invocation capture: PASS' \
  "FAST invocation capture proof missing"
python3 - "$C_COMMON" "$CAPI_INVOCATION" \
  "$ROOT/lang/c-abi/shims/hako_llvmc_ffi_const_string_hoist.inc" \
  "$ROOT/lang/c-abi/shims/hako_llvmc_ffi_string_concat_emit_helpers.inc" \
  "$ROOT/lang/c-abi/shims/hako_llvmc_ffi_string_chain_policy.inc" <<'PY'
import pathlib
import sys
common, invocation, hoist, result, trace = map(pathlib.Path, sys.argv[1:])
assert 'hako_llvmc_fast_enabled' not in common.read_text()
assert 'fast_enabled' in invocation.read_text()
assert 'invocation->fast_enabled' in hoist.read_text()
assert 'invocation->fast_enabled' in result.read_text()
assert 'invocation->fast_enabled' in trace.read_text()
PY
python3 - "$LEGACY_EMITTER" <<'PY'
import pathlib
import sys
text = pathlib.Path(sys.argv[1]).read_text()
assert 'getenv(' not in text, 'legacy emitter still reads ambient environment'
assert 'invocation->legacy_capi.target_machine_enabled' in text
assert 'invocation->legacy_capi.opt_level' in text
PY
if rg -n 'compile_json_via_pure_first_lane|compile_json_compat_pure\(' "$CAPI_ROUTE"; then
  fail "public Generic route still has the retired private wrapper"
fi
if rg -Fq -- 'compile_json_via_default_forwarder' "$CAPI_ROUTE"; then
  fail "generic C export still forwards recipe-unset input to hako_aot"
fi
need_fixed "$AOT_GENERIC" 'hako_aot_reject_ambient_harness_replay' \
  "generic AOT ambient replay gate missing"
need_fixed "$AOT_GENERIC" 'hako_llvmc_compile_json_with_options_v1' \
  "generic AOT options entry missing"
need_fixed "$AOT_GENERIC" 'HAKO_LLVMC_PHYSICAL_PROFILE_GENERIC_COMPAT' \
  "generic AOT profile missing"
need_fixed "$AOT" '#include "hako_aot_generic_ffi_compile.inc"' \
  "generic AOT transport is not included by the shared owner"
if rg -n 'dlsym\(h, "hako_llvmc_compile_json"\)|ffi_compile_fn' "$AOT_GENERIC"; then
  fail "generic AOT old compile dlsym/type is still present"
fi
if rg -n 'dlsym\(h, "hako_llvmc_compile_json"\)' "$AOT"; then
  fail "generic AOT old compile dlsym is still present"
fi
need_fixed "$AOT" 'aot-compat-admission-required' \
  "generic AOT replay failure missing"
need_fixed "$AOT" 'hako_aot_compile_json_compat_harness' \
  "named AOT compatibility export missing"
need_fixed "$CAPI_ROUTE" 'hako_llvmc_reject_ambient_harness_replay' \
  "generic C ambient replay gate missing"
need_fixed "$CAPI_ROUTE" 'hako_llvmc_compile_json_compat_harness' \
  "named C compatibility export missing"
python3 - "$CAPI_ROUTE" <<'PY'
import pathlib
import sys
text = pathlib.Path(sys.argv[1]).read_text()
executor = text.split('static int compile_json_compat_harness_execute(', 1)[1].split('\n}\n', 1)[0]
named = text.split('int hako_llvmc_compile_json_compat_harness(', 1)[1].split('\n}\n', 1)[0]
assert 'getenv(' not in executor, 'harness executor rereads compiler environment'
assert 'compile_json_compat_harness_keep(' not in named, 'named export reentered ambient adapter'
assert 'hako_llvmc_physical_options_copy_named_harness(' in named
assert 'hako_llvmc_physical_options_destroy(&options)' in named
for retired in ['compile_json_compat_harness_keep', 'compile_json_via_explicit_compat_harness_replay']:
    assert retired not in text, 'retired automatic replay adapter returned'
core = pathlib.Path(sys.argv[1]).with_name('hako_llvmc_ffi_pure_compile.inc').read_text()
assert 'compat_harness_replay_enabled(' not in core, 'pure core can dispatch automatic replay'
assert 'hako_llvmc_emit_route_replay("none", "unsupported_pure_shape")' in core
PY
need_fixed "$STAGE1_CONTRACT" 'stage1_contract_resolve_backend_replay' \
  "Stage1 replay admission helper missing"
need_fixed "$STAGE1_CONTRACT" 'replay-unadmitted' \
  "Stage1 inherited replay fail-fast missing"
need_fixed "$STAGE1_BUILD" '--compat-replay <none|harness>' \
  "Stage1 explicit replay option missing"
need_fixed "$STAGE1_BUILD" 'replay_admission=' \
  "Stage1 replay admission receipt missing"
need_fixed "$STAGE1_BUILD" 'compat_replay=${STAGE1_COMPAT_REPLAY}' \
  "Stage1 replay metadata missing"
need_fixed "$STAGE1_BUILD" 'compile_recipe=${HAKO_BACKEND_COMPILE_RECIPE}' \
  "Stage1 recipe metadata missing"
need_fixed "$SELFHOST_README" '--compat-replay harness' \
  "Stage1 explicit replay documentation missing"
need_fixed "$ENV_REFERENCE" 'Stage1 buildでは環境変数だけでは受理せず' \
  "Stage1 environment admission documentation missing"
if rg -Fq -- 'export HAKO_BACKEND_COMPAT_REPLAY="${HAKO_BACKEND_COMPAT_REPLAY:-none}"' "$STAGE1_BUILD"; then
  fail "Stage1 build still accepts inherited replay as an implicit admission"
fi
if rg -Fq -- 'Err(_e) => Ok(None)' "$PLUGIN"; then
  fail "codegen plugin still converts backend failure to None"
fi
need_fixed "$PLUGIN" 'fn codegen_result_to_bid' \
  "codegen plugin typed-result adapter missing"
need_fixed "$PLUGIN" 'BidError::PluginError' \
  "codegen plugin typed failure mapping missing"
need_fixed "$PLUGIN" 'll_text_to_object' "compile_ll_text owner disappeared"
need_fixed "$PLUGIN" '"emit_object_compat_harness"' \
  "named env.codegen compatibility admission missing"
need_fixed "$PLUGIN" 'CodegenRouteRequestV1::BoundaryPureFirst' \
  "ordinary env.codegen Boundary admission missing"
need_fixed "$PLUGIN" 'CodegenRouteRequestV1::ExplicitHarnessCompat' \
  "named harness admission missing"
need_fixed "$PLUGIN" 'validate_ordinary_ambient_replay' \
  "ordinary ambient replay gate missing"
need_fixed "$PLUGIN" 'HAKO_LLVM_CHILD_OBSERVATION_CASE' \
  "separate child observation case selector missing"
need_fixed "$PLUGIN" '[env.codegen/ordinary] rejects ambient compat replay' \
  "ordinary ambient replay error contract missing"
need_fixed "$ROUTE" 'if opts.route_request == CodegenRouteRequestV1::ExplicitHarnessCompat' \
  "named harness route must bypass generic C-ABI probing"
need_fixed "$ROUTE" 'CodegenRouteRequestV1::ExplicitHarnessCompat => {' \
  "named harness route must select its explicit provider"
need_fixed "$ROUTE" 'Boundary route rejects compat replay inheritance' \
  "ordinary inherited replay failure missing"

# The task card and check index are the tracked documentation owners.
need_fixed "$CARD" 'LLVMLITE-ROUTE0-CENSUS0-IDENTITY-GUARD-S0' "identity guard task missing"
need_fixed "$CARD" 'LLVMLITE-AUTO0-HAKO-AOT-FFI-ADMISSION-F0' "hako_aot FFI admission receipt missing"
need_fixed "$CARD" 'generic C export -> `hako_aot_compile_json`' "generic C harness route missing from matrix"
need_fixed "$CARD" 'compile_ll_text' "external-tool route missing from matrix"
need_fixed "$INDEX" 'tools/checks/llvm_codegen_route_identity_guard.sh' "check index entry missing"

# Identity0 documentation surfaces must describe the actual selectors.
need_fixed "$NYLLVM_README" 'explicit ny-llvmc keep lane: `--driver harness`' "ny-llvmc README selector wording drifted"
need_fixed "$NYLLVM_README" 'not a direct `ny-llvmc --driver` selector' "runner hint distinction missing"
need_fixed "$HARNESS_SCRIPT" 'historical script name and NYASH_LLVM_USE_HARNESS hint' "harness script identity note missing"
need_fixed "$FAST_SMOKE" 'name: boundary-and-explicit-compat-smoke' "fast-smoke route label drifted"
need_fixed "$FAST_SMOKE" 'explicit compatibility replay' "fast-smoke replay label missing"
need_fixed "$CABI_README" 'C ABI 自体は LLVM driver/provider の selector ではない' "C ABI selector boundary missing"
need_fixed "$ENV_INVENTORY" 'Top-level LLVM compatibility-runner hint; not a direct `ny-llvmc` driver selector' "env selector classification drifted"

# Optional process-level evidence. Static route checks stay cheap by default;
# G0 closeout can opt in to three isolated test processes and prove that only
# the named compat case reaches a Python child.
if [[ "${LLVM_ROUTE_IDENTITY_CHILD_OBSERVATION:-0}" == "1" ]]; then
  command -v strace >/dev/null 2>&1 || fail "strace is required for child observation"
  test_name='runtime::plugin_loader_v2::enabled::compat_codegen_receiver::tests::child_observation_probe_is_opt_in_only'
  for case_name in ordinary compat replay; do
    trace_file="$(mktemp)"
    case_replay=none
    expected=0
    if [[ "$case_name" == "compat" ]]; then
      expected=1
    elif [[ "$case_name" == "replay" ]]; then
      case_replay=harness
    fi
    if HAKO_LLVM_CHILD_OBSERVATION=1 \
      HAKO_LLVM_CHILD_OBSERVATION_CASE="$case_name" \
      HAKO_BACKEND_COMPAT_REPLAY="$case_replay" \
      NYASH_LLVM_USE_CAPI=0 \
      HAKO_V1_EXTERN_PROVIDER_C_ABI=0 \
      HAKO_ROOT="$ROOT" \
      strace -f -e trace=process,execve -o "$trace_file" \
      cargo test -q "$test_name" --lib -- --exact --nocapture >/dev/null 2>&1; then
      status=0
    else
      status=$?
    fi
    [[ "$status" == "0" ]] || fail "child observation test failed: case=$case_name status=$status"
    child_count="$(rg -c 'execve\("[^"]*/python(3)?"' "$trace_file" || true)"
    child_count="${child_count:-0}"
    if [[ "$case_name" == "compat" ]]; then
      [[ "$child_count" -ge 1 ]] || fail "named compat child count is $child_count, expected >=1"
    else
      [[ "$child_count" == "0" ]] || fail "$case_name child count is $child_count, expected 0"
    fi
    echo "[$TAG] child-observation case=$case_name python_child=$child_count"
  done
fi

echo "[$TAG] ok (selectors, precedence, known hazards, and route matrix are source-backed)"
