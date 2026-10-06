# Result-new source-site provenance D0

Status: mixed result origin S0 verified; pending composite result D0 selected
Date: 2026-10-06
Scope: MIRBUILDER-GATE1-MIXED-RESULT-ORIGIN-D0; prior provenance/trace receipts
Related: CURRENT_STATE.toml; RULES.md;
  mirbuilder-artifact-lifecycle-site-diagnostic-s0-2026-10-06.md;
  mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md.

## Decision and selected owner

Diagnostic S0 is verified and published at d877773df7. Next resolve one
unchanged production constructor's exact source-site/claim/prepare mapping.
Selected caller: HakoAllocHeap.reallocResult/2 in mimalloc-lite compile.
Physical observation: block368 instruction4, raw BirthConstructor
HakoAllocHandleResult.birth/3 receiver51 args48/49/50. No EXE was published.
Authority: parser/resolver exact ReturnValue OwnedExprSite, existing result
claim, home_prefix, argument_rows, construction and Birth ABI handoff.
Canonical take/prepare/emitter remain the existing selected result-new owner.
No new source admission, implementation or physical binding registration is
allowed before this mapping is fixed. Design-stop is internal design work,
not a requested goal pause or a reason to mark the goal blocked.

## Facts and counterexamples

The current source has five constructions in reallocResult: return(0,1,null)
at302, return(0,2,null)306, return(0,3,null)310, return(0,4,null)315,
and return(1,0,replacement)318. Physical args alone do not identify which.
Current result fixture census expects prefix(6,2), not historical(8,0).
The later two exits require an exact ownership relation for Body(3)'s local
me.realloc result. NullableForwarded identity must not mint fresh ownership.
Structured child port already forwards result take/prepare/emit; generic
forwarding loss is not established. Source-complete excludes retained rows,
but NoSelectedLocalNew can also reach artifact coverage; inspect actual state.
Worker ruled out a scalar-only class gate: ordinary_box_is_covered delegates
parser coverage name membership, also required by result claim issuance.
Removing that gate has no evidence and would not resolve the mapping.

## Next concrete observation

Determine in production: class coverage membership; exact owner/source site;
ledger result presence; take success and selected/retained prepare status.
Existing MIR compile trace does not show these (fresh enabled trace log:
/tmp/hako-lifecycle-diagnostic-trace.log). BasicBlock carries instruction_spans
and terminator_span; these can help localize source but are not semantic proof.
If further observation code is needed, select a diagnostic-only slice using
existing trace/error owners, default off for logs, with no repeated take or
proof reissuance. Bind the observed source site to its original claim before
selecting a semantic fix. Do not repair all five constructors speculatively.

## Acceptance and next mapping

D0 closes when the raw Birth edge is tied to one exact ReturnValue site and
its actual claim/take/prepare state, with a bounded corrective Decision.
If claim is issued but transport misses it, connect existing selected emitter.
If prepare is unavailable, resolve that precise missing authority; preserve
rejection until then. Never register raw Birth post-hoc or retry old emission.
Positive/negative candidates: return_position_new_mints_result_claim_and_construction_relation,
nested_if_return_position_new_claims_cover_each_exit, page_heap_fixture_result_claim_census,
artifact_child_accepts_return_position_new_under_its_result_claim; wrong site/class/arity,
field/assignment position, unavailable prefix/actual, foreign source remain rejected.
Stored-child sibling, branch/loop family, Gates2–4 remain outside this selection.
No app PASS or whole MirBuilder completion claim.

## Revalidation

Current-first found a this-change pointer guard failure: workstream row H
was510 characters after the last selection update (limit500). The row is
shortened without dropping the current pointer, proof reference or parked
boundaries. This is not baseline debt; verify the guard after this correction.

## Selected diagnostic execution Decision

