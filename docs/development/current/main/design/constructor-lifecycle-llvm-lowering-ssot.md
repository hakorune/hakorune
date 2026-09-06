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
Smallest next slice: `CONSTRUCTOR-LIFECYCLE-SOURCE-REQUIREMENTS-D0`, an exact source-issuer inventory for the connected retention task below.
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
| Root HomeRelease | Root completion retains terminal homes; `prepare_root_home_exit` checks the exact exit before selected emission records it. | **Partial / CutoverBlockerOpen.** Home binding and exit obligation exist, but each emitted release lacks a retained typed origin row. |
| Root formals and arbitrary root values | Selected root source/completion reaches raw callable lowering and final view. | **Missing / CutoverBlockerOpen.** Pair's zero-formal witness cannot narrow this domain. D1 must name their semantic-class issuer. |
| New allocation | `OrdinaryNewAdmissionClaimV1` retains exact New site, object, destination, Home prefix and construction; the package port takes the same owned site. | **Present.** The allocation identity is source-issued; representation of its result is still missing. |
| Birth formals and result | `BirthAbiHandoffV1` issues owner, receiver/parameter bindings and physical lane order with Unit result; claim take transfers it to root/Birth handoff. | **Partial / CutoverBlockerOpen.** Lane order and Unit are not semantic representation for receiver or each parameter. |
| Birth FieldSet | `ConstructionPlanV1` retains each `ResolvedAssignmentSourceV1` with canonical field; selected construction/body lowering consumes it. | **Present source obligation; physical handoff missing.** D1 must not create a duplicate store issuer. |
| ReclaimUnpublished | `ConstructionPlanV1` retains required outer-storage reclaim and constructor identity; selected construction cleanup emits the operation. | **Partial / CutoverBlockerOpen.** The failed-construction cutpoint has no source-issued operation-origin row. |
| Copy, PHI, frame, InvokeNormalResult, branch/edge values | No selected source requirement issuer exists; the builder creates or binds them. | **Missing as source products.** Later physical binding may admit them only through completed input relations or named emitter rules, never metadata inference. |

The finite state vocabulary is `Present | Partial | Missing`. `Present` means
one existing issuer and named consumer, not backend readiness. `Partial` and
`Missing` are inside this census boundary and therefore
`CutoverBlockerOpen`; they cannot be parked or repaired by C. D1 must decide
one existing issuance owner per missing product, its exact claim-take consumer,
its exclusive old information-loss edge, and its pre-emission negative. If one
is absent, D1 remains `NoSafeSlice`.

The focused field-read command
`mir::normal_callable_semantic_package::ordinary_new_coseal::field_reads::tests::terminal_read_rows_retain_alias_sites_and_commit_only_complete_expression`
still fails with `left: 1`, `right: 0` at
`ordinary_new_field_reads_tests.rs:124`; the identical parent-commit command
reproduces it. It is known baseline debt and does not reopen this BoxShape row.

Step 1's exact source inventory and step 3's final-entry status connection are
explicit remaining design obligations. Do not mislabel this roadmap as a
completed all-SSA issuer. Steps 1–4 close contracts but do not independently
claim production migration; step 5 must remove the selected old production
edge. After step 6, follow the existing workstream order for canonical versus
compatibility dispatch, compile-owned state and single runtime hook storage.
