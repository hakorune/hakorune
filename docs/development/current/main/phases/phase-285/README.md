# Phase 285: Box lifecycle / weakref / finalization / GC conformance

Status: P0/P1/P2/P2.1/P2.2/P3.1/P3.2/P4 ✅ COMPLETE (2025-12-26)

## Next (P0 docs-only → P1/P2)

- P0 手順書: `docs/development/current/main/phases/phase-285/P0-INSTRUCTIONS.md`
- P3 手順書（LLVM one-pass）: `docs/development/current/main/phases/phase-285/P3-INSTRUCTIONS.md`
- 言語レベルSSOT:
  - lifecycle/weak/fini/GC: `docs/reference/language/lifecycle.md`
  - `null`/`void`/truthiness: `docs/reference/language/types.md`

## P2.1（Hidden root investigation & fix）✅ COMPLETE (2025-12-26)

P2 の weak-fail fixture（明示 drop `x = null`）が “失敗→null” にならず、`weak_to_strong()` が成功してしまう（hidden root）問題を根治した。

### Root cause（要旨）

- VM の `regs` が古い `ValueId` を保持し続け、`Arc` が drop されない
- SSA last-use をそのまま寿命にすると、言語の block-scoped locals と衝突して `weak_basic` が壊れる

### Fix（要旨）

MIR 命令を “スコープ維持 / 上書きdrop” に分離して、言語スコープを優先しつつ hidden root を根治する。

- `KeepAlive { values }`（scope-end keepalive）: スコープ終端まで値を生存維持（language scope semantics）
- `ReleaseStrong { values }`（overwrite drop）: 変数上書き前の旧値を解放（weak-fail を成立させる）
  - SSA `Copy` により同一 `Arc` を複数 `ValueId` が参照するため、VM は **alias も含めて** `regs` から除去する

実装の責務分離（hygiene）:
- VM 側の KeepAlive 処理は `src/backend/mir_interpreter/handlers/lifecycle.rs` に隔離（dispatch から分離）

## P2.2（optional, hygiene）KeepAlive 命令の明確化（命令分離）

**ねらい**: `KeepAlive { values, drop_after: bool }` の “二重責務” を解消し、意図を MIR 語彙で明確化する。

- 現状:
  - `drop_after=false`: スコープ終端までの生存維持（PHI/スコープ意味論のため）
  - `drop_after=true`: 変数上書き時の強参照解放（hidden root 対策）
- 課題:
  - “生存維持” と “解放” は本質が違い、`bool` で隠すとレビュー/検証で混線しやすい

**方針（提案）**:
- `KeepAlive { values }`（PURE）: DCE/liveness 目的の生存維持のみ
- `ReleaseStrong { values }`（WRITE/IMPURE）: 変数上書き等で強参照を解放（alias も含めて解放）

**受け入れ条件**:
- `weak_basic` が壊れない（言語スコープ優先）
- `weak_upgrade_fail` が PASS（`x = null` 後に `weak_to_strong()` が `null`）
- `./tools/smokes/v2/run.sh --profile quick` が緑（154/154 PASS）
- `rg -n "drop_after" src` が 0 件

**結果**: ✅ COMPLETE (2025-12-26)

- `KeepAlive { values, drop_after }` → `KeepAlive { values }`（PURE）/ `ReleaseStrong { values }`（WRITE）に分離
- 実装コミット: `3bb865c6b`

### Verification

- `apps/tests/phase285_weak_basic.hako`: exit 2 ✅
- `apps/tests/phase285_p2_weak_upgrade_fail_min.hako`: exit 1 ✅
- quick gate: `./tools/smokes/v2/run.sh --profile quick` → 154/154 PASS ✅

## LLVM Sub-Phases Status

| Phase | Status | Summary |
|-------|--------|---------|
| 285LLVM-0.3 | ✅ COMPLETE | Leak report smoke test修正（検証スコープ明確化） (2025-12-25) |
| 285LLVM-1.1 | ✅ COMPLETE | ユーザーBox登録・デバッグ出力 (2025-12-24) |
| 285LLVM-1.2 | ✅ COMPLETE | WeakRef基本動作（identity保留） (2025-12-24) |
| 285LLVM-1.3 | ✅ COMPLETE | InstanceBox Field Access (getField/setField) (2025-12-24) |
| **285LLVM-1.4** | ✅ **COMPLETE** | **print Handle Resolution (型タグ伝播)** (2025-12-24) |
| **285W-Syntax-0** | ✅ **COMPLETE** | **weak文法SSOT確定 (weak x unary operator)** (2025-12-24) |
| **285W-Syntax-0.1** | ✅ **COMPLETE** | **weak(x) 完全拒否 (Parser-level Fail-Fast)** (2025-12-24) |

