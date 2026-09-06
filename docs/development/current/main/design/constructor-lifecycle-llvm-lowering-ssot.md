---
Status: Accepted design; implementation pending
Date: 2026-09-07
Scope: selected constructor lifecycle from source-owned requirements to LLVM object and linked EXE.
Related:
  - constructor-birth-new-lifecycle-ssot.md
  - mirbuilder-final-pipeline-ssot.md
  - ../../../../reference/language/lifecycle.md
---

# Constructor lifecycle LLVM lowering

Decision: D2 defines the missing producer contracts; absence of an existing issuer is not a permanent park.
Source authority + canonical issuer: existing constructor semantic issuance and ordinary-New/Home plans own semantic requirements and operation origins; existing emission/finalization binds physical values; runtime ABI plus the selected backend invocation own target layout.
Non-authority: optional metadata, names, fixture constants, instruction coordinates as source identity, generic C defaults, and another family's target capability.
Fail-fast boundary: incomplete source coverage stops before physical admission; incomplete physical coverage or target/runtime mismatch stops before LLVM/artifact; no compatibility retry.
Smallest next slice: `CONSTRUCTOR-LIFECYCLE-BIRTH-OBJECT-HANDOFF-I0`; retain
the existing source declaration object through Birth handoff and remove the
transport's name-index reconstruction. See the issuance design below.
Non-claims: new source acceptance, implementation of these contracts, executable lifecycle LLVM support, EXE30/OBJ30, or complete MirBuilder retirement.

This supersedes D1/D2 wording that treated missing existing issuers as a
permanent execution park or treated all physical placement as source meaning.
The implementation stays fenced until the required connected acceptance passes.
No second MirBuilder or new semantic `Verified*`/`Prepared*` receipt is needed.

## Responsibility and lifetime

| Boundary | Sole responsibility | Existing owner to extend |
| --- | --- | --- |
| Source -> Facts/Recipe | Exact object, receiver/formals, expression requirements, stores, Home obligations and exit origins | `instance_constructor_semantic`, `instance_construction`, `ordinary_new_coseal`, terminal/read/Home source owners |
| Recipe -> physical emission | Bind those requirements to emitted values and operations once | `ordinary_new_admission/selected`, `normal_callable_construction_state`, existing root/New completion ledger |
| Finishing -> final view | Verify complete retained bindings against final MIR and borrow them | `ordinary_new_local_commit`, normal pipeline final validation, `published_backend_view/lifecycle` |
| Final view -> LLVM | Project completed physical bindings; backend realizes their representation categories under explicit runtime/target ABI | `published_backend_view/physical_abi`, dedicated lifecycle C consumer |
| LLVM -> object/link | Use the same selected target and matching runtime library | selected host invocation, call-local TargetMachine/DataLayout session, EXE/OBJ linker owner |

Extend existing plans with the information they currently discard. Aggregate
handoff co-seals completed products and issues no new source meaning. Do not
build a second source requirement table synchronized with existing plans, or
attach a public receipt to every instruction. Any physical binding storage
belongs to the existing emission lifetime, is completed once, and moves with
the existing root/Birth handoff. Generic module clones cannot recreate it.

## Value representations

Source owners determine semantic classes from the same resolved source loan,
declaration field contracts, exact bindings, and selected operation/result
relations. Birth lane order alone is not a type contract. Preserve the class
of each admitted formal and source value before physical emission; never use
an absent annotation as an i64 default. Existing accepted argument and Birth
body relations must be reconciled together, including unused formals.

The physical owner maps those classes and runtime operation ABIs to physical
lanes. Keep i64 scalar and object handle provenance distinct even when both
use an LLVM i64. A frame reference is a synthetic ABI value, not a source Box.
Unit is an explicit no-result contract; an absent row is not Unit.

Copy propagates an already established representation. PHI records exact
incoming edges and values, then solves equality constraints using established
incoming representations, including cyclic graphs. Reserve identities before
binding backedges; reject contradictory or unseeded cycles at finalization.
This is physical consistency checking, not source type inference. Do not use
`Phi.type_hint` or optional `FunctionMetadata.value_types` as authority.
InvokeNormalResult is bound to its exact operation and usable only on Normal;
Unit operations cannot acquire a result slot.

Validate every formal, definition, use and return in the selected function
inventory after finishing. Missing/duplicate/foreign bindings, extra producers,
representation drift and uncovered siblings reject. Physical compiler
temporaries require a named emitter rule; no catch-all temporary admission.
Metadata may remain an observation for other consumers; deleting or changing
it must not silently change selected lifecycle admission or lowering meaning.

## Checked operation origins and diagnostic IDs

Source plans retain typed origins before emission:

| Operation | Origin |
| --- | --- |
| New allocation | exact owned New expression site |
| Birth field store | constructor source identity plus resolved assignment site and canonical field |
| HomeRelease | released Home binding plus the exact source exit/cutpoint obligation |
| ReclaimUnpublished | exact construction identity plus its failed-construction cutpoint |

Cleanup may have no explicit source statement; its origin is the already
issued source exit obligation. Allocation site alone cannot stand in for all
cleanup sites. Repeated physical cleanup edges may reference the same logical
origin; identity does not imply one execution. Birth propagation records no
new Fault and must not invent a diagnostic site.

One module-publication encoding owner maps the complete ordered origin set to
nonzero checked u64 IDs, retains the reverse relation, and fails on overflow.
Order uses source declaration/site/operation-role relations, never MIR block
order, names, hashes or fixture values. IDs identify origins within one
published module; cross-build stability is not promised. This encoding cannot
create or repair origins. The existing final handoff retains the dictionary
and bindings; the serializer and C consumer only project/consume them.
Record the exact cleanup-origin fields and their existing issuers before the
source-retention task starts; a missing source exit relation stays a named
design blocker, not an empty cleanup proof.

## Runtime and target ABI

FaultFrame placement belongs to the backend/runtime boundary. The initial
selected target is explicitly x86_64 GNU/Linux LP64; this decision is not
inferred from pinned-Text support. Other targets reject until separately
admitted. Semantic Facts/Recipe carry no target triple or frame byte count.

The runtime ABI owner defines one versioned layout profile, projected to its
Rust representation, C header checks and backend layout validation. Avoid
independent hand-written size tables. The selected compile invocation carries
that profile and target selection through object emission and runtime-library
selection. The selected target's LLVM layout verifies frame size, alignment
and offsets before object emission. This requires target evidence, not an
in-process LLVM API; embedding remains the post-cutover investigation below.
No host `sizeof`, target default,
environment mutation or borrowed pinned-Text capability supplies missing data.
EXE and linked-OBJ acceptance must verify the linked runtime target/revision.

Root allocates aligned storage and initializes once, executes body and cleanup,
reports a pending Fault at the final entry, disposes once even if reporting
fails, then returns under the final-entry status contract. Birth receives the
same hidden pointer and neither allocates, initializes, reports nor disposes
it. Normal=0 and Fault=1 follow the runtime ABI; InvalidContract=2 is a physical
contract failure, never a source cleanup successor. The final-entry owner must
name its process-status mapping before consumer activation; C cannot choose
an arbitrary exit value for report failure or InvalidContract.

## Finite inventory and acceptance boundary

Boundary: selected source root and exact Birth declarations -> retained
source plans -> emitted/finalized MIR -> final view -> selected C LLVM
consumer -> object -> matching runtime link -> EXE terminal.
Includes all selected formals, values, checked operations, normal/fault exits,
unconsumed products and old selected re-entry. Excludes other call families,
VM/WASM parity, generic-C concurrency migration and runtime hook registry.

Function inventory is the retained root plus retained canonical Birth keys.
The existing parser vocabulary is Const(i64/string/Unit), Add, Copy, Phi,
ObjectFieldGet, Birth Call, Invoke(NewBox/FieldSet/HomeRelease/Reclaim/Call),
InvokeNormalResult, FaultFrameEnter, ReturnFault, Branch, Jump and Return.
Parser acceptance is not execution admission. Classify every variant against
the existing selected source domain; Branch/Jump are control, not source
value classes. Branch condition and edge arguments still require completed
value bindings; a missing condition representation rejects. They need no
checked-operation diagnostic origin. Preserve accepted source forms and tests.
Unsupported source-less parser forms reject explicitly; do not remove an
accepted shape or alter the Pair source to make the inventory close.
Inventory is not yet Exhausted: source retention, physical binding, target
agreement and executed cutover remain `CutoverBlockerOpen` inside this boundary.

## Ordered tasks

These are bounded steps in this existing family, not new receipt/guard lanes.
Step 0 is landed. Each source-retention slice requires a named canonical
issuer/consumer and a closed mapping before `fast`; unrelated missing rows
remain explicit cutover blockers rather than stopping a closed slice. Follow
the current issuance design below. Full cutover still requires all in-boundary
`Partial` and `Missing` rows to be resolved.

