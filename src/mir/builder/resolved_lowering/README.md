# Resolved lowering boundary

This directory owns the first production consumer of a sealed semantic owner.

Allowed inputs are only `CanonicalFirstFamilyPlanV1` values produced by the
whole-unit compiler preflight. Recursive lowering accepts sealed located-node
carriers and resolves lexical identity through exact source sites.

Invariants:

- `BindingRefV1 -> ValueId` is the canonical value environment.
- names are diagnostic cross-checks, never lookup keys.
- legacy `allocate_binding_id()` is structurally vetoed while an owner is installed.
- declarations, variable uses, assignment targets, and exits must all finish
  source coverage before the function draft can be published.
- canonical lowering seeds separate RegionId and ScopeId stacks from the sealed
  function/function-body roots; BlockExpr consumes one exact pair and retires
  only pair-owned BindingRefs at scope leave.
- I1b consumes one pre-Builder verified statement-If flow in source preorder.
  Both branches start from the same post-condition BindingRef baseline; the
  sealed join-source matrix selects final PHI inputs, and all PHIs define
  before one effect-authorized batch publication.
- RegionId and ScopeId stacks remain separate. Statement If consumes exact
  control/branch identities and coverage only; no durable RegionId-to-block
  map is published before SA4.
- legacy statement/expression dispatch, Planner/CorePlan, Lambda, production
  Loop activation, Main, REPL, and ProgramV0 are outside this boundary.

## Canonical V2 function finish

The three canonical V2 profile lowerers (`trivial_ssa`, `direct_accum`, and
`nested_predicate`) share one consuming finish terminal:

```text
CanonicalSsaFunctionSessionV2::finish_for_draft_seal
  -> ReadyFunctionDraftSealV1
```

Each profile closes its private effect/After/final-carrier ledger into one
move-only `ReadyCanonicalProfileCloseV1`. The common terminal consumes that
receipt and closes CFG, semantic/If control, identity/Binding SSA, PHI, the
resolved binding ledger, and Completion exactly once. It is the sole V2 issuer
of `ReadyFunctionDraftSealV1`.

The terminal accepts no raw body/site/end/target/current-block facts for
re-inference. Those identities are sealed when the exact resolved function
session opens; the profile receipt carries the already-claimed terminal
witness. A failed or duplicate close rejects before publication, and any late
failure discards the whole unpublished function. `PhiTxn` rollback is only
best-effort local cleanup; the outer function session owns atomic discard.

The existing non-V2 direct constructor caller is an explicit compatibility
allowlist entry. It may not gain callers and is retired by a later bounded row;
it is not part of the V2 finish migration. Focused guards must keep V2 direct
`ReadyFunctionDraftSealV1::new` callers at zero and keep all source files below
the repository's 800-line boundary. Every implementation slice updates this
README, the owning reference, and current-entry mirrors in the same commit.

Implementation receipt (`6bf3dd6b35`, 2026-08-07): the three V2 lowerers now
use the consuming terminal, including the previously missing DirectAccum CFG
finish. The one non-V2 constructor remains an explicit, non-growing
compatibility debt. The focused session, resolved-lowering, and draft tests,
the canonical finish guard, and the current-state pointer guard are green.
The caller-zero Loop physicalization/DraftSeal canary is closed; production
selection remains closed at the next docs-only
`CALLABLE-LOOP-PRODUCTION-EDGE-D0` stop.

The caller-zero topology slice is landed as historical test-only evidence
behind the `loop_recipe_physicalizer/` directory facade. It consumed one
move-only common boundary and one session-local `ReadyLoopEntryV1`, borrowed
the existing canonical CFG service, and allocated the old recursive
header/body/step/After blocks. That P0 topology shape is not the current R3
Callable physical boundary.