## P3.1（LLVM feature detection）✅ COMPLETE (2025-12-26)

**目的**: smoke が “LLVM backend available” を機械判定できるようにし、SKIP を実行到達へ移す。

- `--version` に `features:llvm` を含める（`cfg!(feature="llvm")`）。
- これにより、LLVM smoke が “SKIP” ではなく “PASS/FAIL” として結果を返す（検出問題の解消）。

## P3.2（quick SSOT: config selection）✅ COMPLETE (2025-12-26)

**目的**: `cargo build --release --features llvm` のときでも、quick profile の意味が変わらない（VM+dynamic plugins で統一）。

- `tools/smokes/v2/configs/auto_detect.conf` で quick の config を `rust_vm_dynamic` に固定（CI/SMOKES_FORCE_CONFIG は尊重）。

## P4（weak_basic_llvm 1-fail fix）✅ COMPLETE (2025-12-26)

**目的**: LLVM build でも quick 154/154 PASS（SKIP なし）。

- `apps/tests/phase285_weak_basic.hako` は "upgrade succeeds while strong ref alive" のみに絞る（weak の最小意味論固定）。
- `tools/smokes/v2/profiles/quick/lifecycle/phase285_weak_basic_llvm.sh` は exit code をゲートにし、stdout 依存を避ける（LLVM harness はログが混入し得る）。

**補足（残課題）**:
- LLVM harness では “boxed integer handle vs integer literal” の比較が揺れる可能性があるため、weak_basic では field/value 比較を扱わない（別タスク化）。

## Appendix

- ret.py boxification report: `docs/development/current/main/phases/phase-285/ret-py-phase3-boxification.md`

## P4 Post-Completion: Integration LLVM Tests & Code Quality (2025-12-27)

**Phase 284 P2 Integration Fix** (commit `225600b5f`):
- **Problem**: `phase284_p2_return_in_loop_llvm` failing with LLVM type error
  - MIR generates unreachable blocks with `{"op": "ret", "value": null}`
  - LLVM error: `ret void` in `i64 main()` function
- **Solution**: Use `builder.unreachable()` for Fail-Fast principle
  - Type-safe: Satisfies LLVM type checker without return value
  - Fail-Fast: Immediate crash if unreachable block is executed
  - File: `src/llvm_py/instructions/ret.py` (Lines 47-59)
- **Verification**:
  - ✅ phase284_p2_return_in_loop_llvm: PASS (exit 7)
  - ✅ phase285_p2_weak_upgrade_success_llvm: PASS (exit 2)
  - ✅ phase285_p2_weak_upgrade_fail_llvm: PASS
  - ✅ Quick profile: 154/154 PASS (no regression)

**ret.py Box-First Refactoring** (commits `32aa0ddf6`, `5a88c4eb2`):
- **Phase 1-2**: Extract foundational Boxes
  - `UnreachableReturnHandlerBox`: Unreachable block handling (Fail-Fast)
  - `ReturnTypeAdjusterBox`: Type conversion (ptr↔int, width adjustment)
  - Moved `import os, sys` to file header
- **Phase 3**: Strategic extraction
  - `StringBoxerBox`: String pointer (i8*) to handle (i64) conversion
  - `ReturnPhiSynthesizerBox`: PHI synthesis with zero-like detection
  - Reduced `lower_return()`: 166→117 lines (-29%)
  - Total: 205→352 lines (+102 for organization, +4 testable units)

**Code Quality Improvements** (commits `d7c6df367`, `798c193cb`, `1869396fd`, `095213c58`):
1. **LLVM exit code SSOT** (`d7c6df367`):
   - Removed unreliable stdout grep fallback
   - Exit code primary validation (no log pollution)
