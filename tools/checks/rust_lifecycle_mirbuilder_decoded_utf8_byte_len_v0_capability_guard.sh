#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-decoded-utf8-byte-len-v0-capability"
source "$ROOT/tools/checks/lib/guard_common.sh"

LEAF="$ROOT/src/analysis/bounded_body_snapshot_v0/decoded_utf8_byte_len_v0.rs"
ROUTE_SPEC="$ROOT/src/mir/extern_call_route_plan/route_spec.rs"
BACKEND_CAPABILITY="$ROOT/src/mir/decoded_utf8_byte_len_backend_capability.rs"
BACKEND_OWNER="$ROOT/src/mir/backend_capability.rs"
CONTRACT_REFRESH="$ROOT/src/mir/semantic_refresh/contracts.rs"
EXTERN_BUILDER="$ROOT/src/mir/builder/calls/extern_calls.rs"
PROVIDER_LANE="$ROOT/src/backend/mir_interpreter/handlers/extern_provider/lane.rs"
RUNTIME_DIRECT="$ROOT/src/backend/mir_interpreter/handlers/extern_provider/runtime_direct.rs"
WRAPPER="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot/decoded_utf8_byte_len_v0.hako"
FIXTURE="$ROOT/tools/checks/fixtures/decoded_utf8_byte_len_v0.hako"
ROUTE_TEST="$ROOT/src/mir/extern_call_route_plan/route.rs"
RUNTIME_TEST="$ROOT/src/backend/mir_interpreter/handlers/extern_provider/tests.rs"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$LEAF" "$ROUTE_SPEC" "$BACKEND_CAPABILITY" \
  "$BACKEND_OWNER" "$CONTRACT_REFRESH" "$EXTERN_BUILDER" "$PROVIDER_LANE" \
  "$RUNTIME_DIRECT" "$WRAPPER" "$FIXTURE" "$ROUTE_TEST" "$RUNTIME_TEST"

cargo test -q --lib mir::decoded_utf8_byte_len_backend_capability
cargo test -q --lib \
  decoded_utf8_byte_len_v0_route_publishes_the_internal_integer_contract
cargo test -q --features vm-reference --lib \
  backend::mir_interpreter::handlers::extern_provider::tests::runtime_direct_decoded_utf8_byte_len_counts_unicode_and_embedded_nul

run_fixture() {
  local mode="$1"
  local output
  if [ "$mode" = "unset" ]; then
    if ! output="$(env -u NYASH_STR_CP NYASH_DISABLE_PLUGINS=1 cargo run -q \
      --features vm-reference --bin hakorune -- --backend vm "$FIXTURE" 2>&1)"; then
      printf '%s\n' "$output"
      guard_fail "$TAG" "Hako reference fixture command failed with NYASH_STR_CP unset"
    fi
  else
    if ! output="$(NYASH_STR_CP=1 NYASH_DISABLE_PLUGINS=1 cargo run -q \
      --features vm-reference --bin hakorune -- --backend vm "$FIXTURE" 2>&1)"; then
      printf '%s\n' "$output"
      guard_fail "$TAG" "Hako reference fixture command failed with NYASH_STR_CP=1"
    fi
  fi
  printf '%s\n' "$output"
  if [[ "$output" != *"RC: 0"* ]]; then
    guard_fail "$TAG" "Hako reference fixture failed with NYASH_STR_CP=$mode"
  fi
}

run_fixture unset
run_fixture 1

python3 - "$LEAF" "$ROUTE_SPEC" "$BACKEND_CAPABILITY" "$BACKEND_OWNER" \
  "$CONTRACT_REFRESH" "$EXTERN_BUILDER" "$PROVIDER_LANE" "$RUNTIME_DIRECT" \
  "$WRAPPER" "$FIXTURE" "$ROUTE_TEST" "$RUNTIME_TEST" <<'PY'
import sys
from pathlib import Path

(
    leaf_path,
    route_spec_path,
    capability_path,
    backend_owner_path,
    contract_refresh_path,
    extern_builder_path,
    provider_lane_path,
    runtime_direct_path,
    wrapper_path,
    fixture_path,
    route_test_path,
    runtime_test_path,
) = map(Path, sys.argv[1:])

