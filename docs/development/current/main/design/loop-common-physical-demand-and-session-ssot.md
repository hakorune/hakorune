---
Status: SSOT
Date: 2026-08-16
Decision: accepted after external and independent worker review — `LOOP-COMMON-PHYSICAL-DEMAND-AND-SESSION0-D0-r2`
Activation: `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`, callable static-prefix
P0, bounded `LOOP-PHYSICAL-PREPARE-P0`, common-boundary design stop,
caller-zero `LOOP-PRELUDE-ARGUMENT-RECEIPT-P0`, passive operation/effect S0,
Callable/G0 adapters, and cross-profile parity are closed. Decision B now
separates full-demand preflight from leaf emission; the Builder-free
`LOOP-RECIPE-OPERATION-PHYSICAL-DEMAND-P0` and the behavior-neutral
physicalizer module split, physical block receipt, private ConstI64
leaf-emitter canary, bounded ReadBinding I0, callable full physical P0, and
G0 exact-ingress I0 are closed. Top-down review revised the next boundary:
the private Builder-free segment/resume layout and bounded G0 fresh-session
canary are closed. A later audit found that the landed layout still derives
logical transfers from Recipe instead of consuming JoinSig authority; the
post-M9 pre-cutover R0 rows below own that correction. Operation production
activation remains 0. The bounded After-closure canary is green: the real
Prelude receipt feeds the complete seven-operation Callable dispatch, fixed
CFG edges, and canonical CFG/identity sealing. The Tail handoff now reads the
exact binding through canonical identity, validates the existing trivial ABI,
and claims Completion/return coverage once. The sealed After receipt also
moves a non-Clone callable profile-close receipt proving exact
`7 = Pure4 + Read2 + Write1` coverage, the Bool condition, owner, terminal
block, and After predecessor. Finish must consume that receipt through a
non-no-op `finish_profile_close` closure. DraftSeal, production selection,
retry/fallback, and legacy retirement remain closed. The bounded
`CALLABLE-LOOP-DRAFT-SEAL-P0` canary now consumes the profile-close receipt
through the existing typed finish terminal, then uses DraftSeal
prepare/commit to produce one unpublished `CompletedFunctionDraftV1`; no
collector or module publication is performed. The production-edge census and
Admission D0 are closed as `NoSafeSlice`. The source/facts bridge D0 is
accepted without a new semantic Bridge owner: the existing resolver ledger
plus neutral SyntaxFacts/SourceMap are the target production boundary. The
source/facts issuer S0 and bounded logical issuer D0/S0 are now closed with
bounded negatives, exact parity, and caller-zero/current receipt audit. The
profile Recipe shape is production-owned while the old shape helper remains a
test-only parity wrapper. `CALLABLE-LOOP-PRODUCTION-PREPARED-INGRESS-D0` is
accepted and its S1/S2 caller-zero products are closed.
`LOOP-CALLER-ZERO-PARITY-G0-D0` is also accepted. Its exact resolver-issued
G0 source/input/entry capability is carried by a thin compiler-side composite
ingress; neutral S4 remains the sole Recipe/effect/After owner. I0 is closed
as Builder-free exact ingress plus fifteen-row `prepare_all`. R1 is closed as
a Builder-free derived layout, R2 as a Callable adapter, and R3-I0 as the
selected Callable exact-segment/neutral-After canary. Per-transfer Predicate
value receipts, the profile-neutral `DerivedCarrierEntry` operation, and the
bounded G0 I1 canary are also closed. A later top-down audit found that current
caller-zero products can still be re-paired and current Layout code still
derives logical transfers without JoinSig authority. The accepted target is
unchanged, but semantic-program and transfer-authority R0 rows must close
after M8/M9 and before production selection. No named production caller
switch is open.
The 2026-08-15 S6C audit adds no second physicalizer: it names one missing
common-V2 pre-session contract that must close exact callable ABI and the
complete V2 envelope before any TextEq leaf or canonical session is admitted.
Scope: common Loop physical demand, fresh unpublished function session, failure discard, completion/DraftSeal handoff
Related:
  - docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/reference/mir/loop-recipe-contract.md
  - docs/reference/mir/generic-loop-stage-matrix.md
  - src/mir/builder/resolved_lowering/README.md
  - docs/development/current/main/investigations/loop-physical-prepare-design-correction-r0-task-2026-08-07.md
---

# Loop Common Physical Demand and Session SSOT

## Current Capsule

- **Current decision:** every admitted Loop profile reaches one complete
  semantic program, JoinSig-bound layout, and canonical SSA session; V1 and V2
  are exact projections of that one responsibility graph.
- **Current implementation status:** S6C source/site, ExactText formal,
  result/header, installed child, package physical-signature map, and
  caller-zero residence/backend transport substrate are closed. The common
  V2 operation/control/coverage issuers and installed Port HRTB are landed
  as caller-zero source products. Existing resolver Loop membership can issue
  the outer-If residual, and the installed S6C child can lend the actual
  Completion without cloning. The resolver-owned BlockExpr expectation is
  now batch-owned and reaches the selected/package HRTB as a borrow. The
  callback-scoped common admission is landed; the detached physical skeleton,
  slot-only ExactText adoption canary, and consuming physical-entry/session
  seam are also landed. The session-stamp retention I0 now moves the existing
  mechanical cohort witness exactly once into the canonical session and lends
  only a scoped borrow. The V2-native physical-ID-free layout/placement
  BoxShape and its caller-zero transport I0 are now landed. The Length-result
  canary I0 is also landed; the canonical physical Length-result issuer is the
  next design stop before the parent Bool result BoxShape. A-prime lifecycle
  activation remains parked until its boundary owns
  `PreparedFunctionExitSetV1`.
- **Next ordered task:**
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-PHYSICAL-RESULT-D0` is the
  active design stop. The Length-result canary consumed the same
  relation/inventory/stamp once without issuing ValueId, type, CallSlot, CFG,
  or edge effects; the next BoxShape must name the sole same-session physical
  Length-result owner.
- **Production stop line:** no leaf emission or session admission may infer
  ABI, control, transfer, or source identity from Recipe/MIR, coerce V2 to V1,
  or select a second physicalizer.
- **Retirement finish line:** all admitted profiles use one common physical
  owner and old topology, route-local schedulers, direct transfer inference,
  retry, and fallback have zero callers.

### Canonical session admission D0 (accepted three-step boundary)

```text
Decision:
  Do not open CanonicalSsaFunctionSessionV2 yet. First issue one typed
  resolver-owned block-expression expectation; then a separate caller-zero
  admission may co-seal it with the resolved input, common V2 envelope, one
  borrowed Completion, and resolver-owned outer-If residual.

Source authority + canonical issuer:
  One resolver shadow traversal already issues both
  VerifiedResolvedFunctionV1 and VerifiedResolvedBodyShapeInventoryV1.
  ResolvedBlockExpressionExpectationIssuerV1 is the sole new issuer: it
  matches typed BlockExpr source rows to exact BlockExpr scope/region pairs
  and emits one non-Clone expectation. Later, only the exact resolver
  singleton Loop site may feed empty_for_owned_loop_profile; the installed
  S6C child lends its actual Completion through the existing nested HRTB.

Non-authority:
  FirstFamily verify_body's raw usize, Other("BlockExpr") string matching,
  resolved scope count alone, S6C's fixed zero/13/15 counts, Recipe/JoinSig
  If/Exit rows, caller-supplied Loop paths, Completion parity summaries,
  Dynamic-only constructors, AST/MIR/JSON rescans, legacy finalizers, and
  DraftSeal are not admission issuers.

Fail-fast boundary:
  Reject on function/body-shape owner, function origin, or body-root drift;
  missing/duplicate/extra BlockExpr pair; count overflow; no/multiple Loop;
  Loop-external If; foreign envelope/Completion owner or target; HRTB escape;
  Completion clone; or any need to retrofit the legacy finalizer or rescan
  Returns. Every rejection precedes session effects.

Smallest next slice:
  The source-product and transport rows are landed, and
  LOOP-COMMON-V2-CANONICAL-SESSION-ADMISSION-I0 now issues one callback-scoped
  non-Clone fan-in with exact owner/origin/root/Loop/Completion checks. The
  caller-zero session-open canary consumes that admission once; the next
  bounded design row is LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-D0.

Non-claims:
  No CanonicalSsaFunctionSessionV2 construction, CFG/SSA/PHI mutation,
  Completion consumption, DraftSeal, lifecycle, Text route, production
  caller, fallback, retry, or legacy-finalizer retirement.
```

`RESOLVED-BLOCK-EXPR-EXPECTATION-I0` execution brief:

```text
Change:
  add BodyExpressionShapeV1::BlockExpr { site }; issue one batch-owned
  VerifiedResolvedBlockExpressionExpectationV1 from the same resolved
  function/body-shape row; store it once in that callable batch row
Contract:
  typed source rows and exact BlockExpr scope/region pairs have equal owner,
  origin, root, sites, and count; constructor private; product non-Clone
Done:
  zero/one/nested positives plus missing/extra/duplicate/foreign/root/count
  negatives; existing body-shape consumers stay exhaustive; source files
  stay below the 760-line split trigger and 800-line hard stop
Stop:
  string matching, AST/MIR rescan, raw usize transport, fixed zero, session
  construction, Completion consumption, fallback, or retry is required
```

### RESOLVED-BLOCK-EXPR-EXPECTATION-I0 implementation receipt (2026-08-17)

The shadow resolver now emits a typed `BlockExpr { site }` body-shape row.
`ResolvedBlockExpressionExpectationIssuerV1` co-seals the same resolved
function/body-shape product, checks exact source-site, scope, and region
coverage in both directions, and the callable semantic batch row owns the
non-Clone receipt once. Focused resolver tests cover zero, one, nested, and
foreign-owner cases; selected/package transport, session, CFG/SSA/PHI,
Completion consumption, physical lowering, fallback, retry, and production
callers remain closed.

### CALLABLE-BLOCK-EXPR-EXPECTATION-TRANSPORT-I0 implementation receipt (2026-08-17)

`SelectedCallableLoweringInputRefV1` now lends the same batch-owned
`VerifiedResolvedBlockExpressionExpectationV1` inside the existing package
HRTB. The accessor is borrow-only and non-reconstructible: no clone, reissue,
raw `usize`, AST/MIR rescan, session effect, Completion consumption, or legacy
finalizer connection is added. A selected static callable containing a
BlockExpr passes the focused transport handoff test.

### LOOP-COMMON-V2-CANONICAL-SESSION-ADMISSION-I0 implementation receipt (2026-08-17)

`common_v2_session_admission.rs` now issues one callback-scoped,
non-`Clone` fan-in from the selected resolved input, the resolver singleton
Loop site, the existing Loop-owned outer-If residual, the batch-owned typed
BlockExpr expectation, the common V2 envelope, and the actual borrowed
Completion. Owner, function-origin/body-root, Loop cardinality, Completion
owner/target, and envelope ownership drift reject before any session effect.
The focused S6C admission test proves the 15-placement envelope, exact
Completion borrow, outer-If residual, and duplicate child-consumption fence.
No `CanonicalSsaFunctionSessionV2`, CFG/SSA/PHI, Completion consumption,
DraftSeal, lifecycle, Text route, physical lowering, fallback, retry, or
production caller is opened.

### LOOP-COMMON-V2-PHYSICAL-SESSION-I0 implementation receipt (2026-08-17)

The first caller-zero physical-session seam is now landed as a deliberately
thin opener. `LoopV2CanonicalSessionAdmissionRefV1` is consumed exactly once
into one callback-scoped parts aggregate; the typed BlockExpr expectation is
projected only inside `CanonicalSsaFunctionSessionV2::new_common_v2`, and the
installed semantic Completion remains borrowed. That borrow issues one owned
`ResolvedFunctionCompletionConsumptionV1` for the session, so semantic
Completion is not cloned or moved out of its installed cohort. The same
callback retains the common V2 envelope beside the session and cannot reacquire
another Port loan.

This canary opens the sole canonical CFG/Binding-SSA/PHI/Completion owner, but
does not mutate a Builder or emit a block, operation, control transfer, PHI,
Completion claim, Return, DraftSeal, lifecycle, Text operation, route, or
production caller. It does not reuse `new_selected_dynamic`, pass a raw
BlockExpr count, or expose a second session. This receipt is landed; the
current frontier is the physical condition-result receipt design stop, not a
session reopen or a second physical-entry authority.

### LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-D0

```text
Decision:
  The physical-header BoxShape and its caller-zero I0 are landed. The
  physical-function-entry input BoxShape is now accepted: one same-cohort
  transport-only aggregate may expose source ParamDecl evidence and complete
  physical lane descriptors without making the existing one-value
  formal-adoption API guess how an ExactText pair is represented. The active
  implementation is limited to that aggregate; do not open Builder effects,
  skeleton allocation, or lane adoption in this row.

  The design stop is intentionally split into:
    LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-D0
      accepted physical parameter declaration/lane projection BoxShape
    LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-I0
      active transport-only descriptor aggregate
    LOOP-COMMON-V2-PHYSICAL-FUNCTION-SKELETON-I0
      future fresh unpublished skeleton reservation after the input I0
  Only the middle row is executable in the current fast lane.

Source authority + canonical issuer:
  Existing package issuers remain the authorities: the catalog declaration
  owns symbol/ParamDecls/result/attrs/uses, the physical-signature cohort owns
  lane order/role/BindingRef, the physical header owns result/Completion, and
  `VerifiedS6CPhysicalFunctionEffectsV1` owns only the source-backed physical
  effect projection. A future compiler-side
  `PreparedCanonicalFunctionEntryInputV1` issuer may co-seal these borrowed
  siblings from one installed S6C HRTB loan, but it must issue no new semantic
  fact and must not take `CanonicalFunctionLoweringSessionV1` as an authority.
  Its only output is a non-Clone, pre-effect relation consumed by the existing
  skeleton owner.

  The missing design choice is the physical ParamDecl projection: receiver
  prefix, ordinary scalar lane, and each adjacent ExactText
  `[slot,generation]` lane need a deterministic declared name/type policy.
  Source parameter names/types may be read only through the same storage
  header; `/N`, `FunctionSignature` length, raw `ValueId`, or lane index alone
  cannot supply the missing fields. The wire contract says `u64`, while the
  current MIR/LLVM callable carrier is `i64` and
  `source_type_name_to_mir("u64")` would become an unrelated `Box("u64")`.
  The carrier decision is therefore fixed as a checked package-owned
  `U64BitsOnI64` physical-lane metadata: existing MIR/LLVM `i64` is only the
  bit-preserving mechanical carrier, while the lane role remains the
  authority for unsigned-wire meaning. This does not change semantic
  `MirType`, add a new unsigned source type, or permit generation arithmetic.
  The remaining input census must prove that this carrier row, every source
  `ParamDecl`, and the lane-role rows are exposed by one same-loan issuer;
  a string spelling of `u64` is never a valid substitute. `MirParamDecl` is
  existing source-annotation metadata, not the physical lane carrier: the
  future compiler input must use a separate non-semantic physical-parameter
  descriptor rather than overload `MirParamDecl` for expanded lanes.

  The source census fixes the seam: the installed
  `VerifiedSameModuleCallableDeclarationCatalogV1::declaration` row is the
  source owner for symbol, ParamDecls, result annotation, `uses`, and attrs;
  `issue_callable_physical_signature_v1` is the lane owner; and the retained
  S6C ingress is the owner for Facts/Recipe/Join/Completion relations. The
  new `VerifiedS6CPhysicalFunctionEffectsV1` is issued only after retained
  S6C Facts prove the exact two external calls and their CoreMethod rows prove
  `PureRead`; the local index write remains source-local and does not become a
  heap-write effect. The Dynamic-only
  `CatalogedBoxMethodPhysicalHeaderProjectionV1`, legacy
  `NormalCatalogedBoxMethodDraftAdmissionV1::physical_arity`, attrs/uses
  inference, and a literal `EffectMask::READ` are not substitutes.

Non-authority:
  Common V2 operation/control/coverage rows, Recipe/JoinSig ordinals,
  logical `/N`, FunctionSignature length, raw `MirParamDecl` or source-name
  fallback, existing one-value skeleton/adoption APIs, current_block, raw
  ValueId order, ReadyLoopEntry, LoopPhysicalBlockReceipt, Dynamic-only
  session, or any legacy finalizer may issue the entry input, skeleton, or
  adoption policy.

Fail-fast boundary:
  Before any mutation, one pre-effect input must verify same owner/origin/
  brand, selected identity, complete storage header, result/Completion,
  physical effects, receiver prefix, dense non-duplicate logical ordinals,
  adjacent ExactText `[slot,generation]` lanes, and a complete physical
  physical-parameter descriptor row for every lane. `source_logical_arity` counts explicit
  formals only; `receiver_lane_count` is a separate prefix axis and is never
  inferred from `/N` or a parameter-vector length. ExactText remains one
  BindingRef with two physical lanes; slot publication versus a private
  generation sidecar must be fixed before skeleton I0. Missing lane role/type,
  source-name drift, foreign loan, or incomplete header/effects means
  `NoSafeSlice` before Builder effect. A lane without the sealed
  `U64BitsOnI64` carrier metadata is also `NoSafeSlice`; `MirType::Integer`, a
  source `StringBox` type, or a guessed `Box("u64")` mapping cannot silently
  stand in for it. The generation lane is not an ordinary arithmetic value
  and must not be reconstructed from the `i64` carrier, a raw slot, or a
  `MirType`. Any later mutation failure must discard the unpublished
  transaction once, with no retry or fallback.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-I0` is the active
  transport-only implementation. The physical carrier choice is closed as
  `U64BitsOnI64`; the compiler-side issuer borrows the complete same-cohort
  storage header, physical signature, effects, result, and lane rows while
  projecting every source-backed `ParamDecl` into a separate non-semantic
  physical-parameter descriptor. It constructs no Builder or `ValueId`. After
  this focused gate, the next design stop is the fresh unpublished skeleton
  reservation. Any incomplete relation still rejects before Builder effect;
  it is not repaired by a default or fallback.

  The preceding header/effects I0 already proved the package-side co-seal.
  This D0 is accepted because the compiler-side aggregate can borrow that
  complete cohort and expose every physical ParamDecl row without reissuing
  source meaning. The active I0 still rejects missing/foreign header or
  effects projection, lane gap/swap, receiver drift, parameter/result drift,
  or duplicate ExactText BindingRef adoption before Builder effect; it never
  repairs them with a default or fallback.

Carrier decision receipt (2026-08-17):
  The accepted wire remains two scalar `u64` lanes for one logical ExactText
  formal. The accepted MIR/LLVM mechanical carrier is checked
  `U64BitsOnI64`; it preserves bits but does not become semantic
  `MirType::Integer`, a source `u64` type, or a new unsigned MIR type. The
  package-owned lane role and physical-signature row remain the sole meaning
  authority. The compiler-side same-loan ParamDecl/lane aggregate is now the
  active transport-only I0; no skeleton, session, ValueId adoption, or direct
  Text lowering is authorized by this receipt.

Physical descriptor policy (design-only):
  The next compiler-side view is named
  `PhysicalCallableParameterDescriptorV1` for design purposes. It is one
  row per physical lane and carries only mechanical/source relation:
  `physical_index`, `lane_role`, optional `logical_ordinal`, source
  `BindingRef`, deterministic diagnostic name, source annotation text when
  present, and a physical carrier tag. It has no `ValueId`, `MirType`, runtime
  token, pointer, or semantic ownership effect. The carrier tags are
  `ExistingCallableI64` for the receiver and ordinary scalar lanes, and
  `U64BitsOnI64` for both adjacent ExactText lanes.

  Naming is deterministic and never an authority: an instance receiver is
  `me`; an ordinary lane uses its catalog-backed source `ParamDecl.name`; an
  ExactText formal expands to `<source-name>.slot` followed by
  `<source-name>.generation`. Missing or duplicate source names reject rather
  than falling back to an ordinal, `/N`, or a generated placeholder. The
  source annotation remains attached as evidence only; physical carrier is
  selected by the package-owned lane role. Receiver is a prefix row and is
  not an explicit logical ordinal. This descriptor is projected by the future
  same-loan compiler aggregate, not stored as a second package semantic
  authority.

  Required negatives for the census are: foreign owner/brand, receiver row
  in a static method, missing receiver row in an instance method, non-prefix
  receiver, lane gap/overlap/swap, non-adjacent ExactText pair, duplicate
  logical ordinal or BindingRef, missing source name, guessed `u64` type,
  guessed `MirType::Integer`/`Box("u64")` carrier, source ParamDecl count
  used as physical lane count, and any `ValueId` or Builder access before the
  aggregate is consumed.

D0 acceptance gate (accepted BoxShape, 2026-08-17):
  Accept the input BoxShape only when one compiler-side, non-Clone
  same-loan view exposes the catalog-backed source `ParamDecl` rows and a
  separate physical-parameter descriptor for the receiver-prefix and every
  physical lane, together with physical-signature lane roles,
  `U64BitsOnI64` carrier rows, result/header, and source-backed effects under
  one owner/origin/brand.
  The view must be callback-scoped, issue no new semantic fact, and expose
  complete/disjoint physical coverage. Caller and callee ValueId projection,
  skeleton reservation, and BindingSSA adoption remain downstream consumers;
  they are not evidence for closing this D0. This gate is now closed as a
  design contract: the compiler-side issuer is a mechanical same-loan
  consumer, not a new semantic authority. Its implementation is the bounded
  input I0 below; it must still reject before any Builder effect when the
  relation is incomplete.

Current code census (2026-08-17):
  `S6CInstalledCallableLoanRefV1` already lends `selected()`, `signature()`,
  `storage_header()`, `physical_effects()`, `result()`, and scoped Completion
  from one installed Port callback. The catalog-backed storage projection
  owns source `ParamDecl`/name/type/attrs/uses rows; the physical-signature
  row owns lane role/order/BindingRef; and the S6C effects projection owns
  only the source-backed effect mask. The compiler-side issuer is now landed
  as the transport-only input I0: it joins only these borrowed siblings into
  physical parameter descriptors. Existing `MirParamDecl` and
  `create_resolved_function_skeleton` remain source-annotation and one-lane
  consumers, respectively. This census authorizes only the input aggregate,
  not a skeleton call or inference from `FunctionSignature` length.

Implementation receipt after D0 acceptance:
  `LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-I0` adds one
  callback-scoped compiler module that consumes the same loan and emits no
  Builder effect. It exposes descriptor rows and borrowed source siblings to
  the later skeleton consumer, but reserves no blocks, creates no `ValueId`s,
  adopts no BindingSSA, consumes no Completion, and lowers no loop. Skeleton
  reservation remains a separate later row.

Implementation receipt (2026-08-17):
  The I0 is landed. `VerifiedS6CStorageHeaderProjectionV1` is a distinct
  catalog-backed storage projection, and `VerifiedS6CPhysicalFunctionEffectsV1`
  is issued from the retained S6C call contracts only after both external
  operations prove `PureRead`. The installed Port lends both siblings beside
  the existing signature/result header in one callback-scoped loan. Focused
  S6C/package tests, format, check, and pointer guards are green. The next
  design stop is the physical function-entry/skeleton issuer; no Builder
  effect was opened by this I0.

Entry-input I0 receipt boundary (2026-08-17):
  The accepted D0 now permits one callback-scoped compiler transport module
  to consume the same installed S6C loan and build the nonsemantic physical
  descriptor rows. The module may widen package accessors needed to borrow
  source `ParamDecl`s, but it must not mint a second signature/header/effects
  authority. Its focused gate must cover static and instance receiver order,
  mixed ordinary/ExactText lanes, aliasing occurrences, and every rejection
  in the descriptor policy above. This receipt does not authorize skeleton
  allocation or any `ValueId`/Builder mutation.

Non-claims:
  No skeleton or entry-lane adoption is authorized by this transport slice. No
  Loop CFG block allocation, Loop topology, Binding read/write beyond the
  future entry-lane adoption, PHI sealing,
  operation/control lowering, Completion claim, Return/DraftSeal, lifecycle,
  Text route, production caller, fallback, or retry.
```

### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-FUNCTION-SKELETON`

```text
Decision:
  Accept the skeleton BoxShape. The landed same-loan descriptor aggregate is
  now sufficient to reserve one fresh unpublished physical skeleton. The
  caller-zero I0 below may create that detached shell, but it must not install
  it in a MirBuilder or adopt an ExactText BindingRef.

Source authority + canonical issuer:
  The same `PreparedCanonicalFunctionEntryInputV1` loan co-seals the selected
  catalog key/mode, S6C storage symbol/ParamDecl/return/attrs/uses, source
  effects/result, and the package-owned complete lane descriptor list. The
  new skeleton issuer is mechanical: it projects those facts to one
  unpublished `MirFunction` shell with one existing i64 carrier per physical
  lane. The future Builder/session remains the install/rollback consumer.

Non-authority:
  `MirParamDecl`, `FunctionSignature` length, logical `/N`, raw `ValueId`
  order, `ValueId(ordinal)`, Recipe/JoinSig ordinal, AST re-scan, and the
  existing one-lane `create_resolved_function_skeleton` path cannot define the
  physical carrier or choose the ExactText sidecar policy. A local entry block
  id in the unpublished shell is not a module allocation authority.

Fail-fast boundary:
  Selected/storage-key or mode/namespace drift, owner/effect mismatch, missing
  result/attrs/uses, descriptor count/index/carrier drift, and any attempt to
  install the shell or publish a BindingRef reject before Builder effect. No
  fixture-built skeleton, default effect, or fallback may repair it.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-FUNCTION-SKELETON-I0` consumes one accepted input,
  reserves the detached mechanical-i64 shell, retains descriptor carrier
  tags, and proves that no Builder/module publication occurred. The next
  design stop is a separate ExactText lane-adoption census: one logical
  BindingRef publishes once to the slot lane while generation remains a
  private move-only sidecar.

Non-claims:
  No Builder installation, ExactText lane adoption/BindingSSA publication,
  Loop CFG/PHI, Completion consumption/claims, DraftSeal, lifecycle, Text,
  route, production caller, fallback, or retry.
```

### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-ENTRY-LANE-ADOPTION`

```text
Decision:
  Accept this BoxShape and open the caller-zero adoption I0. BindingSSA remains a
  one-value map `(entry block, BindingRef) -> slot ValueId`: an ExactText
  formal publishes exactly once to the slot lane. The adjacent generation
  lane is retained in a private, move-only physical sidecar and is never
  published as a second semantic BindingRef value. The detached skeleton is
  only a reservation product; it is not itself an adoption owner.

Source authority + canonical issuer:
  The compiler-side same-loan entry issuer consumes the installed S6C
  physical signature/header/effects/descriptor cohort and issues one
  `PreparedCallablePhysicalParameterListV1` together with one private
  `PhysicalTextEntryLaneSidecarV1` plan. The future
  `CanonicalFunctionLoweringSessionV1::open` transaction is the consumer and
  rollback owner, not an issuer of lane meaning. Receiver is prefix `%0`
  with `SourceBindingSite::Receiver`; ordinary formals publish one
  `Parameter{ordinal}` lane; ExactText publishes one logical BindingRef to
  the adjacent slot lane and records the generation lane in the sidecar.
  Callee parameter ValueIds are pairwise distinct by physical lane. Caller
  argument occurrences may reuse a ValueId for aliases, but caller and
  callee ValueId scopes are never compared.

  The sidecar is an entry/forward transport receipt only. It may provide a
  scoped pair view for an admitted physical Text consumer, but it cannot
  rebind a BindingRef, perform generation arithmetic, expose generation via
  ordinary `read_entry`, or become a source/lifetime authority. Rebinding an
  ExactText formal is rejected in this first slice; a later pair-aware
  rebind receipt would be a separate decision.

Non-authority:
  Existing one-lane `adopt_exact_formal_parameter`, repeated publication of
  the same BindingRef, `ValueId(ordinal)`, `MirFunction.params` order alone,
  `/N`, `MirParamDecl`, raw slot/generation integers, ordinary BindingSSA
  reads, Recipe/JoinSig ordinals, a copied detached skeleton, fixture-built
  parameters, or a second Dynamic/legacy session cannot choose the policy.

Fail-fast boundary:
  Before Builder publication, one transaction must verify the same owner,
  selected identity, physical signature revision, dense lane order, receiver
  prefix, adjacent ExactText pair, lane carrier/type, and exact skeleton
  parameter count. It must publish each logical BindingRef at most once,
  retain every generation lane in the same non-Clone sidecar bound to the
  retained skeleton, and reject
  missing/duplicate/foreign lanes, swapped or non-adjacent pairs, duplicate
  BindingRef publication, rebind, generation-only read, type drift, or
  caller/callee scope mixing. Any failure drops the unpublished skeleton,
  pending BindingSSA state, and sidecar together exactly once; no partial
  Builder/module publication, retry, or fallback is allowed.

Smallest next slice:
  `EXACT-TEXT-ENTRY-LANE-ADOPTION-I0` consumes the retained skeleton in one
  fresh unpublished function transaction. It installs the physical shell with
  a live Builder entry block, adopts ordinary lanes and one ExactText slot
  lane through the canonical identity/SSA owner, and stores the adjacent
  generation lane in the skeleton-bound sidecar. The same transaction owns
  discard/rollback; it creates no Loop block/operation/control/PHI, consumes
  no Completion claim, and issues no ReadyLoopEntry.

Acceptance:
  This D0 is accepted as a BoxShape because the same-cohort prepared parameter
  list, retained skeleton, canonical identity publisher, and existing one-shot
  function-session discard terminal describe the intended ownership spine. The
  caller-zero canary proves the positive install/adopt path and duplicate
  adoption rejection, but it does not yet prove that the skeleton, descriptors,
  common-V2 session, BindingSSA publication, sidecar, and discard path are one
  consuming owner. The next design stop therefore keeps the I0 from being
  treated as a complete atomic transaction until the `into_parts` split and
  partial-publication failure path are sealed.

Non-claims:
  No loop CFG/PHI, Completion consumption/claim, DraftSeal, lifecycle,
  PinnedTextOp, Text route, literal/StringBox origin, production caller,
  fallback, retry, or main integration is opened by this D0. The I0 remains
  caller-zero and unpublished; its positive canary is landed, but atomic
  same-cohort/session ownership remains a separate design stop.
```

### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-ENTRY-SESSION-SEAM`

```text
Decision:
  Accept the one consuming transaction BoxShape and open a caller-zero I0.
  Replace the public `PreparedPhysicalFunctionSkeleton::into_parts` seam with
  a compiler-only, non-Clone `PreparedPhysicalEntrySessionInputV1` that owns
  the retained loan, detached shell, descriptor rows, and one cohort stamp.
  A builder-side `with_common_v2_physical_entry_session` consumes that input
  together with the one-shot common-V2 admission and the fresh function
  session; no caller can retain a shell or descriptor slice for re-pairing.

Source authority + canonical issuer:
  The installed S6C HRTB loan remains the sole source/cohort owner. The entry
  input and skeleton issuers project the package-owned signature/header/effects
  rows; the compiler-only session input retains
  `PhysicalFunctionEntryCohortStampV1 { owner, selected_key,
  signature_identity, lane_count }`. The consuming seam compares that stamp
  and all storage/result/effects metadata with the same admission loan, then
  calls `CanonicalSsaFunctionSessionV2::new_common_v2`. The outer
  `CanonicalFunctionLoweringSessionV1` is the Builder transaction and its
  `discard_unpublished` terminal is the sole rollback owner.

Non-authority:
  A public `into_parts` result, detached `MirFunction`, bare descriptor slice,
  `MirParamDecl`, `FunctionSignature` length, raw `ValueId` order, logical
  `/N`, fixture names, `CanonicalSsaFunctionSessionV2::new`, and a copied
  Completion/sidecar cannot establish same-cohort physical entry meaning.

Fail-fast boundary:
  Before the first Builder effect, reject foreign/reordered skeleton or
  descriptor rows, owner/key/signature-identity drift, metadata/result/effects
  drift, lane/value/type/count drift, duplicate BindingRef publication, and
  any non-empty Builder function slot. If a later publish or adoption step
  fails, the consuming owner drops the canonical session and calls
  `discard_unpublished` exactly once. A rejected transaction leaves no current
  function, BindingSSA entry, sidecar, or module-visible state; no retry or
  fallback is permitted.

Smallest next slice:
  The caller-zero I0 is landed. Its next execution slice is
  `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-I0`: lend the accepted typed topology
  through the same operation/control/JoinSig cohort. Keep block/effect
  emission, Loop operation/control lowering, Completion claims, DraftSeal,
  lifecycle, Text lowering, route, and production caller at zero until that
  transport closes.

Implementation receipt (2026-08-17):
  `with_common_v2_physical_entry_session` consumes the prepared input and
  issues admission from its retained loan, installs the detached shell and
  source Binding authority in one fresh `CanonicalFunctionLoweringSessionV1`,
  adopts the slot-only BindingSSA plus generation sidecar once, and calls the
  outer `discard_unpublished` terminal exactly once on both success and late
  callback failure. No session, Builder view, descriptor slice, Completion,
  or sidecar escapes the callback; the focused positive and late-failure tests
  are green.

Non-claims:
  No common-V2 physical operation/control lowering, CFG/SSA/PHI beyond the
  entry reservation, Completion consumption/claim, DraftSeal, lifecycle,
  PinnedTextOp, route/perf, production caller, fallback, retry, or main
  integration is opened by this D0/I0.
```

### `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept retention of the existing PhysicalFunctionEntryCohortStampV1 as a
  mechanical same-cohort witness. Move it exactly once from the prepared
  physical skeleton/session input into CanonicalSsaFunctionSessionV2 before
  any callback-scoped physical consumer is exposed. Do not create a new
  semantic stamp, session nonce, or physical-result authority.

Source authority + canonical issuer:
  reserve_common_v2_physical_function_skeleton is the sole issuer of the
  stamp from the installed S6C loan and complete descriptor cohort. The
  stamp carries only owner, selected callable key, callable signature
  identity, and physical lane count. The consuming physical-entry session
  moves that stamp into the canonical session; CommonV2CanonicalSessionRefV1
  may lend only a scoped borrow to a later same-session materializer.

Non-authority:
  FunctionOwnerId alone, selected key alone, descriptor count, logical /N,
  MirFunction/FunctionSignature, raw ValueId/BasicBlockId, Builder cursor,
  copied or reconstructed stamp, a runtime/session nonce, or a second Port
  loan cannot establish cohort identity. The stamp is not an exit, result,
  Text, lifecycle, or source-semantic authority.

Fail-fast boundary:
  Reject missing or already-moved stamp, owner/key/signature/lane drift
  against the retained loan, callback exposure before attachment, foreign
  session borrow, stamp clone/re-pair, result-receipt value copying, and
  escape beyond the session callback. Late unpublished-function discard must
  drop the session-held stamp exactly once; no retry or fallback is allowed.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-I0` removes the clone/
  discard seam, moves the stamp into the canonical session, adds one private
  scoped accessor through the common session wrapper, and tests positive
  retention plus missing/foreign/drift/late-discard negatives. It issues no
  physical condition result or ValueId.

Non-claims:
  No Compare lowering, physical Bool result, branch/edge effect, CFG/PHI,
  Completion/DraftSeal claim, lifecycle, Text/PinnedTextOp, route/performance,
  production caller, fallback, retry, or main integration.
```

The stamp is a cohort witness, not a unique invocation nonce. Its only valid
consumer is the same unpublished canonical session that consumed the prepared
entry input. A later condition-result receipt must borrow the session-held
stamp rather than copy it into a detached product.

### `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-I0` implementation receipt (2026-08-17)

The prepared physical-entry input is now move-only at the stamp boundary:
`take_install_parts` consumes the existing cohort stamp, and the consuming
physical-entry session attaches it before opening the outer Builder
transaction. `CanonicalSsaFunctionSessionV2` owns the stamp once, while
`CommonV2CanonicalSessionRefV1` exposes only a callback-scoped borrow. The
positive ExactText entry/session canary checks retained owner identity, and
the common-session canary checks the missing-stamp state before attachment.
`cargo test -q physical_entry --lib` (7 tests), `cargo test -q common_v2_ --lib`
(15 tests), format, diff, and pointer guards are green; the repository's
existing warning census is unchanged. No physical condition result, ValueId,
edge, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller
was opened.

### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-D0`

```text
Decision:
  Accept the V2-native physical-ID-free layout/placement BoxShape and open a
  caller-zero transport I0. The I0 only lends typed topology into the common
  envelope; it does not allocate blocks or emit effects.

Source authority + canonical issuer:
  The installed S6C loan's source-owned S6CLogicalOutputRowsV1 supplies typed
  loop/block/item topology, while the same cohort's existing JoinSig transfer
  view supplies control roles and ports. The existing S6C common-V2 issuer
  co-seals one non-Clone physical-layout input from those borrowed views and
  the operation/control/coverage siblings. The future session effect owner
  consumes this input; the outer CanonicalFunctionLoweringSessionV1 remains
  the sole rollback owner.

Non-authority:
  PreparedLoopPhysicalLayoutV1, fixed 13/15 counts, Recipe order alone,
  current_block, MirFunction.blocks, bare BasicBlockId/ValueId, V1 adapters,
  Dynamic topology, EffectMask, generic MirInstruction emission, and the
  entry block are not layout or placement authority.

Fail-fast boundary:
  Before any Builder effect, reject missing/foreign/reordered operation,
  control, JoinSig, segment, or transfer coverage; owner/target/stamp drift;
  duplicate or missing placement; and any attempt to infer layout from MIR.
  Before the transport callback returns, reject missing/foreign/reordered
  topology, item↔block drift, JoinSig role/port/loop mismatch, segment overlap,
  owner/target/stamp drift, or any physical-ID/MIR leakage. If a later
  allocation/effect is opened, every failure must use exactly one outer
  discard terminal with no retry, fallback, or publication.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-I0`: lend the source-owned typed
  topology through the existing installed HRTB/common-V2 envelope, co-seal it
  with operation/control/coverage, and add focused foreign/missing/duplicate
  and drift negatives. No Builder/session effect is allowed.

Census receipt (2026-08-17):
  `S6CLogicalOutputRowsV1` already retains loop/block/item rows, but the
  current common V2 envelope lends only operation rows, control rows, JoinSig
  transfer, and passive coverage. `LoopJoinLogicalTransferViewV2` intentionally
  carries ports/roles/payload, not Recipe block placement. The issuer is
  therefore a typed borrowed topology view, not a new count. The
  accepted layout shape is physical-ID-free `(loop, block, split ordinal)`
  segments with ordered item sets plus entry/After/resume and JoinSig transfer
  bindings. Allocation remains a later session effect; its I0 must decide
  whether the current CoreContext block cursor leaves an allowed ID gap on
  outer-session discard or receives an explicit transaction rollback.

Non-claims:
  No block allocation, operation/read/Const emission, CFG edge/terminator,
  PHI, Completion/DraftSeal, lifecycle, Text/PinnedTextOp, route/perf,
  production caller, fallback/retry, or main integration.
```

### Physical layout input I0 implementation receipt (2026-08-17)

`issue_s6c_common_v2_pre_session_v1` now co-seals a
`PreparedLoopV2PhysicalLayoutInputV1` from the same retained S6C ingress as
the operation/control/coverage siblings. The layout is physical-ID-free: it
contains source-owned loop/block keys, ordered item slices, checked split
ordinals, and the existing After binding. It validates owner, loop parent,
block ownership, item uniqueness, and After membership before the envelope is
returned. A second relation check proves that operation and If/Exit rows cover
the same disjoint item set, every referenced block belongs to the borrowed
topology, and every row item belongs to its specified block segment. The
focused operation/If/Exit block-drift negatives are green.

The new view is transported through the existing common-V2 envelope and is
accessible only inside the installed cohort loan. Focused common-V2 and
prephysical-ingress tests are green, including the foreign-owner rejection;
upstream source-row issuers retain the duplicate/missing topology negatives.
This I0 allocates no blocks, emits no operations/effects, opens no CFG/PHI or
Completion/DraftSeal claim, and does not select a Text route, fallback, retry,
or production caller. The post-layout effect frontier is now closed; this is a
historical receipt, not a live physicalizer or session reopen. The live
frontier is the physical condition-result receipt design stop below.

### Source-segment allocation boundary (landed 2026-08-17)

```text
Decision:
  Receipt: the source-segment/block skeleton allocation BoxShape was accepted
  and its caller-zero I0 landed. The first effect was not ReadBinding, Const,
  operation lowering, synthetic After allocation, or generic MirInstruction
  emission. After allocation was handled by its separate later D0.

Source authority + canonical issuer:
  The same common-V2 envelope's physical-ID-free layout is the topology input
  for source segments. Its retained JoinSig transfer view remains a logical
  transfer input, not an issuer of a new physical After block. A
  canonical-session block allocator is the sole physical block issuer, and the
  outer CanonicalFunctionLoweringSessionV1 is the sole unpublished-function
  rollback owner.

Non-authority:
  V1 physical layout/adapter, Recipe 13/15 counts, split ordinals alone,
  current_block, MirFunction.blocks, raw BasicBlockId/ValueId, EffectMask,
  the JoinSig After port alone, the After binding/class alone, synthetic
  After inference, generic MIR emission, ReadBinding, and a second rollback
  owner do not issue the allocation plan or its physical receipt.

Fail-fast boundary:
  Before Builder effect, require same owner/layout/session stamp, complete
  ordered segment coverage, checked block-count/cursor range, and no collision
  with the existing function entry. Allocate only source segments in layout
  order. Any late failure performs one outer discard with no retry, fallback,
  or publication. This D0 adopts the monotonic unpublished-ID-gap policy:
  discard restores function/CFG state but does not rewind CoreContext's global
  cursor; the gap is unobservable and never reused. No ambient cursor inference
  is allowed.

Smallest next slice:
  The existing source-backed V2 layout seam is sufficient for a segment-only
  plan. Open LOOP-COMMON-V2-PHYSICAL-SEGMENT-BLOCK-ALLOCATION-I0: consume one
  plan in the same callback, allocate only private segment->BasicBlock rows,
  preflight the checked cursor range, and test late discard. Then open a separate
  LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-D0 for a source-backed synthetic After
  row and its allocation owner. If the segment plan's owner/coverage/range
  checks fail, reject before effect.

Non-claims:
  No ReadBinding/Const/CallSlot/Text operation, synthetic After block,
  edges/terminators, CFG/PHI, Completion/DraftSeal, lifecycle, route/perf,
  production caller, fallback, retry, or main integration is opened by this
  D0.
```

### Segment block allocation I0 implementation receipt (2026-08-17)

`LOOP-COMMON-V2-PHYSICAL-SEGMENT-BLOCK-ALLOCATION-I0` is landed as a
caller-zero effect slice. The existing physical-ID-free layout now issues one
callback-scoped `PreparedLoopV2SegmentAllocationPlanV1`; the canonical session
consumes it and allocates exactly one unpublished `BasicBlockId` per ordered
source segment. The receipt retains only the source loop/block/split relation
and the newly allocated physical block; it does not issue edges, terminators,
operations, effects, or a synthetic After block.

Allocation preflights owner/function identity, checked count and cursor range,
entry collision, and segment coverage before mutation. The surrounding
`CanonicalFunctionLoweringSessionV1` remains the sole discard owner. A late
callback error discards the unpublished function while the CoreContext block
cursor remains monotonic; the resulting ID gap is unpublished, unobservable,
and never reused. Focused positive and late-discard tests are green.

This receipt does not open synthetic After allocation, edge/terminator or
operation lowering, ReadBinding/Const, CFG/PHI, Completion/DraftSeal,
lifecycle, Text, route/performance, fallback/retry, publication, or a
production caller. The next design stop is the separate
`LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-D0`, which must obtain a source-backed
resume relation before any After block can be allocated.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-D0` — accepted BoxShape

```text
Decision:
  Accept a source-backed After-boundary relation, but keep After block
  allocation closed.  The segment-only allocation I0 is complete; a
  synthetic After block is authorized only after this typed relation is
  transported.  The current S6C cohort admits RootAfter only; ParentResume
  remains a future source-backed arm until its issuer input exists.

Source authority + canonical issuer:
  One resolver/source semantic handoff retains the owner, function/frame,
  root/parent loop forest, JoinSig After, and source segment membership.  Its
  common issuer produces a callback-scoped
  `VerifiedLoopV2AfterBoundarySourceRelationV1` with a typed `RootAfter` or
  `ParentResume` relation, exact source membership, owner/frame stamp, and a
  later fresh-block allocation policy.  Existing JoinSig/layout issuers are
  inputs only; neither is extended to guess physical meaning.

Non-authority:
  JoinSig's `(loop, binding, class)`, parent index alone, segment order,
  split ordinal, Recipe/JoinSig counts, old V1 layout, nested-predicate
  topology fixture, MirFunction/Builder/BasicBlockId, current cursor,
  EffectMask, and copied source-site arrays cannot issue root/resume meaning
  or an After block.

Fail-fast boundary:
  Before any Builder effect, require one owner/cohort/frame, one root loop,
  one typed `RootAfter | ParentResume` relation, layout/transfer parity, and
  exact source-segment membership. Missing, foreign, ambiguous, nested-drift,
  duplicate-boundary, unsupported ParentResume, or HRTB-escape cases reject;
  no second After authority or fallback is permitted.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-I0` issues and transports the typed
  relation through the existing common-V2 envelope only.  After block
  allocation remains a later effect slice owned by the canonical session and
  outer discard owner.

Non-claims:
  No After BasicBlock allocation, rollback/cursor change, edge/terminator,
  operation/ReadBinding/Const/CallSlot, CFG/PHI, Completion/DraftSeal,
  lifecycle, Text, route, performance, production caller, fallback, retry, or
  publication.
```

### After-boundary transport I0 implementation receipt (2026-08-17)

`LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-I0` is landed as a transport-only
slice. The same S6C ingress that issues the physical-ID-free layout now issues
one non-Clone `VerifiedLoopV2AfterBoundarySourceRelationV1` carrying the
owner, source loop/frame evidence, and typed `RootAfter | ParentResume`
disposition. The admitted S6C path is `RootAfter`; `ParentResume` remains
parked until its source issuer input exists. The relation is retained inside
the existing common-V2 envelope, with focused RootAfter/owner parity evidence
and no second loan or JoinSig reissue.

This receipt performs no After block allocation, cursor/rollback change,
edge/terminator or operation emission, CFG/PHI, Completion/DraftSeal claim,
lifecycle, Text, route, fallback, retry, publication, or production caller.
The next bounded design stop is
`LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-D0`.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-D0` — accepted BoxShape

