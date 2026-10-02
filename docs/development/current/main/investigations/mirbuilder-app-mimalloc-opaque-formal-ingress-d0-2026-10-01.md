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

### S0 whole-argument source proof Decision

The same actual callback currently joins only opaque slots. Before it can lend
ordered continuation arguments, validate every formal/actual ordinal, call owner,
target slot and source argument site against the immutable prepared call. For
nonopaque ExactTrivial parameter lanes, require source Integer or caller-owned
Integer scalar evidence; preserve the existing i64/usize declaration authority
and bit representation, without inventing a numeric range. Bool/Home/unknown
neighbours cannot be accepted merely because all opaque slots are healthy.
Other nonopaque contracts need their existing consumer's proof; keep absence as
a named pending error, not a selected-definition downgrade. Stage the same Err
in the current actual map. The existing installed entry projection and consumed
call lender demand that map, so no new receipt, authority or carrier is added.
Acceptance: mixed literals/scalars, signed I64 and usize lanes, invalid neighbour
domains, nonopaque site/ordinal/formal and foreign scalar drift; real installed
entry must reject a failed mixed incoming. Ordinary nonselected ABI is unchanged.
This closes an actual-proof gap; all four continuation shapes, tagged consumers
and execution acceptance remain required and incomplete.

Whole-argument proof receipt (2026-10-02): borrowed_formal 66/66 and ordinary
co-seal 128/128 PASS, including real installed-entry rejection of a Bool in
the nonopaque I64 slot. Named complete lib observation and final unchanged
baseline verifier both record 8224/127/56, inventory 8407; the sole extra is
the previously recorded nullable receiver ordinary-membership-drift flake.
That exact test passes standalone. All seven new tests pass in the full run;
no baseline failure disappeared and no unclassified/current-change red remains.
Inventory/expected pass count add only those seven owned tests (8225 expected);
the accepted 126-failure receipt/hash is unchanged. Strict verifier is NOT PASS
for this run, and no stable full-baseline receipt is claimed.
Touched Rust rustfmt, diff and pointer PASS. Qualified scope guard still rejects
untouched brand_catalog_tests.rs=961 without waiver. Actual owner 257 and tests
561 lines remain below hard stop. This closes only whole-argument source proof;
caller continuations, carrier activation, physical/C closure and Normal/Fault
execution acceptance remain incomplete.

Nested source-observation Decision (2026-10-02, read-only worker confirmed):
the current observer visits only local/return/discard call roots. Walk exact
direct argument method-call sites inner-first, preserving argument order and
the same prefix-local state. Stage each inner original source row through the
existing callback, never through a new inventory or authority. Installed entry
and Taken lender already demand these rows. Keep an outer opaque CallResult
Unknown; do not recurse into binary/condition descendants or claim executable
continuations. Acceptance: nested/sibling/three-level actuals reach those real
consumers, bad inner incoming stays terminal, unsupported subtree stays missing.
This repairs missing source proof within S0; all four caller continuations and
physical/C activation remain required. No independent sealer extraction is needed.

Nested source-observation receipt: focused borrowed_formal 70/70 and ordinary
co-seal 132/132 PASS. Real installed-entry/Taken-lender tests cover local,
discard and return roots, deeper direct nesting and sibling incoming closure;
an outer opaque CallResult and binary descendants remain rejected. Read-only
worker confirmed exact-site postorder and unchanged prefix/Home state.
Named full lib and unchanged CLI verifier both observe 8228/127/56, inventory
8411; only the recorded nullable receiver ordinary-membership-drift flake is
extra. This time its standalone run also fails with that same known terminal.
No stable baseline PASS is claimed and no current-change/unclassified red was
found. Add only four owned passing inventory rows (8229 expected); accepted
126 failure names/hash unchanged. Rustfmt/diff/pointer PASS; qualified scope
guard still rejects untouched brand_catalog_tests.rs=961 without waiver.
Observer 151, scan 651, actual tests 701 lines. This closes nested actual
observation only; ordered continuation arguments, single terminal affine
handoff, tagged physical/C consumer and Normal/Fault EXE remain owed.
Initial staging was blocked by a managed read-only .git mount. On 2026-10-02,
the user changed the execution permissions; the .git write check and preserved
patch identity check passed. This receipt does not claim carrier activation.

