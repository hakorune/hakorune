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
SELECTED_ABI="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_abi.rs"
SELECTED_EMITTER="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/mod.rs"
WIRE_RS="$ROOT_DIR/src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/call_slot_wire.rs"
WIRE_PY="$ROOT_DIR/src/llvm_py/builders/dynamic_v2_callslot_wire.py"
WIRE_C="$ROOT_DIR/include/nyrt_dynamic_call_slot_v2.h"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$EVIDENCE" "$INPUT" "$EXIT_TX" "$COSEAL_TESTS" \
  "$DEMAND_MOD" "$DEMAND_MODEL" "$DEMAND_ISSUER" "$SELECTED_ABI" \
  "$SELECTED_EMITTER" "$WIRE_RS" "$WIRE_PY" "$WIRE_C"

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

SCHEDULE_BODY="$(sed -n '/^fn build_schedule(/,/^fn segment_for_operation/p' "$SELECTED_ABI")"
if printf '%s\n' "$SCHEDULE_BODY" | rg -q -- "source_role|segment_for_role"; then
  guard_fail "$TAG" "physical schedule must derive from verified placement/control, not source-role policy"
fi

echo "[$TAG] ok"
