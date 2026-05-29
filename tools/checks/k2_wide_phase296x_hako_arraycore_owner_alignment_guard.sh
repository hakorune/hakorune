#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-arraycore-owner-alignment"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_378="docs/development/current/main/phases/phase-296x/296x-378-ARRAY-REPR-DESIGN-ROW.md"
CARD_379="docs/development/current/main/phases/phase-296x/296x-379-HAKO-ARRAYCORE-OWNER-ALIGNMENT.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_arraycore_owner_alignment_guard.sh"

echo "[$TAG] checking .hako ArrayCore owner alignment"

guard_require_files "$TAG" "$CARD_378" "$CARD_379" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_378" "row378 ArrayRepr design row must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_379" "row379 ArrayCore alignment row must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-arraycore-owner-alignment-note-v0' "$CARD_379" "row379 must define the owner-alignment output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=array-repr-ssot-v0' "$CARD_379" "row379 must consume the ArrayRepr SSOT"
guard_expect_fixed_in_file "$TAG" 'hako_arraycore_visible_semantics_owner=1' "$CARD_379" "row379 must keep .hako ArrayCore as visible semantics owner"
guard_expect_fixed_in_file "$TAG" 'stage0_rust_arrayseed_bootstrap_keep=1' "$CARD_379" "row379 must keep stage0 Rust ArraySeed as bootstrap keep"
guard_expect_fixed_in_file "$TAG" 'directarray_family_storage_substrate=1' "$CARD_379" "row379 must keep DirectArray family as storage substrate"
guard_expect_fixed_in_file "$TAG" 'arraybox_public_facade=1' "$CARD_379" "row379 must keep ArrayBox as public facade"
guard_expect_fixed_in_file "$TAG" 'no_collection_semantic_migration=1' "$CARD_379" "row379 must forbid collection semantic migration"
guard_expect_fixed_in_file "$TAG" 'no_rust_private_layout_exposure=1' "$CARD_379" "row379 must forbid Rust/private layout exposure"
guard_expect_fixed_in_file "$TAG" 'selected_next=directarray_family_extension_gate_row' "$CARD_379" "row379 must point to the extension gate row"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD-296X-001"' "$STATE" "current state must point to row381"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-380-DIRECTARRAY-FAMILY-EXTENSION-GATE"' "$STATE" "current state must keep row380 as the latest landed card"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