Discard continuation Decision (read-only worker, 2026-10-02): keep the existing
LocalCallObservation and local_calls/path_calls inventory; replace its separate
declaration/destination fields with LocalBinding { declaration, binding } or
Discard. Issue Discard only for an exact MethodCall statement in the selected
borrowed source-I64 profile, using the callback's owned ordered arguments and
existing prior Homes. Never create a local binding or add a Home for discard.
Source Err stays terminal; opaque ordinals retain neutral ordinal/site references.
The same raw lexical port takes one disposition; its common emitter records one
Invoke/projection group and block_stmt discards the returned ValueId. Do not emit
again in statement dispatch. Existing exit covered_calls and site/group guards
own branch attribution; nested calls fold into the outer discard group. Return
uses the separate agreed Lexical root-entry handoff, never this local group.
Map/Handle/Nullable/direct-local consumers must require LocalBinding explicitly;
no unwrap/default binding and no expansion of those discard contracts. Keep
map.rs=783 below hard stop during accessor adaptation; split by responsibility
before any growth would reach 800. No separate speculative sealer/refactor row.
Next implementation closes callback -> relation -> lexical co-seal for all four
shapes, then the shared tagged physical/C consumer. Acceptance fixes one-shot
Invoke/projection, no local install, branch/sibling exit groups, nested grouping,
Normal/Fault cleanup and malformed source/destination/group rejection. This is
a concrete implementation decision only; no new executable acceptance claim.

Ordered argument projection Decision (read-only seam audit, 2026-10-02):
retain opaque_actuals and full ordered_arguments in the same existing pending
call value. Opaque positions carry only BorrowedActual { ordinal, site }; the
original package proof remains the sole formal/domain/lifetime authority.
Nonopaque positions retain proven Integer or caller-owned Integer Scalar in
source order. Entry and Taken lender validate that same staged value's arity,
opaque ordinal/site and nonopaque class/owner; repeated-walk Err wins. Keep
source Err first and final result corroboration at the existing Taken boundary.
Do not invent CallResult domain proof or arm the current physical emitter.
This is the owned projection prerequisite for the agreed four-shape callback
connection, not a new inventory or a local-only completion. Acceptance includes
mixed i64/usize values and real entry/Taken rejection of truncated, moved or
foreign projection references. Literal content is sealed at construction, not
independently re-read from argument sites by these final consumers.

Ordered projection receipt (2026-10-02): final focused 72/72 and ordinary
co-seal 134/134 PASS. Mixed i64/usize tests distinguish exact literals (-7/7)
from caller-owned Scalars; seven projection corruptions are refused by entry
and the real Taken lender, and ordered-only repeated-walk drift stays poisoned.
Read-only final review found no implementation blocker; fields remain within
the same lexical owner. Unchanged baseline verifier reports KNOWN BASELINE,
exit 0: 8231/126/56, inventory 8413, original failure names/hash unchanged.
Inventory adds only the two owned passing tests, never an accepted failure.
Rustfmt/diff/pointer PASS; scope guard still rejects only untouched
brand_catalog_tests.rs=961 without waiver. Actual tests 753, entry tests 552;
actual owner 289, entry owner 268 and flow 563 remain below hard stop. This
closes retained ordered projection only. Four-shape continuation/physical/C,
carrier activation and whole S0/app acceptance remain incomplete.

Next callback connection Decision (read-only worker, 2026-10-02): reuse one
callback with Some(actuals) for current observation/staging and None for demand
of owned ordered arguments. No temporary site map or second source inventory.
The lexical owner demand helper checks source identity, the same staged row
(including Err/drift), then prepared source-I64 result; ignore only the final
contract_corroborated flag before the walk. Ok(None) requires a successfully
prepared source inventory that positively excludes the site. Selected absence
or Err is terminal, never a strict-only retry. Common lexical sealing demands
this projection before the old strict membership predicate; inner calls recurse
through the same callback and retain enclosing live Homes. Keep observation
errors pending without premature activation. Next: helper/callback/sealer, then
all four local/discard/return/nested consumers per the existing Decisions. Final
incoming/result corroboration and tagged physical/C acceptance remain required.

