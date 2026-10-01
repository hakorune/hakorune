# mimalloc-lite opaque ordinary-formal ingress D0

Status: accepted ingress construction Decision; S0 source implementation in progress.
Scope: selected source-backed Ordinary instance-call borrowed formal transport.
Related: docs/development/RULES.md; CURRENT_STATE.toml;
  mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md;
  docs/reference/abi/nyrt_c_abi_v0.md.

## Decision

`MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-BORROWED-OPERATIONS-D0` selects
`MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-FORMAL-INGRESS-S0` as task 4's
first construction unit. Use one structured borrowed tagged carrier per
existing source/MIR formal ValueId. Only the final lifecycle ABI owner expands
it into i32 kind + i64 payload, following existing Birth transport. Do not add
MIR parameters, a Map allocation, a new invocation authority or runtime dispatch.
This construction supplies an app prerequisite; it does not complete Set/Add.

Source authority + canonical issuer:
- callable_parameter_contract/issuer.rs retains exact declaration loan,
  parameter ordinal, owner, binding and origin; absent spelling stays OpaqueHandle.
- ordinary_new_lexical_instance_call.rs joins the same exact selected receiver,
  callee and actual sites. Extend this existing package/ledger responsibility
  with borrowed actual/formal relations and complete selected-use coverage.
- SelectedCallableSemanticRefV1::Ordinary in semantic_loan_port/source_scope.rs
  bounds activation. Dynamic, Compatibility, S6C and static exact-I64/Map
  contracts do not obtain this ABI by their names or metadata defaults.

Non-authority: caller literals do not classify the formal Integer; backend
names, MirType, payload bits, Map tag 3 and a passing ignored-formal test do not
prove opaque operations or app completion. No hako annotation workaround.

## Representation and lifetime

One aligned PhysicalCallableLaneCarrierV1::BorrowedTaggedValue descriptor owns
one MIR parameter value; source ordinal and BindingRef remain unchanged.
PreparedCallableEntryValues and install_entry_values keep receiver + source
arity and one binding -> one ValueId. Final LLVM adds the hidden kind lane;
physical_ordinal continues to mean the existing MIR parameter ordinal.

Closed ingress kinds:
- 1: immediate signed I64 payload, including zero and negative values.
- 2: immediate Bool with exactly 0/1 payload.
- 3: borrowed typed-object-store handle, with live canonical object provenance.
Host/boxed handles use a different domain; without a source-co-sealed producer
and lifetime contract they reject. A boxed integer is never tag 1 from its bits.
Float, Text, Map, nullable and unknown BoundValue actuals are not admitted here.
Their future representations require their own selected contract, not repair.

Actuals are source-proved Integer/Bool literals or exact scalar bindings,
live claim-Home/entry-loan typed-object bindings, or a previously selected
borrowed tagged formal/Copy. Check exact caller/callee owner, call/argument
site, ordinal, binding, origin and domain. Forwarding copies both lanes unchanged.
Every incoming caller of a changed internal definition must agree with that
formal ABI; no first-caller specialization, old-symbol raw-I64 alias or adapter.
Unselected/Compatibility ingress cannot call that changed internal definition.

Receipt of a pair is BorrowForCall: no retain, transfer or callee End. Caller
roots remain live through Normal/Fault; existing caller unwind releases owned
Homes once after outcome. Typed-object and host-box domains cannot share lookup
or release authority. Store retention, result forwarding and checked projection
are separate obligations, not consequences of ingress.

## Selected use closure and fail-fast boundary

Before body effects, cover every use of the selected formal and its aliases:
ignored formal, tag-preserving Copy and exact argument forwarding into another
selected borrowed formal are transport uses. Arithmetic, comparison, branch,
field/array store, return, rebind, escape and unknown consumers need their own
checked source/operation contract and reject in this ingress slice.

This is incomplete operation coverage, not a restriction on language semantics.
No raw payload can become an I64 operand merely because its ABI lane is i64.
Prove the whole selected owner/call graph before installing carrier rows; errors
leave no partial signature/ledger rows. Derive source uses from sealed rows;
resolve mutually forwarded bindings by exact finite graph coverage, not by
recursive owner inference or assuming an unverified edge succeeds.

At physical verification, propagate the selected carrier through Copy and
check every resulting use against issued call rows. Phi is initially rejected.
Final publication rejects tagged BinOp/Compare/branch/scalar store/projection,
return and nonselected call before JSON/artifact emission. C admission/index
corroborates TAGGED kinds, exact actual/formal domains and use closure. C's
primitive Add guard is evidence, not the sole semantic proof.

## Ordered implementation

1. **Source co-seal.** Extend the existing ordinary-call ledger with exact
   borrowed formal/actual relations and all-use coverage; keep exact-I64
   predicates and static callable_index/direct_call_lifecycle contracts strict.
   Factor responsibility-specific children before parents grow to 760/800.
