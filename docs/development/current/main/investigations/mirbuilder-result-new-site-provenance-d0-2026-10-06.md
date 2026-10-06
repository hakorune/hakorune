# Result-new source-site provenance D0

Status: selected; source mapping unresolved
Date: 2026-10-06
Scope: MIRBUILDER-GATE1-RESULT-NEW-SITE-PROVENANCE-D0
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
