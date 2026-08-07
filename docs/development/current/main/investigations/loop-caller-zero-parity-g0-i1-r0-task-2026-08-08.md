# LOOP-CALLER-ZERO-PARITY-G0-I1-R0

Status: `next implementation row — dependency closed 2026-08-08`
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

## Dependency closeout

`LOOP-COMMON-PREDICATE-CARRIER-I0-R0` is now closed. This row is the next
bounded implementation and may open the G0 caller-zero canary, but it still
must not open production selection, fallback/retry retirement, collector
publication, or broad legacy deletion. The same implementation commit must
update the exact `docs/reference/**` page and the affected README, focused
tests/guards, current pointers, dashboard, and workstream. A final reference
documentation update is required again after the G0 implementation/cutover.
