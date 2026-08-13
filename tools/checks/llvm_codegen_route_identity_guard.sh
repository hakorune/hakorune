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
CAPI="$ROOT/src/host_providers/llvm_codegen/capi_transport.rs"
PROVIDER="$ROOT/src/host_providers/llvm_codegen/provider_keep.rs"
PLUGIN="$ROOT/src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs"
AOT="$ROOT/lang/c-abi/shims/hako_aot_shared_impl.inc"

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

for file in "$CARD" "$INDEX" "$ROUTE_ENTRY" "$ROUTE" "$CAPI" "$PROVIDER" "$PLUGIN" "$AOT"; do
  need_file "$file"
done

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

# Keep the currently observed hazards visible until their later G1 rows change
# behavior. Removing either pattern without changing this guard is drift.
need_fixed "$CAPI" 'or_else(|_| lib.get(defaults::COMPILE_SYMBOL_DEFAULT))' \
  "CAPI symbol-fallback observation disappeared without a route-row update"
need_fixed "$PLUGIN" 'Err(_e) => Ok(None)' \
  "plugin compatibility failure policy changed without a route-row update"
need_fixed "$PLUGIN" 'll_text_to_object' "compile_ll_text owner disappeared"

# The task card and check index are the tracked documentation owners.
need_fixed "$CARD" 'LLVMLITE-ROUTE0-CENSUS0-IDENTITY-GUARD-S0' "identity guard task missing"
need_fixed "$CARD" 'generic C export -> `hako_aot_compile_json`' "generic C harness route missing from matrix"
need_fixed "$CARD" 'compile_ll_text' "external-tool route missing from matrix"
need_fixed "$INDEX" 'tools/checks/llvm_codegen_route_identity_guard.sh' "check index entry missing"

echo "[$TAG] ok (selectors, precedence, known hazards, and route matrix are source-backed)"