2. **Entry adoption.** Lend those rows through the existing ordinary source
   scope/CallableSemanticLoweringState. Join binding with entry ValueId before
   body lowering. Install the selected aligned carrier; conflicting selected
   carriers reject. Do not globally change OpaqueHandle -> OrdinaryScalar,
   S6C common-entry projection or annotation-derived builder_metadata defaults.
   parameter_entry_backend_capability must corroborate the source relation;
   carrier metadata alone cannot authorize execution.
3. **Physical publication.** Reuse one-value tagged machinery for parameter,
   Copy and exact ordinary actual transport; constant-tag and forwarded-tag
   rows are exclusive. Extend the existing published physical view and C
   validator/index/entry/call consumer together. Keep Birth's kinds 1/2 unchanged.
4. **Use verification.** Check selected tagged use closure before publication
   and preserve it in C indexed flow. Unsupported uses remain named failures;
   no retry through untagged ordinary/raw emission.
5. **Acceptance and selected cutover.** Source-backed ignored/forwarded formal
   calls reach MIR -> physical JSON -> OBJ/link/execute. Exercise I64/Bool/live
   typed-object actuals, including equal payload bits with different domains.
   Test malformed Bool, boxed/nonselected domain, consumed root, wrong
   owner/site/ordinal, unsupported actual/use, conflicting carrier and unpaired
   caller negatives. Check callee has no End and caller Normal/Fault cleanup
   is exactly once. Source parameter kind remains OpaqueHandle.

Touched owners: semantic-package ordinary lexical-call/source claim children;
normal_callable_semantic_loan_port/source_scope.rs; lowering-state entry adoption;
common_v2_physical_function_entry_input carrier enum;
parameter_entry_backend_capability.rs; published_backend_view physical input,
use verification and JSON children; lifecycle V2/V4 C admission/index/emission.
Do not add to oversized test parents. Current lexical-call parent is 746 lines,
JSON parent 738: use responsibility-specific children or a separate BoxShape T0
if a behavior-preserving prerequisite split is needed. Source hard limit is 800.

Validation: focused source positives/negatives, physical mutation rejection,
real generated executable Normal/Fault cases, touched-module regression, current
lib baseline verifier, pointer/format/diff and qualified-route guard. Record
existing 961-line guard debt without waiver; do not add regressions to baseline.

Retirement: selected opaque internal definitions and exact incoming calls replace
raw single-I64 opaque transport together. Verify all selected incoming edges,
no raw alias/re-entry and exact one-shot rows. Shared unselected scalar transport
and other consumers remain; physical deletion credit needs an actual caller-zero
helper/edge deletion, not this design's planned replacement.

## Remaining task 4 and non-claims

Finite app ingress remains HakoAllocHeap.allocate -> page.allocate at
page_heap_box.hako:204/208, opaque source parameter ordinal 0. Body(8) ArraySet
(line 93) still requires retained value/replace/Fault contract; ordered
I64 + Dynamic and numeric destination (line 96) remain distinct D0 work.
Existing dynamic Add accepts Dynamic + I64 and its emitter rejects execution;
this ingress does not reverse operands, extract Integer or activate that lane.
Mixed String/call roots and bundle tasks 10-12 remain owed. No app EXE PASS,
platform/selected-C reopening, all-backend parity or MirBuilder completion claim.

Audit receipt: two follow-ups on the same read-only worker resolved the domain,
entry-shape and activation/use-closure questions. Existing entry source counts,
carrier alignment, Birth tagged Copy and C primitive operand rejection inspected.
No Cargo/runtime result is claimed by D0. Next: the selected ingress S0 above.


### Prerequisite — MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-LEXICAL-SIZE-T0

Selected before S0 adds borrowed rows: the 746-line lexical-call owner has
no room for its required claim extension. Move existing parameter/local/
initializer/binding class-provenance methods into a private child; retain
call issuance, exact target selection and affine take in the parent. Only
parent-required method visibility changes to pub(super). Keep the same
universal caller join, depth bound, rejection, source identity and errors.
No new argument kind, carrier, receipt, admission or physical behavior.
Validation: exact moved-code comparison, existing lexical-instance-call tests,
current lib baseline, format/diff/pointer; scope guard's known 961-line debt
remains visible. Register the child and move relevant source pins together.
Next after this T0: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-FORMAL-INGRESS-S0.

T0 receipt (2026-10-01): parent 746 -> 359 lines; private provenance child
392 lines. Exact moved-method/depth comparison unchanged except two
parent-required pub(super) methods; no new acceptance or test inventory.
Focused lexical_instance_call 15/15 PASS; lib verifier KNOWN BASELINE exit 0:
8151 passed / 126 failed / 56 ignored, inventory 8333, failure hash unchanged.
Touched rustfmt, bash -n, diff and pointer PASS. Qualified-route scope guard
still rejects untouched brand_catalog_tests.rs=961 (pre-existing size debt,
no waiver). T0 closes only responsibility placement, not opaque ingress.
Next selected row: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-FORMAL-INGRESS-S0.

