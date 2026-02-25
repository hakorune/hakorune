# Phase 131: loop(true) break-once Normalized Support

Status: DONE
Scope: loop(true) break-once Normalized（StepTree → Normalized shadow pipeline）
Related:
- Entry: `docs/development/current/main/10-Now.md`
- Phase 130 (if-only post_k): `docs/development/current/main/phases/phase-130/README.md`
- ControlTree SSOT: `docs/development/current/main/design/control-tree.md`

## Goal

Add minimal loop support to Normalized shadow path:

- Enable `loop(true) { <assign>* ; break }` (one-time execution loop)
- Keep **PHI禁止**: merge via env + continuations only
- Keep **dev-only** and **既定挙動不変**: unmatched shapes fall back (strict can Fail-Fast for fixtures)

## Non-Goals

- No general loop conditions (only `true` literal for now)
- No continue support (only break at end of body)
- No nested loops/ifs inside loop body (Phase 130 assigns only)
- No post-loop statements beyond simple return

## Accepted Forms

### ✅ Supported (P0)

```nyash
// Form 1: loop(true) with break at end
local x
x = 0
loop(true) {
    x = 1
    break
}
return x  // or print(x)
```

### ❌ Not Supported (Out of Scope)

```nyash
// Continue (not break)
loop(true) { x = 1; continue }

// Nested control flow
loop(true) { if (y == 1) { x = 2 }; break }

// Multiple breaks / conditional break
loop(true) { if (cond) { break }; x = 1; break }

// General loop conditions
loop(x < 10) { x = x + 1; break }

// Complex post-loop computation
loop(true) { x = 1; break }
y = x + 2
return y
```

## Task 3 & 4: smokes Runner Improvements (DONE)

### Task 3: OutputContract 統一

**実装箇所**: `tools/smokes/v2/lib/llvm_exe_runner.sh`

**新しい統一インターフェース**:
```bash
check_output_contract() {
    local contract_type=$1  # "exit_code" / "numeric" / "substring"
    local expected=$2
    local actual=$3
    local context=$4        # (optional) Context description

    # 統一された検証ロジック
}
```

**利点**:
- ✅ exit_code/numeric/substring が統一インターフェース
- ✅ エラーメッセージが一貫している
- ✅ コピペコードが削減（約30行の整理）
- ✅ 既存 smoke 全て PASS（後方互換性）

**変更箇所**:
- `llvm_exe_build_and_run_numeric_smoke`: numeric 検証を統一インターフェース化
- `llvm_exe_build_and_run_expect_exit_code`: exit_code 検証を統一インターフェース化

### Task 4: require_joinir_dev ヘルパー

**実装箇所**:
- `tools/smokes/v2/lib/llvm_exe_runner.sh`
- `tools/smokes/v2/lib/test_runner.sh`

**新しいヘルパー関数**:
```bash
require_joinir_dev() {
    export NYASH_JOINIR_DEV=1
    export HAKO_JOINIR_STRICT=1
    echo "[INFO] JoinIR dev mode enabled (...)"
}
```

**利点**:
- ✅ 環境変数セットアップが一元化
- ✅ dev-only fixture が統一パターンで記述可能
- ✅ コピペ防止（3行→1行）
- ✅ 既存 smoke 全て PASS

**使用例**:
```bash
# Before (phase131_loop_true_break_once_llvm_exe.sh)
export NYASH_JOINIR_DEV=1
export HAKO_JOINIR_STRICT=1

# After
require_joinir_dev
```

**適用済み箇所**:
- `tools/smokes/v2/profiles/integration/apps/phase131_loop_true_break_once_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/phase131_loop_true_break_once_vm.sh`

## SSOT Contracts

### EnvLayout (Phase 126)

Same as Phase 130:
- `writes`: Variables assigned in the fragment
- `inputs`: Variables read from outer scope
- `env_fields()`: `writes ++ inputs` (SSOT for parameter order)

### Loop Structure Contract

For `loop(true) { body ; break }`:

1. **Condition**: Must be Bool literal `true` (cond_ast check)
2. **Body**: Must be a Block ending with Break statement
3. **No exits**: No return/continue in body (only break at end)
4. **Assignments**: Body contains only Assign(int literal/var/add) and LocalDecl

### Generated JoinModule Structure

```
main(env) → TailCall(loop_step, env)

loop_step(env) →
    if true { TailCall(loop_body, env) }
    else { TailCall(k_exit, env) }

loop_body(env) →
    <assign statements update env>
    TailCall(k_exit, env)

k_exit(env) →
    Ret(env[x])  // or TailCall(post_k, env) if post exists
```

**Key Invariants**:
- No PHI instructions (all merging via env parameters)
- All continuations take full env as parameters
- Condition `true` is lowered as constant true comparison

## VM + LLVM Parity

Both backends must produce identical results:
- VM: Rust VM backend (`--backend vm`)
- LLVM: LLVM EXE backend via llvm_exe_runner.sh

Expected contract for fixture: exit code `1` (return value)

## Environment Note (WSL)

- If `cargo build` fails with `Invalid cross-device link (os error 18)` on WSL, a full WSL restart (`wsl --shutdown`) has been sufficient to recover in practice.
- `tools/build_llvm.sh` also forces `TMPDIR` under `target/` to reduce EXDEV risk during artifact finalization.

## Work Items (P0)

### Step 0: Documentation

