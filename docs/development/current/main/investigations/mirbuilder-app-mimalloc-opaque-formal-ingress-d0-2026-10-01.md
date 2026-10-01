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
