#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-physical-input-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

EVIDENCE="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/physical_evidence.rs"
INPUT="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/physical_input.rs"
EXIT_TX="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/exit_transaction.rs"
COSEAL_TESTS="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/tests.rs"
DEMAND_MOD="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/physical_demand/mod.rs"
DEMAND_MODEL="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/physical_demand/model.rs"
DEMAND_ISSUER="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/physical_demand/issuer.rs"
APRIME_MODEL="$ROOT_DIR/src/mir/compiler/a_prime_i64_physical_capability/model.rs"
APRIME_ISSUER="$ROOT_DIR/src/mir/compiler/a_prime_i64_physical_capability/issuer.rs"
APRIME_SOURCE="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/a_prime_source.rs"
CATALOG_ADMISSION="$ROOT_DIR/src/mir/builder/normal_cataloged_box_method_admission.rs"
PACKAGE_INSTALL="$ROOT_DIR/src/mir/normal_callable_semantic_package/install.rs"
PACKAGE_LOWERING_PORT="$ROOT_DIR/src/mir/normal_callable_semantic_package/install/lowering_port.rs"
PACKAGE_LOAN="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port.rs"
SELECTED_ABI="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_abi.rs"
SELECTED_CAPABILITY="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_capability.rs"
SELECTED_EMITTER="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/mod.rs"
SELECTED_SESSION_OPEN="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/session_open.rs"
SELECTED_STORAGE_POLICY="$ROOT_DIR/src/mir/policies/a_prime_i64_callable_storage_layout.rs"
SELECTED_STORAGE_COSEAL="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/a_prime_callable_storage_layout.rs"
TARGET_CAPABILITY="$ROOT_DIR/src/mir/compiler/target_capability.rs"
BINDING="$ROOT_DIR/src/mir/builder/pinned_text_invocation_binding.rs"
SELECTED_ASSEMBLY="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/assembly.rs"
ROOT_LIFECYCLE="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
ROOT_LOWERING="$ROOT_DIR/src/mir/builder/program_root_lowering.rs"
CALL_METADATA="$ROOT_DIR/src/box_callable/provider_admission/call_metadata.rs"
CALL_METADATA_TESTS="$ROOT_DIR/src/box_callable/provider_admission/call_metadata_tests.rs"
POLICIES_MOD="$ROOT_DIR/src/mir/policies/mod.rs"
SELECTED_TARGETS="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/targets.rs"
SELECTED_FORMAL_HEADER="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/formal_header.rs"
SELECTED_VALUE_LEDGER="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/value_ledger.rs"
SELECTED_OPERATION_CURSOR="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/operation_cursor.rs"
FORMAL_REPRESENTATION="$ROOT_DIR/src/mir/a_prime_i64_formal_representation.rs"
FORMAL_CAPABILITY="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_capability/formal_representation.rs"
SKELETON_BUILDER="$ROOT_DIR/src/mir/builder/calls/skeleton_builder.rs"
CANONICAL_SESSION="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/session.rs"
WIRE_RS="$ROOT_DIR/src/abi/dynamic_call_slot_wire.rs"
WIRE_PY="$ROOT_DIR/src/llvm_py/builders/dynamic_v2_callslot_wire.py"
WIRE_C="$ROOT_DIR/include/nyrt_dynamic_call_slot_v2.h"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$EVIDENCE" "$INPUT" "$EXIT_TX" "$COSEAL_TESTS" \
  "$DEMAND_MOD" "$DEMAND_MODEL" "$DEMAND_ISSUER" "$APRIME_SOURCE" "$SELECTED_ABI" \
  "$SELECTED_CAPABILITY" "$SELECTED_EMITTER" "$SELECTED_SESSION_OPEN" "$SELECTED_TARGETS" "$SELECTED_VALUE_LEDGER" "$SKELETON_BUILDER" "$CANONICAL_SESSION" "$APRIME_MODEL" \
  "$APRIME_ISSUER" "$CATALOG_ADMISSION" "$PACKAGE_INSTALL" "$PACKAGE_LOWERING_PORT" "$PACKAGE_LOAN" "$SELECTED_FORMAL_HEADER" \
  "$SELECTED_OPERATION_CURSOR" "$SELECTED_STORAGE_POLICY" "$SELECTED_STORAGE_COSEAL" \
  "$CALL_METADATA" "$CALL_METADATA_TESTS" "$POLICIES_MOD" "$TARGET_CAPABILITY" \
  "$BINDING" "$SELECTED_ASSEMBLY" "$ROOT_LIFECYCLE" "$ROOT_LOWERING" \
  "$WIRE_RS" "$WIRE_PY" "$WIRE_C" "$FORMAL_REPRESENTATION" "$FORMAL_CAPABILITY"