| Order | Owner, caller and terminal | Replacement and acceptance |
| --- | --- | --- |
| 0. Owner split (BoxShape, landed) | `instance_constructor_semantic.rs`, `brand_catalog_tests.rs`, `ordinary_new_local_commit.rs`; existing imports/tests and callers | Constructor tests now live in the child test module, the brand test tail is included without changing its logical paths, and root validation lives beside local-commit emission state. The parent files are respectively 539, 715 and 746 lines; all child files are below 760. No public API, test name/body, source contract or lifecycle route changed. |
| 1. Source retention (BoxShape) | existing semantic package issuer -> construction/New/Home plans -> existing selected claim consumer | Retain formal/value requirements and typed operation origins at their existing issue points. Replace source relation loss before claim take and cleanup emission. Acceptance: unchanged Pair and existing renamed/alias/multiple-Home cases, exact unused formal and cleanup origins; foreign/missing/duplicate/source-unavailable negatives before emission. No disconnected source-only receipt. |
| 2. Physical binding and final handoff (BoxShape) | selected root/Birth emission -> existing completion ledger -> final-view admission | Replace bare value/op binding loss with completed physical relations in the existing handoff. Positive exact source-to-final-view coverage; negative finishing drift, residual sibling, mixed/unseeded PHI, Unit result, wrong Normal result and metadata-independent verdict. Preserve one-way transfer and generic lifecycle fence. |
| 3. Target/runtime agreement (BoxShape) | runtime ABI owner + selected compile invocation -> same-target lifecycle session | Replace implicit host-layout assumption with explicit target ABI agreement; no semantic target receipt. Target-compiled Rust/C checks, LLVM size/alignment/offset agreement, unsupported target/revision/runtime mismatch and absent session reject without artifact. All session resources released on failure. |
| 4. Complete direct input (BoxShape) | final-view physical ABI -> existing JSON/parser -> one parsed consumer input | Project completed representations, origin IDs/dictionary and target ABI once; remove serialize-then-reparse and missing-input assumptions. Actual issued JSON positives, full variant/range/coverage/target negatives; parser-only success never claims LLVM execution. |
| 5. LLVM + production cutover (BoxCount) | `published_mir_object.rs` lifecycle caller -> dedicated call-local consumer -> object/EXE | Emit the complete selected function/control/runtime ABI; switch the real host caller and delete its generic lifecycle JSON + V2 pending companion edge in the same series. Fixed source EXE30 and OBJ->linked EXE30, Fault ordering/cleanup, Normal-only out loads, frame lifetime and no partial artifact are mandatory. |
| 6. Caller-zero retirement (Delete) | selected transport/proof owners -> required production acceptance | Audit direct, manifest, aggregate and directory discovery; delete unused V2 wrappers/pending helpers/private probes and the legacy proof route. Retain shared helpers only with named live callers. No selected old retry/re-entry remains. |

### Step 4 prerequisite: `CONSTRUCTOR-LIFECYCLE-C-FRAME-SENTINEL-CONTRACT-D0`

Before Step 4 can expose one direct physical input, document and validate the
existing absence encoding in
`src/mir/function/published_backend_view_lifecycle_c_transport.rs`. The owner
is that Rust C-frame projection and its paired C lifecycle parser; it does not
issue source meaning. In scope are the source-ordinal absence on a receiver,
the root's absent receiver/object fields, and absent operation/control fields.
Each field must name whether `u32::MAX` means absent, and the parser must reject
an impossible combination instead of treating it as an ordinary ordinal.

Acceptance: a receiver has no source ordinal, source formals retain theirs,
and malformed absent/present combinations reject before object emission. The
exclusive cleanup set is only the anonymous raw sentinel literals in this
frame, replaced by named shared Rust/C ABI constants where that does not alter
the wire. It does not change receiver/source semantics, other `u32::MAX` uses,
or enable LLVM execution.

### Step 4 direct-input task: `CONSTRUCTOR-LIFECYCLE-DIRECT-PHYSICAL-INPUT-I0`

This is the concrete Step 4 owner/caller row: final view physical program and
ABI projection -> one C-parser input. Its successor constructs the final issued
input once; it must not serialize a program String, parse it into a JSON Value,
then serialize it again. The intermediate program-string path and its
`program-parse` branch are the exclusive delete set, but only after the real
selected consumer uses the successor. Acceptance is issued Pair input with
exact function/order/layout plus malformed, range, coverage and target
negatives. Parser-only success remains parser-only evidence; it does not claim
an object, EXE, `llc` behavior or a C execution cutover.

`CONSTRUCTOR-LIFECYCLE-HOME-ORIGIN-OWNER-SPLIT-I0` is landed (BoxShape): root
Home progress/validation now lives in `ordinary_new_local_commit/root_home.rs`.
The parent is 633 lines and the child is 125; public API, tests, behavior and
test paths are unchanged. `CONSTRUCTOR-LIFECYCLE-HOME-ORIGIN-RETENTION-I0`
now owns only the existing `RootHomeExitProgress` -> selected emitter handoff;
it cannot widen source acceptance or touch Birth representation/Reclaim.

Step 0 validation uses the existing semantic-package suite, identical test
discovery before/after, source line counts and pointer guard. No new fixture,
fallback, source shape or baseline change. If a test fails, classify it using
the same command at the parent before calling it baseline debt.

Step 0 is landed. The next row is a design stop: name the source authority and
canonical issuer for each retained formal/value requirement and checked
operation origin before an implementation slice is opened. It cannot use a
physical value, final-MIR observation, metadata, or C-side default as a source
substitute.

## D0: source-requirement issuer census

Historical design progression below records the premises of bounded landed
slices. The current issuance design near the end supersedes blanket
NoSafeSlice and source-rejection interpretations; do not rerun these censuses.

Decision: source retention is `NoSafeSlice` for implementation. Existing
source plans are the only candidates to extend; a second synchronized source
table, a new semantic receipt, or an inferred physical type is not allowed.
Source authority + canonical issuer: `ordinary_new_coseal` issues selected New
claims, root terminal relations and field-read rows; `instance_construction`
issues constructor store obligations; `birth_abi_handoff` issues Birth receiver
and parameter lane relations plus Unit result; root completion issues the Home
exit obligation.
Non-authority: `FunctionMetadata.value_types`, builder `type_ctx`, final MIR
values/blocks/instruction order, spans, Birth lane ordinal alone,
`Phi.type_hint`, and JSON/C defaults.
Fail-fast boundary: a source requirement without exactly one row below rejects
before claim take and physical binding. `None`, unavailable source coverage,
or a Pair-specific witness cannot fall through to inferred i64 or Unit.
Smallest next slice: `CONSTRUCTOR-LIFECYCLE-SOURCE-REPRESENTATION-D1`, which
names the issuer and selected claim-take consumer for missing source
representation and per-operation-origin products.
Non-claims: all-SSA binding, u64 diagnostic encoding, target/frame ABI,
JSON/C lowering, Pair EXE/OBJ30, or retirement.

Census boundary: selected App Main root plus retained canonical Birth keys ->
installed semantic package port and claim take; includes selected New,
FieldSet, HomeRelease, Reclaim, root terminal reads/result and Birth
formals/result; excludes final-MIR bindings, synthetic compiler values,
backend/runtime layout, other call families and VM/WASM.

| Product | Existing issuer and exact consumer | State and next rule |
| --- | --- | --- |
| Root terminal result and reads | `ordinary_new_coseal` co-seals `TerminalI64AddReturnV1` with two exact field-read sites. The selected terminal emitter reserves, records and completes it. | **Present.** It proves the selected terminal i64 relation only; it does not type every root value. |
| Root HomeRelease | Root completion issues ordered terminal homes and exact exit; `RootHomeExitProgress` retains that binding/exit with object and `ValueId`, and the selected root Home emitter records the exact `HomeRelease`. | **Present.** Final root validation checks ordered origins and the concrete release operation; this does not cover Reclaim or arbitrary root representation. |
| Root formals and arbitrary root values | Selected root source/completion reaches raw callable lowering and final view. | **Missing / CutoverBlockerOpen.** Pair's zero-formal witness cannot narrow this domain. D1 must name their semantic-class issuer. |
| New allocation | `OrdinaryNewAdmissionClaimV1` retains exact New site, object, destination, Home prefix and construction; the package port takes the same owned site. | **Present.** The allocation identity is source-issued; representation of its result is still missing. |
| Birth formals and result | `BirthAbiHandoffV1` issues owner, receiver/parameter bindings and physical lane order with Unit result; claim take transfers it to root/Birth handoff. | **Partial.** Unit result is present through final handoff/C transport; receiver and a parameter's value representation require their own existing source-use relation. |
| Birth FieldSet | `ConstructionPlanV1` retains each `ResolvedAssignmentSourceV1` with canonical field; selected construction/body lowering consumes it. | **Present source obligation; physical handoff missing.** D1 must not create a duplicate store issuer. |
| ReclaimUnpublished | `ConstructionPlanV1` issues required outer-storage reclaim and constructor identity; the exact ordinary-New claim site co-seals it, `NewEmissionProgress` retains it, and selected Birth-fault cleanup records the operation. | **Present through selected finalization.** Final validation rejects source, presence, binding and operation drift; target/runtime lowering remains separate. |
| Copy, PHI, frame, InvokeNormalResult, branch/edge values | No selected source requirement issuer exists; the builder creates or binds them. | **Missing as source products.** Later physical binding may admit them only through completed input relations or named emitter rules, never metadata inference. |