S0 work-in-progress: ordinary lexical owner now has a private structural
borrowed-formal use draft. Exact canonical parameter bindings, direct Copy
aliases and exact method argument sites are covered; tagged ABI/forward target
permission is not issued by this draft. Rebind, capture read/write, nontransport
use, missing parameter and ordinal mismatch reject. Every argument row remains
unresolved until exact selected target/formal, incoming-edge and live-actual
joins complete. Expanded source-use revision: 9/9 PASS (quick, serial). Exact forwarding
join now corroborates the existing lexical disposition against source callee
owner/slot/opaque binding and exact argument site/ordinal. Finite mutual
forwarding requires every callee draft; a missing owner is not assumed valid.
Forward-use join revision: 12/12 PASS. Complete-batch incoming draft now
requires exact selected call rows, source callee/formal identity, Ordinary
caller scope and nonzero incoming coverage. Unresolved same-selector/arity
rows only veto selection; they never prove a target. Incoming-call revision: 14/14 PASS after correcting the selected-key enum
spelling to Cataloged plus InstanceBoxMethod namespace. No package activation, carrier installation,
C consumer or EXE acceptance claimed; S0 remains open. Source draft revisions
are included in the source-cohort construction receipt below.

### Prerequisite — MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-SOURCE-TARGET-T0

Decision: move existing source identity fields into one immutable
LexicalInstanceCallSourceTargetV1 within the same lexical owner; the final
affine disposition keeps that source row and its result authorization.
Existing getter signatures, target selection, issuance and take remain
unchanged. This breaks the preflight/disposition dependency cycle: S0's joins
borrow the source row, without early result, completion or ABI claims.
No new acceptance shape, caller switch or retirement. Preserve pending S0
draft files/tests independently; they are not part of this BoxShape commit.
Validation: exact field/getter equivalence and existing lexical tests; run
pending source tests as regression evidence, without claiming S0 closeout.
Next: finish S0 source receiver preflight -> prefix callback -> final co-seal.

T0 receipt: source fields and getters compare exactly with the prior row;
existing lexical tests 15/15 and pending S0 tests 14/14 PASS (29 total).
Captured complete lib observation: 8165 passed / 126 failed / 56 ignored,
inventory 8347; all 126 failure names/hash match the accepted receipt and all
14 inventory additions belong to pending borrowed-formal tests. Manifest is
not updated by T0. The initial verifier observation had 127 reds without
names; that incomplete transient census is superseded by the captured named
run, not attributed to a specific known flake or used as PASS evidence.
Format, field/getter comparison, pointer and diff PASS. No acceptance shape,
ABI activation or physical deletion claim. Pending S0 code/tests are preserved.

### Source-candidate responsibility extraction T0

Decision: move direct-local candidate discovery into the existing source child
as collect_local_candidates_v1. The current cohort calls it at exactly the old
per-declaration point: preflight, Dynamic exclusion, source loan and error
priority remain unchanged. It produces passive source/constructor identity,
not a live Home, prefix availability or borrowed ABI authorization. S0 will
reuse these exact candidates for receiver preparation and the prefix walk;
this extraction alone does not introduce early cohort discovery or activation.
Validation: moved-body token comparison, existing lexical and ordinary-New
co-seal focused regressions, touched rustfmt and pointer/diff checks. Preserve
uncommitted S0 drafts; commit only the invariant responsibility extraction.

Extraction T0 receipt (2026-10-02): moved candidate-body token comparison
PASS after normalizing rustfmt's redundant closure block/trailing commas;
cohort 702 -> 671 lines, source child 499 -> 561. Lexical 29/29 (including
14 pending S0 tests), ordinary co-seal 78/78 and four exact-name ordering/
prefix regressions PASS. Two initial module-name filters selected zero tests;
they are not acceptance and were replaced by nonzero exact-name runs.
The initial missing child import was a current-change compile failure, fixed
and retested. Touched rustfmt, bash syntax, pointer and diff PASS. Qualified
scope pins pass through to the unchanged brand_catalog_tests.rs=961 hard-limit
debt; the guard is not green and no waiver is added. Full lib baseline was
not rerun for this invariant extraction; no baseline manifest update. Pending
S0 drafts remain separate, and ingress/carrier/C/EXE completion remains open.

### Source-target preparation T0

Decision: prepare each selected/AppMain non-Dynamic slot's existing local
candidates once after prepare_source_claims. Store the complete per-slot Result;
the old prefix position consumes it after its old preflight. Only complete Ok
slots lend passive class facts. Reuse the existing lexical receiver provenance
algorithm with those classes and the same field/result source claims. Preserve
needs order and retain each target-preparation Result until its original final
signature/result position; do not sort targets or publish errors early.
Final disposition consumes the same immutable source row, without re-selecting
receiver class or target. CurrentOwner/me, borrowed admission, ABI activation
and result authority are excluded from this invariant prerequisite. Validate
source/final correspondence and error ordering, existing lexical/co-seal
regressions, full named lib baseline, format/diff/pointer and scope guard.
One read-only worker confirmed the error-order and whole-slot publication
constraints; shared class evidence never proves a live Home or prefix success.

