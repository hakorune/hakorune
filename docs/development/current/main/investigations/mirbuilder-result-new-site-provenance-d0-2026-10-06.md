# Result-new source-site provenance D0

Status: child-relation D0 accepted; call witness composition BoxShape S0 selected
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

## PENDING-COMPOSITE-RESULT-D0 Decision (2026-10-07)

Accept completed fresh outer + exact Null/OwnedFresh/BorrowedFromFormal child
relation; reject pending ownership-hole completion as the unchanged-source path.
Evidence: apps/mimalloc-result-contract-proof/main.hako11-12 passes alloc.handle
and same.handle, then22-24 reads alloc/same/moved. Those reads are handles,
not owning roots; field move-out would require a separate parked law and alter
later reads. Facade78-89 supplies an ordinary unannotated formal and contains no
consuming destination. Delaying consumption until local result assignment would
still violate declaration-only parameter demand. Preserve input Home and infer
an anchored result lifetime, not a consuming parameter.

Source authority + canonical issuer: sealed source return outcomes, existing
constructor/field destination and Home-prefix/completion verification. The
language result law is clarified in ownership.md composite anchored results,
and exact construction residence teardown in lifecycle.md Fields and ordering.
These are target semantics only; current owning-only child schema is unchanged.
Non-authority: source origin sets, worker proposal, class-wide payload type,
pointer equality, policy release, local assignment or runtime representation.
Fail-fast boundary: acquired child requires verified Home transfer; borrowed
child requires exact original support and lifetime; neither can impersonate the
other. Complete outer publication and Fault/drop obligations must be verified.
Smallest next slice: MIRBUILDER-GATE1-RESULT-ORIGIN-WITNESS-S0 — retain immutable
source witness DAG in the SAME result Facts draft/fixpoint/product. Multiple
same-origin source paths and formal-derived null provenance must survive.
Non-claims: no input move, field store/cleanup expansion, runtime discriminator,
anchored residence receipt or production EXE from this observation-only slice.

### RESULT-ORIGIN-WITNESS-S0 execution contract

Production caller: prepare_source_claims -> existing result draft.finish ->
ledger product. Replace erased source derivations in that existing product;
keep old class/membership exactly unchanged. Leaf witnesses retain exact owned
return site and NullLiteral/FreshConstruction/Formal(binding,ordinal) kind.
Call edges retain caller return site, actual initializer/direct call site,
callee key and immutable callee witness plus exact argument-site/binding/ordinal
substitution for formal provenance. Share already resolved callee witnesses;
no second scanner or fixpoint. Formal-derived null retains its formal anchor
provenance. Fresh source leaf is not an acquisition/ownership receipt.

Paths: result-class parent and evaluate/product + private witness child,
existing origin test owner and registration/reexports, package README, active
card/pointer; existing scope guard pins only if needed. No source preparation
793/797 owner growth, app, parameter, constructor or C edit. Source brand and
actual ordinal/site corruption must leave the row unavailable. Acceptance:
prior25-result pins plus2 initializer tests remain; exact realloc paths,
multiple same-origin leaves, direct/local forwarding and two-hop ordinal
permutation; facade outer-constructor relay witnesses; foreign owner/target,
rebind, class/ordinal and cycles remain rejected. Scope/pointer/diff/file caps.
Result.handle child relation is NOT claimed by outer-constructor witnesses:
constructor argument/destination verification must join these with realloc's
result provenance and prove acquisition or support in the later required slice.
Unchanged production frontier315/318 stays required and fail-closed.

Required follow-on sequence: source constructor argument/anchored-field relation
verification -> Home-prefix and completion lifetime/cleanup integration -> sole
physical owner retains source-issued owned/borrowed disposition and true outcome
-> publication and unchanged production acceptance. Both315 and318, facade
scalar observations/effects, ignored results, anchor escape/release, dying local,
forged acquisition and constructor/relay Fault are mandatory counterexamples.

## RESULT-ORIGIN-WITNESS-S0 implementation verification

Implementation retains immutable Rc source ancestry in the existing fold.
Direct leaves distinguish literal null, source construction and original formal
(binding/ordinal); formal-derived null carries the same provenance as forward.
Call composition retains exact caller return, initializer/direct call, callee
and actual substitution while sharing existing callee nodes. Per-path witnesses
are not deduplicated with the alternatives cache. Exact owner/argument path,
position, arity and declared formal ordinal reject corruption. No Home,
constructor capability, physical field mode or admission consumer was added.

