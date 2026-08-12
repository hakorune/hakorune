---
Status: active compact card
Date: 2026-08-12
Scope: selected Dynamic callable, canonical session admission, hako.text.scan@1,
  AOT/LLVM production activation
ParentHistory: docs/development/current/main/design/archive/dynamic-fault-exit-transaction-d0-history-2026-08-10.md
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/ring2-provider-link-abi-lifecycle-ssot.md
  - docs/reference/mir/loop-recipe-contract.md
---

# Dynamic callable current card

## Current capsule

Current decision: the final `VerifiedDynamicExitTransactionCoSealV1` is the
selected cohort's sole semantic plan. `CanonicalTrivialBindingSsaPlanV1` is a
different family and must not be extended to accept this Loop. The installed
package port remains the exactly-once transport owner; the existing A-prime
demand/emission plan opens the existing canonical CFG/SSA/PHI session inside
that scoped loan.

Current implementation status: W0-W3 semantic/header/admission, strict
CodePoint entries, checked CallOut ABI, generation-aware neutral lease owner,
test-only metadata/link facts, the I8 unpublished canary, and neutral
CheckedCallOut MIR R0 are landed. Production still uses the selected raw
AST/JoinIR edge and every production CheckedCallOut/LLVM/link caller remains
zero. The next bounded row is the full unpublished physical session.

Next ordered task: `PHYSICAL-SESSION-I0-E`. The lease identity and neutral
CheckedCallOut R0 prerequisites are closed; W4 remains a non-production
unpublished session until the complete activation cell is ready.

Production stop line: provider/AOT/runtime activation and the selected
production switch remain closed until both R0 prerequisites and the complete
activation cell are green. No trivial-plan widening, second
Completion/If/profile, raw AST repair, arbitrary session pairing, fallback,
retry, or Rust-VM DynamicV2 consumer may cross the seam.

Retirement finish line: one atomic AOT activation consumes the selected package
loan through exact-two DraftSeal, removes the selected old edge in the same
commit, and leaves provider/selector/registry/image reselection, legacy
fallthrough, fallback, retry, and Rust-VM DynamicV2 callers at zero.

## Accepted design decision

```text
Decision:
  Build one atomic AOT activation from the final Dynamic program and the
  existing A-prime demand/emission plan. Do not widen the trivial family or
  land a provider/session fragment as a selectable route.
Source authority + canonical issuer:
  Installed package same-batch loan + VerifiedDynamicExitTransactionCoSealV1;
  retained CoreMethod rows own callable result/effect, the normalized TextScan
  contract owns the complete role/profile/lifecycle contract, and one neutral
  AOT export artifact owns the strict physical entry/ABI declarations.
Non-authority:
  generic trivial analysis, mutable compatibility registry, selector/name
  lookup, generic String, raw AST/MIR inference, LLVM, and Rust VM.
Fail-fast boundary:
  complete TextScan/provider symbolic AOT admission and session authority
  validate before Builder mutation. Exact image/digest/symbol validation occurs
  only at link and must succeed before executable publication.
Smallest next slice:
  PHYSICAL-SESSION-I0-E; consume one move-only activation aggregate and one
  Recipe-order cursor for the full unpublished physical session.
Non-claims:
  no Dynamic registry, runtime lookup, VM feature, generic fallback, retry,
  legacy collector key, or production switch before all subrows are green.
```

### Why the former trivial-plan premise is rejected

The target is a mixed typed/Dynamic Loop with an inner If Return. The generic
trivial verifier has no Loop arm, rejects Return inside If, and owns its own
trivial profile, If control, and Completion. Widening it would create a second
semantic planner beside the already-complete Dynamic Recipe/JoinSig program.

The correct products already exist:

```text
SelectedCallableLoweringInputRefV1
  -> VerifiedDynamicExitTransactionCoSealV1
       owns source / mixed Recipe / JoinSig / Completion / cleanup / exits
  -> VerifiedAPrimeI64PhysicalDemandV1
  -> PreparedSelectedDynamicV2EmissionPlanV1
```

The landed pre-activation seam is:

```text
borrowed sole Completion
+ Dynamic-program-owned Loop control disposition
+ exact same-source resolved input
+ move-only A-prime emission plan
  -> existing canonical CFG/SSA/PHI session
```

`PreparedProgramRootWorkPlanV1` stays a root scheduling owner. It does not gain
a borrowed canonical-plan field, avoiding a self-reference and foreign-plan
pairing surface.

### Landed preflight invariants

The body-free physical header/effect work is already landed and remains a hard
precondition of activation:

```text
catalog physical symbol + arity
+ exact declared parameter/return representations
+ verified operation-program EffectMask projection
  -> APrimePhysicalFunctionHeaderV1
  -> create_resolved_function_skeleton(header facts)
```

The selected route never formats a symbol from the raw function name, scans a
body with `contains_value_return`, or supplies a Builder-fixed effect mask.
Header/Completion/control/executable validation finishes before the unpublished
Builder session opens. `PureRead` is a callable semantic effect; it does not
erase the Dynamic invocation outcome, Fault, suspension, or lifecycle axes.