guard_expect_fixed_in_file "$TAG" \
  "DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2: usize = 17" "$EVIDENCE" \
  "physical evidence must retain the exact bounded item coverage"
guard_expect_fixed_in_file "$TAG" \
  "DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2: usize = 15" "$EVIDENCE" \
  "physical evidence must retain the exact bounded operation coverage"
guard_expect_fixed_in_file "$TAG" \
  "issue_physical_evidence_v2" "$EVIDENCE" \
  "the source/effect ledger must have one envelope-owned issuer"
guard_expect_fixed_in_file "$TAG" \
  "with_physical_input" "$EXIT_TX" \
  "the final exit transaction must be the sole physical-input ingress"
guard_expect_fixed_in_file "$TAG" \
  "physical_evidence_coseals_exact_placement_operation_and_effect_coverage" "$COSEAL_TESTS" \
  "exact physical evidence coverage test is missing"
guard_expect_fixed_in_file "$TAG" \
  "VerifiedDynamicLoopOperationPhysicalDemandV2" "$DEMAND_MODEL" \
  "Dynamic physical demand must remain a distinct V2 product"
guard_expect_fixed_in_file "$TAG" \
  "physical_demand_consumes_the_complete_view_inside_the_htrb_loan" "$EXIT_TX" \
  "whole-program Dynamic demand HRTB test is missing"
for formal_fact in \
  "src_binding" "pos_binding" "end_binding" "pred_chars_binding" \
  "src_class" "pos_class" "end_class" "pred_chars_class" \
  "DynamicFullBodyBindingRoleV1::Src" "Value(value(0))" \
  "DynamicFullBodyBindingRoleV1::PredChars" "Value(value(3))"; do
  guard_expect_fixed_in_file "$TAG" "$formal_fact" "$APRIME_SOURCE" \
    "A-prime source relation is missing exact formal-lane fact: $formal_fact"
done

INPUT_PRODUCTION="$(mktemp "${TMPDIR:-/tmp}/dynamic-v2-physical-input.XXXXXX")"
sed '/^#\[cfg(test)\]/,$d' "$INPUT" >"$INPUT_PRODUCTION"
trap 'rm -f "$INPUT_PRODUCTION"' EXIT

for forbidden in \
  "as_sig(" \
  "as_recipe(" \
  "LoopItemKeyV1::new(" \
  "VerifiedLoopJoinSigV2" \
  "LoopRecipeItemV2"; do
  if rg -F -q -- "$forbidden" "$INPUT_PRODUCTION"; then
    guard_fail "$TAG" "physical-input view contains forbidden raw/reconstructed authority: $forbidden"
  fi
done

for forbidden in \
  "VerifiedLoopOperationPhysicalDemandV1" \
  "LoopOperationPhysicalDemandV1" \
  "as_sig(" \
  "as_recipe(" \
  "BasicBlockId" \
  "ValueId" \
  "MirBuilder" \
  "operation_row("; do
  if rg -F -q -- "$forbidden" "$DEMAND_MOD" "$DEMAND_MODEL" "$DEMAND_ISSUER"; then
    guard_fail "$TAG" "Dynamic demand contains forbidden V1/physical/single-item authority: $forbidden"
  fi
done

for file in "$EVIDENCE" "$INPUT" "$EXIT_TX" "$DEMAND_MOD" "$DEMAND_MODEL" "$DEMAND_ISSUER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

LEDGER_HEADER="$(rg -n -B3 -- "struct DynamicV2NativePreflightLedgerV1" "$SELECTED_ABI")"
if printf '%s\n' "$LEDGER_HEADER" | rg -q -- "Clone"; then
  guard_fail "$TAG" "the move-only V2 preflight ledger must not derive Clone"
