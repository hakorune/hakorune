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
---

# Dynamic callable current card

## Current capsule

Current decision: the final `VerifiedDynamicExitTransactionCoSealV1` is the
selected cohort's sole semantic plan. `CanonicalTrivialBindingSsaPlanV1` is a
different family and must not be extended to accept this Loop. The installed
package port remains the exactly-once transport owner; the existing A-prime
demand/emission plan opens the existing canonical CFG/SSA/PHI session inside
that scoped loan.

Current implementation status: exact-I64 semantic recut, exact-two Completion
and DraftSeal machinery, the Rust-VM nonconsumer fence, neutral output wire,
the I8 unpublished canary, and the complete R0 canonical-session projection
series are landed. The selected emitter now consumes the exact package input,
borrows the final Dynamic program only through its private HRTB authority,
snapshots Completion/control expectations, and opens its unpublished canonical
session internally. Production still uses the selected raw AST/JoinIR edge.

Next ordered task: implement the already-accepted AOT physical activation cell
(`DYNAMIC-V2-AOT-PHYSICAL-ACTIVATION-I0`) with the complete provider contract,
admission, strict LLVM leaf, I6/I7 receipts, End/lifecycle, and atomic selected
production switch. No provider/runtime/LLVM implementation is implied by the
completed R0 BoxShape row itself.

Production stop line: provider/AOT/runtime activation and the selected
production switch remain closed until the projection series is green. No
trivial-plan widening, second Completion/If/profile, raw AST repair, arbitrary
session pairing, fallback, retry, or Rust-VM DynamicV2 consumer may cross the
seam.

Retirement finish line: one atomic AOT activation consumes the selected package
loan through exact-two DraftSeal, removes the selected old edge in the same
commit, and leaves provider/selector/registry/image reselection, legacy
fallthrough, fallback, retry, and Rust-VM DynamicV2 callers at zero.

## Accepted design decision

```text
Decision:
  Reuse the final Dynamic program as the sole semantic plan and the existing
  A-prime demand/emission plan as the sole pre-session physical plan. Do not
  admit this cohort into CanonicalTrivialBindingSsaPlanV1.
Source authority + canonical issuer:
  Installed package same-batch loan + VerifiedDynamicExitTransactionCoSealV1;
  issue_selected_a_prime_i64_physical_demand is the existing co-seal issuer.
Non-authority:
  generic trivial analysis, package-local verify_function, canary semantic
  AST/header re-verification, reissued Completion/If, names/ordinals, provider,
  LLVM, VM.
Fail-fast boundary:
  ordinary/foreign/mismatched identity, authority reissue, arbitrary session
  pairing, borrow escape, double consume, or incomplete physical capability
  rejects before Builder effect.
Smallest next slice:
  DYNAMIC-V2-CANONICAL-SESSION-PROJECTION-R0, a behavior-neutral refactor
  series with the existing unpublished canary as its named consumer.
Non-claims:
  no accepted source shape, provider/runtime feature, LLVM hook, VM feature,
  production switch, generic typed-trivial expansion, fallback, or retry.
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

The missing seam is only the physical session projection:

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
                    | opens its own scoped session
                    v
             CanonicalSsaFunctionSessionV2
               sole CFG / Binding SSA / PHI owner
                    |
                    v
             site-keyed Completion claims
             -> DraftSeal prepare: Return x 2
             -> DraftSeal commit
             -> Collector / Atomic Publish
```

The scoped loan may yield a private view, not a durable semantic receipt. The
view cannot escape the callback and exposes no raw AST, Recipe, JoinSig,
Completion parts, `ValueId`, or `BasicBlockId`.

## Ordered implementation DAG

### 1. `DYNAMIC-V2-SELECTED-SESSION-ADMISSION-D0` — accepted Decision

