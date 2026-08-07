# Loop recipe ReadBinding leaf-emitter design stop D0

Status: `design-stop`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-EMITTER-CONST-S0`
Authority: `docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

Fix the next leaf boundary before any ReadBinding implementation. The full
`VerifiedLoopOperationPhysicalDemandV1` remains a complete move-only program
input and must not gain a single-item extraction API. A future orchestrator
may issue one prepared ReadBinding row only after complete semantic preflight;
the leaf emitter must consume that prepared row and borrow canonical owners.

## Design questions to close

- Which exact source/effect receipt authorizes the ReadBinding operation?
- How does the prepared row bind owner, logical Loop/Block, physical role, and
  the canonical `BindingRefV1` without re-reading AST or names?
- Which existing `ResolvedSsaIdentityStateV2::read_entry` receipt publishes the
  physical `ValueId`, and what is the exact failure/poison boundary?
- How does the operation result key map to the leaf receipt without creating a
  second BindingSSA/PHI/value environment?
- Which pre-emission rejects are required for foreign owner, missing source
  anchor, missing binding, wrong class, wrong placement, and terminated block?

## Required decision

Produce one SSOT section and a focused task for the implementation row. The
decision must keep `Recipe`, profile identity, Tail/ABI/Completion,
continuation, DraftSeal, selector, retry/fallback, and production authority
outside the leaf emitter. It must define whether the ReadBinding row is
allowed only after a `ReadyLoopEntryV1` binding receipt exists, and must state
that late failure discards the whole unpublished function session.

## Worker review: `REVISE` (2026-08-07)

The broad boundary is accepted, but implementation is not yet authorized. The
following contracts must be fixed in the SSOT before an implementation card
can be opened:

1. The source/effect input is projected only once from a complete prepared
   program. The projection must match the Recipe `binding`/`result`, the
   verified effect row (`source_binding`, `anchor`, `role`), and the owner;
   no AST, name, ordinal, or full-demand re-extraction is allowed.
2. D0 admits `LoopBindingEffectAnchorV1::Expr` only. The Generic item-3
   `DerivedCarrierEntry` path is rejected as `CarrierSeedUnavailable` and is
   a separate future row.
3. The raw `ValueId` returned by `ResolvedSsaIdentityStateV2::read_entry`
   cannot become the leaf receipt directly. A thin canonical seam must issue
   `CanonicalBindingReadReceiptV1 { owner, binding, physical_block,
   physical_value }` after the read is verified.
4. Placement is authorized only by `LoopPhysicalBlockReceiptV1` plus the
   orchestrator's logical Loop/Block/role. `current_block` and ordinal
   inference are forbidden. All checks occur before the canonical read/PHI
   operation.
5. The logical result key is an alias publication only. No second ValueId,
   BindingSSA map, or PHI owner is created; the leaf returns one immutable
   receipt `{ owner, item, binding, result, block, value }` to the outer
   operation ledger. Return/Completion/DraftSeal remain outside.
6. Identity and `PhiTxn` are borrowed from the canonical function session by
   one explicit read-service bundle. The physicalizer must not grow a second
   session or silently add another owner.
7. Pre-effect rejects are typed `NoSafeSlice`. A post-read type/receipt
   mismatch is a late terminal: discard the whole unpublished function,
   preserve only local Phi cleanup diagnostics, and never retry/fallback.
8. The reject matrix must cover operation-not-ReadBinding, missing/mismatched
   source anchor or binding, Core effect/role mismatch, CarrierSeedUnavailable,
   owner/logical/physical placement, entry binding, canonical BindingRead,
   result type, terminated block, and late emission.

Acceptance for this design row is the source/effect mapping matrix, the typed
reject matrix, the canonical receipt ownership rule, and explicit non-claims
in the common physical-demand SSOT. No Rust Builder mutation is part of D0.

## Stop boundary

This card is design-only. Do not add Builder mutation, BindingSSA/PHI writes,
full-program scheduling, source rereading, production selection, fallback,
or legacy deletion until the decision is accepted and the implementation
card is created. After implementation, update the reference documentation,
current state, workstream, and this card in the same implementation commit.