fi
for file in "$SELECTED_ABI" "$SELECTED_EMITTER"; do
  if rg -F -q -- "ledger.clone(" "$file"; then
    guard_fail "$TAG" "V2 preflight ledger was copied or split: ${file#"$ROOT_DIR/"}"
  fi
done

# A borrowed ledger view is test evidence only.  The first live session handoff
# must consume the plan and move the ledger; keeping this API production-visible
# would reopen a second emission authority before that handoff exists.
SELECTED_ABI_PRODUCTION="$(sed '/^[[:space:]]*#\[cfg(test)\]/,$d' "$SELECTED_ABI")"
if printf '%s\n' "$SELECTED_ABI_PRODUCTION" | rg -q -- "with_ledger"; then
  guard_fail "$TAG" "preflight ledger borrow escaped the test-only boundary"
fi

guard_expect_fixed_in_file "$TAG" "DynamicV2CallOutV1" "$WIRE_RS" \
  "I0-A Rust CallSlot wire schema is missing"
guard_expect_fixed_in_file "$TAG" "HakoDynamicV2CallOutV1" "$WIRE_C" \
  "I0-A C CallSlot wire schema is missing"
guard_expect_fixed_in_file "$TAG" "DynamicV2CallOutV1" "$WIRE_PY" \
  "I0-A Python/LLVM schema mirror is missing"

# R0 canonical-session projection: the selected emitter must consume the
# package-backed input and lend the final Dynamic authority internally.  The
# canary may not re-verify Completion/If control or accept an externally
# paired session.
guard_expect_fixed_in_file "$TAG" "with_canonical_session_authority" "$APRIME_MODEL" \
  "selected demand must expose only the scoped final-program authority view"
guard_expect_fixed_in_file "$TAG" "new_selected_dynamic" "$SELECTED_SESSION_OPEN" \
  "selected emitter must construct the Dynamic canonical session itself"
if rg -n -A6 -- "fn new_selected_dynamic" "$CANONICAL_SESSION" | rg -q -- "block_expr_count"; then
  guard_fail "$TAG" "Dynamic canonical semantic block count must not be selected by the physical emitter"
fi
guard_expect_fixed_in_file "$TAG" "NormalCatalogedBoxMethodDraftAdmissionV1" "$APRIME_MODEL" \
  "A-prime demand must own the single catalog-backed physical-header admission"
guard_expect_fixed_in_file "$TAG" "physical_header" "$APRIME_MODEL" \
  "selected physical session must consume the demand-owned header projection"
guard_expect_fixed_in_file "$TAG" "function_effects" "$APRIME_MODEL" \
  "the physical header must retain the verified effect projection"
guard_expect_fixed_in_file "$TAG" "SelectedCatalogedCallableLoweringInputV1" "$PACKAGE_INSTALL" \
  "catalog admission must travel with the selected package loan"
guard_expect_fixed_in_file "$TAG" "CatalogedBoxMethodPhysicalHeaderProjectionV1" "$CATALOG_ADMISSION" \
  "catalog must issue the bounded physical-header projection"
guard_expect_fixed_in_file "$TAG" "dynamic_physical_header" "$PACKAGE_INSTALL" \
  "the installed package must retain the linear catalog-header projection"
guard_expect_fixed_in_file "$TAG" "take_dynamic_physical_header" "$PACKAGE_INSTALL" \
  "the package loan must consume the catalog-header projection exactly once"
guard_expect_fixed_in_file "$TAG" "with_selected_cataloged_lowering_input" "$PACKAGE_LOWERING_PORT" \
  "package must expose one exactly-once cataloged admission loan"
guard_expect_fixed_in_file "$TAG" "with_cataloged_callable_source_scope" "$PACKAGE_LOAN" \
  "Builder adapter must forward the existing catalog admission"
if rg -F -q -- "NormalCatalogedBoxMethodDraftAdmissionV1::seal" "$APRIME_ISSUER"; then
  guard_fail "$TAG" "A-prime issuer must not re-seal a catalog physical header"
fi
if rg -F -q -- "format!(" "$APRIME_ISSUER"; then
  guard_fail "$TAG" "A-prime issuer must not reconstruct the catalog physical symbol"