```text
Decision:
  Accept one synthetic After allocation effect after the typed source-backed
  boundary is transported. Keep allocation separate from edges, operations,
  CFG/PHI, Completion, and session construction.

Source authority + canonical issuer:
  Consume the existing VerifiedLoopV2AfterBoundarySourceRelationV1 and the
  source-segment allocation receipt from the same common-V2 envelope/session
  scope. A compiler-side PreparedLoopV2AfterAllocationPlanV1 is the one-shot
  plan issuer. CanonicalSsaFunctionSessionV2::create_unpublished_block is the
  sole BasicBlockId issuer; the outer unpublished-function transaction is the
  sole discard owner.

Non-authority:
  JoinSig After tuple, layout order, segment split ordinal, current Builder
  cursor, MirFunction blocks, Recipe counts, BasicBlockId, ValueId, ParentResume,
  or a profile-specific After physicalizer cannot infer allocation or a
  successor. A raw block receipt cannot be reacquired or paired with another
  relation after the plan is consumed.

Fail-fast boundary:
  Before Builder effect require one owner/function/relation stamp, RootAfter
  disposition, exact source-segment coverage, one unconsumed allocation slot,
  no entry/segment collision, and checked monotonic cursor range. The receipt
  is session-scoped and cannot escape the callback. Late failure must discard
  the unpublished function once; cursor gaps remain non-semantic and no
  retry/fallback is allowed.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-I0`: consume one prepared plan,
  allocate exactly one unpublished After block, and return only a
  session-branded `PreparedAfterBlockViewV1` inside the callback. Add no edges,
  terminators, operations, CFG/PHI, Completion/DraftSeal, lifecycle, Text,
  route, fallback, retry, or production caller.

Non-claims:
  No ParentResume admission, physical successor choice, operation/ReadBinding,
  effect emission, session publication, route selection, or legacy retirement.
```

The accepted BoxShape is intentionally a one-shot placement effect rather than
a new topology owner. The plan must be derived from the already-issued typed
After relation plus the already-issued source-segment receipt; it may not
rescan Recipe/layout facts or use the Builder cursor as meaning. The returned
physical block view is unpublished and callback-scoped. `create_unpublished_block`
advances the monotonic cursor; a late outer discard may leave an unused numeric
gap, which is non-semantic and is never reused.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-I0` — landed 2026-08-17

```text
Change:
  Add one caller-zero allocator for a RootAfter-only prepared plan. The plan is
  issued and consumed inside the common session, allocates one unpublished
  block through the canonical session allocator, and exposes only a
  session-branded view.

Contract:
  same owner/function/relation stamp
  RootAfter only; ParentResume = 0
  source-segment receipt is same-scope and complete
  one allocation slot -> one BasicBlockId
  outer function transaction owns every discard
  monotonic cursor gaps are non-semantic

Done:
  positive one-block allocation
  duplicate allocation rejection through the session-local one-shot state
  RootAfter and exact segment-coverage preflight
  cursor range/collision preflight with no mutation
  late callback failure leaves no published function/module
  callback-scoped view cannot escape or be consumed twice
  focused physical-entry gate, README/reference receipt, pointer guard

Stop:
  ParentResume, successor/edge/terminator, operation or ReadBinding emission,
  CFG/PHI, Completion/DraftSeal, lifecycle/Text, route/performance,
  production caller, fallback/retry, and legacy retirement.
```

Implementation receipt: the new `common_v2_after_block_allocation` module is
216 lines and keeps the prepared plan private to the common session. The
canonical CFG allocator issues the one physical block; no generic Builder
allocator or profile-specific physicalizer is introduced. Existing focused
physical-entry tests cover positive allocation, one-shot rejection, and late
discard after the After block has already been created.

The next design stop is a separate source-backed successor/edge decision. This
I0 does not make the new block reachable or choose a resume target.

The current layout view still carries only source loop/block/item segments and
an After binding, so it remains a transport input rather than an After
authority. The accepted boundary BoxShape names the missing source-backed
relation explicitly: the next I0 must issue and transport a typed
`RootAfter | ParentResume` relation from the resolver/source handoff. Current
S6C admits only `RootAfter`; `ParentResume` stays parked until its source
issuer input exists. No After block or physical resume target is inferred from
the JoinSig tuple, `EffectMask`, or Recipe order. `ReadBinding` stays a later
sibling: its source `BindingRef`, source-site, and Core effect anchor are not
present in the physical-ID-free layout rows and must not be invented.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-EDGE-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept a complete source-backed predicate branch plan and condition-carrier
  requirement as the next transport boundary, while keeping the newly
  allocated RootAfter block unreachable.
  The logical shape is the root predicate's Header -> Body / Header -> After
  pair, with PredicateFalse targeting RootAfter. A false-only edge plan is
  invalid because the canonical CFG branch issuer requires both successors.

Source authority + canonical issuer:
  The resolver-backed S6C logical loop condition (`condition_block` and
  `condition_value`), the existing `LoopJoinBoundaryTransferRefV2` predicate
  boundary rows, the
  source physical segment receipt, and the typed RootAfter relation are the
  inputs. The common-V2 source issuer co-seals the condition logical value, its
  condition segment, PredicateTrue -> Body, PredicateFalse -> RootAfter,
  owner/frame stamps, and a future physical condition carrier requirement in
  one same-session scope. The resulting plan is physical-ID-free and
  callback-scoped; CanonicalSsaFunctionSessionV2 remains the sole physical
  branch/edge issuer after a later effect slice consumes it.

Non-authority:
  JoinSig tuple alone, Recipe order, segment ordinal, current cursor,
  BasicBlockId, MirFunction predecessor lists, EffectMask, a copied source-site
  array, an already allocated After view, or a legacy V1 physical condition
  receipt may not infer a successor or condition carrier. Tail, Completion,
  ParentResume, operations, and generic CFG repair remain outside this
  boundary.

Fail-fast boundary:
  Before any edge effect require one owner/function/frame relation, RootAfter
  disposition, exact branch row and source-segment coverage, both Body and
  After targets, a source-backed condition carrier admission, and unused edge
  slots. Missing/duplicate/foreign rows, nested or outer-loop drift, absent
  condition projection, false-only planning, HRTB escape, late failure, or a
  second edge owner reject and invoke the outer unpublished-function discard
  exactly once; no retry or fallback.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-BRANCH-PLAN-I0` must transport one typed,
  callback-scoped complete branch plan and condition-carrier requirement from
  the same S6C cohort. It must not issue a ValueId, call `emit_branch`, mutate
  CFG, or add operations, ReadBinding, Completion/DraftSeal, lifecycle, Text,
  route, or production. A later edge-effect D0/I0 may consume the plan only
  after the condition physical carrier is itself closed.

Non-claims:
  No ParentResume admission, physical condition ValueId, `emit_branch` or
  `emit_jump`, edge/terminator implementation, operation lowering, PHI,
  Completion claim, DraftSeal, lifecycle, Text, route/performance, fallback,
  retry, publication, or production caller.
```

The After allocation I0 remains the only landed effect: it creates one
unpublished block and does not make it reachable. The source condition and both
logical branch rows already exist in the same installed S6C ingress. The
missing physical condition `ValueId` is deliberately represented as a future
carrier requirement rather than guessed or issued here. `CanonicalCfgSessionV1::emit_branch`
is one atomic two-successor operation; a false-only edge is never a valid
intermediate product.

### Branch-plan transport I0 implementation receipt (2026-08-17)

`common_v2_predicate_branch_plan.rs` now issues the accepted transport product
from the same installed S6C ingress. The non-`Clone`
`PreparedLoopV2PredicateBranchPlanV1` retains the resolver Bool condition, its
condition segment, the `Header -> Body` and `Header -> RootAfter` logical
targets, and a future condition-carrier requirement. It contains no physical
IDs and performs no CFG mutation. The envelope lends it through the existing
exactly-once HRTB; the focused installed-loan test checks owner, Bool class,
Body, and RootAfter. Unit negatives reject a missing or duplicate predicate
boundary. The next design stop is the physical condition-result BoxShape; no
edge-effect slice is opened until that carrier has its own session receipt.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-CARRIER-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept a two-stage condition carrier boundary. First, the same-cohort
  common issuer co-seals the exact source-backed CompareI64 producer relation
  with the logical branch requirement. Later, an operation materializer issues
  a session-local physical result receipt; no physical carrier is issued by
  this D0 or by the next transport-only I0.

Source authority + canonical issuer:
  The resolver-sealed S6C logical loop condition and its exact CompareI64
  producer row are the source spine. The common-V2 issuer must co-seal the
  producer item/block, `Less` operation, left/right logical operands, result
  key, Bool class, root loop, and owner/frame stamp. A later operation
  materializer uses operand physical receipts and
  `CanonicalSsaFunctionSessionV2` as the sole ValueId/type issuer; the branch
  consumer only borrows that physical receipt.

Non-authority:
  `PreparedLoopOperationProgramV2` result keys alone, Recipe order, MIR
  `ValueId`, `EffectMask`, legacy V1 Compare emitters, block cursor, or the
  branch plan may not infer the physical carrier. The condition requirement is
  logical evidence, not a physical value receipt.

Fail-fast boundary:
  Missing/duplicate/non-Compare producer, operand/result/block/class drift,
  foreign session/owner, stale or duplicate physical result, and Body/RootAfter
  target drift reject before `emit_branch` or any edge mutation. A producer
  must be materialized before the branch consumer can borrow its carrier.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-PRODUCER-I0` transports one typed
  producer relation from the same envelope and tests missing/duplicate,
  non-Compare, operand/result/block/class, and owner drift. It opens no
  operation lowering, ValueId issuance, `emit_branch`, CFG/PHI,
  Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production
  caller.

Non-claims:
  No physical condition ValueId, operation emission, edge/terminator, CFG/PHI,
  Completion/DraftSeal, lifecycle, Text route, performance, fallback/retry, or
  production caller is accepted by this D0.
```

### Condition producer relation I0 implementation receipt (2026-08-17)

`common_v2_condition_producer.rs` now issues one private, non-`Clone`
`PreparedLoopV2ConditionProducerRelationV1` from the same installed S6C
ingress as the predicate branch plan and operation program. It checks the
resolver loop condition against exactly one source `CompareI64` row, requires
the fixed `Less` producer and `I64` operands, and then checks the matching
generic operation row, owner, block, result, and Bool class before the envelope
is returned. The relation retains only logical keys and the producer item; the
canonical session remains the sole future `ValueId`/type issuer.

The focused common-V2 suite is green with the new non-Compare operation-row
drift negative and the installed-loan positive assertions. This I0 does not
issue a physical result, lower a Compare, call `emit_branch`, mutate CFG/PHI,
claim Completion/DraftSeal, open lifecycle/Text/route/performance, or add a
production caller. The next design stop is the physical result BoxShape, not a
second condition or edge authority.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-D0` — parent design stop 2026-08-17

```text
Decision:
  Keep this D0 at NoSafeSlice. A physical condition result may be issued only
  by one common-V2 operation materializer after it has borrowed the logical
  producer relation and exact physical operand receipts from the same
  canonical session. The materializer then uses the canonical session's sole
  ValueId/type issuer and returns one session-scoped Bool result receipt to the
  later branch consumer. This D0 does not issue or lower that result.

Source authority + canonical issuer:
  The source producer is `PreparedLoopV2ConditionProducerRelationV1`. Physical
  SSA/type authority remains `CanonicalSsaFunctionSessionV2`; the future
  materializer is only the co-seal bridge. The receipt must retain owner, a
  scoped borrow of the session-held physical-entry stamp, producer
  item/block/result key, and Bool type relation. Session-stamp retention is now
  landed; the result seam may borrow it but may not copy or reconstruct it.

Operand issuer census (2026-08-17):
  A source-backed two-row inventory is now carried by the common V2 envelope:
  Left is the condition-block ReadBinding and Right is the condition-block
  Length CallSlot. This closes source provenance only. The left still lacks a
  LoopValueKey/session-stamp physical receipt, and the right still lacks a
  canonical-session physical result receipt. `PreparedLoopOperationProgramV2`
  retains source rows only; verification definition maps are transient; old V1
  and Selected-Dynamic value ledgers are foreign authorities. Therefore both
  operands still cannot be borrowed as one same-session typed physical pair.

Smallest source-backed relation candidate:
  `PreparedLoopV2LengthOperandProducerRelationV1` is the only narrow candidate
  currently justified. The canonical issuer is the existing
  `issue_s6c_common_v2_pre_session_v1` source-ingress callback, which must
  co-seal the Length source contract (`StringLen`, Condition placement, arity
  zero), its `CallSlot` row/result/class, the matching Common V2 operation row,
  and the CompareI64 right key. It is a physical-demand relation, not a second
  language-semantic value authority; raw `LoopValueKeyV1`, generic CallSlot,
  role numbers, and later MIR lookup remain non-authorities.

  The relation and its fixed two-row inventory are issued with
  owner/placement/item/block/result/class coverage and foreign/duplicate/drift
  rejection. They are still logical transport only; no physical operand receipt
  may be guessed until the result BoxShape and its Length-call materializer
  boundary close.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-BOXSHAPE-D0` — design stop 2026-08-17

```text
Decision:
  Keep the parent result at NoSafeSlice and fix one thin, session-local
  materialization shape before any Compare or branch effect. The future
  `PreparedLoopV2PhysicalConditionResultPlanV1` is a physical-demand plan, not
  a semantic result: it borrows the source CompareI64 relation, the fixed
  two-row operand inventory, and the session-held cohort stamp. Its eventual
  `PreparedLoopV2ConditionResultReceiptV1<'session>` contains exactly one
  canonical Bool result ValueId plus its published Bool type and is valid only
  inside the same unpublished canonical session callback.

Source authority + canonical issuer:
  `PreparedLoopV2ConditionProducerRelationV1` and
  `PreparedLoopV2ConditionOperandInventoryV1` remain the source authorities.
  `CanonicalSsaFunctionSessionV2` remains the sole ValueId/type issuer; one
  future common-V2 condition materializer is the only bridge allowed to consume
  the two source rows, resolve the Left binding through canonical identity,
  materialize the Length CallSlot result through the existing canonical call
  owner, emit the CompareI64 `Less`, and issue the Bool receipt. The stamp is
  borrowed from the session wrapper and never copied into the receipt.

Non-authority:
  raw LoopValueKey, operation result key, raw ValueId, caller-supplied
  `MirType`, `EffectMask`, old V1/Selected-Dynamic value ledgers, generic
  CallSlot lookup, Recipe order, branch-plan condition key, or a standalone
  `emit_compare_i64_at` call cannot issue the result. The Length call's
  temporary physical result is private to the same materializer; it is not a
  second public operand authority.

Fail-fast boundary:
  Reject before physical mutation on missing/foreign session stamp, owner or
  function drift, producer item/block/result/op drift, missing or duplicate
  operand rows, Left binding read failure, absent/incorrect Length call result,
  non-`I64` operands, non-`Less` op, duplicate result publication, pre-existing
  result ValueId/type, materializer re-entry, branch-consumer mismatch, or
  receipt escape. Late failure uses the existing outer unpublished-function
  discard exactly once; no local rollback, fallback, or retry.

Acceptance boundary:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-BOXSHAPE-D0` closes this
  plan/receipt pair and the exact common-materializer owner. It must decide
  whether a same-session canonical Length result issuer exists without a
  second authority. The current census found no such issuer: keep
  `NoSafeSlice::CanonicalLengthResultIssuerMissing` and follow the separate
  Length-physical-result D0 below; do not infer it from raw MIR or reuse a
  legacy CallSlot emitter.

After acceptance:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-I0` may begin as a typed
  materializer/receipt-admission canary. It still emits no ValueId, Compare,
  edge, or terminator until its own BoxCount is opened.

Non-claims:
  No ValueId issuance, Compare instruction, call lowering, branch/edge,
  `emit_branch`, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is opened by this D0.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-OPERAND-PRODUCER-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept only a fixed two-row, physical-ID-free operand inventory for the
  current S6C predicate. Row Left is the condition-block ReadBinding producer;
  row Right is the condition-block Length CallSlot producer. The inventory is
  not a ValueId ledger and does not issue a physical result.

Source authority + canonical issuer:
  The existing source-ingress issuer
  `issue_s6c_common_v2_pre_session_v1` co-seals the typed Length source call,
  its CallSlot row/result/class, the matching common operation row, and the
  Compare producer relation. The same ingress supplies the ReadBinding source
  relation. This is a projection of existing authority, not a new semantic key
  space.

Non-authority:
  raw LoopValueKey, role numbers, generic CallSlot, transient verifier maps,
  CanonicalBindingReadReceiptV1 alone, raw ValueId, old V1/Selected-Dynamic
  ledgers, and later MIR lookup cannot issue or re-pair either row.

Fail-fast boundary:
  owner/cohort, producer item/block, result/class, Length role/op/placement/
  arity, receiver/argument shape, Compare-right relation, duplicate/missing
  row, detached provenance, or loan escape rejects before any physical value,
  instruction, session mutation, or edge effect.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-OPERAND-INVENTORY-I0` transports
  this non-Clone two-row inventory through the existing callback only.

Non-claims:
  no ValueId, Compare instruction, call lowering, branch, CFG/PHI,
  session-stamp retention, lifecycle, Text, route, fallback, retry, or
  production caller.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-OPERAND-INVENTORY-I0` — landed 2026-08-17

The I0 now issues/transports the accepted inventory through the existing
callback only. It adds no physical operand, ValueId, Builder, or session
effect. Focused positive, foreign-owner, and Length-operation-drift negatives
are green. The parent Result D0 remains the sole later authority for the
physical Bool result.

Non-authority:
  `LoopOperationV2` rows, branch-plan condition keys, raw `ValueId`, legacy V1
  Compare emitters, block cursors, MIR type maps, or a copied session stamp may
  not issue or re-pair the result. The branch plan is only a consumer of the
  future receipt; CFG/SSA/PHI ownership stays in the canonical session.

Fail-fast boundary:
  Missing same-session operand receipts, producer/item/block/result/op/class
  drift, owner/session/function-stamp drift, use-before-producer, duplicate
  materialization or receipt, missing Bool type publication, and receipt escape
  reject before `emit_branch` or any edge mutation. Late failure uses the outer
  unpublished-function discard exactly once; monotonic unissued ValueId gaps
  are non-semantic and no local rollback/retry or fallback is allowed.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-D0` is now the next
  design stop. Session-stamp retention is landed, but a canonical same-session
  Length result issuer is still missing; the parent Bool result must remain
  unaccepted until that seam is named.

Ordered sub-slices (design-only; no session/CFG effect):
  1. `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-OPERAND-PRODUCER-D0`
     and its `...OPERAND-INVENTORY-I0` transport are landed. They issue only
     the fixed source-backed logical rows; no physical receipt is issued.
  2. `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-D0` and its I0 are
     landed; the prepared skeleton and installed loan remain the only stamp
     authorities.
  3. `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-D0` and its I0
     canary are landed; they prove only same-session source/stamp transport.
  4. `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-PHYSICAL-RESULT-D0`
     must first name a source-backed
     `PreparedLoopV2StringLenCallTargetPlanV1` (target/receiver/zero-args/I64
     result) and then the one same-session `CanonicalLengthCallMaterializerV1`
     (that plan -> canonical session Call -> I64 result receipt). Existing
     Dynamic/legacy CallSlot emitters and CheckedCallOut are not reused.
  5. `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-BOXSHAPE-D0`
     may be accepted only after the physical Length issuer is closed; it then
     fixes the Bool receipt, outer discard owner, and sole later branch
     consumer.
  6. `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-I0`
     may begin only after both BoxShapes are accepted; it is a typed
     materializer/receipt admission canary and still emits no ValueId,
     Compare, edge, or terminator.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept one thin same-session boundary for the Length CallSlot result before
  the parent Bool result. The future physical result remains unimplemented;
  the first accepted consumer is only a private, non-Clone, one-shot
  `LengthCallMaterializationCanaryV1` that carries source/stamp provenance and
  issues no ValueId, type, CallSlot, CFG, or edge effect.

Source authority + canonical issuer:
  The existing S6C Length source contract, fixed Right operand row, matching
  common operation row, and Compare-right relation are the only source inputs.
  A future common-V2 materializer must consume them inside the same canonical
  session callback and use only the session's
  `issue_physical_value_id` / `publish_physical_value_type` mechanics. The
  `CommonV2CanonicalSessionRefV1` is the only future bridge and the canonical
  session remains the eventual mechanical ValueId/type issuer. The current
  session has no physical Length issuer yet; this D0 therefore authorizes only
  the no-effect canary below.

Non-authority:
  Generic `LoopOperationV2::CallSlot`, raw `LoopValueKeyV1`, raw `ValueId`,
  `MirType`, selected-Dynamic/legacy CallSlot emitters, CheckedCallOut
  projections, Recipe order, or a later MIR lookup may not issue or re-pair
  the Length result. No existing emitter may be renamed as the common owner.

Fail-fast boundary:
  Reject before physical mutation on missing/foreign Length role, non-
  `StringLen`, wrong Condition placement or zero-arity shape, receiver/args/
  result/class drift, producer or session owner/stamp drift, duplicate or
  re-entered materialization, missing canonical result publication, and any
  loan/receipt escape. Late failure uses the outer unpublished-function
  discard exactly once; local rollback, fallback, and retry remain forbidden.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-I0` consumes the
  relation, inventory, and session stamp once inside the existing callback and
  proves duplicate/foreign/drift rejection while leaving Builder state intact.
  After this canary, the parent Bool result BoxShape remains the next design
  stop; no physical Length result is claimed.

Non-claims:
  No ValueId issuance, CallSlot lowering, Compare instruction, branch/edge,
  `emit_branch`, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is admitted by this D0.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-I0` — landed caller-zero canary 2026-08-17

```text
Decision:
  Consume one source-backed Length relation, two-row operand inventory, and
  borrowed physical-entry stamp inside the existing common-V2 session callback
  exactly once. The canary is protocol state only: it emits no ValueId, type,
  CallSlot, instruction, CFG, edge, or semantic result.

Source authority + canonical issuer:
  `CommonV2CanonicalSessionRefV1` owns the callback-scoped consumer seam;
  envelope accessors lend the existing S6C relation/inventory and the session
  lends the existing cohort stamp. No source fact is reissued by the canary.

Non-authority:
  raw logical keys, raw ValueId, Builder maps, MIR type maps, generic
  CallSlot rows, selected-Dynamic/legacy emitters, and a second session are
  forbidden inputs and cannot be used to prove success.

Fail-fast boundary:
  Missing/foreign/duplicate relation, inventory or stamp, owner/function drift,
  wrong Length role/operation/placement/arity/receiver/result/class, re-entry,
  receipt escape, or late callback failure rejects with Builder unchanged.
  The outer unpublished-function transaction remains the only discard owner.

Acceptance:
  Positive same-cohort consumption, duplicate one-shot rejection, missing
  stamp, source-shape validation, and late-failure no-mutation tests are green.
  The parent Bool result BoxShape is now the next design stop for physical
  materialization.

Non-claims:
  No physical Length result, CallSlot lowering, Compare, branch/edge,
  `emit_branch`, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is admitted by this I0.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-I0` — landed caller-zero 2026-08-17

```text
Decision:
  Land one source-backed, non-physical StringLen target plan in the existing
  canonical-session callback. The one-shot plan is a pure admission product;
  it does not create a MIR Call, ValueId, type, or physical result.

