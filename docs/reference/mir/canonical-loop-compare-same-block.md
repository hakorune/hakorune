# Canonical Loop Compare same-block operand contract

Status: accepted C-prime P0 plus selected Dynamic I9 transactional CONNECT0
handoff (2026-08-23).

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
The later strict writer may append only after all fallible preparation is
complete. The generic Loop physicalizer remains outside production selection;
the selected Dynamic I9 connection is documented below and uses this same
canonical operand issuer without projecting into the generic Loop ledger.

Rejections are typed and pre-effect: missing/duplicate definitions,
parameter use, cross-block definition, foreign owner/target, and
non-Integer/unknown/unavailable type leave MIR, type context, and the existing
ledger unchanged. The task and evidence ledger are maintained in
`docs/development/current/main/investigations/mirbuilder-loop-compare-same-block-operands-p0-2026-08-22.md`.

## Result lifecycle boundary

The next physical result lane uses the same one-authority rule. An
owner-bound `LoopOperationValueLedgerV1` reserves an absent key with a private
affine slot token. The transition is:

```text
map-vacant -> Reserved -> Published
                       \-> Poisoned (uncommitted token drop)
```

The selected Dynamic ledger's private pending token is reserved only after all
Compare and Branch preparation succeeds. Its commit is reached only through
the private I9 aggregate, so it accepts no arbitrary writer definition and
performs no post-append pairing check. Dropping the token poisons the slot and
the unpublished outer session discards the partial function. The generic Loop
ledger and its old publication helpers remain caller-zero and outside this
transaction.

## Strict Compare writer boundary

The strict physical writer is generic-Loop caller-zero but has one named
selected-Dynamic production consumer. Its closed state machine is:

```text
verified open target
  + verified same-block Integer lhs/rhs
  + session-owned fresh destination
  + prepared Bool fact
      -> PreparedCanonicalCompareAppendV1
      -> one append_instruction_core commit
      -> non-Clone/non-Copy CanonicalCompareDefinitionSourceV1
```

All rejection-capable checks happen before append. The strict path does not
use ambient `current_block`, create missing blocks, invoke the repair-capable
`emit_instruction_at`/`emit_instruction` front door, materialize LocalSSA or
PHI inputs, infer types, or run post-append Result checks. The legacy front door
and strict front door share the same direct MIR append primitive, so there is
one physical mutation owner.

This P0 proves the writer contract with focused tests. It does not connect the
generic dispatcher or retire its legacy lane. The selected Dynamic I9 handoff
is the one named production consumer and has its own Dynamic result lifecycle
and CONNECT0 guard.

## Dynamic I9 direct handoff

The selected Dynamic normal landing is the bounded CONNECT0 consumer. Its
authority chain is:

```text
Dynamic V11/V12 published views
  -> canonical owner-bound same-block Integer witnesses
  -> fresh canonical destination + prepared Bool fact
  -> strict Compare preparation
  -> session-bound Branch preparation
  -> Dynamic V13 reservation (last fallible step)
  -> private aggregate: Compare append + Bool + V13 + Branch commit
```

`DynamicV2PhysicalSessionBrandV1` carries the same `FunctionOwnerIdV1` as the
canonical SSA session. `DynamicV2PhysicalValueLedgerV1` is the sole V13
publication owner; no `LoopOperationValueLedgerV1` projection, legacy
`emit_compare_i64_at` fallback, post-append `publish`, or assert-based
definition pairing is allowed. A dropped pending reservation poisons the
result slot and the unpublished outer draft is discarded rather than retried.
This slice still excludes the I7 header Compare, cross-block operands,
generic dominance, and old generic-loop retirement.
