# APP-binary-trees
Status: selected app; EXE not reached. Date: 2026-10-25
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [fixed smoke](../../../../../tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh).

## Current stop
- The unchanged `binary_trees_exe.sh` smoke fails during MIR emission with `[source-loop/v2/source Loop has an unavailable path]` at `BinaryTreesBench.run`, `Body(14)/LoopBody(1)` (`local check = me.iterationCheck(builder, depth, iterations)`).
- Clean-source `cargo quick-build-fast` passed in 40.78s. The smoke explicitly used `target/quick/hakorune`; the old `target/release/hakorune` is stale and is not acceptance evidence.
- The smoke advanced past `stretch_tree.itemCheck()` and `long_lived_tree.itemCheck()`; the returned `TreeNode` receiver Homes are now accepted far enough to reach the loop.
- `iterationCheck` has three unannotated formals and returns computed `sum`. The original-return source owner declines parameterized nonliteral returns, so this call has no `ScalarReturnSource` row; do not treat the missing source row as an I64 result.
- `Main.pow2(...)` is already recognized by the qualified-static ExactI64 route and its local result reaches the loop body. The `builder` loan is retained at `iterationCheck`; exact scalar input loans for loop-carried `depth` and `iterations` are still unavailable.
- Connect an exact computed-return source through the existing result owner, leaving the result class to its sealed proof, then connect the existing argument-source projections. Preserve the Loop Header proof for `depth` and the qualified-static source proof for `iterations`.
- `cargo quick-test-fast` compiled, but the full library run had two failures (`production_skip_while...`, `selected_exact_usize_wrapper...`) and a stack-overflow abort in `owned_array_original_construction...`; attribution to this WIP is not established.
- Keep untyped fields Dynamic and `Add(Dynamic, Dynamic)` refused. Keep `apps/binary-trees/main.hako` unchanged.
- The last fixed-suite result remains 9/11 PASS (2026-10-24); the full suite was not rerun. The focused smoke failed as described above.

## Scope
- Keep `apps/binary-trees/main.hako` unchanged. The acceptance expects the existing binary-trees output and exit 0.
- Preserve the source/ownership law and current Dynamic operator boundary. Do not add retry, fallback, app-specific route, or a source workaround.

## Next work
1. Extend the existing source-result owner to retain the exact computed Return plan needed by `iterationCheck`; let sealed result facts determine whether it is scalar.
2. Project the exact `builder`, `depth`, and `iterations` inputs from their existing source owners, without inferring from runtime/MIR representation.
3. Rerun the unchanged smoke with `NYASH_BIN=target/quick/hakorune` and keep the app source unchanged.
