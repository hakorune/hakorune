# LOOP-CALLER-ZERO-PARITY-G0-I1-R0

Status: `closed — implementation receipt 2026-08-08`
Date: `2026-08-08`
Design SSOT: `docs/development/current/main/investigations/loop-caller-zero-parity-g0-i1-design-d1-2026-08-08.md`

## Objective

Run the first Generic G0 physical caller-zero canary through the common
segment/operation pipeline. This is a test-only proof, not production
selection or a new G0 physicalizer.

## Exact sequence

```text
exact compiler-side G0 ingress
  -> fresh unpublished function session
  -> move VerifiedFunctionCompletion exactly once
  -> two parameter entries through canonical identity
  -> five R1 segments + root After through common allocation
  -> fifteen Recipe rows exactly once
       item 3 = common DerivedCarrierEntry seed
       item 4 = structural nested Loop, no operation emission
  -> per-transfer root/child Predicate receipts
  -> root/child recursive After closure
  -> canonical read of post-loop b1
  -> exact I64 G0 Tail/Completion
  -> finish_for_draft_seal -> DraftSeal
```

## Acceptance

- all pre-allocation source, owner, frame, scope, segment, transfer,
  condition, carrier, and placement checks are Builder-free;
- every operation and transfer is consumed exactly once from the complete
  prepared program; no first/select/filter extraction API exists;
- root condition and child condition use different verified Bool values;
- item 3 uses the common carrier-seed emitter and canonical
  `read_entry_receipt`;
- late failure discards the whole unpublished session, restores the caller
  once, and a fresh session reproduces the same semantic shape;
- G0 Tail reads `L0.After/b1` through canonical identity and claims exact I64
  ABI/Completion once;
- focused tests, guards, README, exact `docs/reference/**`, current state,
  `10-Now.md`, and workstream are updated in the same implementation commit;
- touched source/check files remain below 800 lines.

## Explicit non-claims

```text
production selector / caller switch = 0
M8/M9 all-route coverage           = 0
M10b/M11/M12 retirement             = 0
retry/fallback/reselection          = 0
collector/module publication        = 0
backend/performance parity         = 0
G0-specific CFG/SSA/PHI owner      = 0
```

If any exact capability is missing, foreign, duplicated, stale, or inferred,
return typed `NoSafeSlice`; do not reconstruct source or add a compatibility
route.

## Implementation receipt (2026-08-08)

The bounded canary is closed in the implementation commit that adds
`generic_production_canary_tests.rs`. The exact resolver-issued G0 ingress is
split once into the full common operation program and the profile-specific
Tail. The canary opens the existing fresh unpublished function session,
publishes the resolver-declared receiver/parameters through canonical
identity, allocates the five R1 segments plus root After, and dispatches all
fifteen rows exactly once. Item 3 uses the common `CarrierSeed` emitter and
canonical `read_entry_receipt`; item 4 remains a structural nested Loop row.
Root and child Predicate transfers consume distinct completed Bool receipts.
The G0 post-loop `b1` read is canonical, the Tail is exact I64, Completion is
claimed once, and the existing typed finish/DraftSeal path is reached.

The late duplicate injection observes a typed `ValueAlreadyPublished` after
earlier rows have emitted, discards the whole unpublished session, and a fresh
session reproduces the same semantic receipt. The carrier emitter now uses the
existing `ensure_provisional_value_class` contract for unsealed PHI values;
no second type/SSA authority was added. All touched source files remain below
800 lines and focused common/G0 tests are green.

## Closed scope / next task

This row remains caller-zero evidence only. It does not authorize a production
caller switch, M8/M9 all-route Recipe coverage, M10b/M11/M12, retry/fallback
retirement, collector/module publication, backend/performance parity, or broad
legacy deletion. The next task is a design-only top-down audit that chooses
the smallest bounded M8/M9/production-selection prerequisite; no new physical
route may be opened before that audit fixes its exact owner, evidence, and
same-commit retirement boundary.

Every later implementation/cutover row must update the exact
`docs/reference/**` page and affected README in the same commit, and must add
the final reference update again after the production cutover.

## Dependency closeout

`LOOP-COMMON-PREDICATE-CARRIER-I0-R0` was the prerequisite and is closed.
This card's bounded canary is now closed with the implementation receipt above.
The next design-only audit must choose the next M8/M9/production prerequisite;
it must not silently open selection, fallback/retry retirement, collector
publication, or broad legacy deletion. The implementation commit updated the
exact reference, README, focused tests, current pointers, dashboard, and
workstream. A final reference update remains mandatory after the later
production cutover.
