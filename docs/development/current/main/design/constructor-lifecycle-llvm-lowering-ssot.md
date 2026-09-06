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
Smallest next slice: `CONSTRUCTOR-LIFECYCLE-ROOT-BIRTH-REPRESENTATION-D2`, a
design-stop census for the remaining root/Birth value representation rows whose
existing issuer is still partial or absent.
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
selection. One call-local LLVM TargetMachine/DataLayout verifies frame size,
alignment and offsets and emits the object. No host `sizeof`, target default,
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
Step 0 is landed. Source retention remains fenced until the finite issuer
inventory below has a canonical issuer for every `Partial` and `Missing` row;
later rows require their concrete mapping before changing `work_mode` to
`fast`.

| Order | Owner, caller and terminal | Replacement and acceptance |
| --- | --- | --- |
| 0. Owner split (BoxShape, landed) | `instance_constructor_semantic.rs`, `brand_catalog_tests.rs`, `ordinary_new_local_commit.rs`; existing imports/tests and callers | Constructor tests now live in the child test module, the brand test tail is included without changing its logical paths, and root validation lives beside local-commit emission state. The parent files are respectively 539, 715 and 746 lines; all child files are below 760. No public API, test name/body, source contract or lifecycle route changed. |
| 1. Source retention (BoxShape) | existing semantic package issuer -> construction/New/Home plans -> existing selected claim consumer | Retain formal/value requirements and typed operation origins at their existing issue points. Replace source relation loss before claim take and cleanup emission. Acceptance: unchanged Pair and existing renamed/alias/multiple-Home cases, exact unused formal and cleanup origins; foreign/missing/duplicate/source-unavailable negatives before emission. No disconnected source-only receipt. |
| 2. Physical binding and final handoff (BoxShape) | selected root/Birth emission -> existing completion ledger -> final-view admission | Replace bare value/op binding loss with completed physical relations in the existing handoff. Positive exact source-to-final-view coverage; negative finishing drift, residual sibling, mixed/unseeded PHI, Unit result, wrong Normal result and metadata-independent verdict. Preserve one-way transfer and generic lifecycle fence. |
| 3. Target/runtime agreement (BoxShape) | runtime ABI owner + selected compile invocation -> same-target lifecycle session | Replace implicit host-layout assumption with explicit target ABI agreement; no semantic target receipt. Target-compiled Rust/C checks, LLVM size/alignment/offset agreement, unsupported target/revision/runtime mismatch and absent session reject without artifact. All session resources released on failure. |
| 4. Complete direct input (BoxShape) | final-view physical ABI -> existing JSON/parser -> one parsed consumer input | Project completed representations, origin IDs/dictionary and target ABI once; remove serialize-then-reparse and missing-input assumptions. Actual issued JSON positives, full variant/range/coverage/target negatives; parser-only success never claims LLVM execution. |
| 5. LLVM + production cutover (BoxCount) | `published_mir_object.rs` lifecycle caller -> dedicated call-local consumer -> object/EXE | Emit the complete selected function/control/runtime ABI; switch the real host caller and delete its generic lifecycle JSON + V2 pending companion edge in the same series. Fixed source EXE30 and OBJ->linked EXE30, Fault ordering/cleanup, Normal-only out loads, frame lifetime and no partial artifact are mandatory. |
| 6. Caller-zero retirement (Delete) | selected transport/proof owners -> required production acceptance | Audit direct, manifest, aggregate and directory discovery; delete unused V2 wrappers/pending helpers/private probes and the legacy proof route. Retain shared helpers only with named live callers. No selected old retry/re-entry remains. |

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
| Birth parameters | Birth handoff owns bindings/lanes/Unit; constructor stores identify i64 use only for admitted parameter RHS. Unused or untyped formals have only `OpaqueHandle`. | **Partial / NoIssuer.** Add an i64 use requirement only where the existing store proves it; untyped unused formals reject rather than infer i64. |
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

Decision: design only; extend no code, fixture, fallback or production switch.

Source authority + canonical issuer: the exact resolver-batch loan in
`issue_ordinary_new_claims_v1` is the named existing issuer. The candidate
relation is an affine, ordered child row co-sealed inside its existing
`OrdinaryNewAdmissionClaimV1`, at minimum `(owner, New site, CallArgument
ordinal, trivial source class/site)`.

Non-authority: `CallerNewHomePrefixV1` as an argument issuer, raw AST/child
scope, names, `MirType`, `ValueId`, physical ABI/C and a new semantic receipt.

Fail-fast boundary: an absent, duplicate, foreign, ordinal-drifted or
nontrivial row rejects before child effects. Until a final terminal consumes
every row exactly once, the selected raw argument loop remains necessary and
cannot be deleted.

Smallest next slice: audit existing source-site consumption primitives and the
final New terminal to name one bounded consumer and its same-series exclusive
raw-replay delete-set, or retain `NoSafeSlice`.

Non-claims: a general root SSA/argument representation, nontrivial arguments,
Birth receiver co-seal, target ABI/C activation, Pair EXE/OBJ30 and production
cutover.

Step 1's exact source inventory and step 3's final-entry status connection are
explicit remaining design obligations. Do not mislabel this roadmap as a
completed all-SSA issuer. Steps 1–4 close contracts but do not independently
claim production migration; step 5 must remove the selected old production
edge. After step 6, follow the existing workstream order for canonical versus
compatibility dispatch, compile-owned state and single runtime hook storage.