The finite state vocabulary is `Present | Partial | Missing`. `Present` means
one existing issuer and named consumer, not backend readiness. `Partial` and
`Missing` are inside this census boundary and therefore
`CutoverBlockerOpen`; they cannot be parked or repaired by C. D1 must decide
one existing issuance owner per missing product, its exact claim-take consumer,
its exclusive old information-loss edge, and its pre-emission negative. If one
is absent, D1 remains `NoSafeSlice`.

## D1: representation and cleanup-origin disposition

Decision: retain only products with an existing source issuer. Root/Birth
representation remains partially `NoIssuer`; it is not a prerequisite for the
bounded Home-origin row, but it blocks any claim of complete source retention.

| D0 product | Existing owner, consumer and loss edge | D1 disposition |
| --- | --- | --- |
| Root i64 terminal reads/result | `scan_new_home_flow` issues the exact literal/field/Add relation; selected terminal emission consumes it. Only this terminal relation crosses the claim. | **Existing issuer, bounded.** It cannot classify arbitrary root values. |
| Root New arguments | The same walk observes exact New children and only recognizes source trivial literals; `CallerNewHomePrefixV1` drops their sites/classes before selected emission receives bare values. | **Existing issuer, later extension.** Retain site/class in the existing prefix/claim only when an accepted direct consumer is named. |
| Root formals | The selected root Home walk rejects receiver/parameter declarations as `EntryDemandMissing`; no root formal relation reaches the retained handoff. | **NoIssuer.** Reject this source domain before claim take; zero-formal Pair is not a domain reduction. |
| Birth receiver | `BirthAbiHandoffV1` owns binding/lane and `ConstructionPlanV1` owns the exact object, but they are not co-sealed after claim take. | **Existing issuers, later co-seal.** D1 does not infer object handle from lane. |
| Birth parameters | Birth handoff owns bindings/lanes/Unit; constructor stores identify i64 use only for admitted parameter RHS. Unused or untyped formals have only `OpaqueHandle`. | **Partial.** Retain i64 use requirements separately from declaration contracts; missing execution support is unavailable, not a source rejection. See current task 1b. |
| HomeRelease origin | Completion co-issues ordered terminal homes and exact exit; the prefix carries binding plus outward-fault site. `RootHomeExitProgress` retains each binding/exit with its existing object/value through selected emission. | **Landed.** The naked positional handoff is deleted. Final validation rejects origin count/order/exit or concrete `HomeRelease` operation drift. |
| ReclaimUnpublished cutpoint | `ConstructionPlanV1` retains constructor identity and the construction-fault reclaim obligation; the exact claim site is co-sealed before `NewEmissionProgress` transfers it to selected cleanup. | **Landed.** Final validation rejects missing, duplicate or drifted Reclaim operation/binding; generated CFG position is non-authority. |

`CONSTRUCTOR-LIFECYCLE-HOME-ORIGIN-RETENTION-I0` is landed. It has one
authority (`RootHomeExitProgress`), one consumer (the selected root Home
emitter), and deleted the bare `(CanonicalObjectIdV1, ValueId)` positional
handoff. Acceptance covers ordered Pair `HomeRelease` emission with distinct
retained origins and concrete operation drift, alongside the existing
foreign/missing/duplicate Home, wrong-exit, reordered-row and unavailable
root-cleanup negatives. It makes no representation or Reclaim claim.

## Reclaim origin D0 decision

Decision: retain the existing construction-fault Reclaim origin in
`NewEmissionProgress`; do not issue another semantic receipt.

Source authority + canonical issuer: `instance_construction::issue_construction_plan`
issues the outer-storage reclaim obligation and its exact
`(ConstructorSourceIdV1, FunctionOwnerIdV1)` identity; `ordinary_new_coseal`
co-seals that plan with the exact `OrdinaryNewAdmissionClaimV1` owned New site.

Non-authority: generated cleanup CFG/block placement, final MIR coordinates,
`ValueId` layout, target/runtime ABI, C transport and a bare `Birth` branch.

Fail-fast boundary: plan/claim object or constructor-identity mismatch rejects
before emission; missing, duplicate or drifted concrete Reclaim rejects during
existing final new-emission validation, before final observation/seal.

Smallest next slice: `CONSTRUCTOR-LIFECYCLE-RECLAIM-ORIGIN-RETENTION-I0`.

Non-claims: NoBirth allocation cleanup, Birth FieldSet cleanup, root Home,
root/Birth value representation, target ABI, C activation and LLVM execution.

Census boundary: eligible direct-local canonical-Birth `ConstructionPlanV1`
construction-fault obligation co-sealed into one ordinary-New claim ->
`NewLocalCommitV1` emission progress -> selected Birth-fault cleanup -> existing
final new-emission validation; includes only selected direct-local Birth New;
excludes NoBirth, unavailable/override construction, constructor-body FieldSet
cleanup, generated CFG position, target/runtime/C.

| Finite source/emission state | Authority and required action | Terminal / fallback |
| --- | --- | --- |
| Eligible canonical Birth plan | Existing plan and exact claim site issue/co-seal the reclaim origin; progress retains it once and selected cleanup consumes it once. | Emit exact Reclaim only on the Birth fault suffix; validate origin and operation. |
| NoBirth | Existing claim has no constructor identity for a constructed object. | No reclaim origin; allocation-fault cleanup only. No fallback classification. |
| Unavailable or override construction | Existing construction eligibility is unavailable. | Existing pre-artifact rejection; mint no origin. |
| Missing or drifted plan/object/constructor relation | Existing plan/claim consistency check fails. | Typed pre-emission error; never infer from generated CFG or `Birth` alone. |
| Missing, duplicate or drifted recorded Reclaim | Existing physical progress/final validation observes a mismatch. | Typed finalization error before observation/seal; no retry. |

The exclusive old loss edge is `NewEmissionProgress::Prepared` and
`begin_new_emission()`: they currently retain only prior Home operands. The
selected emitter reconstructs `ReclaimUnpublished { object, value }` from
`claim.constructor()==Birth`, losing the plan's source constructor identity and
exact New-site-to-construction-fault relation. I0 replaces that internal bare
reconstruction with one retained origin under the existing progress owner.
Its acceptance is a two-Birth ordinary-New positive proving one distinct
source origin per fault suffix and none on Normal, source negatives for
NoBirth/unavailable/foreign/missing/duplicate relation, and final-MIR mutation
negatives for object/value/block removal or duplication. It reuses existing
construction-take evidence and does not add a fixture, fallback or guard.

`CONSTRUCTOR-LIFECYCLE-RECLAIM-ORIGIN-RETENTION-I0` is landed. The existing
progress owner retains the exact New site, constructor source/owner and object;
selected Birth-fault cleanup records its concrete `ReclaimUnpublished` once.
The prior bare `claim.constructor()==Birth` reconstruction is deleted. Focused
evidence covers two distinct Birth claims and a mutated Reclaim value rejected
as `reclaim-origin-operation-drift`. No target/runtime or C execution claim is
made.

## Root/Birth representation D2 decision

Decision: retain only an already-issued source representation; a physical
`ValueId`, `MirType`, lane ordinal, metadata or C `input_kind` cannot fill a
missing source class.

Source authority + canonical issuer: `instance_construction::issue_construction_plan`
issues the accepted Birth FieldSet RHS relation; `scan_new_home_flow` issues
root direct-New trivial arguments; `BirthAbiHandoffV1` and
`ConstructionPlanV1` supply the two existing products for a receiver co-seal.

Non-authority: `type_ctx`, final-MIR values, a Birth lane ordinal alone,
metadata, generated CFG, target/runtime/C transport and Pair's zero-formal
witness.

Fail-fast boundary: an accepted source row without exact binding/class rejects
before claim take; a selected emitter never defaults it to i64 or handle.

Smallest next slice: `CONSTRUCTOR-LIFECYCLE-BIRTH-FIELDSET-RHS-CONSUMER-D3`,
which must name an AST-free selected consumer before a retained RHS can be
implemented.

Non-claims: root formals/arbitrary root SSA, unused or untyped Birth formals,
New-result handles, target ABI/C execution, Pair EXE/OBJ30 or complete
representation coverage.

Census boundary: selected App Main root plus canonical Birth construction
plans -> exact ordinary-New claim take -> selected construction/New consumers;
includes root direct-New arguments, Birth receiver/formal/result and accepted
FieldSet RHS; excludes source-less temporary/PHI/control values, other call
families, final-MIR layout and target/runtime/C.

| Row | Existing issuer -> consumer | D2 disposition |
| --- | --- | --- |
| Root terminal i64 reads/result | `ordinary_new_coseal` terminal relation -> selected terminal emitter | Present / landed; does not classify arbitrary root values. |
| Root formals and arbitrary root values | root Home walk gives `EntryDemandMissing`; no representation relation | NoIssuer / NoSafeSlice. |
| Root direct-New trivial arguments | `scan_new_home_flow` exact child classification -> `CallerNewHomePrefixV1` -> selected New emitter | Existing issuer, later bounded retention. |
| New allocation identity/result | ordinary-New claim site/object/destination -> selected New emitter | Identity present; result handle partial and cannot be inferred from `MirType::Box`. |
| Birth Unit result | `BirthAbiHandoffV1` Unit completion -> final handoff -> lifecycle transport | Present; no new representation row. |
| Birth receiver handle | Birth receiver lane plus construction object -> claim take -> selected New emitter | Existing products, later co-seal only. |
| Birth FieldSet literal/parameter RHS | construction plan exact RHS recognition -> selected construction-state consumer -> FieldSet emitter | Existing issuer and consumer / I0 selected. Retain the descriptor and delete the selected target/RHS raw replay together. |
| Birth unused/untyped parameter | lane exists but source class/use relation does not | NoIssuer; never default to i64. |
| Copy/PHI/frame/normal-result/control values | no source issuer | NoIssuer as source product; later physical owner only. |

