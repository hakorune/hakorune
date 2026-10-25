# APP-binary-trees
Status: selected app; source/result/Loop series landed; EXE not reached. Date: 2026-10-25
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [fixed smoke](../../../../../tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh).

## Current stop
- Fresh quick CLI build PASS (1m14s); fixed suite 6/11 in40.050s, five compile FAIL, no new acceptance PASS. Explicit CLI/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`.
- binary-trees: `lexical-instance-call/source-pending`, recursive `BinaryTreeBuilder.make`, owner slot5 Body(4)/Initializer(0), before EXE. Two final object-field-read errors are historical best evidence, not the current stop after stricter source proof.
- mimalloc-lite and allocator-stress: `borrowed-object/forward-source-identity`, owner slot37 Body(5). Original entry-formal site proof remains owed. JSON: `borrowed-entry/source-use-completion-pending`, owner slot22. Boxtorrent: `stored-child/receiver-unavailable`.

## Landed source and consumer connection
- Computed Return trees retain original plans; grounded source classes feed the same original SUM without switching typed-only calls into borrowed-input transport. Unchanged Core premises reuse their canonical proof identity.
- Source actuals retain original census, source phase identity, argument ordinal/site and reaching-value/formal class evidence. Original argument plans independently match binding to source; facts alone grant no receiver, Home or execution.
- Object Call selection uses original owned-object input before Dynamic selection. Local object arguments, stored parent lifetime, Loop result Homes, Normal/Fault cleanup, exact prefix liveness and independent final Call coverage remain connected through existing owners.
- Qualified SourceStatic arguments are revalidated with the original projector before physical co-seal deferral. Their selected reservation stays intact; selected physical demand still refuses an unfinished cohort. No generic retry or capture-execution permission was added.
- Source SUM producer and read-only loans are separated (695/546 lines); solver tests moved to the existing solver module (400 lines). No new test file, hook, checker, guard or card.
- Existing cases now check result permission rather than an empty Err census, field requests rather than all passive Return plans, original transport/return identity, source-only Loop observations, and semantic refusal rather than which guard runs first. Inputs/case counts stay unchanged; the captured case additionally guards physical-demand refusal after co-seal deferral.
- Keep every `.hako` input and acceptance expectation unchanged. No fallback, AST rewrite, backend repair or app-specific route. Temporary instrumentation and omission prototype were removed; no temporary adapter is retained.

## Verification and limits
- Independent earlier commits: borrowed-call final consumers27 PASS; constructor order repair changes one existing Unit publication failure to discard-family4 PASS. Pure owner moves retain original bodies; isolated entry21 and terminal1 PASS, print3 PASS/1 identical prior failure.
- Same selected lexical family (`cargo quick-test-fast ordinary_new_coseal::lexical_instance_call:: -- --test-threads=4`, `RUST_MIN_STACK=16777216`): baseline296 PASS/60 FAIL, current319/37. All8 newly failing cases from the previous cleanup are closed; no new failing test versus baseline.
- Existing raw compiled families: local-commit baseline92/12 -> current94/10; Loop213/21 ->213/21; borrowed publication24/28 ->38/14. Combined746 cases:39 prior failures fixed, no newly failing cases. The82 remaining failures are still open; this is not full-lib or product acceptance.
- Shared target/quick reused sequentially. Baseline and current binary identities were kept separate; no zero-match filter counts as evidence. The temporary isolated checkout is removed. Pointer guard and diff checks PASS.

## Next work
1. With WIP committed, trace the recursive make source-pending and shared entry-formal forward evidence to the existing source/argument owner; restore mimalloc/allocator/JSON without weakening source or final checks.
2. Continue binary-trees through its original checked stored-child reads and sole physical owner. Its existing output and exit0 remain acceptance; historical final-verifier progress is not current success.
3. Attribute and resolve required remaining verification failures. Keep R7/canonical closeout obligations open; no app or finite-product completion is claimed by this internal commit.
