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

Finite ingress remains HakoAllocHeap.allocate -> page.allocate, opaque ordinal 0.
After transport S0, select OPAQUE-CHECKED-COMPARE-NORMAL-INTEGER-D0 before task4 stores/Add:
1. Pin original allocate `requested_size > me.block_size` site/binding and later uses.
2. Existing dynamic_operator_contract owner adds Greater/Normal Integer view contract;
   currently Add/Less result contracts do not issue operand refinement. No new Receipt/Seal.
3. Prove rhs logical Integer separately from `usize`/U64Bits storage; high unsigned bits
   cannot select signed icmp. Keep source evaluation/Fault order even if lowering uses Less.
4. Retain tagged formal; lend view only for same binding/value dominated by compare Normal,
   on both true/false successors; reject pre-check/Fault/foreign-owner/rebound loans.
5. Next task4 D0 connects original ArraySet -> ordered Add -> constructor via that view;
   field usize range checks stay at original write, not parameter ingress. ArraySet retains
   old-slot cleanup/Fault duties; object borrowing still needs lifetime/retention contracts.
6. Tests: true/false, nonInteger Fault, -1 late range Fault and prior effects, rebind/dominance.
Cohort immutable-index optimization is separate BoxShape; retain final mutation checks.
This is future operation scope; transport S0 still rejects these uses and stays selected.
Mixed String/call roots and bundle tasks 10-12 remain owed. No app EXE PASS,
platform/selected-C reopening, all-backend parity or MirBuilder completion claim.

Audit receipt: two follow-ups on the same read-only worker resolved the domain,
entry-shape and activation/use-closure questions. Existing entry source counts,
carrier alignment, Birth tagged Copy and C primitive operand rejection inspected.
No Cargo/runtime result is claimed by D0. Next: the selected ingress S0 above.

## Landed implementation ledger

Completed prerequisite T0s and S0 decisions moved to
`mirbuilder-app-mimalloc-opaque-formal-ingress-landed-2026-10-01.md`.
Index (each section retains its full decision text there):

- OPAQUE-LEXICAL-SIZE-T0 — split the 746-line lexical-call owner first.
- OPAQUE-SOURCE-TARGET-T0 — LexicalInstanceCallSourceTargetV1.
- Source-candidate extraction T0 — collect_local_candidates_v1.
- Source-target preparation T0 — complete per-slot Result preparation.
- S0 source-cohort construction — transport-only closures, no carriers.
- S0 pending actual preparation — passive prep until ABI consumer.
- Entry/physical projection size T0 — source_prepare child split.
- S0 entry-source adoption — ordinary borrowed entry projection only.
- S0 scope/entry physical-value correspondence — with_selected_source_scope.
- S0 caller source/result correspondence — strict I64 predicate retained.
- S0 source-result preparation timing — map prepared before prefix walks.

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

Terminal validation size T0 Decision/receipt: `5a2ae4d347` retains the original
validation contracts, mechanical comparison and unchanged baseline receipt.
Parent record/group ownership and private validation child stay distinct;
no new carrier/consumer permissions. Qualified scope debt is not waived.
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

Nested Return source/affine contracts and receipts: `76cae30f54`; strict source
rejects nonInteger literals, S0 open. Emitter size T0 receipt: `834243d2e7`; ABI closed.

Original handoff/preparation/Call contracts: `49990f8e36` / `dd6cebfee3`;
packet/source/Copy/Return contracts: `38cd4191ce` and referenced predecessors.
Taken producer/read/Copy loans corroborate source; affine rows remain private,
one ValueId expands only at final ABI, Birth kinds1/2 unchanged, ABI closed.
Ordinary carrier contracts: `18066ef4f8` / `e3e61f8de8`; storage is not authority.
Pending/full-use/writer/C/Normal-Fault/retirement obligations remain open.

Original incoming/entry/Call-coordinate Decisions and receipts: `fc8e403459` /
`4cbbc86d3a`. Original affine nodes/held Boundary remain authority for both-way
Call coverage; all results/continuations, no metadata-only proof or hidden caller.
Full-use/writer/C/Normal-Fault execution and selected retirement remain owed.


Original alias/Copy Decisions and receipts: `30aa4838fc`, `03f9012a04`,
`313e6149f4`, `f94ede096a`. Entry owner retains original LocalProvenance and
same-Boundary Copy loans. Packet-required copies remain mandatory; unused-only
omission requires no residual dst use/definition and cannot satisfy an argument.
Mandatory own-site loans remain distinct from optional omission/Invoke uniqueness.
Full-use/source/writer/C/EXE and actual Loop mapping remain separate obligations.
Full-use consumer/projection Decisions and receipts: `4acebb7ddc` / `ae202f471a`.
Original entry/Copy/Forwarded loans, exact indices, copied edges and module/published
use/definition checks stay authoritative; no metadata-only or scan authority.
Synthetic operand/projection evidence only: production lender still closed.
Projection 4/4, borrowed171/171+4ignored, captured full/verifier8321/126/56,
inventory8503, failures unchanged. Initial count-only127 run is unaccepted drift.
Source-to-publication/writer/C/EXE/retirement remain owed; S0 remains open.
ABI audit: writer Borrowed/Map actual lookup needs callee.has_receiver offset; reuse Birth
kind/payload machinery (Birth kinds 1/2 only). Ordinary kind3 requires live typed handles,
TAGGED forwarding preserves both lanes, BorrowForCall no lease/End; ordinary cannot inherit
Birth tagged stores. Writer741/C preartifact793 need responsibility T0 before growth.
Lib/tests check, pointer/fmt/diff/shell PASS; existing scope red brand_catalog_tests.rs=961.

