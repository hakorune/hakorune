#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-array-repr-design-ssot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_377="docs/development/current/main/phases/phase-296x/296x-377-ARRAY-SLOT-NATIVEDIRECT-POST-RETIREMENT-PERF-OWNER-REFRESH.md"
CARD_378="docs/development/current/main/phases/phase-296x/296x-378-ARRAY-REPR-DESIGN-ROW.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DESIGN="docs/development/current/main/design/array-repr-ssot.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_array_repr_design_ssot_guard.sh"

echo "[$TAG] checking ArrayRepr design SSOT"

guard_require_files "$TAG" "$CARD_377" "$CARD_378" "$STATE" "$INDEX" "$DESIGN" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_377" "row377 perf owner refresh must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_378" "row378 ArrayRepr design row must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=array-repr-ssot-v0' "$CARD_378" "row378 must define the ArrayRepr SSOT output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=array-slot-nativedirect-post-retirement-perf-owner-refresh-v0' "$CARD_378" "row378 must consume row377"
guard_expect_fixed_in_file "$TAG" 'array_repr_ssot_path=docs/development/current/main/design/array-repr-ssot.md' "$CARD_378" "row378 must point at the ArrayRepr SSOT"
guard_expect_fixed_in_file "$TAG" 'array_repr_variants=DirectI64|PublicArrayBoxFallback' "$CARD_378" "row378 must pin the ArrayRepr variants"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_facade=1' "$CARD_378" "row378 must keep ArrayBox as public facade"
guard_expect_fixed_in_file "$TAG" 'directarray_family_storage_substrate=1' "$CARD_378" "row378 must keep DirectArray family as storage substrate"
guard_expect_fixed_in_file "$TAG" 'materialization_route=explicit' "$CARD_378" "row378 must require an explicit materialization route"
guard_expect_fixed_in_file "$TAG" 'nyash_array_birth_h_behavior_change=0' "$CARD_378" "row378 must not change nyash.array.birth_h"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_hako_arraycore_owner_alignment_row' "$CARD_378" "row378 must point at the next owner-alignment row"
guard_expect_fixed_in_file "$TAG" 'Status: Active' "$DESIGN" "ArrayRepr SSOT must be active"
guard_expect_fixed_in_file "$TAG" 'DirectI64' "$DESIGN" "ArrayRepr SSOT must name DirectI64"
guard_expect_fixed_in_file "$TAG" 'PublicArrayBoxFallback' "$DESIGN" "ArrayRepr SSOT must name PublicArrayBoxFallback"
guard_expect_fixed_in_file "$TAG" 'ArrayBox:' "$DESIGN" "ArrayRepr SSOT must describe ArrayBox ownership"
guard_expect_fixed_in_file "$TAG" 'DirectArray family:' "$DESIGN" "ArrayRepr SSOT must describe DirectArray ownership"
guard_expect_fixed_in_file "$TAG" 'Materialization Route' "$DESIGN" "ArrayRepr SSOT must define materialization"
guard_expect_fixed_in_file "$TAG" 'no `nyash.array.birth_h` behavior change' "$DESIGN" "ArrayRepr SSOT must forbid birth-h behavior change"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD-296X-001"' "$STATE" "current state must point to row381"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-380-DIRECTARRAY-FAMILY-EXTENSION-GATE"' "$STATE" "current state must keep row380 as latest landed card"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
