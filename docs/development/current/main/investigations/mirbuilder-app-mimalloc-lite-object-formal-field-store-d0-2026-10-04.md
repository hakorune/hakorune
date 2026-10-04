# mimalloc-lite object-formal field store D0 (caller-provided object into declared field)

Status: design stop — census recorded, Decision below. Owns the EXE
lane's next frontier after TASK4-ARTIFACT-S0 (ad1fc53776): the
`artifact-unowned-lifecycle-site` stop on the
`Call(BirthConstructor HakoAllocHandleResult.birth/3)` sites inside
`HakoAllocHeap.allocateResult`/`reallocResult` (owner slot 34).
Parent bundle slice: `RESULT-NEW-OBJECT-ARGS` (bundle card
`mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md` slice 11).
Scope: `MIRBUILDER-APP-MIMALLOC-LITE-OBJECT-FORMAL-FIELD-STORE-D0`
Related: `mirbuilder-app-mimalloc-lite-opaque-checked-compare-task4-artifact-d0-2026-10-04.md`

## Census — eight `HakoAllocHandleResult` result claims

`page_heap_fixture_result_claim_census` retains nine result claims: one
`HakoAllocHandle` (fully sealed post-S0) and eight
`HakoAllocHandleResult` sites — three in `allocateResult`
(`page_heap_box.hako:214-225`), five in `reallocResult` (`:300-319`).

| site | `home_prefix` | `construction` | `argument_rows` |
|---|---|---|---|
| alloc:216/221/224 | Ok | Err | Ok |
| realloc:302/306/310 | Ok | Err | Ok |
| realloc:315/318 | Err `PrefixNotCovered(Body(3))` | Err | Ok |

Decomposition (worker census, verified):

- `construction()` Err is `ConstructionUnavailableV1::
  FieldContractUnsupported` at `instance_construction.rs:451-472`:
  `birth`'s `me.handle = handle` stores a `Parameter` RHS into the
  `handle: HakoAllocHandle` declared-type field; the `Some(name)` arm
  admits only `ProviderConstruction` (`new` at the store site). This is
  the universal blocker — all eight rows.
- `home_prefix` Err on the last two `reallocResult` sites is
  `PrefixNotCovered(Body(3))`: `local replacement =
  me.realloc(handle, requested_size)` (`page_heap_box.hako:313`) is an
  unclaimed forward — a callee-claim question (`realloc` takes the
  object-typed `handle` formal and forwards to `release`), separate
  from this card's store arm.
- `argument_rows()` Ok on all eight: `null` literal and `handle`/
  `replacement` BoundValue args all convert to
  `OrdinaryNewTrivialArgumentV1` rows. Ok means source observation
  sealed; physical consumer admission is verified by the lane, not the
  row.
- `main.hako` never calls `allocateResult`/`reallocResult`: the eight
  sites sit in methods unreachable from `main`. The compiled set is
  every same-module declaration — `source_backed.rs:280-303` seals the
  full parser inventory unconditionally, `program_root_work_plan.rs:551-603`
  classifies every non-static box `ImmediateAndRuntime`, and
  `install/lowering_port.rs:528-543` demands
  selected==consumed==emitted (`IncompleteSelectedCoverage`/
  `IncompleteOrdinaryNewCoverage`). The package README explicitly
  states the sealed undertaking — "never AppMain reachability" — is the
  coverage criterion.
- Caller exclusion alone cannot resolve this lane: `HakoAllocHandleResult
  .birth` is emitted through the declaration-derived constructor batch
  regardless of callers; narrowing would have to drop the whole
  `HakoAllocHandleResult` box and would need a new transitive-closure
  authority over birth edges, field-initializer provider `new`s,
  `me.<field>` receivers, and static calls — none exists.

## Decision — `MIRBUILDER-APP-MIMALLOC-LITE-OBJECT-FORMAL-FIELD-STORE-D0` (accepted)

The `Some(<user-class>)` field arm of `issue_construction_plan` gains
one RHS form: a caller-provided object — `Parameter` (birth formal)
whose resolved type matches the declared field class — admitted as a
new `ConstructionStoreRhsV1` variant recorded with its exact site and
binding. Ownership transfers into the storing box: the field is an
owned object field, so the box's destruction disposition and
construction-fault reclaim treat the stored value exactly like a
`ProviderConstruction` child — same `PlainI64NoHook` child bound (the
declared class must resolve to a `PlainI64NoHook` definition), same
field-release discharge machinery. `HakoAllocHandle` (all-scalar
fields) is `PlainI64NoHook`, so `me.handle = handle` becomes the first
admitted instance.

Rejected alternatives:

- Reachable-set narrowing: no reachability authority exists, the
  sealed-undertaking coverage criterion explicitly excludes AppMain
  reachability, emission-time skipping violates
  `IncompleteSelectedCoverage`, and caller-only exclusion leaves the
  declaration-derived constructor batch. A new transitive-closure
  selection authority is a far larger design than the frontier needs.
- Borrowed (non-owning) field: the caller returns immediately after
  the `new`; nothing retains ownership, so a borrowed view would
  dangle. Transfer is forced by the usage shape.
- Widening `ProviderConstruction`: the provider arm records a `new`
  site with child identity and sealed trivial args; a parameter has no
  construction site — conflating them forges authority the source does
  not carry.

Boundaries that stay closed in this slice:

- `BoundValue` (call-result) RHS: `allocateResult` binds `local handle
  = me.allocate(size)` and passes it to `new`, but the STORE in
  `birth` sees only the `Parameter`. Locals rebound or copied inside
  `birth` stay rejected — the store RHS must resolve to the formal.
- Non-`PlainI64NoHook` children, wrong-class formals, weak fields,
  re-stores: unchanged `FieldContractUnsupported`.
- The `me.realloc` unclaimed forward (`reallocResult` Body(3) prefix)
  and object-formal call-argument use kinds: separate rows — the 2
  prefix-blocked sites stay `RetainedUnavailable` even after this
  slice.
- No physical whitelist, emission, or selection change; the physical
  lane's object-arg admission is observed, not widened.

Smallest next slice:
`MIRBUILDER-APP-MIMALLOC-LITE-OBJECT-FORMAL-FIELD-STORE-S0` — the new
`ConstructionStoreRhsV1` arm plus the `Some(name)` support check for a
class-matching `Parameter`, `PlainI64NoHook` child bound, unit pins
(admit `handle`; reject wrong class, non-plain child, rebound/copy
locals, non-formal expressions), census flip on the six prefix-clean
sites (`construction()` Ok → their `RetainedUnavailable` lifts), and
real-lane observation of the next named stop.

## Ordered tasks

1. `OBJECT-FORMAL-FIELD-STORE-S0`: implement the arm above; focused
   positive/negative unit tests; flip the census pin (six
   `construction()` Ok, two still prefix-blocked Err); record the new
   EXE-lane terminal; scope-guard pin.
2. Residual census on `me.realloc` unclaimed forward — object-formal
   call-argument/callee-claim owner; assign to its own card (bundle
   slices 10/12 territory).

## Non-claims

- No `all`/`complete` claim on result claims: `reallocResult`'s two
  `PrefixNotCovered` sites stay unavailable under this slice.
- No reachable-set selection authority; no emission-time skip.
- No app EXE PASS and no production switch claim from this Decision —
  the smoke's `--emit-exe` PASS leg remains the finish line.
- No claim on `HakoAllocHandle` non-plain children: the arm is bound
  to `PlainI64NoHook`; richer child dispositions are their own
  Decisions.