fi
for forbidden in \
  "use crate::ast::ASTNode" \
  "ASTNode::FunctionDeclaration" \
  "input.source().source().root()" \
  "source().source().root()"; do
  if rg -F -q -- "$forbidden" "$APRIME_ISSUER"; then
    guard_fail "$TAG" "A-prime issuer must not re-observe raw AST/header authority: $forbidden"
  fi
done
guard_expect_fixed_in_file "$TAG" "physical_function_effects" "$DEMAND_MODEL" \
  "the operation/effect plan must be the selected function-effect projection source"
if rg -F -q -- "NormalCatalogedBoxMethodDraftAdmissionV1::seal" "$SELECTED_EMITTER"; then
  guard_fail "$TAG" "selected emitter must not re-seal the catalog physical header"
fi
for contract in \
  "CoreMethodOp::StringSubstring" \
  "CoreMethodResultKindV1::StringValue" \
  "CoreMethodEffectV1::PureRead" \
  "CoreMethodOp::StringIndexOf" \
  "CoreMethodResultKindV1::I64Value" \
  "CoreMethodEffectV1::PureRead" \
  "DynamicV2PhysicalRepresentationV1::ImmediateI64"; do
  guard_expect_fixed_in_file "$TAG" "$contract" "$SELECTED_CAPABILITY" \
    "selected physical capability must consume the generated I7 contract and exact representation: $contract"
done
if rg -F -q -- "DynamicV2ProducerLaneV1" "$SELECTED_CAPABILITY" || \
   rg -F -q -- ".lane()" "$SELECTED_CAPABILITY"; then
  guard_fail "$TAG" "producer family and physical representation must not be collapsed into a legacy lane"
fi
guard_expect_fixed_in_file "$TAG" "create_resolved_function_skeleton" "$SELECTED_SESSION_OPEN" \
  "canonical selected skeleton must consume an exact header without body inference"
guard_expect_fixed_in_file "$TAG" "physical_header.effects()" "$SELECTED_SESSION_OPEN" \
  "canonical selected skeleton must consume the demand-owned physical-header effect projection"
if rg -n -A35 -- "fn create_resolved_function_skeleton" "$SKELETON_BUILDER" \
  | rg -q -- "EffectMask::READ|Effect::ReadHeap"; then
  guard_fail "$TAG" "canonical skeleton must not hardcode its function effect"
fi
guard_expect_fixed_in_file "$TAG" "DynamicV2PhysicalBlockTargetV1" "$SELECTED_ABI" \
  "selected schedule must carry an explicit logical-to-physical block target"
for target in Header BodyPrelude ThenTerminal Continuation After; do
  guard_expect_fixed_in_file "$TAG" "${target}" "$SELECTED_ABI" \
    "selected block-target projection is missing: ${target}"
done
for target in Header BodyPrelude ThenTerminal Continuation After; do
  guard_expect_fixed_in_file "$TAG" "${target}" "$SELECTED_TARGETS" \
    "session-private target set is missing role: ${target}"
done
for target_fact in \
  "DynamicV2PhysicalTargetSetV1" \
  "DynamicV2OpaquePhysicalTargetV1" \
  "with_role" \
  "create_unpublished_block" \
  "let enter = canonical.entry_block(builder)?"; do
  guard_expect_fixed_in_file "$TAG" "$target_fact" "$SELECTED_TARGETS" \
    "session-private target ownership is missing: ${target_fact}"
done
if ! rg -n -q -- 'let blocks = \[' "$SELECTED_TARGETS"; then
  guard_fail "$TAG" "session-private target ownership is missing: let blocks = [enter, header, body_prelude, then_terminal, continuation, after]"
fi
for formal_fact in \
  "DynamicV2FormalSeedV1" \
  "adopt_exact_formal_parameter" \
  "claim_variable_use_binding" \
  "emit_jump(function, enter, header)" \
  "read_entry_receipt"; do
  guard_expect_fixed_in_file "$TAG" "$formal_fact" "$SELECTED_FORMAL_HEADER" \
    "formal Enter/Header admission is missing: ${formal_fact}"
done
for ledger_fact in \
  "DynamicV2PhysicalValueLedgerV1" \
  "DynamicV2PhysicalValueLedgerRejectV1" \
  "DynamicV2PhysicalValueViewV1" \
  "pub(super) fn publish" \
  "pub(super) fn with_value"; do
  guard_expect_fixed_in_file "$TAG" "$ledger_fact" "$SELECTED_VALUE_LEDGER" \
    "session-private physical value ledger is missing: ${ledger_fact}"