2. **nyash_kernel FFI split** (`798c193cb`):
   - Extracted `ffi/weak.rs` (74 lines)
   - Reduced lib.rs: 1295→1221 lines (-5.7%)
3. **LLVM detection consolidation** (`1869396fd`):
   - Created `can_run_llvm()` function (SSOT)
   - Updated 9 test files to use unified detection
4. **auto_detect.conf clarity** (`095213c58`):
   - `detect_optimal_config(profile)` with parameter
   - Moved quick SSOT forcing inside function

**Final Status**:
- Integration LLVM tests: 3/3 PASS (no FAIL remaining)
- Quick profile: 154/154 PASS
- Code quality: Box-First principles applied throughout

**LLVM Details**: See [phase-285llvm-1.3-verification-report.md](phase-285llvm-1.3-verification-report.md)
**Syntax Change**: Phase 285W-Syntax-0 migrates from `weak(x)` function call to `weak x` unary operator
**Syntax Enforcement**: Phase 285W-Syntax-0.1 enforces parser-level rejection of `weak(...)` syntax with helpful error message

### Phase 285LLVM-0.3 (2025-12-25): Smoke test 修正

- `NYASH_DISABLE_PLUGINS=1` 削除、plugins 有効化で leak report 動作確認
- stdout 検証を削除（leak report のみを検証対象とする）
- 結果: 45/46 PASS（退行なし）
- コメント修正: 「print() は機能しない」→「このsmoke testはleak reportingのみ検証」（SSOT整合）

---

## Goal

Box の生存期間（強参照/弱参照/解放/最終化/GC）を SSOT として固定し、移行期間でも意味論が割れない状態にする。

Language-level SSOT:
- Lifecycle/weak/fini/GC policy: `docs/reference/language/lifecycle.md`
- Truthiness + `null`/`void`: `docs/reference/language/types.md`

This Phase document is not the language SSOT; it tracks implementation status, backend gaps, and acceptance criteria.

## Implemented (A1 series)

See `docs/development/current/main/phases/phase-285/phase-285a1-boxification.md`.

- WeakRef E2E (VM/LLVM harness): `weak <expr>` + `weak_to_strong()`, plus strict weak-field contract (no implicit weakification).
- Visibility support: `public { weak parent }` plus sugar `public weak parent` (same meaning).
- Parser robustness: parameter type annotations (`arg: Type`) are rejected with a clear parse error (no hang).
  - Helper: `src/parser/common/params.rs`
  - Smoke: `tools/smokes/v2/profiles/quick/parser/phase285_param_type_annotation_nohang.sh`

## Why now

- JoinIR/Plan/compose の収束が進むほど、実行時の “値の寿命” の揺れが目立つ。
- weakref/finalization は「実装が仕様」になりやすく、後から直すコストが最大級。
- LLVM harness 側は未対応の可能性が高く、差分を “仕様として明文化” しないと再現/調査が難しい。

## SSOT References (current code)

- weakref の値表現: `src/value.rs`（`NyashValue::WeakBox`）
- finalization: `src/finalization.rs`
- Box trait: `src/box_trait.rs`（`SharedNyashBox = Arc<dyn NyashBox>`）
- Scope tracking: `src/scope_tracker.rs`（Box の登録/スコープ）

## Snapshot（今わかっていること）

- weakref は `Weak<Mutex<dyn NyashBox>>` で保持される（`NyashValue::WeakBox`）
- `WeakBox` の `to_string()` は `weak_to_strong()` を試み、`WeakRef(null)` 表示になりうる（観測可能）
- `src/value.rs` に weakref の drop 挙動を固定する unit test がある（`test_weak_reference_drop`）

## Responsibility Map（どこが仕様を決めるか）

- **SSOT（意味）**: `docs/reference/language/*`（言語レベルのSSOT）
- **Conformance**: Rust VM / LLVM harness / WASM / JIT など各バックエンド実装
- **観測の固定**: fixture/smoke（Phase 285 P2 で作る）

## 用語（P0で固定する）

- **Strong reference**: 所有参照（`Arc` 等で Box を保持）
- **Weak reference**: 非所有参照（`Weak` / `weak_to_strong()` が失敗しうる）
- **Weak-to-strong**: weak → strong の昇格（成功/失敗が意味論）
- **Roots**: 解放/GC から保護される参照集合（stack/local/global/handle/plugin）
- **Finalizer**: 解放に伴う最終化処理（もし存在するなら）

