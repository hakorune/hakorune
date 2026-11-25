#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-physical-input-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

EVIDENCE="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/physical_evidence.rs"
EXIT_TX="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/exit_transaction.rs"
COSEAL_TESTS="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/tests.rs"
CATALOG_ADMISSION="$ROOT_DIR/src/mir/builder/normal_cataloged_box_method_admission.rs"
PACKAGE_INSTALL="$ROOT_DIR/src/mir/normal_callable_semantic_package/install.rs"
PACKAGE_LOWERING_PORT="$ROOT_DIR/src/mir/normal_callable_semantic_package/install/lowering_port.rs"
PACKAGE_LOAN="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port.rs"
SELECTED_STORAGE_POLICY="$ROOT_DIR/src/mir/policies/a_prime_i64_callable_storage_layout.rs"
TARGET_CAPABILITY="$ROOT_DIR/src/mir/compiler/target_capability.rs"
BINDING="$ROOT_DIR/src/mir/builder/pinned_text_invocation_binding.rs"
ROOT_LIFECYCLE="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
ROOT_LOWERING="$ROOT_DIR/src/mir/builder/program_root_lowering.rs"
CALL_METADATA="$ROOT_DIR/src/box_callable/provider_admission/call_metadata.rs"
CALL_METADATA_TESTS="$ROOT_DIR/src/box_callable/provider_admission/call_metadata_tests.rs"
POLICIES_MOD="$ROOT_DIR/src/mir/policies/mod.rs"
FORMAL_REPRESENTATION="$ROOT_DIR/src/mir/a_prime_i64_formal_representation.rs"
SKELETON_BUILDER="$ROOT_DIR/src/mir/builder/calls/skeleton_builder.rs"
CANONICAL_SESSION="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/session.rs"
WIRE_RS="$ROOT_DIR/src/abi/dynamic_call_slot_wire.rs"
WIRE_PY="$ROOT_DIR/src/llvm_py/builders/dynamic_v2_callslot_wire.py"
WIRE_C="$ROOT_DIR/include/nyrt_dynamic_call_slot_v2.h"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$EVIDENCE" "$EXIT_TX" "$COSEAL_TESTS" \
  \
  "$SKELETON_BUILDER" "$CANONICAL_SESSION" \
  "$CATALOG_ADMISSION" "$PACKAGE_INSTALL" "$PACKAGE_LOWERING_PORT" "$PACKAGE_LOAN" \
  "$SELECTED_STORAGE_POLICY" \
  "$CALL_METADATA" "$CALL_METADATA_TESTS" "$POLICIES_MOD" "$TARGET_CAPABILITY" \
  "$BINDING" "$ROOT_LIFECYCLE" "$ROOT_LOWERING" \
  "$WIRE_RS" "$WIRE_PY" "$WIRE_C" "$FORMAL_REPRESENTATION"

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
  "physical_evidence_coseals_exact_placement_operation_and_effect_coverage" "$COSEAL_TESTS" \
  "exact physical evidence coverage test is missing"


for file in "$EVIDENCE" "$EXIT_TX" ; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done


# A borrowed ledger view is test evidence only.  The first live session handoff
# must consume the plan and move the ledger; keeping this API production-visible
# would reopen a second emission authority before that handoff exists.

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
if rg -n -A35 -- "fn create_resolved_function_skeleton" "$SKELETON_BUILDER" \
  | rg -q -- "EffectMask::READ|Effect::ReadHeap"; then
  guard_fail "$TAG" "canonical skeleton must not hardcode its function effect"
fi

# P3-I0 callable storage policy: one plain owner row is issued only at the
# selected close, then stored on the existing projection.  This is not a
# backend activation or a fourth semantic receipt.
for file in "$SELECTED_STORAGE_POLICY" "$CALL_METADATA" "$CALL_METADATA_TESTS"; do
  guard_expect_fixed_in_file "$TAG" "APrimeI64CallableStorageLayoutV1" "$file" \
    "A-prime callable storage-policy owner is missing"
done
guard_expect_fixed_in_file "$TAG" "NonAddressableSsaI64" "$SELECTED_STORAGE_POLICY" \
  "the A-prime storage policy must remain the non-addressable SSA-i64 singleton"
guard_expect_fixed_in_file "$TAG" "callable_storage_layout:" "$CALL_METADATA" \
  "existing AOT projection must carry a non-optional storage policy"
guard_expect_fixed_in_file "$TAG" "fn callable_storage_layout" "$CALL_METADATA" \
  "storage-policy projection accessor is missing"
