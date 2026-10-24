# APP-binary-trees
Status: selected app; EXE not reached. Date: 2026-10-24
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [fixed smoke](../../../../../tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh).

## Current stop
- The unchanged EXE smoke stops at `iterationCheck` line 113, where `negative.itemCheck()` feeds the loop-carried sum: `[ordinary-new/borrowed-result/source-not-i64]` at `LoopBody(3), Value, Rhs`.
- Re-run with `HAKO_BIN=target/quick/hakorune bash tools/smokes/v2/profiles/integration/apps/binary_trees_exe.sh` reproduces the same stop during MIR compilation; EXE execution is not reached.
- `TreeNode.itemCheck()` returns `me.value` or combines it with recursive `me.left/right.itemCheck()` calls. `value`, `left`, and `right` are untyped stored fields; the method has no result annotation.
- The language contract keeps untyped fields Dynamic and does not infer a declared return type. `Add(Dynamic, Dynamic)` is explicitly unsupported, so do not widen that operator or infer I64 from MIR representation.
- The current quick binary lacks VM-reference support. Both quick and release `--backend vm --emit-mir-json` attempts stop earlier at `birth-global-legacy-stopped`; use the unchanged smoke's expected output/exit as the product oracle.
- The latest fixed-suite result remains 9/11 PASS; `boxtorrent-mini` is the other source failure.

## Scope
- Keep `apps/binary-trees/main.hako` unchanged. The acceptance expects the existing binary-trees output and exit 0.
- Preserve the source/ownership law and current Dynamic operator boundary. Do not add retry, fallback, app-specific route, or a source workaround.

## Next work
1. Establish one source-owned proof across the `TreeNode` constructions: exact constructor/formal/field correspondence, complete construction coverage, and the paired `left/right` Null-or-TreeNode alternatives for each allocation. Existing child relations are per `new` site and do not yet establish the complete paired invariant. Do not infer it from field names or MIR types.
2. Connect that proof to physical storage. `DynamicFieldSetContractV1` can describe Object/ObjectHome, but no production Dynamic field Object issuer was found; current production field writers issue Array or Dynamic-I64. Add the source-checked nullable Object store through the existing field/Home/End path, consuming the exact Home only on Normal. `DynamicResidence` remains a teardown requirement, not child identity or ownership evidence.
3. Feed stored calls into the same source and result owners. `stored_child_source_v1` currently requires a typed `Provider` field; the Dynamic child lane needs an exact owned-child proof. `ResultExpressionSourceV1` discards the prepared stored-call need, `calls_ready` accepts only lexical/self receivers, and the recursive-result fixed point needs the exact receiver and grounded result. Preserve the `me.left == null` field-place refinement and prove that `right` is non-null on that same allocation path before lowering `me.right.itemCheck()`.
4. Keep `Add(Dynamic, Dynamic)` refused; ground `itemCheck()` from source-backed I64 `value` plus exact recursive calls, then connect the already-blocked loop consumer. Rerun this unchanged smoke before moving to the next app.