Source authority + canonical issuer:
  `CommonV2CanonicalSessionRefV1` retains the installed S6C envelope and the
  moved physical-entry stamp. Its issuer consumes the existing Length source
  relation, CallSlot row, condition operand inventory, and CoreMethod target
  facts to issue `PreparedLoopV2StringLenCallTargetPlanV1` exactly once.
  The plan records only target/receiver/zero-args/I64/PureRead/non-suspending
  facts; canonical session remains the future physical ValueId/Call issuer.

Non-authority:
  Raw logical keys, raw ValueId, `CoreMethodOp` alone, canonical spelling,
  `/N`, MIR/JSON, generic/legacy CallSlot, CheckedCallOut, Selected-Dynamic
  ledgers, host/default target data, and a second session cannot issue or repair
  the plan.

Fail-fast boundary:
  Missing stamp, owner/brand/block/placement drift, wrong receiver/args/result/
  class/effect/policy, malformed source CallSlot, duplicate plan issuance, or
  late callback failure rejects before physical mutation; the outer unpublished
  function transaction remains the sole discard owner.

Acceptance:
  Same-cohort target facts, canonical StringBox.length, plan/canary parity,
  duplicate one-shot rejection, missing-stamp rejection, and late-discard
  no-mutation tests are green. Source and builder modules remain under the
  760/800-line limits.

Non-claims:
  No canonical Length Call/result receipt, parent Bool result, Compare,
  branch/edge, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is opened.
```

The canary is landed without opening a physical result. The next bounded
decision is `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-PHYSICAL-RESULT-D0`:
one same-session physical Length-result receipt issuer must be named before
the parent Bool plan/receipt can be admitted.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept one non-physical target-realization plan for the S6C StringLen row.
  This closes the source-to-target shape only; it does not emit a Call or
  issue a ValueId. The plan is the only input allowed to the later canonical
  Length-call materializer.

Source authority + canonical issuer:
  One same-cohort callback borrows `S6CLogicalCallInputRefV1`, its verified
  CoreMethod target facts, the fixed Length CallSlot row, the two-row condition
  operand inventory, and the retained physical-entry stamp. A resolver/recipe
  seam issues one non-Clone
  `PreparedLoopV2StringLenCallTargetPlanV1` containing owner/item/block,
  target/manifest brands, canonical StringBox method target, receiver relation,
  zero argument shape, I64 result relation/class, PureRead effect, and the
  non-suspending/non-control policy. It owns no physical IDs.

Non-authority:
  `CoreMethodOp::StringLen` alone, a canonical method string, `/N`, raw
  `LoopValueKeyV1`, `MirInstruction::Call`, generic/legacy CallSlot,
  Selected-Dynamic ledgers, CheckedCallOut, host/default target data, or a
  second session cannot issue or repair this plan.

Fail-fast boundary:
  Foreign owner/function/target/manifest brand, wrong placement or item/block,
  receiver/args/result/class/effect/policy drift, missing or duplicate target
  facts, missing inventory/stamp, target-plan re-entry, and loan/plan escape
  reject before Builder/session physical mutation. No selector, method name,
  receiver ValueId, or result ValueId may be reconstructed from MIR/JSON.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-I0` issues and
  consumes this plan once in the existing callback, with positive mixed-cohort
  coverage and foreign/duplicate/drift/late-failure no-mutation tests. It
  leaves the outer Builder unpublished and emits no Call, ValueId, type,
  Compare, edge, terminator, CFG, or PHI.

Non-claims:
  No canonical Length Call/result receipt, parent Bool result, Compare,
  branch/edge, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is opened.
```

### `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Add one mechanical projection from the already allocated common-V2 segment
  receipt to the physical block that owns the logical condition block. This is
  required before a Length Call can be placed; it does not issue a Call or a
  ValueId.

Source authority + canonical issuer:
  `allocate_v2_segment_blocks` remains the sole BasicBlockId issuer. The same
  `CommonV2CanonicalSessionRefV1` creates the segment receipt, finds the exact
  condition-block row from the envelope layout, and lends a callback-scoped
  `ConditionBlockPhysicalTargetRefV1`. The projection carries owner, logical
  condition block, physical block, and the retained entry-session stamp. It is
  a mechanical view, not a second layout or semantic authority.

Non-authority:
  Builder current block, block cursor arithmetic, logical `LoopBlockKeyV1`
  alone, `BasicBlockId` guesses, owner equality alone, copied segment rows,
  After allocation, target-plan facts, and a receipt imported from another
  session cannot select the condition block.

Fail-fast boundary:
  Missing/duplicate condition row, foreign owner, layout/segment drift,
  missing retained stamp, After-row confusion, callback escape, or late
  failure rejects before the target is lent. The generated segment blocks stay
  unpublished and the outer function transaction remains the sole discard
  owner; Call, ValueId, Compare, edge, and PHI effects are still closed.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-I0` uses a same-session
  callback to allocate the existing source segments and lend exactly one
  condition-block target. Positive, missing/duplicate-layout, foreign/late
  discard, and target-escape negatives are required.

Non-claims:
  No Length Call/result receipt, receiver ValueId, Compare, branch/edge,
  terminator, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is opened.
```

The condition-block target must be closed before the Length materializer can
be admitted. It is deliberately callback-scoped so a segment receipt from a
different physical session cannot be re-paired by owner or block equality.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-PHYSICAL-RESULT-D0` — design stop 2026-08-17

```text
Decision:
  Keep the parent Bool result at NoSafeSlice and first name the one
  same-session physical Length-result receipt issuer. The existing canary is
  source/cohort protocol only; it is not a physical operand receipt.

Source authority + canonical issuer:
  The source Length contract, fixed Right operand row, matching operation row,
  Compare-right relation, retained physical-entry stamp, and the
  callback-scoped condition-block physical target are borrowed from the same
  common-V2 session. The target-plan BoxShape and its caller-zero I0 are
  landed. The next missing product is a non-Clone
  `CanonicalLengthCallMaterializerV1` that becomes the sole plan-to-effect
  bridge: it
  issues exactly one canonical session `Call` and returns one
  `CanonicalLengthCallResultReceiptV1` through the session's sole ValueId/type
  mechanics. The later Bool-result materializer consumes that receipt in the
  same callback; it never reconstructs a CallSlot or re-pairs operands.

Non-authority:
  `LengthCallMaterializationCanaryV1`, raw `LoopValueKeyV1`, raw `ValueId`,
  generic/legacy `CallSlot`, `CoreMethodOp::StringLen` alone, canonical method
  strings, Selected-Dynamic ledgers, CheckedCallOut, MIR lookup, or a second
  session cannot issue the target plan or re-pair the physical Length result.
  The source inventory remains logical transport until the target plan is
  issued.

Fail-fast boundary:
  Missing/foreign session stamp, owner/function drift, Length role/operation/
  placement/arity/receiver/result/class drift, absent canonical result/type,
  target/manifest/target-brand drift, receiver/args/result drift, duplicate
  target-plan or result publication, result re-entry, operand-pair mismatch,
  or receipt escape rejects before any Call/Compare/branch/edge effect. Late
  failure uses the outer unpublished-function discard exactly once;
  fallback/retry is forbidden.

Smallest next slice:
  First land `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-I0`. Only then
  may `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-CALL-MATERIALIZER-D0`
  fix the receiver projection, direct canonical Call insertion, one-shot
  result receipt, and outer rollback boundary. The effectful I0 may emit
  exactly one canonical Call and one I64 result receipt only after those facts
  are closed; until then it must not emit ValueId, CallSlot, Compare, edge,
  terminator, CFG, or PHI.

Non-claims:
  No parent Bool receipt, Compare instruction, `emit_branch`, edge/terminator,
  CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route, production,
  fallback, or retry is opened by this D0.
```

The earlier canonical-session admission and physical-function-entry rows are
already landed caller-zero seams. This stop must not reopen them or use their
logical producer/descriptor rows as a second physical-result authority.

Non-claims:
  No physical ValueId, Compare lowering, `emit_branch`, edge/terminator,
  CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, performance,
  production caller, fallback, or retry is admitted by this D0.
```

Current blockers are deliberately explicit:

```text
NoSafeSlice::AfterConditionOperandPhysicalReceiptMissing
NoSafeSlice::AfterConditionSessionStampRetentionMissing
NoSafeSlice::AfterConditionPhysicalResultBoxShapeUnsealed
NoSafeSlice::AfterConditionPhysicalReceiptUnsealed
NoSafeSlice::CanonicalLengthResultIssuerMissing
NoSafeSlice::CanonicalStringLenTargetRealizationUnsealed
NoSafeSlice::CanonicalLengthCallMaterializerUnsealed
```

## Decision

Close the post-Recipe boundary before physical implementation begins.

```text
resolver / source map
  -> versioned VerifiedLoopSemanticProgramV1 | V2
  -> versioned complete operation/source/control demand
  -> PreparedLoopOperationProgramV1 | target V2
  -> one thin prepared execution product
       PreparedCallableLoopPhysicalizationV1
       OR PreparedGenericG0LoopPhysicalizationV1
       OR scoped target PreparedLoopV2PreSessionEnvelopeV1<'loan>
  -> target LoopV2CanonicalSessionAdmissionV1
       fan-in of the neutral envelope, separately admitted route policy,
       and callable signature/residence demands
  -> one fresh unpublished function session
       completion moves here exactly once
  -> outer callable lowerer + one common recursive Loop physicalizer
  -> open After continuation
  -> existing completion / DraftSeal
  -> one unpublished function draft
```

The target canonical full-operation input is a private, move-only, AST-free,
and physical-ID-free semantic-program receipt which feeds
`VerifiedLoopOperationPhysicalDemandV1`. It co-seals the Core-bearing
operation/effect product with one common continuation capability issued by
that Core's own JoinSig; the two are never independently re-paired at the
physical boundary. Each thin
prepared product is move-only and physical-ID-free. Its source-backed input
must be issued by one existing-owner ingress receipt that pairs the exact
`ResolvedFunctionLoweringInputV1` with its resolver ledger view and, where the
profile requires it, the exact callable index/header. No prepare/physicalizer
policy may inspect or rematch its AST. The current
`NormalCallableSemanticLoanPortV1` is only the raw-lowering host and does not
yet issue this receipt; that source-loan expansion is the remaining D0 gate,
not a reason to add a second resolver or to remove `cfg(test)` from prepared
types prematurely.
A profile prepare consumes one inner demand plus already sealed boundary
capabilities and fixes their exact compatibility before the first Builder
effect. Only the common inner demand is consumed by the recursive Loop
physicalizer. Neither product is a new Recipe, selector, SSA, CFG, PHI,
transaction, Return writer, or publication owner.

Nested control can split one logical Recipe block into multiple physical
segments. Generic G0 is the counterexample: root block B1 contains a carrier
read, a nested Loop item, then the root update. Therefore logical-block-to-one-
physical-block mapping is not a sufficient execution contract. A private,
move-only `PreparedLoopPhysicalLayoutV1` target is mechanically derived from
the complete Recipe/JoinSig and exact operation coverage before Builder
effect. It owns only ordered segment placement and transfer compatibility:

```text
Recipe item -> exact segment
segment -> ordered operation rows + one verified transfer
nested After -> exact parent resume segment
```

Recipe/JoinSig remain the sole logical authority. The layout may not infer
control meaning, reorder by item key, accept a profile name, or survive as a
second Recipe. Unsupported structural items reject with typed `NoSafeSlice`
before Builder mutation. Canonical CFG remains the sole physical block/edge/
terminator owner after the layout is admitted.

### Pre-cutover authority correction (2026-08-08)

Decision: accepted after external review and independent code audits. The
direction above remains authoritative, but current caller-zero code has two
known gaps and must not be activated in production yet.

First, current `VerifiedLoopOperationPhysicalDemandV1::issue` accepts semantic
context, operation/effect product, and continuation as separate verified
arguments. Its checks establish owner/scope/root-key compatibility but do not
prove that all three came from the same resolver Loop site/frame and the same
Core-owned JoinSig. The final issuer is:

```text
resolver-issued Loop source capability
+ exact LoopNodeKey -> source relations
+ complete item/carrier source relations
+ one existing entry-source owner's complete-coverage receipt
+ Core-bearing operation/effect product
    -> require continuation from this Core's JoinSig
    -> VerifiedLoopSemanticProgramV1
    -> VerifiedLoopOperationPhysicalDemandV1
```

`VerifiedLoopSemanticProgramV1` owns only the relational proof that these
existing products describe one executable Loop program. It is not a second
Core, Recipe, source observer, selector, input owner, or Callable plan. The
actual initialized-local input set and Generic parameter input contract stay
typed and distinct; each may issue only an opaque coverage receipt over the
same Recipe inputs. Raw `VerifiedLoopSemanticContextV1::from_parts`, external
continuation `from_after`, and the three-argument physical-demand issue path
are compatibility debt and reach zero callers in the co-seal migration.

For Recipe V2 Dynamic, the same rule is stronger: the caller passes only the
exact source/Recipe/envelope aggregate. A common JoinSig engine is invoked
inside the issuer, and `After(L0,B0,Dynamic)` is requested from that exact
JoinSig before `VerifiedLoopSemanticProgramV2` exists. A V2 caller cannot pass
owner, root key, JoinSig, After, or Continuation separately. `V10/ch`, Dynamic
Fault, Callable Tail, and the two-site Completion remain external to the Loop
carrier/continuation identity.

#### S6C complete-V2 pre-session admission (2026-08-15)

S6C uses the same rule without borrowing the Selected-Dynamic fixed cursor or
coercing its Recipe to V1. The landed common recursive physicalizer is
test-only and its operation program/dispatcher is V1-shaped. It therefore
cannot become production merely because the caller-zero S6C ingress and one
TextEq route policy exist.

The missing contract is owned by one ordered task family:

```text
LOOP-S6C-INSTALLED-CHILD-COMPOSITION-D0
LOOP-S6C-INSTALLED-CHILD-COMPOSITION-I0
CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0/I0
LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
LOOP-S6C-COMMON-V2-PRESESSION-I0
```

The child row is the sole pre-install semantic admission seam. The package
issuer co-seals the selected role/identity with the retained S6C Facts/Recipe/
Join child and moves one non-`Clone` Completion seed into it exactly once. The
Port only verifies the already-issued role/identity and takes/lends the child
exactly once after install; it does not reclassify S6C or accept a caller slot,
key, ingress, or fixture.

The callable-signature Decision is accepted: one logical ExactText formal and
one semantic BindingRef expand to adjacent scalar `u64` lanes
`[slot,generation]`. Logical `/N` and `physical_formal_lane_count` are separate
authorities; a by-value 16-byte aggregate is rejected. The package-owned sole
issuer consumes same-brand selected/batch identity plus the complete parameter
contracts only. Header/Completion, `MirType::String`, C validator argument
order, raw `ValueId`, root residence, and call-edge origin are not signature
inputs. The active caller-zero I0 may implement only this total lane map and
its combined Installed S6C loan.

The parent common-V2 row then closes the shared callable-signature, single-
Completion, and operation/control envelope:

```text
LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
LOOP-S6C-COMMON-V2-PRESESSION-I0

VerifiedNormalCallableSemanticPackageV1
  -> install consumes the verified product
InstalledNormalCallableSemanticPackageV1
  -> NormalCallableSemanticPackagePortV1 target extension
       exact lowering input and formal/header projections
       target TextFormal callable-signature mapping
       target VerifiedS6CSemanticChildV1 + one Completion (issued pre-install)
       target InstalledS6CChildAdmissionRefV1<'loan> (Port take/lend only)

AdmittedLoopTextEqRoutePolicyV1
  separate sibling branch; site-free; no Loop/item/value/source identity

PreparedLoopV2PreSessionEnvelopeV1<'loan>          target contract
  -> PreparedLoopOperationProgramV2
       every operation placement for the admitted V2 program
  -> Recipe + JoinSig + Layout control subproduct
       every If/Exit control placement for the admitted V2 program
  -> complete envelope receipt
       generic disjoint-union coverage

S6C adapter requirements
  exact 13 operations + one If + one Exit = all 15 placements

LoopV2CanonicalSessionAdmissionV1                 later fan-in
  consumes the neutral envelope + admitted route policy
  -> CanonicalSsaFunctionSessionV2
```

The verified package is consumed at install time; the catalog moves into
`CompilationContext`, while `InstalledNormalCallableSemanticPackageV1`
retains the same-brand batch/selection/formal/header state.
`NormalCallableSemanticPackagePortV1` is the only target lending seam. The
pre-session envelope is route-free and borrows that cohort only inside the
Port callback; it cannot retain a site borrowed from its own parent, clone
source/Completion ledgers, or accept raw keys from a caller. The installed S6C
child, one Completion ownership path, and the Completion-independent package
lane map are closed by their caller-zero I0 rows. The common-V2 BoxShape,
source-level transport R0, and the three-issuer I0 contract are now landed as
caller-zero source products. No generic V2 execution effect or Builder/session
admission is implied. Exact call-edge origin, Canonical composite adoption,
and Completion-backed lifecycle finish remain a later canonical-session
fan-in; they are never inferred from Recipe/MIR. The generic V2 exact-set
partition is the current common boundary.

### Common V2 pre-session BoxShape (accepted boundary; source implementation landed)

The D0 boundary is one non-splittable parent loan, not a public aggregate whose
fields can be independently reacquired. The target Port shape is a single
HRTB callback:

```text
installed package/Port
  -> with_s6c_common_v2_presession(|parent| { ... })

parent (private, same-brand, scoped)
  ├─ selected callable identity + catalog brand
  ├─ one Completion owner (borrowed only)
  ├─ package physical-signature sibling view
  ├─ retained S6C Facts/Recipe/Join/prephysical sibling view
  └─ neutral operation/control/coverage projections
```

The operation and control projections are sibling views of that parent. The
operation product covers every Operation placement for the admitted V2 input;
it does not hard-code the S6C cardinality. The control product owns only
`If`, `Exit`, and JoinSig-issued transfers. JoinSig, Completion, and callable
Tail add no placement. A passive coverage projection proves the disjoint union
of the operation and control sets. The S6C adapter alone must prove the exact
`13 + 1 + 1 = 15` coverage before any leaf is emitted.

The boundary is limited to same-brand ownership, scoped sibling lending,
generic operation/control partition, and passive exact coverage. The landed
adapter rejects missing/duplicate/overlapping/foreign placement, a detached
key/source ledger, and any need to infer the partition from MIR/Recipe order
before effect. It does not add Builder/session effects or issue a second
semantic authority.

### Common V2 transport R0 (accepted boundary; installed Port implementation landed)

Transport is a source-level projection, not a MIR/JSON carrier. The sole target
Port seam is one callback of the following shape:

```text
NormalCallableSemanticPackagePortV1
  -> with_s6c_common_v2_presession(|parent| { ... })
```

The callback consumes the same installed selected key once and lends a private
parent whose sibling views are selected identity/brand, the package-owned
physical signature, the one retained Completion owner, and the retained S6C
Facts/Recipe/Join ingress. The existing `with_s6c_child` and separate
signature/header accessors are not a second route: they are compatibility
substrates to be replaced or wrapped by this single parent seam in the I0.
No view may escape, be stored, or be recombined after the callback; the Port's
existing consumption ledger remains the exactly-once owner.

The Port callback now emits the three caller-zero sibling products from the
same retained cohort. JSON, MIR metadata, Builder/session, physical IDs, Text
route, lifecycle, fallback, and production caller remain out of scope.

The stable callable boundary is not the function-internal Text carrier. A
later non-splittable residence set couples the invocation lease-set token to
immutable UTF-8 root descriptors. Session-branded slices and backend-local
`TextPlan` values borrow those roots; raw pointer/length values are scoped
backend projections only. The common pre-session envelope owns none of these
physical identities.

### Common V2 I0 issuer contract (accepted and landed caller-zero, 2026-08-16)

All three issuers run inside the one installed-package/Port HRTB parent loan.
They are scoped sibling projections and cannot be independently acquired,
stored, or re-paired:

```text
same retained S6C source/Recipe/Join/prephysical cohort
  ├─ S6C operation adapter
  │    -> generic PreparedLoopOperationProgramV2<'loan>
  ├─ JoinSig control adapter
  │    -> generic If/Exit + transfer control product
  └─ passive coverage issuer
       -> disjoint-union/complete-coverage receipt
```

The operation issuer is the sole S6C profile adapter allowed to consume the
retained prephysical source view. It projects borrowed generic operation rows
(placement, operation payload, and existing execution-class evidence) for every
`Operation` placement in the admitted V2 program. It must not expose
`S6CPrephysicalOperationRoleV2`, the fixed `OPERATION_COUNT = 13`, S6C names,
`If`/`Exit`, JoinSig transfers, Completion, Builder, MIR/CFG/SSA/PHI, or route
policy. Only the S6C adapter may later assert its profile fact of 13
operations; the generic product never hard-codes 13.

The control issuer consumes the existing
`VerifiedLoopJoinClosureV2::logical_transfer_view()` as its sole JoinSig
logical source and co-seals only the matching Recipe `If`/`Exit` rows from the
same retained cohort. It does not reissue JoinSig, scan physical blocks, or
classify `If`/`Exit` as operations. Owner/loop/condition/exit-item,
branch-transfer, and After relations must agree before effect.

The coverage issuer is passive. It consumes the two already-issued sibling
products, rejects missing/duplicate/foreign/overlapping placement keys, and
issues only the disjoint-union/complete-coverage receipt. It does not rescan
Recipe or MIR and does not mint a third semantic product. The S6C adapter may
then prove `13 + 1 + 1 = 15`; the generic coverage product remains cardinality
neutral.

The source-level seams are implemented by the private/non-Clone
`issue_s6c_common_v2_pre_session_v1` adapter and its operation, control, and
passive coverage siblings. They may be issued only within the parent loan. Any
missing source anchor, foreign cohort, duplicate/overlap, HRTB escape,
detached row/key, or downstream re-pair is a typed rejection before effect.
The implementation does not open Builder/session, lifecycle, Text residence,
route, fallback/retry, production callers, or publication.

The complete envelope does not turn control into operations. The operation
program target covers every operation placement for the admitted V2 program;
the S6C adapter alone requires exactly 13. Recipe plus JoinSig remains the
logical owner of the S6C `If`, `Exit`, and their transfers; Layout only binds
that control to placement. A passive union receipt proves the generic
disjoint union, while the S6C adapter proves exact 15-placement coverage,
without creating a second control ledger.

Decision B applies to V2 as well: the complete V2 envelope is prepared and
checked before a private TextEq or CallSlot leaf is emitted. The existing
canonical CFG/Binding-SSA/Phi/Completion/finish/DraftSeal services are reused
inside one fresh unpublished session. V2-to-V1 adaptation, Dynamic-cursor
reuse, standalone leaf scheduling, and an S6C physicalizer are forbidden.
Actual handle/span/ValueId residence remains session-local and cannot be
issued by the pre-session product.

Second, current `physical_layout.rs` does not consume JoinSig transfers. It
reconstructs Predicate true/false, body backedge, nested entry, and child-After
resume from Recipe; `segment_allocator.rs` also rereads Recipe condition roles,
and `recursive_after.rs` emits the resulting transfer. This is caller-zero
evidence, not the accepted final transfer authority.