Preparation T0 receipt (2026-10-02): existing receiver-class proof compares
exactly after replacing only the claim-class lookup with complete candidate
class evidence. Read-only review found no source/final correspondence or error
priority regression. Finalization consumes preparation once; its only
production caller is issuer.rs. Parent/source/provenance/issuer-child sizes are
314/128/429/699 lines; no touched Rust file reaches 800. Lexical 29/29 (15
existing + 14 pending S0), ordinary co-seal 78/78 and four exact-name ordering/
prefix pins PASS. Initial private re-export compile errors were current-change
failures, corrected and retested. Full named observations twice:
8164 passed / 127 failed / 56 ignored, inventory 8347. All 126 deterministic
baseline failures remain exact; the only extra red is the previously documented
nullable_receiver_call_serializes_nullable_handle_and_checked_release flake,
with the same ordinary-membership-drift in isolation. The strict full baseline
comparison is FAIL, not PASS; no failure or inventory manifest is changed for
preserved S0 drafts. Touched format, shell syntax, pointer and diff PASS;
qualified scope guard reaches the unchanged 961-line brand_catalog_tests.rs
debt, without waiver. S0 remains open: next use these prepared source rows for
borrowed use/incoming closure and the separate prefix callback, then entry,
physical/C and generated Normal/Fault acceptance. No ABI or deletion credit.


### S0 source-cohort construction (not carrier activation)

Decision: the existing lexical source owner prepares transport-only opaque
formal/Copy/forward closure from its already prepared exact targets before the
prefix walk. It retains the result in the same ordinary ledger; final lexical
issuance corroborates those source rows without re-resolving the target.
Source OpaqueHandle and the existing strict-I64 predicate remain unchanged.
Legitimate UnsupportedUse, Rebound, Captured and annotated Copy are outside the
profile before selection. Alias identity/cardinality corruption, SourceIdentity
and AmbiguousUse are named errors; annotation now has a separate error variant.
Finite forwarding dependencies close before selection. Every remaining
incoming call must match the same Ordinary source cohort, callee, argument
site and ordinal; unresolved or foreign ingress is a named pending error.

Read-only review found no source-only selection/fallback regression. Existing
selected/AppMain non-Dynamic scope agrees with source_scope's Ordinary branch;
S6C has separate completion/port ownership, not a separate source class.
Next: both readiness and verified prefix walks consume these exact profiles,
prove actual domain/root liveness, and propagate pending Err when that profile
is requested. Keep strict-I64 callback unchanged; add a sibling borrowed
argument seal on the existing LocalCallObservation path. Preserve original
variable_ref binding for forwarding; generic Handle/self-rooted tests do not
prove typed-object domain (entry actual must match the verified loan receiver).
Unknown selected actuals reject by name. I64 local-result integration is an
implementation step, not permission to narrow the complete ingress contract.
Do not interpret Err as None or retry strict-I64. Entry adoption,
physical/C transport and generated executable Normal/Fault acceptance remain
required; source preparation alone is not S0 completion or deletion credit.


Source-cohort receipt (2026-10-02): borrowed-formal focused 22/22 PASS
(21 owned source tests plus one existing sibling), ordinary co-seal 85/85 PASS.
Initial negative fixture used absent Main.me.probe and stopped at the earlier
static-target inventory; corrected to the legal instance receiver and retested.
Complete named lib observation: 8172 passed / 126 failed / 56 ignored,
inventory 8354. Exact existing failure names/hash unchanged; only the 21 owned
source tests were added to the accepted inventory. No failure is re-baselined. Updated baseline verifier KNOWN BASELINE exit 0.
Touched rustfmt, diff, pointer and bash syntax PASS. Qualified scope guard
continues to fail only at the recorded untouched brand_catalog_tests.rs=961;
no waiver. Source/forward/incoming closure is connected to the production
issuer, but actual liveness, carriers, C/EXE acceptance and cutover remain open.


### S0 pending actual preparation — construction Decision

Keep actual source/domain/liveness preparation passive in the existing ordinary
ledger until the entry/ABI consumer is connected. The same prefix-local flow
observes exact argument bindings: immediate signed I64/Bool, declaration-backed
scalar, live claim Home, exact entry receiver or selected formal/Copy forwarding.
The lexical child corroborates those observations with the complete incoming
source relation and retains named errors. No LocalCallObservation/path-call,
Home transfer, physical carrier or runtime permission is installed by this step.
Missing selected incoming observations remain named pending errors, never None.

Preserve the original readiness guard and its error propagation. A verified
prefix walk alone supplies actual facts when that original lane is selected.
Otherwise borrow explicit exits from the existing function-control verifier and
run the same prefix observer without publishing/replacing Completion or seeds.
This additional source-only observation cannot change readiness or acceptance;
its control/probe failures invalidate every partial actual row for that owner.
Unreachable calls after a return or two terminal branches cannot prove ingress.
The source-only result must be consumed as a complete prerequisite by subsequent
entry adoption; it is not permission to skip a selected failed incoming edge.