This is not a production physicalizer or selector. It emits no operation MIR,
Return, DraftSeal, publication, retry, fallback, or legacy deletion. The
DirectAccum binding port remains profile-specific and must not be reused as
the common port; no second CFG/SSA/PHI owner is allowed. The historical
passive `LOOP-RECIPE-OPERATION-EFFECT-S0` product issues an item-keyed
exact source/effect ledger before operation emission is opened. The product is
test-only, moves the verified Core once, and emits no operation MIR, Return,
DraftSeal, selector, retry, fallback, or production route. The callable
adapter is now closed as a separate source-evidence receipt. The Generic G0
anchor ledger is also closed: its 15 item keys are issued before source facts
are dropped, with item 3 matching the child-entry carrier relation.
Cross-profile parity is also closed as a diagnostic-only receipt; it compares
neither profile counts nor source order. Reviewed Decision B separates full
demand/preflight from leaf emission. The Builder-free complete operation
demand, topology module split, and the private logical-to-physical block
receipt are now closed. The private
`LOOP-RECIPE-OPERATION-EMITTER-CONST-S0` and the bounded
`LOOP-RECIPE-OPERATION-EMITTER-READ-I0` leaf canaries are closed. ReadBinding
projects the complete prepared program, claims the exact source through the
canonical BindingSSA/PHI owner, validates explicit entry requirements, and
returns distinct logical/physical receipts. Pre-claim rejects are typed;
claim/read/type/receipt failures terminate the unpublished function session as
one discard transaction. No single-operation demand extraction was added.
Full operation physicalization, production activation, carrier seeds,
selector, retry/fallback retirement, and legacy deletion remain closed until a
named production caller switch is authorized. Continuation, Tail, and the
bounded caller-zero DraftSeal seam are landed but remain test-only.

The current bounded preparation slice adds a private Prepared-product
move-only handoff, complete WriteBinding projection, typed pure leaf bridges
for Const/Binary/Compare, exact logical-to-physical operation target receipts
(all validated before the first leaf effect),
and phase-separated physical dispatch errors. The focused fixture proves a
Const -> Binary -> Compare chain without introducing a second CFG/SSA/PHI
owner. The callable Prelude adapter is now landed as caller-zero evidence:
exact resolver-backed argument bindings are read through canonical identity,
the external Prelude result is emitted through the shared direct-call emitter,
and the Loop initializer is materialized from its exact source site into a
separate `ReadyLoopEntryV1` binding/value. The Prelude result local is not the
Loop input and is never used as an implicit entry value. The
`CALLABLE-LOOP-AFTER-CLOSURE-P0` and `CALLABLE-LOOP-TAIL-COMPLETION-P0` rows
are now bounded caller-zero seams. The
latter reads the exact Tail binding through canonical identity, validates the
declared trivial ABI, claims `tail.value_site()` before the non-claiming
`read_entry_receipt`, and claims Completion/return coverage once. The sealed
After receipt moves a non-Clone profile-close receipt proving the exact
callable `7 = Pure4 + Read2 + Write1` coverage, Bool condition, owner,
terminal block, and After predecessor. The later finish step must consume it
through a non-no-op `finish_profile_close` closure. The bounded DraftSeal
canary consumes that evidence through the typed finish terminal and existing
DraftSeal prepare/commit, producing one `CompletedFunctionDraftV1` without
collector/module publication. The common physicalizer deliberately stops
before Tail, ABI, Completion, Return, and DraftSeal; the caller-zero profile
adapter owns that handoff. This separation is now exercised end-to-end by the
closeout receipt below. Production selection and legacy retirement remain
later bounded work.

The bounded `CALLABLE-LOOP-AFTER-CLOSURE-P0` slice is now landed as a
caller-zero continuation proof. It uses the real Prelude receipt, emits the
complete seven-operation Callable schedule (`Pure=4`, `Read=2`, `Write=1`),
then emits the fixed preheader/header/body/step/After edges and seals CFG and
BindingSSA in backedge-safe order. An unsealed `Unknown` PHI is typed only by
the verified Recipe value class; concrete or missing type facts reject as
`ResultTypeMismatch`. The success canary commits the draft and restores the
caller; rejection paths continue to discard the unpublished session.
Production selection, retry, and legacy deletion remain closed.

### Callable full physical canary closeout (2026-08-08)

`CALLABLE-LOOP-PHYSICAL-CANARY-P0` is now closed as caller-zero evidence. The
test-only bridge starts from the exact resolved-module input and its existing
resolver ledger, so S2 full-demand preparation, Prelude, topology, all five
operation families, sealed After, Tail/Completion, the sole
`finish_for_draft_seal` terminal, and DraftSeal prepare/commit all use one
owner-branded request. It does not re-resolve, clone source, or create a
second CFG/SSA/PHI owner.