The D3 audit found one closed selected consumer route. It is recorded below;
the prior raw replay is the old edge to delete in the same I0, rather than a
fallback retained beside the new consumer.

Root trivial-New arguments share the same raw-child re-lowering problem and
are also NoSafeSlice until their consumer is named. The later Birth receiver
co-seal remains separately auditable because selected emission already uses the
allocation result as receiver, but it must prove a final validator before it is
selected. None of these rows creates a representation for root formals,
arbitrary values or unused/untyped Birth parameters.

## D3: Birth FieldSet RHS consumer decision

Decision: open `CONSTRUCTOR-LIFECYCLE-BIRTH-FIELDSET-RHS-CONSUMER-I0` as one
BoxShape. Extend the existing `ConstructionPlanV1` store row; do not add a
second source table, semantic receipt, child port, target ABI, or C path.

Source authority + canonical issuer: `instance_construction::issue_construction_plan`
already owns the parser-declaration loan, the exact resolved assignment/source
sites, canonical field and accepted RHS relation. Its existing store row issues
one private RHS descriptor: `LiteralI64(i64)` or
`Parameter { site: SourceExprSiteV1, binding: BindingRefV1 }`. It also retains
the FieldSet receiver's exact `SourceExprSiteV1` and `BindingRefV1`. These are
the source-use relations that the selected consumer must consume; they are not
new semantic products.

Non-authority: assignment/field-access AST after issuance, variable names,
binding/lane ordinals, `MirType`, emitted `ValueId`, raw child sources,
metadata, target/runtime/C transport and generic assignment lowering.

Fail-fast boundary: source issuance rejects every RHS other than the exact
integer literal or a resolved local Birth parameter before plan installation.
Installation/take rejects duplicate, missing or foreign stores. Take obtains
the exact receiver value and consumes its retained source site through the
existing `observe_variable_site`; emission does the same for a parameter RHS
after `value_for_exact_binding(owner, binding)`. Completion therefore rejects
an unconsumed, duplicate, foreign, or value-drifted source use before final
publication. Emission rejects an unavailable selected state before `FieldSet`;
it never re-enters raw AST lowering.

Smallest next slice: retain that descriptor with the existing store, transfer it
in `TakenConstructionStore`, and make `emit_construction_store` consume the
taken store directly. Literal physicalization uses the existing named
`emission::constant::emit_integer`; parameter physicalization uses the existing
exact-binding primitive. The selected statement branch becomes
`take -> emit -> Lowered(value)`, deleting only its assignment destructure,
target-shape check, prepared target/RHS child sources, structured child scope,
two `drive_legacy_expression_v1` calls and demand completion.

Non-claims: generic child-port retirement, root trivial-New arguments, Birth
receiver co-seal, source-shape widening, representation coverage for untyped
parameters, target ABI/C activation, Pair EXE/OBJ30 and production cutover.

Census boundary: the selected Birth assignment statement after exact
construction-store take -> its unique selected construction emitter -> retained
construction final validation. Includes literal and Birth-parameter RHS,
receiver/field/value/block FieldSet bindings and the selected raw-replay edge.
Excludes generic assignment consumers, other child-port callers, other
constructor forms, final-view/LLVM/runtime/C and root New arguments.

| Owner/caller/terminal | I0 change and exclusive delete-set | Acceptance |
| --- | --- | --- |
| `issue_construction_plan` -> `install_construction` -> `take_construction_store` -> `statement_surface` -> `emit_construction_store` -> `validate_bindings` | Store the exact receiver source use and RHS descriptor beside the existing resolved assignment and canonical field; carry the RHS in `TakenConstructionStore`; take consumes the receiver source use and emit consumes a parameter RHS source use. Emit the RHS and FieldSet from that taken product. Delete the selected branch's raw target/RHS descent only. `RawStructuredChildScopePortV1`, `RawInvocationChildPortV1` and generic helpers retain their other callers. | Ordinary-New Birth positives for literal and parameter stores, including reversed declaration/store order and zero unconsumed variable sites at finalization. Source negatives for unsupported RHS and non-parameter/wrong binding; physical negatives for foreign/missing/duplicate stores and unavailable/foreign binding; final-validation mutations for FieldSet base/value/block and literal-value drift. |

`statement_surface` is the only direct `take_construction_store_v1` caller;
the raw invocation/structured ports only forward it. The terminal already checks
concrete `FieldSet` ownership, base/value/block and completion. I0 extends those
checks with the retained descriptor relation as needed; no compatibility retry
or AST re-read is permitted.

The focused field-read command
`mir::normal_callable_semantic_package::ordinary_new_coseal::field_reads::tests::terminal_read_rows_retain_alias_sites_and_commit_only_complete_expression`
still fails with `left: 1`, `right: 0` at
`ordinary_new_field_reads_tests.rs:124`; the identical parent-commit command
reproduces it. It is known baseline debt and does not reopen this BoxShape row.

## CONSTRUCTOR-LIFECYCLE-ROOT-NEW-TRIVIAL-ARGUMENT-CONSUMER-D4

Decision: `NoSafeSlice`; do not retain or lower root direct-`New` arguments yet.

Source authority + canonical issuer: `issue_ordinary_new_claims_v1` is the
sole issuer of the selected direct-local `OrdinaryNewAdmissionClaimV1` under
one resolver-batch loan. It retains owner/site, class, aggregate arity,
destination/declaration, Home-prefix result, construction/object/destruction
and Birth recipe. `scan_new_home_flow` only checks that each `CallArgument(i)`
is trivial; `CallerNewHomePrefixV1` retains destination, prior Homes, fault
continuation and covered statements, then the per-child identity/class is
discarded.

Non-authority: argument AST after selected claim take,
`PreparedRawChildSourceV1`, argument names, raw source context, inferred MIR
types, `ValueId`, target ABI and C transport.

Fail-fast boundary: claim take presently checks site/class/aggregate arity and
the final New terminal validates only lifecycle bindings/root completion. No
consumer can prove individual argument source-use consumption. The selected
route takes the claim and then loops over raw `Vec<ASTNode>` arguments through
`drive_legacy_expression_v1`; deleting that loop would remove the only
materialization path. `complete_exact_demands_v1` checks a child queue only and
is not a semantic argument consumer. Therefore no exclusive delete-set exists.

Smallest next slice: `CONSTRUCTOR-LIFECYCLE-ROOT-NEW-TRIVIAL-ARGUMENT-ISSUER-D5`.
Design the finite per-child relation as a co-sealed part of the existing claim,
then name its sole consuming terminal before implementation.

Non-claims: root formals/arbitrary root SSA, handles, nontrivial argument
shapes, Birth receiver co-seal, target ABI/C activation, Pair EXE/OBJ30 and
production cutover.

Census boundary: selected App Main direct-local ordinary `New` candidates
issued by `issue_ordinary_new_claims_v1` -> selected prepared New -> ordinary
New admission -> raw child descent -> existing final New validation. Includes
each finite `CallArgument(i)`; excludes field initializers, generic raw
children, Core13/integer routes, non-App-Main/nonselected callables and
target/runtime/C.

## CONSTRUCTOR-LIFECYCLE-ROOT-NEW-TRIVIAL-ARGUMENT-ISSUER-D5

Decision: implementation-ready BoxShape. Co-seal ordered affine argument rows
inside the existing `OrdinaryNewAdmissionClaimV1`; add neither an issuer nor a
standalone receipt.

Source authority + canonical issuer: the exact resolver-batch loan in
`issue_ordinary_new_claims_v1` issues every selected direct-local New claim.
Its existing child walk already owns exact `CallArgument(i)` source roles. The
co-sealed row is `(owner, New site, ordinal, argument site, kind)`, where kind
is `Integer(i64)`, `Bool(bool)`, or `LocalTrivial { binding }`. Ordinals are
exactly `0..arity`, unique and owner/site-consistent. A Bool value is copied
from the same resolved literal source expression at issuance; its generic
literal category alone is insufficient.

Non-authority: `CallerNewHomePrefixV1` as an argument issuer, raw AST/child
scope, names, `MirType`, `ValueId`, physical ABI/C and a new semantic receipt.

Fail-fast boundary: missing, duplicate, foreign, ordinal-drifted or nontrivial
rows reject before child effects. `emit_integer`/`emit_bool` materialize literal
rows; `observe_variable_site` consumes a trivial-local row with its exact
binding/current value. The root New finalizer rejects residual rows,
value/order/call-argument drift before publication.