Responsibility boundary: the cohort issuer is close to the 760-line split
threshold. Before further entry/ABI additions, move readiness/source-observation
predicate orchestration into a private owner child in a separate invariant T0;
do not expand this construction tranche into carrier activation. Whole S0 still
requires ordinary entry, physical/C use closure, and generated Normal/Fault
acceptance. No app completion or physical deletion credit from passive facts.

Pending-actual receipt (2026-10-02): focused borrowed-formal 36/36 and ordinary
co-seal 99/99 PASS, including 14 new actual-preparation tests. Exact scalar,
signed I64/Bool domains, live Home, entry receiver, copied formal forwarding,
consumed/unsupported actuals, ordinal drift and missing nested incoming covered.
The source-only fixture asserts the original plain Completion has no Home flow;
the existing verified forwarding fixture asserts pending actuals arm no local
lifecycle call. Unreachable root/both-branch returns reject at the earlier
NonTerminalReturn header boundary; those are not evidence of an actual callback.
A failed partial source walk overwrites every selected incoming row for its owner.
Read-only review found no readiness/Completion selection or carrier activation
change. Parent issuer 756 lines; new observer/source/test children 107/225/291.

Initial focused failures were current-change test assumptions: absent bridge
incoming, expecting package acceptance of nonterminal returns, and assuming a
verified fixture used the source-only path. Corrected fixtures and the distinct
source-only assertion all pass; none were added to the failure baseline.
The first full observation had 127 reds and isolated the already documented
nullable_receiver_call_serializes_nullable_handle_and_checked_release flake
(ordinary-membership-drift; also failed singly). The named repeat and updated
baseline verifier both confirm 8186 passed / 126 failed / 56 ignored, inventory
8368, unchanged accepted failure names/hash. Only 14 owned test inventory rows
were added; no failure or waiver was added. Rustfmt on 15 touched Rust files,
pointer, diff and shell syntax PASS. Qualified scope guard still rejects the
unchanged brand_catalog_tests.rs=961 debt; not green. Next: invariant cohort
predicate orchestration split if entry work touches this near-limit parent,
then Ordinary entry adoption + physical/C closure + real Normal/Fault acceptance.
Pending actual preparation is landed construction only; S0 remains open.

### Prerequisite — entry and physical projection size T0

Decision: before connecting Ordinary borrowed entry/publication, move the
existing source-state constructor family into a private source_prepare child,
and the existing final-view lifecycle program issuance method into a private
physical_program_projection child. Lowering state is 784 lines; physical
program is already 806. Keep source owner/navigation, origin, entry shape,
function selection/order, error precedence, validation and all method callers
unchanged. Only relative sibling paths and equivalent method visibility adapt
to the private child placement. No borrowed carrier activation or new receipt.
The 756-line cohort issuer does not need to be touched for the selected entry
connection; its optional predicate split is not selected by this T0.
Validation: exact moved-body comparison, existing state/physical projection and
borrowed-formal/co-seal tests, named lib baseline, source size/format/pointer and
scope pins. Register parent and both children; keep 961-line known guard debt.
Next: existing Ordinary scope/entry install consumes complete source/actual
relations, preserves one formal ValueId, and retains source correspondence at
the existing final artifact boundary before JSON/C consumer cutover.


Entry/physical size T0 receipt (2026-10-02): moved bodies match HEAD exactly
apart from relative sibling paths and equivalent constructor visibility.
Lowering state is 558 + 232 lines; physical program is 600 + 199 lines.
Borrowed-formal 36/36, ordinary co-seal 99/99 and constructor projection 2/2
PASS; constructor relation projection guard PASS with its source pin moved.
Physical-program batch is 26 PASS plus the previously recorded nullable-call
flake at ordinary-membership-drift. Initial strict full verifier also observed
127 reds. The subsequent named observation verifies 8186/126/56, inventory
8368, with no inventory or failure-name differences and unchanged failure hash;
strict compare_observation PASS. Neither failure nor manifest was changed.
Scope guard remains red only for untouched brand_catalog_tests.rs=961. The
installed-callable consumer guard already demands absent constructor_count in
HEAD; left unchanged as unrelated stale-pin debt. Pointer, shell syntax, moved
body comparison and diff check PASS. This T0 changes no semantic acceptance,
entry carrier, production caller or physical ABI. Next remains selected
Ordinary entry/caller adoption and retained-source physical/C corroboration.


### S0 selected entry-source adoption Decision

The existing ordinary lexical-call ledger lends a borrowed entry projection
only to an installed selected Ordinary instance-method input with source
OpaqueHandle formals. Static/S6C, unrelated/nonopaque or Dynamic input does not
demand pending borrowed source. A demanded input must belong to the existing
retained Completion owner index; a foreign installed source loan rejects.
A legitimate outside-profile definition stays unselected; a preparation Err
for a demanded opaque input is a named terminal, never outside/None fallback.
Before lending, corroborate exact source owner/ordinal/formal identity and every
incoming actual row in the selected finite transport cohort, including callers
other than the first visited caller. Missing, failed, truncated, reordered or
foreign actuals reject before an entry carrier can be installed. Borrow the
existing source/use/actual products; do not mint a second semantic authority.
This construction step prepares the entry consumer seam. It does not activate
metadata/carriers, change LocalCallObservation or claim runtime cutover. The
following consumer connection must retain this source projection through the
existing final artifact handoff and close all supported incoming positions.
Acceptance: installed package source loan plus positive mixed domains and Copy
forwarding; missing/failed later caller and ordinal/formal/source corruption;
nonopaque input does not demand unrelated preparation Err.