The corrected physical contract is:

```text
private Recipe traversal events
  -> item order and structural segment boundaries only
JoinSig-issued VerifiedLoopTransferV1
  -> logical role, exact control point, ports, payload, exit/After obligation
PreparedLoopPhysicalLayoutV1
  -> bind each verified transfer and operation to exact segments
CanonicalCfgSessionV1
  -> allocate and emit each admitted edge/terminator exactly once
```

The private traversal event stream may be retained inside
`PreparedLoopOperationProgramV1` and reused by schedule/layout preparation.
It carries no control target, is not public, and is never serialized as a
second Recipe. JoinSig must first gain exact item/control-point-keyed
capabilities for every admitted family. Therefore current typed
`UnsupportedAlways`, `UnsupportedIf`, and `UnsupportedExit` remain correct
until separate BoxCount rows land after the BoxShape authority cutover.

The existing `VerifiedLoopPhysicalDemandV1` is a closed topology-only P0
compatibility transport. It feeds only the historical caller-zero topology/
After probe, carries no complete operation/effect ledger, and cannot be
extended, renamed, or reused as the canonical operation input. The module-
split row moves the flat file into one directory facade, deletes the old flat
module, and quarantines that entry behind the topology-only test facade. Two
module entries or two topology authorities are forbidden.

`Admission` remains the semantic family-selection term. `Prepared...` means
only that already verified capabilities have been related into one executable
request. The prepared product owns exactly one new relational fact: its Loop,
Prelude/input, Tail, return ABI, Completion, owner, and frame may execute
together once. It does not own or copy the component semantic truths.

The existing owners remain authoritative:

| Concern | Sole owner |
| --- | --- |
| source membership, owner, frame, Scope/Region | resolver ledger and source map |
| logical operations, keys, recursive nesting | one recursive Loop Recipe algebra through its exact `LoopRecipeV1` or `LoopRecipeV2` projection; never V2-to-V1 coercion |
| logical ports, edges, carrier obligations | the JoinSig issued from that exact versioned Recipe/program cohort |
| source/effect/input relations | existing Core, initialized-local input, Generic parameter-input, item-source, and carrier-source products; none is replaced by the semantic program |
| cross-product source/Core/continuation compatibility | the version-matched semantic-program co-seal (landed/target `VerifiedLoopSemanticProgramV1` responsibility plus the D0-named V2 projection); relational co-seal only, never V2-to-V1 conversion |
| `BindingRef -> ValueId`, lexical SSA | `CanonicalSsaFunctionSessionV2.identity` |
| physical blocks, predecessors, sealing | `CanonicalCfgSessionV1` |
| provisional and patched PHI lifecycle | the function session's one `PhiTxn` |
| source completion evidence | `VerifiedFunctionCompletionV1`; remains owned by the installed cohort and is borrowed once to issue the session's physical consumer |
| mutable physical completion consumption | fresh `CanonicalSsaFunctionSessionV2.completion` / `ResolvedFunctionCompletionConsumptionV1` |
| common function-local finish terminal | `CanonicalSsaFunctionSessionV2::finish_for_draft_seal` target API |
| captured caller restore, unpublished discard, prepared close | `CanonicalFunctionLoweringSessionV1` |
| detached DraftSeal prepare and rejected-owner retention | `OpenFunctionDraftSealV1` |
| sole function commit terminal | `PreparedFunctionDraftSealV1::commit` through prepared session close |
| draft collection and module publication | `ModuleDraftCollectorV1` plus the existing module transaction / `ModuleBuilderInvocationSessionV1` |

## Why this boundary exists

Recipe completion is not physical completion. A verified Recipe proves the
logical program, but it does not prove:

- which already-resolved callable prelude supplies an external value;
- that every Recipe input can be materialized in the preheader;
- that the function return has an exact supported ABI;
- that the terminal source binding is the value returned;
- that one fresh session can finish CFG, SSA, PHI, completion, and DraftSeal;
- that late failure leaves the live caller unchanged.

These obligations must be sealed once before mutation. A physicalizer must not
rediscover them from AST, source names, route labels, or existing MIR.

### Repository audit receipt

| Observed code authority | Confirmed boundary |
| --- | --- |
| `CanonicalSsaFunctionSessionV2::new` consumes `VerifiedFunctionCompletionV1` into `ResolvedFunctionCompletionConsumptionV1` | Completion cannot remain owned by a prepared sibling after session open |
| `CanonicalDirectAccumSsaLowererV1::lower` finishes semantics/If/identity/Phi/binding/completion but omits `cfg.finish` | prose ordering alone is insufficient; the V2 finish terminal is required |
| `ReadyFunctionDraftSealV1::new` currently accepts only ready completion + current block | current Ready type does not prove common CFG/SSA/PHI closure by construction |
| `ResolvedFunctionLoweringInputV1` is an existing exact read-only source/function/forest/header view | prepared outer product may retain it; common Loop demand must not receive it |
| `NormalCallableSemanticLoanPortV1` currently forwards a raw body after consuming a loan, while `VerifiedNormalCallableSemanticLoanV1::into_parts()` retains only lineage + request-local lowering state | a source-loan expansion receipt must be issued before I0; AST re-walk, name lookup, and synthetic catalog/header pairing are `NoSafeSlice` |
| `CompilationContext::callable_declaration_catalog()` is installed before normal lowering | the catalog is an existing borrowed authority, not an automatic source/forest pairing; owner/frame/scope/index/header identity must be checked once |
| `loop_physical_prepare.rs::VerifiedCallableFunctionLoweringInputV1` is `cfg(test)` and static-header-profile-specific | it remains a canary witness; removing `cfg(test)` is not a production ingress design, and normal callables must not be forced through that header profile |
| `VerifiedCallableSingleLoopSourceMapV1` carries source roles, BindingRefs, loop context, and resolved exit evidence only | current co-seal cannot issue ABI or Completion authority |
| `PhiTxn::abort_on_err` sees only still-pending provisional PHIs | whole-session discard, not PHI rollback, owns atomicity |

### Current co-seal stop correction

`RECIPE-COSEAL-I0-R0` has no authority to issue an exact return ABI or a new
`VerifiedFunctionCompletionV1`; its current source map contains source-role and
resolved-exit evidence only. The current row therefore publishes these
disjoint caller-zero products:

```text
VerifiedLoopRecipeCoSealV1
  Core / Recipe / JoinSig
  operation-source and input-source relations
  semantic context
  VerifiedLoopContinuationContractV1

VerifiedCallablePreludeV1
VerifiedCallableTailV1
```

The existing exact ABI and Completion capabilities remain with their existing
issuers. `LOOP-PHYSICAL-PREPARE-P0` later consumes all components once and
either issues one prepared execution product or returns typed `NoSafeSlice`.
`VerifiedLoopAfterTailEnvelopeV1` is rejected: Loop continuation and callable
Tail must never be fused and then split again.

## Product boundary

### Common product

### Operation physical demand

The current caller-zero compatibility shape is:

```text
VerifiedLoopOperationPhysicalDemandV1 {
  context: VerifiedLoopSemanticContextV1,
  operation_effect: VerifiedLoopOperationEffectProductV1,
  continuation: VerifiedLoopContinuationContractV1,
  index: private LoopOperationPhysicalIndexV1,
}
```

The context owns only the resolver-issued semantic identity relation
(owner/origin/source-kind/loop-site/frame/Scope/Region); it is moved from the
existing source authority and is not re-derived from Recipe keys. The
operation/effect product owns the moved Core and item-keyed source/effect
ledger. The continuation owns only the logical Loop After capability. Callable
and Generic G0 adapters issue the common context and continuation by exact move
from their existing resolver/source products; they do not share source types,
compare counts, or pass two independent arguments to the physicalizer. The
index is a private key-only lookup aid and never a second semantic or physical
truth. The existing
`VerifiedLoopPhysicalBoundaryV1` remains topology-only and is invalid for the
operation program because it drops source anchors.

Decision B keeps whole-program preparation and leaf emission separate:

```text
VerifiedLoopOperationPhysicalDemandV1
  -> prepare_all
  -> PreparedLoopOperationProgramV1
       complete Recipe-derived operation schedule
       exact complete-coverage receipt

PreparedLoopOperationEmissionV1
  -> one private leaf emitter
```

The full demand exposes no first/select/filter/take-operation API. Recipe
Loop/Block/Item structure is the sole execution-order authority; an evidence
vector sorted by key is only storage order. `PreparedLoopOperationProgramV1`
retains the complete demand and common continuation. A leaf emission owns only
one already-prepared operation, source evidence, expected Loop, and expected
logical block; it never sees Recipe, profile, Tail, ABI, Completion, Return,
DraftSeal, publication, or continuation.

The first leaf canary may use a private test-only ConstI64 constructor, but it
must not obtain that row by extracting it from a seven-operation Callable or
fifteen-operation Generic G0 demand. A synthetic one-operation full Recipe is
not the first authority and may be added only as a later integration fixture.

The accepted conceptual shape has two layers:

```text
PreparedCallableLoopPhysicalizationV1
  input: exact ResolvedFunctionLoweringInputV1
  loop: VerifiedLoopOperationPhysicalDemandV1
  prelude: VerifiedCallablePreludeV1
  tail: VerifiedCallableTailV1
  return_abi: existing exact ABI capability
              (ExactTrivialReturnAbiV1 for the first profile)
  completion: VerifiedFunctionCompletionV1

PreparedGenericG0LoopPhysicalizationV1
  input: exact ResolvedFunctionLoweringInputV1
  loop: VerifiedLoopOperationPhysicalDemandV1
  tail: VerifiedGenericG0TailV1
  return_abi: existing exact ABI capability
              (ExactTrivialReturnAbiV1 for G0)
  completion: VerifiedFunctionCompletionV1
```

The exact Rust field split may remain private, but the following contract is
fixed.

The common prepare consumes the Loop co-seal once and moves it wholly into one
`VerifiedLoopOperationPhysicalDemandV1`. Callable Prelude/Tail remain separate
inputs; they are not
split back out of the co-seal. The prepare never borrows, clones, or re-catalogs
the co-seal. The inner receives, without duplication:

- verified Recipe/Core and JoinSig;
- the moved resolver-issued semantic context (including Scope/Region);
- semantic owner/origin/source-kind, loop source and execution frame;
- Scope/Region relation;
- exact operation-source and input-source relations;
- `VerifiedLoopContinuationContractV1`, which owns only the logical Loop After
  port/capability.

`LoopOperationPhysicalIndexV1` is a private, key-only search index over
existing logical keys:

- Recipe binding/value/item/block keys and their placement roles;
- Recipe input value -> logical preheader port;
- Recipe item -> owning Loop/Block + exact input/output value keys;
- JoinSig port/edge obligations -> logical placement roles.

`BindingRef`, source site, source/effect relation, semantic identity, Recipe,
and JoinSig truth remain solely in the moved co-seal owner held by the inner
demand. The private index cannot be independently constructed, published, or
verified; it refers to those logical keys and may be rebuilt only inside the
same prepare operation. The physicalizer consumes the demand as one product,
not Recipe plus a second public topology truth.

The physicalizer boundary is move-only. Prepare must issue a private consuming
operation for `VerifiedLoopOperationPhysicalDemandV1`; borrowing, cloning, a
second co-seal, or MIR reconstruction is invalid. This prevents logical demand
from being silently reused after it crosses into physical lowering.

The callable prepared product relates the non-Loop obligations:

- exact prelude caller/site/target/result contract when a prelude result is
  required;
- destination source `BindingRef` for that result;
- exact terminal return statement and value sites;
- exact terminal source `BindingRef`;
- exact supported return ABI capability;
- the matching `VerifiedFunctionCompletionV1`.

The prelude contract contains no `ValueId`. Its current source shape also does
not prove argument bindings: arity is not an argument materialization receipt.
The selected prerequisite is one move-only, AST-free
`VerifiedCallablePreludeArgumentListV1`. Each row carries an exact ordinal,
`SourceExprSiteV1`, resolver-issued `BindingRefV1`, and exact `i64` ABI. The
issuer reads `VerifiedResolvedFunctionV1.variable_ref(site)` and admits only
`ResolvedLexicalRefV1::Local` owned by the caller. Upvar, literal, nested
expression, unknown site, foreign binding, and unsupported ABI are typed
`NoSafeSlice`. No new resolver or semantic owner is introduced.

The outer lowerer consumes this list once, reads each BindingRef through the
canonical session identity, and materializes the external Prelude result. The
Loop input initializer is a separate exact source-site obligation: it is
resolved through the existing source view, emitted as the entry value, and
published under the co-sealed Loop input BindingRef. The Prelude result local
and Loop input binding must never be conflated. Only then does the adapter
issue one private `ReadyLoopEntryV1`. AST reread, name lookup, and arity-only
reconstruction are forbidden. It then opens the exact function session, moves
Completion into `CanonicalSsaFunctionSessionV2::new` exactly once, and retains
Prelude/Tail/ABI evidence only. The future full operation physicalizer consumes
the `VerifiedLoopOperationPhysicalDemandV1` plus that entry receipt and never
observes the callable boundary.

The Generic G0 prepared product wraps one instance of the same complete
operation-demand type but retains its existing `L0.After/b1` boundary
capability. It neither
reuses the callable prefix `value` Tail nor creates a G0 physicalizer.

The Generic G0 window lease is the source authority for its Scope/Region pair.
The lease therefore retains the pair alongside its existing owner/source/frame
brand, and the G0 product moves that context into the common demand. If a
profile cannot provide this exact pair, the demand is a typed `NoSafeSlice`; no
synthetic region or route-local context is permitted.

The G0 adapter must consume the existing S4 product into a common co-seal view
by a disjoint move of its already verified Core/relations/After evidence. If
that view cannot be issued without copying source truth or re-verifying the
Recipe, the G0 adapter is `NoSafeSlice` and parity remains parked.

### These are prepared execution products, not callable megaboxes

No profile prepared product becomes a universal callable semantic owner. It
implements no new Call, ABI, Loop, Return, or publication algorithm. It owns
only the relational compatibility proof that already sealed capabilities
belong to the same callable execution:

```text
source/target identity  -> existing resolver/catalog authority
argument/result ABI     -> existing verified ABI capabilities
Loop meaning            -> Recipe/JoinSig/co-seal
terminal disposition    -> existing completion capability
physical commit         -> existing DraftSeal owner

profile prepared product
  -> one exact owner/site/BindingRef compatibility proof
  -> one fixed Prelude -> Loop -> Tail -> Completion order
```

Prelude/Input, Loop, Tail/Return, and Completion stay typed sub-capabilities.
The two-layer product prevents the Loop physicalizer from observing the
callable boundary at all; only the outer callable lowerer sees both siblings.
They are not flattened into an opaque `CallablePlan` payload. The envelope
moves or borrows sealed evidence and cannot copy facts into a second catalog.
A non-Loop callable remains outside this Loop-specific prepared product, so
this D0 does not pre-empt the final general callable design.

### Completion and ABI are separate

`VerifiedFunctionCompletionV1` is necessary but insufficient. It seals exit
cardinality, terminal statement kind, target function, cleanup, and declared
result contract. It does not by itself carry the return value `BindingRef`,
the return expression site, or a concrete physical ABI. An unannotated
explicit return can therefore pass completion verification without being safe
for this physical row.

Each prepared profile with a value return requires both:

```text
exact terminal value site + BindingRef + exact return ABI
AND
matching VerifiedFunctionCompletionV1
```

For the first row the supported ABI is the already verified exact trivial
`i64` capability. Unannotated, dynamic, unknown, or inferred-by-name return
types reject before Builder effects. Later ABI profiles require separate
verified capabilities, not widening inside the physicalizer.

### Loop After and callable Tail are separate

The selected callable profile returns the prefix `value` binding:

```hako
local value = helper.to_i64(n)
local i = 0
loop i < 1 { i = i + 1 }
return value
```

Its terminal operand is not a Loop carrier After value. Generic G0 currently
returns `L0.After/b1`; that is a different profile adapter.

```text
logical Loop After capability != callable terminal Tail capability
```

The callable prepared product keeps both fields distinct. A profile adapter may prove
the same binding supplies both, but no consumer may infer that equality.
Generic G0's `VerifiedGenericAfterEffectG0` remains its boundary input and is
adapted beside the same common inner demand; it is neither the common Loop
authority nor the callable Tail authority.

## Forbidden contents

The inner demand and co-seal must be AST-free. The prepared profile product may
retain only the exact existing `ResolvedFunctionLoweringInputV1` source view;
it must not add independent raw source fields. Across these products the
following are forbidden:

```text
raw AST / StmtRef / ExprRef fields outside ResolvedFunctionLoweringInputV1
source or callable name selectors
path-suffix or ordinal rematching
legacy route ID or scheduler cursor
ValueId / BasicBlockId / PHI destination
MirBuilder or CanonicalSsaFunctionSessionV2
PhiTxn or rollback journal
ResolvedFunctionCompletionConsumptionV1
retry / fallback / reselection
commit or publication capability
```

The old DirectAccum-only `VerifiedLoopPhysicalInputV1` contains Recipe and
JoinSig only. It is a pilot input and must not be renamed or reused as the
final common demand; it lacks the co-sealed source/effect, continuation,
private-index, ABI, Tail, and prepared execution contract.

No session brand is added merely to pair either demand with a session. The
pre-effect issuer verifies semantic owner/frame/scope contracts; the consumer
then checks them against the freshly opened existing session. If the existing
session cannot expose the required prepare facts, the result is
`SessionPreparationUnavailable`, not a second session identity.

## Exact consumption

The common prepare consumes one `VerifiedLoopRecipeCoSealV1` plus the complete
operation/effect product and either issues one non-Clone
`VerifiedLoopOperationPhysicalDemandV1` or returns a typed rejection retaining
the sole unconsumed owner. A thin callable or G0 prepare then consumes exactly
one full demand plus the profile's disjoint boundary capabilities and issues
one prepared product. Neither step re-runs Recipe verification, mints keys, or
consults the legacy scheduler.

The outer profile entry consumes one prepared product to open the exact fresh
function session. The installed `VerifiedFunctionCompletionV1` remains owned
by its cohort; the session consumes one owned
`ResolvedFunctionCompletionConsumptionV1` issued from that scoped borrow. The
semantic Completion is not cloned or moved into a sibling boundary. The outer
lowerer retains only Prelude/Tail/ABI evidence, transfers the full operation
demand exactly once to the future full physicalizer, and later claims the
exact return operand through `session.completion`. Lowering by `&demand`,
cloning a split/prepared product, recreating one from MIR, or trying a second
route is forbidden.

Logical keys map to physical owners as follows:

| Logical evidence | Physical interpretation |
| --- | --- |
| `LoopBindingKey` + source `BindingRef` | canonical identity/BindingSSA |
| Recipe input + preheader relation | outer prelude/input materialization |
| `LoopItemKey` + owning block + value keys | common recursive physicalizer |
| JoinSig port/edge role | canonical CFG allocation and sealing |
| carrier obligation | canonical identity plus the one PHI transaction |
| Loop After capability | open allocation result first; sealed `ReadyLoopAfterContinuationV1` before any Tail read |
| terminal Tail capability | outer callable lowerer and completion consumer |

The topology physicalizer initially returns an open After/continuation receipt.
That receipt is not readable by Tail. A callable profile must first consume it,
issue the verified CFG edges, seal CFG and identity for every loop block, and
mint one session-local `ReadyLoopAfterContinuationV1`. Only that sealed receipt
may be passed to the outer Tail handoff. The physicalizer must not write
`Return`, take the function, publish a draft, or close the module.

### Session-local entry receipt

Opening a function session does not prove that Prelude/parameter/input values
have been installed. The outer profile lowerer must materialize every required
entry binding first and issue one private, session-local `ReadyLoopEntryV1`.
The future full operation physicalizer requires:

```text
PreparedLoopOperationProgramV1
+ ReadyLoopEntryV1
+ borrowed canonical CFG / Binding SSA / PhiTxn services
```

`ReadyLoopEntryV1` owns no source or callable semantics. It proves the
temporal fact that the exact logical input keys required by the demand, and
their resolver-issued BindingRef-to-entry materialization, are already
installed in this function session. It is non-Clone, cannot cross a session,
and is consumed once by the physicalizer. A receipt containing only arity or a
source-site label is insufficient.

The argument list is a Prelude product, not a Loop demand field. It is
consumed before `ReadyLoopEntryV1` is issued and is never passed to the common
physicalizer. This preserves the single common physical algebra while keeping
call argument source proof at the callable boundary.

## Fresh session and atomic failure law

Neither demand owns freshness or rollback. Existing transactions do.

```text
A. semantic prepare failure
  -> no Builder/session effect

B. fresh session open + exact owner binding
  -> reversible caller-capture/function-state effect
  -> no MIR / ValueId / BasicBlockId emission yet

C. physical emission failure
  -> rollback only still-pending unpatched provisional PHIs
  -> retain any PHI cleanup failure in the typed failure
  -> discard the complete unpublished function session
  -> restore the captured caller once
  -> no repair, retry, fallback, or route advance

fresh request
  -> open a new candidate/session from source authority
  -> allocate new physical IDs
  -> lower independently
```

Stage A must complete before B. Exact owner/session binding in B must complete
before C. Any B/C failure discards the whole session; it does not return to A
or select another route.

The sole owners are:

- fresh module/candidate state: existing `ModuleBuilderInvocationSessionV1`
  with the canonical Fresh seed policy;
- fresh function state and caller capture:
  `CanonicalFunctionLoweringSessionV1` over the existing function-owned state
  transaction;
- PHI-local provisional abort: the session's `PhiTxn`;
- whole unpublished function discard:
  `CanonicalFunctionLoweringSessionV1::discard_unpublished` or rejected
  DraftSeal discard;
- function commit: `PreparedFunctionDraftSealV1::commit` through the prepared
  function-session close;
- module commit: the existing module transaction / `ModuleBuilderInvocationSessionV1`
  terminal after `ModuleDraftCollectorV1` admission.

There is no Loop-local Builder clone, `LoopEmissionDraft`, undo log, second
transaction, or same-session retry. A fresh-session proof compares semantic
result and live-caller fingerprints; it must not require `ValueId` or
`BasicBlockId` numbers to match across sessions.

`PhiTxn::abort_on_err` rolls back only provisional PHIs that are still pending
and unpatched. It is best-effort local cleanup and diagnostic hygiene, not the
atomicity owner. It does not repair patched PHIs, other MIR instructions, or ID
allocation. Even if PHI cleanup itself reports a suppressed failure, the
poisoned unpublished function is still removed by whole-session discard.

## One typed function-finish terminal

The new common path must not rely on every lowerer remembering this order.
`CanonicalSsaFunctionSessionV2` gains one consuming target API:

```text
CanonicalSsaFunctionSessionV2::finish_for_draft_seal(...)
  -> Result<ReadyFunctionDraftSealV1, CanonicalFunctionFinishErrorV1>
```

Profile-specific ledgers remain with their profile lowerer. Before entering the
common terminal, that lowerer must consume them and provide one private
`ReadyCanonicalProfileCloseV1`. This is a temporal receipt, not a semantic
owner. The common terminal then consumes every common function-local owner and
is the only issuer of `ReadyFunctionDraftSealV1` for
`CanonicalSsaFunctionSessionV2` paths.

The target order is:

```text
1. materialize verified callable prelude and Recipe inputs
2. physicalize the recursive Loop, leaving After open
3. close the fixed profile's CFG edges and seal the After continuation
4. materialize the verified Tail operand and claim completion once
5. consume profile-specific ledgers -> ReadyCanonicalProfileCloseV1
6. close semantic scopes and seal the terminal CFG
7. finish CanonicalCfgSessionV1
8. finish semantic, If-control, and identity/BindingSSA preconditions
9. commit the one PhiTxn
10. finish the remaining resolved-binding ledger and
   ResolvedFunctionCompletionConsumptionV1
11. issue ReadyFunctionDraftSealV1
12. prepare every detached DraftSeal check
13. commit DraftSeal once
```

The current production resolved DirectAccum lowerer is a parity oracle, not the
final common owner. The earlier census found a missing whole-function
`CanonicalCfgSessionV1::finish` call; the typed
`CanonicalSsaFunctionSessionV2::finish_for_draft_seal` terminal now owns that
finish for the V2 path. The omission must not be copied into a future common
path. Existing non-V2 direct construction is frozen compatibility debt: the
first R0 adds no caller there, and final retirement makes
`ReadyFunctionDraftSealV1::new` unavailable to every production lowerer. Tests
may then use only an explicit test factory. For every migrated V2 path the
invariant is:

```text
ReadyFunctionDraftSealV1 exists
  == common CFG / SSA / PHI / binding / Completion owners are closed
  && the profile-specific close receipt was consumed
```

### R0 audit lock (2026-08-07)

The repository audit fixes the migration boundary before implementation. The
canonical V2 session is constructed by exactly three profile lowerers:

```text
trivial_ssa/lowerer.rs
direct_accum_lowerer.rs
nested_predicate_lowerer.rs
```

The current production `ReadyFunctionDraftSealV1::new` census contains those
three V2 callers plus one non-V2 `CanonicalFunctionLowererV1` compatibility
caller. R0 migrates only the three V2 paths. The non-V2 caller is a named
compatibility debt and may not gain new callers; its later retirement is a
separate decision. Test-only constructors are allowed only through an explicit
test factory and are not production evidence.

The finish API must consume a typed terminal receipt rather than re-deriving
source facts at the end of lowering. The target shape is conceptually:

```text
CanonicalSsaFunctionSessionV2::finish_for_draft_seal(
    self,
    builder,
    profile_close: ReadyCanonicalProfileCloseV1,
) -> Result<ReadyFunctionDraftSealV1, CanonicalFunctionFinishErrorV1>
```

The exact Rust visibility may remain private, but the contract is fixed:

- `body`, `body_end`, `target_function`, `current_block`, source site, and
  return operand are not re-inferred from raw AST/source/MIR arguments at the
  terminal. Function/body identity and completion target are sealed when the
  V2 session opens; the profile close receipt carries the exact terminal block
  and already-claimed completion witness.
- `ReadyCanonicalProfileCloseV1` is move-only, non-cloneable, and contains only
  profile-ledger closure evidence. It is a temporal receipt, not a new
  semantic owner or a second Completion/CFG/PHI authority.
- the common terminal is the sole issuer of `ReadyFunctionDraftSealV1` for V2
  sessions. A direct V2 `ReadyFunctionDraftSealV1::new` caller count of zero
  is a guard, not a prose claim.
- a mismatch, duplicate close, missing close, or completion/body identity
  mismatch rejects before the terminal consumes the session. Any failure
  after the fresh session opens discards the whole unpublished function and
  restores the caller once; same-session repair/retry is forbidden.

The R0 acceptance pack therefore includes all of the following, with no
profile or MIR acceptance delta:

```text
DirectAccum omission: missing cfg.finish cannot issue Ready/DraftSeal
finish order: CFG/semantic/If/identity/binding/Phi/completion close once
profile receipt: missing/duplicate/foreign receipt rejects
completion identity: body/site/end/target mismatch rejects before effects
late failure: unpublished function is discarded and caller is unchanged
fresh reuse: a failed session cannot poison the next session
caller census: V2 direct Ready constructor callers = 0
non-V2 census: compatibility caller remains named and non-growing
source/README/reference/current-entry update in the same implementation commit
```

This audit lock is deliberately narrower than a universal function-finalizer
redesign. It does not migrate the non-V2 lowerer, add a semantic owner, change
accepted profiles, or open physical Loop lowering.

### Callable production-edge census (2026-08-08)

The new callable physical products remain test-only:

```text
loop_physical_prepare.rs
callable_loop_physical_canary.rs
loop_recipe_physicalizer/callable_canary.rs
```

No production caller currently supplies
`PreparedCallableLoopPhysicalizationV1 -> profile-close -> Completion ->
DraftSeal`. The nearest production host is
`NormalCallableSemanticLoanPortV1::lower_normal_top_level_function`, whose
loop child edge still enters
`RawInvocationChildPortV1::lower_loop ->
PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1 ->
lower_loop_or_freeze_v1`. Its current output is a legacy pending function
session and `LegacyReplaceWholePair`, not `CompletedFunctionDraftV1`.

Therefore `CALLABLE-LOOP-PRODUCTION-EDGE-D0` closes as `NoSafeSlice`. The
Admission D0 confirmed that `NormalCallableSemanticLoanPortV1` is only a
production host/outer orchestrator. The accepted source/facts bridge design
does not add a semantic owner: `CallableSemanticSourceLedgerView` remains the
resolver source authority, while neutral SyntaxFacts and SourceMap are split
from test fixtures and promoted in
`CALLABLE-LOOP-PRODUCTION-SOURCE-FACTS-ISSUER-S0`. That source/facts slice is
closed with bounded negatives, exact resolver parity, and caller-zero audit.
The resolver seam is
`CallableSemanticSourceLedgerView::only_loop_site()` and the observer seam is
`FunctionSourceViewV1::stmt_at(membership)`; zero/multiple sites are typed
`NoSafeSlice`. The neutral SyntaxFacts and SourceMap issuers now compile in
production scope; their bounded entry uses resolver `only_loop_site()` plus
branded `stmt_at`, and the SourceFacts -> SourceMap parity receipt preserves
resolver identity. They still have no production caller or physical consumer.
Recipe/Prepared issuance remains closed; the next stop is the bounded logical
Recipe/JoinSig/After issuer implementation. A by-name adapter, fixture copying,
selector, retry, fallback, Generic G0 substitution, or legacy deletion is not
authorized by this census.

### Production admission contract (design-only)

The future production chain is fixed, but not implemented:

```text
NormalCallableSemanticLoanPortV1
  -> production source/facts bridge
  -> PreparedCallableLoopPhysicalizationV1
  -> fresh CanonicalFunctionLoweringSessionV1
  -> CanonicalSsaFunctionSessionV2
       (one physical Completion consumer issued from one scoped semantic borrow)
  -> Prelude / common Loop / After / Tail
  -> finish_for_draft_seal
  -> DraftSeal prepare/commit
```

Before production activation, `LOOP-SEMANTIC-PROGRAM-COSEAL-R0` replaces the
three separately supplied semantic fields with one consumed
`VerifiedLoopSemanticProgramV1`. The demand may retain a private lookup index,
but it cannot expose `first`/`select`/`filter`, split the semantic program, or
reconstruct context/continuation from matching keys. The old multi-argument
issuer and any caller that manufactures context or continuation from parts are
deleted in the same Refactor Series.

The source-facts step must promote the existing neutral
`VerifiedSourceSyntaxFactsV1` and `VerifiedCallableSingleLoopSourceMapV1`;
it must not create a new aggregate Bridge owner. It may consume only
resolver-backed source/facts/forest/projection and callable lineage products.
It must not re-walk AST, recover names from route labels, infer Recipe keys
from MIR, or remove `cfg(test)` from a fixture issuer. The SourceMap does not
own Recipe/JoinSig, ABI, Completion, physical IDs, CFG/SSA/PHI, DraftSeal,
collector, or module publication. Until S0 is accepted, the production
ingress returns typed `NoSafeSlice` before opening a function session.

The sole unpublished-function/discard owner remains
`CanonicalFunctionLoweringSessionV1::discard_unpublished`. Adapter failure is
pre-effect rejection; every later failure discards the whole unpublished
function and restores the caller once. Phi rollback is auxiliary cleanup, and
same-session repair/retry/fallback is forbidden.

## Typed rejection boundary

Before Builder effects, reject at least:

```text
foreign owner/origin/source-kind/frame/Scope/Region
missing, duplicate, foreign, or unconsumed logical key/relation
Recipe item/block owner mismatch
JoinSig port/edge mismatch
input without an exact preheader producer
prefix target/result ABI unavailable
prefix destination BindingRef mismatch
terminal value site/BindingRef mismatch
missing or unsupported exact return ABI
completion owner/site/result-kind mismatch
Loop After confused with callable Tail
unsupported logical operation, exit, or recursive depth
second Recipe/SSA/CFG/PHI/completion owner
physical ID or Builder capability present in the demand
```

These are typed `NoSafeSlice`/contract rejections. They do not fall back to a
profile-specific physicalizer or the 19-route scheduler.

After physical effects begin, any failure is terminal for that unpublished
session. It is not reclassified as a pre-effect decline.

## One recursive algebra; 19 is coverage only

The canonical full operation demand accepts one recursive Loop Recipe algebra
through an exact V1 or V2 projection. V2 adds typed operation/value vocabulary;
it is not converted into V1 and does not create another physicalizer. The
algebra does not contain `DirectAccum`, `GenericG0`, `LoopTrue`, `LoopCond`, or
the 19 legacy route labels as physical variants.

```text
source profiles/adapters: many bounded rows
portable Recipe algebra:  one, with exact V1/V2 projections
prepared profiles:        bounded callable/G0 compatibility products
full operation demand:    one
common physicalizer:      one
```

If the selected callable profile cannot later enter the existing family
selection envelope exactly, production selection returns `NoCandidate` and
parks it. Shape similarity must not relabel it as LoopV0 or Generic G0, and a
20th Recipe kind or second selector is forbidden.

## Finite implementation ladder

The bounded design is closed, but physical activation is intentionally split
into three mechanical commits. This is not a new semantic ladder: each row
consumes an existing owner and has one named temporal prerequisite. Do not
skip the After closure or reopen a Tail-only route.

| Order | Row | One claim | Stop line |
| ---: | --- | --- | --- |
| 0 | `RECIPE-COSEAL-I0-R0` | caller-zero common logical co-seal plus separate Prelude/Tail source contracts | closed caller-zero implementation; no ABI/Completion issuance |
| 1 | `CANONICAL-FUNCTION-FINISH-TERMINAL-R0` | migrate existing canonical V2 paths to one `finish_for_draft_seal` issuer; freeze non-V2 direct construction as compat debt | BoxShape-only; accepted profiles and MIR unchanged |
| 2 | `LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0` | fix callable input/prelude/terminal/G0/lifetime pairings in the existing prepare design | design-only; no code, Builder, physicalizer, selector, or caller |
| 3 | `LOOP-PHYSICAL-PREPARE-P0` | caller-zero common demand plus callable prepared product; exact ABI/Completion are consumed from existing issuers | no physicalizer, Builder emission, selector, or I0 claim |
| 4 | `GENERIC-G0-PHYSICAL-PREPARE-P0` | exact-move G0 adapter issues the same inner demand plus distinct G0 Tail | `NoSafeSlice` if source truth must be copied or reverified |
| 5 | `LOOP-PRELUDE-ARGUMENT-RECEIPT-P0` | resolver-issued variable-only i64 argument rows -> one move-only Prelude product | caller-zero; no Builder physicalizer or selector |
| 6 | `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0` | closed test-only inner demand + `ReadyLoopEntryV1` + borrowed V2 services -> topology/After continuation | no production caller; operation MIR remains `NoSafeSlice` |
| 7 | `LOOP-RECIPE-OPERATION-EFFECT-PLAN-D0` | one neutral `LoopItemKey` + exact source-anchor effect projection before operation emission | closed preparation; no production caller |
| 8 | `CALLABLE-LOOP-AFTER-CLOSURE-P0` | complete fixed callable operation schedule, issue CFG edges, seal CFG/identity, and mint one `ReadyLoopAfterContinuationV1` | closed caller-zero; no production selection |
| 9 | `CALLABLE-LOOP-TAIL-COMPLETION-P0` | consume sealed After, read exact Tail binding, `mark_return`, and claim completion once | closed caller-zero; no selector, retry, or fallback |
| 10 | `CALLABLE-LOOP-DRAFT-SEAL-P0` | consume profile close, call only `finish_for_draft_seal`, then DraftSeal prepare/commit | closed caller-zero; production selection and legacy deletion remain closed |
| 11 | `LOOP-CALLER-ZERO-PARITY-G0-D0` | accepted design: compiler-side exact-input composite ingress, neutral S4 owner, common physicalizer, distinct G0 After/Tail | no source reconstruction, physical emission, or production selection |
| 12 | `LOOP-CALLER-ZERO-PARITY-G0-I0-R0` | exact G0 ingress -> common fifteen-row `prepare_all` with Builder effect zero | closed 2026-08-08; no physical emission, Completion/DraftSeal, selector, retry/fallback, or legacy deletion |
| 13 | `LOOP-CALLER-ZERO-PARITY-G0-I1-D0` | top-down counterexample fixes segment/resume as a common prerequisite | superseded historical design; R1/R2/R3-I0 closed |
| 14 | `LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` | Builder-free Recipe-derived segment/resume layout plus exact order/coverage | **closed 2026-08-08**; no Builder effect or new accepted structural family |
| 15 | `LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` | exact segment-to-old-topology adapter and operation placement; Callable parity | **closed 2026-08-08**; not a segment allocator; no G0 physical |
| 16 | `LOOP-COMMON-RECURSIVE-AFTER-R3-I0` | exact segment allocator, retained completed program, complete transfer preflight, neutral After handoff | **closed 2026-08-08** for Callable caller-zero; G0 physical and production selection remain closed |
| 17 | `LOOP-CALLER-ZERO-PARITY-G0-I1-D1` | per-transfer Predicate receipts, neutral After boundary, and common DerivedCarrierEntry emitter contract | **accepted design 2026-08-08**; implementation is split into the common I0 row below and G0 I1 |
| 18 | `LOOP-COMMON-PREDICATE-CARRIER-I0-R0` | common per-transfer Predicate values plus profile-neutral DerivedCarrierEntry emission | **closed 2026-08-08**; no G0-specific owner or production selection |
| 19 | `LOOP-CALLER-ZERO-PARITY-G0-I1-R0` | exact parameters, five segments + root After, all fifteen operations, distinct Tail/Completion, finish/DraftSeal | **closed 2026-08-08** caller-zero; no G0-specific physicalizer |
| 20 | existing M8 S6A..S6G + M9 S7A..S7G | close all-19 ingress coverage and Rust/.hako portable producer parity | repository-wide convergence; not a prerequisite for the bounded selected H2 first cutover unless its unchanged source needs that family |
| 21 | `LOOP-SEMANTIC-PROGRAM-COSEAL-R0` | exact node/source/entry coverage + Core-owned continuation -> one semantic-program input; migrate callers and delete split issuance | BoxShape Refactor Series; no accepted-shape or production change |
| 22 | `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` | one private traversal, JoinSig-issued transfers, Layout binding only, direct transfer inference deletion | BoxShape Refactor Series; current Predicate/nested cohort only |
| 22a | `LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0` | make V1/V2 physical consumers borrow one complete ordered operation/source-effect ledger; remove repeated Recipe/evidence `find` scans | behavior-preserving consumer refactor only; no V2-to-V1 adapter or new source/effect authority |
| 22b | `LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0` | move Callable profile-close/Tail/ABI/Completion out of the common Loop physicalizer; common stop is `ReadyLoopAfterContinuationV1` | BoxShape only; no accepted shape, profile callback, selector, or production switch |
| 22c | `LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0` | parent BoxShape: order the installed child, TextFormal mapping, one Completion owner, and generic V2 operation/control envelope | closed design boundary 2026-08-16; one parent HRTB/sibling views, generic operation/control partition, and passive coverage are fixed; no session effect |
| 22c-a | `CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0/I0` | accepted mapping: one logical ExactText ordinal/BindingRef -> adjacent scalar `[slot,generation]` lanes; issue one complete/disjoint Completion-independent package cohort and transport it through one combined Installed S6C loan | closed caller-zero implementation; no call-edge actualization, `ValueId`, aggregate ABI, fallback, or retry |
| 22d | `LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0` | transport the generic parent/sibling boundary through one installed Port HRTB without emitting an execution product | closed caller-zero source transport 2026-08-16; one selected-key consumption seam; no JSON/MIR, route policy, Builder/session effect, or production caller |
| 22e | `LOOP-S6C-COMMON-V2-PRESESSION-I0` | implement the named source-backed operation adapter, JoinSig/Recipe control co-seal, and passive coverage issuer inside one caller-zero parent loan | closed caller-zero implementation 2026-08-16; focused positive/negative/duplicate tests green; no S6C physicalizer, Builder/session, lifecycle, route, or production caller |
| 22f | `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0` | census fixed-role receipts versus segment receipts and publish the caller-zero deletion gate | independent census before cutover; never a prerequisite for issuing V2 meaning and delete only after production/test callers reach zero |
| 23 | `LOOP-PHYSICAL-ALWAYS-COVERAGE-I0` | add one JoinSig-authorized Always physical family | one BoxCount commit; no fallback |
| 24 | `LOOP-PHYSICAL-IF-COVERAGE-I0` | add exact branch/merge transfer capabilities and common physicalization | one BoxCount commit; no Layout inference |
| 25 | `LOOP-PHYSICAL-EXIT-COVERAGE-I0` | add item-keyed Break/Continue/Return transfer capabilities and common physicalization | one BoxCount commit; no route-local exit writer |
| 25a | `LOOP-COMMON-V2-CANONICAL-SESSION-ADMISSION-D0` | fix the two-stage admission BoxShape and census its three source authorities | accepted 2026-08-16; outer-If and Completion reuse existing issuers, and typed BlockExpr issuance/transport are now landed |
| 25a-a | `RESOLVED-BLOCK-EXPR-EXPECTATION-I0` | co-seal typed BlockExpr body-shape sites with the exact resolver scope/region pairs and store one non-Clone receipt in the callable batch row | landed 2026-08-17; no selected/package transport, raw count API change, or session effect |
| 25a-b | `CALLABLE-BLOCK-EXPR-EXPECTATION-TRANSPORT-I0` | lend the batch-owned expectation through the existing selected/package HRTB | landed 2026-08-17; transport only, no reissue, clone, Completion consumption, or session construction |
| 25a-c | `LOOP-COMMON-V2-CANONICAL-SESSION-ADMISSION-I0` | co-seal exact Loop outer-If residual, typed BlockExpr expectation, common V2 envelope, and actual borrowed Completion in one callback-scoped admission | landed 2026-08-17; caller-zero/effect-free; no `CanonicalSsaFunctionSessionV2`, DraftSeal, lifecycle, Return rescan, or legacy-finalizer retrofit |
| 25b | `LOOP-COMMON-V2-PHYSICAL-SESSION-I0` | consume the accepted admission and open one caller-zero canonical session owner without exposing a second loan | landed 2026-08-17; typed expectation projects inside `new_common_v2`, borrowed Completion yields one owned physical consumer, and the envelope remains callback-scoped; no Builder/CFG effect, claim, DraftSeal, lifecycle, or physicalizer |
| 25b-a | `LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-D0` | census one same-cohort physical function skeleton and exact entry-lane adoption boundary | accepted BoxShape; input and detached skeleton I0 are landed, while lane adoption remains a separate design stop; no Builder effect, Loop CFG/block allocation, operation/control physicalization, PHI, Completion claims, DraftSeal, lifecycle, route, fallback, retry, or production caller |
| 25b-b | `LOOP-COMMON-V2-PHYSICAL-HEADER-COSEAL-D0` | accept one package/installed-loan issuer for S6C storage header, result, attrs/uses, source-backed effects, and physical signature relation | accepted BoxShape; caller-zero I0 is the only open effect; no skeleton or Builder effect |
| 25b-b-I0 | `LOOP-COMMON-V2-PHYSICAL-HEADER-COSEAL-I0` | issue/transport the same-brand S6C storage header and source-backed physical-effects projection beside the existing signature | landed 2026-08-17; focused package/S6C tests green; no session, skeleton, ValueId, ExactText adoption, Loop block, PHI, Completion claim, DraftSeal, lifecycle, route, fallback, retry, or production caller |
| 25b-c0 | `LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-D0` | carrier choice is fixed as package-owned `U64BitsOnI64` over the existing i64 mechanical carrier; define the same-loan physical-parameter descriptor/lane-role contract, including source ParamDecl, receiver, and ExactText pair policy | accepted BoxShape 2026-08-17; no skeleton, ValueId, lane adoption, Loop blocks, PHI, Completion claim, DraftSeal, lifecycle, route, fallback, or production caller |
| 25b-c0-I0 | `LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-I0` | consume one accepted same-loan view and expose nonsemantic physical parameter descriptors for the later skeleton consumer | landed 2026-08-17; caller-zero transport only; no skeleton allocation, ValueId, BindingSSA, Completion consumption, Loop CFG, lifecycle, route, fallback, or production caller |
| 25b-c | `LOOP-COMMON-V2-PHYSICAL-FUNCTION-SKELETON-I0` | reserve one fresh unpublished physical function skeleton from the accepted same-cohort entry input | landed 2026-08-17; detached mechanical-i64 shell and descriptor retention only; no Builder installation, ExactText adoption, Loop blocks, PHI, Completion claim, DraftSeal, lifecycle, route, fallback, or production caller |
| 25b-d | `LOOP-COMMON-V2-PHYSICAL-ENTRY-LANE-ADOPTION-D0` | accept the one-value BindingSSA plus private generation-sidecar adoption and its fresh-transaction rollback owner | accepted BoxShape 2026-08-17; slot-only publication and skeleton-bound sidecar are fixed; no Loop CFG/PHI, lifecycle, route, fallback, or production caller |
| 25b-d-I0 | `EXACT-TEXT-ENTRY-LANE-ADOPTION-I0` | consume one prepared skeleton for ordinary lanes and one logical ExactText slot lane plus adjacent private generation sidecar | landed caller-zero canary 2026-08-17; positive install/adopt and duplicate-adoption rejection are green, but atomic same-cohort/session ownership remains the next design stop |
| 25b-e | `LOOP-COMMON-V2-PHYSICAL-ENTRY-SESSION-SEAM-D0` | bind retained skeleton, descriptor cohort, common-V2 session, slot-only BindingSSA, sidecar, and one discard/poison owner into a consuming transaction | accepted BoxShape 2026-08-17; compiler-only consuming input and Builder rollback owner fixed; no Loop CFG/PHI, lifecycle, route, fallback, or production caller |
| 25b-e-I0 | `LOOP-COMMON-V2-PHYSICAL-ENTRY-SESSION-SEAM-I0` | consume one prepared input and one common-V2 admission, install/adopt once, and return only a callback-scoped success view | landed 2026-08-17; same-loan admission, fresh Builder transaction, slot-only BindingSSA plus generation sidecar, and outer discard/no-retry are covered by positive and late-failure tests; no Loop CFG/PHI, lifecycle, route, fallback, or production caller |
| 25b-f | `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-D0` | accept one source-backed V2-native physical-ID-free layout/placement BoxShape | accepted 2026-08-17; topology transport is the only next I0; no block/effect emission, Loop CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-f-I0 | `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-I0` | lend typed loop/block/item topology and JoinSig transfer bindings through the same common-V2 cohort | landed 2026-08-17; relation guard is green; no Builder/block allocation, operation/read/Const, CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-f-I0-RG | `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-I0-RELATION-GUARD` | require each operation/If/Exit item to belong to its specified layout block and add focused negatives | landed 2026-08-17; positive transport plus operation/If/Exit block-drift negatives are green; no Builder/block allocation, operation/read/Const, CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-g | `LOOP-COMMON-V2-PHYSICAL-ENTRY-EFFECTS-D0` | after layout input is sealed, name the first source-segment block allocation carrier and rollback boundary | accepted BoxShape 2026-08-17; monotonic unpublished-ID gaps are explicit, synthetic After allocation is a separate D0; no ReadBinding/effect emission, Loop CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-g-I0 | `LOOP-COMMON-V2-PHYSICAL-SEGMENT-BLOCK-ALLOCATION-I0` | consume one accepted segment allocation plan and allocate only ordered source-segment blocks under one outer discard owner | landed 2026-08-17; positive and late-discard gates are green; no synthetic After block, edges/terminators, operation/read/Const, CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-h | `LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-D0` | issue a source-backed synthetic After row with root/resume relation and its separate allocation owner | accepted BoxShape 2026-08-17; typed RootAfter transport I0 landed, ParentResume remains parked until its issuer input exists, and the allocation D0/I0 are now landed before any After edge/terminator, operation/read/Const, CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-h-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-I0` | transport the typed RootAfter/ParentResume boundary relation through the same common-V2 cohort | landed 2026-08-17; RootAfter is the only admitted S6C arm, ParentResume remains parked, and no block/edge/terminator/effect/CFG/PHI/Completion/DraftSeal/lifecycle/Text/route/production is open |
| 25b-i | `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-D0` | accept one RootAfter-only one-shot unpublished After placement and its outer discard owner | accepted BoxShape 2026-08-17; one prepared plan, canonical BasicBlockId issuance, exact segment coverage, and monotonic unpublished cursor gaps are fixed; the next I0 is allocation-only |
| 25b-i-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-I0` | consume the private plan and allocate exactly one unpublished After block | landed 2026-08-17; positive/one-shot/late-discard gates are green; no After edge, successor, operation, CFG/PHI, Completion, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-j | `LOOP-COMMON-V2-PHYSICAL-AFTER-EDGE-D0` | close the source-backed complete Predicate branch plan and condition-carrier admission for Header -> Body / Header -> RootAfter | accepted BoxShape 2026-08-17; the false-only edge is rejected because canonical `emit_branch` requires both successors; the next caller-zero I0 transports the physical-ID-free plan; no edge/terminator, operation, CFG/PHI, Completion, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-j-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-BRANCH-PLAN-I0` | transport one typed complete predicate branch plan plus condition-carrier requirement from the same S6C cohort | landed 2026-08-17; focused positive/duplicate/missing-boundary gates are green; no ValueId issuance, `emit_branch`, CFG mutation, operation/read/Const, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-k | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-CARRIER-D0` | name the source-backed physical condition carrier and its canonical issuer before any edge effect | accepted BoxShape 2026-08-17; logical CompareI64 producer relation is the next transport-only I0, while physical ValueId/operation/edge effects remain closed |
| 25b-k-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-PRODUCER-I0` | transport one exact source-backed CompareI64 producer relation for the root predicate | landed 2026-08-17; source/operation row, owner, block, operand/result/class drift and non-Compare negatives are green; no ValueId issuance, Compare emission, `emit_branch`, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-l | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-D0` | parent boundary for same-session operand receipts, stamp retention, and the canonical physical result | parent remains blocked; operand inventory and stamp retention are landed, while the result BoxShape below must name the single plan/receipt owner before any physical effect |
| 25b-l-a | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-OPERAND-PRODUCER-D0` | co-seal the source Length contract, CallSlot/result/class, matching common operation row, and Compare-right key from one S6C ingress | accepted BoxShape 2026-08-17; fixed two-row physical-ID-free inventory only; no physical receipt, ValueId, call lowering, Compare, branch, CFG/PHI, fallback, retry, or production |
| 25b-l-a-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-OPERAND-INVENTORY-I0` | transport the typed Left ReadBinding/Right Length CallSlot inventory with wrong-role/op/result/block/class, duplicate, foreign, provenance, and late-failure negatives | landed 2026-08-17; three focused inventory tests are green; no physical emission, Builder/session mutation, or parent-result unlock |
| 25b-l-b | `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-D0` | retain the existing physical-entry cohort stamp through the consuming canonical session without copy/reconstruction, then expose only a scoped borrow | accepted BoxShape 2026-08-17; caller-zero I0 landed 2026-08-17 with move-only session ownership and scoped borrow; no physical condition result, ValueId, edge, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-c | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-BOXSHAPE-D0` | fix one session-local Bool result plan/receipt that borrows the producer/inventory/stamp, uses canonical ValueId/type issuance, and has one outer discard owner and one later branch consumer | active NoSafeSlice design stop after the Length canary landed; no ValueId, Compare, Length-call materialization, edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-d | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-D0` | name the sole same-session issuer for the Length CallSlot physical result required by the parent Bool receipt | accepted BoxShape 2026-08-17; the first consumer is a no-effect one-shot canary, with no ValueId, CallSlot lowering, Compare, edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-d-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-I0` | consume the same-cohort Length relation/inventory/stamp exactly once as a Builder-neutral canary | landed 2026-08-17; positive, duplicate, missing-stamp, source-shape, and late-failure no-mutation gates are green; no physical Length result, CallSlot lowering, Compare, edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-e-D0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-D0` | accept one source-backed StringLen target/receiver/zero-args/I64 plan before any canonical Call effect | accepted BoxShape 2026-08-17; the next I0 issues the plan once with no ValueId, Call, Compare, edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-e-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-I0` | issue and consume the source-backed target plan exactly once in the existing callback | landed 2026-08-17; same-cohort facts, canonical StringBox.length, plan/canary parity, duplicate, missing-stamp, and late-discard gates are green; no canonical Call/result receipt or parent Bool effect |
| 25b-l-f-D0 | `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-D0` | project the allocated source-segment receipt to the exact physical condition block through the same canonical session | accepted BoxShape 2026-08-17; callback-scoped owner/logical-block/physical-block/stamp view only; no Call, ValueId, Compare, edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-f-I0 | `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-I0` | allocate source segments once and lend exactly one same-session condition-block target with late-discard and escape negatives | next fast slice after the BoxShape; no Length Call/result receipt, receiver ValueId, Compare, edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-e | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-PHYSICAL-RESULT-D0` | first issue the source-backed StringLen target-realization plan, then name the sole same-session canonical Length Call/result issuer and consumer before the parent Bool result | active NoSafeSlice design stop; the existing canary is protocol-only, target/receiver/args/result realization is not yet sealed, and no physical Length result, Bool receipt, CallSlot lowering, Compare, edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production is open |
| 26 | `LOOP-PRECUTOVER-AUTHORITY-G0` | all-19 semantic-program/JoinSig/Layout/CFG coverage plus zero competing target-subtree authorities | caller-zero gate; missing coverage blocks selection |
| 27 | `LOOP-PRODUCTION-SELECTION-D0` | decide exact family admission after all required gates | human consultation stop; `NoCandidate` is valid |
| 28 | existing `M10b-I0-R0` + R1/M11/M12/R2 | one production switch, same-commit old-edge deletion, direct Ready-constructor retirement, then manifest-led sole-authority proof | no fallback; cutover must be green before retirement |