Smallest next slice: `CONSTRUCTOR-LIFECYCLE-ROOT-NEW-TRIVIAL-ARGUMENT-CONSUMER-I0`.
Take the selected claim before raw child-demand creation; the existing selected
New emitter consumes all rows and retains their emitted argument values for the
existing root New finalizer.

Non-claims: a general root SSA/argument representation, nontrivial arguments,
Birth receiver co-seal, target ABI/C activation, Pair EXE/OBJ30 and production
cutover.

| Owner/caller/terminal | I0 change and exclusive delete-set | Acceptance |
| --- | --- | --- |
| `issue_ordinary_new_claims_v1` -> selected claim take -> selected ordinary-New emitter -> root New finalizer | Co-seal ordered argument rows in the existing claim; materialize/consume them at the selected emitter and validate exact emitted Call argument order/value. Delete the selected raw argument loop, selected `CallArgument` child-source creation/queue completion, and selected route's raw argument carriage. The unselected compatibility loop and generic child infrastructure remain. | Positive selected direct-New integer, bool and trivial-local argument cases; source negatives for nontrivial/missing/duplicate/foreign/ordinal drift; finalizer mutations for residual/value/order/call-argument drift; guard confirms selected path has no raw argument descent. |

### I0 execution order (2026-09-07 acceptance audit)

The selected claim take and raw-child deletion are landed, but I0 is not closed:
the finalizer currently compares a retained `Vec<ValueId>` with the Birth Call.
That catches a Call-only mutation, but cannot prove that literal values still
denote their issued source rows.  Do not create a second semantic receipt or
recover an argument from AST to close this gap.

1. **Physical-row validation (BoxShape).** `NewEmissionProgress::Emitted` is
   the sole owner of a non-semantic physical snapshot for each already-issued
   row: source row plus emitted `ValueId`.  The selected emitter records it;
   `validate_new_emissions` checks cardinality, owner/site/ordinal, Birth Call
   order, and the matching `ConstValue::{Integer,Bool}` definition for literal
   rows.  A local row is checked at consumption by
   `value_for_exact_binding` plus `observe_variable_site`; the finalizer
   preserves that emitted identity and Call order.  This replaces the bare
   `Vec<ValueId>` snapshot.  It does not add a source fact, selector, ABI, or
   C state.
2. **Terminal evidence.** Add one selected App Main compile case with
   `new Page(11, true, local_value)`.  Inspect its single Birth Call and prove
   its arguments are, in order, the integer constant, bool constant, and the
   exact current local `ValueId`; retain the existing source-complete/final
   publication assertions.  Add compact finalizer mutations for residual,
   literal value, order, and Call drift.
3. **Boundary evidence and retirement guard.** Cover nontrivial, missing,
   duplicate, foreign, and ordinal source-row rejection at their existing
   issuer/consumer boundary.  Extend the existing
   `mir_call_canonical_corridor_guard.sh`: the selected New branch prepares
   before child-demand creation and has no raw argument descent; its
   compatibility branch and generic child infrastructure are explicitly
   outside this assertion.
4. **I0 closeout.** Update the owning builder/package README only with the
   issued-row -> physical-snapshot -> finalizer contract, run the focused
   positive/negative/mutation tests and the reused guard, then update the
   current-state summary.  Pair EXE/OBJ30, typed C ingress, and compatibility
   retirement remain separate rows.

I0 closed at `5e7fadb686`: selected direct-`New` now consumes the issued
integer/bool/trivial-local rows without raw argument descent; the existing
ledger retains only the physical row/`ValueId` snapshot and final validation
rejects residual, literal, order, or Birth-Call drift. The focused source,
consumer and mutation tests plus the canonical-corridor guard are green. This
does not open root/Birth ABI handoff, C execution, Pair EXE/OBJ30, or selected
compatibility retirement.

## CONSTRUCTOR-LIFECYCLE-ROOT-NEW-BOOL-SOURCE-FACT-D6

Decision: return to design stop. `scan_new_home_flow` observes direct-New
argument sites but exports no row; more importantly its resolver inventory has
`ResolvedLiteralSourceV1::Bool` without the Bool payload. Recovering that value
from raw AST would create a second source authority.

Source authority + canonical issuer: the resolver source inventory must retain
the Bool payload. `issue_ordinary_new_claims_v1` remains the only claim issuer;
`scan_new_home_flow` is a helper that may export a neutral observation, never a
package-row issuer.

Non-authority: raw AST, `CallerNewHomePrefixV1`, builder child scope, MIR type,
ValueId, ABI/C and a standalone semantic receipt.

Fail-fast boundary: until the source inventory has exact Bool payload and the
single walk can export integer/bool/trivial-local observations, I0 may not
co-seal or consume rows.

Smallest next slice: decide and audit the resolver-owned Bool payload change
and a neutral source-walk callback/output that leaves nonselected callers
unchanged.

Non-claims: generic literal redesign, source-form widening, raw-AST fallback,
root SSA, ABI/C, Pair EXE/OBJ30 or production cutover.

## D7: neutral selected-New argument observation

Decision: implementation-ready. `home_new_prefix` owns a neutral observation
and remains below the package dependency boundary. Its inner source walk emits
`SelectedNewArgumentObservationV1 { new_site, arguments }`; each child row is
`(ordinal, site, Integer(i64)|Bool(bool)|Local(binding))`. Unsupported source
is an explicit per-New unavailable result, separate from Home-prefix cleanup.

Source authority + canonical issuer: the existing resolver source inventory
and `scan_new_home_flow` are the only fact owner. A new inner walk returns the
existing tuple plus an observation map; existing public wrappers discard that
map. Package-only companion wrappers retain it under the same walk and
`issue_ordinary_new_claims_v1` alone converts it into the existing claim row.

Non-authority: package types in resolved semantics, raw AST, child scope,
CallerNewHomePrefix, MIR type/ValueId, ABI/C and a standalone receipt.

Fail-fast boundary: absent/foreign/duplicate/ordinal/arity drift rejects at
claim co-seal. A nontrivial selected argument remains explicit source
unavailable and reaches the existing pre-effect terminal; no raw retry.

Smallest next slice: add the neutral model/wrappers and co-seal rows in the
existing claim, with source-only mapping tests in new small test modules.

Non-claims: completion-flow changes, a second source walk, new source forms,
raw fallback, generic argument representation, ABI/C or Pair EXE/OBJ30.

Step 1's exact source inventory and step 3's final-entry status connection are
explicit remaining design obligations. Do not mislabel this roadmap as a
completed all-SSA issuer. Steps 1–4 close contracts but do not independently
claim production migration; step 5 must remove the selected old production
edge. After step 6, follow the existing workstream order for canonical versus
compatibility dispatch, compile-owned state and single runtime hook storage.

## Root source handoff: accepted premise reset (2026-09-07)

Decision: extend the existing ordinary-New issuer and ledger to retain the
selected AppMain source obligation through final handoff (BoxShape). This
supersedes the D0/D1 permanent `NoSafeSlice` conclusion at `27bc10f183`.
Source authority + canonical issuer: `issue_ordinary_new_claims_v1` co-seals
the validated AppMain identity with Completion and the existing exact terminal
relation from one resolved source loan; no new semantic receipt is issued.
Non-authority: generic main-thunk results from another pipeline, MIR signatures,
metadata, physical key strings, empty parameter arrays, C defaults and fixtures.
Fail-fast boundary: identity/site/Completion mismatch or unconsumed/drifted
selected emission rejects before final handoff; absent coverage stays explicit
physical-unavailable, independently of accepted source syntax.
Smallest next slice: `CONSTRUCTOR-LIFECYCLE-ROOT-SOURCE-HANDOFF-I0`, below.
Non-claims: arbitrary root results/formals, complete Birth representation,
physical entry/status ABI, C execution, Pair EXE/OBJ30 or full cutover.

### Why the previous stop was too broad

An independent worker read the complete Home/terminal classifier and challenged
its premise; the primary agent checked physical root creation/finalization.
The canonical-core main-thunk relation is indeed outside this session. It is
not a prerequisite for retaining the selected ordinary-New terminal relation.
`TerminalI64AddReturnV1` already records source owner, Return, Add and two exact
read sites. Requiring a generic full-result issuer first conflated broader
coverage with this existing source obligation and contradicted this SSOT's
source/physical/target split. The prior audit does not justify permanent park.

Source membership is available before lowering: `issuer.rs` obtains AppMain
parser identity from the catalog; `app_main_relation.rs` verifies its brand,
identity, arity, owner and complete forest. The current ordinary-New call reduces
that identity to a batch slot. Its one `with_lowering_input` loan already issues
Completion, Home prefixes, terminal and argument observations together.
Final sealing later reduces the terminal to `I64AddReturn { owner }`.
These are the two concrete information-loss edges to replace.

Physical root placement remains owned by `module_lifecycle`: it creates the
root function, lowers into that function, inserts it and passes it to final
validation. The key captured by `normal_default_root_catalog_post_install`
is a physical lookup for that exact function, not source result authority.
Backend entry layout, hidden frame and process-status mapping belong to the
later physical/target tasks. Source Facts need no physical entry symbol.
Erasing source bindings at the C wire is allowed after their exact physical
mapping has been validated; C does not need a second semantic binding registry.