Existing diagnostic dump rejects named-array/retained-source-required without
a module; no production acceptance or site mapping is claimed from it. Worker
identified the actual method take owner: RawInvocationChildPort implementation
in raw_ordinary_new_claim.rs, not the package adapter. Same ledger/source loan
is already passed into that scoped port.
Select MIRBUILDER-GATE1-RESULT-NEW-CLAIM-TRACE-S0, an observation-only prerequisite. Reuse
NYASH_MIR_COMPILE_TRACE (default off) at the existing actual take/prepare and
raw constructor emission owners. Observe single existing results; never repeat
a take or reconstruct claims. A private formatter child keeps the parent below
800 lines (currently746); no new semantic registry or carrier.
Correlate function/class/source site and original argument AST with emitted
raw receiver/args; take presence, prefix/argument/construction errors and
prepare selected state expose the existing boundary without changing it.
Acceptance: existing result-new positive/negative filters; trace-on unchanged
production identifies the source/physical edge; trace-off has no new events
and keeps the same failure. Scope/pointer guards, file caps and diff check.
Changed paths: raw_ordinary_new_claim.rs + private result_claim_trace child,
new_expression.rs, ordinary_new_admission.rs, builder README, this card/pointer.
This completes observation selection, not the unresolved semantic mapping.

## Fresh trace mapping (verification pending)

CLI build succeeded quick2m32s. Trace-on unchanged production identifies owner
compilation1/slot38 reallocResult. First3 result sites Body0/1/2.IfThen0.Value
are claimed, all proof errorsNone, preparetrue. Body4.IfThen0.Value is claimed
with PrefixNotCovered(Body3), arguments/construction valid, preparefalse:
source315 (expanded439) return new Result(0,4,null) -> receiver51 args48/49/50.
Body5.Value is claimed with the same prefix error -> source318 final return
new Result(1,0,replacement) -> receiver57 args55/56/42. Final error may select
51 or57 because the function block map iteration is not ordered. Both belong
to the same actual unresolved Body3 result relation. Log:
/tmp/hako-result-new-claim-trace-on.log. Missing class/claim/child forwarding
hypotheses are disproven for these observed sites. No arbitrary raw Birth
registration is justified; fix the exact source result ownership relation.
Formatter-only changes follow the completed CLI build; no meaning changed.
Focused positive/negative and trace-off acceptance remain pending.

## TRACE-S0 closeout and D0 mapping outcome

Focused regression5/5 PASS, nozero selection, log
/tmp/hako-result-new-claim-trace-tests.log (quick4m53s, execution0.02s).
Fresh trace CLI SHA256 c117621255347c1b99bc6c34e0be0ab5eadce90de753d9c5083178f420c5502c.
Trace OFF: no new events, unchanged artifact-unowned-lifecycle-site boundary,
no EXE. Trace ON: same boundary; both rejected raw constructor sites mapped
above. No change to acceptance or affine consumption. Scope/pointer and diff
checks PASS. Raw parent755, private formatter43; other changed Rust owners<800.
Rustfmt changed only new hunks; old three-line result-take condition in
new_expression differs from rustfmt already at HEAD and is left unchanged.
The workstream510-character this-change failure is corrected and guard PASS.
No unclassified or this-change test failure remains.
D0 exact-site/claim/prepare mapping is resolved. The corrective responsibility
is Body3 call-result ownership relation, not missing claim/transport registration.

## Next ownership Decision boundary

Worker confirmed realloc combines Fresh(allocate), ForwardFormal0(resizeInPlace)
and null. The existing result solver rejects that mixture, and current negative
pins intentionally deny realloc's owned-result observation. Ordinary typed
formal is borrowed; allocator release changes policy, not descriptor ownership.
Do not convert class/nullable/forward identity into an acquired Home. A source
result relation must retain exact origin and separate owning-destination authority.
Both failure315 and success318 remain in the required scope; making only the
early failure constructor green cannot close reallocResult.
The source assignment/lifetime law must be grounded before implementing that
relation's consumer. Parked take/share is not implicitly activated.
Reachability shortcut considered and rejected as an existing fix: source catalog
selects all declarations and artifact validates all lifecycle functions. App
closure pruning has no existing owner/API here and would require a separate
explicit root/export/dynamic-dependency contract; it is not chosen merely to
avoid this ownership gap. Next design addresses the actual result/destination
law within current ownership SSOTs, preserving the complete goal.

