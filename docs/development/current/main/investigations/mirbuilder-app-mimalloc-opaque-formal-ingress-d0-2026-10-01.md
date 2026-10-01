# mimalloc-lite opaque ordinary-formal ingress D0

Status: accepted ingress construction Decision; implementation not started.
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