The previously listed projection row is now opened by this owner decision.
The target is a Dynamic Loop with an inner If Return, so
`CanonicalTrivialBindingSsaPlanV1`, `CanonicalLoweringPreflightV1`, and the
first-family trivial analyzer are not valid session inputs.  Making them
accept this shape would issue a second semantic plan, Completion, or If
authority.

Decision:
  choose one same-source admission boundary that lends the existing Dynamic
  semantic authority to the existing canonical CFG/SSA/PHI engine.
Source authority + canonical issuer:
  the installed package's exactly-once selected loan and its
  `VerifiedDynamicExitTransactionCoSealV1`; the existing
  `VerifiedAPrimeI64PhysicalDemandV1` ->
  `PreparedSelectedDynamicV2EmissionPlanV1` chain remains the only pre-session
  physical plan.
Non-authority:
  `CanonicalTrivialBindingSsaPlanV1`, generic trivial analysis, package-local
  Completion/If reissuance, canary semantic AST/header re-verification, raw
  AST/JoinIR meaning, names/ordinals, provider/LLVM/runtime/VM, and arbitrary
  external sessions.
Fail-fast boundary:
  if owner/function/forest/projection/source-root identity, Completion,
  Dynamic Loop control disposition, or lifetime cannot be lent exactly once
  without clone/reverification, remain `NoSafeSlice` before Builder effect.
Smallest next slice:
  decide the private HRTB/consuming callback shape and whether the existing
  canonical session can consume the borrowed facts without leaking a
  semantic borrow into DraftSeal.
Non-claims:
  no code, new durable `Verified*`/`Prepared*` semantic receipt, provider or
  LLVM implementation, VM work, production switch, fallback, or retry.

Required acceptance:

```text
selected Dynamic -> CanonicalTrivialBindingSsaPlan consumer = 0
selected Dynamic -> CanonicalLoweringPreflight consumer      = 0
Dynamic Completion semantic issuer                          = 1
Dynamic Loop control issuer                                  = 1
canonical-session admission issuer                          = 1
source/Recipe/Completion/If reissue                         = 0
semantic AST/header re-verification                          = 0
foreign/arbitrary session pairing                            = 0
```

Only after this D0 is accepted may the following parked BoxShape row open.

The accepted private boundary is:

```text
VerifiedDynamicExitTransactionCoSealV1
  -> with_canonical_session_authority(HRTB callback)
       borrows the retained Completion and Dynamic Loop control disposition
       and carries owner/target/source-root identity
  -> existing A-prime demand/emission plan
  -> CanonicalSsaFunctionSessionV2::new_selected_dynamic
       Completion consumer = Owned | Borrowed
       control consumer   = Resolved | DynamicProfileOwned
```

The HRTB callback is the only place where the borrowed authority and the
mutable canonical session meet.  It cannot return the borrow, a session, or a
semantic part.  `finish()` snapshots only the borrow-free physical claims and
return kind needed by DraftSeal.  The Dynamic control disposition is a
private view of the already sealed JoinClosure; it is not an empty
`VerifiedResolvedFunctionIfControlV1`, and it is not reissued from the source.

This closes the D0 design question.  The next row is the parked BoxShape
projection, beginning with the completion consumer's `Owned | Borrowed`
storage and borrow-free ready close.

### 2. `DYNAMIC-V2-CANONICAL-SESSION-PROJECTION-R0` — landed BoxShape

The existing Completion consumer now snapshots owned/borrowed expectations,
the final Dynamic program lends its control/Completion authority through a
private HRTB view, and the selected emitter opens the canonical unpublished
session internally. The I8 canary is unpublished and test-only; no provider,
LLVM, VM, fallback, retry, or production caller was opened. Detailed R0
chronology and evidence live in `ParentHistory` and git history.

### 3. `DYNAMIC-V2-AOT-PHYSICAL-ACTIVATION-I0` — atomic BoxCount

This is one activation cell built in small owner modules. Intermediate code
does not become an independently selectable provider or production route.