First test-inclusive check found this-change E0599: boxed actual source rows
need Clone where the existing target classifier shares its actual inventory.
Derive Clone on that private source row; no added scan or mutation authority.
Corrected check PASS43.03s, /tmp/hako-result-origin-witness-check-fixed.log.
Read-only review confirms same fold/projection compatibility and branded edge
checks, no blocking issue. Shared ancestry does not claim elimination of path
count growth in arbitrary branching chains; current acceptance is the selected
bounded source cohort. Focused32-test executable build running, not yet PASS.
New negatives exercise foreign call/actual owners, wrong argument path, missing
formal binding and declaration ordinal disagreement. Actual facade source pins
both terminal relays with intervening scalar reads/counters; witnesses identify
only fresh outer constructor leaves, not Result.handle ownership. Source owners
parent727, evaluate218, product76, witness57, witness tests97, origin tests338.

First executable invocation ran53 tests because bare witness_tests also matched
unrelated temporal/lineage suites. Result50PASS/3FAIL: two this-change test
assumptions, one incidental Array get Call-count assertion needing parent proof.
Resize has both literal-null and formal-null callee exits; only the latter has
formal substitution. Correct that witness test distinction without weakening
formal-derived null checks. Full facade package issue stops later at existing
BorrowedFormalIngress stored-child/receiver-unavailable; use its unchanged full
source through the actual production prepare_source_claims phase, not a reduced
facade fixture or source workaround. Test-only loan builds the canonical catalog,
constructors, batch, selected map and parameter catalog, then invokes that same
source preparation. No package/EXE or later ingress acceptance is claimed.

Scope expands by two internal visibility lines in coseal_issue/source_claims,
limited to the enclosing ordinary-New module, for the phase-specific test loan;
no line growth (797/793), tuples, production caller or meaning change. Private
loan stays in witness_tests. Its first check E0433 used wrong module spelling
(issue vs coseal_issue); corrected. Exact incidental Array rerun still fails;
first abbreviated --exact attempt ran0 and is not evidence. Parent-source
isolated evidence is required before classifying that unrelated red as baseline.

Parent-source baseline comparison completed: exact Array temporal test at
3a5a96b299 executes1 and fails the SAME call_count0/expected1 assertion at516;
/tmp/hako-result-witness-parent-array-test.log. This is known baseline debt,
not witness-source regression. Parent test build finished normally (Cargo101
from assertion), driver restored all8 tracked Rust files byte-for-byte, SHA256
checks PASS, restore.json has0 conflicts. No new private source was removed.
Current corrected phase-test source is restored; previous30 selected tests had
28PASS/2 this-change test assumptions, both corrected as described above.
Source check and properly qualified32-test invocation remain required.

Corrected source test-inclusive check PASS27.72s;
/tmp/hako-result-origin-witness-phase-check-final.log. Scoped32 test command
uses ordinary_new_coseal::result_class_claim::witness_tests, not the bare
witness_tests filter. Running executable build is not yet test PASS. Scope
guard, pointer guard, diff check and owned Rust formatting PASS. Current source
lines: parent730/evaluate218/product76/witness57/witness-tests182/origin-tests358;
existing source preparation owners remain797/793. Final read-only review finds
no authority or admission expansion; both internal visibility lines stay within
ordinary_new_coseal and phase helper is cfg(test).

Corrected scoped executable invocation PASS32/32,0 ignored,8660 filtered,
runtime0.04s; build quick5m39s. Log:
/tmp/hako-result-origin-witness-focused-final.log. All prior25 result/source
pins,2 initializer corroboration pins and5 new witness pins execute. Full
unchanged facade source passes source-preparation witness assertions, not
later package/EXE admission. Corrected test assumptions and E0599/E0433 are
resolved; incidental Array assertion remains parent-proven baseline debt.
Fresh CLI build and unchanged app frontier observation remain required.

### Next child-relation source design brief (read-only, not execution admission)

Decision: join exact returned-construction child source relations using existing
constructor stores and finished result witness Facts. Preserve fail-closed
execution until later acquired-Home/anchored lifetime and cleanup verification.
Source authority + canonical issuer: existing prepare_source_claims after
result_class_draft.finish, before prefix verification; exact FreshConstruction
leaf site joined to instance constructor construction_for/birth_for, canonical
store field/RHS Parameter binding/provided child and Birth formal ordinal.
Resolve the exact caller actual with branded expr_at and child_expr_from_expr
CallArgument(ordinal); existing sole initializer/no-rebind and call witness
composition provide forwarded provenance, without another scanner/fixpoint.
Non-authority: class-only Fresh, ordinary local assignment, pointer identity,
OwnedFieldChildKind::Object metadata or candidate availability.
Fail-fast boundary: malformed field/constructor/formal/actual identity, ordinal,
owner/path/class, rebound locals or missing callable provenance cannot issue a
candidate. Source Fresh is not available Home; formal source is not acquired
child. Owning-only field schema cannot authorize anchored residence.
Smallest next slice: first separate behavior-preserving extraction of existing
prepare_source_claims from793-line issue_source into a private preparation
module; preserve sole caller, source order/tuple and failure boundary. Then
semantic candidate join there and a shared private call-witness composer.
Non-claims: no Home transfer, input consume, borrowed field store, cleanup,
physical discriminator, publication or EXE from the source candidate.