The cataloged-method transport correction is landed. The adapter moves its one
already-sealed `NormalCatalogedBoxMethodDraftAdmissionV1` into the scoped
package loan/A-prime demand, and issuer-side `seal(source_key)` is zero. The
same admission supplies the physical header and remains the future cataloged
Box-method collector identity; no raw key is resealed downstream.

## Final owner graph

```text
InstalledNormalCallableSemanticPackageV1
  owns batch + selected mapping + parameter contracts + final Dynamic program
             |
             | NormalCallableSemanticPackagePortV1
             | exactly-once HRTB selected loan
             v
SelectedCallableLoweringInputRefV1::Dynamic
  same-source ResolvedFunctionLoweringInputV1
  + &VerifiedDynamicExitTransactionCoSealV1
             |
             +-> borrowed canonical-session authority
             |     sole Completion
             |     Dynamic-owned Loop If disposition
             |     common outer If rows = 0
             |
             +-> issue_selected_a_prime_i64_physical_demand
                    |
                    v
             PreparedSelectedDynamicV2EmissionPlanV1
                    |
                    | private TextScan loan
                    | + normalized ProviderSlot artifact
                    | + neutral embedded-AOT export facts
                    | + canonical String alias projection
                    | -> consuming admission
                    v
             PreparedSelectedDynamicV2AotActivationV1
               immutable admitted rows + PreparedAotExecutableAdmissionV1
               strict I6/I7 entry IDs/lanes + PlanStamp + V10 lease capability
                    |
                    | validates, then opens one scoped session
                    v
             CanonicalSsaFunctionSessionV2
               sole CFG / Binding SSA / PHI owner
                    |
                    v
             site-keyed Completion claims
             -> DraftSeal prepare: Return x 2
             -> DraftSeal commit
             -> Collector / atomic MIR-module candidate publish
                (not executable publication)
                    |
                    v
             LLVM object + AOT link finalizer
             -> exact ProviderImageId / artifact digest / resolved entries
             -> RuntimeExecutablePlanV1
             -> executable publication; link failure publishes no executable
```

The scoped loan may yield a private view, not a durable semantic receipt. The
view cannot escape the callback and exposes no raw AST, Recipe, JoinSig,
Completion parts, `ValueId`, or `BasicBlockId`.

## Ordered implementation DAG

The former session-admission Decision and its canonical-session/I8 BoxShape
series are closed. Their detailed evidence lives in `ParentHistory` and git.
The only active parent row is the following atomic production replacement.

### 1. `DYNAMIC-V2-AOT-PHYSICAL-ACTIVATION-I0` — active BoxCount

Change:
  consume the retained I6/I7 CoreMethod rows into one complete TextScan
  AOT admission, lower the whole selected Loop through the existing canonical
  session and exact-two DraftSeal, finalize the exact linked executable plan,
  admit the completed draft with a cataloged Box-method key, and delete the
  selected raw AST/JoinIR edge in the same activation unit.

Contract:
  `CoreMethodContractBox` is the sole callable result/effect authority. The
  normalized TextScan contract owns only the complete two-role grouping,
  shared CodePoint profile, and lifecycle requirements. A neutral AOT export
  artifact owns symbolic strict entry IDs and ABI declarations; the runtime
  type registry owns String/StringBox vocabulary. `ProviderAdmissionSeal`
  owns provider/ABI admission and issues one canonical Text branch plus a
  symbolic `PreparedAotExecutableAdmissionV1`. Only the post-object AOT link
  finalizer may issue an exact `RuntimeExecutablePlanV1` with image digest,
  resolved entry and the carried compile-session `PlanStamp`. The session owns
  physical values, CFG/SSA/PHI, cleanup, and Completion claims. No layer
  re-searches selector, name, generated rows, registry, provider, or image.

Acceptance (not yet landed):
  exact two-role admission, immutable deterministic admitted registry,
  receiver-bearing symbolic AOT admission, strict AOT/LLVM I6/I7 leaf, exact
  link-time RuntimeExecutablePlan, one V10 lease and End, I7 ImmediateI64 with
  no lease, full I0-I16/control/backedge close, two Completion claims and
  physical Returns, one cataloged Box-method collector handoff, selected
  canonical caller=1, selected old edge=0, and all focused tests/guards green.

Stop:
  missing/foreign/duplicate Core row, incomplete role coverage, alias
  ambiguity, wrong symbolic entry/stamp/lane/lifecycle, or Builder mutation
  before pre-MIR validation rejects the MIR candidate. A stale/foreign linked
  image, digest, ABI, or symbol rejects executable publication. Synthetic
  return join/PHI, legacy key, generic fallthrough, fallback, retry, sentinel
  repair, or Rust-VM dependency rejects the cell.