No provider contract, registry, executable branch, wire, LLVM leaf, runtime
lease, or receipt is landed ahead of this cell.  The complete
`hako.text.scan@1` contract, admission, executable branch, strict AOT leaf,
canonical I6/I7 receipts, and lifecycle must land as one activation unit; an
isolated preparatory authority is forbidden.

Change:
  activate the complete `hako.text.scan@1` provider capability, strict AOT/LLVM
  I6/I7 execution, full Dynamic Loop physical session, exact-two DraftSeal,
  and selected package production caller; delete the selected raw AST/JoinIR
  edge in the same activation commit.

Contract:
  `TextSliceRange` and `TextFindNeedle` are the complete two-role capability.
  `CoreMethodContractBox` and its generated rows remain the sole callable
  result/effect authority (`StringValue` for substring and `I64Value` for
  indexOf). The TextScan ProviderSlot contract is only the complete two-role
  aggregate: it borrows and co-seals those generated rows, then owns the
  shared semantic profile and cross-role lifecycle/capability requirements.
  ProviderAdmissionSeal separately owns selected provider/ABI admission, and
  RuntimeExecutablePlan owns executable binding. No layer may reissue a
  second result/effect table.
  The global provider spine is reused; runtime consumes a presealed executable
  branch and never searches a registry or reselects provider/image/selector.
  The LLVM formal lane is exact and role-bound: `src=0`, `pos=1`, `end=2`,
  `pred_chars=3`; swapped or shifted receipt rows reject before effect.

Activation preflight invariants (P0):

```text
physical symbol/header source       = existing catalog/source identity projection
raw method name -> physical symbol  = 0
raw body return scan in canonical    = 0
canonical skeleton input             = exact physical header + Completion contract
physical header effect source        = borrowed verified operation/effect plan
authority validation before Builder mutation = 1
legacy raw skeleton/body inference   = selected AOT path only, 0
semantic block count chosen by emitter = 0
  DynamicProfileOwned disposition       = explicit unit until the full
                                         operation/control/cleanup cursor
                                         validates profile close
selected collector key               = CanonicalCallable, never LegacySymbol
```

The selected physical symbol must come from the existing cataloged method
admission (`ParserScanLoopBox.skip_while/4` for this cohort), never from
`format!("{name}/{}", params.len())`. The canonical skeleton may allocate
function storage and entry blocks only after the selected package loan,
physical header projection, and Dynamic Completion/control expectations have
validated. It must not call `contains_value_return` or otherwise rescan raw
AST to infer a return shape; the existing legacy skeleton remains a
compatibility route only. These checks are part of the same activation cell,
not a new semantic authority.

Acceptance criteria:
  one consuming ProviderAdmissionSeal, immutable deterministic admitted
  registry, receiver-identity RuntimeExecutablePlan, strict CodePoint AOT leaf,
  I6 V10 value+one-shot lease/End, I7 ImmediateI64/no lease, complete operation
  and control schedule, two Completion claims, two physical Returns, one new
  selected production caller, and zero selected old callers are green. The
  schedule is mechanically derived from verified Recipe order/placement and
  JoinSig control (`Prelude`, `ThenTerminal`, `Continuation`); source-role names
  are diagnostic cross-checks only. The preflight ledger is move-only and has
  no Clone, clone, or split emitter path.

Stop:
  missing/foreign/duplicate contract or image, alias ambiguity, incomplete
  slot coverage, missing capability, lifecycle drift, synthetic return join or
  PHI, generic-method fallthrough, fallback, retry, or VM dependence rejects
  before activation.

Internal implementation order, without creating separate authorities:

```text
hako.text.scan@1 role aggregate
  + borrowed generated CoreMethodContractBox rows (sole result/effect source)
  + A-prime I6/I7 role requirement co-seal
  -> BoxCallableRegistryDraft
  -> consuming ProviderAdmissionSeal
  -> immutable admitted registry
  -> MethodCallRoutePlan / RuntimeExecutablePlan
  -> neutral call-in admission wire + PlanStamp
  -> canonical-session I6/I7 receipts
  -> strict LLVM early hook / CodePoint leaf
  -> V10 lease and exact End
  -> full unpublished physical session
  -> exact-two DraftSeal
  -> atomic selected production switch + old-edge deletion
```