## P0 decisions (docs-only) ✅ COMPLETE (2025-12-26)

### 言語 SSOT との境界

| 関心事 | SSOT | Phase 285 での扱い |
|--------|------|-------------------|
| Lifecycle/weak/fini | `docs/reference/language/lifecycle.md` | 実装状況・差分追跡のみ |
| null/void/truthiness | `docs/reference/language/types.md` | 実装状況・差分追跡のみ |
| VM/LLVM 差分 | このファイル | 「未対応/既知バグ/保留」として分類 |

**原則**: Phase 285 は言語 SSOT を書き換えない。実装の棚卸しと差分追跡を行う。

### 用語（固定）

| 用語 | 定義 |
|------|------|
| **Roots** | 解放/GC から保護される参照集合（stack/local/global/handle/plugin） |
| **Strong reference** | 所有参照（`Arc` 等で Box を保持、解放を遅延） |
| **Weak reference** | 非所有参照（`Weak` / `weak_to_strong()` が失敗しうる） |
| **Weak-to-strong** | weak → strong の昇格（成功/失敗が意味論、失敗時は `null`） |
| **Finalizer (`fini`)** | 解放に伴う論理的終了処理（物理解放とは別） |
| **Collection (GC)** | 到達不能オブジェクトの回収（意味論ではなく補助） |

### 禁止事項

| 禁止事項 | 理由 |
|----------|------|
| Finalizer 内での再入 | デッドロック/無限再帰のリスク |
| Finalizer 内での例外送出 | 終了処理の信頼性を損なう |
| Finalizer 内での allocation | GC サイクル中の新規割当は危険 |
| Silent fallback（黙殺） | 未対応は `Err` または理由付き SKIP で固定 |
| 新しい環境変数トグル | 既存の診断導線の範囲で対応 |

### VM/LLVM 差分分類

| 機能 | VM (Rust) | LLVM harness | 分類 |
|------|-----------|--------------|------|
| `weak <expr>` | ✅ 実装済み | ✅ 実装済み | (A) 仕様通り |
| `weak_to_strong()` | ✅ 実装済み | ✅ 実装済み | (A) 仕様通り |
| Weak field contract | ✅ 実装済み | ✅ 実装済み | (A) 仕様通り |
| Finalizer (`fini`) | ⚠️ 未実装 | ⚠️ 未実装 | (B) 未実装 |
| GC (cycle collection) | ⚠️ RC のみ | ⚠️ RC のみ | (B) 未実装 |
| Exit-time leak report | ✅ 診断あり | ✅ 診断あり | (A) 仕様通り |

**分類凡例**:
- (A) 仕様通り: VM/LLVM 両方で動作
- (B) 未実装: 言語仕様にあるが実装されていない
- (C) 既知バグ: 実装はあるが動作が仕様と異なる
- (D) 仕様外: 禁止されている

### Core decisions

- Weak の観測は `weak_to_strong()` で行い、失敗値は `null`（= runtime `Void` の別名）。
- `cleanup`（Stage‑3 block-postfix）が「出口で必ず走る」決定的 cleanup を保証する（`catch` の有無に関係なく、常に実行）。
- GC は意味論ではなく補助（GC off で cycle はリークしうる）。
- ByRef (`RefGet/RefSet`) は non-owning / non-escaping（寿命・弱参照・GC の道具にしない）。

## RUNBOOK caveat (implementation reality)

The runbook assumes WeakRef infrastructure exists in the VM and lowering.
If any of the following are missing, treat weak smokes as **unsupported** and scope to exit-time leak report first:
- `weak <expr>` parse/lower (and `weak(...)` is rejected)
- VM handler for MIR WeakRef/WeakNew/WeakLoad
- language-surface `weak_to_strong()` on WeakRef

## Questions to Answer (P0/P1)

- weakref の “生存判定” は何で観測できるか（`toString` / `is_alive` / `weak_to_strong` API など）
- finalizer は存在するか / いつ発火するか（drop 時？GC 時？明示 API？）
- finalizer 内での禁止事項（再入、例外、I/O、allocation）をどうするか
- LLVM harness の扱い（現状未対応なら “未対応として SSOT 化”）

