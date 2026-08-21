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
