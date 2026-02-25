Status: SSOT
Scope: `src/runner/json_v0_bridge/lowering.rs` structure cleanup (behavior-neutral)
Related:
- `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- `CURRENT_TASK.md`

# JSON v0 Bridge Lowering Split SSOT

Purpose: keep JSON v0 bridge maintainable while preserving Stage-B/selfhost behavior.
This SSOT defines the responsibility map, split completion state, and post-split decision criteria.

## 1) Current Responsibility Inventory (as of 2026-02-08)

### R1. Loop runtime state + shared flow helpers
- File: `src/runner/json_v0_bridge/lowering/loop_runtime.rs`
- Contents:
  - loop snapshot stacks (`EXIT_SNAPSHOT_STACK`, `CONT_SNAPSHOT_STACK`)
  - increment hint stack (`INCR_HINT_STACK`) and push/pop/peek helpers
  - `jump_with_pred`, `lower_break_stmt`, `lower_continue_stmt`
- Contract:
  - continue path still applies `apply_increment_before_continue` hint before snapshot/jump.
  - break/continue snapshots are frame-local and LIFO.

### R2. Bridge environment and function-definition bootstrap
- File: `src/runner/json_v0_bridge/lowering.rs`
- Contents:
  - `BridgeEnv`
  - `FunctionDefBuilder`
- Contract:
  - param ValueId convention remains aligned with `MirFunction::new()`.
  - stage-b static method table behavior remains unchanged.

### R3. Statement lowering entry and legacy-shape window matching
- Main file: `src/runner/json_v0_bridge/lowering/stmts.rs`
- Facade entry: `src/runner/json_v0_bridge/lowering.rs`
- Contents:
  - `lower_stmt_with_vars`
  - `lower_stmt_list_with_vars`
  - legacy pattern windows (`if_legacy`, `while_legacy`, `lambda_legacy`)
- Contract:
  - window matching order remains fixed: `if_legacy` -> `while_legacy` -> `lambda_legacy` -> default stmt.

### R4. Program/module assembly + defs lowering + call resolution
- Main file: `src/runner/json_v0_bridge/lowering/program.rs`
- Facade entry: `src/runner/json_v0_bridge/lowering.rs`
- Contents:
  - `lower_program`
  - defs lowering loop
  - optional call-target resolution (`HAKO_MIR_BUILDER_CALL_RESOLVE`)
- Contract:
  - CLI entry keeps `args` injection behavior.
  - call-resolution toggle semantics remain unchanged.

### R5. MIR dump helper
- Main file: `src/runner/json_v0_bridge/lowering/dump.rs`
- Facade entry: `src/runner/json_v0_bridge/lowering.rs`
- Contents:
  - `maybe_dump_mir`
  - tests for `RUST_MIR_DUMP_PATH`
- Contract:
  - file dump + verbose stdout behavior remains unchanged.

### R6. BlockExpr-specific lowering window
- Main file: `src/runner/json_v0_bridge/lowering/expr/block_expr.rs`
- Facade entry: `src/runner/json_v0_bridge/lowering/expr.rs`
- Contents:
  - `lower_blockexpr_with_vars`
  - scope-exit-aware BlockExpr path (`lower_blockexpr_with_scope_exit`)
  - BlockExpr prelude/tail validation helpers
- Contract:
  - `ExprV0::BlockExpr` lowering entrypoint remains `lower_expr_with_vars`.
  - legacy local-chain behavior and scope-exit (`FiniReg`) handling remain behavior-equivalent.

## 2) Split Queue (1 commit = 1 structural unit)

Status: `CLEAN-JSONV0-BRIDGE-SPLIT-{2..6}` completed on 2026-02-08.

### CLEAN-JSONV0-BRIDGE-SPLIT-2
- Target: extract R1 into `src/runner/json_v0_bridge/lowering/loop_runtime.rs`.
- Allowed diff:
  - move snapshot/hint stacks + break/continue helper functions.
  - re-export/use from `lowering.rs`.
- Forbidden diff:
  - no statement semantics changes.
  - no env toggle changes.
- Result:
  - completed (`04c224e81`)

### CLEAN-JSONV0-BRIDGE-SPLIT-3
- Target: extract R4 defs/call-resolution path into `lowering/program.rs` (or equivalent).
- Allowed diff:
  - move `lower_program` subparts (`defs lower`, `call resolve`) behind thin entry.
- Forbidden diff:
  - no change to entry `args` handling and call-resolve toggle semantics.
- Result:
  - completed (`6b4b05c1e`)

### CLEAN-JSONV0-BRIDGE-SPLIT-4
- Target: extract R3 dispatcher/list-walker into `lowering/stmts.rs`.
- Allowed diff:
  - move `lower_stmt_with_vars`, `lower_stmt_list_with_vars`, keep same call order.
- Forbidden diff:
  - no reorder of legacy window checks.
- Result:
  - completed (`9c1ef3efc`)

### CLEAN-JSONV0-BRIDGE-SPLIT-5
- Target: extract R5 into `lowering/dump.rs`.
- Allowed diff:
  - move `maybe_dump_mir` and adjacent tests.
- Forbidden diff:
  - no dump format/environment behavior changes.
- Result:
  - completed (`fac58131b`)

### CLEAN-JSONV0-BRIDGE-SPLIT-6
- Target: extract R6 BlockExpr window from `expr.rs` into `expr/block_expr.rs`.
- Allowed diff:
  - move BlockExpr-only lowering/validation helpers behind `expr.rs` facade.
- Forbidden diff:
  - no change to BlockExpr prelude/tail acceptance rules.
  - no change to `FiniReg` scope-exit path behavior.
- Result:
  - completed (local structural split; behavior-neutral)

## 3) Acceptance Command (for code-split commits)

- `cargo check --bin hakorune`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

## 4) Guardrails

- No AST rewrite.
- No new environment variables.
- No selfhost route behavior change.
- `try/catch` legacy handling remains as-is until dedicated task.

## 5) Post-Split Decision Criteria (R2)

R2 (`BridgeEnv` + `FunctionDefBuilder`) is intentionally still in `lowering.rs`.
Decide additional split only when at least one trigger holds:

- `BridgeEnv` gains new policy/toggle branches and starts coupling with non-entry modules.
- `FunctionDefBuilder` grows beyond signature/var-map bootstrap and owns independent lowering policy.
- changes regularly touch both R2 and non-R2 responsibilities in one PR/commit.

Keep-as-is criteria:

- R2 remains small and entry-local.
- `lowering.rs` stays as a readable façade around split modules.
- no drift in Stage-B contracts (`args` injection, static method table bootstrap).
