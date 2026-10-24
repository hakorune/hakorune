# APP-binary-trees
Status: selected app; EXE not reached. Date: 2026-10-24
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [fixed smoke](../../../../../tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh).

## Current stop
- The unchanged EXE smoke stops while lowering `iterationCheck`'s loop-carried sum: `[ordinary-new/borrowed-result/source-not-i64]` at `LoopBody(3), Value, Rhs`.
- The fixed suite confirms this source stop under the current optimized quick CLI. `boxtorrent-mini` is the other remaining suite failure.

## Scope
- Keep `apps/binary-trees/main.hako` unchanged. The acceptance expects the existing binary-trees output and exit 0.
- Trace the result source for `positive.itemCheck()` / `negative.itemCheck()` through the canonical call and loop owners. Use the explicit VM Compatibility MIR only as an investigation oracle; preserve the source/ownership law and do not add retry or fallback.

## Next work
1. Re-run `binary_trees_exe.sh` with the current binary and identify the exact source expression and canonical issuer.
2. Connect the returned value through the same semantic source owner and the loop-carried sum consumer; retain Fault cleanup and final verification.
3. Re-run this unchanged smoke before moving to the next app.
