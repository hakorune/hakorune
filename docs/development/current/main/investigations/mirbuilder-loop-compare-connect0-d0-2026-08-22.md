Status: D0-A/D0-B accepted conditionally; selected Dynamic body-to-semantic-state bridge P0 is next, before live-publication evidence
Task: MIR-CALLABLE-DYNAMIC-BODY-STATE-BRIDGE-P0
Date: 2026-08-22
Priority: implement one private non-emitting observation bridge for resolver-backed Dynamic body and closed W6 evidence; keep backend, live publication, and generic retirement closed
Parent: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
PreviousCard: MIR-LOOP-COMPARE-CONNECT0-EVIDENCE-D0
NextCard: MIR-CALLABLE-DYNAMIC-BODY-STATE-BRIDGE-P0 (same rolling card)
---

# Loop Compare CONNECT0 handoff

## Six-line brief

```text
Decision: accept and connect the active Selected Dynamic I9 normal-landing Compare as the one named non-test production caller; keep live publication and generic-loop retirement gated until the focused evidence and reusable guard are green. The handoff uses the Dynamic value ledger as the sole I9 publication ledger; it does not project into the Loop ledger.
Source authority + canonical issuer: `DynamicV2CompareI64CapabilityDemandV1`/the prepared I9 row and the session-owned `DynamicV2PhysicalValueLedgerV1` own I9 facts; canonical CFG/SSA owns target and definition witnesses; one private `SelectedDynamicI9CompareHandoffIssuerV1` co-seals them before any Compare effect.
Non-authority: the test-only Generic G0 dispatcher, focused canaries, old emit_compare_i64_at, Dynamic brand alone, Dynamic value views alone, raw ValueId/state.get, current_block, Builder cursor, operation enum alone, and a fallback retry.
Fail-fast boundary: I9 row/target/operand provenance, Dynamic-brand-to-function-owner binding, canonical same-block witnesses, destination/Bool preparation, and the Dynamic ledger's V13 reservation must all be complete before the first Compare append; a rejected strict row cannot return to the old leaf.
Smallest next slice: `MIR-LOOP-COMPARE-CONNECT0-EVIDENCE-D0`, adding the caller/fallback guard and recording focused positive, negative, and reservation-poison evidence; no new semantic path and no unrelated family migration.
Non-claims: no I7 header Compare, no generic dispatcher connection, no general dominance, no cross-block operands, no Const/Binary migration, no A/C/Recipe redesign, no old-leaf retirement, and no production I0/R0.
```

## Next frontier design stop

Why this is not Fast path: live publication and generic-loop retirement would
change the production publication boundary and delete an old physical edge, so
the named consumer, publication owner, fail-fast terminal, and atomic cutover
must be re-audited before implementation.

## Next frontier: live publication boundary D0

The read-only audit confirms that the selected I9 handoff is reachable from a
real non-test compile path, not only from a helper or focused test:

```text
compile_normal
  -> normal_default_root_catalog_post_install
  -> program root lowering
  -> installed semantic package
  -> lower_cataloged_static_box_method
  -> selected Dynamic I9 handoff
  -> strict Compare writer
```

This does not yet prove an end-to-end live publication fixture. The word
`publication` is split into four distinct stages:

| Stage | Meaning | Owner | This D0 |
| --- | --- | --- | --- |
| `DraftAdmission` | completed callable draft becomes a collector receipt | `ModuleDraftCollectorV1` | context only |
| `ModuleDrain` | prepared collector rows are committed into `current_module` | `PreparedNormalCollectorDrainLifecycleV1` | define evidence |
| `ExternalCommit` | sealed module passes compiler-owned verification and is externally committed | `PreparedModuleExternalCommitV1` / invocation session | define evidence |
| `BackendEmission` | LLVM/object/native or VM consumer observes the module | backend owner | explicit non-claim |

The publication audit is refined by the following prerequisite design task:

```text
MIR-CALLABLE-DYNAMIC-BODY-STATE-BRIDGE-D0
```

Six-line brief:

```text
Decision: conditionally accept `SelectedDynamicBodyStateBridgeV1` as one private, non-emitting, one-shot transaction; it co-seals existing resolver source, Dynamic Rc, source relation, closed W6 evidence, and `CallableSemanticLoweringState` without issuing a semantic receipt or physical instruction.
Source authority + canonical issuer: `SelectedCallableLoweringInputRefV1::source()`/`FunctionSourceViewV1` owns the exact body, the existing `Rc<VerifiedSourceBackedDynamicCallableV1>` owns Dynamic source semantics, `DynamicAPrimeI64SourceRelationViewV1` owns source/W6 relations, W6 owns formal/ledger/profile receipts, and the selected Dynamic adapter is the sole bridge issuer.
Non-authority: A-prime validation alone, package `complete()` alone, `RawInvocationChildPortV1::lower_*`, AST rescan, raw `ValueId` equality, operation cursor alone, `state.finish()` alone, collector/module/backend observation, old leaf, fallback, and a second writer.
Fail-fast boundary: W6 cleanup/operation close and `profile_close` must retain the outer-return receipt before the bridge; the bridge consumes every source/state obligation exactly once, then `state.finish()` succeeds before canonical `finish_for_draft_seal()` and DraftSeal. Any failure discards the unpublished session and cannot reach collector commit or fallback.
Smallest next slice: `MIR-CALLABLE-DYNAMIC-BODY-STATE-BRIDGE-P0`: transport the existing Dynamic Rc, retain I11 `V14` and the profile-close After receipt, add explicit alias/local/read/rebind/tail observation APIs, invoke the bridge exactly once before DraftSeal, and add focused negative/duplicate/no-effect evidence.
Non-claims: no new semantic `Verified*`/`Prepared*` receipt, no AST-only authority, no duplicate physical lowering, no generic dispatcher activation, no cross-block dominance, no Const/Binary migration, no old-leaf retirement, no live publication claim, no LLVM/VM/object promotion, and no performance work.
```

### Worker audit result

The static route is real, but the live publication claim is not yet evidenced:

```text
parser_scan_loop_box.hako
  -> compile_normal
  -> normal_default_root_catalog_post_install
  -> program root lowering
  -> selected Dynamic adapter
  -> W6 unpublished emitter
  -> (not yet proven: complete body consumption)
  -> DraftAdmission / ModuleDrain / ExternalCommit
```

Evidence anchors are `normal_default_pipeline.rs:452-484`,
`normal_callable_semantic_loan_port.rs:463-495`,
`selected_dynamic_physical_emitter/mod.rs:685-728`,
`program_root_lowering.rs:237-244`, and
`normal_default_pipeline.rs:505-510`. The selected Dynamic branch receives the
source `body` but the W6 call uses `inspect = |_| Ok(())`; the focused adapter
test also supplies empty parameter/body vectors. These prove reachability of
the handoff, not consumption of the unchanged source body and not a live
module publication.

The first D0 deliverable is therefore an existing-owner state-bridge contract,
not a new semantic receipt. `NormalCallableSemanticPackagePortV1::complete()`
currently proves only selected-key coverage: `with_selected_*` marks a key
after its callback returns. It must not be relabeled as proof that the selected
method body was consumed. The selected Dynamic branch currently receives
`body` but passes `inspect = |_| Ok(())` to W6, so it also bypasses the
`CallableSemanticLoweringState` scope whose `finish()` checks entry, locals,
variables, assignments, direct lambdas, brand constructors, and Dynamic
origins.

The exact resolver body authority is
`SelectedCallableLoweringInputRefV1::source()` ->
`ResolvedFunctionLoweringInputV1` -> `FunctionSourceViewV1::root_body()` and
its checked body cursor. The state authority is the existing
`CallableSemanticLoweringState`. The selected-Dynamic adapter must connect
these to the existing full Dynamic demand/W6 evidence through one private
non-emitting consume/commit operation, and call `state.finish()` before
`finish_unpublished_draft`. A-prime demand validation alone is insufficient.