done
if rg -n -B3 -A1 -- "struct DynamicV2PhysicalValueLedgerV1" "$SELECTED_VALUE_LEDGER" \
  | rg -q -- "Clone" || rg -F -q -- "into_parts" "$SELECTED_VALUE_LEDGER"; then
  guard_fail "$TAG" "session-private physical value ledger must remain move-only"
fi
guard_expect_fixed_in_file "$TAG" "self.values" "$SELECTED_EMITTER" \
  "the I8 canary must retain a session-owned physical value ledger"
guard_expect_fixed_in_file "$TAG" "with_physical_value_for_test" "$SELECTED_EMITTER" \
  "the canary must exercise callback-scoped ledger reads"
for cursor_fact in \
  "DynamicV2RecipeOperationCursorV1" \
  "consume_all" \
  "DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2" \
  "UseBeforeProduce" \
  "DuplicateResult" \
  "CoreMethodOp::StringSubstring" \
  "CoreMethodOp::StringIndexOf"; do
  guard_expect_fixed_in_file "$TAG" "$cursor_fact" "$SELECTED_OPERATION_CURSOR" \
    "V2 Recipe-order cursor is missing required fact: $cursor_fact"
done
guard_expect_fixed_in_file "$TAG" "operation_cursor::validate" "$SELECTED_EMITTER" \
  "selected session must run the exact-once V2 cursor before opening Builder state"

# P3-I0 callable storage policy: one plain owner row is issued only at the
# selected close, then stored on the existing projection.  This is not a
# backend activation or a fourth semantic receipt.
for file in "$SELECTED_STORAGE_POLICY" "$SELECTED_STORAGE_COSEAL" "$CALL_METADATA" "$CALL_METADATA_TESTS"; do
  guard_expect_fixed_in_file "$TAG" "APrimeI64CallableStorageLayoutV1" "$file" \
    "A-prime callable storage-policy owner is missing"
done
guard_expect_fixed_in_file "$TAG" "NonAddressableSsaI64" "$SELECTED_STORAGE_POLICY" \
  "the A-prime storage policy must remain the non-addressable SSA-i64 singleton"
guard_expect_fixed_in_file "$TAG" "a_prime_callable_storage_layout::issue" "$SELECTED_EMITTER" \
  "selected emitter must issue the storage policy at close"
guard_expect_fixed_in_file "$TAG" "callable_storage_layout:" "$CALL_METADATA" \
  "existing AOT projection must carry a non-optional storage policy"
guard_expect_fixed_in_file "$TAG" "fn callable_storage_layout" "$CALL_METADATA" \
  "storage-policy projection accessor is missing"
guard_expect_fixed_in_file "$TAG" "ReceiptFormal" "$SELECTED_STORAGE_COSEAL" \
  "co-seal must reject formal/receipt drift"
guard_expect_fixed_in_file "$TAG" "SessionBrand" "$SELECTED_STORAGE_COSEAL" \
  "co-seal must reject a foreign physical session"
guard_expect_fixed_in_file "$TAG" "projection.callable_storage_layout()" "$CALL_METADATA_TESTS" \
  "positive projection test must observe the storage policy"
for forbidden in "MirType" "StorageClass" "ASTNode" "as_recipe(" "serde"; do
  if rg -F -q -- "$forbidden" "$SELECTED_STORAGE_POLICY" "$SELECTED_STORAGE_COSEAL"; then
    guard_fail "$TAG" "storage policy/co-seal must not infer layout from non-authority: $forbidden"
  fi
done
for forbidden in "APrimeI64CallableStorageLayoutV1" "NonAddressableSsaI64"; do
  if rg -F -q -- "$forbidden" "$ROOT_DIR/src/runner" "$ROOT_DIR/src/llvm_py" "$ROOT_DIR/lang" "$ROOT_DIR/include"; then
    guard_fail "$TAG" "storage policy must not cross into JSON/C/backend activation: $forbidden"
  fi