## MIRBUILDER-GATE1-MIXED-RESULT-ORIGIN-D0 selected

Decision: retain fresh and borrowed-forward result origins in the existing
source result solver before selecting their ownership consumer. Do not relax
the owned-result projection. TRACE-S0 is published at 8915833d44; source-site
mapping is no longer an unresolved dependency.
Source authority + canonical issuer: verified Completion value sites and the
existing OrdinaryNewResultClassClaimDraftV1, sealed local initializer/no-rebind
and selected callee/actual substitution. Reuse its one observation and fixpoint;
no parallel source walk or backend classifier.
Non-authority: nullable representation, declared class, constructor field shape,
pointer equality, raw Birth coverage, or caller reachability.
Fail-fast boundary: a mixed fresh/borrowed relation cannot authorize an owned
received result, field transfer, or artifact lifecycle binding. Unresolved call,
foreign binding, rebind, incomplete exits and unsupported SCC stay rejected.
Smallest next slice: preserve Null/Fresh(class)/ForwardFormal(ordinal) as passive
result evidence and derive the old ownership-eligible class projection from it.
Read-only review must fix the exact output and consumer scope before execution.
Non-claims: no Home acquisition, implicit share, moved-in ordinary formal,
conditional consumption, source syntax extension, or app EXE acceptance.

### Conditional return destination review

Worker review identifies a possible future ClosedCallable exact destination
contract, not an existing implementation authority. ownership.md ordinary
parameter permits another exact boundary contract, but its destination matrix
currently only lends input Home for the call. lifecycle.md requires a single
field ownership commit. Conditional return transfer would therefore require a
normative destination row and one composite completion before publication.
Do not infer that contract from an annotated formal or Birth Provided.
Required counterexamples: surviving input aliases; borrowed-only actual; Fault
or escape after child store; publication before ownership commit; ambiguous
fresh/forward origin. Any future plan must propagate demand through the full
realloc -> reallocResult -> exact available-Home caller chain and handle both
source315 and318. No input Home is consumed merely by recording origin Facts.
This alternative remains unselected pending source result/destination proof;
parked HomeV1 take/share is not reopened.

### Closeout required for this D0

Name one canonical relation owner, its compatible old projections and exact
source-substitution tests. Audit all consumers that assume map membership means
owned result, especially callable_result_classes.contains_key. Select explicit
paths, file-cap split if needed, and positive/negative acceptance; only then
switch work_mode to fast. Full ownership destination remains a named follow-up
requirement, not silently dropped to make the early null return pass.

## MIRBUILDER-GATE1-RESULT-EVALUATOR-BOXSHAPE-S0 execution

Read-only worker confirmed the existing result solver is the sole origin issuer.
Its current evaluate_row is the correct composition owner. Select a behavior-
neutral extraction before adding origin alternatives: move ExitVerdictV1 and
exact evaluate_row into a private ordinary_new_result_class_claim/evaluate.rs.
Keep function arguments, old class map, fixpoint, substitution, coverage filter,
error/rejection behavior and every caller unchanged. Parent currently701 lines;
origin expansion must not use the remaining cap as permission for a large owner.
Source authority remains existing Completion and sealed expression/parameter
facts; no new source interpretation or Home law is issued by this split.
Replacement: the one in-parent evaluate_row definition becomes the one private
child definition; finish calls it directly through a private import.
Files: result_class_claim parent + private evaluator, package owner README,
this card and selection pointer. No other source or app modification.
Acceptance: focused existing result tests cover fresh/null/forward, declared
formal identity and page-heap mixed rejection; nonzero selection, stable scope
and pointer guards, diff check, source caps. No additional semantic tests are
needed for a verbatim private extraction. This closes only BoxShape preparation.
Next semantic slice must preserve mixed origins and exact return sites in the
same issuer/product, keep existing consumers on safe projections, validate every
Fresh coverage arm, and compose actual formal substitution. Both315/318 and the
owning-destination obligation remain open; no new conditional move is selected.

