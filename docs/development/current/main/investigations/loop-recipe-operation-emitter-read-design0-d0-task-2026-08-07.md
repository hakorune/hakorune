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

## Stop boundary

This card is design-only. Do not add Builder mutation, BindingSSA/PHI writes,
full-program scheduling, source rereading, production selection, fallback,
or legacy deletion until the decision is accepted and the implementation
card is created. After implementation, update the reference documentation,
current state, workstream, and this card in the same implementation commit.