leaf = leaf_path.read_text(encoding="utf-8")
route_spec = route_spec_path.read_text(encoding="utf-8")
capability = capability_path.read_text(encoding="utf-8")
backend_owner = backend_owner_path.read_text(encoding="utf-8")
contract_refresh = contract_refresh_path.read_text(encoding="utf-8")
extern_builder = extern_builder_path.read_text(encoding="utf-8")
provider_lane = provider_lane_path.read_text(encoding="utf-8")
runtime_direct = runtime_direct_path.read_text(encoding="utf-8")
wrapper = wrapper_path.read_text(encoding="utf-8")
fixture = fixture_path.read_text(encoding="utf-8")
route_test = route_test_path.read_text(encoding="utf-8")
runtime_test = runtime_test_path.read_text(encoding="utf-8")
capability_production = capability.split("#[cfg(test)]", 1)[0]

for needle in (
    "HakoAnalysisDecodedUtf8ByteLenV0",
    'route_id: "extern.hako.analysis.decoded_utf8_byte_len_v0"',
    'symbol: "hako.analysis.decoded_utf8_byte_len_v0"',
    "aliases: &[]",
    'value_demand: "string_handle"',
):
    if needle not in route_spec:
        raise SystemExit(f"missing closed extern route contract: {needle}")

for needle in (
    "ExternCallRouteKind::HakoAnalysisDecodedUtf8ByteLenV0",
    "route.kind()",
    "DECODED_UTF8_BYTE_LEN_BACKEND_UNSUPPORTED_TAG",
):
    if needle not in capability:
        raise SystemExit(f"missing metadata-only backend capability contract: {needle}")

for forbidden in ("MirInstruction", "Callee::Extern", "source_symbol()"):
    if forbidden in capability_production:
        raise SystemExit(f"backend capability must not raw-scan extern calls: {forbidden}")

for needle in (
    "enforce_decoded_utf8_byte_len_backend_supported",
    "ContractRefreshBoundary::BackendPreflight",
    "refresh_module_extern_call_routes(module)",
    "hako.analysis.decoded_utf8_byte_len_v0",
    "MirType::Integer",
    "DecodedUtf8ByteLenV0::count",
    'externcall "hako.analysis.decoded_utf8_byte_len_v0"(value)',
):
    corpus = "\n".join(
        (backend_owner, contract_refresh, extern_builder, provider_lane, runtime_direct, wrapper)
    )
    if needle not in corpus:
        raise SystemExit(f"missing capability execution/preflight seam: {needle}")

for forbidden in (
    "String.length",
    "String.len",
    "String.size",
    "nyash.string.len_h",
    "nyash.any.length_h",
    "nyrt_string_length",
    "strlen",
    "CStr",
    "hostbridge",
    "hako.intrin",
):
    if forbidden in "\n".join((runtime_direct, wrapper)):
        raise SystemExit(f"capability route depends on forbidden length/bridge surface: {forbidden}")

for needle in ("猫", "😸", "猫😸", "DecodedUtf8ByteLenV0Box.count"):
    if needle not in fixture:
        raise SystemExit(f"missing HHako capability fixture: {needle}")
for needle in ("embedded_nul", '"a\\0b"', '"e\\u{0301}"'):
    if needle not in runtime_test:
        raise SystemExit(f"missing runtime direct byte fixture: {needle}")
for needle in ("route.kind()", "string_handle", "analysis.decoded_utf8_byte_len_v0"):
    if needle not in route_test:
        raise SystemExit(f"missing route metadata fixture: {needle}")

for path in (
    leaf_path,
    route_spec_path,
    capability_path,
    backend_owner_path,
    contract_refresh_path,
    extern_builder_path,
    provider_lane_path,
    runtime_direct_path,
    wrapper_path,
    fixture_path,
    route_test_path,
    runtime_test_path,
):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")

print("output_contract=DecodedUtf8ByteLenV0Capability")
print("hako_direct_extern=1")
print("backend_preflight_metadata_only=1")
print("product_backend_fallback=0")
print("public_string_surface=0")
print("embedded_nul=1")
print("environment_independent=1")
print("summary=ok")
PY