The activation is one production product. Work-branch checkpoints may be
small, but none is an independently selectable mainline route. Closed details
live in `ParentHistory` and git:

```text
W0  b7ecfd161a                         catalog admission transport
W1  ca193378ce                         normalized TextScan/export authority
W2  8e94d95d26..e721a300ec             symbolic admission/header/session seed
W3  7a9728e5ff..d3c25a5af9             strict entries, checked ABI, test-only link facts
```

All W0-W3 production LLVM/CallOut/link callers remain zero. The remaining
order is lease identity R0, neutral CheckedCallOut R0, full physical session,
exact-two collector, and atomic selected cutover.

#### `DYNAMIC-V2-CALLOUT-CFG-OUTCOME-REPRESENTATION-D0` — accepted

```text
Decision: CheckedCallOut is one canonical MIR terminator; its Normal result is a separate first instruction in the site-local Normal landing, never a terminator dst or edge-defined value.
Source authority + canonical issuer: retained AOT admission/call rows own entry and lanes, the exit transaction owns Normal/Fault meaning, the function-local site plan owns physical shape/effect/slots, and CanonicalSsaFunctionSessionV2 alone issues CFG and SSA state.
Non-authority: instruction/cache/JSON/LLVM do not issue Recipe, provider, ABI, lifecycle, selector, result class, or successor meaning; runtime tokens and raw pointers never enter MIR.
Fail-fast boundary: plan/terminator/projection cardinality, brands, shapes, distinct site-local landings, effect-cache parity, ABI/wire revisions, and backend policy reject before publication; unpublished-session failures discard.
Smallest next slice: DYNAMIC-V2-LEASE-IDENTITY-R0, then DYNAMIC-V2-CHECKED-CALLOUT-PHYSICAL-R0.
Non-claims: no full cursor, LLVM production lowering, DraftSeal/collector, executable publication, cutover, fallback/retry, or VM parity.
```

The exact neutral representation is:

```text
CheckedCallOut terminator:
  site_id, receiver, ordered arguments,
  normal_landing, fault_landing, verified_effect_cache
  dst_value = None

CheckedCallOutNormalResult:
  site_id, dst
  first instruction of normal_landing
  ordinary block-local SSA definition

function-local CheckedCallOutSitePlanV1:
  site_id, admitted entry ID, call ABI revision, wire revision,
  normal_shape = EndAuthorizedHandle { lease_slot } | ImmediateI64,
  physical effect authority, outcome_slot, PlanStamp,
  contract_violation_policy = BackendFailStopNoSuccessor
```

Each terminator has exactly one plan and one Normal projection. Normal and
Fault landings are distinct, site-local, and have exactly the CallOut source as
their predecessor. The Normal projection is the only result definition; the
terminator is never a generic `dst_value`, and no shared def-map, dominance, or
PHI rule gains an edge-defined exception. Fault publishes no result or lease;
after its site-local chronology is fixed it may jump through ordinary canonical
edges to shared cleanup.

The site plan is the physical-effect authority. The terminator carries only a
verified cache for context-free `MirInstruction::effects()`, and verification
requires equality. `CanonicalCfgSessionV1::emit_checked_callout` alone installs
the terminator, successors and predecessors;
`CanonicalSsaFunctionSessionV2::define_checked_callout_normal_result` alone
installs the landing projection and ordinary SSA definition.

Semantic `Fault` follows the MIR fault landing. Nonzero transport failure,
malformed wire, unknown revision, or `Suspended` from the sync-only TextScan
entry follows the non-rejoining backend fail-stop policy: it is not semantic
Fault, creates no MIR-visible value or successor, and cannot fall back or
rejoin. LLVM may emit the physical conditional branches, but their Normal/Fault
targets and meaning come only from the MIR terminator.

I6 has `EndAuthorizedHandle { lease_slot }`; I7 has `ImmediateI64`. The static
lease slot is not a runtime token. Future End consumes the exact I6 slot once;
I7 and every Fault path issue no lease and no End. JSON carries only neutral
IDs/revisions/shapes. Function pointers, provider objects, session brands,
runtime tokens and raw outcome storage are never serialized.

#### `DYNAMIC-V2-LEASE-IDENTITY-R0` — landed BoxShape

Change:
  replace token-to-raw-handle storage with one host-handle-owner-issued
  generation-aware identity; make raw `issue_end_authorized(handle)` private.

Contract:
  `host_handles` alone captures `{ raw_handle, generation }` and conditionally
  drops under the same slot-table lock. Every slot publication advances a
  non-wrapping generation. The lease table stores that identity; the strict
  public surface remains aggregate publish plus exactly-once End consume.

Done:
  valid aggregate publish/End is one-shot; ordinary drop followed by LIFO reuse
  makes the old token reject as stale while the replacement object remains
  live. Duplicate, foreign, zero, missing, generation mismatch and exhaustion
  tests plus the existing authority guards are green.

Stop:
  raw handle alone, legacy generation zero, drop-epoch inference, unlocked
  check-then-drop, reusable token, public raw issuer, fallback or VM work is
  forbidden. This row opens no production caller.

