# Borrowed I64 call-result source D0

Status: Design / existing owner dependency mapping
Execution row: MIRBUILDER-BORROWED-I64-CALL-RESULT-SOURCE-D0
Scope: required I64 result composition inside the existing borrowed-result owner;
no implementation until source-site/target/result/completion mapping is closed.
Related:
- mirbuilder-terminal-field-return-finished-projection-s0-2026-10-05.md
- mirbuilder-declared-object-tagged-borrow-physical-s1-2026-10-05.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Decision brief

Source authority: original resolved function's verified return sites, binding and
sole initializer sites; already prepared LexicalInstanceCallSourceTargetV1 and
existing VerifiedCallableResultContractCohortV1 completion. Exact owner/site/key/
batch-slot/result correspondence is mandatory. Source/result facts precede MIR.
Canonical issuer: existing ordinary_new_borrowed_formal_result::source_result,
prepare_borrowed_i64_results_v1 and corroborate_borrowed_i64_result_v1; existing
lexical instance call issuer remains the only target/affine call authority.
No separate result registry, annotation-manufactured proof or MIR inference.

Production consumer: ordinary-new borrowed scalar call/result issuance and its
existing local/direct return continuation. Selected old responsibility is the
result classifier's blanket rejection of a proven call-result source as
borrowed-result/source-not-i64. Other unproven result forms remain fail-closed.
Initial bounded shape: exact lexical call directly returned, or a never-rebound
local whose sole initializer is that exact call. Join the original prepared
call target to the callee's independently proven I64 result and completion;
never treat a declared i64 annotation alone as a body proof.

Non-authority: receiver/selector names, caller constants, layout observations,
physical Call/Invoke result tags, a missing/empty loan, fallback, .hako rewriting.
Fail-fast boundary: missing/foreign/ambiguous call target or initializer, wrong
owner/site, rebind, unproven callee result, mixed nullable/scalar exit set,
incomplete completion or circular dependency without independent seed.

Unresolved construction decision: the existing prepare phase precedes prefix
walk and completion corroboration. Map exact source dependencies without
requiring the caller's completed prefix as its own prerequisite. Close issuer
ordering, dependency/cycle handling and completion lending before implementing.
Read-only worker audit_cost_oct5 confirmed this owner/site/completion mapping;
primary adopts one Decision here. The first prospective S0 is the sole-local
call-result return; direct-call return and stored-child receiver remain subsequent
required obligations, not reduced goal scope. Dependency/issuer ordering still
needs concrete closure. No production switch/fixture or new receipt authorized yet.

## Evidence and prerequisite separation

S1 original source witness with local out=recv.read(p); return out stopped at
borrowed-result/source-not-i64; S1 card149 preserves that required obligation.
Current classifier accepts integer literal/exact I64 formal/guarded I64 field,
then null/new nullable; all other return sites reject. Prepared lexical targets
are available before borrowed result preparation (coseal_issue126-159).
Final corroboration checks the exact complete return-site set and declaration.

Real Heap.isLiveHandle returns me.small_page.isLiveHandle(handle) or medium_page;
these are stored-child direct receivers. Worker confirmed lexical source target
preparation admits only Lexical(Local binding), while source field provenance
currently proves local recv=me.field, not a direct structural field receiver.
That receiver target/borrowed actual mapping is an independent required
prerequisite; scalar result composition alone cannot complete this production
shape. Do not change .hako to manufacture the supported local receiver spelling.
No claim that this D0 or its first slice resolves the app's first terminal.

## Required next acceptance

- Existing source-issued lexical leaf I64 call, direct and sole-local return,
  annotated/unannotated caller where source completion proves I64; object/null
  incoming borrowed parameter and both optimization settings.
- Forwarding chains and reversed declaration order; dependency handling must
  preserve exact source rows. Self/mutual unsupported cycles remain rejected.
- Reject rebound/duplicate initializer, foreign owner/site/target, wrong result,
  nullable/scalar mixed exits, missing completion and annotation-only permission.
- Existing literal/formal/field/nullable results and final borrowed-use guards
  retain their acceptance. Execute issued JSON Normal/Fault cleanup when the
  resulting physical shape is already supported; preserve exact rejection otherwise.
- Separate stored-child target mapping, Provided ownership and comparison/use
  vocabulary from scalar composition. Keep independent final MIR/wire validation.
- Map focused tests, selected guard and owner README before selecting fast S0.

## Restart

Previous slice a47039d3fe landed/pushed: three production field-return validation
callers share the existing FinishedBindings checker; original-block-only read
lookup retired. Source27/entry21/boundary11/JSON29/borrowed-use17 and six C
Normal/Fault programs PASS. Package623/exact known3 and HEAD-identical root1351
scope-guard debt remain classified baseline, never whole-green evidence.
No Cargo live. Older C/Array/S2 WIP remains protected and scoped landing is owed.
Unchanged app last observed artifact-unowned-lifecycle-site, no EXE. Full finite
pipeline, production/retirement and acceptance are still open; parked lanes unchanged.