Callback selection parity Decision (2026-10-02, read-only worker): source Err
is demanded only for an exact successfully prepared lexical target whose callee
owner/batch-slot declaration contains OpaqueHandle. Reuse the already-prepared
immutable lexical_source_targets, including existing parameter receiver provenance;
do not run another resolver or gate only on claim-local receivers. Unrelated
strict-I64 calls do not demand a global source error. The projection helper still
returns None only when successful borrowed inventory excludes the exact site.
Callback connection receipt (2026-10-02): broad borrowed_ 115 PASS / 4 ignored,
co-seal 136/136 PASS. Early selected source/actual/result rejection and late
entry/Taken corruption remain separately tested; the two new tests prove
pre-walk corroboration/priority and parameter receiver error-scope parity.
Accepted locals require the exact retained projection; the proved inner opaque
call preserves its nested BorrowedActual and following Home coverage. Other
construction/deeper-subtree/strict-Bool negatives remain unchanged.
Final named full lib is 8233/126/56, inventory 8415; complete failure names match
the original 126 exactly. The unchanged verifier with captured observation is
KNOWN BASELINE exit 0 with the original failure hash. A preceding summary-only
127 comparison failed and is not an accepted receipt; no extra failure identity
is claimed from that incomplete output. Captured final observation has no extra
or missing failure. Inventory adds only the two verified passing tests and
renames the scope test to reflect early rejection; failure receipt is untouched.
Touched Rust formatting, source hard limits, diff and pointer PASS. Qualified
scope guard still stops only on untouched brand_catalog_tests.rs=961 debt,
without waiver. This closes the callback/shared-sealer local+nested connection,
not the whole ingress S0 or physical activation.
Local/nested connection is implemented; discard/return and tagged physical/C
Normal/Fault acceptance remain required within the original whole S0 scope.

Discard source/continuation component receipt (2026-10-02): the same
LocalCallObservation now owns LocalBinding { declaration, binding } or Discard.
The exact LocatedStmt MethodCall issuer demands the original borrowed callback
once; strict-only statement roots stay unselected, and selected Err is terminal.
Scanner retains prior Homes and branch-local path_calls, without installing a
local or adding a Home. Owned Map/Handle/Nullable and direct-local consumers
explicitly require LocalBinding; their receiving-binding contracts are unchanged.
Read-only worker found no ownership/site defect and required per-branch exits:
new 4/4 tests prove actual-slice identity, one-shot take, no destination/Home,
strict exclusion, unsupported-domain rejection and distinct per-exit call groups.
Borrowed regression 119 PASS/4 ignored; co-seal 140/140 PASS. Broad initial
`discard_tests` filter also ran two existing DynamicCarrierMismatch baseline reds.
Named full lib and captured unchanged verifier both report 8236/127/56; their
only extra failure is the pre-existing nullable receiver serialization flake
(ordinary-membership-drift), with no missing baseline failure. Verifier exit 1
is not a green receipt; failure inventory/hash remain unchanged. Only four
verified passing new names update inventory to 8419, expected 8237/126/56.
Pointer/diff checks PASS. Qualified scope guard remains red at untouched
brand_catalog_tests.rs=961. Existing mixed-result tests receive only accessor
adaptation (905 -> 901); no tests/acceptance removed or new scope added there.
map.rs=790 and direct_call_lifecycle.rs=797 require responsibility split before
further growth; source flow=663, scan=669. Return Lexical root-entry handoff,
real tagged physical/C consumers, single Invoke/group and Normal/Fault EXE
acceptance are still owed. This receipt does not close ingress S0 or the goal.