Writer/C continuation Decision (read-only worker): install writer/C together before
production materialize_with_ledger cutover; JSON-only closure cannot authorize other
MIR consumers. Source-success is construction acceptance, not a prerequisite.
Transport size T0 landed: JSON `1d97a8df8e` (parent599/private child163;
three moved bodies match, JSON9/9 byte-identical), C `d789bc6b69` (parser692,
formal30/Call77; reinlined bytes identical). Existing writer/C/Birth EXE regressions
pass apart from reproduced nullable flake; KNOWN BASELINE8321/126/56, inventory8503.
Existing untouched brand_catalog_tests961 scope debt remains; no waiver or activation.

Original-loan continuation Decision: physical program holds an external module-lifetime
handoff, not owned self-referential source. Existing incoming witnessed coordinates become
one temporary map of original immutable actual slices, consumed into existing ordinary
Call rows; reject residual coordinates. No clone, new receipt, Seal or permanent inventory.
This remains source-lent transport, not Integer operand refinement or production cutover.
Wire Decision: formal borrowed_kind_payload_v1 (ABI reference); exact actual {kind,value}.
Original Integer/Bool/live typed object classes issue numeric kinds1/2/3; Forwarded uses
"tagged" and preserves both lanes. Match exact Call coordinates/result and receiver offset; demand original or current
Borrowed selection and compare final snapshot producers/edges, rejecting carrier erasure.
C kind3 requires canonical new_box or verified receiver producer through Copy, matching
layout and live borrowing; HANDLE/nonzero alone cannot prove it (exclude array/null/nullable).
Closed internal incoming/Copy/forward closure proves the loan; no external raw entry,
ordinary tagged store or callee End. Install writer/C before production ledger cutover,
then real Ordinary source-to-publication and generated Normal/Fault EXE/retirement evidence.
Original source-loan component landed at `d387ad1a82`: original external slices,
residual-map rejection, focused5/borrowed171 and unchanged8321/126/56 baseline.
Production cutover and original-loan source-to-publication acceptance remain owed.

Writer/C Decision and execution receipts: `e8cf74ce03` retains source-codec,
full snapshot/Birth swap negatives, physical Normal/Fault and17 C negatives,
unchanged8323/126/56 failure baseline and classified scope debt. Physical
ABI evidence alone is not source publication/cutover. Outside-scope const_null
definition-index gap remains a separate followup; no silent repair permission.

Production cutover Decision (read-only worker): existing lexical_i64 emitter
uses materialize_with_ledger and value_with_ledger; terminal lexical_return
uses materialize_with_ledger. Original source ledger supplies the same proofs;
local/discard/nested/Return keep ordered evaluation and the existing sole Invoke.
Retain lender-free APIs and their borrowed refusal. Validate original source
publication/domain/continuation and carrier erasure/Integer-Bool Const/Copy drift,
then generated Normal/Fault execution; no new Receipt, authority or range rule.
Construction test review: nested uses strict outer wrap(q:i64), borrowed
inner probe; CallResult into Opaque is not selected. Include Bool scalar and
verified EntryReceiver source positives; generated EXE inputs remain unchanged.
Initial source positives2 failed at artifact-actual-root-source-missing:
return local-call-result/out is OtherTrivial with no terminal relation. Keep
local result production but use existing IntegerLiteral exit in ingress proof;
original typed-local Return handoff is a separate explicit owner followup.
Source-aware final strict gate added: `invoke_borrowed_calls.rs` lends exact
original entry/Call loans to final module verification — ordinal/incoming/
residual coverage; pretransform report retained unchanged.

S0 landed (2026-10-03): focused publication tests 2/2 —
`borrowed_original_source_publishes_all_continuations_and_closed_domains`
(4 continuations × 7 domains: local/discard/return/nested, kinds 1/2/3)
and `borrowed_original_forwarded_copy_and_entry_receiver_publish_without_
producer_drift` (tagged Forwarded and EntryReceiver kind3) — with in-test
source-loan mutation negatives: erased-carrier and changed-producer
(Integer/Bool Const/Copy) programs all reject at emission
(`borrowed-carrier-function-drift`, `borrowed-carrier-projection-drift`).
Real-source JSON EXE: all 30 source-issued inputs pass unchanged through
`hako_llvmc_ffi` OBJ and the real-kernel probe — Normal rc7 and
V4_PROBE_FAULT_AT callee-Birth-store Fault rc70 with exact HOME/RECLAIM
cleanup once (probe site lookup retargeted to `Transport.birth/0`; scratch
unwinds via reclaim_unpublished). Production cutover live: `lexical_i64`
and `lexical_return` emitters use `materialize_with_ledger`/
`value_with_ledger`; lender-free APIs retained per Decision. Caller-zero
retirement: `invoke::check_module` wrapper deleted (no callers; compiler
warned never-used). Baseline verifier: KNOWN BASELINE exit 0 —
8325/126/56, inventory 8507, failure hash unchanged (+2 = the two new
tests). Pointer/diff guards PASS.

Remaining open followups (separate owners, non-blocking): actual Loop
mapping, const_null definition-index gap, original typed-local Return
handoff, selected retirement beyond the deleted wrapper, mixed String/call
roots, and bundle tasks 10-12 (Retained ArraySet / ordered Add /
destination / original app EXE).

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-NORMAL-INTEGER-D0
(existing operation owner; see the Remaining task 4 selection rule above).
