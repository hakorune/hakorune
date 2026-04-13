# Concurrency / Async (pre-selfhost) — VM+LLVM stabilization plan

Status: SSOT (design + execution plan)  
Scope: Nyash/Hakorune の **既存構文**（`nowait` / `await`）と、並行の **状態モデル**（`lock<T>` / `scoped` / `worker_local`）を、selfhost 前に **VM + LLVM で矛盾なく動く**ところまで固める。  

Related:
- Semantic SSOT: `docs/reference/concurrency/lock_scoped_worker_local.md`
- Concurrency API (Phase‑0): `docs/reference/concurrency/semantics.md`
- Long-term note (deferred): `docs/development/current/main/design/exception-cleanup-async.md`（state-machine lowering）
- Current lowering: `src/mir/builder/stmts/async_stmt.rs`
- LLVM harness runner: `tools/run_llvm_harness.sh`

---

## 0. Positioning (why this is “pre-selfhost”)

- `nowait` / `await` は既に言語構文として存在しており、selfhost 後に「Rust層と自己ホスト層で意味がズレる」リスクが高い。
- いまの段階で必要なのは “本物のマルチスレッド実装” ではなく、**最小の意味論 + fail-fast + backend 一致**。
- ここで固めるのは **仕様と導線**（SSOT / gate / 最小実装）。高級機能（structured spawn/join、キャンセル、state machine）は後段。

Non-goals (この文書で今すぐやらない):
- “真の並列性” の保証（スレッド/ワーカープールの意味論化）
- 例外 + cleanup + async を統合した state-machine lowering（Phase 260 以降に委譲）

### Structured task-scope vocabulary (Phase 242x)

- user-facing structured concurrency should be read as `task_scope`
- current runtime scaffold behind that boundary is `TaskGroupBox` plus `push_task_scope()` / `pop_task_scope()`
- `RoutineScopeBox` is historical wording only; do not treat it as the current code name
- this vocabulary alignment does **not** change Phase-0 `nowait` / `await` lowering
- detached tasks and sibling-failure policy remain later-phase work

### Detached / root-scope policy (Phase 247x)

- bare `nowait` is **not** detached
- `nowait` inside explicit `task_scope` belongs to that structured scope
- `nowait` outside explicit `task_scope` falls back to the implicit runtime root scope
- current root scope is the `global_hooks` fallback registry used by `register_future_to_current_group(...)`
- `env.task.cancelCurrent` cancels the active explicit scope if present, otherwise the implicit root scope
- detached work remains a future explicit surface; do not read the current root-scope fallback as detached-task semantics

### Sibling-failure policy (Phase 248x)

- current sibling-failure policy is scoped to explicit `task_scope` only
- the first child future that reaches `TaskFailed(error)` becomes the current main failure for that scope
- pending sibling futures owned by the same explicit scope are cancelled with reason `sibling-failed`
- already-ready sibling futures are not rewritten
- implicit root scope does not participate in sibling-failure cancellation in this cut
- aggregate failure reporting remains later-phase work

### Runtime hygiene follow-up (Phase 250x)

- closed scope ownership is now read as a one-way latch:
  - explicit `task_scope` that has been cancelled or has already latched first failure must not accept a late child future as pending work
  - implicit root scope that has been cancelled must not accept a late root-owned future as pending work
- current Phase-0 behavior for such late registrations is immediate `Cancelled(reason)` using the already-latched scope reason
- `FutureBox` success is also terminal:
  - `set_result(...)` is single-assignment just like `set_failed(...)` and `set_cancelled(...)`
  - later writes to a ready future are ignored
- owner-seam cleanup is still in progress:
  - `TaskGroupBox` / `TaskGroupInner` remains the policy box
  - `runtime::global_hooks` may register/push/pop current scope state, but should not define a parallel failure contract
- current plugin/runtime `env.future.await` timeout remains a plugin-only escape hatch:
  - it may still return `ResultBox::Err("Timeout")`
  - that timeout shape is not part of the MIR `Await` contract
  - do not describe it as VM-side `await` semantics

### Scope-exit structured shutdown (Phase 251x)

- each explicit `task_scope` closes independently on scope exit
- current Phase-0 scope-exit rule is:
  - cancel pending futures owned by the popped explicit scope with reason `scope-exit-cancelled`
  - then perform best-effort bounded join for that same explicit scope
- this applies to nested scopes too:
  - popping an inner explicit scope must not defer its cleanup to the outermost scope
  - outer explicit scopes keep their own owner/token state after an inner scope exits
- current `joinAll(timeout_ms)` / scope-exit still do **not** surface first failure or aggregate failure
- current return shape therefore remains `void` / no rethrow in this cut

### Scope-exit first-failure surface (Phase 252x)

- explicit `task_scope` exit now surfaces the popped scope's latched `first_failure`
- current Phase-0 order is:
  - cancel still-pending child futures owned by the popped explicit scope with `scope-exit-cancelled`
  - bounded-join that same explicit scope
  - if `first_failure` is latched, return/rethrow that first failure for the scope-exit path