Terminal validation size T0 Decision (2026-10-02, read-only worker confirmed):
before the accepted Return Lexical handoff grows root_call_entry.rs=746, move
only validate_call_entry, resolve_instance_receiver and its test helper into
root_call_entry/validation.rs. Preserve exact source/disposition/MIR and cleanup
checks, Plain/root-instance tolerance, errors and visibility. Parent keeps
record/group selection/rebind/finalize and entry accessors; validation borrows
those existing methods. Register the child in the same scope guard. Mechanically
compare moved bodies and run existing root_call_entry/co-seal and lib baseline.
No terminal argument, carrier or consumer permission changes in this T0.

Terminal validation size T0 receipt: parent 746 -> 484; validation child 270.
Moved-method comparison is unchanged except module paths/format, with no new
semantic permissions, source rows, tests or inventory. Existing binding-group
negative tests 2/2 and co-seal 140/140 PASS. Initial filename-based filter ran
zero tests and is not acceptance. Captured unchanged baseline verifier is
KNOWN BASELINE exit 0: 8237/126/56, inventory 8419 and original failure hash
unchanged. Pointer/format/diff/bash syntax PASS. Qualified scope guard remains
red at untouched brand_catalog_tests.rs=961; child is registered without waiver.
Next: the accepted Return Lexical handoff, then shared tagged physical/C closure.
Return uses the same owned ordered arguments wrapped in TerminalCallArgument's
Lexical arm; Direct/Instance reject that arm. Root source selection precedes the
old expected mark using prepared evidence, while final corroboration and one
Taken lexical row remain package-complete/raw responsibilities. Share receiver
and argument preparation only: the existing root owner emits the outer Invoke
once. Validate ordinal-bound physical values/proofs in the same Call entry,
never instruction count or expected args copied from the actual Invoke.
Ingress S0, carrier activation, Normal/Fault EXE and the overall goal stay open.

Direct Return source/affine: `ef874d20bf` retains ordered arguments/Taken row
and exact exit identity without a local/Home. Its full source/affine receipt,
unchanged baseline and classified guard debt are retained in that commit.
Physical/C activation and this S0 remain open.

Nested Return Decision implemented in `76cae30f54`: original strict sealer,
exact Argument ordinals/same Homes, nonempty real I64 result completion,
canonical whole-tree Ready closure. The physical handoff below remains owed.

Strict result proof repair Decision: return annotations establish the declared
contract but do not verify literal representation. In the existing lexical I64
source predicate, accept Integer literals or exact-I64 formals only; Float,
Bool, String, Null and mixed exits cannot prove this lane. Preserve the existing
callee selection and borrowed result owner. Check both local and nested Return
selection against actual packages; no new result authority or carrier activation.

TypedInteger is not a supported canonical source shape: the existing grammar
rejects typed_integer_suffix before package issuance. Its retained Rust-evidence
projection is not source authority for this selection; no suffix lane is reopened.

Next physical responsibility T0: extract the existing lexical I64 receiver/arg
and Invoke emitters into a private terminal_call child, preserving behavior.
Then the same original Taken lexical row feeds RootCallDisposition::Lexical.
Share receiver/ordered argument preparation only; the existing root Call ingress
emits the outer Invoke once. Retain each ordinal's producer binding, nested
InvokeNormalResult, exact scalar binding and original borrowed lender through
root exit validation/rebind/finalization. Do not derive expected arguments from
actual Invoke or count instructions as arity. Tagged carrier/use closure and
physical/C Normal/Fault execution are required before ingress S0 can close.

Nested Return source/affine: `76cae30f54` retains original strict/borrowed
rows and exact owner/ordinal/Homes; its focused and unchanged-baseline receipts
are in the commit. Strict source rejects nonInteger literals. S0 remains open.

Lexical I64 emitter size T0 and complete receipt: `834243d2e7`; ABI closed.