Receipt (2026-08-13):
  text-only identity capture, non-wrapping slot generation, conditional
  same-slot drop, collision-preserving lease insertion, and LIFO stale-token
  tests are green. The raw issuer remains private; aggregate publish and
  exactly-once End consume remain the only public lease surface.

#### `DYNAMIC-V2-CHECKED-CALLOUT-PHYSICAL-R0` — after lease identity

Change:
  add the neutral site-plan/terminator/Normal-projection vocabulary, one
  canonical CFG issuer, verifier and Rust JSON roundtrip; keep all execution
  backends explicitly unsupported.

Contract:
  one new small CheckedCallOut owner holds IDs, typed Normal shape and rejects.
  Generic instruction/BasicBlock/CFG/SSA surfaces only project that owner;
  result definition stays in the Normal landing and effect stays plan-owned.

Done:
  local plan admission, canonical two-edge emission, ordinary Normal-block
  dominance, I6/I7 typed shapes and test-only JSON parity are green. The
  function-level plan:terminator:Normal-projection census and final landing
  predecessor proof remain the first E0 closeout; LLVM and VM execution
  allowlists reject by name and production issuers/callers remain 0.

Stop:
  duplicate/orphan/foreign site, shared or identical landing, result in
  terminator `dst_value`, cache drift, wrong lease shape, Fault payload,
  backend hidden semantic branch, fallback/retry or any source at 800 lines
  rejects the row.

Receipt (2026-08-13):
  neutral CheckedCallOut owner, function-local site-plan admission, canonical
  two-edge CFG emission, Normal-landing SSA projection, test-only JSON parity,
  and explicit LLVM/VM rejection are green as local evidence. Exact function
  census, fresh SSA destination issuance, and final landing predecessor proof
  are intentionally deferred to E0; production callers remain zero.

#### `PHYSICAL-SESSION-I0-E0` — CheckedCallOut function census / corridor precondition

This is the first private BoxShape substep of `PHYSICAL-SESSION-I0-E`; it is
not a new card, semantic receipt, backend route, or production caller.

```text
Authority:
  CheckedCallOutPlanTableV1 owns admitted plans;
  CanonicalCfgSessionV1 owns terminators/edges;
  CanonicalSsaFunctionSessionV2 owns Normal projections and fresh ValueIds.

Change:
  add one consuming function-level census and verify plan:terminator:Normal
  projection = 1:1:1, site-local distinct landings, exact source predecessor,
  effect-cache/PlanStamp parity, and unique outcome/lease slots. Keep six
  logical target anchors; model I6/I7 Normal/Fault landing pairs as a
  session-private corridor without executing providers or emitting calls.

Acceptance:
  orphan/duplicate/foreign plan, terminator, projection, landing predecessor,
  slot, or stamp rejects before publication; Normal projection destination is
  session-issued, not caller-supplied; I6 shape is EndAuthorizedHandle and I7
  shape is ImmediateI64. Positive/negative focused tests and one reusable guard
  are green.

Non-claims:
  no I6/I7 runtime execution, lease/End consumption, full cursor, LLVM hook,
  DraftSeal, collector, production switch, fallback/retry, or VM consumer.
```

Checkpoint receipt (2026-08-13): the borrow-free function census is now
consumed once by canonical function finish. It closes exact
plan:terminator:Normal-projection cardinality, final site-local predecessor
sets, effect/PlanStamp parity, unique outcome/lease slots, and canonical fresh
Normal-result ValueId issuance. The session-private I6/I7 physical corridor
and result representation/value-ledger publication remain the open half of E0.

`PHYSICAL-SESSION-I0-E0-SITE-PLAN-TRANSPORT-D0` is the design stop before the
corridor: `targets.rs` must not mint site plans or
landing pairs. First transport exactly two already-admitted physical call
plans from the existing capability aggregate into the selected session:
I6=`EndAuthorizedHandle/V10`, I7=`ImmediateI64/V11`, one shared PlanStamp.
The transport is private, move-only, has no selector/entry lookup or parts API,
and is consumed by canonical CFG/SSA before any corridor block mutation. A
missing, duplicate, swapped, foreign, or partial pair is RejectBeforeEffect.
Only after this transport is accepted may the same session allocate the four
site-local landings, emit two CheckedCallOut terminators/projections, publish
their results to its existing value ledger, and satisfy the final census.

Transport owner/issuer: the existing `SelectedDynamicV2PhysicalCapabilityAdmissionV1`
consumes its retained A-prime relation and `PreparedAotExecutableAdmissionV1`;
it issues one private move-only pair, not a new semantic receipt. The selected
emitter consumes that pair through `begin(builder, activation)` before target
allocation. No selector/by-name lookup, `into_parts`, or target-side plan minting.