## Mixed-origin D0 integrated Decision

Choose one VerifiedSourceCallableResultFactsV1 product from the existing finish:
exact source outcomes are primary; legacy class rows are derived once from those
outcomes. Keep OrdinaryNewResultClassClaimsV1 as its internal alias if that avoids
changing transport slots. Its get/contains_key/iter expose only the legacy safe
projection; outcomes(key) is an explicitly passive borrowed observation API.
MixedOrigins is not added to the old class enum/map. This preserves every old
membership gate and avoids adding lines to source preparation793/issue797 owners.
No second scanner, solver, fixpoint or mutable external map is introduced.
Caller ReturnValue site is retained through callee composition, and forwarded
formal ordinal is substituted through the exact actual binding. Fresh coverage
is checked before the relation is exposed to dependent callable composition.
All-null may issue NullOnly passive Facts, but cannot appear in old class rows.
Unknown/rebound/foreign input, unsupported formal identities and ungrounded cycle
remain unavailable. Declared class never changes ForwardFormal to Fresh.
Implementation paths: existing result solver/evaluator, private product/origin
vocabulary, ledger empty initialization + borrowed observation API, new focused
result_origin_tests child, package README/card/pointer. Reuse transport slots;
no physical, constructor, parameter-demand or app edit. Parent size after the
selected extraction is600; further scheduling split only if the actual bounded
implementation would otherwise exceed its owner budget.
Acceptance must include real realloc all exact exits, null-only and nested exit
sites, transitive forward/ordinal permutation/declaration order, Fresh coverage,
rebind/foreign/cycle rejection, and mixed/null-only absence from old projections.
Preserve fixture census(6,2) and absence of owned me.realloc observation. Complete
origin observation is a required precursor, not destination or EXE completion.

## BoxShape verification in progress

First focused Cargo attempt ended101 with this-change E0583: parent is installed
using a path attribute, so its child also requires an explicit relative path.
Corrected the child declaration after Cargo/rustc terminated; no second Cargo
was started concurrently. Exact evaluator source-body comparison PASS (only
private visibility/import/module placement differ). Retest uses the same quick
profile/jobs4 plus Cargo timings, log
/tmp/hako-result-evaluator-boxshape-tests-fixed.log. No PASS is claimed yet.

## Focused test discrepancy audit (2026-10-07)

Extracted-source result tests:19 PASS/1 FAIL. Failing pin is
provided_parameter_store_rejects_builtin_and_non_plain_fields, which expects
RichHolder(null) construction to stay FieldContractUnsupported. Do not declare
this baseline solely from the split's exact-body comparison. Parent HEAD code
is undergoing an exact single-test repro; selected parent is backed up and
restored by the command's EXIT trap after Cargo/rustc terminate. No shared main
or other author change is touched.
Read-only worker identifies the existing contract change265e67753a: Provided
child class admits PlainI64/OwnedArrayFields/OwnedObjectFields in
instance_construction.rs; Rich owns Token and has the latter disposition.
Source owned-child issuer still rejects builtin/ArrayBox. Existing nested/self
object/array child pins cover that expansion. If parent reproduces, repair the
stale pin in a separate responsibility: retain ArrHolder builtin rejection and
assert RichHolder verified construction/nested canonical child. Do not change
constructor acceptance or mix that repair into the evaluator extraction.
Cargo timing report for extracted-source test build:
/mnt/workdisk/hako-field-array-get-i64-target/cargo-timings/cargo-timing-20261006T145421.342791963Z.html
One rebuilt unit: nyash-rust lib(test),294.32s; build4m54s, test execution0.02s.
This measures whole crate compilation, not typecheck/codegen/link breakdown or
an optimization improvement. Profile and resource settings remain unchanged.