### Selected Dynamic first-cutover overlay (2026-08-11)

The table above remains the repository-wide convergence order. The bounded
`ParserScanLoopBox.skip_while/4` replacement uses this smaller exact overlay so
the first production cutover does not wait for unrelated all-19 families:

```text
A-PRIME-PARAMETER-CONTRACT-I0
  -> exact pos/end source/binding/ABI relation
  -> A-PRIME-MIXED-RECIPE-SEMANTIC-RECUT-I0
  -> A-PRIME-PHYSICAL-INPUT-I0

LOOP-UNIFICATION-AFTER-DYNAMIC-D0 (BoxShape only)
  -> LOOP-SEMANTIC-PROGRAM-COSEAL-R0
  -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
  -> LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
  -> LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
  -> LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0

selected BoxCount only
  -> LOOP-PHYSICAL-IF-COVERAGE-I0
  -> LOOP-PHYSICAL-EXIT-COVERAGE-I0
  -> LOOP-PRECUTOVER-AUTHORITY-H2

backend siblings
  -> AOT/LLVM production exact-I64 capability
  -> LLVM-SELECTED-DYNAMIC-EXACT-I64-DIRECT-I0
  (Rust VM exact-I64 row is reference/smoke evidence only; it is not a
   production gate, provider consumer, or session prerequisite.)

outer callable owner
  -> FUNCTION-COMPLETION-SITE-KEYED-CLAIMS-R0
  -> DRAFT-SEAL-EXIT-PROJECTION-SPLIT-R0
  -> A-PRIME-I64-LOOP-PHYSICAL-SESSION-I0
  -> H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0

post-cutover
  -> LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0
```

`LOOP-UNIFICATION-AFTER-DYNAMIC-D0` never absorbs If/Exit BoxCount. `Always`,
unrelated G0 retirement, and broader all-family parity remain later unless the
unchanged selected source proves they are required. The topology D0 is census
and deletion-gate preparation only; fixed-role hard deletion occurs after the
selected old production edge is removed and remaining callers reach zero.

### Pre-cutover execution briefs

`LOOP-SEMANTIC-PROGRAM-COSEAL-R0`

```text
Change:
  issue one move-only semantic program from existing source/Core authorities;
  migrate caller-zero Callable/G0/all-route logical products; delete split issue
Contract:
  exact resolver site/frame and Core-owned JoinSig are co-branded once;
  profile input owners, Tail, ABI, Completion, and physical owners stay outside
Done:
  mixed-Core/context/continuation and wrong-node/source fixtures reject;
  raw from_parts/from_after and three-argument demand callers are zero
Stop:
  any need to copy input truth, infer source coordinates, or add a selector
  returns to design
```

`LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0`

```text
Change:
  share one private structural traversal; issue current-cohort transfers from
  JoinSig; bind them in Layout; delete Recipe-derived transfer inference
Contract:
  Recipe owns structure, JoinSig owns logical transfers, Layout owns placement,
  Canonical CFG owns physical edges; accepted shapes remain unchanged
Done:
  Callable/G0 layouts and MIR receipts retain parity; missing/duplicate/foreign/
  wrong-target transfer fixtures reject; direct Layout/allocator/writer inference
  callers are zero
Stop:
  If/Exit/Always support, profile-specific repair, or a public traversal Plan
  is a different row and cannot enter this Refactor Series
```

The same behavior-preserving series may include the ledger-bound consumer
cleanup `LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0`: V1 and V2 consumers must
borrow one complete ordered operation/source-effect ledger instead of calling
`find` over Recipe/evidence arrays repeatedly. This is a consumer protocol, not
a V2-to-V1 adapter or a new source/effect authority. If the ledger cannot be
borrowed without re-pairing rows, stop with `NoSafeSlice` and keep the current
physical demand owner unchanged.

`LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0` is the next BoxShape slice in the same
series. The common Loop physicalizer may consume only the neutral continuation
boundary and complete physical layout/ledger products. It must not import or
construct `ReadyCallableLoopProfileCloseV1`, inspect Callable-specific counts
such as `Pure/Read/Write`, or own Tail, ABI, Completion, Return, DraftSeal, or
callable symbols. `recursive_after.rs` stops at
`ReadyLoopAfterContinuationV1`; the callable owner consumes that receipt in a
separate adapter. A guard must prove zero Callable profile symbols and zero
hard-coded profile cardinalities in the common physicalizer. Moving a file is
not sufficient: the owner and import boundary must change together.

`LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0` is a census gate, not an eager
deletion. It inventories production, test, and guard callers of the old
fixed-role receipts (`LoopPhysicalBlockReceiptV1` / role-indexed boundary) and
the newer segment receipts (`LoopPhysicalSegmentBlockReceiptV1`). The old path
is removable only after the segment path is the sole production route and its
remaining test callers are either migrated or explicitly allowlisted. Numeric
role, current-block, name, ordinal, or Recipe-order repair is never an
acceptable bridge. If the census cannot prove caller-zero ownership, leave the
old type in place and return `NoSafeSlice`.

### Post-Dynamic audit additions (2026-08-11)

The external review did not add a fifth Loop authority. It makes the existing
four-row BoxShape series mechanically checkable. The following file-level
responsibilities are part of the rows above, not independent execution cards:

| Row | Existing surface | Required final owner | Forbidden bridge |
| --- | --- | --- | --- |
| `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` | `physical_layout.rs`, `recursive_after.rs` | JoinSig-issued transfer view bound to private Recipe placement | rebuilding Predicate/Jump/Backedge/nested resume from `LoopConditionV1` or `as_recipe()` |
| `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` | `segment_allocator.rs` | verified segment-placement receipt | rereading Recipe condition roles to classify Header/Body, current-block repair |
| `LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0` | V1/V2 physical-demand consumers | one complete ordered operation/source-effect ledger borrowed by the consumer | per-access `find` over Recipe/evidence/effect arrays, zip-by-order, V2-to-V1 adapter |
| `LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0` | `recursive_after.rs`, `tail_completion.rs` | common stop at `ReadyLoopAfterContinuationV1`; Callable adapter owns profile close/Tail/ABI/Completion | `ReadyCallableLoopProfileCloseV1`, callable symbols, or hard-coded `Pure/Read/Write` counts in common code |
| `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0` | `operation_target.rs` and fixed-role/segment receipts | one proven segment production route, then caller-zero deletion | keeping old and new topology issuers live without a census, or repairing by role/name/ordinal |

The `tail_completion.rs` file location is itself part of the boundary audit:
moving a file is insufficient if the common physicalizer still imports or
constructs Callable profile products. The final common module may stop at the
neutral continuation receipt; the outer Callable owner consumes it and owns
Tail, ABI, Completion, Return, DraftSeal, and Callable symbols.

The ledger row is a consumer-protocol refactor, not a new semantic authority.
Each family may retain its own verified source/effect product and lend one
complete ordered view. The view must be complete before physical preparation,
must retain exact item/source/placement identity, and must make missing,
duplicate, foreign, or extra rows reject before Builder effects. If this cannot
be done without re-pairing rows, the row returns to design with `NoSafeSlice`.

The topology census must include both the old role-indexed entry points and the
new segment entry points, including the dual `operation_target.rs` issuers and
their tests/guards. Deletion is allowed only after the segment route is the
sole production route and all remaining tests are migrated or explicitly
allowlisted. This keeps retirement reversible and prevents a second topology
authority from surviving behind a compatibility wrapper.

These are structural acceptance rules only. They do not authorize a new Loop
shape, a production selector, a Builder/CFG change, a fallback/retry path, or
the current H2 parser execution lane.

These three rows are the canonical post-Dynamic unification series. They are
ordered as one BoxShape-only refactor boundary:

```text
LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
  -> LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
  -> LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
  -> LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0
```

`LOOP-PHYSICALIZER-COMMON-OWNER-R0` in the portable-Recipe SSOT is a related
but separate historical Accum-owner split. It may not absorb these rows or
become a second authority for the Dynamic/Common physicalizer boundary. The
post-Dynamic series owns transfer/evidence consumption and the Callable
profile boundary; the older row owns only the behavior-neutral Accum service
split. If an implementation touches both owners, keep the changes in separate
refactor-series commits with independent guards.

The three structural-coverage I0 rows each use the same four-block contract:

```text
Change:
  add exactly one previously typed-unsupported structural family
Contract:
  Recipe + JoinSig + common physicalizer only; no new route or fallback
Done:
  one positive fixture, exact transfer/coverage negatives, common guards, and
  implementation-coupled README/reference update are green
Stop:
  a missing JoinSig vocabulary returns to design before Layout or CFG edits
```

### Closed implementation receipt: `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`

```text
Change:
  add one consuming finish_for_draft_seal target to the V2 session;
  migrate existing V2 profile finish sequences through it;
  add no non-V2 ReadyFunctionDraftSealV1::new caller

Contract:
  profile-specific ledgers close into ReadyCanonicalProfileCloseV1;
  common CFG / semantics / If / identity / PhiTxn / resolved binding /
  Completion close exactly once; whole unpublished session remains the
  failure atomicity owner

Done:
  DirectAccum cannot reach DraftSeal without cfg.finish;
  V2 direct Ready constructor callers are zero; the one non-V2 compatibility
  caller is named and non-growing; profile close is move-only and completion
  body/site/target metadata is not re-inferred at finish; focused
  omission/order/receipt/identity/failure-discard/fresh-reuse tests and the
  existing canonical gates are green; loop/function-exit references and the
  owning README update in the same commit

Stop:
  any accepted-profile or MIR delta, new semantic owner, non-V2 migration,
  or same-session repair/retry returns to design
```

### Closed design correction receipt: `LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0`

The existing prepare architecture is directionally accepted but has one
bounded BoxShape correction before implementation. The correction task fixes
the callable input brand, resolved Prelude target/result capability, one-shot
Tail/Completion/ABI compatibility receipt, G0 owner/ABI pairing, and the
borrowed `ResolvedFunctionLoweringInputV1` lifetime wording. It adds no code or
Builder authority.

The correction is accepted only when these facts are explicit:

```text
callable input = non-Clone brand over exact input + current header/index
Prelude        = resolved target/header/arity/result capability, not syntax shape
terminal       = one-shot Tail/Completion/ABI relation receipt
G0             = same owner/source-type/ABI/terminal relation check
lifetime       = owned AST-free demand/receipts separate from borrowed input
```

Missing/foreign header, target, arity, result ABI, owner, tail site/binding,
Completion site/value-kind, G0 source brand, duplicate receipt, or any physical
authority is a pre-effect typed `NoSafeSlice`. The detailed task and its
acceptance matrix were the correction checklist; that row is closed and the
current execution row is the caller-zero recursive physicalizer below.

The static-call fixture and Prelude argument receipt close the remaining
positive prepared-input prerequisites without opening a production caller.

### Closed implementation receipt: `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0`

```text
Change:
  add one test-only caller-zero common recursive topology boundary that
  consumes the topology-only compatibility VerifiedLoopPhysicalDemandV1
  exactly once together with the
  private, single-use ReadyLoopEntryV1 receipt and opens one Loop After
  continuation without emitting operation MIR.

Contract:
  the physicalizer sees only the AST-free demand, ReadyLoopEntryV1, and
  borrowed CanonicalSsaFunctionSessionV2 services. It does not see callable
  Tail/ABI/Completion, profile names, legacy route labels, source AST/name
  lookup, or a second Recipe/CFG/SSA/PHI owner. Late failure discards the
  unpublished fresh session; retry and same-session repair are forbidden.

Done:
  the focused canary proves recursive child/root After topology, exact entry
  coverage, owner/binding checks, parent/preheader placement, and rejection
  before block allocation. The module is cfg(test), has no production caller,
  and keeps source/check files below 800 lines. Exact MIR references, the
  owning README, and the compact current-row receipt were updated together.

Stop:
  operation emission without the accepted operation physicalizer design and
  canary task, missing logical relation,
  copied/reverified source truth, a new Recipe kind, profile-specific
  physicalizer, public topology, Return/DraftSeal/publication, selector,
  fallback, retry, or legacy deletion returns to design.
```

The topology probe may allocate only the common logical child/header/body/
step/After structure and the existing session-local continuation receipt. It
does not claim that `ReadBinding`, `WriteBinding`, constants, comparisons, or
arithmetic have been physically emitted. Those operations need an AST-free,
item-keyed source/effect projection so repeated ordinals in nested loops
cannot be guessed or matched by name.

### Operation/effect design boundary

The operation/effect relation product and both profile adapters are now
closed caller-zero cells, and cross-profile parity is closed as a diagnostic
receipt. They remain one relation product, not a new operation owner:

```text
Recipe:
  sole logical owner of LoopItemKey -> LoopOperationV1 and operand values

profile source adapter:
  sole issuer of exact source anchor / BindingRef evidence

VerifiedLoopOperationEffectProductV1:
  move-only { co-sealed Core, item-keyed source evidence ledger }
  evidence row = item + exact anchor + optional Core BindingRef view
               + checked block/loop provenance
  no copied LoopOperationV1, no ordinal lookup, no second Recipe
```

The existing `VerifiedLoopBindingEffectRelationV1` remains a separate
binding-level read/write/carrier product. The callable test producer's
item/site/operation relation is evidence for the adapter, not the common
authority. Generic G0 must retain or issue its item-keyed source evidence at
the producer boundary before structural source facts are consumed; it may
not reconstruct anchors from source preorder after Core issuance.

The operation product joins against the Core's already-sealed effect rows. If
the current Core API lacks the anchor/class view needed for that join, the
implementation may add one non-authority accessor or a consuming join helper
at the Core boundary. A second effect catalog or copied effect rows are not
allowed.

Coverage is by Recipe operation item, not by every Core effect row. Each
`LoopRecipeItemV1::Operation` has one exact source-evidence row;
`ReadBinding`/`WriteBinding` rows may additionally reference their sealed Core
effect row, while literal/compare/binary rows need no binding-effect row.
Most structural carrier rows and callable Tail/After reads remain with their
existing owners. The nested Generic G0 item 3 is the explicit exception: its
`ReadBinding` operation uses the existing child-entry
`DerivedCarrierEntry` anchor for carrier 2, and the Core effect relation must
match that anchor exactly. Item 4, C0/C1 carriers, and Generic tail reads stay
outside the operation product.

### Operation physicalizer design closeout

Decision B is accepted: full-demand preparation and one-operation emission are
different proofs. The full demand bundles the complete operation/effect product
with one neutral continuation and exposes only `prepare_all`; the private leaf
emitter consumes `PreparedLoopOperationEmissionV1` and never sees continuation
or any profile/function terminal contract.

The full semantic preflight runs before Builder mutation. After topology
allocation, the Callable R2 adapter derives an owner-branded
`LoopPhysicalSegmentBlockReceiptV1` from the R1 layout and binds each exact
segment to one physical block before instruction emission. The leaf emitter may
borrow only the existing canonical CFG, BindingSSA, and PhiTxn services plus a
session-local `ReadyLoopEntryV1`. It creates no second CFG, SSA, PHI,
transaction, or retry owner. A post-emission failure poisons the unpublished
function and uses whole-session discard; local Phi rollback is diagnostic
cleanup only. The older logical block receipt remains only for pre-existing
test seams and is not a fallback for the selected Callable dispatcher.

Generic item 3 remains a normal parent-body `ReadBinding`, but its source
anchor is the child-entry `DerivedCarrierEntry` for carrier 2. It is **not
admitted by ReadBinding D0**: the row is rejected as
`CarrierSeedUnavailable` and belongs to a later carrier-seed row. That later
row must assert parent-block placement and issue a child-entry carrier-seed
receipt through canonical BindingSSA; it must never relabel the operation or
infer placement from the anchor. The bounded leaf canaries are ConstI64 and
ReadBinding only; they do not constitute full Loop physicalization.