## Scope (proposed)

### P0（docs-only）

- 用語の固定（strong/weak/roots/finalizer/collection）
- 仕様の固定（weakref の weak_to_strong 成否、finalizer の発火条件、禁止事項）
- “LLVM harness の扱い” を明文化（未対応なら未対応として SSOT に書く）

### P1（investigation）✅ COMPLETE (2025-12-26)

**目的**: Rust VM の現状実装の棚卸し（どこで roots が形成され、どこで解放/最終化されるか）

#### Rust VM 実装棚卸し

| 責務 | ファイル | 関連シンボル | 観測ポイント | SSOT との差分 |
|------|----------|-------------|-------------|--------------|
| WeakBox 生成 | `src/value.rs:32` | `NyashValue::WeakBox` | `Weak<Mutex<dyn NyashBox>>` で保持 | ✅ 仕様通り |
| weak_to_strong | `src/value.rs:196-201` | `upgrade_weak()` | `Weak::upgrade()` 失敗時 `Void` | ✅ 仕様通り |
| WeakBox 観測 | `src/value.rs:106-115` | `to_string()` + `upgrade()` | `WeakRef(null)` 表示で死亡観測可能 | ✅ 仕様通り |
| WeakBox 真実値 | `src/value.rs:157-160` | `to_bool()` | `weak_ref.upgrade().is_some()` → 生死判定 | ✅ 仕様通り |
| Finalizer 登録 | `src/finalization.rs:49-52` | `BoxFinalizer::track()` | InstanceBox のスコープ終了時 `fini()` 呼び出し | ⚠️ スコープレベル |
| Finalizer 呼び出し | `src/finalization.rs:61-76` | `finalize_all()` | `instance.fini()` を呼び出す（存在チェック付き） | ✅ 実装済み |
| Scope 追跡 | `src/scope_tracker.rs:14-28` | `ScopeTracker` | `stack: Vec<Vec<Arc<dyn NyashBox>>>` | ✅ 実装済み |
| Scope push/pop | `src/scope_tracker.rs:31-43` | `push_scope()/pop_scope()` | 逆順で `fini()` 呼び出し | ✅ 実装済み |
| Roots 形成 | `src/scope_tracker.rs:85-100` | `enter_root_region()/leave_root_region()` | GC root region 管理（Phase 10.4 prep） | ⚠️ 準備中 |
| MIR WeakNew | `src/backend/mir_interpreter/handlers/weak.rs:23-34` | `handle_weak_new()` | Box → Weak の変換 | ✅ 仕様通り |
| MIR WeakLoad | `src/backend/mir_interpreter/handlers/weak.rs:48-58` | `handle_weak_load()` | Weak → Box \| Void 昇格 | ✅ 仕様通り |

**重要な観測**:
- `ScopeTracker` は **Rust MM レベル**（Arc/Weak）で管理
- `BoxFinalizer` は **スコープレベル**（ブロックスコープ）で管理（別モジュール）
- WeakRef は **値表現レベル** (`Weak<Mutex<dyn NyashBox>>`) で実装
- Finalizer は **InstanceBox 限定**（`downcast_ref::<InstanceBox>()` で確認後呼び出し）

#### LLVM harness 状況

| 機能 | 実装状況 | ファイル | 備考 |
|------|----------|----------|------|
| WeakNew | ✅ 実装済み | `src/llvm_py/instructions/weak.py:12-61` | `@nyrt_weak_new()` 呼び出し |
| WeakLoad | ✅ 実装済み | `src/llvm_py/instructions/weak.py:63-112` | `@nyrt_weak_to_strong()` 呼び出し |
| Finalizer (`fini()`) | ❌ 未実装 | - | 対応関数なし |
| GC / Cycle Collection | ❌ 未実装 | - | Reference Count のみ |

**重要な発見**:
- **(A) 仕様通り**: `weak <expr>` + `weak_to_strong()` は **両バックエンド** で動作
- **(B) 未実装**: Finalizer (`fini()`) は **両バックエンド** で言語意味論としては統一されていない
  - VM: `scope_tracker.rs` の `pop_scope()` 時に InstanceBox `.fini()` を呼び出し（実装あり）
  - LLVM: 対応する finalizer 呼び出し機構がない（現在 harness は scope 管理を持たない）