### Finite source boundary and counterexamples

Boundary: validated AppMain membership -> ordinary-New source loan -> existing
Home/terminal classification -> selected terminal emission -> final handoff.
Includes identity issue/drop/consume, entry demands, prefix locals/aliases/New,
all terminal classifier arms, physical binding validation and residual siblings.
Excludes Birth-body issuance, generic main-thunk flow and target/runtime ABI;
these exclusions do not certify the surrounding lifecycle cutover as complete.

| Existing source arm | Retention/coverage decision |
| --- | --- |
| Receiver, parameter or capture entry demand | Preserve `EntryDemandMissing`; empty MIR params cannot establish source eligibility. |
| Local integer/bool/trivial binding, Home alias, direct New prefix | Preserve existing source observations and exactly-once consumption. |
| Bare Return; integer/bool/trivial local; single initialized integer field | Preserve source acceptance. No exact two-read terminal product means physical coverage remains unavailable. |
| Add of two direct initialized integer field reads | Retain the existing exact Return/Add/ordered-read relation; this is the selected I0 result. |
| Recursive scalar Add | Preserve existing classification, but do not promote its coarse `Integer` arm into a generic result/type issuer. |
| Unsupported prefix, uninitialized use, overrides, nontrivial argument, uncovered terminal | Preserve the existing unavailable cause; do not synthesize a relation or retry. |

Counterexample: `return (pair.left + pair.right) + 1` may have scalar source
coverage but does not issue the direct two-read terminal relation. Likewise
`return true` must never acquire an i64 result merely because it is trivial.
These are acceptance distinctions to exercise in existing source tests, not
new implementation fixtures added during this design turn. Other accepted
root-result arms remain in-boundary `CutoverBlockerOpen` for broader coverage.

### CONSTRUCTOR-LIFECYCLE-ROOT-SOURCE-HANDOFF-I0

One responsibility: retain the already-selected root source obligation through
its existing production consumer and finalization. No classifier change.

- Owner/caller: package `issuer.rs` -> `issue_ordinary_new_claims_v1`.
  Pass the existing `CallableDeclarationIdentityV1` instead of only `Option<u32>`;
  resolve its exact declaration in the same batch using identity equality and
  the existing uniqueness/owner checks. Slot remains navigation, not authority.
- Source storage: extend the existing `OrdinaryNewClaimLedgerV1`; retain root
  identity with its existing Completion/terminal products from that loan.
  Do not duplicate the terminal table, issue another semantic `Verified*` or
  re-run the Home classifier. Catalog identity and source result stay distinct.
- Consumer/terminal: `main_root::lower_app_main_root_body_v1` validates identity
  before `register_new_root`/body descent; existing terminal emitter records
  ordered read/Add/Return bindings. Existing finishing validation checks these
  against the final root, then `seal_finalized_root_birth_handoff` retains the
  exact source relation instead of projecting only its owner. The borrowed view
  receives that relation together with the existing physical root binding.
- Exclusive replacement set: slot-only AppMain handoff and owner-only terminal
  projection on this route. Update existing view/V2 readers to project the
  retained relation without reclassification. No new public endpoint; no C
  activation. Physical map keys remain lookup fields, never semantic evidence.
- Acceptance: existing Pair and renamed/alias/multiple-Home source cases reach
  final handoff with the exact identity/Return/Add/read ordering. Missing,
  foreign-compilation, foreign-identity, duplicate/residual and finishing
  operand/site drift reject. Preserve non-Add source acceptance and its existing
  physical-unavailable terminal; metadata changes cannot create admission.
- Checks/docs: focused source, terminal/finalization and borrowed-view tests;
  reused canonical-corridor and pointer guards; update builder/package README
  and lifecycle reference in the same slice. No new guard family. Touched
  parents are currently 663/666 lines for co-seal/local-commit: design a child
  at 760 and stop before 800; never compress code to satisfy the limit.

Landed at `482666ef07`: AppMain identity and exact terminal remain one ledger
relation through final validation and the borrowed view. Alias and multiple-Home
Pair cases prove exact source retention; several exact New sites retain one
canonical Birth definition while their local emissions remain independently
validated. Pair EXE/OBJ30 and C activation remain open.

## Issuance design: replace the waiting premise

Decision: extend existing source owners to retain missing relations; select the
Birth declaration-object handoff as the next bounded BoxShape. The blanket
`CONSTRUCTOR-LIFECYCLE-SOURCE-REPRESENTATION-ORIGIN-NOSAFE` at `5d77803efc`
is superseded. Missing implementation is a design obligation, not a demand
that an issuer appear externally before work may resume.
Source authority + canonical issuer: the constructor semantic batch owns exact
Box membership/object identity; its Birth row and `BirthAbiHandoffV1::issue`
retain that relation. Root observations remain owned by the same Home scan.
Non-authority: object names, MIR types, lane positions, optional metadata,
caller literals as a declaration ABI, and C wire tags.
Fail-fast boundary: inconsistent source identities reject before selected
emission; final binding/membership drift rejects before transport/artifact.
Smallest next slice: `CONSTRUCTOR-LIFECYCLE-BIRTH-OBJECT-HANDOFF-I0` below.
Non-claims: completed formal/root-result coverage, new source restrictions,
C execution, Pair EXE/OBJ30, or completion of the surrounding cutover inventory.

### Premise audit and finite boundary

Boundary: exact Box/Birth declaration plus selected AppMain source loan ->
existing plans/claims -> final source handoff -> its physical projection.
Includes source issue/drop/consume, repeated definition relations, per-New
receivers, all current Home/terminal classifier arms and uncovered siblings.
Excludes runtime allocation identity issuance, other callable families and
target layout. This inventory identifies tasks; in-boundary blockers remain
`CutoverBlockerOpen`, so it does not claim cutover closure or Exhausted coverage.

`CanonicalObjectIdV1` denotes a declaration. An allocation instance is the
exact New site plus its checked Normal result. Two `new Pair` sites share one
declaration object, but cannot exchange their receiver values. Requiring a new
semantic "handle class" before retaining this declaration relation was wrong;
the backend owns the runtime representation of the source object relation.

The complete current classifier distinguishes Home, Handle, Trivial and
Uninitialized locals; terminal classification has Integer, OtherTrivial,
IntegerField and direct two-field I64Add, with a coarse recursive Add arm.
That coarse arm is not a proof of integer operands. Entry receiver/parameter/
capture demands and unsupported prefixes remain explicitly unavailable.
Transferred construction plans and per-New ledgers retain their one-way
ownership; no cross-pipeline main-thunk product or opaque subtree may fill gaps.
Counterexamples to preserve in implementation acceptance: two New sites of
one declaration; an unavailable construction body; an unused untyped formal;
`return true`; and `return (pair.left + pair.right) + 1`.

### `CONSTRUCTOR-LIFECYCLE-BIRTH-OBJECT-HANDOFF-I0`

- **Owner and change:** retain the object already selected by exact
  `box_source.same_source_as` in `instance_constructor_semantic.rs` on its
  existing semantic row. Move that lookup outside the construction-eligibility
  branch. `BirthAbiHandoffV1::issue` copies it with source ID, owner, receiver
  binding and target. This is immutable retention, not a new issuer/table.
- **Consumer and deletion:** existing ordinary-New co-seal/take passes it to
  local-commit finalization. Check object and eligible construction source/owner
  agreement at issuance and finalization. Compare complete Birth relations
  before deduplicating by key: equal repeats are one definition; unequal
  repeats reject. The C-frame projection reads the retained object and checks
  installed membership. Delete its `membership.get(key.owner())` derivation;
  symbol resolution and independent module membership validation remain.
- **Acceptance:** Pair, renamed Box, alias and repeated New reach the existing
  final handoff and C-frame construction; receivers remain independently
  validated. Foreign object/source/owner/receiver, unequal duplicate relations,
  receiver swap and final membership drift reject. Unavailable construction
  retains declaration identity and its existing unavailable behavior: it must
  not become a new source error or gain artifact admission.
- **Finish:** focused source/finalizer/view positives and negatives, reused
  lane/pointer guards, package README and lifecycle reference receipt. No new
  fixture or guard family. Co-seal/local-commit are 701/715 lines at design;
  use their existing children for validation additions before 760, hard stop
  at 800. No formal wire-tag change or C executable activation in this slice.

Landed at `5472b0ab09`: the semantic row retains its exact declaration object;
Birth handoff, final deduplication and C transport carry it. The transport now
checks object membership against that retained relation instead of deriving its
object from the Birth key owner. Focused Birth, alias/multiple-New final-handoff
and C parser tests pass. This does not select a formal representation or execute
the Birth C path.

### `CONSTRUCTOR-LIFECYCLE-BIRTH-FORMAL-DECLARATION-USE-D0`

**Decision (accepted):** the exact parser Birth declaration, its resolver
bindings, and the complete selected-body use inventory issue one
`BirthFormalContractV1` per formal inside
`issue_instance_constructor_semantic_batch_v1`.  The existing
`VerifiedInstanceConstructorSemanticRowV1` is the canonical issuer/retainer;
`BirthAbiHandoffV1` only carries that relation forward.  Do not borrow the
generic `callable_parameter_contract` catalog: it is issued for a different
direct-callable batch and resolver session, not constructor Birth.