## RESULT-EVALUATOR-BOXSHAPE-S0 closeout (2026-10-07)

Parent HEAD8915833d44 exact single-test repro FAILS at the same RichHolder
construction assertion: /tmp/hako-result-evaluator-parent-baseline-test.log
(quick4m49s;1 executed). This proves the stale pin is pre-existing, matching
already accepted nested-field construction265e67753a; it is not this-change
regression. Selected extracted parent restored byte-for-byte, cmp PASS.
Extracted-source suite19/20 PASS; the sole baseline failure is classified above.
Required result composition/identity/mixed rejection positives and negatives
PASS. The stale constructor-disposition pin is outside evaluator responsibility
and has a separate concrete repair; it does not prevent this private verbatim
extraction from closing, and it is not counted as a green test or app acceptance.
Initial E0583 was this-change and is corrected; no unresolved this-change failure
or unclassified red remains. Scope/pointer/diff guards PASS after restoration.
Parent601/private child115 lines. The old evaluator body equals the child body
apart from module placement/private visibility/imports. No source acceptance,
owned result, parameter contract, constructor or physical binding changes.
Next repair the stale provided-child test boundary in a separate slice, then
implement the selected same-issuer mixed-origin product. Owning destination,
source315/318 and production EXE remain incomplete.

## MIRBUILDER-GATE1-PROVIDED-CHILD-TEST-BOUNDARY-S0 selected

Decision: correct one stale construction-disposition test after verified parent
reproduction; production acceptance is unchanged. Split published08e11449ad.
Authority:265e67753a's existing declared Provided user-child disposition and
source-owned-child canonical inventory. The test observes that existing plan.
Non-authority: class-name guesses in production, runtime tags, a new constructor
plan, Home demand, widened source admission or fixture workaround.
Replace old uniform Err expectation with exact builtin ArrHolder rejection and
RichHolder construction Ok plus one Object child equal to the canonical Rich
identity from this package's instance-constructor catalog. Keep both source
fixtures unchanged. Rename the stale pin to describe the current boundary and
update its existing stable guard reference; fix the exact stale child-kind
owner comment. No separate guard/test harness, source rewrite or backend edit.
Paths: ordinary_new_result_claim_tests.rs, ordinary_new_coseal.rs comment,
mirbuilder_qualified_route_lifecycle_scope.inc.sh test pointer, this card and
CURRENT_STATE. Acceptance:20-test result claim cohort including both sides of
this boundary; nonzero, scope/pointer guards, rustfmt/diff check. No app or whole
pipeline PASS claim. After closeout select the mixed-origin product execution.

Test-boundary first build caught this-change E0599: result claims deliberately
have no children accessor. Use the existing ledger canonical object inventory
instead; no production API is added for this test. Cargo ended101 before the
correction/retest. This is corrected this-change, not baseline debt.

## PROVIDED-CHILD-TEST-BOUNDARY-S0 closeout

Corrected-source result claim cohort20/20 PASS, nozero; quick4m48s, execution0.02s.
Log: /tmp/hako-provided-child-test-boundary-tests-fixed.log.
Read-only review confirms exact builtin rejection is strengthened and admitted
nested child retains one canonical Rich identity. No API or production source
admission change. First E0599 is corrected; no this-change/unclassified failure
remains. The old baseline assertion is now repaired, not excluded or ignored.
Test owner716, other source owners<800; scope/pointer/diff guards PASS.
No source EXE, Fault teardown or whole-suite completion is claimed by this pin.

