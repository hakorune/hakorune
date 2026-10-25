# APP-binary-trees
Status: selected app; source/result/Loop series landed; EXE not reached. Date: 2026-10-25
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [fixed smoke](../../../../../tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh).

## Current stop
- Fresh quick CLI build PASS (32.83s); fixed suite 6/11 in52.276s, five compile FAIL, no new acceptance PASS. The six include the non-EXE probe marker. Explicit CLI/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`.
- binary-trees now passes recursive `BinaryTreeBuilder.make` source/result sealing and reaches final MIR verification: `object-field-read-definition-invalid` at bb25/bb27. EXE remains unbuilt; these are current results, not historical best evidence.
- mimalloc-lite and allocator-stress: `borrowed-object/forward-source-identity`, owner slot37 Body(5). Original entry-formal site proof remains owed. JSON: `borrowed-entry/source-use-completion-pending`, owner slot22. Boxtorrent: `stored-child/receiver-unavailable`.

## Landed source and consumer connection
- Constructor source Dynamic stores now finish before input/result sealing; stored-child Unit calls finish after lexical dispositions. Both use the same original qualifiers and common End/claim/handoff installer, once per responsibility. No speculative rerun or app-specific gate.
- Computed Return trees retain original plans; grounded source classes feed the same original SUM without switching typed-only calls into borrowed-input transport. Unchanged Core premises reuse their canonical proof identity.
- Source actuals retain original census, source phase identity, argument ordinal/site and reaching-value/formal class evidence. Original argument plans independently match binding to source; facts alone grant no receiver, Home or execution.
- Object Call selection uses original owned-object input before Dynamic selection. Local object arguments, stored parent lifetime, Loop result Homes, Normal/Fault cleanup, exact prefix liveness and independent final Call coverage remain connected through existing owners.
- Qualified SourceStatic arguments are revalidated with the original projector before physical co-seal deferral. Their selected reservation stays intact; selected physical demand still refuses an unfinished cohort. No generic retry or capture-execution permission was added.
- Source SUM producer and read-only loans are separated (695/546 lines); solver tests moved to the existing solver module (400 lines). No new test file, hook, checker, guard or card.
- Existing cases now check result permission rather than an empty Err census, field requests rather than all passive Return plans, original transport/return identity, source-only Loop observations, and semantic refusal rather than which guard runs first. Inputs/case counts stay unchanged; the captured case additionally guards physical-demand refusal after co-seal deferral.
- Keep every `.hako` input and acceptance expectation unchanged. No fallback, AST rewrite, backend repair or app-specific route. Temporary instrumentation and omission prototype were removed; no temporary adapter is retained.

## Verification and limits
- Constructor staging verification reuses460 existing cases: lexical319 PASS/37 FAIL and local-commit94/10, with the same failing names as the preceding commit. Unit/discard4 PASS. The nullable-source negative keeps the same input and refusal; it now checks that a completed packet remains unconsumed when original ingress becomes unavailable. No cases added.
- Independent earlier commits: borrowed-call final consumers27 PASS; constructor order repair changes one existing Unit publication failure to discard-family4 PASS. Pure owner moves retain original bodies; isolated entry21 and terminal1 PASS, print3 PASS/1 identical prior failure.
- Same selected lexical family (`cargo quick-test-fast ordinary_new_coseal::lexical_instance_call:: -- --test-threads=4`, `RUST_MIN_STACK=16777216`): baseline296 PASS/60 FAIL, current319/37. All8 newly failing cases from the previous cleanup are closed; no new failing test versus baseline.
- Existing raw compiled families: local-commit baseline92/12 -> current94/10; Loop213/21 ->213/21; borrowed publication24/28 ->38/14. Combined746 cases:39 prior failures fixed, no newly failing cases. The82 remaining failures are still open; this is not full-lib or product acceptance.
- Shared target/quick reused sequentially. Baseline and current binary identities were kept separate; no zero-match filter counts as evidence. The temporary isolated checkout is removed. Pointer guard and diff checks PASS.

## Next work
1. Replace raw Dynamic stored-child reads with the original-source checked borrowed-object read: exact field/expected child class/nullability, Normal borrowed result, Fault cleanup and parent lifetime; connect the same packet to independent final validation. Do not reinterpret the I64 getter as a handle or relax the verifier.
2. Keep shared entry-formal forward evidence and other app failures open. binary-trees existing output and exit0 remain acceptance; final-verifier progress alone is not success.
3. Attribute and resolve required remaining verification failures. Keep R7/canonical closeout obligations open; no app or finite-product completion is claimed by this internal commit.