Each contract retains exact ordinal and binding, declaration class
(`Unannotated`, `ExactI64`, `ExactText`, or explicit unsupported), and complete
use coverage (`NoUse`, i64-field stores with exact sites, or general/conflicting
use).  It also records a physical disposition.  A parameterized Birth is
`DeferredActualBinding` until a later physical binding proves its actual
representation; unsupported/tagged-or-checked cases remain explicit physical
unavailability.  This contract neither makes an unannotated formal `i64` nor
changes source acceptance.

The selected construction-store consumer and later final lifecycle admission
consume the same relation.  This I0's immediate consumer is the Birth ABI
handoff, which validates and retains it without granting artifact admission.
Declaration/binding/ordinal drift rejects at semantic issuance.  Opaque, text,
unsupported, general/conflicting, and not-yet-bound actuals stop at the later
physical admission boundary, preserving source validity and argument
evaluation.  Caller literals, MIR types, value IDs and C input tags are
non-authority.  The future exclusive deletion set is the C transport's
positional `input_kind` defaults and its anonymous absent-ordinal sentinel;
I0 below retains the contract only and does not alter those rows or enable C
execution.

### `CONSTRUCTOR-LIFECYCLE-BIRTH-FORMAL-CONTRACT-I0`

- **Bounded change:** add the child formal-contract model beneath the existing
  constructor semantic owner; issue it from exact `param_decls`, resolver
  bindings and the already selected construction-use inventory, then retain it
  through `BirthAbiHandoffV1`.
- **Authority / non-authority:** the constructor declaration loan is sole
  authority.  Construction plans contribute use requirements but do not declare
  a formal's class.  Actual New arguments, physical MIR values and C tags are
  not classifiers.
- **Consumers and terminal:** selected construction-store lowering owns exact
  use sites; `BirthAbiHandoffV1::issue` verifies and retains the same contracts.
  A later final lifecycle admission consumes the disposition before artifact.
  No generic callable catalog, raw AST re-entry, C default, fallback, or caller
  specialization may enter.
- **Acceptance:** typed-unused, untyped-unused, untyped i64-field store,
  repeated store, and differing actual values/classes retain source acceptance
  and the exact relation; uncovered/general/conflicting or unbound cases retain
  their explicit unavailable disposition for the later admission consumer.
  Declaration spelling, binding, ordinal, foreign-row and relation-drift
  mutations reject at issuance. Focused package/final-handoff positives and
  negatives plus existing guards pass.
- **Size / non-claims:** put the model in a child module; keep the parent below
  760 lines (800 hard stop).  This I0 does not choose a runtime tagged ABI,
  emit a C formal row, open source-to-EXE/OBJ execution, or delete the
  positional C defaults.

Landed in this implementation row: the child contract owner classifies only exact parser formal
spelling and selected construction-store use sites. The constructor row and
Birth handoff retain the same ordered relation; handoff rejects ordinal/binding
drift. Focused typed, untyped, unused, repeated-store and uncovered-body tests
pass. The later physical-binding consumer remains the sole owner of actual
argument representation and admission.

### `CONSTRUCTOR-LIFECYCLE-ROOT-SCALAR-TERMINAL-D0`

Finite audit boundary: selected AppMain source loan -> root-return dispatch ->
selected root-home exit emitter -> final root handoff -> lifecycle view -> C
schema ingress. It includes Unit/bare return and the existing direct-I64Add
relation; it excludes value-bearing Unit/literal/local/field/Add arms, process
status, artifact creation and executable behavior.

`VerifiedFunctionCompletionV1` already retains the same-session declared result
and exact exit site. `TerminalI64AddReturnV1` is the existing source relation
for the direct two-field Add arm. Other source arms remain accepted but have no
selected physical consumer and stay unavailable; they must not be reclassified
from MIR types, `ValueId`, function signatures, C tags, or fixture values.

Decision: split the finite terminal inventory by source authority. The next
bounded row admits only `return <direct untyped Integer literal>` following
selected New/Home cleanup. Its source authority is the same-session AppMain
identity, Completion's exact explicit return site, and that return child's
exact `ResolvedLiteralSourceV1::Integer(i64)` site. `scan_new_home_flow` is
the canonical issuer; it co-seals one literal relation into the existing
ordinary-New ledger. The selected root-return dispatcher is the physical
consumer: it emits that issued integer once and then reuses root-home cleanup.
The final handoff retains a distinct literal result relation and the existing
V2 transport maps it to the already validated `ROOT_I64 / I64` physical pair.

The exclusive selected old edge is the root `Return(Some(_))` raw value descent
that follows the unsuccessful direct-Add probe. I0 deletes it only while the
literal relation is reserved; it leaves generic and nonselected raw lowering
untouched. `Bool`, local, field, recursive/general Add, typed integer,
parameters, calls, process status, MIR constants/types/ValueIds and C tags are
non-authority and remain unavailable in this row. Missing/foreign completion,
return/value/literal-site drift, mixed terminal relations, or Const/Return
physical drift reject before lifecycle publication.

### `CONSTRUCTOR-LIFECYCLE-ROOT-INTEGER-LITERAL-I0`

Issue `TerminalIntegerLiteralReturnV1 { owner, return_site, value_site, value
}` under the existing scanner/ledger; reserve it in the selected return
consumer; emit one `Const(Integer(value))` and one cleanup-backed
`Return(Some(value))`; retain it through final handoff and lifecycle view.
The existing C schema is reused without revision. Acceptance is selected Pair
`return 30` through C `body-consumer-pending` with no object, exact relation
and physical mutation negatives, plus regressions proving Unit and direct
I64Add stay on their existing paths. Typed integer, Bool, local, field and
Add counterexamples must stop before effects. Keep the near-limit local-commit
owner split before adding retention fields; no new semantic receipt, C schema,
object or EXE claim opens.

Landed: the scanner issues the exact literal site/value relation, selected physical lowering consumes it once without raw return-value descent, final handoff retains a distinct result arm, and V2 reuses the existing I64 pair. Focused literal, Unit and I64 regressions pass; no object is created.

### `CONSTRUCTOR-LIFECYCLE-ROOT-SCALAR-TERMINAL-D1`

Design stop: audit the next scalar arm as a separate finite source relation. Do not promote Bool, local, field, typed integer or general Add through the literal I64 consumer. Name its source issuer, selected physical consumer, exclusive raw edge and fail-fast boundary before implementation.

### `CONSTRUCTOR-LIFECYCLE-ROOT-UNIT-RETURN-D1`

**Prior decision withdrawn by physical-owner audit (2026-09-07).** The source
scan can co-seal an explicit bare-return site, but the selected root cleanup
path does not preserve a value-free MIR return. `build_return_with_port_v1`
selects root-home cleanup, calls `emit_void`, and
`ordinary_new_admission::selected::emit_root_home_exit` unconditionally emits
`Return { value: Some(void_value) }`. The published view and C transport
truthfully project that value-bearing return; they do not erase it. Therefore
`UnitReturn -> existing C pending` was not an executable I0 contract.

### `CONSTRUCTOR-LIFECYCLE-ROOT-UNIT-RETURN-PHYSICAL-D2`

Decision: explicit bare `return` has one source-bound physical payload choice:
`Unit`; the existing direct-I64Add relation remains `Value(ValueId)`. The
canonical physical issuer is the selected root-return dispatch and its
`emit_root_home_exit` consumer. It receives the already co-sealed Completion
and exact return site, then emits the cleanup graph terminating in exactly one
`MirInstruction::Return { value: None }`. This is a physical binding, not a new
semantic receipt.

Source authority + canonical issuer: the same-session AppMain identity,
`VerifiedFunctionCompletionV1`, and exact explicit bare-return site; selected
root-return dispatch converts that source relation once into the root-exit
payload. Non-authority: `ConstValue::Void`, `ValueId`, MIR type/signature,
frame/mode tags, C role strings and process status. C cannot infer Unit from a
value-bearing return.

Fail-fast boundary: missing/foreign completion or source site, implicit/value
return, mixed I64Add/Unit relation, `Some(void)`/missing/duplicate value-free
root return, or drift after finishing rejects before lifecycle publication.
The old selected edge to delete is `Return(None) -> emit_void ->
Return(Some(void))`; generic/nonselected void lowering is outside this delete
set.

Smallest next slice (`CONSTRUCTOR-LIFECYCLE-ROOT-UNIT-RETURN-PHYSICAL-I0`):
make the selected root-exit payload explicit in the existing emitter lifetime,
emit/validate the exact Unit terminator, retain `UnitReturn { owner }` through
final handoff, and make the borrowed lifecycle view return its named
`unit-c-role-unavailable` terminal. It must preserve I64Add byte-for-byte and
must not call C. Positive evidence is selected New plus explicit bare return
through final MIR and the named view terminal; negatives mutate the terminal to
`Some(void)`, duplicate/remove it, or drift completion/site/owner. No
fixture, fallback, ABI inference, object, executable or C claim opens.

