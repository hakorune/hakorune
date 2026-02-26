---
Status: Active
Decision: accepted
Date: 2026-02-26
Scope: WASM lane の文法→MIR→WASM対応表を固定し、`WSM-02+` の実装順を最小ステップで明示する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-117-wsm01-wasm-unsupported-inventory-sync-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - src/parser/statements/control_flow.rs
  - src/mir/builder/stmts/async_stmt.rs
  - src/backend/wasm/codegen/instructions.rs
  - src/backend/wasm/codegen/builtins.rs
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-118 WASM Grammar Compatibility Map SSOT

## 0. Goal

WASM lane の議論を「どの文法がどこまで動くか」で固定する。

1. `.hako` 文法要素ごとの実行可否を明示
2. 可否の根拠を MIR/WASM 実装ファイルへ紐づける
3. `WSM-02+` の順番を 1 blocker = 1 受理形で固定

## 1. Grammar -> MIR -> WASM Compatibility

| Grammar element (.hako) | MIR shape (representative) | WASM status | Evidence |
|---|---|---|---|
| `if / else` | `MirInstruction::Branch` + `Jump` + PHI merge | supported | `src/parser/statements/control_flow.rs`, `src/mir/builder/if_form.rs`, `src/backend/wasm/codegen/instructions.rs` |
| `loop` | `Jump` + `Branch` loop form | supported | `src/parser/statements/control_flow.rs`, `src/mir/loop_api.rs`, `src/backend/wasm/codegen/instructions.rs` |
| integer/bool const | `MirInstruction::Const` | supported | `src/backend/wasm/codegen/instructions.rs` |
| binary arith/logic | `MirInstruction::BinOp` | supported (subset) | `src/backend/wasm/codegen/instructions.rs` |
| compare | `MirInstruction::Compare` | supported (subset) | `src/backend/wasm/codegen/instructions.rs` |
| external call | `MirInstruction::Call { callee: Extern }` | partial | supported: `env.console.log`, `env.console.warn`, `env.console.error`, `env.console.info`, `env.console.debug`, `env.canvas.fillRect`, `env.canvas.fillText` in `src/backend/wasm/codegen/instructions.rs` |
| method call | `MirInstruction::Call { callee: Method }` | partial | supported methods: `toString`, `print`, `equals`, `clone`, `log`, `info` in `src/backend/wasm/codegen/builtins.rs` |
| `nowait/await` | `FutureNew`, `Await`, `Safepoint` | stub-supported | `src/mir/builder/stmts/async_stmt.rs`, `src/backend/wasm/codegen/instructions.rs` (sync-like stub, no real scheduler) |
| local/assignment | `Copy`, `ReleaseStrong` 等が出る経路あり | partial | `src/mir/builder/stmts/variable_stmt.rs`, `src/mir/builder/builder_build.rs`, `Copy`/`ReleaseStrong`/`KeepAlive` は `src/backend/wasm/codegen/instructions.rs` で対応済み。`Load`/`Store` は未対応。 |

## 2. Thread / Concurrency Contract

1. 現行 WASM lane に OS thread / worker scheduler 実装はない。
2. `nowait/await` は文法・MIR上は存在するが、WASM backend では同期スタブとして lower される。
3. 本格並行（shared memory / atomics / worker orchestration）は scope 外（別proposal）。

## 3. WSM-02+ Fixed Order

- `WSM-02a`: assignment/local path unblock（`Copy`/`ReleaseStrong` など頻出命令の最小対応）[done]
- `WSM-02b`: ExternCall coverage expansion（1 extern familyずつ）[active]
- `WSM-02c`: BoxCall coverage expansion（1 method familyずつ）
- `WSM-02d`: boundary gates（supported/unsupported 両境界を fixture で固定）

Rule:
- 1 commit = 1 blocker = 1受理形
- unsupported path は fail-fast 維持（silent fallback 禁止）

## 4. Acceptance

Daily:
1. `cargo check --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh`

Milestone (WASM lane touched):
1. wasm lane fixture pack（WSM-02d で追加）
2. `bash tools/vm_plugin_smoke.sh`
