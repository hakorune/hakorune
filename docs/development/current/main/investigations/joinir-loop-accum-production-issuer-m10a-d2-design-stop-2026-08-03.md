---
Status: Design stop — implementation is not authorized until the issuer boundary is sealed
Date: 2026-08-03
Decision: reuse the existing canonical PHI/SSA owner; add only a builder-free resolved singleton policy issuer
Related:
  - joinir-loop-accum-production-bridge-m10a-n2-design-stop-2026-08-03.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
---

# DirectAccum production issuer: M10a/D2 design stop

## Purpose

PHI/SSA is already SSOT'd. The canonical owner is the existing
`CanonicalSsaFunctionSessionV2`, containing one
`ResolvedSsaIdentityStateV2`, one `BindingSsaBuilderV1`, one
`CanonicalCfgSessionV1`, and one `PhiTxn` for the whole function. This card
does not introduce another PHI materializer, SSA ledger, `MirBuilder`, or
route scheduler.

The missing piece is earlier: the resolved production ingress has no
builder-free issuer that can turn one exact singleton observation into a
typed `DirectAccum` admission. The current winner minting helper is test-only.
Until this issuer exists, wiring a physicalizer or `route_loop` would either
reselect a route, rebuild source identity, or bypass the resolved source/frame
authority.

## Required production issuer

Add one neutral, builder-free API near the existing loop policy SSOT:

```text
resolved singleton observation
  -> freeze_loop_route_schedule_v1
  -> evaluate_frozen_loop_route_schedule_v1
  -> VerifiedDirectAccumRouteAdmissionV1
```

The issuer must consume an already resolved source/frame capability and an
owned structural observation. It may accept only the exact schedule
`[AccumConstLoop]` for this row. It must not inspect a raw cursor, route names,
`diagnostic_effective`, `LoopRouteContext::route_kind`, the legacy registry, or
an AST-bearing fact. It must not rescan AST, reconstruct names/paths, or create
`ValueId`/`BasicBlockId` values.

The resulting admission is a one-shot, branded product for the existing
`admit_direct_accum_profile_v1` boundary. It co-seals the policy winner and
singleton schedule; Recipe/JoinSig, resolved loop source, binding-effect plan,
input projection, and Unit completion remain separate verified products and
are consumed later by the canonical function owner.

## Explicit non-claims

- This row does not add a Loop arm to `CanonicalTrivialSsaLowererV1`.
- This row does not create `CanonicalDirectAccumSsaLowererV1` or a second SSA
  owner; the later lowerer is only a facade over the existing canonical
  session.
- This row does not wire `route_loop`, the legacy registry, or a live
  `MirBuilder` caller.
- This row does not classify Generic V0/V1, remove Retry, or retire the old
  Accum/PHI edge.
- This row does not invent input values, seed `Const(0)`, or map Unit to
  `None`/a fabricated `ValueId`.

## Acceptance gates

1. **Policy single authority**: the issuer calls the existing policy SSOT;
   no second selector or route-name match is introduced.
2. **Builder-free**: the issuer and its tests compile without a
   `MirBuilder`, `CanonicalCfgSessionV1`, `BindingSsaBuilderV1`, or `PhiTxn`
   mutation.
3. **Exact singleton**: `[AccumConstLoop]` admits; overlapping, Generic, and
   ambiguous schedules reject with a typed pre-effect disposition.
4. **Frame/source branding**: the admission cannot be mixed with a different
   execution frame, resolved source, or owner.
5. **No fake projection**: no name lookup, AST rescan, raw `ValueId` table, or
   fabricated literal input is accepted by the API.
6. **Focused proof**: policy parity, singleton acceptance, overlap rejection,
   frame mismatch, and builder-free construction tests are green.

## Ordered follow-up after this card

```text
issuer green
  -> CanonicalFirstFamilyPlanV1::DirectAccum
  -> canonical-session DirectAccum facade
  -> identity-owned binding/input projection
  -> one physicalizer caller inside the unpublished compile candidate
  -> Unit completion proof
  -> full schedule census
  -> selected old Accum/PHI edge retirement
```

The current production caller remains zero until these gates pass. That is a
deliberate safety boundary, not a missing PHI/SSA design.