## MIRBUILDER-GATE1-MIXED-RESULT-ORIGIN-S0 execution contract

Select the same-issuer product described above. Production caller is the existing
prepare_source_claims -> result_class_draft.finish -> source-cohort/ledger slot;
replace loss of mixed/null-only source outcomes there. Parameter demand and
physical emission remain unchanged. One Completion observation and one result
fixpoint issue exact branded caller ReturnValue sites plus passive origin sets.
Private product/vocabulary owns immutable outcomes and the compatibility view.
Existing get/contains_key/iter/keys read only the old safe class projection;
unknown, mixed and null-only provenance cannot grant its membership. No second
scanner/fixpoint or mutable external map. Preserve old class eligibility rules
through transitive calls: neither observing NullOnly nor mapping a borrowed
formal identity may invent a newly acquired Home/class projection. Fresh source
coverage is checked before facts are available to dependent composition.
Origin composition uses exact callee/formal actual binding and no-rebind proof;
callee return sites never replace the caller's own branded exit sites.

Paths fixed: ordinary_new_result_class_claim.rs + private evaluate/product/origin
children, ordinary_new_ledger.rs empty initialization + read API, new private
result-origin test owner and its existing parent registration, package README,
this card/pointer. Existing source prepare/issue tuples retain their single slot
without growth in793/797-line owners. No app, parameter, constructor or C change.
Acceptance: current20 result pins remain green; focused origin positives cover
real realloc all exits, null-only, nested exits, multistage forward and formal
ordinal permutation/declaration order. Negatives cover unresolved/foreign source
or target, rebind, fresh coverage and ungrounded recursion. Assert mixed/null-only
facts are absent from old get/contains_key and owned receiver observations;
keep page-heap census(6,2). Required scope/pointer guards and file caps.
Unchanged production observation must retain the actual315/318 ownership frontier;
no app success claim from passive source outcomes. Conditional destination law
and its alias/Fault/publication/available-Home caller chain remain subsequent
mandatory work; no implicit share or ordinary-formal ownership exception.

## MIXED-RESULT-ORIGIN-S0 implementation verification in progress

One immutable source product now retains each branded return and its passive
Null/Fresh/ForwardFormal alternatives; old lookup APIs expose only the safe
projection. The same fold checks fresh coverage before dependent composition,
exact selected target and source owner before publication. No Home or backend
capability is issued. Real realloc pins map all eight exact source exit paths
to their own alternatives, including both forwarded resize returns and the
fresh replacement return; guarded nullness is not strengthened by this slice.

Scope extends by two empty-product initializers in existing
ordinary_new_borrowed_formal_source_tests.rs: the old BTreeMap construction is
incompatible with the replacement product type. No production borrowed-formal
contract change. First focused invocation used invalid Cargo filter syntax;
corrected command then exposed these two E0308 this-change failures. Both
initializers are corrected. Test-inclusive quick check is running before the
focused executable build; no PASS or closeout claim until actual results.

The first test-inclusive check caught this-change E0433 in the initializer
qualification (test module depth differs from the production owner). Replace
that qualification with inferred Default::default() at the existing typed call
sites; no new visibility/export is needed. Corrected check is running. Scope
guard PASS and unchanged app source SHA remains33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685.
Read-only review confirms exact per-site pins, legacy projection and source
identity guards; no implementation blocker. Future conditional destination
contract remains separate and must preserve ordinary borrowed entry semantics.

Corrected test-inclusive check PASS: cargo check --profile quick --tests,
quick1m26s, /tmp/hako-mixed-origin-tests-check-fixed.log. E0308/E0433 are
corrected this-change failures; no warning suppression. Focused test executable
build is running with result_origin_tests, source_brand_tests, existing20-result
cohort and borrowed-formal source tests in one invocation. Existing lifecycle
scope guard gains four origin-test name pins (759 lines); it remains the same
guard, with no new harness or semantic authority. CLI/app observation follows
successful focused results, retaining unchanged source and selected route.

