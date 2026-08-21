# Canonical Loop Compare same-block operand contract

Status: accepted C-prime P0, caller-zero physical contract (2026-08-22).

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
complete; this P0 itself performs no destination allocation, result
reservation, append, fallback, or production selection.

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

`commit` consumes a writer-owned definition source and returns one full
receipt without a fallible post-append check. The legacy unbound ledger keeps
its old publication helpers for caller-zero compatibility; it cannot open a
strict reservation. This lifecycle is implemented in the linked result-ledger
task and does not yet connect a Compare writer or production caller.

## Strict Compare writer boundary

The strict physical writer is a separate caller-zero P0. Its closed state
machine is:

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

This P0 proves only the writer contract with three focused tests. It does not
reserve the result ledger, connect the dispatcher, select a production caller,
remove fallback, or claim I0/R0. The next design task must census the named
caller before any connection is attempted.