Entry-source read-only review confirms whole-cohort actual validation, Ordinary
instance-only demand and compilation-branded Completion membership. Next scope
adopter keeps the existing ledger Rc, copies only ordinal/BindingRef descriptors
into lowering state, and maps the same PreparedCallableEntryValues parameter
ValueIds. Carrier indexes include the receiver offset. Final artifact retention
copies validated source + actual + entry/call commit correspondence at the
existing root handoff boundary; no ledger Rc or borrowed accessor outlives scope.
Caller coverage must account for local-initializer, standalone, return and
nested-argument positions. Standalone calls have no existing local continuation
row: represent discard explicitly in the existing call owner, never synthesize
a local destination or claim an already implemented emitter. Nested inputs
currently retain named unobserved/unsupported facts and need exact inner
source/result evidence. New borrowed permission must use explicit result cohort
and source return proof, never the old empty-terminal I64 default by itself.


Entry-source construction receipt (2026-10-02): borrowed-formal focused 46/46
and ordinary co-seal regression 109/109 PASS, including ten new entry tests.
An installed Ordinary loan covers signed Integer, Bool and live typed-object
actuals from separate callers without changing the source OpaqueHandle kind.
Copy forwarding, failed later caller, missing/truncated actuals, exact ordinal/
site/foreign formal corruption, source formal cardinality, foreign installed
loan, static/nonopaque unrelated-error isolation and outside-profile selection
are covered. Source child 176 lines, test child 375; scope size list includes
both. No new unused import or suppression was retained.

Initial compilation errors were missing local imports and a test constructor
API spelling; fixed. Initial mixed-domain positive incorrectly assumed a Home
remained proven after an unarmed call made its prefix unknown. Separate real
caller bodies fix the fixture without weakening source liveness. These are
current-change failures resolved by the focused run, not baseline additions.
Named complete lib receipt: 8196 passed / 126 failed / 56 ignored, inventory
8378; all original failure names/hash unchanged. Updated only the ten owned
passing-test inventory rows/count/hash. Updated compare_observation PASS.
Direct verifier also observed 8195/127/56; a name-preserving execution of the
unchanged verifier confirms the sole extra name is the already recorded
nullable_receiver_call_serializes_nullable_handle_and_checked_release flake
(ordinary-membership-drift), with no missing accepted failures. That run is
not PASS and the flake was not added to the accepted failure receipt. Exact
126-red named receipt remains the accepted evidence; no baseline waiver.
Rustfmt check, pointer, shell syntax and diff PASS. Qualified scope guard
continues to reject only the untouched 961-line brand_catalog_tests debt.

This API is the selected entry consumer seam, not production carrier adoption:
production entry has not called it yet, caller continuations/retained physical
rows/C admission and generated Normal/Fault acceptance remain open. Whole S0
and the original mimalloc-lite bundle remain incomplete. Next: connect the
Ordinary scope/entry descriptors and caller continuation coverage to the same
retained source owner before enabling its physical ABI.


### S0 scope/entry physical-value correspondence Decision

Wire only with_selected_source_scope: project the existing accessor outcome to
owned ordinal/BindingRef rows and stage it on the created callable state before
its body callback. Keep generic/Compatibility scope signatures untouched.
Pending source Err remains Err in the same ledger until an actual borrowed
consumer demands it; never turn it into outside/None or activate a raw ABI as
its substitute. This construction tranche changes no current carrier policy.
At existing install_entry_values, validate the source/entry correspondence and
record the same existing formal ValueIds under the exact source owner. Exclude
the receiver from logical formals; reject duplicate/foreign/changed ordinals,
receiver-value collisions, wrong arity or repeated entry recording. Borrowed
mapping is recorded only after normal entry materialization succeeds, so a
failed install cannot leave successful physical correspondence. Source facts
and complete actual proof stay in the existing ledger Rc across scope lifetime.
Acceptance: installed package -> selected scope/state -> real existing entry
values -> same ledger mapping; pending failed actual preserved; foreign/missing
source, ordinal/binding/value drift and duplicate preparation/record rejected.
No metadata/carrier activation, caller ABI switch, final source retention or
C/EXE completion from this correspondence alone.

Scope/entry correspondence receipt (2026-10-02): real selected source scope
stages the existing source/actual projection before entry adoption. Eight state
tests cover existing signed/Bool/live Home values, pending failed actual,
receiver and nonopaque-parameter collisions, ordinal/binding drift, truncated
formals rejected before state mutation, duplicate staging/recording and explicit
unselected preparation. Two actual scope tests pass through the existing
ModuleLoweringInvocation and adopt_callable_entry_values_v1: formal ordinal 0
retains ValueId 72, receiver 51 stays separate, params stay unchanged and no
physical carrier is activated. Failed actual evidence stays Err after the same
scope closes. A ledger negative proves another callee's failed incoming cannot
be bypassed by recording healthy-owner entry values. Shared full-cohort validation
is preflighted before entry state mutation and checked again at recording.

