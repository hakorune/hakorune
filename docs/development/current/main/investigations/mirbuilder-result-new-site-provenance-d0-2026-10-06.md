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
