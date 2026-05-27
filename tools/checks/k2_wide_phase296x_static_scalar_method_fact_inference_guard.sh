#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-static-scalar-method-fact-inference"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_136="docs/development/current/main/phases/phase-296x/296x-136-STATIC-SCALAR-METHOD-FACT-INFERENCE.md"
CARD_137="docs/development/current/main/phases/phase-296x/296x-137-STATIC-SCALAR-CALL-LOWERING-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SOURCE="lang/src/hako_alloc/memory/object_lifecycle_facade_reason_box.hako"
SELECTION_TOOL="tools/allocator/static_scalar_method_fact_selection.py"
TOOL="tools/allocator/static_scalar_method_fact_inference.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_static_scalar_method_fact_inference_guard.sh"
RUST_FACTS="src/mir/builder/static_scalar_facts.rs"
RUST_COMP_CTX="src/mir/builder/compilation_context.rs"
RUST_INDEXER="src/mir/builder/declaration_indexer.rs"

echo "[$TAG] checking static scalar method fact inference"

guard_require_files "$TAG" "$CARD_136" "$CARD_137" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SOURCE" "$SELECTION_TOOL" "$TOOL" "$SELF_SCRIPT" "$RUST_FACTS" "$RUST_COMP_CTX" "$RUST_INDEXER"
guard_require_exec_files "$TAG" "$SELECTION_TOOL" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_136" "row136 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_137" "row137 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=static-scalar-method-fact-inference-v0' "$CARD_136" "row136 must record output contract"
guard_expect_fixed_in_file "$TAG" 'fact_family=object_lifecycle_facade_reason_zero_arg_return_literal_i64' "$CARD_136" "row136 must record fact family"
guard_expect_fixed_in_file "$TAG" 'verified_fact_count=19' "$CARD_136" "row136 must record verified fact count"
guard_expect_fixed_in_file "$TAG" 'const_lowering=0' "$CARD_136" "row136 must keep const lowering closed"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-136-STATIC-SCALAR-METHOD-FACT-INFERENCE"' "$CURRENT_STATE" "current state latest card must advance to row136"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "STATIC-SCALAR-CALL-LOWERING-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row137"
guard_expect_fixed_in_file "$TAG" '| 136 | `STATIC-SCALAR-METHOD-FACT-INFERENCE-296X-001` | Landed |' "$TASKBOARD" "taskboard row136 must be landed"
guard_expect_fixed_in_file "$TAG" '| 137 | `STATIC-SCALAR-CALL-LOWERING-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row137 must be current"
guard_expect_fixed_in_file "$TAG" 'infer_static_scalar_method_fact' "$RUST_FACTS" "Rust fact verifier must exist"
guard_expect_fixed_in_file "$TAG" 'static_scalar_method_facts' "$RUST_COMP_CTX" "compilation context must store facts"
guard_expect_fixed_in_file "$TAG" 'register_static_scalar_method_fact_if_verified' "$RUST_INDEXER" "declaration indexer must register verified selected facts"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

cargo test -q static_scalar

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_static_scalar_inference.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
python3 "$TOOL" --selection-tool "$SELECTION_TOOL" --source "$SOURCE" --out "$tmp_dir/report.out"

guard_expect_fixed_in_file "$TAG" 'output_contract=static-scalar-method-fact-inference-v0' "$tmp_dir/report.out" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=static-scalar-method-fact-selection-v0' "$tmp_dir/report.out" "tool must record input contract"
guard_expect_fixed_in_file "$TAG" 'candidate_count=19' "$tmp_dir/report.out" "tool must record candidate count"
guard_expect_fixed_in_file "$TAG" 'verified_fact_count=19' "$tmp_dir/report.out" "tool must record verified fact count"
guard_expect_fixed_in_file "$TAG" 'unverified_count=0' "$tmp_dir/report.out" "tool must record unverified count"
guard_expect_fixed_in_file "$TAG" 'const_lowering=0' "$tmp_dir/report.out" "tool must keep lowering closed"
guard_expect_fixed_in_file "$TAG" 'selected_next=static_scalar_call_lowering_selection' "$tmp_dir/report.out" "tool must select lowering selection"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$tmp_dir/report.out" "tool must end ok"

echo "[$TAG] ok"