- this cut is still explicit-scope-only:
  - implicit root scope does not gain scope-exit failure surfacing here
  - `joinAll(timeout_ms)` still does not surface failure in this cut
  - aggregate/multi-failure reporting remains later work

### `joinAll()` first-failure surface (Phase 253x)

- `TaskGroupBox.joinAll(timeout_ms)` now surfaces the same first failure latch used by explicit scope exit
- current Phase-0 public shape is:
  - `ResultBox::Ok(void)` when no first failure is latched after the bounded join
  - `ResultBox::Err(first_failure_payload)` when a first failure is latched
- current timeout behavior is intentionally unchanged:
  - timeout does not yet have a dedicated public error payload
  - a timeout with no latched failure still returns `Ok(void)` in this cut
- current failure payload preservation is now box-based:
  - explicit-scope owner state stores the first failure as a box payload, not only a rendered string
  - scope exit and `joinAll()` now read the same preserved first-failure payload
- aggregate/multi-failure reporting remains a separate later cut

### Aggregate / multi-failure reporting (Phase 254x)

- explicit `task_scope` owner state may now preserve failures beyond the first one
- current aggregation is diagnostic-only:
  - `joinAll(timeout_ms)` still returns only `ResultBox::Ok(void)` / `ResultBox::Err(first_failure_payload)`
  - explicit scope exit still returns/rethrows only the latched first failure
- current aggregate report surface is `TaskGroupBox.failureReport()`
  - returns `ArrayBox`
  - empty when no failed child future has been observed
  - otherwise ordered as `[first_failure, additional_failures...]`
- current aggregate report stores only failed child outcomes:
  - sibling cancellations with reason `sibling-failed` are not appended as extra failures
  - pending siblings cancelled by the first failure remain cancellation side-effects, not aggregate causes
- current aggregate report is explicit-scope-owner only:
  - implicit root scope does not expose aggregate reporting in this cut
  - `pop_task_scope()` does not yet return aggregate failure payloads

---

## 1. Current reality (2026-02-04 snapshot)

### 1.1 Surface / IR
- AST: `ASTNode::Nowait` / `ASTNode::AwaitExpression` が存在する（構文は既にある）。
- MIR: `MirInstruction::{FutureNew, FutureSet, Await}` が存在する。
- Optimizer: `NYASH_REWRITE_FUTURE=1` で Future 命令を `ExternCall env.future.*` に rewrite できる。

### 1.2 Rust VM (MIR interpreter)
- `FutureNew/FutureSet/Await` は実装済み（Phase‑0: resolved FutureBox + `await` は同期ブロック）。
- `nowait` は “spawn” の意味を持たず、式を順次評価して resolved future を作る（Phase‑0 semantics）。

Repro (VM):
- `./target/release/hakorune --backend vm apps/tests/async-await-min/main.hako`
  - expected: exit 42
- `./target/release/hakorune --backend vm apps/tests/async-nowait-basic/main.hako`
  - expected: exit 33
- `./target/release/hakorune --backend vm apps/tests/async-spawn-instance/main.hako`
  - expected: exit 3

### 1.3 LLVM line
- LLVM harness は `--features llvm` でビルドされた `hakorune` が必要（未ビルドだと fail-fast）。
- `tools/run_llvm_harness.sh` が SSOT 導線（ビルド前提込み）。
- 現状の LLVM harness は `FutureNew/FutureSet/Await` を直接 lower しないため、LLVM mode では `NYASH_REWRITE_FUTURE=1` を強制して `ExternCall env.future.{new,set,await}` に寄せる（NyRT 側に対応 export を置く）。

Repro (LLVM harness):
- `tools/run_llvm_harness.sh apps/tests/async-await-min/main.hako`

---

## 2. Phase‑0 semantics to pin (minimal + backend-neutral)

この段階で pin するのは “正しさ” と “導線”。
並列実行の保証はしない（順次実行でも OK）。

### 2.1 `nowait` (spawn-like surface)
- `nowait fut = expr` は “Future 値を得る” 構文である。
- Phase‑0 では `expr` の評価は **順次でもよい**（実装は future を “resolved” として作っても良い）。
- `nowait` が “スレッド” を意味する仕様にはしない。

### 2.2 `await`
- `await fut` は fut が完了していれば値を返す。未完了なら Phase‑0 では待つ（実装は即完了のみでもよい）。
- strict/dev では `await` の前後に `Safepoint` があることを verifier で要求する（既存方針に従う）。

Current VM contract to pin:
- subset/schema gate:
  - `await` requires both `dst` and `future`
  - malformed shapes fail-fast as `await(missing-dst)` / `await(missing-future)`
- runtime gate:
  - `await` requires the `future` operand to hold a `Future`
  - non-`Future` operands fail-fast as `TypeError("Await expects Future in \`future\` operand")`
- completion rule:
  - current VM path blocks until the future is ready, then returns the stored value
  - current Phase‑0 `FutureNew` creates an already-resolved future on the VM path
- current non-goals:
  - no timeout result shape
  - no general cancellation result shape beyond the current scope-owned `Cancelled(reason)` path
  - `task_scope.cancelAll()` does not yet interrupt `await`

