# APP-binary-trees
Status: selected app; final MIR verification fails; EXE not reached. Date: 2026-10-25
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [fixed smoke](../../../../../tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh).

## Current stop
- Fresh `cargo quick-build-fast` PASS (latest 42.71s). The unchanged smoke explicitly uses `NYASH_BIN=target/quick/hakorune` and `NYASH_EMIT_EXE_NYRT=target/quick`.
- MIR final verification fails at `mir/invoke/borrowed-source-incoming-coverage`: the two `iterationCheck` loop initializers (`builder.make(depth, i)` and `builder.make(depth, 0 - i)`) remain in original incoming but are absent from ordinary final Call visitation.
- Temporary diagnostics confirmed their retained groups are initializer expressions with zero lexical Call packets; the Loop aggregate's ordinary Calls are the two `itemCheck` operations. Diagnostics were removed; do not add a parallel Call inventory or weaken coverage.
- Read-only worker found `issue_received_producer_local_call_with_alias_v1` excludes Parameter receivers before asking the package for `ReceivedObjectArguments`. Co-seal separately prioritizes Dynamic admission for the same unannotated receiver. Confirm the exact retained expression producer and align these source authorities using the original owning-object loan; a matching name/class tag alone is insufficient.
- Ordinary object Call selection will also require explicit Loop result Home/Normal/Fault cleanup support; current V2 scheduled Call locals admit scalar results only. Preserve the independent final consumer.

## Scope and changes
- Keep `apps/binary-trees/main.hako` unchanged; acceptance is its existing output and exit 0. No retry, fallback or app-specific route.
- Whole Bool conditions now admit exact I64 subtraction through the existing scalar operation loan and independently recheck its original producer/operands.
- Remaining WIP connects exact computed Return source and argument projections, structural Jump contraction, stored receiver lifetime, Return-owned Call/field inventory, and nested Loop copy dependencies. Build success does not close those changes.
- Latest fixed suite is still 9/11 PASS (2026-10-24); no suite rerun or binary-trees EXE success is claimed.

## Verification
- Existing compiled quick lib binary: `original_scalar_append_requires_same_source_and_exact_ordered_producer`, `scalar_expression_scope_keeps_whole_bool_and_outside_nested_conditions_distinct`, `scalar_expression_claims_bool_initializer_and_field_condition`, and `scalar_expression_rejects_non_bool_field_condition_before_staging`: 4 PASS.
- Broader focused runs: scalar-expression family 15 PASS/1 FAIL; borrowed-entry family 19 PASS/2 FAIL; print-expression family 3 PASS/1 FAIL. Initializer final-consumer and stored-child identity tests also fail before their intended mutation checks. Attribution remains open; these are not green evidence for the remaining WIP.
- Physical-boundary threading test fails its `branch-args` mutation: unchanged HEAD `same_finished_instruction` accepts empty carrier args as equivalent to None. Preserve the production ownership/control law and resolve the stale assertion separately from missing source Call coverage.
- Previous full lib run also had two failures (`production_skip_while...`, `selected_exact_usize_wrapper...`) and a stack-overflow abort in `owned_array_original_construction...`; still unclosed. Zero-match filters are excluded from evidence.

## Next work
1. Align original object Call admission for borrowed Parameter receivers with the existing source loan and argument owner; retain genuine Dynamic calls without an exact ordinary proof.
2. Connect object Call result/Home lifetime through the existing source Loop schedule, emission and final consumer; rerun the unchanged smoke.
3. Attribute and resolve the relevant focused failures before committing the remaining WIP, then refresh runtime/suite at app landing.
