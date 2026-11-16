# Canonical Loop Compare same-block operand contract

Status: current shared canonical operand/append contract. F22 generic and
F25 selected Dynamic I9 adapters are retired.

The bounded strict Compare lane admits only this source-to-physical relation:

```text
full Published LoopOperationValueReceiptV1
  -> neutral owner/target/physical-value request
  -> same CanonicalSsaFunctionSessionV2 open-target witness
  -> exactly one actual MIR definition in that target block
  -> exact MirType::Integer
```

The Loop side owns the logical receipt and validates owner, class, and claimed
target. Canonical SSA owns the physical-definition scan and revalidates the
session-owned target. The co-sealed operand witness retains the full receipt;
no bare `ValueId` escapes as a standalone proof.

This P0 intentionally does not admit parameters, inherited values,
cross-block values, or general CFG dominance. `compute_def_blocks()` and
`compute_dominators()` are not acceptance authorities because they lose
same-block instruction order or retain legacy unreachable-block behavior.
The strict writer may append only after all fallible preparation is complete.
Canonical operand issuance remains shared after retirement of the old generic
Loop and selected Dynamic I9 adapters.

Rejections are typed and pre-effect: missing/duplicate definitions,
parameter use, cross-block definition, foreign owner/target, and
non-Integer/unknown/unavailable type leave MIR, type context, and the existing
ledger unchanged. The task and evidence ledger are maintained in
`docs/development/current/main/investigations/mirbuilder-loop-compare-same-block-operands-p0-2026-08-22.md`.

## Strict Compare writer boundary

The retained writer is `builder/resolved_lowering/canonical_compare_writer.rs`.
Its closed state machine is:

```text
verified open target
  + verified same-block Integer lhs/rhs
  + session-owned fresh destination
  + prepared Bool fact
      -> PreparedCanonicalCompareAppendV1
      -> one append_instruction_core commit (mir/ssot/cf_common.rs)
      -> non-Clone/non-Copy CanonicalCompareDefinitionSourceV1
```

All rejection-capable checks happen before append. The strict path does not
use ambient `current_block`, create blocks, invoke the repair-capable
`emit_instruction_at`/`emit_instruction` front door, materialize LocalSSA/PHI
inputs, infer types or run post-append Result checks. The Builder front door
delegates through canonical CFG to the same shared span-append owner.

F22 retires the generic Loop writer adapter; F25 retires selected Dynamic I9
and its private V13 publication/poisoning machinery. Their old guards and
caller-count claims are retired. Shared preparation/definition-source,
no-ambient-repair, exact single span append, source caps and split-trigger
checks remain in the existing
`tools/checks/dynamic_v2_physical_input_authority_guard.sh`. Current callers
retain responsibility for their own result publication and cleanup contracts;
retiring the old adapters does not relax this shared operand/append boundary.
