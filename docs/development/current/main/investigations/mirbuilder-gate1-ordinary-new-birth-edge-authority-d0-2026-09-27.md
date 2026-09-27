# MIRBUILDER-GATE1-ORDINARY-NEW-BIRTH-EDGE-AUTHORITY-D0

Status: closed — Decision accepted 2026-09-27 (no implementation slice required)
Parent: workstream row H / Gate-1 internal owner series; follows
`MIRBUILDER-GATE1-ORDINARY-NEW-ARGUMENT-SOURCE-D0`+`S0` (landed —
BoundValue provenance cleared `ordinary-new/argument-source-unavailable`).
Selected because a mid-session measurement reported the next boxtorrent
terminal as `[freeze:contract][ordinary-new/birth-global-legacy-stopped]`.

## Premise correction — the terminal does not reproduce on HEAD

The recorded observation was taken on a binary built before the
BoundValue S0 landed. Re-measured on `881d2b14c0` (binary stamped after
the S0 commit):

- `./target/release/hakorune --dump-mir apps/boxtorrent-mini/main.hako`
  and `--emit-mir-json` both stop at
  `[freeze:contract][callable-loop/route-not-front-selected]`
  `LoopCondRouteRejected` — function `BoxTorrentManifest.chunkListText/0`,
  site `[Body(4), LoopBody(1), Value, Rhs]` — the **already-parked**
  callable-loop family, not the ordinary-new family.
- Line-bisect: replacing `new ContentChunk(cid, data, alloc_handle)`
  (main.hako:78) changes nothing about the terminal — `put`'s `new`
  lowers cleanly through the claim lane on the current binary.
- Positive fixture proof (`/tmp/birth_edge_d0.hako`, `Store.put`-shaped
  `local chunk = new Chunk(cid, data, alloc)` inside an instance
  method): the lowered MIR contains
  `call_birth CanonicalSameModuleCallableKeyV1 { namespace:
  BirthConstructor, owner: "Chunk", name: "birth", arity: 3 }
  (%1, %2, %4) [recv: %5]` — the typed `Callee::BirthConstructor`
  edge is already emitted.

Conclusion: the question "which claim authority should co-seal the
birth edge" is already answered by the existing claim authority. No
new authority, join, or lane widening is needed for the observed case.

## Census (worker + direct verification)

### Claim-lane mechanism — authority already co-seals

- `issue_ordinary_source_cohort_v1`
  (`ordinary_new_coseal_issue.rs:34`) mints
  `OrdinaryNewAdmissionClaimV1` per selected `new` destination;
  `OrdinaryNewCandidate::resolve`
  (`ordinary_new_candidate.rs:20-104`) runs
  `birth_for(box_source, arity)` — `Some(row)` ->
  `verified_birth_recipe_for_site_v1` (`:110-174`) requiring a
  `published_birth_key` with namespace `BirthConstructor`, owner =
  box name, arity match (`:130-142`), unit single-root
  `birth_completion` (`:143-150`), `OpaqueObservable` effect
  (`:151-157`), and `BirthAbiHandoffV1::issue` (`:158`) ->
  `OrdinaryNewConstructorDispositionV1::Birth(recipe)` plus the
  matching `birth_abi_handoffs` entry.
- `OrdinaryNewConstructorDispositionV1`
  (`ordinary_new_coseal.rs:138-142`) has exactly two arms:
  `NoBirthZero` | `Birth(VerifiedOrdinaryNewBirthRecipeV1)`. For an
  arity-3 `new`, `birth_for` returns `Some` or the **whole cohort
  issue fails** (`BirthConstructorMissing`,
  `ordinary_new_coseal_helpers.rs:67-80`) — `Birth(recipe)` is the
  only reachable disposition for `new ContentChunk(...)`.
- `claim.constructor()` (`ordinary_new_claim_access.rs:33-35`)
  returns the disposition **by value**, not `Option`. Therefore
  `ordinary_claim.map(|c| c.constructor())` is `Some(...)`
  whenever a claim is taken
  (`new_expression.rs:201-205`).
- Take path for `put`: `BoxTorrentStore.put` is `Cataloged`; the
  adapter arms the ledger via
  `with_cataloged_callable_source_scope`
  (`cataloged_instance_scope.rs:23,55` ->
  `source_scope.rs:95`), so `try_take_ordinary_new_claim` reaches
  `OrdinaryNewClaimLedgerV1::try_take`
  (`ordinary_new_coseal.rs:321-381`), which affinely removes the
  claim, removes the matching `birth_abi_handoffs` entry (target
  must equal `recipe.target_ref()` else `Mismatch`), and installs
  the `local_commits` row with `birth_target`/`birth_abi`.
- `prepare_new_emission`
  (`ordinary_new_local_commit.rs:384-459`): `available =
  construction().is_ok() && home_prefix.is_ok() && prior
  homes end_available`. `put`'s `home_prefix` is `Err`
  (instance method -> `EntryDemandMissing`), and `construction()`
  is `Err` for this field shape -> `available = false` -> row
  becomes `RetainedUnavailable{PendingExpression}`, returns
  `false` -> `selected_ordinary_claim = false` -> the
  `Ordinary` arm runs with `constructor = Some(Birth(recipe))`.
- `lower_ordinary_raw_new_with_port_v1`
  (`ordinary_new_admission.rs:31-75`): validates
  `recipe.abi().validate(3, 4)`, drives the raw argument ASTs,
  emits `MirInstruction::NewBox` then
  `MirInstruction::call(Callee::BirthConstructor { key:
  recipe.target(), receiver: dst }, arg_values, effects)` ->
  `Ok(dst)`. This is the D18-designed carrier: "the raw lane
  reuses the claim's verified `Birth` recipe — the only remaining
  birth carrier after `birth-global-legacy-stopped`"
  (`mirbuilder-exe-acceptance-ordinary-new-lifecycle-d18-2026-09-26.md:89-97`).