Bounded implementation subrows (all part of this one activation cell; none is
an independently selectable provider or production route):

`CORE-METHOD-CONTRACT-COMPLETE-ROW-PROJECTION-R0`
  Extend the existing `CoreMethodContractBox` code generator so the generated
  Rust semantic row carries the typed `effect` (and only the source-owned
  contract fields needed by the selected co-seal) alongside `result_kind`.
  The `.hako` `CoreMethodContractBox` remains the only effect authority; no
  TextScan-only effect table, selector lookup, or Builder-fixed `EffectMask`
  is allowed. The by-`CoreMethodOp` projection must expose both result and
  effect, and missing, unknown, or drifted rows reject before provider
  admission. `pure_read` is the callable effect axis; it must not reclassify
  the Dynamic invocation envelope's `OpaqueObservable`/suspension semantics.
  This is a semantic projection BoxShape, not a new TextScan authority or
  production route.

Status (landed BoxShape, 2026-08-12): the generator validates source effects,
emits `CoreMethodEffectV1` beside `result_kind`, and keeps JSON/Rust parity.
Unknown effect values reject in the generator; this does not open a provider,
runtime, LLVM, VM, or production route.

`DYNAMIC-V2-TEXT-SCAN-CONTRACT-COSEAL-R0`
  The existing generated `CoreMethodContractBox` rows are borrowed by a
  private TextScan role view that is consumed immediately by the same-slice
  ProviderAdmission; it is not a standalone durable authority. I6/substring
  must match the generated `StringValue` row and I7/indexOf must match the
  generated `I64Value` row; the view adds only the complete two-role profile
  and shared lifecycle requirements. Missing, foreign, duplicated, or
  mismatched rows reject before provider admission. A hand-written result
  table, selector-only result classification, or independent provider catalog
  is forbidden.

Status (landed BoxShape, 2026-08-12): generated CoreMethod rows now carry
typed `result_kind/effect`; I6/I7 resolve them once by `CoreMethodOp`/arity,
cross-check spelling, and retain the same rows through Recipe verification.
A-prime owns the catalog-derived physical header and one-way MIR effect
projection; the canonical skeleton consumes it without fixed effects or raw
body inference. The selected capability rejects incomplete/mismatched I6/I7
contracts before effect. Provider/session activation remains open. Focused
tests and the manifest/physical-input guards are green; full evidence is in
`ParentHistory` and git history.

Activation order after the effect projection is fixed:

```text
TextScan role aggregate -> consuming admission -> immutable registry
-> receiver-identity RuntimeExecutablePlan -> strict LLVM leaf
-> PHYSICAL-SESSION-BLOCKS (Header/BodyPrelude/Then/Continuation/After)
-> TYPED-OPS -> CARRIER-SSA-END (I6 lease/End, I7 ImmediateI64)
-> exact-two DraftSeal -> CanonicalCallable collector -> cutover
```

Each arrow is a named child of this activation cell, not an independently
selectable authority. Any missing relation remains `RejectBeforeEffect`.

The activation handoff is one consuming physical aggregate, not a sequence of
landable provider/session fragments:

```text
admitted TextScan two-role executable cell
  (receiver identity + image/PlanStamp + strict I6/I7 lanes + one-shot lease)
        -> DynamicV2PhysicalEmissionSessionV1::begin(builder, plan, executable)
        -> one session-owned operation/control/cleanup cursor
        -> exact-two DraftSeal -> CanonicalCallable collector
```