`RawInvocationChildPortV1::lower_*` cannot serve as this bridge: those methods
perform physical lowering, so calling them as an observation pass would emit
the body twice before W6. Do not add a default `Consumed` receipt, treat an
empty body as consumed, rescan the AST in a second authority, create an
ad-hoc aggregate, or create a Dynamic-to-Loop adapter. If the bridge cannot
consume W6-owned evidence without a second physical writer, keep `NoSafeSlice`.

After this D0 is accepted, the bounded P0 may implement the bridge and then
observe one `DraftAdmission`, one `ModuleDrain`, and one `ExternalCommit`.
Generic retirement remains separately gated by the shared legacy leaf census.

The acceptance therefore has two independent facts plus one ordering fact:

```text
package completion = every selected key was consumed exactly once
body completion    = every selected source/body operation was consumed by
                     the existing Dynamic demand/W6 owner exactly once
state ordering     = CallableSemanticLoweringState::finish() succeeds before
                     finish_unpublished_draft
```

Neither fact may stand in for the other. If the existing owner cannot consume
the unchanged body without a second source authority, the result is
`NoSafeSlice`, not a synthetic success marker.

### Exact `finish()` obligation audit: NoSafeSlice

The main audit is complete for the unchanged `parser_scan_loop_box.hako`
fixture. The resolver and W6 products contain useful relations, but none of
the non-empty obligations below currently has an existing non-emitting
exact-once bridge into `CallableSemanticLoweringState`:

| `finish()` obligation | Existing resolver/W6 evidence | Missing safe connection |
| --- | --- | --- |
| entry | `DynamicV2OpenedFormalHeaderV1` owns the four formal values and their source roles | no production adapter constructs the existing `PreparedCallableEntryValuesV1` from these exact values and calls `install_entry_values` without re-reading or allocating |
| locals | source rows identify `i` and `ch`; W6 emits their physical declarations/values | `publish_declaration_exact` is a physical publication, not `record_completed_local`; no non-emitting completed-local observation exists |
| variables | W6 operation rows identify the exact `ReadBinding` sites | operation-cursor claims do not consume the state variable site/value; `read_variable` has no bridge receipt carrying the exact physical relation |
| assignments | the W6 `WriteBinding` and `define_assignment_exact` identify the step assignment | physical assignment definition is not `state.rebind`; no bridge proves the state transition and its dynamic-origin invalidation exactly once |
| direct lambda captures | the unchanged fixture has a resolver-backed empty set | the zero proof is available, but the bridge still needs an explicit empty-set check; no generic observation path exists |
| brand constructors | the unchanged fixture has a resolver-backed empty set | same: empty coverage can be proven, but must not be inferred from a missing/default map |
| dynamic origins | formal/local origin rows and the loop rebind relation exist | entry install, local completion, and rebind invalidation are not connected by one non-emitting state transaction |

Two additional transport/ordering gaps prevent a bounded implementation now:

1. `issue_selected_a_prime_i64_physical_demand` consumes the selected input
   before W6 starts and currently drops the existing Dynamic source `Rc`. A
   bridge cannot recreate that source from the AST, digest, name, or physical
   demand; the existing source owner must be transported or the state scope
   must be established before that move.
2. W6 does not retain the I11 `V14` inner-return value in its production
   value ledger, and the outer-return read is created by `profile_close` inside
   `finish_unpublished_draft`. The current `inspect` seam is therefore before
   part of the required tail evidence. Moreover, the state rebind is a linear
   binding update while the physical outer return reads the loop-header current
   value. A bridge must name that relation explicitly; ignoring the value
   mismatch would be a second semantic guess.

The only existing discard-safe insertion seam is the `inspect` callback in
`assemble_unpublished_selected_dynamic_w6`, after `begin` and before
`finish_unpublished_draft`. It is a candidate seam, not an implementation
authorization. To make it valid, the design must either retain all required
tail evidence before the callback or split the unpublished close sequence so
that state consumption occurs after `profile_close` and still before DraftSeal,
with the same unpublished-session discard on every failure. Calling
`RawInvocationChildPortV1::lower_*`, `callout_corridor::emit`,
`continuation_backedge::emit`, or `publish_declaration_exact` again is
explicitly rejected because those are physical writers/mutations.

