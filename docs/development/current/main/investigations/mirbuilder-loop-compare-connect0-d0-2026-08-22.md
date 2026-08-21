Status: CONNECT0 handoff/evidence closeout landed; live-publication boundary is deferred on full-body consumption; implementation and retirement remain gated
Task: MIR-LOOP-COMPARE-LIVE-PUBLICATION-FULL-BODY-CONSUMPTION-P0
Date: 2026-08-22
Priority: prove complete selected-Dynamic body consumption before claiming collector drain or external commit; keep backend and generic retirement closed
Parent: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
PreviousCard: MIR-LOOP-COMPARE-CONNECT0-EVIDENCE-D0
NextCard: MIR-LOOP-COMPARE-LIVE-PUBLICATION-FULL-BODY-CONSUMPTION-P0 (same rolling card)
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

The publication audit is refined by the following prerequisite task:

```text
MIR-LOOP-COMPARE-LIVE-PUBLICATION-FULL-BODY-CONSUMPTION-P0
```

Six-line brief:

```text
Decision: accept the static selected-Dynamic production path, but defer live publication until complete source-body consumption is evidenced on the same path; do not open backend emission or generic Loop retirement.
Source authority + canonical issuer: the existing selected Dynamic I9 demand/ledger and private I9 handoff own Compare facts; collector drain and external commit own module publication transitions.
Non-authority: collector admission alone, `current_module` observation alone, test helpers, generic dispatcher caller-zero, old shared Compare leaf, backend/object output, and any fallback route.
Fail-fast boundary: all selected-Dynamic source/body/operation consumption must complete at `package_port` before `finish_unpublished_draft`, collector drain, or external commit; any failure discards the unpublished session and cannot fall back.
Smallest next slice: drive unchanged `parser_scan_loop_box.hako` through public `compile_normal`, prove every selected-Dynamic source operation is consumed, then observe one `DraftAdmission`, one `ModuleDrain`, and one `ExternalCommit`.
Non-claims: no new semantic receipt, no generic dispatcher activation, no cross-block dominance, no Const/Binary migration, no shared old-leaf retirement, no LLVM/VM/object promotion, and no performance work.
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

The first P0 deliverable is therefore an existing-owner consumption
contract/census at the package-port completion boundary, not a new semantic
receipt. `NormalCallableSemanticPackagePortV1::complete()` currently proves
only selected-key coverage: `with_selected_*` marks a key after its callback
returns. It must not be relabeled as proof that the selected method body was
consumed. The selected Dynamic branch currently receives `body` but passes
`inspect = |_| Ok(())` to W6, so the body-consumption fact is still absent.

The P0 must make the existing source/physical-demand owner consume and
validate the exact selected body before `finish_unpublished_draft`; the
package port may close only after that body-consuming operation returns
successfully. A code/API boundary may be introduced only as part of that
existing owner, for example a private consuming operation whose input is the
selected source loan plus the transported body and whose output is the
already-existing full Dynamic demand/plan. It must co-seal source identity and
body identity once. Do not add a default `Consumed` receipt, treat an empty
body as consumed, rescan the AST in a second authority, or create a
Dynamic-to-Loop adapter. No production switch or old-edge deletion is required
for this P0; generic retirement remains separately gated by the shared legacy
leaf census.

The acceptance therefore has two independent facts:

```text
package completion = every selected key was consumed exactly once
body completion    = every selected source/body operation was consumed by
                     the existing Dynamic demand/physical owner exactly once
```

Neither fact may stand in for the other. If the existing owner cannot consume
the unchanged body without a second source authority, the result is
`NoSafeSlice`, not a synthetic success marker.

Finite state routing for this D0 is:

| State | Meaning | Allowed next state |
| --- | --- | --- |
| `Unavailable` | selected Dynamic package or publication input is absent | typed reject; no I9 effect |
| `Selected` | named non-test compile route and I9 row are selected | `BodyConsumptionReady` only after exact source/body coverage is proven |
| `BodyConsumptionReady` | selected source rows and Dynamic operation demand are consumed exactly once before publication | `DraftAdmission` |
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

Acceptance for this P0 is one real compile-path fixture, exact once-only
source/body consumption before draft finishing, explicit stage
receipts/observations for `DraftAdmission`, `ModuleDrain`, and
`ExternalCommit`, zero selected-I9 fallback/retry, unchanged failure
atomicity, and a reusable guard. If the fixture reaches only a helper or
test-owned collector, if the body is not consumed before drain, or if
publication requires a second Dynamic/Loop ledger or backend authority, keep
`NoSafeSlice`.

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
