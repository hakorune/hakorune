#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3349-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-MISMATCH-GUARD-EXPANSION-001.md"
SHADOW_RS="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

python3 - "$STATE" "$CARD" "$SHADOW_RS" <<'PY'
import sys
import tomllib
from pathlib import Path

state_path, card_path, shadow_path = map(Path, sys.argv[1:])
state = tomllib.loads(state_path.read_text())
card = card_path.read_text()
shadow = shadow_path.read_text()

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-MISMATCH-GUARD-EXPANSION-001"
next_card = "MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-SELECTION-001"

need(state.get("latest_card") == token, "latest card drift")
need(state.get("latest_card_path", "").endswith(card_path.name), "latest path drift")
need(state.get("current_blocker_token") == next_card, "current blocker drift")
need(token in card, "card token missing")
need(next_card in card, "selected next missing")

for needle in [
    "parse_hako_mapstore_i64_policy_row",
    "mapstore_i64_shadow_rejects_route_kind_mismatch",
    "mapstore_i64_shadow_rejects_core_op_mismatch",
    "mapstore_i64_shadow_rejects_role_mismatch",
    "#[should_panic",
]:
    need(needle in shadow, f"missing mismatch guard fixture: {needle}")

for claim in [
    "hako_shadow_mismatch_guard_expanded = 1",
    "route_kind_mismatch_rejected = 1",
    "core_op_mismatch_rejected = 1",
    "role_mismatch_rejected = 1",
    "rust_authority_retained = 1",
]:
    need(claim in card, f"missing claim: {claim}")

for non_claim in [
    "hako_runtime_route_authority = 0",
    "hako_backend_lowering_authority = 0",
    "route_selection_authority = 0",
    "runtime_mutation_authority = 0",
    "publication_execution = 0",
    "runtime_fallback = 0",
    "new_backend_route = 0",
    "new_abi = 0",
    "source_selfhost_claim = 0",
]:
    need(non_claim in card, f"missing non-claim: {non_claim}")

print("hako_shadow_mismatch_guard_expanded=1")
print("route_kind_mismatch_rejected=1")
print("core_op_mismatch_rejected=1")
print("role_mismatch_rejected=1")
print("rust_authority_retained=1")
print("source_selfhost_claim=0")
PY

cargo check -q --lib
cargo test -q --lib mapstore_i64_shadow_artifact_matches_rust_fastpath_policy
cargo test -q --lib mapstore_i64_shadow_rejects

cat <<'EOF'
output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-shadow-consume-mismatch-guard-expansion
token=MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-MISMATCH-GUARD-EXPANSION-001
hako_shadow_mismatch_guard_expanded=1
route_kind_mismatch_rejected=1
core_op_mismatch_rejected=1
role_mismatch_rejected=1
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
selected_next_card=MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-SELECTION-001
summary=ok
EOF
