Status: queued behind MIR-EMIT-CANONICAL-STRICTNESS-D0; implementation not authorized yet
Task: MIR-EMIT-CANONICAL-COMPARE-I0
Date: 2026-08-22
Priority: High
Owner: `src/mir/builder/resolved_lowering/loop_recipe_physicalizer/operation_emitter.rs` + `src/mir/builder/builder_emit.rs`
NextCard: MIR-EMIT-MOVE-COMMIT-R0
---

# MIRBuilder canonical CompareI64 emission I0

## Six-line brief

Decision: connect exactly one existing Loop physicalizer `CompareI64` leaf to
the strict canonical prepared-request boundary. Keep generic `compare::emit_to_at`,
Call/receiver, PHI, and legacy routes unchanged.

Source authority + canonical issuer: consume the existing
`PreparedLoopOperationEmissionV1`, `LoopPhysicalBlockReceiptV1`, and
`VerifiedLoopOperationTargetBlockV1`; a new private issuer must co-seal the
target's owner/role/block relation with strict sealed-state and operand/value
evidence before producing `PreparedCanonicalCompareEmissionV1`.

Non-authority: raw `BasicBlockId`, `current_block`, `ensure_block_exists`,
`require_i64_operands_at` by itself, generic Compare helper, LocalSSA, PHI
repair, AST/name lookup, and post-append result-type checks.

Fail-fast boundary: target, sealed-state, operand definition/type/dominance,
destination type, and ledger/cardinality checks finish before the sole writer
appends the Compare. No canonical error may retry through legacy repair.

Smallest next slice: close the missing placement/operand issuer, prepare one
Compare instruction, route it through one strict writer commit, then publish
Bool type and value-ledger facts only after commit success.

Non-claims: no generic writer migration, Call/receiver, PHI, assignment,
Recipe/Join changes, `EmitReceipt`, performance optimization, backend, or
main integration.

## Required authority closure before code

The D0 must be accepted first. I0 may begin only when these facts have one
named owner:

```text
target owner/loop/item/logical-block/role relation
target block exists and is not terminated or sealed
lhs/rhs are defined for the target block and have exact Integer type
destination ValueId is issued by the canonical owner
Compare result Bool publication is prepared, not inferred after append
Loop value ledger result is unique and ready to consume
outer unpublished function session owns discard on any commit failure
```

The existing `VerifiedLoopOperationTargetBlockV1` may be reused and extended,
but `BasicBlockId` must not be rewrapped into a second independent target
authority. If the existing target cannot be extended without becoming a
second CFG/SSA owner, create a private co-sealed adapter whose sole issuer is
the existing Loop physicalizer target path.

## Proposed move chain

```text
PreparedLoopOperationEmissionV1
  + ReadyLoopEntryV1
  + LoopPhysicalBlockReceiptV1
  -> issue_target_for_pure()
  -> strict target/operand/result preflight
  -> PreparedCanonicalCompareEmissionV1 (non-Clone, private)
  -> MirBuilder sole commit writer
  -> prepared Bool type publication
  -> one LoopOperationValueLedger publication
  -> LoopOperationEmissionReceiptV1
```

The prepared object must not contain AST, source names, Recipe keys beyond the
already-owned operation identity, a second physical ID authority, or a
fallback flag. Its constructor is private and its production count is one.

## Commit order

```text
read-only preflight
  -> allocate/validate destination under the existing canonical owner
  -> build Compare instruction
  -> sole physical append
  -> commit prepared Bool type
  -> publish value ledger exactly once
  -> return physical receipt
```

If the physical writer remains fallible after append, the outer function
session must discard the complete unpublished function draft and the focused
negative test must prove instruction, block, type, ledger, and publication
state are restored. A local cleanup or retry is not an accepted repair.

## Acceptance evidence

Positive:

- one valid CompareI64 reaches the exact target block;
- exactly one Compare instruction, one Bool type fact, one ledger row, and one
  physical receipt are published;
- owner/loop/item/role/block relations remain exact through consumption.

Negative:

- missing, foreign, terminated, or sealed target rejects before append;
- missing/foreign/non-dominating/wrong-type lhs or rhs rejects before append;
- destination type conflict rejects before append;
- duplicate ledger/result publication rejects without a second physical effect;
- canonical failure has zero calls to `ensure_block_exists`, LocalSSA repair,
  PHI repair, or legacy fallback;
- an injected post-prepare emission failure discards the unpublished session.

Structural:

```text
sole physical append site                                  = 1
canonical adapter -> ensure_block_exists                    = 0
canonical adapter -> LocalSSA/PHI repair                    = 0
canonical error -> legacy retry/fallback                    = 0
new canonical prepared constructor outside its issuer      = 0
touched Rust owner                                           < 760 lines
```

## Explicit non-overlap

`MIR-EMIT-MOVE-COMMIT-R0` may later remove successful-path clones and move the
instruction once, but it must preserve this strictness boundary and cannot be
combined with I0. LocalSSA definition-index and PHI analysis-batch work remain
separate D0 rows. This card does not authorize production switch or retirement
of any legacy caller.