Duplicate item keys, foreign or missing anchors, wrong block/loop membership,
and repeated-ordinal ambiguity are typed `NoSafeSlice`. No operation MIR is
opened by these passive rows. Each profile product must be issued before the
P0 topology-only `into_physical_boundary` path, which intentionally drops
source anchors; P0 cannot be reused as the operation source.

`LOOP-PHYSICAL-PREPARE-P0`, the static-call fixture/profile, and
`LOOP-PRELUDE-ARGUMENT-RECEIPT-P0` are closed caller-zero prerequisites. The
cross-profile parity receipt and reviewed Decision-B closeout are closed.
Callable has seven item rows and Generic G0 has fifteen, but parity compares
neither counts nor source order. The full-demand P0, behavior-neutral module
split, canonical physical block receipt, private ConstI64 leaf-emitter canary,
bounded ReadBinding I0, and the caller-zero full callable physical canary are
closed. G0 D0/I0 are accepted/closed. R1 is now closed with Builder effect
zero; `LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` and
`LOOP-COMMON-RECURSIVE-AFTER-R3-I0` are now closed by the Callable segment and
neutral After canaries. G0 physical parity,
production selection, M8/M9 coverage, and retirement remain separate gates.

### ReadBinding leaf D0 correction (2026-08-07; Decision: accepted and landed)

The broad B boundary remains accepted. Worker review closed the following
contracts, and the bounded ReadBinding I0 implementation landed with focused
tests. These constraints remain normative for the leaf:

- Project the row exactly once from a complete
  `PreparedLoopOperationProgramV1`. Its Recipe `binding`/`result`, verified
  effect row (`source_binding`, `anchor`, `role`), owner, and logical
  placement must agree. AST, name, ordinal, and ad-hoc full-demand
  re-extraction are forbidden.
- The ordinary expression-read leaf admits only
  `LoopBindingEffectAnchorV1::Expr`. `DerivedCarrierEntry` (including Generic
  G0 item 3) belongs to the separate common carrier-seed projection closed by
  `LOOP-COMMON-PREDICATE-CARRIER-I0-R0`.
- The raw `ValueId` from `ResolvedSsaIdentityStateV2::read_entry` must not
  become the public leaf receipt directly. A thin canonical seam must issue
  `CanonicalBindingReadReceiptV1 { owner, binding, physical_block,
  physical_value }` after canonical BindingSSA/PHI verification.
- Placement comes only from the sole `LoopPhysicalBlockReceiptV1` and the
  orchestrator's logical Loop/Block/role. `current_block` and ordinal
  inference are not authorities. All checks happen before the canonical
  read/PHI operation.
- The logical result key is alias publication only. The leaf returns one
  immutable receipt `{ owner, item, binding, result, block, value }`; the
  outer operation ledger owns publication. No second ValueId, BindingSSA map,
  PHI owner, Return, Completion, or DraftSeal is introduced.
- Identity and `PhiTxn` are borrowed through one explicit canonical read
  service bundle. The physicalizer does not become a second session or owner.
- Pre-effect rejects are typed `NoSafeSlice`. A post-read type/receipt
  mismatch is a late terminal: discard the whole unpublished function,
  retain only local Phi cleanup diagnostics, and never retry or fallback.

The required reject matrix is: operation-not-ReadBinding; missing or
mismatched expression source anchor/binding; Core effect/role mismatch;
owner, logical, or physical placement mismatch;
missing entry binding; canonical BindingRead failure; result-type mismatch;
terminated block; and late emission failure.

This D0/I0 boundary claims no full-demand extraction API, AST reread, second
CFG/SSA/PHI/catalog owner, derived/G0 carrier bridge, other operation kinds,
return/seal/module publication, selector, retry/fallback, legacy retirement,
or performance result. The bounded implementation is landed. The current
authorized row is `CALLABLE-LOOP-AFTER-CLOSURE-P0`; Tail-only lowering is a
NoSafeSlice until its sealed After receipt exists. Each subsequent Tail and
DraftSeal slice must update reference documentation in the same commit as
code and focused tests.

#### ReadBinding source/effect mapping matrix

The following table is the complete D0 mapping. The full prepared program is
the only projection input; a one-row test fixture is allowed because the
complete program itself contains one ReadBinding row, not because a single
operation is extracted from a demand.

| Recipe operation | Evidence item | Core effect / anchor | D0 admission | Canonical read | Result publication owner |
| --- | --- | --- | --- | --- | --- |
| `ReadBinding { binding: LoopBindingKeyV1, result: LoopValueKeyV1 }` | same `LoopItemKeyV1` | `SourceRead { ordinal }`, `source_binding: BindingRefV1`, `Expr(OwnedExprSiteV1)` | admit only when all keys, owner, block, and role match | claim exact `SourceExprSiteV1` for `BindingRefV1`, then issue `CanonicalBindingReadReceiptV1` | outer operation ledger maps `LoopValueKeyV1` to the immutable leaf receipt |
| same operation | same item | `DerivedCarrierEntry` anchor | ordinary expression leaf excludes it; common carrier-seed row owns it | canonical `read_entry_receipt` | outer operation ledger maps the logical result |
| any non-`ReadBinding` operation | same item | any effect row | reject `OperationNotReadBinding` | none | none |

The canonical receipt has exact field types and one issuer:

```text
CanonicalBindingReadReceiptV1 {
  owner: FunctionOwnerIdV1,
  binding: BindingRefV1,
  physical_block: BasicBlockId,
  physical_value: ValueId,
}
```

Only `CanonicalSsaFunctionSessionV2`'s borrowed read service may issue it.
The order is fixed: validate the prepared row and physical placement; claim
the exact `SourceExprSiteV1` with `claim_variable_use_binding`; call the
canonical `read_entry_receipt`; validate owner, block, and physical type;
then return the receipt. A raw `ValueId` from `read_entry` is never a leaf
receipt and cannot be fabricated or rewrapped by the physicalizer.

Before the loop block is sealed, canonical BindingSSA may return a provisional
PHI with `MirType::Unknown`. The verified Recipe class is the only permitted
publication evidence in that state: `Unknown -> exact class MirType` is
published once by the private operation-type owner, while a concrete conflict
or missing type rejects as `ResultTypeMismatch`. Block/identity sealing then
revalidates the now-concrete PHI inputs; this is not type inference or a
fallback route.

The leaf receipt uses distinct logical and physical names:

```text
ReadBindingEmissionReceiptV1 {
  owner: FunctionOwnerIdV1,
  item: LoopItemKeyV1,
  binding: BindingRefV1,
  result: LoopValueKeyV1,
  logical_block: LoopBlockKeyV1,
  physical_block: BasicBlockId,
  physical_value: ValueId,
}
```

`result` is an alias key only. The Recipe's `LoopValueClassV1` for `result`
and the binding/effect class are the logical type authority; the canonical
BindingSSA type fact is the physical observation and must match the class-to-
`MirType` mapping before the receipt is returned. The outer operation ledger,
not the leaf, publishes the result mapping. No second SSA/PHI/value map is
created.

#### Entry, placement, service, and failure contracts

`ReadyLoopEntryV1` is a **preheader seed receipt**, not a complete map of all
live bindings. The private ReadBinding projection carries an explicit
`entry_requirement: LoopReadEntryRequirementV1` with exactly two cases:
`PreheaderSeed` or `CanonicalLive`. The full-program orchestrator issues this
field from the existing Recipe input set and source-binding relations, then
checks `PreheaderSeed` against `ReadyLoopEntryV1`; the leaf never infers the
case. Body/step bindings use canonical SSA availability at their exact
physical block; absence from the preheader rows is not itself an error. A
required preheader seed missing from `ReadyLoopEntryV1` is the typed
pre-effect reject `EntryBindingMissing`.

The orchestrator supplies `expected_role: LoopPhysicalBlockRoleV1` together
with `LoopBlockKeyV1`; the sole placement authority is
`LoopPhysicalBlockReceiptV1`. The leaf never derives a role from
`current_block`, an ordinal, or source-anchor shape. The logical block and
expected role must resolve to the same `BasicBlockId` before claim/read.

The canonical borrowed bundle is the only physicalizer service boundary:

```text
CanonicalBindingReadServicesV1<'a> {
  builder: &'a mut MirBuilder,
  identity: &'a mut ResolvedSsaIdentityStateV2,
  phis: &'a mut PhiTxn,
}
```

It is created by the fresh canonical function session, borrowed for one
read, and never stores a second CFG/SSA/PHI owner. CFG placement is borrowed
separately from the sole `LoopPhysicalBlockReceiptV1`; simultaneous borrows
are sequenced so the receipt is fully validated before the canonical read.
There is no new type-fact owner or `TypeFactContext`: physical type
validation reads the existing `TypeContext` at
`MirBuilder::function_state.type_ctx`, and any publish/idempotence decision
uses the existing `TypeFactDecisionV1`/
`PreparedTypeFactPublicationV1` seam. The service bundle therefore borrows
only the canonical Builder/identity/Phi owners named above.

Phase ownership is fixed as follows:

| Phase | Allowed failure | Owner/action |
| --- | --- | --- |
| prepared-row/source/effect/entry/placement validation before claim | typed `NoSafeSlice` | no Builder/claim/PHI effect |
| canonical validation before the atomic claim/read service starts | typed `NoSafeSlice` | no claim/PHI effect |
| claim succeeds or canonical read starts, then any read/type/receipt error | terminal `Freeze` | whole unpublished function discard; caller restore once; PhiTxn abort is diagnostic only |
| post-read type/receipt/result mismatch or injected late failure | terminal `Freeze` | whole unpublished function discard; caller restore once; no retry/fallback |

This phase split is part of D0 acceptance and must be represented by focused
negative tests in the later implementation row.

## Implementation and documentation obligation

Every implementation row above must update its exact live references in the
same commit after code and focused tests land:

- `docs/reference/mir/loop-recipe-contract.md` for the landed co-seal/demand/
  physical boundary and sole-owner claims;
- `docs/reference/mir/generic-loop-stage-matrix.md` for caller-zero,
  canary, activation, and retirement status;
- `docs/reference/language/function-exit-and-entry-result.md` when the typed
  function-finish/Completion handoff lands;
- `src/mir/loop_recipe_contract/README.md` and the owning canonical lowering
  README (`src/mir/builder/resolved_lowering/README.md`) when their code
  contract changes;
- `docs/reference/mir/phi_policy.md` and `phi_invariants.md` only when a
  physical PHI contract actually changes;
- `CURRENT_STATE.toml` and the active rolling workstream for the next exact
  row and compact closeout;
- `docs/tools/check-scripts-index.md` only if a reusable public guard entry is
  added.

References must describe only landed behavior. This design SSOT may name the
accepted target now, but the reference pages must not claim physical,
production, backend, or retirement capability before the corresponding
implementation receipt exists.

For the new pre-cutover rows, the co-seal and transfer-authority implementation
commits each update the reference with their exact caller-zero status; each
Always/If/Exit BoxCount commit updates the supported structural matrix; M10b
then updates the same reference once more to record the real production caller
and removed legacy authorities. A design-only commit does not pre-announce any
of those capabilities in `docs/reference/**`.

## Current execution boundary

The architecture, `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`, bounded
`RECIPE-COSEAL-I0-R0`, callable static-prefix prepare, and Prelude argument
receipt are closed under the typed-receipt and no-reinference contract above.
The topology/After canary `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0` is closed.
The operation/effect plan, passive product, Callable adapter, Generic G0
15-row anchor ledger, cross-profile parity receipt, worker-reviewed
physicalizer Decision-B closeout, and Builder-free full-demand P0 are closed.
The `LOOP-RECIPE-OPERATION-EMITTER-CONST-S0` boundary is now closed as a
private prepared ConstI64 leaf canary. It proves exact physical placement,
canonical Const/type-fact emission, typed pre-emission rejects, and
whole-session discard/fresh-session repeat. Full operation emission,
operation production activation, callable physical completion, production
selection, retry/fallback retirement, and legacy deletion remain closed. The
logical callable issuer S0 is closed without a production caller. R1 and R2 are
now closed by the receipts below; the next row is bounded neutral recursive
After R3. No single-item extraction API may be added to the full demand.

### Callable physical-canary preparation slice (2026-08-07)

The current preparation slice is mechanically green without claiming the full
callable physicalizer. The Prepared callable product has one private test
handoff that moves `input`, complete operation demand, Prelude, Tail, terminal
compatibility, and Completion exactly once. The full operation contract also
projects every WriteBinding row with its exact Recipe item, source
binding/site, class, and logical placement.

Private leaf bridges cover `ConstI64`, `BinaryI64`, and `CompareI64` through
the existing Builder/type emitters. Their schedule-local value map is only a
temporary `LoopValueKey -> ValueId` transport; it is not a second SSA or PHI
owner. A focused test proves the Const -> Binary -> Compare chain. A bounded
row-level dispatcher and full Recipe-order Builder-free prepare now join
Read/Const/Compare/Binary/Write leaf services with an opaque typed value
ledger. The physical operation boundary now issues one exact
logical-to-physical target receipt per row, validates all target blocks before
the first leaf effect, and separates target/pre-claim physical failure from
semantic preflight. The caller-zero full physical canary is now closed:
the exact resolved-module input/ledger enters S2 once, then reaches Prelude,
topology, all five operation families, sealed After, Tail/Completion,
`finish_for_draft_seal`, and DraftSeal prepare/commit. Its late-failure test
discards the whole unpublished function and reruns the same semantic fixture
in a fresh session. Production selection, Generic G0 parity, retry/fallback
retirement, module publication, and legacy deletion stay closed.

### Callable full physical canary closeout (2026-08-08)

`CALLABLE-LOOP-PHYSICAL-CANARY-P0` is a caller-zero-only integration receipt.
The test-only source bridge borrows the exact existing resolver ledger from
`ResolvedFunctionLoweringInputV1`; it does not resolve a second owner or clone
the source AST. The complete seven-row Recipe schedule is consumed once and
the existing owners remain sole authorities for CFG/SSA/PHI, completion,
DraftSeal, and unpublished-function discard. The focused positive and late
duplicate/discard/fresh-reuse tests are green. G0 D0 is accepted; the next
authorized row is the Builder-free
`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` exact-input composite gate.

### Generic G0 exact-ingress I0 closeout (2026-08-08)

`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` now has a compiler-side `cfg(test)` ingress
at `src/mir/compiler/generic_g0_physical_prepare.rs`. It pairs the exact
resolver-issued `ResolvedFunctionLoweringInputV1` with the existing neutral
S4 product, validates source/owner/frame/forest/entry/tail relations, splits
`VerifiedGenericAfterEffectG0` once into the neutral continuation and the
distinct `VerifiedGenericG0TailCapabilityV1`, then issues the common demand
and `prepare_all` for all fifteen G0 Recipe items. The schedule is checked by
Recipe membership rather than Callable/G0 count or evidence order. Focused
positive, missing-input, foreign-input, and tail-separation tests are green;
existing demand/producer tests retain duplicate/missing-evidence coverage.
This remains Builder/MIR/physicalizer/selector/Retry/publication-free; later
negative expansion must use typed sealed-product rejection, not tampering or
reconstruction.

### Recursive segment plan R1 closeout (2026-08-08)

`LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` is closed as a Builder-free derived
product. `VerifiedLoopOperationPhysicalDemandV1::prepare_all` now traverses
the verified recursive Recipe preorder instead of flattening logical blocks.
`PreparedLoopPhysicalLayoutV1` consumes the complete prepared program and
records only mechanically derived segments, operation placement, and nested
After-to-parent-resume targets. It creates no `ValueId`, `BasicBlockId`, CFG,
SSA, PHI, function session, selector, retry, fallback, or legacy authority.

The exact fixtures are green:

```text
Callable: seven operation rows in Recipe preorder
Generic G0: [0,1,2,3,5,6,7,8,9,10,11,12,13,14,15]
Generic G0 segments: root B0, root B1-pre, child B2, child B3, root B1-resume
coverage: 16 items / 15 operations / 5 segments
```

R2 may bind these private segments to the already allocated old topology. Until
the R3 correction closes, true segment block allocation, recursive After emission, G0
physical parity, production selection, retry/fallback retirement, and legacy
deletion remain closed. The R2 task is
`investigations/loop-common-segment-block-cutover-r2-task-2026-08-08.md`.

### Segment block cutover R2 closeout (2026-08-08)

`LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` is closed for the Callable canary.
`LoopPhysicalSegmentBlockReceiptV1` is a private adapter receipt derived from
the closed R1 layout and the already allocated canonical topology. It verifies
exact segment coverage, owner/preheader branding, and unique physical blocks.
The selected Callable dispatcher builds one complete item-to-segment index from
that layout and issues each target through the exact segment key; it no longer
uses logical-block-only execution lookup. The existing canonical CFG,
BindingSSA, and PhiTxn services remain the only physical owners.

The R2 receipt is intentionally only a Callable adapter: segments that would alias
one current topology block reject rather than silently sharing a block. This
keeps Generic G0's parent pre-child/resume split closed until R3 supplies the
neutral recursive After/edge physicalization. The focused canary preserves the
seven-row `Pure=4 + Read=2 + Write=1` parity and covers exact placement,
foreign-owner, missing-segment, duplicate-block, late-failure discard, and
fresh-session reuse. No G0 physical emission, selector, fallback, retry,
collector/publication, or legacy retirement is claimed.

The implementation receipt is recorded below; R3-I0 is closed. The next task
is the bounded D1 common Predicate/carrier contract row, not production
selection.

### R3-I0 implementation receipt (2026-08-08; Decision: accepted)

R2 is an adapter over the old fixed topology, not the physical allocator for
the R1 segment graph. A neutral edge writer cannot consume it safely because
R1 transfers do not use the synthetic Step block. The corrected physical
boundary is implemented for the selected Callable caller-zero canary:

```text
PreparedLoopPhysicalLayoutV1 + ReadyLoopEntryV1
  -> one block per R1 segment + one root After (no Step)
  -> CompletedLoopSegmentProgramV1 retains layout, segment receipt,
     and completed operation receipts
  -> preflight entry plus every R1 transfer exactly once
  -> canonical CFG/identity/PhiTxn edge emission and sealing
  -> neutral ReadyLoopAfterContinuationV1
```

The layout carries an explicit sealed `entry_segment`; ordinal zero is not an
entry authority. `segment_allocator` allocates exactly one block per R1
segment plus one root After and no synthetic Step. The completed segment
program retains layout, entry, segment receipt, completed operation receipts,
and the value ledger. R3 preflights the entry edge and every R1
Jump/Predicate/OpenNestedLoop transfer, emits each once through canonical
CFG/identity/PhiTxn, seals all segment blocks plus root After, and returns one
neutral `ReadyLoopAfterContinuationV1`. Callable's seven-row coverage stays in
its thin profile wrapper; Tail/Completion meaning is unchanged. The old fixed
Callable close helper and `from_callable_layout` adapter are removed from the
selected path. G0 receives no physical allocation or operation emission, and
selector, fallback/retry, publication, and broad legacy retirement remain
later boundaries.

### G0 I1 D1 review closeout (2026-08-08; Decision: accepted)

The post-R3 worker audit found two common contracts that must be implemented
before G0 I1. The current Callable recursive writer cannot use the first
Predicate value for every transfer: `LoopPhysicalTransferV1::Predicate` must
resolve its own completed Bool value and physical source segment. The neutral
After receipt therefore carries only common owner/root/predecessor and
coverage facts; Callable's `7 = Pure4 + Read2 + Write1` remains profile-local.

The G0 child-carrier row is a `ReadBinding` with a `DerivedCarrierEntry`
anchor, not an expression anchor. The common operation family must add a
profile-neutral prepared carrier-seed variant which delegates to canonical
identity `read_entry_receipt`. It must not fabricate an expression site or
introduce a G0-specific dispatcher/SSA owner.

The next two commits are intentionally separated:

```text
LOOP-COMMON-PREDICATE-CARRIER-I0-R0
  common contracts + Callable regression; no G0 allocation

LOOP-CALLER-ZERO-PARITY-G0-I1-R0
  exact ingress, 5 segments + root After, 15 rows, per-Predicate values,
  G0 Tail/Completion/DraftSeal, whole-session discard/fresh rerun
```

Both remain cfg(test) caller-zero evidence. Production selection, M8/M9,
M10b/M11/M12, retry/fallback retirement, collector publication, and broad
legacy deletion remain closed. Each implementation commit updates the exact
reference page, README, tests/guards, current pointers, and workstream.

### Common Predicate/carrier I0 closeout (2026-08-08; Decision: accepted)

`LOOP-COMMON-PREDICATE-CARRIER-I0-R0` is closed. The neutral After receipt no
longer carries a profile-specific condition key or operation counts. Recursive
After validates one completed Bool receipt per Predicate transfer, including
owner, type, and physical source-segment placement; Callable's coverage and
condition proof remain in its outer profile close.

The common operation demand now has a separate full-program
`PreparedLoopDerivedCarrierSeedRowV1` for `DerivedCarrierEntry` anchors. The
private `CarrierSeed` emitter delegates to canonical identity
`read_entry_receipt`, so no fake expression source site, G0-name dispatch, or
second SSA owner is introduced. The focused Callable gate is green (25/25),
the Generic demand fixture identifies exactly one item-3 carrier row, and all
touched source files remain below 800 lines. The next implementation row is
`LOOP-CALLER-ZERO-PARITY-G0-I1-R0`; physical G0, production selection,
fallback/retry retirement, publication, and legacy deletion remain closed.

The earlier matrix rows that described `CarrierSeedUnavailable` as the final
DerivedCarrier boundary are historical for this cell and are superseded by
this receipt; expression-anchor reads keep their original contract.

### Generic G0 I1 caller-zero receipt (2026-08-08; Decision: accepted)

`LOOP-CALLER-ZERO-PARITY-G0-I1-R0` is closed as a profile wrapper around the
same common physical services. The exact resolver-issued G0 ingress moves once
into the full common operation program and the separate G0 Tail. The canary
opens a fresh unpublished function session, publishes the resolver-declared
receiver and two parameters through canonical identity, allocates five R1
segments plus root After, and dispatches the fifteen prepared rows exactly
once. The structural nested Loop item remains a control/layout row rather
than a fabricated operation.

The carrier row uses the profile-neutral `CarrierSeed` emitter and canonical
`read_entry_receipt`; an unsealed PHI value is typed only through the existing
`ensure_provisional_value_class` contract. Each Predicate transfer consumes
its own completed Bool receipt, so root and child conditions have distinct
physical values and source segments. The G0 `L0.After/b1` Tail read is
canonical, exact I64 Completion is claimed once, and
`finish_for_draft_seal`/DraftSeal reaches one unpublished completed draft.

The late duplicate fixture fails after earlier emission, discards the whole
unpublished session, and a fresh session reproduces the same semantic receipt.
No G0-specific CFG/SSA/PHI owner, production selector, caller switch,
retry/fallback, collector publication, backend/performance claim, M8/M9
coverage, or M10b/M11/M12 retirement is opened. The behavior-preserving
`LOOP-INPUT-SOURCE-RELATION-SET-R0` is now closed: callable consumes the common
exact-coverage initialized-local input set and Generic parameter inputs remain
separate. S6A's caller-zero Facts/producer and typed Main C/D/U/R ingress are
landed; its exact identity/source-coherence negative closeout remains current.
After M8/M9, the semantic-program and transfer-authority rows above are the
mandatory production-selection prerequisites. Current source task order is
owned by the Loop pipeline SSOT and `CURRENT_STATE.toml`.
