#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-record-decl-metadata-transport"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-209-C203A-RECORD-DECL-METADATA-TRANSPORT.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
MIR_TYPES="src/mir/function/types.rs"
MIR_FUNCTION="src/mir/function.rs"
COMP_CTX="src/mir/builder/compilation_context.rs"
DECL_INDEXER="src/mir/builder/declaration_indexer.rs"
MODULE_LIFECYCLE="src/mir/builder/module_lifecycle.rs"
PROGRAM_JSON_AUTHORITY="src/stage1/program_json_v0/authority.rs"
PROGRAM_JSON_TESTS="src/stage1/program_json_v0/tests/basics_and_enums.rs"
BRIDGE_AST="src/runner/json_v0_bridge/ast.rs"
BRIDGE_LOWERING="src/runner/json_v0_bridge/lowering.rs"
BRIDGE_TESTS="src/runner/json_v0_bridge/tests.rs"
MIR_JSON_DECLS="src/runner/mir_json_emit/decls.rs"
MIR_JSON_ROOT="src/runner/mir_json_emit/root.rs"
MIR_JSON_TESTS="src/runner/mir_json_emit/tests/decl_values.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_record_decl_metadata_transport_guard.sh"

echo "[$TAG] checking C203a record declaration metadata transport"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$MIR_TYPES" \
  "$MIR_FUNCTION" \
  "$COMP_CTX" \
  "$DECL_INDEXER" \
  "$MODULE_LIFECYCLE" \
  "$PROGRAM_JSON_AUTHORITY" \
  "$PROGRAM_JSON_TESTS" \
  "$BRIDGE_AST" \
  "$BRIDGE_LOWERING" \
  "$BRIDGE_TESTS" \
  "$MIR_JSON_DECLS" \
  "$MIR_JSON_ROOT" \
  "$MIR_JSON_TESTS" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C203a card must be complete"
guard_expect_in_file "$TAG" 'C203a status:' "$PLAN" "mimalloc plan must record C203a status"
guard_expect_in_file "$TAG" '`C203a` is complete as `293x-209`' "$RECORD_SSOT" "record SSOT must mark C203a complete"
guard_expect_in_file "$TAG" 'pub struct RecordDecl' "$MIR_TYPES" "MIR metadata must define RecordDecl"
guard_expect_in_file "$TAG" 'pub record_decls: BTreeMap<String, RecordDecl>' "$MIR_TYPES" "MIR metadata must carry record_decls"
guard_expect_in_file "$TAG" 'RecordDecl' "$MIR_FUNCTION" "MIR function module must re-export RecordDecl"
guard_expect_in_file "$TAG" 'register_record_decl' "$COMP_CTX" "compilation context must register record declarations separately"
guard_expect_in_file "$TAG" 'register_record_decl' "$DECL_INDEXER" "declaration indexer must route records to record metadata"
guard_expect_in_file "$TAG" 'record_decls' "$MODULE_LIFECYCLE" "module lifecycle must copy record metadata"
guard_expect_in_file "$TAG" 'collect_record_decls' "$PROGRAM_JSON_AUTHORITY" "Program JSON authority must collect record declarations"
guard_expect_in_file "$TAG" '"record_decls"' "$PROGRAM_JSON_AUTHORITY" "Program JSON authority must emit record_decls"
guard_expect_in_file "$TAG" 'RecordDeclV0' "$BRIDGE_AST" "JSON bridge AST must define RecordDeclV0"
guard_expect_in_file "$TAG" 'record_decls' "$BRIDGE_LOWERING" "JSON bridge lowering must preserve record_decls"
guard_expect_in_file "$TAG" 'collect_sorted_record_decl_values' "$MIR_JSON_DECLS" "MIR JSON decls must expose record decl collector"
guard_expect_in_file "$TAG" '"record_decls"' "$MIR_JSON_ROOT" "MIR JSON root must emit record_decls"
guard_expect_in_file "$TAG" 'source_to_program_json_v0_emits_record_decls_separate_from_user_boxes' "$PROGRAM_JSON_TESTS" "Program JSON tests must cover record_decls separation"
guard_expect_in_file "$TAG" 'parse_json_v0_to_module_preserves_record_decls_metadata_only' "$BRIDGE_TESTS" "JSON bridge tests must cover record metadata only"
guard_expect_in_file "$TAG" 'collect_sorted_record_decl_values_preserves_record_lane' "$MIR_JSON_TESTS" "MIR JSON tests must cover record lane"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C203a guard"

if rg -n 'record_decls|RecordDecl|record layout' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: C203a record metadata matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

cargo test -q collect_sorted_record_decl_values_preserves_record_lane
cargo test -q source_to_program_json_v0_emits_record_decls_separate_from_user_boxes
cargo test -q parse_json_v0_to_module_preserves_record_decls_metadata_only

echo "[$TAG] ok"