Required candidate acceptance: exact315 null-child,318 replacement initializer
mixed provenance and existing allocateResult fresh-child reference; canonical
field and Birth formal/actual ordinal identity plus corruption/rebind/missing
facts negatives. Candidate availability must coexist with PrefixNotCovered and
preparefalse. Later lifetime/cleanup and sole physical/publication slices remain
required. Gate1 is incomplete; stored-child sibling and Gates2-4 stay parked.

## RESULT-ORIGIN-WITNESS-S0 closeout

Fresh CLI quick PASS3m06s, /tmp/hako-result-origin-witness-cli.log.
Unchanged mimalloc-lite SHA256
33d3e8b9a4b9bc9ce983486f1e48334c8f186f9bbd05e98774ef155fe1b3d685;
EXE probe exits1 with the same required fail-closed frontier: both315/318
PrefixNotCovered(Body3), selectedfalse, then artifact-unowned-lifecycle-site in
reallocResult/2 block369 instruction3 raw birth receiver57. No EXE produced.
Log /tmp/hako-result-origin-witness-production.log. Required frontier observation
is complete; this is not app PASS or a new regression. Focused32/32 and guards
close the passive witness slice. Source witnesses now reach the existing ledger
product through the sole result solver; no old physical execution edge retired.
Anchored field lifetime/cleanup and actual production EXE remain outstanding.

## SOURCE-PREPARATION-BOXSHAPE-S0 Decision and execution contract

Select MIRBUILDER-GATE1-SOURCE-PREPARATION-BOXSHAPE-S0, work_mode fast.
Source authority + canonical issuer: existing prepare_source_claims called by
issue_ordinary_source_cohort_v1 and test-only source phase loan. Move this exact
function into private ordinary_new_source_claim_preparation.rs registered under
source_claims; reexport with the SAME enclosing ordinary_new_coseal visibility.
Keep its body byte-identical, signatures/tuple, observation and finish order,
provider child sealing, errors and caller behavior. No semantic candidate added
in this slice. Reduce issue_source793 before adding any child-relation logic.
Non-authority: extraction, module placement or source facts do not mint Home.
Fail-fast boundary: all current source coverage, projection, corruption and
PrefixNotCovered boundaries retained. No duplicate implementation/caller retry.
Replacement: old prepare_source_claims body in issue_source; after caller
reexport switch confirm only one implementation and unchanged callers, then
remove the original body in the same bounded move.
Acceptance: exact function body comparison, test-inclusive quick check,
focused32 positive/negative pins, scope/pointer/diff and owned rustfmt guards,
source caps; unchanged-source frontier baseline already captured above. This
shape-only slice does not require another full CLI build absent changed behavior
or new test failure. After this move, select CHILD-RELATION-S0 only once its exact
candidate mapping and acceptance above are verified against the actual APIs.
Non-claims: no field/source widening, second scanner/fixpoint, input consume,
borrowed field store, Home/anchor verifier or production completion.

SOURCE-PREPARATION-BOXSHAPE-S0 implementation: source owner793->697, sole
private child104 before rustfmt signature layout. Executable body bytes match
parent a6b57ccebc exactly; only tuple signature whitespace formatted. Same
production/test reexport paths, visibility, tuple, errors and observation/finish
order. Read-only worker confirms sole implementation/no behavior change.
Test-inclusive quick check PASS27.52s (before signature-only formatting), log
/tmp/hako-source-preparation-shape-check.log. Initial new-child formatting red
resolved with signature whitespace only; exact executable-body comparison PASS.
Scope guard initially FAIL because its field_write_draft/result_class_draft
pins still pointed at old file; update those two pins to new sole owner, keeping
all semantic conditions. Guard/README owner pointers are necessary selected
scope extensions. Focused32 execution after move remains required.

## SOURCE-PREPARATION-BOXSHAPE-S0 closeout / next CHILD-RELATION-D0

Corrected focused32/32 PASS,0 ignored,8660 filtered, runtime0.05s, quick build
4m45s. Log /tmp/hako-source-preparation-shape-focused.log. Scope/pointer/diff and
new-child rustfmt guards PASS. Executable preparation body byte-identical to
parent a6b57ccebc; original implementation removed and only one fn remains.
Same production/test callers route through reexport, no retry or second solver.
Source owner697/new child101; scope guard760 (future growth requires responsibility
review), active card under1000. Existing near-cap issue797 unchanged. Signature
formatting and moved guard pointer reds are resolved; no new unclassified red.
No additional CLI/probe required for byte-identical behavior-preserving move;
prior witness CLI/frontier receipt retained. No compile-time speedup claim:
measurements have different cache/incremental conditions.