Checkpoint (2026-08-13): the private transport is implemented and consumed by
the unpublished emitter before target allocation. The next bounded row is
`PHYSICAL-SESSION-I0-E0-CALLOUT-CORRIDOR-D1`: site-local Normal/Fault landings,
canonical Normal-result/value-ledger publication, and final 1:1:1 census only.

Decision (2026-08-13): D2 preflight is landed; D1 must not be split into an
I6-only or I7-only leaf. The smallest safe physical slice is one unpublished
session that adopts the verified formal seeds, emits I0-I5 prerequisites,
emits site-local I6/I7 CheckedCallOut terminators and Normal projections, and
publishes all resulting receipts through the existing V2 ledger.

```text
Decision: task one combined typed-ledger/callout corridor; no new semantic shape.
Source authority + canonical issuer: operation_rows() for order; A-prime formal
  relation for V0..V3; CanonicalSsaFunctionSessionV2 for ValueId/SSA; CanonicalCfgSessionV1
  for CheckedCallOut/edges; existing DynamicV2PhysicalValueLedgerV1 for receipts.
Non-authority: six logical targets, site-plan transport, V1 physicalizer, selector/name
  lookup, caller ValueId, MirType inference, LLVM/runtime/VM, lease/End, DraftSeal.
Fail-fast boundary: missing/foreign/duplicate formal or operation, use-before-produce,
  typed representation drift, PlanStamp/site mismatch, shared landing, or orphan projection
  rejects before publication and discards the unpublished session.
Smallest next slice: consume existing projections (formal Dynamic stays a canonical SSA
  receipt; operation rows provide ImmediateI64/ImmediateBool; admitted site plans provide
  I6 EndAuthorizedHandle and I7 ImmediateI64) in one private corridor for formal->I0-I5->I6->I7.
Non-claims: no I9/control/backedge/cleanup completion, provider/LLVM/runtime activation,
  collector, production caller, fallback, retry, or VM parity.
```

Implementation checkpoint (2026-08-13): D2 preflight remains the complete
15-row dependency/order gate. D1 now consumes one unpublished session through
formal -> I0-I5 -> site-local I6/I7 CheckedCallOut and Normal projections; E1
and E2 close I8/I9 and Fault/End terminals, E3 closes I11/inner Completion,
E4 consumes I13-I16 and seals the Enter+Continuation Header PHI path, and E5
claims the outer Completion and prepares exact-two DraftSeal. Canonical SSA/CFG
remain the sole ValueId/edge owners; publication and collector handoff remain
closed.

#### `PHYSICAL-SESSION-I0-E`
```text
Entry precondition: lease identity R0, local CheckedCallOut evidence, and E0
function-level census/corridor closeout are green.
Decision: consume one move-only activation aggregate and one Recipe-order cursor for all I0-I16/control/cleanup in an unpublished session.
Authority: A-prime demand, admitted CallOut site plans, exit/cleanup projection, target/formal/value ledgers, and canonical SSA/CFG session only.
Acceptance: six logical targets, 15 operations, both call outcomes, V10 lease/End, backedge/PHI and profile close are consumed exactly once; any mismatch discards the session.
E1 landed: `PHYSICAL-SESSION-I0-E1-I8-I9-CONTROL` consumes the I7 Normal landing, emits I8/V12 and I9/V13, then branches canonically. E2 landed: `PHYSICAL-SESSION-I0-E2-FAULT-END` emits I6 Fault without End and I7 Fault with one V10 End, both as successorless terminals. E3 landed: `PHYSICAL-SESSION-I0-E3-INNER-RETURN-THEN` reads I11/V14, consumes the canonical I6 End cutpoint, claims the inner Completion return, and seals ThenTerminal without emitting Return. E4 landed: `PHYSICAL-SESSION-I0-E4-CONTINUATION-BACKEDGE-PHI` consumes I13-I16, emits the Backedge End, jumps Continuation to Header, and closes Header with Enter and Continuation predecessors plus the canonical induction PHI. E5 landed: profile close claims the outer Completion, seals the remaining corridor, and hands exact-two DraftSeal to the existing owner. Publication and collector remain closed.
Non-claims: no CanonicalCallable collector, production publication/caller, fallback/retry, or VM DynamicV2 work.
```

#### `DYNAMIC-V2-PHYSICAL-END-FAULT-TERMINAL-R0`
```text
Decision: accepted BoxCount; `CheckedCallOutEnd` is the neutral physical lease-consumption instruction and non-rejoining `CheckedCallOutFault` is the canonical fault terminal.
Source authority + canonical issuer: retained cleanup/site-plan/JoinSig facts; one move-only lifecycle owner feeds Canonical SSA/CFG. Runtime lease is the execution consumer, not the MIR issuer.
Non-authority: runtime lease API alone, `ReleaseStrong`, existing `Throw`, `After`, V1 physicalizer, generic Call, provider/LLVM/VM, selector/name lookup, fallback, or a second semantic receipt.
Fail-fast boundary: missing/foreign/stale/duplicate lease, End, site/landing/predecessor, Fault rejoin, or profile-close evidence rejects before publication; no I8-I16 cursor or production route is opened by R0.
Landed R0 slice: `ca31203fba` adds the typed MIR vocabulary/issuers and `fe8e70b83a` co-seals cleanup/site-plan facts into a move-only lifecycle plan; full cursor execution remains unpublished.
Non-claims: no full I8-I16/control/cleanup, provider/runtime activation, DraftSeal/collector, production caller, or VM parity in R0.
```