The late-failure canary seeds a Recipe-derived Compare result key, observes a
typed duplicate after earlier emission, discards the complete unpublished
function, and reruns the same semantic fixture in a fresh session. The
focused P0 tests are green and the canary file remains below 800 lines. The
G0 parity design is accepted: a compiler-side composite pairs exact resolver
input with neutral S4, while `L0.After/b1` is split into neutral continuation
and a distinct tail capability. The Builder-free
`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` ingress and common
`LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` layout are now closed. R1 derives
recursive Recipe preorder and parent-resume segments without Builder effect;
R2 is closed as an adapter receipt only. R3-I0 now allocates one block per R1
segment plus one root After (no Step), retains the complete segment program,
preflights and emits every entry/transfer once through canonical
CFG/identity/PhiTxn, and returns the neutral After continuation used by the
existing Tail/Completion/DraftSeal path. Physical G0, selector, collector,
retry/fallback, and legacy retirement remain closed.

## Generic G0 exact-ingress I0 receipt (2026-08-08)

`src/mir/compiler/generic_g0_physical_prepare.rs` is the bounded `cfg(test)`
compiler-side ingress for `LOOP-CALLER-ZERO-PARITY-G0-I0-R0`. It pairs the
exact resolver input with neutral S4, validates source/owner/frame/forest/
entry/tail provenance, preserves G0's post-loop read and I64 ABI in
`VerifiedGenericG0TailCapabilityV1`, and proves all fifteen Recipe members
through common `prepare_all` with no Builder effect. It does not own AST,
resolver, CFG/SSA/PHI, physical IDs, Completion, DraftSeal, selector,
fallback/retry, or publication. Physical G0 and production cutover remain
closed.

## Full operation demand P0

The Builder-free `VerifiedLoopOperationPhysicalDemandV1` now consumes the
resolver semantic context, complete operation/effect ledger, and one logical
After continuation as a move-only product. `prepare_all` derives Recipe
Loop/Block/Item order and proves complete seven-row Callable and fifteen-row
Generic G0 coverage before any Builder effect. The resolved-lowering layer is
not imported by this product; physical block allocation, leaf operation
emission, session opening, and DraftSeal remain later boundaries.

## Caller-zero Loop physical prepare

`src/mir/compiler/loop_physical_prepare.rs` is a test-only pre-effect contract
boundary for `LOOP-PHYSICAL-PREPARE-P0`. It brands the exact resolved callable
input, derives a prelude target/result capability from the existing callable
index/header, and seals one Tail/ABI/Completion compatibility relation before
any Builder session opens. The moved topology-only compatibility
`VerifiedLoopPhysicalDemandV1` owns the co-sealed logical product for the
closed P0 topology probe; the retained resolved input remains a borrowed view.
It is not the current full-operation demand and cannot feed operation emission.

The current `helper.to_i64(n)` MethodCall fixture intentionally has no
resolver-issued direct callable target and therefore rejects with typed
`NoSafeSlice::MissingPreludeTarget`. It remains a typed `Method` negative. The
bounded `CALLABLE-STATIC-PREFIX-S0` fixture is separate: top-level
`int_to_str(n: i64)` calls catalog-backed `to_i64(n: i64)` as a real
`FunctionCall`, and the observer records only the resolver-issued target and
explicit `FreeStatic` shape. No target injection, name lookup, AST rematch,
physical ID, Builder effect, selector, retry, fallback, or production caller
is opened by this row.

The neutral shape vocabulary remains in
`callable_single_loop_source_shapes.rs`; embedded syntax/source-map/static-
fixture tests remain sibling test-only modules and all touched files stay
below the 800-line limit. `CALLABLE-STATIC-PREFIX-MAP-S1` is now closed as a
source-only relation: same-brand different-owner resolver targets are kept,
while foreign compilation brands reject as `ForeignOwner`. The next bounded
cell is `CALLABLE-STATIC-PREFIX-P0` for declaration-derived ABI/Prepared
evidence; no physicalizer or production route is opened.

`CALLABLE-STATIC-PREFIX-P0` is now closed: the static fixture yields one
positive Prepared relation whose caller ABI comes from the completion/header
declarations and whose callee ABI comes from the resolver target header. ABI
is no longer accepted as an external argument at this boundary. The next
step is a design-only audit of the common physicalizer/session finish seam;
physical Builder effects remain closed.

## Disconnected canonical CFG prerequisite

`canonical_cfg/` owns the SSA-C1 edge/seal substrate. It emits a terminator and
its cached predecessor witness as one fallible operation, derives predecessor
truth directly from terminators, and rejects late edges or cache drift without
calling CFG repair. During SSA-C1 it has zero production If, Loop, and Binding
SSA callers; the existing A+ If path remains unchanged.

## Callable segment block cutover R2

`LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` is closed as a bounded adapter cutover.
The private `LoopPhysicalSegmentBlockReceiptV1` adapts the closed R1 layout to
the existing canonical topology and brands each exact segment with its
physical block. It is not the R1 segment allocator. The selected Callable
dispatcher builds its complete item-to-segment index from the layout and
issues targets by segment; the selected canary no longer uses
logical-block-only execution lookup.

The receipt rejects missing, foreign, duplicate, and aliased placements. The
Callable canary preserves seven-row parity (`Pure=4`, `Read=2`, `Write=1`),
and its late failure still discards the unpublished function as one transaction
before a fresh-session rerun. No new CFG/SSA/PHI owner, G0 physical route,
recursive After writer, selector, fallback/retry, publication, or legacy path
was introduced. The old Step block remains outside the R1 transfer graph, so
this adapter is historical and no longer feeds the selected Callable canary.

## Callable recursive After R3-I0

`LOOP-COMMON-RECURSIVE-AFTER-R3-I0` is closed for the selected caller-zero
Callable canary. `segment_allocator` is the only R3 allocation entry: it
allocates exactly one physical block per R1 segment and one root After block.
`PreparedLoopPhysicalLayoutV1` carries an explicit sealed `entry_segment`, and
`CompletedLoopSegmentProgramV1` retains layout, entry, segment receipt,
completed operation receipts, and the value ledger. The recursive After module
preflights the entry edge and each R1 Jump/Predicate/OpenNestedLoop transfer,
emits them exactly once through canonical CFG/identity/PhiTxn, seals every
segment and root After block, and returns one neutral
`ReadyLoopAfterContinuationV1`.

The Callable wrapper alone checks `7 = Pure4 + Read2 + Write1`; Tail,
Completion, and DraftSeal remain their existing owners. The focused canary
asserts exact segment coverage, a distinct root After, late-failure whole
session discard, and fresh-session reuse. No G0 physical allocation,
production selection, retry/fallback retirement, publication change, or broad
legacy deletion is claimed.

The source-backed Dynamic carrier P2 close does not yet produce this After
receipt. P2 seals only Header/Backedge and is deliberately discarded in its
tests. Reuse of recursive After is permitted only after a complete Dynamic
Recipe/full-program physical input covers the unchanged callable body. The
single-Tail completion canary is not a substitute for the method's inner and
final returns; those must merge into one function exit before the sole
Completion claim and DraftSeal Return.

## Common Predicate/carrier I0 (2026-08-08)

`LOOP-COMMON-PREDICATE-CARRIER-I0-R0` is closed. The neutral After receipt
contains only common owner/root-After/predecessor facts; each Predicate edge
uses its own verified Bool operation receipt and source segment. Callable's
coverage and condition-key checks stay in the profile wrapper.

`DerivedCarrierEntry` is represented by the full-program
`PreparedLoopDerivedCarrierSeedRowV1` and the private `CarrierSeed` emitter.
It delegates to canonical identity `read_entry_receipt` and never fabricates
an expression site, re-resolves a name, or creates a G0-specific SSA owner.
The focused Callable suite is 25/25 and the source files touched by this row
remain below 800 lines. The next row is the test-only G0 I1 canary; production
selection, retry/fallback retirement, and legacy deletion remain closed.

## Generic G0 I1 caller-zero canary (2026-08-08)

`LOOP-CALLER-ZERO-PARITY-G0-I1-R0` is closed as a `cfg(test)` profile
harness. The exact compiler-side G0 ingress is split once into the complete
common operation program and the profile-specific Tail. The harness honors
the resolver's instance receiver contract, publishes receiver/parameters
through canonical identity, allocates the five R1 segments plus root After,
and dispatches all fifteen rows exactly once. Item 3 uses the common
`CarrierSeed` emitter and canonical `read_entry_receipt`; item 4 is structural
nested control and emits no operation.

Each Predicate transfer consumes its own completed Bool receipt, so root and
child conditions cannot silently share one value. The G0 post-loop `b1` Tail
read is canonical, exact I64 Completion is claimed once, and the existing
`finish_for_draft_seal`/DraftSeal path is reached. A late duplicate after
earlier emission discards the whole unpublished session, and a fresh session
replays the same semantic shape. The carrier leaf uses the shared provisional
type publication contract for unsealed PHI values; it owns no type, SSA, CFG,
Tail, selector, retry, fallback, collector, or publication authority.

This is caller-zero evidence only. M8/M9, production selection, M10b/M11/M12,
and broad legacy retirement remain closed. The next row is the design-only
top-down audit `LOOP-CALLER-ZERO-PARITY-G0-POST-I1-AUDIT-D0`.
