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
RUNNER_EXEC="$ROOT/src/runner/modes/common_util/exec.rs"
CAPI="$ROOT/src/host_providers/llvm_codegen/capi_transport.rs"
PROVIDER="$ROOT/src/host_providers/llvm_codegen/provider_keep.rs"
PLUGIN="$ROOT/src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs"
AOT="$ROOT/lang/c-abi/shims/hako_aot_shared_impl.inc"
C_COMMON="$ROOT/lang/c-abi/shims/hako_llvmc_ffi_common.inc"
CAPI_ROUTE="$ROOT/lang/c-abi/shims/hako_llvmc_ffi_route.inc"
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
  "$RUNNER_EXEC" "$C_COMMON" "$CAPI_ROUTE" \
  "$NYLLVM_README" "$HARNESS_SCRIPT" "$FAST_SMOKE" "$CABI_README" "$ENV_INVENTORY"; do
  need_file "$file"
done
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

# Pin actual selectors, not descriptive names or NYASH_LLVM_USE_HARNESS labels.
need_fixed "$ROUTE" 'Some("llvmlite") => mir_json_to_object_llvmlite' "explicit llvmlite selector missing"
need_fixed "$ROUTE" 'Some("ny-llvmc") => mir_json_to_object_ny_llvmc' "explicit ny-llvmc selector missing"
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
need_fixed "$AOT" '\"%s\" --driver harness --in' \
  "generic AOT child-command owner drifted"
need_fixed "$CAPI_ROUTE" '"child"' "CAPI child observation producer missing"
need_fixed "$CAPI_ROUTE" '"driver=harness"' "CAPI child observation shape drifted"
need_fixed "$AOT" 'hako_aot_emit_child_trace(' \
  "generic AOT child observation producer missing"
need_fixed "$RUNNER_EXEC" 'validate_selected_dynamic_boundary_route_request()?' \
  "selected Dynamic route-request gate missing"
need_fixed "$RUNNER_EXEC" 'expected pure-first or unset' \
  "selected Dynamic recipe boundary drifted"
need_fixed "$RUNNER_EXEC" 'expected none or unset' \
  "selected Dynamic replay boundary drifted"
need_fixed "$RUNNER_EXEC" 'rejects explicit HAKO_LLVM_EMIT_PROVIDER' \
  "selected Dynamic provider inheritance gate missing"
need_fixed "$AOT" 'stage=child result=%s reason=ny-llvmc extra=driver=harness' \
  "generic AOT child observation shape drifted"
need_fixed "$CARD" 'LLVMLITE-ROUTE0-OBSERVE0-R0' "OBSERVE0-R0 row missing"
for field in request_id entry_family driver export recipe compat_replay python_child artifact_result; do
  need_fixed "$CARD" "\`$field\`" "OBSERVE0-R0 field missing: $field"
done

# F0 closes the requested-symbol fallback. A missing pure-first symbol must
# fail at the CAPI lookup rather than silently entering the generic harness
# ingress. The generic recipe/no-recipe route remains a later G1 row.
if rg -Fq -- 'or_else(|_| lib.get(defaults::COMPILE_SYMBOL_DEFAULT))' "$CAPI"; then
  fail "CAPI requested-symbol fallback is still present"
fi
need_fixed "$CAPI" '.get(compile_symbol)' \
  "CAPI requested-symbol lookup missing"
need_fixed "$CAPI_ROUTE" 'hako_llvmc_require_pure_first_recipe' \
  "generic C recipe gate missing"
need_fixed "$CAPI_ROUTE" 'generic-capi-recipe-required' \
  "generic C recipe failure missing"
if rg -Fq -- 'compile_json_via_default_forwarder' "$CAPI_ROUTE"; then
  fail "generic C export still forwards recipe-unset input to hako_aot"
fi
need_fixed "$AOT" 'hako_aot_require_explicit_harness_replay' \
  "direct AOT replay gate missing"
need_fixed "$AOT" 'direct-aot-replay-required' \
  "direct AOT replay failure missing"
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

# The task card and check index are the tracked documentation owners.
need_fixed "$CARD" 'LLVMLITE-ROUTE0-CENSUS0-IDENTITY-GUARD-S0' "identity guard task missing"
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

echo "[$TAG] ok (selectors, precedence, known hazards, and route matrix are source-backed)"