The ordered next tasks are:

1. **D0-A — define the observation contract:** name the existing source owner,
   the exact W6 row/value relation, and the state operation for each obligation
   above. The contract must be non-emitting, owner-bound, once-only, and must
   include the empty lambda/brand proofs.
2. **D0-B — close transport and tail ordering:** decide how the already-issued
   Dynamic source reaches the bridge, how I11/OuterReturn/header-current
   relations are retained, and where `state.finish()` runs relative to
   `profile_close` and DraftSeal. If this needs a second source authority,
   physical writer, or guessed tail value, remain `NoSafeSlice`.
3. **P0 only after D0-A/B:** implement one private bridge and focused
   missing/foreign/duplicate/partial-consumption tests. Then, and only then,
   observe `DraftAdmission`, `ModuleDrain`, and `ExternalCommit` on the public
   compile path.

D0-A is accepted as a design task only when it names, for every source site,
the existing resolver owner, W6 row/value, state-consumption operation, and
owner/once-only check. It must explicitly cover the `PreludeLocalI` alias to
the `pos` entry value; the current generic `record_completed_local` contract
rejects an initializer/local alias and cannot simply be called with the W6
declaration.

D0-B is accepted as a design task only when it carries the already-issued
Dynamic source `Rc` without re-issuing it and retains the complete loop-carrier
relation: Enter value, Header current/PHI, I11 `V14`, backedge `V17`, and
OuterReturn. The current physical order is not acceptable:

```text
begin -> inspect -> profile_close -> DraftSeal
```

The only acceptable future order is:

```text
W6 physical close
  -> profile_close retains OuterReturn/Header-current evidence
  -> non-emitting state bridge
  -> state.finish()
  -> canonical.finish_for_draft_seal()
  -> DraftSeal
```

`CallableSemanticLoweringState::finish()` by itself is not evidence: it checks
consumption cardinality, not the source-site/physical-value relation. A
bridge that treats `V17` and Header current as interchangeable, or that uses
operation-cursor claims without a state relation, remains `NoSafeSlice`.

This audit changes no runtime route and adds no semantic receipt. The working
tree was clean before the audit; the only intended changes are this card and
the compact `CURRENT_STATE.toml` pointer.

### State-bridge audit and ordered task split

The selected Dynamic branch must not call the ordinary body lowerer merely to
make the state counters look consumed. `RawInvocationChildPortV1::lower_body`,
`lower_statement`, and `lower_expression` are physical lowering entrypoints;
using them before W6 would create a second physical route and duplicate body
effects. The bridge therefore belongs at the selected-Dynamic adapter, which
already owns `body`, the selected package input, and `inner.callable_ledger`.

The accepted contract shape is:

```text
resolver source loan + exact body relation
  -> existing CallableSemanticLoweringState loan
  -> W6-owned physical evidence is consumed once by a private non-emitting bridge
  -> state.finish()
  -> existing finish_unpublished_draft()
  -> existing package key completion / collector admission
```

The bridge may reuse the existing state methods and the existing full Dynamic
demand/physical session, but it returns only `Result<(), typed_error>` inside
the adapter. It does not create, store, or transport a new `Verified*`,
`Prepared*`, or `Consumed` product. The resolver-backed body remains the source
authority (`FunctionSourceViewV1::root_body()` and its checked cursor); the
physical session remains the physical evidence authority; the bridge only
co-seals their already-issued relations and advances the existing state owner.

The accepted D0-A/D0-B mapping is fixed as follows:

| Source/state obligation | Existing W6 evidence | Private non-emitting observation |
| --- | --- | --- |
| formal entry `src/pos/end/pred_chars` | `DynamicV2OpenedFormalHeaderV1` values V0..V3 | existing `install_entry_values` with an exact static entry snapshot |
| `PreludeLocalI` alias | induction declaration plus `pos` entry value V1 | dedicated `observe_preloop_alias`; never generic `record_completed_local` |
| `ChLocal` | I6 normal result V10 plus exact declaration publication | `observe_existing_local` once for the distinct binding/value |
| condition/substring/indexOf reads | exact W6 operation rows and V0/V2/V4/V6/V7/V10/V11 views | `observe_variable_site` with source binding and expected W6 relation |
| `InnerReturnI` | I11 canonical read retained as V14 | `observe_variable_site` against the retained I11 row |
| `StepReadI` and assignment | I13/V15 and I16/V17 | existing source-backed rebind prepare/commit, with the source relation and W6 values co-sealed |
| `OuterReturnI` | `profile_close` After read receipt | `observe_tail_site`; do not reread After or equate it with V17 |
| direct lambdas/brands | resolver-owned exact empty sets for the fixture | explicit zero-cardinality checks; no default map or missing-row inference |

The loop-carrier evidence remains separate and is never reduced to a
`ValueId == ValueId` assertion:

```text
Enter/pos       = V1
Header current  = formal-header canonical binding receipt
InnerReturn     = I11 / V14
Backedge        = I16 / V17
OuterReturn     = profile-close After binding receipt
```

The bridge checks owner, canonical session/brand, induction binding, source
site, W6 producer row, and physical block for each relation. It does not assert
`V1 == Header current`, `V14 == Header current`, `V17 == Header current`, or
`OuterReturn == V17`. I11 adds V14 to the existing physical value ledger without
emitting an instruction; `profile_close` returns its already-issued After
receipt so the bridge cannot create a second PHI/read observation.

The existing Dynamic `Rc` is transported through A-prime demand by cloning the
already-issued handle before the selected input is consumed. The bridge may
call `from_shared_source` with that handle, but it may not rerun the source
issuer or reconstruct source state from AST, digest, name, or physical demand.

The close order is fixed:

```text
W6 cleanup/operation close
  -> profile_close retains OuterReturn/Header-current evidence
  -> SelectedDynamicBodyStateBridgeV1 (once, non-emitting)
  -> CallableSemanticLoweringState::finish()
  -> canonical.finish_for_draft_seal()
  -> DraftSeal
  -> collector commit
```

The bridge is reached through the existing unpublished-session discard path.
Any bridge or state error leaves the function unpublished and prevents DraftSeal,
collector drain, external commit, fallback, and backend observation.

The task order is now implementation-first within this bounded P0:

1. **P0-A — transport and close seam:** carry the existing Dynamic `Rc`, retain
   I11 V14 and the `profile_close` After receipt, and move bridge invocation
   after `profile_close` but before `state.finish()`/DraftSeal.
2. **P0-B — observation bridge:** implement the private exact-once source/W6
   mapping above, including the explicit pre-loop alias and distinct `ChLocal`.
   Do not call `RawInvocationChildPortV1::lower_*`, rescan AST, issue a second
   source product, or add a physical writer.
3. **P0-C — focused evidence:** cover missing/foreign/duplicate/partial
   consumption and verify no instruction/type/ledger publication occurs before
   a rejected bridge. Add a reusable caller/ordering guard and keep touched
   production files below the 760/800-line limits.
4. **Following publication evidence:** run the unchanged production fixture
   through public `compile_normal` and observe `DraftAdmission`, `ModuleDrain`,
   and `ExternalCommit` only after the bridge and state finish are green.

If any counter lacks an existing source/W6 relation, or the bridge would need
to allocate/emit independently of W6, stop at `NoSafeSlice` and design that
missing authority rather than adding a synthetic completion flag.

Finite state routing for this D0 is:

| State | Meaning | Allowed next state |
| --- | --- | --- |
| `Unavailable` | selected Dynamic package or publication input is absent | typed reject; no I9 effect |
| `Selected` | named non-test compile route and I9 row are selected | `BodyConsumptionReady` only after exact source/body coverage is proven |
| `BodyConsumptionReady` | selected source rows and Dynamic operation demand are consumed exactly once, state.finish() succeeds, and no duplicate physical lowering occurred | `DraftAdmission` |
| `DraftAdmission` | completed draft is held by the collector | `ModuleDrain` |
| `ModuleDrain` | collector preflight/commit has inserted the draft into `current_module` | `ExternalCommit` |
| `ExternalCommit` | compiler-owned verification and external commit succeeded | terminal D0 evidence |
| `Deferred` | body consumption or publication stage is not yet evidenced | design stop; no runtime fallback |
| `Rejected` | typed preflight/verification failure | unpublished discard only |
| `NoSafeSlice` | stage owner, exact fixture, or atomic failure boundary cannot be proved | remain in design stop |