Select MIRBUILDER-GATE1-CHILD-RELATION-D0, design_stop, next execution none.
Internal next contract audit is ongoing, not goal pause or external blocked.
Verified API decision: put passive candidate map inside SAME
VerifiedSourceCallableResultFactsV1, keyed (OwnedExprSiteV1,CanonicalFieldRefV1);
retain preparation tuple and ledger callable_result_classes owner. Use private
all-origin row/direct FreshConstruction leaf iterator; legacy get/iter/keys stay
projection-only. Deduplicate original constructor site; never analyze a callee
leaf using facade caller input. Constructor birth_for/construction_for, sealed
plan stores field/RHS Parameter(binding,provided child), row formal_contracts
binding/ordinal and branded CallArgument child navigation provide exact joins.
Existing resolve_forward_local/sole_initializer_site/binding_is_unrebound supply
local source identity; call composition currently inline in evaluate_row must
be extracted once and reused, rather than duplicated or invoked with fake exit.

D0 remaining concrete action: fix shared composer API/result/failure ownership,
private candidate product sealing and unavailable-field behavior, then select
separate composition BoxShape before child-relation semantic addition. Bounded
source candidates315Null/318mixed/allocateResultFresh must coexist with old
prefix errors/preparefalse. No acquired Home or field capability from candidates.
Unsupported constructor/formal/actual identities, class/ordinal/foreign owner,
rebind, missing facts, ungrounded recursion and unsupported field/index/upvar
actuals remain unavailable. Subsequent anchor lifetime/acquisition, cleanup,
sole physical outcome and publication remain required production work.

Next-pointer guard initially rejected workstream row H length549; shorten only
that current row while keeping full contract/evidence here. Corrected pointer
and scope guards PASS; no outstanding guard red.

## CHILD-RELATION-D0 accepted Decision / COMPOSITION-BOXSHAPE-S0

Read-only review corroborates exact current APIs and prelookup failure order.
Decision: one private borrowed CallWitnessSourceV1 view verifies output/call owner,
arity and each actual owner/Argument(index) identity; sole verify constructor
returns Option<Self>. Its compose(self,callee_rows,parameter_contracts,batch_slot)
returns Option<Vec<Rc<ResultOriginWitnessV1>>> with existing formal substitutions.
No new public/semantic receipt: ephemeral view groups current checks only.
Original evaluator keeps verify-before-lookup, missing/pending handling and legacy
eligibility, then composition and unchanged alternatives/coverage/class fold.
Reject an all-in-one Waiting/Ready helper: finished candidate joins should not
invent pending policy. Reject duplicated verification or fake return exits.
Malformed pending source must stay Dead; valid unresolved source stays Waiting.

Source child relation mapping accepted: private candidate map in SAME finished
result Facts, keyed branded outer construction site+canonical field. Private
all-origin direct FreshConstruction enumeration avoids projection-only iter/keys
and skips facade Call wrappers; original owning callable supplies lowering input.
Join sealed constructor store/formal/actual identities and existing sole local
initializer+call source witnesses. Missing/unsupported field relations expose no
candidate, never default Null/Fresh or all-fields completion. Partial passive
relations cannot authorize a completed construction. Fresh/forward witnesses
are not acquired Home/anchor lifetime. Actual field admission stays unchanged.

Select MIRBUILDER-GATE1-CALL-WITNESS-COMPOSITION-BOXSHAPE-S0, fast, first.
Source authority + canonical issuer: current evaluate_row Fwd arm and exact
existing PendingExit source rows; private call_witness child owns sole verify
and compose implementation, called only by the existing result fold for now.
Replacement: inline identity and callee-witness composition loops; switch the
one existing evaluator caller and remove those inline bodies in this slice.
Non-authority: ephemeral grouping, candidate/class/source origins, pointer tags.
Fail-fast boundary: preserve ALL current identity/arity/formal-kind/substitution
checks and prelookup Dead versus Waiting order. No pending/legacy/coverage policy
inside composer, no second scan or fixed point, no execution admission.
Acceptance: test-inclusive quick check and existing focused32; extend existing
corruption test with missing-pending callee cases proving malformed call/actual
owner/path stays Dead and structurally valid missing formal binding stays Waiting
until callee resolution. Existing multi-hop/formal-null/same-origin sharing and
full facade source pins retain composition behavior. Scope/pointer/diff/rustfmt
and source caps; no full CLI rebuild for shape-only change absent behavior drift.
Paths: result-class registration/evaluate/new call_witness, existing witness test,
package README, active card and changed current pointers; guard only if moved pin
requires it. No candidate map, field contract, Home, app or C edit in this slice.
After this separately verified extraction, source child candidate semantic slice
uses the SAME helper and product. Lifetime/cleanup/physical/publication and
unchanged production EXE remain incomplete, Gates2-4/stored-child remain parked.