### When the freeze can still legitimately fire

`constructor == None` requires `ordinary_claim == None` **and**
`ordinary_birth_recipe == None`. The reachable `Ok(None)` gates:

- `RawLegacyChildLoweringPortV1::try_take_ordinary_new_claim` is
  hardwired `Ok(None)` (`raw_ordinary_new_claim.rs:148-157`) — the
  compatibility facade has no claim ledger by design.
- `RawInvocationChildPortV1::try_take_ordinary_new_claim`
  (`raw_ordinary_new_claim.rs:598-631`) returns `Ok(None)` when
  the ledger is absent (`:606`), no current source site (`:609`),
  the site is not `[Body, Initializer]` (`:612-620`), or the class
  is absent from `ordinary_box_names`
  (`ordinary_new_coseal.rs:327-332`).
- `take_birth_site_recipe` (`ordinary_new_coseal.rs:387-410`)
  covers only **non-**`[Body,Initializer]` sites and skips claimed
  sites (`collect_birth_site_index_v1`,
  `ordinary_new_coseal_issue.rs:433-476`) — a claimed
  initializer site never has an index entry.

For any such lane, a lowered `<Class>.birth/N` reachable through
`use_lowered` (`ordinary_new_admission.rs:80-104`) correctly stops
the line rather than minting `LegacyCallV0{Global(StaticBoxMethod
(<Class>.birth/N))}` with arity-malformed argv — the freeze is the
designed boundary, not a defect to widen.

`put` hits none of these gates on HEAD: the claim take succeeds,
the disposition is `Birth(recipe)`, and the typed edge is emitted
(proven by the positive fixture above).

## Six-line brief

```text
Decision: close — the ordinary-new birth edge is already co-sealed
  by existing claim authority; `claim.constructor()` =
  `Birth(VerifiedOrdinaryNewBirthRecipeV1)` drives
  `Callee::BirthConstructor` on the Ordinary arm. The stale
  observation does not reproduce on `881d2b14c0`; no implementation
  slice is required.
Source authority + canonical issuer:
  `issue_ordinary_source_cohort_v1` ->
  `OrdinaryNewCandidate::resolve` -> `birth_for` +
  `verified_birth_recipe_for_site_v1` (published_birth_key,
  unit birth_completion, OpaqueObservable effect, BirthAbiHandoff)
  — the sole birth-recipe issuer; `OrdinaryNewClaimLedgerV1::
  try_take` is the sole claim consumer.
Non-authority: `birth_site_index` (destination-less,
  non-[Body,Initializer] sites only — unchanged); lanes without a
  claim ledger carry no birth authority by design.
Fail-fast boundary: `constructor == None` + lowered
  `<Class>.birth/N` -> `ordinary-new/birth-global-legacy-stopped`
  remains the correct stop for unarmed lanes and uncovered
  classes; `try_take` `Unavailable`/`Mismatch` keeps its own
  freeze tags.
Smallest next slice: none for this family — the observed case is
  closed by existing authority. Next observable terminal is the
  parked `callable-loop/route-not-front-selected` family at
  `BoxTorrentManifest.chunkListText/0`.
Non-claims: no Gate-1 or MirBuilder completion; no claim-lane
  widening; no birth-site-index expansion into claimed
  initializer sites; no legacy `LegacyCallV0` carrier restoration;
  no resume of the parked callable-loop lane.
```

## Bounded S0

None emitted. The D0's premise ("claim-lane birth without claim
authority") is falsified on HEAD: the claim lane already carries the
verified `Birth` recipe, and the Ordinary arm already emits the
typed edge. Emitting an S0 would duplicate an existing authority —
the exact pattern the design boundary forbids.

## Non-claims

- No new claim authority, co-seal join, or target issuer — the
  existing `VerifiedOrdinaryNewBirthRecipeV1` is sufficient.
- No change to `prepare_new_emission`/`RetainedUnavailable` —
  `put`'s unavailable home prefix keeps its designed Ordinary-route
  outcome and still reaches `Installed` through
  `complete_new_expression`/`complete_local_installation`.
- No record_new_emission requirement on the Ordinary arm — bare
  `MirInstruction::call` plus `RetainedUnavailable -> Installed`
  is the designed accounting (D18 documented debt, unchanged).
- No `birth_site_index` expansion — claimed initializer sites keep
  their claim-only path.
- No callable-loop census claim — that family is a separate next
  design row (`MIRBUILDER-GATE1-CALLABLE-LOOP-ROUTE-FRONT-D0`),
  not part of this Decision.
- No Gate-1 or overall MirBuilder completion claim.

## Next

`MIRBUILDER-GATE1-CALLABLE-LOOP-ROUTE-FRONT-D0` — the queued next
observable terminal: `callable-loop/route-not-front-selected`
`SourceCallOutsideSelectedFamily` at
`BoxTorrentManifest.chunkListText/0`
`[Body(4), LoopBody(1), Value, Rhs]` — the `ids.get(i)` method call
inside the loop body. A different family (callable Loop consumer),
selected as the next design row by observed-terminal order.

`MIRBUILDER-GATE1` workstream row H: the ordinary-new family is
evidence-complete for `apps/boxtorrent-mini/main.hako` — argument
source (BoundValue S0), caller coverage (D0+S0+S1), and the birth
edge (this census) are all closed.