The generic dispatcher being caller-zero is useful census evidence but is not a
retirement proof: the shared legacy leaf still has non-test compatibility and
canary callers. Generic retirement requires a separate selected production
consumer and a complete old-leaf caller census.

P0 acceptance is one real compile-path fixture with exact once-only body/state
consumption, explicit alias/tail relation checks, V14/After evidence retained
without re-emission, typed unpublished discard on every rejection, and zero
`lower_*`/AST-rescan/fallback/second-writer edges. `DraftAdmission`,
`ModuleDrain`, and `ExternalCommit` remain later evidence; they are not claimed
by this bridge P0. If any counter lacks an existing source/W6 relation, if the
bridge requires a second AST authority or physical writer, or if state failure
cannot discard the unpublished session, return to `NoSafeSlice`.

## Current census

The generic Loop strict writer P0 remains caller-zero; the selected Dynamic
handoff is now its one named non-test consumer:

```text
CanonicalLoopCompareI64WriterV1::emit production callers = 1
CanonicalLoopCompareI64WriterV1::emit focused test callers = 3
emit_loop_segment_operation_dispatch_v1 non-test callers = 0
emit_prepared_pure_operation_v1 non-test callers = 0
old I9 emit_compare_i64_at_with_dst callers = 0
selected I9 Dynamic V13 post-append values.publish callers = 0
```

The old `emit_compare_i64_at` remains a shared legacy leaf with callers in
`pure_operation_emitter.rs` and other canonical canary/compatibility areas.
Those callers are evidence of existing physical routes, not permission to
connect the strict writer. The generic Loop segment dispatcher is still
caller-zero outside `#[cfg(test)]`; it is not the production caller for this
row.

## Named production caller decision

The active production edge is:

```text
normal_callable_semantic_loan_port.rs:lower_cataloged_static_box_method
  -> assemble_unpublished_selected_dynamic_w6
  -> callout_corridor::emit
  -> i8_i9_control::emit
  -> I9 normal-landing Compare
```

The exact I9 row is co-checked against `I9`, `V11`, `V12`, and `V13` by
`DynamicV2CompareI64CapabilityDemandV1`; the physical normal landing is created
by the canonical CFG session, `V11` is the I7 normal-result definition, and
`V12` is emitted immediately before the Compare in that same landing. This is
now the named production handoff: the strict writer accepts only the
canonical same-block witnesses reissued from those exact Dynamic views.

The I7 header Compare is explicitly excluded: its current/formal operands
cross the formal/header relation and are outside the C-prime same-block slice.

The handoff is now connected. The active route retains
`DynamicV2PhysicalValueLedgerV1` as the sole I9 publication owner, rebinds the
exact V11/V12 views through canonical same-block Integer witnesses, reserves
V13 before the strict append, and commits V13 from the writer definition
source. A generic Dynamic-to-Loop adapter, second dispatcher, post-append
publish, or legacy fallback is absent from the I9 route. Live module
publication and old generic-loop retirement remain outside this card.

## Authority and handoff table

| Handoff fact | Existing owner | Required connection proof |
| --- | --- | --- |
| Compare operation, item, result key, and schedule order | verified Recipe/operation row | row identity and result key are passed once; no reclassification |
| logical target to physical block | `DynamicV2PhysicalTargetSetV1` plus canonical CFG session | exact Dynamic brand/owner/normal landing; no Builder cursor lookup |
| lhs/rhs source values | `DynamicV2PhysicalValueLedgerV1` | exact V11/V12 producer/result/representation/block views are re-bound once by the private I9 issuer to canonical same-block Integer witnesses; no Loop-ledger projection |
| Compare destination | `CanonicalSsaFunctionSessionV2` | fresh wrapped destination capability; no raw allocator in dispatcher |
| Bool result fact | prepared Compare type plan | all type conflict checks before append; commit after the writer definition |
| result publication | `DynamicV2PhysicalValueLedgerV1` | reserve V13 before append and commit only from the writer-owned definition source; no second I9 publication ledger |
| physical mutation | strict writer/shared append core | exactly one append; no legacy retry or post-append fallible check |