guard_expect_fixed_in_file "$TAG" "projection.callable_storage_layout()" "$CALL_METADATA_TESTS" \
  "positive projection test must observe the storage policy"
for forbidden in "MirType" "StorageClass" "ASTNode" "as_recipe(" "serde"; do
  if rg -F -q -- "$forbidden" "$SELECTED_STORAGE_POLICY" ; then
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
  "target_storage_layout:" "$CALL_METADATA" \
  "existing AOT metadata must store the target-bound row"
guard_expect_fixed_in_file "$TAG" \
  "fn target_storage_layout" "$CALL_METADATA" \
  "target-bound metadata accessor is missing"
if rg -F -q -- ".map(|binding| binding.target_capability())" "$ROOT_LIFECYCLE"; then
  guard_fail "$TAG" "root lifecycle stripped the live invocation binding to a bare target"
fi
target_child_issuers="$(rg -n -F -- "pub(crate) const fn project_a_prime_i64_target_storage_layout(" "$TARGET_CAPABILITY" | wc -l | tr -d '[:space:]')"
if [[ "$target_child_issuers" != 1 ]]; then
  guard_fail "$TAG" "target child must have exactly one capability projection method: $target_child_issuers"
fi
for file in "$SELECTED_STORAGE_POLICY" "$CALL_METADATA" "$CALL_METADATA_TESTS" ; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "P3-I0 source reached hard 800-line boundary: $file has $lines"
  fi
done

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
guard_expect_fixed_in_file "$TAG" \
  "APrimeI64FormalPhysicalRepresentationProjectionV1" "$FORMAL_REPRESENTATION" \
  "opaque exact-i64 formal representation carrier is missing"
guard_expect_fixed_in_file "$TAG" "formal_physical_representation" "$CALL_METADATA" \
  "existing AOT metadata must carry the co-sealed formal representation opaquely"
for file in \
  "$CALL_METADATA" "$FORMAL_REPRESENTATION" ; do
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


# Shared canonical SSA relation guarantees retained from retired bridge guard.
IDENTITY="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/identity.rs"
guard_require_files "$TAG" "$IDENTITY"
guard_expect_fixed_in_file "$TAG" "verify_single_predecessor_phi" "$IDENTITY" \
  "canonical SSA must own the one-predecessor PHI relation check"
guard_expect_fixed_in_file "$TAG" "single_predecessor_phi_relation_accepts_distinct_target_value" "$IDENTITY" \
  "relation evidence must allow distinct target and predecessor ValueIds"

BUILDER_EMIT="$ROOT_DIR/src/mir/builder/builder_emit.rs"
APPEND_SSOT="$ROOT_DIR/src/mir/ssot/cf_common.rs"
CFG_INSTRUCTION="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_cfg/instruction.rs"
guard_require_files "$TAG" "$BUILDER_EMIT" "$APPEND_SSOT" "$CFG_INSTRUCTION"
# The Builder front delegates via canonical CFG to the shared mutation
# primitive in cf_common.
# CFG also appends to a temporary BasicBlock projection for preflight, retained.
direct_append_count="$(rg -F -o -- 'add_instruction_with_span(' "$BUILDER_EMIT" "$APPEND_SSOT" | wc -l | tr -d '[:space:]')"
if [[ "$direct_append_count" -ne 1 ]]; then
  guard_fail "$TAG" "physical span append count must be one; found $direct_append_count"
fi
if rg -n -F -- 'add_instruction_with_span(' "$BUILDER_EMIT"; then
  guard_fail "$TAG" "Builder front must not own a second direct span append"
fi
guard_expect_fixed_in_file "$TAG" '.append_selected_instruction(' "$BUILDER_EMIT" \
  "Builder front door must delegate through canonical CFG"
guard_expect_fixed_in_file "$TAG" 'crate::mir::ssot::cf_common::append_instruction_core(function, source, instruction, span)' "$CFG_INSTRUCTION" \
  "canonical CFG must delegate to the shared append owner"
for file in "$BUILDER_EMIT" "$APPEND_SSOT" "$CFG_INSTRUCTION"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "shared append source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done
builder_lines="$(wc -l < "$BUILDER_EMIT" | tr -d '[:space:]')"
if (( builder_lines >= 700 )); then
  guard_fail "$TAG" "builder_emit.rs reached the 700-line split trigger: $builder_lines"
fi

echo "[$TAG] ok"