#### `EXACT-TWO-COLLECTOR-I0-F-KEY-AUTHORITY-R0`

Decision: accepted BoxShape; implement the selected identity as
`SelectedNormalCallableKeyV1::Cataloged(CanonicalSameModuleCallableKeyV1)`.
Source authority + canonical issuer: package catalog admission and the retained
Box-method key; `CompletedFunctionDraftV1` must carry that identity once.
Non-authority: FreeStatic-only `CanonicalCallableKeyV1`, `LegacySymbol`, raw
`MirFunction.signature.name/arity`, `into_legacy_collector_parts`, or re-sealing.
Fail-fast boundary: owner/namespace/name/arity/symbol, collector brand, duplicate,
foreign, and second-consume mismatch rejects before collector mutation/publication.
Landed R0/R1 canary: `662e50847b` adds `FunctionDraftKeyV1::CatalogedBoxMethod`
and retains the admission key in a move-only completed-draft projection;
`7502802af9` routes that draft once through the invocation-owned
`ModuleLoweringPortV1` branded collector terminal with the existing
`CanonicalRejectDuplicate` policy. Publication and production cutover remain
closed.
Non-claims: no provider/runtime/LLVM/VM activation, legacy retirement, or
production caller in R0; the selected production collector handoff is next.

#### `SELECTED-CUTOVER-I0-G`

The installed package adapter consumes the selected Dynamic program instead
of forwarding only a source seed. The located Loop keeps its retained method
and admission evidence, invokes the activation cell once, and cannot enter
`lower_loop_or_freeze_v1`. Ordinary/foreign callables keep their existing
route. The same activation commit establishes new selected caller=1, old
selected edge=0, fallback=0, and retry=0.

#### Work-branch and main landing boundary

Implementation may use bounded internal commits on this feature branch:

```text
W0  move existing catalog admission through package/A-prime; delete reseal
W1  normalized TextScan contract + neutral export facts + alias projection
W2  consuming admission + immutable registry + symbolic AOT aggregate
W3  strict runtime leaf + checked ABI/metadata + test-only link verifier
R0a generation-aware carrier lease identity
R0b neutral CheckedCallOut MIR/JSON representation; backend execution closed
W4  full physical session + selected LLVM CheckedCallOut physicalizer
W5  exact-two DraftSeal + CanonicalCallable collector
W6  package cutover + old-edge deletion + guards/docs
```

Every checkpoint before W6 keeps production callers at zero and the capability
closed. Main receives only the complete activation unit (squashed or otherwise
presented as one indivisible activation commit). A provider-only, registry-only,
LLVM-only, link-plan-only, lease-only, PHI-only, or partial-cursor main landing
is forbidden.

Required counts at the I0 terminal:

```text
TextScan roles / provider profile                              = 2 / 1
normalized contract artifact / neutral AOT export artifact    = 1 / 1
provider executable entry IDs                                 = 2
ProviderAdmissionSeal / immutable admitted registry           = 1 / 1
mutable admitted insert / duplicate overwrite                 = 0 / 0
canonical Text receiver branch                                = 1
symbolic AOT admission / strict entries / LLVM consumer        = 1 / 2 / 1
link finalizer / RuntimeExecutablePlan / exact image pin       = 1 / 1 / 1
I6 receipt / lease / End                                      = 1 / 1 / 1
I7 ImmediateI64 / lease / End                                 = 1 / 0 / 0
Completion expected / claimed / physical Return               = 2 / 2 / 2
synthetic return join / Return PHI                             = 0 / 0
selected Box-method collector key / legacy collector key       = 1 / 0
adapter admission move / A-prime catalog reseal                = 1 / 0
new selected caller / selected old edge                       = 1 / 0
runtime lookup / generic fallthrough / fallback / retry        = 0 / 0 / 0 / 0
Rust VM DynamicV2 production consumer                         = 0
```

### 2. `DYNAMIC-V2-SELECTED-LEGACY-RETIREMENT-R0` — after cutover

Delete only after caller-zero evidence:

```text
selected source-seed-only route
selected raw JoinIR edge and legacy finalizer edge
test-side Completion/If reissuance helpers
superseded I8-only canary shell
diagnostic-only raw role/fingerprint authority uses
selected old topology callers
```

Global fixed-topology deletion waits for all remaining callers to reach zero.
H2/H3/H5 parity and the AOT mimalloc gate then run as independent siblings;
both must be green before Hako producer activation.

### 3. `MIRBUILDER-MODULE-DRAIN-CONVERGENCE-D0 -> I0` — after selected cutover