Focused borrowed_entry 9/9 (eight new state tests plus one existing Map negative),
borrowed_formal 48/48, selected_scope 2/2, ordinary co-seal 110/110 PASS.
Complete named lib: 8207 passed / 126 failed / 56 ignored; inventory 8389,
all failure names/hash unchanged. Added only eleven owned passing inventory rows;
unchanged baseline verifier reports KNOWN BASELINE exit 0. Initial Box conversion
and test arity-type compile errors were corrected; no current-change failure is
accepted into baseline. Rustfmt/diff/bash/pointer PASS. Qualified scope guard
still rejects the unchanged brand_catalog_tests.rs=961 debt; no waiver.
Read-only worker checked pending-error semantics, all-parameter collision and
preflight atomicity. Repeated-owner entry recording remains a named failure.

Entry source/value correspondence is connected; selected carrier activation,
caller continuation coverage, physical/C publication and Normal/Fault EXE are
still incomplete. No production ABI switch, retirement or S0 completion claim.
Next construction series uses the existing lexical local-call owner, exact
pending actual rows and explicit source/result I64 proof; Integer-literal probes
must not replace the required exact source-I64 result coverage. Close local,
statement-discard, return and nested incoming consumers in their existing owners
before activating a changed callee ABI. Do not create a fake local for a discarded
call, leave an unconsumed variant as finished work, use an empty-terminal I64
default as result permission, or retry raw transport after selected failure.
Final physical consumer must compare retained entry values with real params.

### S0 caller source/result correspondence Decision

Keep the existing strict lexical-I64 predicate unchanged. The same lexical
ledger caches one pending source-result projection per selected borrowed owner:
all verified explicit value-return sites must be Integer or that callee's exact
I64 formal. Bool/Text/opaque returns and absent/implicit value returns do not
receive I64 permission. Exact parameter declarations, owner and source ordinal
are corroborated, not inferred from MIR. Existing final result cohort then
corroborates this projection; errors remain pending until borrowed consumption.
No empty-terminal I64 default, result annotation alone, or first caller suffices.
The emitter seam borrows the same actual rows using its consumed lexical row;
check owned one-shot disposition, exact call/callee/argument sites, final I64
result and full source/actual cohort. None is outside the selected profile only;
selected missing/corrupt/error rows reject. This correspondence does not activate
caller continuations or carriers. Their complete local/discard/return/nested
coverage and the real physical/C consumer remain required in the same S0 series.
Acceptance: Integer and exact-I64 formal results, mixed formal ordinals,
source-domain mismatch, missing result corroboration, wrong/foreign/unconsumed
call row and actual identity/coverage failures; existing scalar routes unchanged.

Caller source/result correspondence receipt (2026-10-02): borrowed_call 11/11,
borrowed_formal 59/59 and ordinary co-seal 121/121 PASS. Signed/Bool/live Home
actuals lend the original rows after one-shot lexical consumption; exact-I64
formal results with mixed opaque/scalar ordinals are covered. Bool/Text/null
source results, missing or uncorroborated result source, changed return sites,
opaque/nonopaque argument-site drift, target-slot/receiver drift, foreign or
unconsumed rows, result drift and failed actuals reject. Incoming preparation
retains the same immutable full lexical source target; it does not select again.
Final result corroboration compares the complete return-value site set with the
existing completion's explicit exits before marking the pending projection.

Initial compilation exposed only result-cache type visibility; corrected to
package-local visibility. Initial no-return/i64 fixture correctly failed the
existing PhysicalHeader completion boundary before reaching the lender. The test
now asserts that refusal and uses a legal unannotated empty method to prove the
new lender refuses an empty-terminal default. Removed the new unused import;
no suppression or current-change failure was retained.
First full run: 8217/127/56, with only the pre-existing nullable receiver flake
extra (same ordinary-membership-drift). That observation is not a PASS or an
accepted failure-set extension. Separate full named receipt: 8218/126/56,
inventory 8400, unchanged failure names/hash. Added only eleven owned passing
inventory rows; unchanged baseline verifier reports KNOWN BASELINE exit 0.
Rustfmt/diff/bash/pointer PASS. Scope guard still reports only unchanged
brand_catalog_tests.rs=961 debt; new source/test children registered. Result
child 150 lines, tests 234, co-seal issuer stays 756 without line compression.

This lends source/actual/result facts for the real emitter seam; no caller
continuation or tagged ABI has been activated. Whole S0 and app remain open.
Next must project the prefix callback's same ordered actual rows into the
existing continuation owners, with source result preparation available before
that walk rather than cyclically reading the completed ledger. Close every
selected local/discard/return/nested incoming, then physical/C use closure and
Normal/Fault EXE; do not substitute a local-only accepted corpus for that scope.

