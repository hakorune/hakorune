---
Status: closed cfg(test)-only execution brief; production remains closed
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-CROSS-SESSION-BRAND-AUDIT0-D3-S2-S1`
Ceremony: T2 design-gated premise repair
---

# Cross-session resolver brand audit

## Change

Add one private, non-`Clone` handoff-only brand that is issued by the same
`FunctionSemanticResolverSessionV1` as the resolved forest, frame, and
`BindingRefV1` roles. The test-only handoff witness must co-seal the issuer
owner with those products and reject an adversarial `forest_A + roles_B +
frame_B` pairing from two fresh sessions resolving the same source.

The ownerless structural `LoopExecutionFrameKeyV1` used by DirectAccum remains
unchanged. This row adds no Generic snapshot, logical key, seed, selector,
Builder/MIR/Recipe/PHI route, or production caller.

## Contract

- Resolver/session owner is the sole brand issuer; source coordinates,
  `FunctionOriginV1`, names, route IDs, and physical `ValueId`s are not brands.
- The brand is consumed as one unit with forest/frame/BindingRef evidence;
  independent component pairing and hidden re-rooting are rejected before any
  Builder effect.
- Same-session natural-Both and existing S0 typed negatives remain green.
- No production type or selector arm is connected by this task. If preserving
  the DirectAccum ownerless frame requires a second authority, return to the
  D3-S2 design stop instead of widening this row.

## Done

- A focused adversarial cfg(test) witness rejects cross-session mixing with a
  typed owner/invocation mismatch.
- The positive same-session witness and existing S0 rejection matrix remain
  green; production caller/import and artifact counts remain zero.
- `cargo test` for the focused resolved-carrier suite, current-state pointer
  guard, `git diff --check`, and the repository <800-line source/check guard
  are green.
- The implementation commit updates the exact D3-S2 design card,
  `CURRENT_STATE.toml`, the active workstream, and any touched reference or
  README surface in the same commit. A later passive provenance product still
  requires a new design/selection row.

## Stop

Return to the D3-S2 design stop if the brand cannot be co-sealed without
changing DirectAccum frame semantics, minting a second BindingRef/source
authority, exposing AST/ValueId data, or requiring production selector policy.
Do not add fallback, retry, `Option<Capability>`, or a global route change.

## Closeout

`JOINIR-GENERIC-RESOLVED-CARRIER-CROSS-SESSION-BRAND-AUDIT0-D3-S2-S1` closes
with one private test-only forest/frame brand wrapper and one adversarial
cross-session witness. The focused provenance suite is 5/5 green; the
implementation adds no production caller, import, artifact, selector arm,
or DirectAccum frame change. Exact current-state, workstream, and D3-S2
design pointers are updated in the implementation commit. No public language
or reference contract changed, so no `docs/reference/**` surface is claimed.