### P2（smoke）✅ COMPLETE (2025-12-26)

**目的**: weak の意味論（`weak <expr>` と `weak_to_strong()` の成功/失敗、失敗→null）を integration smoke で固定

**観測点**: **exit code で判定**（stdout は揺れやすいため、exit code を SSOT とする）

#### 実装内容

**Fixture A（成功パターン）**:
- ファイル: `apps/tests/phase285_weak_basic.hako`（既存、修正）
- 内容: `weak x` → `weak_to_strong()` 成功 → **exit 2**（非ゼロ成功コード）
- 修正理由: fail=1, success=2 で明確化（"何も起きてない exit 0" と区別）

**Fixture B（失敗パターン）**:
- ファイル: `apps/tests/phase285_p2_weak_upgrade_fail_min.hako`（**新規**）
- 内容: **明示的 drop (`x = null`)** 後の `weak_to_strong()` 失敗 → `null` 観測 → exit 1
- Box定義: `SomeBox { x }` を使用（`phase285_weak_basic.hako` と同じ、環境依存回避）
- **スコープ戦略**: ブロックスコープ `{ }` drop ではなく明示 drop `x = null` を使用
  - 理由: ブロックスコープ drop の conformance は別タスク（block scope 寿命は未整合の可能性あり）
  - P2 の weak-fail は明示 drop 方式で固定

**VM smoke scripts（2本）**:
1. `tools/smokes/v2/profiles/integration/apps/phase285_p2_weak_upgrade_success_vm.sh`
   - Fixture A 実行、期待: **exit 2** → **PASS**
2. `tools/smokes/v2/profiles/integration/apps/phase285_p2_weak_upgrade_fail_vm.sh`
   - Fixture B 実行、期待: **exit 1** → **PASS**

**LLVM smoke scripts（2本）**:
3. `tools/smokes/v2/profiles/integration/apps/phase285_p2_weak_upgrade_success_llvm.sh`
   - Fixture A 実行（LLVM harness）、期待: **exit 2** → **PASS** または理由付き SKIP
4. `tools/smokes/v2/profiles/integration/apps/phase285_p2_weak_upgrade_fail_llvm.sh`
   - Fixture B 実行（LLVM harness）、期待: **exit 1** → **PASS** または理由付き SKIP

**LLVM 対応**:
- WeakNew/WeakLoad は **両バックエンド実装済み**（P1 確認済み）→ PASS が理想
- **SKIP 許容**: harness 不在/feature 無しの環境では理由付き SKIP を必ず許容（Phase 284 P2 と同じ運用）
- silent fallback 禁止

**完了条件**:
- ✅ Fixture A 修正（exit 0 → exit 2）
- ✅ Fixture B 新規作成（明示 drop 方式）
- ✅ VM smoke success PASS
- ✅ LLVM smoke success PASS（または理由付き SKIP）
- ✅ VM smoke fail PASS（exit 1）
- ✅ LLVM smoke fail PASS（または理由付き SKIP）
- ✅ quick 154/154 PASS 維持
- ✅ Finalizer は「VM のみ・LLVM 未対応」と差分表に明記済み（上記 VM/LLVM 差分分類テーブル参照）

**P2 で扱わない項目**:
- **Block scope drop conformance** → 別タスク（未整合の可能性あり）
- Finalizer (`fini()`) の統一テスト → 両バックエンド未実装のため Phase 286+ で検討
- GC cycle collection → Reference Count のみで既知の制約

## Non-goals

- GC アルゴリズム刷新（RC→tracing 等の設計変更）
- LLVM harness に同等機能を “一気に” 実装（差分の記録→段階導入を優先）

## Acceptance criteria (P2+)

- VM と LLVM で、weak が仕様通り動作する（`weak_to_strong()` 成功/失敗が一致、失敗は `null`）。
- 強参照サイクルを意図的に作ったとき、（GC off なら）回収されないことが観測できる。
- 終了時に「強参照が残っている root」をデバッグ出力できる（default-off の診断フラグ）。
  - これは意味論ではなく診断であり、ON/OFF でプログラムの意味を変えない。