This is a post-cutover publication cleanup, not a second module authority.
First census every production lowering route, then converge the routes onto the
existing one-shot `ModuleLoweringInvocationDrainOwnerV1` and post-drain
finalization owner. The disconnected `module_invocation_cut0_p0` candidate is
not production truth and must not be activated as a parallel drain.

Done requires:

```text
production route census                                      = complete
one production drain owner                                   = 1
one production post-drain finalizer                          = 1
duplicate drain/finalizer callers                            = 0
legacy finalize_function_draft production callers             = 0
candidate-only drain path promoted                           = 0
one-shot drain / atomic publish                              = green
```

The row opens only after the selected AOT caller switch and old-edge
retirement. It does not change semantic source, Recipe, JoinSig, Completion,
provider, or VM ownership.

### 4. `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0` — after legacy retirement

After `DYNAMIC-V2-SELECTED-LEGACY-RETIREMENT-R0`, perform a full production
and test caller census for the fixed-role topology and its old issue APIs.
Only when every caller is zero may the fixed-role receipt, legacy boundary
receipt, and old `issue(...)` compatibility path be hard-deleted. A segment
route is not considered complete merely because a new caller exists; the
retirement row requires proof that no remaining route depends on the old
topology.

Done requires:

```text
fixed-role production callers                             = 0
fixed-role test/guard callers                              = 0
old issue(...) callers                                     = 0
segment route completeness                                 = green
fixed-role receipt / boundary types                        = deleted
compatibility fallback                                     = 0
```

This is a post-cutover BoxShape/retirement row. It cannot be used to bypass
the AOT capability gate or to delete an old path while it still owns a live
production edge.

## hako.text.scan@1 semantic contract

```text
profile: utf8-codepoint-clamped-v1
receiver: canonical Text
aliases: String | StringBox, canonicalized before admission

TextSliceRange / substring/2
  CanonicalText + ImmediateI64 + ImmediateI64 -> CanonicalText
  CP half-open range, endpoint clamp, synchronous
  Normal result = one EndAuthorized lease

TextFindNeedle / indexOf/1
  CanonicalText + CanonicalText -> exact ImmediateI64
  first CP index, empty needle = 0, miss = -1
  Normal result lease = 0, End = 0
```

Selector and diagnostic strings only cross-check dispatch keys. They do not
decide result class, representation, lifecycle, provider, or executable entry.
The strict leaf does not call the environment-selected/generic String surface,
string-to-i64 compatibility parsing, or sentinel-zero helpers.

## Activation file boundaries

Keep one public/session entry per owner and put the new logic in small modules.
Exact filenames may vary only within the owning directories; ownership and
line budgets may not move into the near-limit files.

```text
lang/src/runtime/meta/provider_slot_contract_box.hako
tools/provider_slot_contract_manifest_codegen.py
lang/src/runtime/meta/generated/provider_slot_contract_manifest.json
  normalized TextScan semantic contract; generated Core row identities only

include/nyrt_dynamic_text_scan_v1.h
src/abi/text_scan_aot_export_facts.rs
src/llvm_py/builders/dynamic_v2_text_scan_export_facts.py
  one physical export/ABI owner plus checked projections

src/box_callable/provider_admission/
  seal.rs                 consuming TextScan/alias/export co-seal
  admitted_registry.rs    immutable deterministic selected rows
  aot_admission.rs        symbolic entry IDs/generation/PlanStamp aggregate

src/runtime/dynamic_v2_lease.rs
  one-shot ABA-safe lease issuer/consumer for the strict leaf

crates/nyash_kernel/src/exports/dynamic_v2_text_scan.rs
  strict CodePoint I6/I7 entries consuming the neutral lease API

src/llvm_py/instructions/mir_call/selected_dynamic_v2.py
  short early hook; no provider lookup or generic fallback

src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/
  executable.rs / call_slots.rs / lifecycle.rs / terminal.rs
  move-only aggregate consumption and full session realization

src/bin/ny_mir_builder_aot_provider_plan.rs
  post-object artifact digest/symbol verification and link-plan finalization
```

The AOT driver must pass the exact `--nyrt` artifact path into the link
finalizer explicitly; production code may not rediscover it from an environment
variable. `ProviderImageId` is derived from the verified artifact digest, while
`PlanStamp` remains the carried compile/module-invocation stamp. Static link
success alone is insufficient if the descriptor, ABI, or required symbols do
not match.

## Mandatory cleanup and line-budget gates

These are BoxShape boundaries, not substitutes for the active BoxCount.

Closed and archived in `ParentHistory`/git:

```text
CURRENT-STATE-LIVE-SCHEMA-I0           CURRENT_STATE = compact live pointer
MIRBUILDER-WORKSTREAM-ARCHIVE-R0       closed chronology archived
MIRBUILDER-BUILDER-BUILD-SPLIT-R0      thin facade + four responsibility files
MIRBUILDER-LINE-BUDGET-R0              collector/completion test splits landed
MIRBUILDER-COMPLETION-COMMENT-CLEANUP  site-keyed exact-two wording current
```