Focused verification PASS:25/25 (existing20 result claims + four source-origin
pins + foreign-owner/target pin), execution0.02s, quick build6m01s;
/tmp/hako-mixed-origin-focused-tests-final.log. The guessed borrowed-formal
module filter matched no additional tests; it is not counted as coverage.
The two changed initializer callers were run by exact test names separately:
2/2 PASS, execution0.01s, /tmp/hako-mixed-origin-borrowed-initializers-tests.log.
No this-change or unclassified red remains. Real fixture census remains(6,2).
CLI build is running; app observation still required for S0 closeout.

## Next required destination design brief (read-only review integrated)

Decision: reject eager body-inferred conditional input consumption. Investigate
one pending composite result contract at the existing source-result/field
construction owners. This is a design candidate, not accepted Home authority.
It supersedes any interpretation of the earlier conditional-return discussion
that silently moves an ordinary input Home at the call boundary.
Source authority + canonical issuer: exact branded return outcomes, constructor
field destinations and Completion; a future verified destination/completion
product must identify the real source outcome, available Home and unique commit.
Non-authority: class annotation, origin-set union, pointer equality, policy
release(handle), ordinary parameter projection, worker proposal or raw MIR tag.
Fail-fast boundary: ownership.md555-564 forbids body-invented consuming inputs;
ordinary typed/unannotated entry remains borrowed. Current OrdinaryNewResultClaim
assumes a fully owned fresh result at Return; it cannot publish an outer object
with a pending borrowed child. No existing receipt represents that state.
Smallest next slice: PENDING-COMPOSITE-RESULT-D0, resolve source result law,
pending child/storage obligations, propagation and exact contextual destination
commit. Clarify reference result/acquisition/publication contracts before any
source or physical expansion. Pending candidate would preserve input Home on
precommit Fault and release only acquired resources/unpublished outer storage.
Non-claims: no automatic move at ordinary call entry/local assignment, no live
pending result transport, no take/share activation or EXE acceptance.

Caller scope includes realloc -> reallocResult315/318 and the real
allocator_facade_box.reallocResult78-89 relay: its formal is unannotated and
borrowed, it reads result.ok, updates counters, and forwards the composite.
A store-only/terminal-tail-only law would not cover that relay. Required
counterexamples: borrowed-only acquisition at final destination; ignored pending
result; escaped/captured/stored incomplete outer; surviving alias after commit;
Fault during facade counters; fresh/borrowed mixed cleanup; double destination;
publication before commit. The fresh alternative's input Home is preserved;
forwarded alternative can only acquire a separately proven available Home at
one explicit destination commit. Neither law is implemented by passive S0.

## MIXED-RESULT-ORIGIN-S0 closeout and selected next design

Fresh CLI quick build PASS3m17s; /tmp/hako-mixed-origin-cli-build.log.
Unchanged mimalloc-lite selected EXE probe exits1 with the same designed
artifact-unowned-lifecycle-site frontier in HakoAllocHeap.reallocResult/2,
block369 instruction3 raw Result birth receiver57. Both source315/318 still
carry PrefixNotCovered(Body3); no EXE exists. This is classified required
frontier observation, not app acceptance or this-change regression.
Log: /tmp/hako-mixed-origin-production-probe.log. Scope/pointer/diff guards and
owned-result source rustfmt check PASS. Passive S0 acceptance is complete;
actual constructor ownership and whole production migration remain incomplete.

Select MIRBUILDER-GATE1-PENDING-COMPOSITE-RESULT-D0 in design_stop; no execution
card yet. Resolve the above source pending-result law and declaration-only demand
conflict using the full facade caller, then seal one Decision and reference
contract before implementation. This is ongoing internal design work, not a
requested goal pause or blocked condition. Gates2-4 and stored-child sibling
remain parked. No optimizer/profile or unrelated cleanup slice is selected.