Current failure taxonomy to pin:
- `ContractError`
  - malformed `await` shape
  - non-`Future` operand
- `TaskFailed(error)`
  - a future may now complete in a failed terminal state
  - current VM `await` surfaces that state as `VMError::TaskFailed(<stringified error payload>)`
  - current `env.future.await` plugin/runtime route surfaces that state as `ResultBox::Err(error)`
- `Cancelled(reason)`
  - current cuts are explicit scope-owned cancellation only
  - `task_scope.cancelAll()` / current-scope cancellation mark owned pending futures as `Cancelled: scope-cancelled`
  - explicit scope exit marks still-pending owned futures as `Cancelled: scope-exit-cancelled`
  - explicit-scope sibling-failure cancellation marks owned pending sibling futures as `Cancelled: sibling-failed`
  - current VM `await` surfaces that state as `VMError::TaskCancelled(<stringified reason payload>)`
  - current `env.future.await` plugin/runtime route surfaces that state as `ResultBox::Err(reason)`
  - deadline/timeout remains outside the current VM-side `await` contract
  - current plugin/runtime `env.future.await` timeout remains a plugin-only `ResultBox::Err("Timeout")` escape hatch

### 2.3 Method-call `nowait`
最短の selfhost 安定化として、以下のどちらかを SSOT として選ぶ（決め打ちが必要）。

Option A (recommended for Phase‑0):
- method-call `nowait fut = obj.m(args...)` は **通常の式評価**で値を作り、`FutureNew` で包む（spawn_instance を使わない）。
- ねらい: backend に “spawn_instance” の ABI を増やさず、VM/LLVM を揃えやすくする。

Option B (later / full runtime route):
- `ExternCall env.future.spawn_instance` を実装し、scheduler に enqueue できる形にする。
- ねらい: ちゃんと並行にする下地を作る（ただし ABI/実装が増える）。

この文書の実装計画は Option A を前提とする（Option B は後段の拡張として残す）。

---

## 3. `lock<T>` / `scoped` / `worker_local` (meaning model)

意味論 SSOT は `docs/reference/concurrency/lock_scoped_worker_local.md` に固定済み。
ここでは selfhost 前の “実装境界” のみ pin する。

- `local`: call activation / lexical scope。スレッド/ワーカーとは無関係。
- `lock<T>`: 共有 mutable の唯一入口（`lock {}` は構文案。実装は後段でも良い）。
- `scoped`: 文脈（trace/request/config）。**nowait の wrapper ではない**。structured child に継承する（detached は別物）。
- current root-scope fallback does not pin detached/task-local propagation semantics yet; only structured child inheritance is stable
- `worker_local`: cache 専用。意味論に使わない。

---

## 4. Execution plan (1 task = 1 commit)

### CONC‑0 (docs-first) — SSOT + drift inventory
- 目的: “何が動く/動かない” の SSOT を 1 箇所に寄せる（本ファイル）。
- 追加: docs 内の “Implemented” 記述の棚卸し（Future/await 周りの drift を減らす）。

### CONC‑1 (VM) — implement `FutureNew`/`Await` in MIR interpreter (minimal)
- 受け入れ基準:
  - `./target/release/hakorune --backend vm apps/tests/async-await-min/main.hako` が `NYASH_REWRITE_FUTURE` 無しで exit 42
  - `apps/tests/async-nowait-basic/main.hako` が exit 33
- 方針: Phase‑0 は resolved future のみ（即完了）でもよい。スケジューラ連携は後段。

### CONC‑2 (lowering) — method-call nowait を Option A に寄せる
- 変更: `src/mir/builder/stmts/async_stmt.rs` から `env.future.spawn_instance` を消し、式評価 + `FutureNew` へ統一。
- 受け入れ基準:
  - `./target/release/hakorune --backend vm apps/tests/async-spawn-instance/main.hako` が exit 3

### CONC‑3 (LLVM) — harness parity for Phase‑0 futures
- 受け入れ基準:
  - `tools/run_llvm_harness.sh apps/tests/async-await-min/main.hako` が exit 42
  - `tools/run_llvm_harness.sh apps/tests/async-nowait-basic/main.hako` が exit 33
  - `tools/run_llvm_harness.sh apps/tests/async-spawn-instance/main.hako` が exit 3（CONC‑2 後）

### CONC‑4 (gates) — VM+LLVM smoke wiring **(done)**
- Status: **done** (2026‑02‑04)
- Added smokes:
  - `tools/smokes/v2/profiles/integration/async/async_min_vm.sh`
  - `tools/smokes/v2/profiles/integration/llvm/async_min_harness.sh`
- ポリシー: 期待比較は stdout のみ（stderr は診断混入で flake しやすい）。

---

## 5. Restart commands (short)

VM:
- `cargo build --release --bin hakorune`
- `./target/release/hakorune --backend vm apps/tests/async-await-min/main.hako`
- `tools/smokes/v2/profiles/integration/async/async_min_vm.sh`

LLVM:
- `tools/run_llvm_harness.sh apps/tests/async-await-min/main.hako`
- `tools/smokes/v2/profiles/integration/llvm/async_min_harness.sh`