The Loop operation ledger remains a separate generic caller-zero lane. It is not
an authority or transport requirement for this selected Dynamic row. Re-issuing
V11/V12 into it would duplicate publication state without adding a proof that
the strict writer needs.

## Worker audit and landed handoff

The read-only worker confirmed the following decision:

```text
implementation state: NoSafeSlice
design shape: B, conditionally accepted
```

The selected shape is:

```text
Dynamic demand + Dynamic V11/V12 views
  -> private I9 handoff
  -> canonical owner/target and unique same-block Integer witnesses
  -> strict writer
  -> Dynamic V13 commit
```

The alternative of projecting V11/V12 into
`LoopOperationValueLedgerV1` and publishing V13 into both ledgers is rejected
for this row. It would create two publication authorities for one selected
physical value, while `CanonicalLoopCompareI64WriterV1` already accepts the
canonical operand witnesses directly and does not require a Loop ledger.

The landed bounded implementation task was:

```text
MIR-LOOP-COMPARE-I9-HANDOFF-PREPARE-D0
```

It has exactly four deliverables:

1. Bind `DynamicV2PhysicalSessionBrandV1` to the same
   `FunctionOwnerIdV1` used to construct `CanonicalSsaFunctionSessionV2`.
2. Rebind exact Dynamic V11/V12 views once through
   `prepare_existing_same_block_integer`; do not add a generic Dynamic-to-Loop
   adapter or let `canonical_ssa` depend on the Dynamic ledger.
3. Add a private Dynamic V13 reservation/commit pair. Reservation is the last
   fallible step; commit consumes the strict writer's definition source and is
   infallible.
4. Specify the handoff states and focused acceptance before enabling the named
   production edge.

The implemented order is:

```text
I9 demand/row and Dynamic brand-owner check
-> exact Dynamic V11/V12 views
-> canonical same-block witnesses and open target
-> fresh destination and Bool plan
-> Dynamic V13 reservation
-> strict writer one-shot append
-> Bool commit
-> Dynamic V13 commit
```

If owner binding or Dynamic reservation cannot be made non-forgeable without
raw-value reconstruction, this task returns to `NoSafeSlice` rather than
introducing a second ledger or a post-append repair.

## Implementation authorization

The B design is accepted for this bounded implementation slice. This is not a
claim that the production edge is complete; it authorizes only the missing
transport/proof connection needed by the named I9 row.

The implementation must preserve these exact contracts:

```text
DynamicV2PhysicalSessionBrandV1::for_owner(demand.identity().owner())
  -> Dynamic ledger and canonical SSA share one function owner

Dynamic V11/V12 view
  -> CanonicalSameBlockIntegerRequestV1
  -> CanonicalSsaFunctionSessionV2::prepare_existing_same_block_integer
  -> strict writer operand witness

Dynamic V13 vacant slot
  -> private PendingDynamicV2PhysicalValuePublishV1
  -> strict Compare append
  -> Bool commit
  -> Dynamic V13 commit(definition source)
```

The Dynamic brand constructor and reservation constructor remain private and
have exactly one production caller in the selected I9 handoff. The reservation
must retain the producer, result, target brand, destination, and representation
already admitted by the I9 demand; it may not infer or repair any of them.

Focused acceptance is required before the handoff is called complete:

```text
positive:
  I9 normal landing, V11/V12 exact views, unique same-block Integer defs,
  one strict Compare append, one Bool fact, one Dynamic V13 publication

negative:
  foreign owner/brand, wrong target, missing or duplicate definition,
  cross-block/parameter operand, wrong representation, duplicate/reserved V13,
  Bool type conflict, strict preparation failure, old-leaf fallback

atomicity:
  every preparation reject leaves instruction/type/Dynamic-ledger state
  unchanged; pending reservation Drop poisons and the outer draft discards
```