I0 landed in this series: source Unit relation, selected cleanup
`Return(None)`, final-handoff retention, and the named `unit-c-role-unavailable`
stop are covered by focused source, final-MIR, terminal-drift, I64 regression,
and pipeline-negative tests. It makes no C call.

### `CONSTRUCTOR-LIFECYCLE-ROOT-UNIT-C-SCHEMA-D3`

Decision: retain V2 layout and revision. The sole schema issuer is
`PublishedLifecycleCFrameV2::populate`, mapping final-view
`UnitReturn`/`I64AddReturn` once into the finite vocabulary:
`BIRTH_UNIT=1`, `ROOT_I64=2`, `ROOT_UNIT=3`; `UNIT=0`, `I64=1`.
`root_unit` is exactly `(ROOT_UNIT, UNIT, arity=0, receiver/object=UINT32_MAX,
flags=1)` with one same-function `Return(None)` control
`(kind=4, operand=UINT32_MAX, mode=0)`. Existing I64 stays `(ROOT_I64, I64)`
and mode 1. C validates this issued row/control relation; it never selects from
names, JSON spelling, `Return(Some(void))`, missing result rows or status.

### `CONSTRUCTOR-LIFECYCLE-ROOT-UNIT-C-SCHEMA-I0`

The I0 deletes the Rust Unit rejection and I64-only root-frame match, adds this
finite C validator before body-site checks, and reaches only
`body-consumer-pending`. It leaves physical-program projection, object/EXE,
ABI revision/layout, codegen, status and compatibility untouched. Positive
Pair Unit/I64 cases reach pending with no object; role/result, return-mode,
operand, root-count and body-site mutations reject before pending. Keep the
745-line transport below its limit by adding a small schema child.

Landed in this implementation row: Rust issues `ROOT_UNIT / UNIT` from the
retained final result, the C header shares the finite vocabulary, and the C
validator requires its exact value-free return control before the pending
consumer. The existing I64 pair remains `ROOT_I64 / I64`; focused Rust and C
preartifact tests cover both pairs and malformed row/control rejection. No
object is created.


### Following tasks and unresolved decisions

| Order within existing Steps 1–4 | Issuer / consumer / deleted loss edge / acceptance |
| --- | --- |
| 1a. Birth object handoff | The bounded I0 above; does not wait for all formal classes. |
| 1b. Formal declaration/use agreement | `CONSTRUCTOR-LIFECYCLE-BIRTH-FORMAL-CONTRACT-I0` issues and retains the exact declaration/use relation in the constructor's existing source loan. It never imports another-session catalog. A later physical-binding row, not I0, consumes `DeferredActualBinding` and deletes positional parameter-kind defaults. Acceptance includes typed-unused, untyped-unused, repeated uses and differing actual classes across New sites. |
| 1c. Root scalar and terminal retention | Extend the existing Home scan's local state to preserve Integer/Bool and exact initializer/binding relations; replace the coarse discarded terminal classification with retained Unit/literal/local/field/Add relations in the existing ledger. The selected terminal consumer emits those relations and removes their raw replay in the same series. Keep logical Facts separate from any Recipe keys. Acceptance covers every listed classifier arm, recursive Add, aliases, uninitialized use, residual reads and finishing drift. Operator/coercion rules must come from reference semantics before this mapping opens. |
| 2. Complete physical binding | Existing emission/finalization owns source-to-value bindings and named synthetic frame/Copy/PHI/Normal-result rules. Seed formals, verify all definition/use/edge relations, remove lost bare projections, and hand off once. Missing coverage remains an artifact blocker; no type inference from metadata. |
| 3. Entry/runtime agreement | Use the existing source-result/process policy linked below; implement checked target/frame agreement and exact cleanup/report/dispose behavior. Runtime InvalidContract/report-failure behavior must be named separately before emission. |
| 4–6. Direct input, execution, retirement | Sentinel contract precedes one complete direct parser input and JSON reparse deletion. Then switch the actual host to the dedicated physical consumer; require unchanged EXE30 and OBJ-linked EXE30 plus Fault/cleanup evidence and delete selected old edges. Parser-only success never closes this gate. |

Formal declaration and use are different: storing a parameter into an i64 field
requires an integer check or an existing exact proof; it does not declare the
parameter i64. Unannotated declarations remain unconstrained at source (the
general parameter contract currently projects `OpaqueHandle`). No first-caller
literal specialization or silent i64 default is allowed. Task 1b must choose a
supported tagged/checked physical boundary or explicit execution-unavailable
disposition, preserving source validity and evaluation obligations for unused
arguments. A missing backend capability is never itself a source type error.
See [types](../../../../reference/language/types.md) and
[lifecycle](../../../../reference/language/lifecycle.md).

The root exit policy already exists in
[function exit and entry result](../../../../reference/language/function-exit-and-entry-result.md):
Unit maps to status 0, Integer 0..255 to its value, out-of-range Integer and
Bool process results to their specified Fault, final program Fault to status
70. Source Bool remains Bool; its process-result Fault is not source rejection.
The physical entry transports the source result and cannot invent conversions.
HomeRelease/Reclaim and selected store-use retention are already landed; do
not reopen those closed censuses. Broader result/formal coverage remains an
explicit obligation and is not discarded to obtain a smaller Pair-only goal.

### Remaining order

After this I0, resume the existing Ordered tasks in this document. Complete
remaining source representations/origins (including Birth receiver/formal
classes and uncovered result arms), then physical binding and final handoff,
then explicit target/runtime entry and status agreement. Project that complete
input into the direct parser before implementing the selected LLVM consumer.
The direct parser is currently a validation-only ingress; the actual host still
uses generic lifecycle body JSON plus the V2 pending companion. Do not call
parser success host cutover or complete program proof.

The LLVM series switches that actual host edge and removes its selected old
route, with unchanged Pair EXE30 and OBJ-linked EXE30 plus Fault/frame/cleanup
acceptance. Caller-zero retirement follows within the same series. Only then
resume canonical/compatibility separation, compile-call state and runtime-hook
single storage. No new second MirBuilder or cross-session main-thunk adapter.

### Post-cutover physical backend investigation

`C-LLVM-COMPILE-SESSION-INPROCESS-D0` is parked after Steps 5 and 6. The C
selected compile terminal currently writes textual LLVM and invokes external
tools (`opt` and `llc`) through `system()`. This is compile-time physical work,
not runtime hot-path dispatch. Before changing it, choose the sole execution
owner between an embedded LLVM API/static distribution and the retained tool
driver, with target/toolchain/link-distribution evidence.

Acceptance is a finite tool-invocation census, an explicit selected C-path
choice, object/diagnostic/temp-cleanup equivalence, and named failure behavior.
Only a chosen successor may delete the C command builders and canonical temp
`.ll` route. `ll_tool_driver.rs` is a separate Rust seam and is not deleted by
implication. This task does not claim runtime speed, concurrent compilation,
or automatic LLVM embedding.

### Feedback reconciliation queue (2026-09-07)

The following audit findings are queued here rather than opened as parallel
authorities. Their order follows the existing constructor lifecycle: source and
physical input first, selected host cutover next, then physical cleanup.

| Order / task | Owner and bounded change | Gate and acceptance |
| --- | --- | --- |
| 4-pre. `CONSTRUCTOR-LIFECYCLE-C-FRAME-SENTINEL-CONTRACT-D0` | Step 4's existing Rust/C physical ABI owner names each absent source ordinal currently encoded as `u32::MAX`, replaces only anonymous literals in that frame, and rejects impossible presence combinations. | Wire bytes stay unchanged; Rust/C boundary cases prove absent versus ordinal semantics. It cannot issue receiver/source meaning or permit LLVM execution. |
| 4. `CONSTRUCTOR-LIFECYCLE-DIRECT-PHYSICAL-INPUT-I0` | Step 4's existing `physical_program_json.rs` owner constructs the final ABI input once and passes it directly to its consumer; delete the intermediate JSON-string `from_str` reparse. | Only after Steps 1–3 and the sentinel contract have issued the complete input. Actual issued Pair input and variant/range/coverage/target negatives pass the same parser. This is parser evidence, never host cutover. |
| 6a. `PUBLISHED-BACKEND-VIEW-PLACEMENT-R0` | After the selected reader is stable and old wrapper callers are zero, move the two `#[path]`-mounted transport implementation files under their owning published-view module and remove the mounts. | One physical module tree, direct/manifest/aggregate/directory caller census zero for the old paths, and unchanged selected acceptance. This is placement only, not a second view authority. |
| 6b. `NORMAL-CALLABLE-SEMANTIC-README-R0` | After constructor lifecycle closeout, condense dated journal material in the semantic package README into current authority, boundary and landed-evidence sections. | Preserve every live contract link and current decision; no semantic or production-edge change. |
| post-6. `C-LLVM-COMPILE-SESSION-INPROCESS-D0` | The parked in-process investigation above owns the C `system()` tool route. | It begins only after Steps 5–6; choice and equivalence acceptance above are required before deleting tool invocations. |

No separate task is opened for a claimed generic JSON hot path: the audited
reparse is the Step-4 physical-input construction above. The `#[path]` files
are one implementation mounted from two module locations during migration, not
two semantic issuers. The README and placement cleanups wait for their live
consumer/contract to stabilize, so they cannot obscure constructor cutover.
