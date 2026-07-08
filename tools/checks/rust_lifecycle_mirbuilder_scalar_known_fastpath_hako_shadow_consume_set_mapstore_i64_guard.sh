#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3348-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001.md"
SHADOW_RS="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_RS="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
HAKO="$ROOT/lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako"

python3 - "$STATE" "$CARD" "$SHADOW_RS" "$WRITE_RS" "$HAKO" <<'PY'
import sys
import tomllib
from pathlib import Path

state_path, card_path, shadow_path, write_path, hako_path = map(Path, sys.argv[1:])
state = tomllib.loads(state_path.read_text())
card = card_path.read_text()
shadow = shadow_path.read_text()
write = write_path.read_text()
hako = hako_path.read_text()

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-MISMATCH-GUARD-EXPANSION-001"

need(state.get("latest_card") == token, "latest card drift")
need(state.get("latest_card_path", "").endswith(card_path.name), "latest path drift")
need(state.get("current_blocker_token") == next_card, "current blocker drift")
need(token in card, "card token missing")
need(next_card in card, "selected next missing")

need('include_str!("../../../lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako")' in shadow, "hako artifact not included")
need("mapstore_i64_shadow_consumed_decision" in shadow, "shadow decision entry missing")
need("candidate_scalar_known_surfaces" in shadow, "Rust contract boundary not consumed")
need("ScalarKnownSurfaceId::WriteScalarI64Routes" in shadow, "Write contract boundary missing")
need("GenericMethodRouteKind::MapStoreI64" in shadow, "MapStoreI64 route check missing")
need("CoreMethodOp::MapSet.as_manifest_name()" in shadow, "MapSet check missing")
need('"classifier_policy_mirror_only"' in shadow, "mirror role check missing")
need("GenericMethodPublicationPolicy::NoPublication.as_metadata_name()" in shadow, "publication non-authority check missing")

need("scalar_known_hako_shadow::mapstore_i64_shadow_consumed_decision()" in write, "write_routes does not consume shadow")
need("route_kind == GenericMethodRouteKind::MapStoreI64" in write, "MapStoreI64 fast path branch missing")

expected_row = "map_store_i64_set_surface|SetSurfacePolicy|MapStoreI64|MapSet|ColdFallback|NoneResult|None|WriteAny|ScalarI64|NonePublication|mutate|MutatesReceiverOrContainer|classifier_policy_mirror_only"
need(expected_row in hako, "hako classifier row drift")

non_claims = [
    "hako_runtime_route_authority = 0",
    "hako_backend_lowering_authority = 0",
    "route_selection_authority = 0",
    "runtime_mutation_authority = 0",
    "publication_execution = 0",
    "runtime_fallback = 0",
    "new_backend_route = 0",
    "new_abi = 0",
    "source_selfhost_claim = 0",
]
for claim in non_claims:
    need(claim in card, f"missing non-claim: {claim}")

print("hako_artifact_fastpath_shadow_consumed=1")
print("surface=SetSurfacePolicy/MapStoreI64")
print("rust_fastpath_decision_observed=1")
print("hako_policy_result_observed=1")
print("rust_hako_policy_match=1")
print("mismatch_policy=fail_fast_guard")
print("rust_authority_retained=1")
print("source_selfhost_claim=0")
PY

cargo check -q --lib
cargo test -q --lib mapstore_i64_shadow_artifact_matches_rust_fastpath_policy
cargo test -q --lib records_direct_array_and_map_set_core_method_routes

cat <<'EOF'
output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-shadow-consume-set-mapstore-i64
token=MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001
hako_artifact_fastpath_shadow_consumed=1
surface=SetSurfacePolicy/MapStoreI64
rust_fastpath_decision_observed=1
hako_policy_result_observed=1
rust_hako_policy_match=1
mismatch_policy=fail_fast_guard
rust_authority_retained=1
hako_runtime_route_authority=0
hako_backend_lowering_authority=0
route_selection_authority=0
runtime_mutation_authority=0
publication_execution=0
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-MISMATCH-GUARD-EXPANSION-001
summary=ok
EOF