### S0 source-result preparation timing T0

Decision: prepare the existing owner-keyed source-result Result map immediately
after borrowed ingress preparation, before either prefix walk. Move that same
map into the existing ledger at final installation. This is an invariant timing
refactor: source_result reads only the immutable source loan, declarations and
verified explicit return sites; it does not need caller-flow or final Completion.
Keep per-owner Err pending, contract_corroborated false until the existing final
result cohort checks it, and source Err ahead of result demand. Do not arm a
continuation, change the strict lexical predicate or activate an ABI here.
Acceptance: existing borrowed source/result negatives and full ordinary co-seal
regression; unchanged named lib failure set. Read-only worker confirmed these
dependencies. Subsequent caller connection still closes all four incoming shapes
and the real tagged consumer in the same S0 series.

Next caller connection order (read-only consumer audit, 2026-10-02):
1. Move the probe invocation into the existing source-claim child before adding
   to the 758-line issuer; keep readiness, explicit exits and error staging.
2. Project the same callback actuals into neutral existing call-flow arguments;
   package domain/formal authority stays in the same ledger. Preserve exact
   source-I64 result proof, all argument sites and live roots in both walks.
3. Local/nested use the existing lexical disposition/emitter; nested instructions
   join the outer binding group. Discard needs an explicit destination in the
   existing continuation inventory and per-exit coverage, never a fake local.
4. Resolve terminal affine ownership before emission: current root-instance
   terminal and lexical lender have different Taken rows. Do not consume both
   or pass one as the other's proof. Extend the existing root-exit handoff and
   validation to the same selected source/actual/result row; changing terminal
   arguments alone cannot close this path (old emitter passes empty arguments).
5. Prove all four shape consumers and complete incoming coverage before tagged
   carrier activation; then physical/C closure and Normal/Fault EXE acceptance.

Timing T0 receipt (2026-10-02): ordinary co-seal 121/121 and borrowed_formal
59/59 PASS. Named complete lib observation is 8217/127/56, inventory 8400
unchanged; the sole extra failure is the previously recorded nullable receiver
ordinary-membership-drift flake, with no baseline failure removed. Direct
baseline verifier and a final captured verifier also report summary drift;
final captured failure names confirm that same sole extra. The exact nullable
test passes standalone. These full runs are not PASS and the manifest is not
expanded. No unclassified/current-change failure remains; stable baseline receipt
for this refactor is not claimed. Initial helper visibility compilation errors
were corrected before the passing focused runs.
Private result/lexical child rustfmt checks, diff and pointer PASS. Qualified
scope guard still rejects untouched brand_catalog_tests.rs=961, without waiver.
Issuer 758, result child 163 and lexical parent 342 remain below hard stop.
The same pending Result map now precedes both walks and moves into the ledger;
caller continuations, physical/C consumers, ABI activation and S0 remain open.

Probe placement T0 Decision: move only the readiness/source-actual probe's
predicate invocation into the existing source-claim child. Keep readiness
guards, empty/explicit exit inputs, Completion selection, source-only error
overwrite and drop(probe) in the issuer. Compare the moved body mechanically
and run existing co-seal/borrowed regression; do not add a new source authority,
change callback output or activate a continuation in this invariant slice.

Terminal ownership Decision (same read-only worker): add a Lexical arm to the
existing RootCallDisposition that retains the same Taken lexical row, not a
second root-instance row. For each exact selected borrowed terminal, corroborate
the prepared incoming source before the old root issuer's owner-level expected
mark; unselected Direct/Instance sites keep their existing owner. Prefix callback
must issue the actual terminal relation with sealed arguments and live roots.
Completion proves this call is that explicit return's direct value; raw port
checks selected lexical terminal before any owner-level old-root unavailable
check. Reuse one Invoke/projection group in the existing root exit, never a local
group for this terminal. Final root-entry validation checks the same Taken row,
original actual ordinals/source and real values/tags. Nested instructions belong
to this terminal entry. Source/result/actual Err on selected demand is terminal;
neither opaque spelling alone nor an unproved incoming row grants this path.
This fixes the affine design gap; consumers and dynamic acceptance remain owed.

Probe placement T0 receipt (2026-10-02): moved-body comparison unchanged apart
from child module paths, predicate borrowing and formatting; read-only worker
confirmed source loans, error priority and parent selection are preserved.
Co-seal 121/121 and borrowed_formal 59/59 PASS. First named lib is 8217/127/56
with only the known nullable ordinary-membership-drift extra, standalone PASS.
Separate captured unchanged baseline verifier is KNOWN BASELINE exit 0:
8218/126/56, inventory 8400 and original failure hash unchanged. No manifest edit.
Source-child rustfmt, diff and pointer PASS; qualified guard still reports only
untouched brand_catalog_tests.rs=961 debt. Issuer 707, source child 710 lines.
This closes only probe placement; next is the same source/actual/result callback
to all selected continuation shapes, then real tagged physical/C consumption.