Physical handoff Decision (read-only worker, same accepted S0): retain an
optional lexical projection tree on the original Call entry, not a semantic
inventory. Integer keeps actual Const; receiver/Scalar keep the state-issued
exact-read witness; CallResult retains original inner Taken row and Invoke/
NormalResult; BorrowedActual retains ordinal/site plus original lender and
physical producer/read. Rebind/finalized handoff preserve this tree; validate
against original sealed arguments and source proofs, never outer Invoke args.
Birth's existing kind/payload expansion is at the final physical boundary, not
a new MIR producer. Reuse that machinery without changing Birth kinds 1/2.
Forwarded must demand borrowed_ordinary_entry_values and exact formal/value
read; these alone do not authorize tags. Install BorrowedTaggedValue only via
selected Ordinary source/full incoming/use coverage. Extend parameter capability
corroboration, published writer and C index/useflow/call/emitter together for
constant versus forwarded tags and ordinary typed-object kind 3. Metadata,
MirType and physical_signature's old OrdinaryScalar default are non-authority.
Missing carrier activation remains terminal until this same S0 is closed.

Physical preparation `49990f8e36` and root Call owner T0 `dd6cebfee3`:
original contracts and complete receipts remain in those commits. No ABI activation.

Original local/Return packet retention Decision and complete receipt:
`0c873be1f5`; finished-coordinate/source-loan Decision and receipt: `480d357c85`.
Original affine rows remain private in the existing handoff; only read-only
coordinate/source loans may be used. Ordinary full-use/C closure remains owed.
Borrowed projection Decision (same S0, read-only worker): add original ordinal/
site/formal and literal/exact-read/entry physical evidence to the existing tree.
Corroboration borrows original Taken-row actuals again; no source row clone.
Forwarding demands original entry correspondence and Copy proof for a distinct
alias ValueId. Production activation stays closed until Return/carrier/full use/
writer/C/Normal-Fault closure; strict emission cannot authorize borrowed payload.
Focused tests cover source Integer/Bool/scalar/typed Home and proof substitution.
Entry receiver, forwarding Copy and tagged execution remain owed.
Borrowed projection: `1a54469222` / receipt `bbf0e8b271`; Copy: `647f4bb33b`.
Return Decision `602408591c`, implementation/complete receipt `fbd781670b`; ABI closed.
Ordinary carrier/full-use, writer/C and Normal/Fault EXE remain owed.
Copy Decision/complete receipt: `861e50aeb2` / `38cd4191ce`; ABI remains closed.

Ordinary carrier Decision/implementation/complete classified receipt:
`18066ef4f8` / `e3e61f8de8`; source/entry/signature preflight and last metadata
commit are live. Unknown/Integer are storage placeholders, not numeric authority.
Pending source stays pending. Writer parameter/actual and strict Invoke remain
closed; all incoming/use/C/Normal-Fault and selected raw-edge retirement are owed.

Next incoming/use Decision (same S0, read-only worker): extend existing
compiled-entry owner after physical program issuance, before ordinary rows publish.
Retain caller/block/instruction coordinates in existing ordinary rows. Loan original
local/Discard/Return/nested packets and entry source/value correspondence through
existing final handoff visitors; do not clone Taken rows or expose the ledger Rc.
Original node/producer/Copy -> held FinishedBindings -> unique actual function
coordinate must agree with source actual ordinal/formal/receiver and Invoke args.
Use ephemeral coordinate sets for both directions: every original selected
incoming has exactly one final Invoke, and every Call to a borrowed callee has
exactly one original incoming. Inspect all Call results/roles before filtering;
Unit/Handle substitution, hidden callers and duplicate identical Calls reject.
From original entry values and proved Copies, inspect every actual operand use:
only original source-proved Copy/borrowed argument positions are allowed. Arithmetic,
conditions, return/store/capture/rebind and unproved receiver uses remain terminal.
Metadata selects inspection, never authority. No new semantic inventory or retry.
Positive/negative: original direct/alias forwarding and four continuations;
missing/extra incoming, foreign site/ordinal/formal, Copy/tag loss, metadata-only
spoof, altered result, and profile-out uses. Normal/Fault execution remains required
later; do not activate tagged payload before final writer/C and full closure.