Pre-cutover freeze:

```text
src/mir/builder.rs                         794 lines; additions forbidden
src/mir/resolved_value_profile/analyzer.rs 769 lines; freeze or private seam split
crates/nyash_kernel/src/exports/string.rs  694 lines; strict leaf additions forbidden
new/touched Rust source                    design split at 760, hard stop at 800
new activation owner modules              target below 650, mandatory split by 700
LLVM method_call.py / ny_mir_builder.rs    short hook only; plan logic in new module
```

Post-cutover queue:

```text
MIRBUILDER-EMIT-INSTRUCTION-PHASE-SPLIT-R0
  keep one public writer; split private prepare/validate/commit/post-metadata

MIRBUILDER-MODULE-REGISTRY-CLASSIFY-R0
  after caller/cfg census, keep one MirBuilder facade and classify modules as
  state/session, source admission, semantic plans, physical lowering,
  collection/publication, compatibility, and tests/migration. Preserve paths,
  visibility, cfg, and re-exports; delete only caller-zero modules.
```

Each pending BoxShape is a two-to-five-commit refactor series with unchanged
behavior and callers, focused parity/failure tests, `git diff --check`, and all
touched Rust files below 760 lines. It cannot overlap the activation BoxCount.

## Common negative matrix

```text
Ordinary or foreign selected loan                         -> reject/not selected
owner/function/forest/projection/root drift              -> reject
missing/duplicate/extra Completion site                  -> reject
wrong function target or return operand                  -> reject
Loop/local If/inner Return shape drift                   -> reject
catalog admission resealed below the package adapter     -> guard failure
raw physical symbol/body Return/effect inference         -> guard failure
missing/duplicate ProviderSlot or AOT export row         -> RejectBeforeEffect
String/StringBox alias/profile/entry disagreement        -> RejectBeforeEffect
symbolic entry/ABI/generation/PlanStamp drift            -> RejectBeforeEffect
final image/digest/address requested before AOT link     -> guard failure
stale/foreign linked artifact or missing strict symbol   -> link reject
second Completion/If/profile issuance                    -> guard failure
raw AST/Recipe/JoinSig/ValueId handoff                    -> guard failure
arbitrary canonical-session pairing                      -> API/guard failure
borrow escaping package callback                         -> compile failure
provider/LLVM/End incomplete at activation               -> RejectBeforeEffect
selected legacy/generic fallthrough                      -> guard failure
name/ordinal/selector repair, fallback, retry             -> guard failure
Rust VM DynamicV2 provider/receipt/session                -> guard failure
```

## Focused gates

```bash
cargo test -q --lib normal_callable_semantic_package
cargo test -q --lib dynamic_full_body_recipe
cargo test -q --lib selected_dynamic_physical_emitter
cargo test -q --lib completion
cargo check -q --lib

bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dynamic_v2_physical_input_authority_guard.sh
bash tools/checks/dynamic_v2_callslot_wire_authority_guard.sh
bash tools/checks/dynamic_v2_vm_nonconsumer_fence_guard.sh
bash tools/checks/loop_precutover_authority_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

The activation implementation must add one reusable
`tools/checks/dynamic_v2_aot_activation_authority_guard.sh` during W0/W1 and
make it green only at W6. Before W6 it runs in closed mode and requires
production caller=0; at W6 the same guard flips atomically to new=1/old=0. It
owns export/header projection parity, single admission/alias/PlanStamp issuers,
pre-link versus post-link plan boundaries, strict symbols, cataloged Box-key
handoff, and zero VM/lookup/fallback/retry assertions. It is not claimed to
exist or be green in the current docs-only task.

Gate classification (2026-08-12): the focused `completion` command has one
known parent-baseline failure in
`mir::compiler::canonical_physical_completion_p0::compiler_bridge_drains_a_plus_single_route`
(`ReturnValueTypeMissing`, `ValueId(12)`). It reproduces at parent
`b69f5e11fe` and is outside the selected Dynamic activation diff; it remains
recorded as baseline debt, not a green production claim. All currently
existing selected Dynamic/package/Recipe/emitter checks and authority guards
pass; the activation guard described above is a future acceptance item.

## Non-claims

```text
CanonicalTrivialBindingSsaPlanV1 Dynamic expansion
generic all-V2 Loop admission
full String surface or I6-only provider slot
Dynamic-specific registry
runtime provider/selector/image lookup
pre-link final image/address fabrication
generic String compatibility route
Rust VM provider/receipt/session
new Recipe/JoinSig/CFG/SSA/PHI/Completion authority
production cutover before the complete atomic activation cell
fallback / retry / legacy dual-production
```
## History

Detailed landed chronology lives in git history and the historical archive
named in `ParentHistory`. This card owns only the live Decision, next slice,
activation boundary, retirement conditions, and cleanup queue.
