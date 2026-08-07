# Loop recipe ReadBinding leaf emitter I0

Status: `closed`
Date: 2026-08-07
Parent design: `LOOP-RECIPE-OPERATION-EMITTER-READ-DESIGN0-D0`
Authority: `docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Decision

The ReadBinding D0 design is accepted after the final worker audit. Implement
one private Expr/SourceRead leaf emitter behind the complete prepared program.
The first fixture may contain one ReadBinding row, but the row must come from
the complete program; no demand or full program single-item extraction API may
be added.

Implementation receipt (2026-08-07): the complete prepared program now
projects every Expr/SourceRead row with exact Core effect/source/placement
checks. The private leaf borrows canonical BindingSSA/PHI services, issues a
`CanonicalBindingReadReceiptV1`, validates the existing TypeContext, and
returns an immutable `ReadBindingEmissionReceiptV1`. Focused positive,
preheader-requirement, canonical type-failure/discard, Generic carrier-reject,
and full-program/no-extraction tests are green. This remains a disconnected
test-only leaf; no production physicalizer was opened.

## Scope

Implement only the following bounded slice:

1. Add the private read projection with exact typed fields:
   `LoopBindingKeyV1`, `BindingRefV1`, `LoopValueKeyV1`, logical
   `LoopBlockKeyV1`, orchestrator-supplied `LoopPhysicalBlockRoleV1`, and
   `LoopReadEntryRequirementV1::{PreheaderSeed,CanonicalLive}`.
2. Project rows once from `PreparedLoopOperationProgramV1`. The projection
   must match Recipe operation, source evidence, Core effect role/anchor,
   source binding, owner, and logical placement. It must admit only
   `Expr(OwnedExprSiteV1)` plus `SourceRead`; `DerivedCarrierEntry` returns
   `CarrierSeedUnavailable`.
3. Add the canonical read seam
   `CanonicalBindingReadReceiptV1 { owner, binding: BindingRefV1,
   physical_block: BasicBlockId, physical_value: ValueId }`. Its sole issuer
   performs exact source claim, canonical BindingSSA read, and receipt
   validation in the fixed order. Raw `ValueId` is never a leaf receipt.
4. Borrow the canonical Builder/identity/Phi owners through one explicit
   `CanonicalBindingReadServicesV1` bundle. Use the existing `TypeContext`,
   `TypeFactDecisionV1`, and `PreparedTypeFactPublicationV1`; add no type-fact
   owner or parallel CFG/SSA/PHI owner.
5. Bind logical block and expected role through the sole
   `LoopPhysicalBlockReceiptV1`. The leaf must not use `current_block`, source
   shape, or ordinal inference.
6. Return one immutable `ReadBindingEmissionReceiptV1` containing distinct
   logical/physical fields. The outer operation ledger owns `result` alias
   publication; the leaf creates no second value or SSA map.

## Failure boundary

- Before claim/read starts, malformed source/effect/entry/placement data is a
  typed `NoSafeSlice` with zero Builder, claim, and PHI effect.
- Once the source claim succeeds or canonical read starts, every read,
  type, receipt, or late emission failure is terminal `Freeze`: discard the
  whole unpublished function, restore the caller once, and retain PhiTxn abort
  only as local diagnostic cleanup. There is no retry or fallback.

## Focused acceptance evidence

- Positive Expr/SourceRead ReadBinding emits an exact value in the role/block
  supplied by the physical block receipt and returns the canonical receipt.
- `DerivedCarrierEntry` rejects as `CarrierSeedUnavailable` before claim.
- `PreheaderSeed` requires an exact `ReadyLoopEntryV1` row; `CanonicalLive`
  does not require a preheader row and uses canonical SSA availability.
- A post-read type mismatch discards the whole unpublished function; no caller
  ledger or module publication remains. The success path also discards its
  unpublished session before the test ends.
- A Generic G0 carrier-entry row rejects before it can be projected as a read
  leaf. The broader mismatch matrix (foreign entry, terminated block, missing
  canonical binding, and source/effect corruption) remains a separate full
  physicalizer integration row.
- Full Callable/G0 `prepare_all` coverage remains exact and Builder-free;
  this row does not add a single-operation extraction helper.

## Nonclaims and stop lines

This row does not implement other operations, carrier seeds, full Loop
physicalization, continuation/Tail, Return/Completion/DraftSeal, module
publication, production selection, retry/fallback retirement, legacy deletion,
or performance claims. Do not read AST/names, add a second registry/SSA/PHI
owner, or route failures to an old physicalizer.

## Gates and documentation

Before commit, run the focused physicalizer and demand tests, `cargo check
--lib`, direct rustfmt check for changed Rust files, `git diff --check`,
`current_state_pointer_guard.sh`, and `mirbuilder_inplace_replacement_guard.sh`.
Every implementation commit must also update the exact reference docs,
`src/mir/loop_recipe_contract/README.md`, the canonical lowering README, this
card, `CURRENT_STATE.toml`, and the active workstream. No touched source or
test file may exceed 800 lines.