- ✅ Create `docs/development/current/main/phases/phase-131/README.md`
- ✅ Update `docs/development/current/main/10-Now.md` Next section

### Step 1: Fixtures + Smokes

- New fixture: `apps/tests/phase131_loop_true_break_once_min.hako`
  - Expected exit code: `1`
  - Form: x=0; loop(true) { x=1; break }; return x
- VM smoke: `tools/smokes/v2/profiles/integration/apps/phase131_loop_true_break_once_vm.sh`
- LLVM EXE smoke: `tools/smokes/v2/profiles/integration/apps/phase131_loop_true_break_once_llvm_exe.sh`

### Step 2: Implementation

- New file: `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs`
  - Box: `LoopTrueBreakOnceBuilderBox`
  - Accept: `loop(true) { body ; break }` only
  - Reject (Ok(None)): Non-matching patterns
  - Generate: main → loop_step → loop_body → k_exit (no PHI)
- Integration: Add to `src/mir/control_tree/normalized_shadow/builder.rs` orchestrator

### Step 3: Verification

```bash
cargo test --lib

# Phase 131 smokes
bash tools/smokes/v2/profiles/integration/apps/phase131_loop_true_break_once_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase131_loop_true_break_once_llvm_exe.sh

# Regressions (minimum 2)
bash tools/smokes/v2/profiles/integration/apps/phase130_if_only_post_if_add_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase97_next_non_ws_llvm_exe.sh
```

### Step 4: Commits

1. `test(joinir): Phase 131 loop(true) break-once fixture + VM/LLVM smokes`
2. `feat(control_tree): Phase 131 normalized loop(true) break-once builder (dev-only)`
3. `docs: Phase 131 P0 DONE`

## Implementation Notes

### Fail-Fast vs Ok(None)

- **Out of scope patterns**: Return `Ok(None)` (not an error, just unsupported)
- **Contract violations in supported patterns**: Fail-Fast with `freeze_with_hint`
- **Internal errors**: Return `Err(...)`

### Reusing Phase 130 Infrastructure

- `LegacyLowerer::lower_assign_stmt`: Reuse for body assignments
- `EnvLayout`: Same SSOT for env parameter management
- `alloc_value_id`, `build_env_map`, `collect_env_args`: Same helper patterns

### Loop Detection

Check StepNode structure:
```rust
match &step_tree.root {
    StepNode::Loop { cond_ast, body, .. } => {
        // Check cond_ast is Bool(true)
        // Check body is Block ending with Break
        // Check no return/continue in body
    }
    StepNode::Block(nodes) => {
        // Check if loop is embedded in block with pre/post statements
    }
    _ => Ok(None)
}
```

## Current Status

Phase 131 P2 - Variable Propagation (2025-12-18)

### ✅ P0 Completed (Structure)

- **Fixtures**: `apps/tests/phase131_loop_true_break_once_min.hako` created
- **Builder**: `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs` (407 lines)
- **Integration**: Wired into `builder.rs` orchestrator via `lower_with_loop_support()`
- **Smoke Tests**: VM and LLVM EXE smoke scripts created
- **Structure**: Shadow JoinModule generation working correctly

### ✅ P1 Completed (Execution Path)

- **Routing**: `try_cf_loop_joinir` now checks Normalized shadow before Pattern2 (`routing.rs`)
- **Dev Gating**: Routes to Normalized when `NYASH_JOINIR_DEV=1`
- **Merge Pipeline**: Uses existing `JoinIRConversionPipeline` (JoinModule → MirModule → merge)
- **Execution**: Loop executes successfully without freeze (verified in trace output)
- **Void Return**: Emits void constant when loop has no explicit return value

**Evidence of Success**:
```
[joinir/meta] MirFunc 'main', 'loop_step', 'loop_body', 'k_exit' converted
[cf_loop/joinir] Phase 189: Merge complete: 4 functions merged
[trace:debug] build_block: Statement 4/4 ... (loop completed, execution continues)
RC: 0  (program completes without freeze)
```

### ✅ P1.5 Completed (DirectValue Exit Reconnection)

**DirectValue mode**: Normalized shadow path does not build exit PHIs. Final env values are reconnected directly to host `variable_map`.

**Example**:
```hako
x = 0
loop(true) { x = 1; break }
return x  // Returns 1 ✅
```

**Root Cause** (fixed): PHI-based exit merge assumptions did not match continuation-based Normalized control flow.

**Fix**:
- Introduce `ExitReconnectMode::DirectValue` and skip exit PHI generation in that mode.
- Carry `remapped_exit_values` through the merge result and update host `variable_map` directly.

### ✅ P2 Completed (k_exit contract alignment)

**Problem**: continuation entry-jumps / exit edges were inconsistent, so the updated env visible at `k_exit` could be lost.

**Fix**: normalize tail-call/exit edges so updated env reaches the exit reconnection point.

### Deliverables

Phase 131 P2 provides:
- ✅ Execution path fully wired (dev-only, `NYASH_JOINIR_DEV=1`)
- ✅ Loop completes without freeze and returns the updated value
- ✅ DirectValue mode skips exit PHIs (PHI-free)
- ✅ Existing patterns unaffected (既定挙動不変)

### Design Notes (historical)

- `docs/development/current/main/phases/phase-131/p1.5-root-cause-summary.md`
- `docs/development/current/main/phases/phase-131/p1.5-option-b-analysis.md`
- `docs/development/current/main/phases/phase-131/p1.5-implementation-guide.md`