The executable cell is move-only and carries no registry reference, selector
lookup, or semantic result/effect authority. `begin` must reject before
opening Builder state when either I6/I7 lane, receiver/image/PlanStamp, or the
V10 lease capability is missing, foreign, duplicated, stale, or mismatched.
There is no independently selectable provider-only commit, partial operation
cursor, lease-only commit, VM adapter, generic String fallback, or retry. Until
the complete aggregate is available, the current capability remains
`RejectBeforeEffect` and production callers remain zero.

```text
DYNAMIC-V2-CANONICAL-CHILD-ADMISSION-R0
  The package adapter's existing cataloged-method admission is the sole
  physical-header source. Its symbol/arity/parameter-return representation
  and verified operation-effect summary are borrowed into the selected
  Dynamic activation input and retained through the unpublished session and
  collector; the emitter must not re-seal the source key. Foreign symbol,
  arity, owner, target, effect, or header evidence rejects before Builder
  mutation. `begin` only orchestrates prepare -> validate -> open -> install;
  it does not invent header/effect facts.

DYNAMIC-V2-SESSION-EXACT-TWO-TERMINAL-I0
  The same session consumes the complete operation/control schedule, I6/I7
  producer receipts, and V10 End evidence; it claims exactly two Completion
  sites and prepares the two physical Return terminators.  Missing, extra,
  duplicate, mixed-representation, or synthetic join/PHI evidence rejects.
  DynamicProfileOwned is an explicit unit disposition until the full
  operation/control/cleanup cursor validates profile close; retaining an
  unchecked owner token or silently discarding it is forbidden.
  Physical targets remain exact: condition B0 -> Header, body operations
  before If -> BodyPrelude in B1, then Return -> ThenTerminal in B2, body
  operations after If -> Continuation in B1, and callable After -> After.
  `Prelude` is an order label only; it must never collapse B0 and B1 into one
  physical block.
  Before full operation emission, three physical preconditions must be
  closed in the same session admission: canonical loop `Enter` is distinct
  from the allocated loop `Header` PHI block; each I6/I7 `Normal|Fault`
  outcome has an explicit fault terminal and cleanup edge (I7 Fault ends the
  live V10 carrier); and formal lanes `src=0,pos=1,end=2,pred_chars=3` are
  projected to the Recipe V0/V3 physical seeds rather than inferred at the
  consumer. Missing Enter/Header, outcome-terminal, or formal-seed evidence
  rejects before Builder mutation.
  Status (landed BoxShape, 2026-08-12): the unpublished I8 canary now consumes
  a move-only session-local six-block topology (`Enter` plus five roles), with
  `Enter != Header`; full operation emission remains closed until the atomic
  AOT cell.

  Status (landed BoxShape, 2026-08-12): the same session now adopts the exact
  four reserved formal lanes from that relation, claims the `pos` initializer,
  publishes the induction seed at `Enter`, emits only `Enter -> Header`, and
  reads the provisional Header current through canonical Binding SSA. This
  remains an unpublished canary; value representation, loop backedge, Fault,
  End, and full operation emission remain closed until the atomic AOT cell.

DYNAMIC-V2-SESSION-PRIVATE-VALUE-LEDGER-R0
  The existing I8/V12 canary publishes its emitted physical value exactly once
  into a session-owned move-only ledger. The ledger stores only the logical
  result, producer, physical block, ValueId, and already-selected representation
  and exposes reads through a callback-scoped view. Duplicate producer/result,
  foreign target, missing result, and representation mismatch reject. It does
  not choose Recipe order, block layout, effect, lifecycle, provider, or result
  class, and it has no production raw-ValueId getter or split API. This is a
  BoxShape only: no provider, LLVM, VM, PHI, Return, Completion, DraftSeal,
  collector, production caller, fallback, or retry is opened.

Status (landed BoxShape, 2026-08-12): the canary owns and publishes I8/V12 in
`value_ledger.rs`; the focused positive/missing-value test and physical-input
authority guard are green. Full I0-I16 consumption remains NoSafeSlice until
I6/I7 executable admission and the V10 lease/End owner land together.

DYNAMIC-V2-CANONICAL-DRAFT-COLLECTOR-HANDOFF-I0
  ModuleLoweringPortV1 receives the already-completed canonical draft through
  one direct collector admission.  It must not open a second function session
  or re-run finalize/type/return inference. The selected package-owned
  catalog identity maps to `FunctionDraftKeyV1::CanonicalCallable` with the
  existing whole-batch collision preflight; `into_legacy_collector_parts()`
  and `FunctionDraftKeyV1::LegacySymbol` are compatibility-only and forbidden
  on this path.

DYNAMIC-V2-SELECTED-PACKAGE-ADAPTER-CUTOVER-I0
  NormalCallableSemanticPackagePortAdapterV1 routes only the selected Dynamic
  variant through demand -> full session -> DraftSeal -> collector.  Ordinary
  remains on its existing route.  In the same activation commit the selected
  raw AST/JoinIR edge is deleted; selected canonical caller=1, old edge=0,
  fallback=0, retry=0.
```