The old `emit_compare_i64_at_with_dst` call is zero for this I9 row in
the same production connection series. Generic Loop dispatcher callers,
I7-header Compare, and other operation families remain outside this slice.

The dispatcher may sequence these existing authorities, but it may not become
a second CFG/SSA/ledger/source authority. In particular, `state.get()` and the
old `emit_compare_i64_at` result path cannot be the canonical handoff contract.

## Landed implementation evidence

The bounded code connection is now present:

```text
DynamicV2PhysicalSessionBrandV1::for_owner(demand.identity().owner())
  -> exact V11/V12 Dynamic views
  -> canonical same-block Integer witnesses
  -> fresh destination + Bool plan
  -> Dynamic V13 reservation
  -> one strict Compare append
  -> Bool commit + Dynamic V13 commit(definition source)
```

Observed focused evidence:

```text
cargo test --lib selected_dynamic_physical_emitter
  8 passed

cargo test --lib same_block_operand
  3 passed

cargo test --lib compare_i64_writer_tests
  3 passed
```

The reusable structural guard is
`tools/checks/rust_mirbuilder_loop_compare_connect0_guard.sh`. Its final green
result, `cargo check`, and pointer/SSOT checks are recorded closeout evidence.
No live module publication,
generic Loop dispatcher connection, or old generic-loop retirement is claimed.

## Finite design states

| State | Meaning | Effect | Next |
| --- | --- | ---: | --- |
| `CallerSelected` | Selected Dynamic I9 normal-landing route and exact Compare row are named | none | handoff design |
| `HandoffUnresolved` | I9 is named, but Dynamic target/value and canonical/owner-bound receipts are not co-sealed | none | design only |
| `HandoffPrepared` | Dynamic owner/target, exact operand views, same-block witnesses, destination, Bool plan, and Dynamic V13 reservation are co-sealed | none | strict writer |
| `AppendedPendingCommit` | strict writer appended the one Compare and returned its definition source | one Compare | Bool/Dynamic commits only |
| `Committed` | writer definition, Bool fact, and Dynamic V13 publication completed | one Compare | caller-specific postcondition |
| `RejectedBeforeEffect` | typed relation or preparation failure | none | outer unpublished discard |
| `Poisoned` | a pending Dynamic reservation was dropped before commit | one attempted append at most | outer unpublished discard only |
| `NoSafeSlice` | a required fact comes only from old leaf/cursor/test evidence | none | return to design |

`CallerUnselected` is not a runtime disposition and must not be converted to
`NonCandidate`, `Declined`, or a legacy fallback. It is the current SSOT
development state.

## Required closeout evidence

1. Run the CONNECT0 guard and record one selected non-test writer caller,
   zero I9 legacy/fallback edges, and the source-size limits.
2. Keep the exact row handoff from the caller's verified operation/target to
   the C-prime operand witnesses without re-pairing by name, ordinal, or raw
   `ValueId`.
3. Keep the same-session owner binding for the Dynamic brand and the exact
   normal landing used by V11/V12.
4. Keep Dynamic V13 reservation as the last fallible step, followed by strict
   append, Bool commit, and Dynamic ledger commit only.
5. Record focused positive, negative, and reservation-poison tests, then run
   `cargo check` and the current-state pointer guard.
6. Do not claim live publication, generic Loop dispatcher activation, or old
   generic-loop retirement in this card.

## NoSafeSlice

Keep this card at `design_stop` if I9 cannot receive a canonical open-target
witness, if either Dynamic operand lacks a unique same-block definition, if the
Dynamic brand cannot be bound to the canonical function owner, if the Dynamic
result slot cannot be reserved before append, if the active route needs
cross-block/parameter operands outside C-prime, if a legacy leaf is needed
after strict rejection, or if connection requires a second target/value/ledger
authority. The Generic dispatcher remains caller-zero and cannot be promoted
by test evidence. Do not solve these gaps with a Builder-cursor adapter, a
default/empty receipt, a Loop-ledger re-publication, or a post-append repair.