done
policy_defs="$(rg -n -F -- "enum APrimeI64CallableStorageLayoutV1" "$SELECTED_STORAGE_POLICY" | wc -l | tr -d '[:space:]')"
policy_variants="$(rg -n -F -- "NonAddressableSsaI64" "$SELECTED_STORAGE_POLICY" | wc -l | tr -d '[:space:]')"
if [[ "$policy_defs" != 1 || "$policy_variants" != 1 ]]; then
  guard_fail "$TAG" "storage policy must have exactly one enum definition and one singleton variant"
fi
co_seal_calls="$(rg -n -F -- "a_prime_callable_storage_layout::issue(" "$SELECTED_EMITTER" | wc -l | tr -d '[:space:]')"
if [[ "$co_seal_calls" != 1 ]]; then
  guard_fail "$TAG" "selected emitter must have exactly one storage-policy co-seal call: $co_seal_calls"
fi
python3 - "$SELECTED_EMITTER" <<'PY'
from pathlib import Path
import sys

text = Path(sys.argv[1]).read_text(encoding="utf-8")
co_seal = text.index("a_prime_callable_storage_layout::issue(")
projection = text.index("project_dynamic_v2_aot_call_metadata(")
if co_seal > projection:
    raise SystemExit("storage policy must be co-sealed before projection")
PY

# P3-I0 target row: the existing move-only compile capability is the sole
# issuer.  The selected Dynamic route must retain its live invocation binding
# through assembly and store the typed child on the existing AOT projection;
# no data-layout parsing or target-less fallback is allowed.
guard_expect_fixed_in_file "$TAG" \
  "project_a_prime_i64_target_storage_layout" "$TARGET_CAPABILITY" \
  "the compile-target capability must issue the exact-i64 target child"
guard_expect_fixed_in_file "$TAG" \
  "APrimeI64TargetStorageLayoutV1" "$TARGET_CAPABILITY" \
  "the exact-i64 target child type is missing"
guard_expect_fixed_in_file "$TAG" \
  "target_binding" "$ROOT_LOWERING" \
  "root lowering must preserve the invocation binding"
guard_expect_fixed_in_file "$TAG" \
  "target_binding" "$PACKAGE_LOAN" \
  "the package adapter must retain the invocation binding"
guard_expect_fixed_in_file "$TAG" \
  "target_capability()" "$SELECTED_ASSEMBLY" \
  "selected Dynamic assembly must project the target child from the binding"
guard_expect_fixed_in_file "$TAG" \
  "target_layout" "$SELECTED_ASSEMBLY" \
  "selected Dynamic assembly must pass the typed target row into the session"
guard_expect_fixed_in_file "$TAG" \
  "target_storage_layout:" "$CALL_METADATA" \
  "existing AOT metadata must store the target-bound row"
guard_expect_fixed_in_file "$TAG" \
  "fn target_storage_layout" "$CALL_METADATA" \
  "target-bound metadata accessor is missing"
if rg -F -q -- ".map(|binding| binding.target_capability())" "$ROOT_LIFECYCLE"; then
  guard_fail "$TAG" "root lifecycle stripped the live invocation binding to a bare target"
fi
for forbidden in "data_layout()" "TargetMachine" "StorageClass" "MirType"; do
  if rg -F -q -- "$forbidden" "$SELECTED_STORAGE_COSEAL" "$SELECTED_ASSEMBLY"; then
    guard_fail "$TAG" "exact-i64 target/storage close inferred from forbidden authority: $forbidden"
  fi
done
target_child_issuers="$(rg -n -F -- "pub(crate) const fn project_a_prime_i64_target_storage_layout(" "$TARGET_CAPABILITY" | wc -l | tr -d '[:space:]')"
if [[ "$target_child_issuers" != 1 ]]; then
  guard_fail "$TAG" "target child must have exactly one capability projection method: $target_child_issuers"
fi
for forbidden in \
  "loop_recipe_physicalizer" \
  "MirInstruction" \
  "BasicBlockId" \
  "ValueId" \
  "selector" \
  "lookup"; do
  if rg -F -q -- "$forbidden" "$SELECTED_OPERATION_CURSOR"; then
    guard_fail "$TAG" "V2 operation cursor must not become a second physical/selector authority: $forbidden"
  fi
done
cursor_lines="$(wc -l < "$SELECTED_OPERATION_CURSOR" | tr -d '[:space:]')"
if (( cursor_lines >= 800 )); then
  guard_fail "$TAG" "V2 operation cursor reached hard 800-line boundary: ${SELECTED_OPERATION_CURSOR#"$ROOT_DIR/"} has $cursor_lines"
fi
for file in "$SELECTED_STORAGE_POLICY" "$SELECTED_STORAGE_COSEAL" "$CALL_METADATA" "$CALL_METADATA_TESTS" "$SELECTED_EMITTER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "P3-I0 source reached hard 800-line boundary: $file has $lines"
  fi
done
if rg -n -- "physical_value\(" "$SELECTED_EMITTER" "$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/i64_const.rs"; then
  guard_fail "$TAG" "production emitter must not expose a raw physical ValueId getter"
fi
for file in "$SELECTED_VALUE_LEDGER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "session value ledger reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done
if rg -n -B3 -A3 -- "struct DynamicV2PhysicalTargetSetV1" "$SELECTED_TARGETS" \
  | rg -q -- "Clone"; then
  guard_fail "$TAG" "physical target set must remain move-only"
fi
for forbidden in \
  "DynamicV2OpaqueBodyPreludeTargetV1" \
  "body_prelude_target"; do
  if rg -F -q -- "$forbidden" "$SELECTED_EMITTER" "$SELECTED_TARGETS"; then
    guard_fail "$TAG" "selected emitter retained the old singleton physical target: $forbidden"
  fi
done
for file in "$SELECTED_EMITTER" "$SELECTED_SESSION_OPEN" "$SELECTED_TARGETS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "selected physical target source reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done
selected_emitter_lines="$(wc -l < "$SELECTED_EMITTER" | tr -d '[:space:]')"
if (( selected_emitter_lines >= 760 )); then
  guard_fail "$TAG" "selected physical emitter parent reached the 760-line split boundary: $selected_emitter_lines"
fi
if rg -F -q -- 'format!("{name}/' "$SELECTED_EMITTER"; then
  guard_fail "$TAG" "selected physical symbol was reconstructed from the raw method name"
fi
if rg -F -q -- "create_function_skeleton(" "$SELECTED_EMITTER"; then
  guard_fail "$TAG" "selected canonical emitter still uses the body-aware legacy skeleton"
fi
python3 - "$SELECTED_SESSION_OPEN" <<'PY'
from pathlib import Path
import sys

text = Path(sys.argv[1]).read_text(encoding="utf-8")
authority = text.index("with_canonical_session_authority")
open_session = text.index("open_resolved_function_draft_seal_session_v1")
if authority > open_session:
    raise SystemExit(
        "selected Dynamic authority validation must precede Builder session mutation"
    )
PY
if rg -n -- "verify_function_completion_v1|empty_for_owned_loop_profile" \
  "$SELECTED_EMITTER" "$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/tests.rs"; then
  guard_fail "$TAG" "selected Dynamic canary reissued Completion/If authority"
fi
if ! rg -n -- "begin\(" "$SELECTED_EMITTER" >/dev/null || \
   ! rg -n -- "builder: &'builder mut MirBuilder" "$SELECTED_EMITTER" >/dev/null; then
  guard_fail "$TAG" "selected emitter begin must own the unpublished session handoff"
fi

for pair in \
  "Invalid = 0" \
  "HostHandle = 1" \
  "ImmediateI64 = 2" \
  "Normal = 0" \
  "Fault = 1" \
  "Suspended = 2" \
  "None = 0" \
  "Forwarded = 1" \
  "EndAuthorized = 2"; do
  guard_expect_fixed_in_file "$TAG" "$pair" "$WIRE_RS" \
    "Rust I0-A enum value drifted: $pair"
done
for pair in \
  "TAG_INVALID = 0" \
  "TAG_HOST_HANDLE = 1" \
  "TAG_IMMEDIATE_I64 = 2" \
  "STATUS_NORMAL = 0" \
  "STATUS_FAULT = 1" \
  "STATUS_SUSPENDED = 2" \
  "DISPOSITION_NONE = 0" \
  "DISPOSITION_FORWARDED = 1" \
  "DISPOSITION_END_AUTHORIZED = 2"; do
  guard_expect_fixed_in_file "$TAG" "$pair" "$WIRE_PY" \
    "Python I0-A constant drifted: $pair"
done

# D1-I0 exact formal representation: the selected capability is the sole
# issuer, formal adoption attaches ValueIds once, and downstream metadata only
# carries the opaque co-sealed product.  No wire/backend surface may observe
# or reconstruct this source-owned representation.
for d1_fact in \
  "DynamicV2APrimeFormalRepresentationPairV1" \
  "DynamicV2APrimeFormalRepresentationRejectV1" \
  "DynamicV2PhysicalRepresentationV1::ImmediateI64" \
  "adopt_after_formals"; do
  guard_expect_fixed_in_file "$TAG" "$d1_fact" "$FORMAL_CAPABILITY" \
    "selected capability is missing the exact pos/end representation contract: $d1_fact"
done
for d1_fact in "formal_representation" "adopt_after_formals"; do
  guard_expect_fixed_in_file "$TAG" "$d1_fact" "$SELECTED_FORMAL_HEADER" \
    "formal header is missing the one-shot representation adoption: $d1_fact"
done
for d1_fact in "formal_representation" "DynamicV2APrimeFormalRepresentationPairV1"; do
  guard_expect_fixed_in_file "$TAG" "$d1_fact" "$SELECTED_SESSION_OPEN" \
    "selected session-open handoff is missing the exact representation product: $d1_fact"
done
guard_expect_fixed_in_file "$TAG" \
  "APrimeI64FormalPhysicalRepresentationProjectionV1" "$FORMAL_REPRESENTATION" \
  "opaque exact-i64 formal representation carrier is missing"
guard_expect_fixed_in_file "$TAG" "formal_physical_representation" "$CALL_METADATA" \
  "existing AOT metadata must carry the co-sealed formal representation opaquely"
for file in "$SELECTED_CAPABILITY" "$SELECTED_SESSION_OPEN" "$SELECTED_FORMAL_HEADER" \
  "$SELECTED_EMITTER" "$CALL_METADATA" "$FORMAL_REPRESENTATION" "$FORMAL_CAPABILITY"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "D1-I0 source reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done
for forbidden in \
  "APrimeI64FormalPhysicalRepresentationProjectionV1" \
  "DynamicV2APrimeFormalRepresentationPairV1" \
  "DynamicV2PhysicalRepresentationV1::ImmediateI64"; do
  if rg -F -q -- "$forbidden" "$ROOT_DIR/src/runner" "$ROOT_DIR/src/llvm_py" \
    "$ROOT_DIR/lang" "$ROOT_DIR/include"; then
    guard_fail "$TAG" "D1-I0 formal representation crossed into wire/backend activation: $forbidden"
  fi
done
if rg -F -q -- "DynamicV2APrimeFormalRepresentationPairV1" "$SELECTED_VALUE_LEDGER" \
  "$SELECTED_OPERATION_CURSOR"; then
  guard_fail "$TAG" "D1-I0 pair must not become a synthetic operation/value-ledger producer"
fi
python3 - "$SELECTED_FORMAL_HEADER" <<'PY'
from pathlib import Path
import sys

header = Path(sys.argv[1]).read_text(encoding="utf-8")
if header.index("adopt_after_formals") < header.index("adopt_exact_formal_parameter"):
    raise SystemExit("D1-I0 representation adoption must follow formal ValueId adoption")
PY

for forbidden in \
  "hako_dynamic_call_slot_v2" \
  "nyrt_host_call_slot" \
  "substring_hii" \
  "indexOf_hh" \
  "RuntimeDataBox" \
  "BoxCall"; do
  if rg -F -q -- "$forbidden" "$WIRE_RS" "$WIRE_PY" "$WIRE_C"; then
    guard_fail "$TAG" "I0-A wire schema must not dispatch through runtime/provider helper: $forbidden"
  fi
done

SCHEDULE_BODY="$(sed -n '/^fn build_schedule(/,/^fn schedule_for_operation/p' "$SELECTED_ABI")"
if printf '%s\n' "$SCHEDULE_BODY" | rg -q -- "source_role|segment_for_role"; then
  guard_fail "$TAG" "physical schedule must derive from verified placement/control, not source-role policy"
fi

echo "[$TAG] ok"