The four names are execution subrows, not new authority types. Their sole
owners are respectively the existing Dynamic physical session, the same
session/DraftSeal path, `ModuleLoweringPortV1`, and the installed package
adapter. They may be developed in a work branch, but main receives them only
as the complete activation unit.

Required activation counts:

```text
complete TextScan roles / same provider-profile                 = 2 / 1
generated CoreMethodContractBox result/effect source             = 1
generated typed effect projection                              = 1
TextScan result/effect reissuance                                = 0
ProviderAdmissionSeal / immutable admitted registry             = 1 / 1
mutable admitted insert / duplicate overwrite                   = 0 / 0
String|StringBox canonical branch                               = 1
RuntimeExecutablePlan with receiver/provider/image/PlanStamp    = 1
LLVM selected early consumer / strict leaf                      = 1 / 1
I6 receipt / lease issuer / End consumer                        = 1 / 1 / 1
I7 receipt / lease / End                                        = 1 / 0 / 0
Completion expected / claimed / physical Return                 = 2 / 2 / 2
synthetic return join / return PHI                              = 0 / 0
new selected production caller / selected old edge              = 1 / 0
runtime registry/selector/provider/image lookup                  = 0
selected legacy finalizer / name-type repair                     = 0 / 0
selected Builder-fixed effect summary / legacy collector key      = 0 / 0
Rust VM DynamicV2 production consumer                           = 0
fallback / retry / sentinel-zero repair                         = 0 / 0 / 0
```

### 4. `DYNAMIC-V2-SELECTED-LEGACY-RETIREMENT-R0` — after cutover

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

### 5. `MIRBUILDER-MODULE-DRAIN-CONVERGENCE-D0 -> I0` — after selected cutover

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

### 6. `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0` — after legacy retirement

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

## Mandatory cleanup and line-budget gates

These are BoxShape siblings, not semantic progress and not substitutes for the
current session projection or production cutover.

```text
MIRBUILDER-LINE-BUDGET-R0
  split module_draft_collector.rs (434 after test extraction)
  split completion tests into completion_consumption_tests.rs (202),
  completion_draft_seal_tests.rs (627), and completion_test_support.rs (98)
  split src/mir/resolved_value_profile/analyzer.rs (769) at its policy/
  verification seam; keep one analyzer authority and move only private
  helpers/tests. Freeze src/mir/builder.rs (788): no additions before its
  module-registry classification row below.
  treat this as a pre-cutover hard gate for the formerly 801/894-line files: no new
  production authority or physical activation code may be added to them;
  analyzer.rs is either split at the same private seam or frozen unchanged.

  Landed cleanup evidence is archived in `ParentHistory`/git. Current guards
  enforce the line budget, explicit physical targets, and Dynamic count
  ownership; the focused recipe suite has 33 passing tests. Repository-wide
  `cargo fmt --check` remains a known baseline and is not part of this row.
CURRENT-STATE-LIVE-SCHEMA-I0
  CURRENT_STATE.toml -> live pointer/blocker/next/parked + bounded landed tail
  historical key registry -> generated/archive index

MIRBUILDER-WORKSTREAM-ARCHIVE-R0
  rolling workstream current brief below 800 lines
  closed chronology -> archive/git history
  Status (landed BoxShape, 2026-08-12): closed chronology was compressed to
  the archive/git pointer; live invariants, active queue, and parked lanes
  remain in the current workstream.

MIRBUILDER-EMIT-INSTRUCTION-PHASE-SPLIT-R0
  keep one public emit_instruction writer; split private prepare/validate/
  physical-commit/post-metadata phases

MIRBUILDER-BUILDER-BUILD-SPLIT-R0
  keep the existing MirBuilder methods, visibility, callers, and emission order
  builder_build.rs becomes a thin facade over:
    literal_lowering.rs
      literal dispatch + exact-numeric constant metadata
    variable_read.rs
      variable lookup + undefined-variable diagnostics
    assignment_lowering.rs
      assignment + local/typed-array contract publication
      + previous strong-reference release
    new_expression.rs
      PreparedRawNewExpression + raw new route + legacy child descent
  move file-local tests to one sibling test module
  do not add a new lowering entry, receipt, fallback, env policy, or authority

  Landed evidence is archived in `ParentHistory`/git. The selected canary
  already uses catalog-derived `ParserScanLoopBox.skip_while/4`, a body-free
  skeleton, and pre-mutation authority validation; it remains unpublished and
  adds no provider, LLVM, VM, or production caller.

MIRBUILDER-MODULE-REGISTRY-CLASSIFY-R0
  run after selected production cutover and a caller/cfg census
  keep builder.rs as the sole MirBuilder facade and preserve module paths,
  re-exports, visibility, and cfg gates
  reorder its declarations into:
    state / session
    source admission
    semantic plans
    physical lowering
    draft collection / publication
    legacy compatibility
    tests / migration-only
  move inline binding tests to a sibling test module
  move historical phase/migration prose to archive or owning README
  remove #[allow(dead_code)] or disconnected modules only after caller-zero;
  classification alone never retires them

MIRBUILDER-COMPLETION-COMMENT-CLEANUP-R0
  update the stale completion-consumption comment that still describes a
  single explicit claim; exact-two site-keyed consumption is already landed
  and must remain the only physical claim model
```

Both rows are behavior-neutral refactor series of two to five commits. Their
Done boundary is unchanged public callers and diagnostics, focused parity and
failure tests, no new production edge, `git diff --check`, and every touched
Rust file below 760 lines. If moving a method changes metadata/publication/
release ordering or a module move changes its Rust path or cfg reachability,
the series stops and returns to its parent commit.

The current active card is intentionally compact. Its multi-thousand-line
predecessor is retained only under `design/archive/` as historical evidence.
It is not a current pointer or implementation authority.

## Common negative matrix

```text
Ordinary or foreign selected loan                         -> reject/not selected
owner/function/forest/projection/root drift              -> reject
missing/duplicate/extra Completion site                  -> reject
wrong function target or return operand                  -> reject
Loop/local If/inner Return shape drift                   -> reject
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

Gate classification (2026-08-12): the focused `completion` command has one
known parent-baseline failure in
`mir::compiler::canonical_physical_completion_p0::compiler_bridge_drains_a_plus_single_route`
(`ReturnValueTypeMissing`, `ValueId(12)`). It reproduces at parent
`b69f5e11fe` and is outside the selected Dynamic activation diff; it remains
recorded as baseline debt, not a green production claim. All selected
Dynamic/package/Recipe/emitter checks and the listed authority guards pass.

## Non-claims

```text
CanonicalTrivialBindingSsaPlanV1 Dynamic expansion
generic all-V2 Loop admission
full String surface or I6-only provider slot
Dynamic-specific registry
runtime provider/selector/image lookup
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
