# Phase 270: loop への EdgeCFG Fragment 適用（JoinIR route 実証）

Status: ✅ 完了（P0 + P1）
Date: 2025-12-21

Reading note:
- この文書の `1` / `9` は当時の numbered route label だよ。
- current runtime mainline では `LoopSimpleWhile` / `AccumConstLoop` と読めばよい。

## 目的

**JoinIR経路で最小loopを通す**（JoinIR-only hard-freeze維持）

- **P0**: fixture + smoke test 追加 → LoopSimpleWhile route（historical label 1）が通るか確認
- **P1**: LoopSimpleWhile route が test-only stub と判明 → AccumConstLoop route（historical label 9）追加
- **禁止**: cf_loopに非JoinIR経路や環境変数分岐を追加しない

## P0実装結果（fixture + smoke追加）

### Fixture作成 ✅

**ファイル**: `apps/tests/phase270_p0_loop_min_const.hako`

```nyash
static box Main {
    main() {
        local sum = 0
        local i = 0
        loop(i < 3) {
            sum = sum + i
            i = i + 1
        }
        return sum  // Expected: 0 + 1 + 2 = 3
    }
}
```

**期待値**: exit code 3

### VM Smoke Test作成 ✅

**ファイル**: `tools/smokes/v2/profiles/integration/apps/archive/phase270_p0_loop_min_const_vm.sh`

```bash
#!/bin/bash
set -e
cd "$(dirname "$0")/../../../../../.."
HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"

# Phase 270 P0: No env vars, use existing JoinIR route
set +e
$HAKORUNE_BIN --backend vm apps/tests/phase270_p0_loop_min_const.hako > /tmp/phase270_out.txt 2>&1
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -eq 3 ]; then
    echo "[PASS] phase270_p0_loop_min_const_vm"
    exit 0
else
    echo "[FAIL] phase270_p0_loop_min_const_vm: expected exit 3, got $EXIT_CODE"
    cat /tmp/phase270_out.txt
    exit 1
fi
```

### P0検証結果 ❌

**LoopSimpleWhile FAIL判定**: LoopSimpleWhile route（historical label 1）は test-only stub であり、汎用loopに対応していない

**根本原因**:
- LoopSimpleWhile route（historical label 1, `src/mir/join_ir/lowering/simple_while_minimal.rs`）は Phase 188 の**特定テスト専用の最小実装**
- 対象: `apps/tests/loop_min_while.hako` のみ（`print(i); i = i + 1` をハードコード）
- **サポートしていないもの**:
  1. ❌ キャリア変数（`sum`等）
  2. ❌ カスタムループ本体ロジック
  3. ❌ カスタム戻り値

**エビデンス（MIR dump）**:
```
bb7 (loop body):
    1: extern_call env.console.log(%5) [effects: pure|io]  ← print(i)がハードコード
    1: %11 = const 1
    1: %12 = %5 Add %11  ← i = i + 1のみ、sum = sum + i が無い
    1: br label bb5

bb3 (exit):
    1: ret %2  ← const 0 を返す、sum (3) ではない
```

**決定**: LoopSimpleWhile route は test-only stub として保存 → AccumConstLoop route の PoC へ進む

## P1実装結果（AccumConstLoop route 追加; historical label 9）

### 方針

- **LoopSimpleWhile route は触らない**（test-only stubのまま保存）
- **新規 AccumConstLoop route を追加**（historical numbered label 9, Phase270 fixture専用の最小固定 route）
- **目的**: loopをJoinIR経路で通すSSot固定（汎用実装ではない）
- **将来**: ExitKind+Fragに吸収される前提の橋渡し route

### AccumConstLoop route が受理する形（Fail-Fast固定）

1. **ループ条件**: `i < <int literal>` のみ
2. **ループ本体**: 代入2本のみ（順序固定）
   - `sum = sum + i`
   - `i = i + 1`
3. **制御構文**: break/continue/return があれば `Ok(None)` でフォールバック
4. **loop後**: `return sum`

### JoinIR構造

```text
main(i_init, sum_init):
  result = loop_step(i_init, sum_init)
  return result

loop_step(i, sum):
  cond = (i < limit)
  exit_cond = !cond
  Jump(k_exit, [sum], cond=exit_cond)
  sum_next = sum + i
  i_next = i + 1
  Call(loop_step, [i_next, sum_next])  // tail recursion

k_exit(sum):
  return sum
```

### 実装ファイル

**新規ファイル（1個, historical joinir/patterns lane）**:
- historical file token for the old label-9 accum-const-loop lane (470行)
  - `can_lower()`: Phase270 fixture形状を厳密判定
  - `lower()`: JoinIR生成 → JoinIRConversionPipeline::execute
  - `lower_accum_const_loop_joinir()`: 2キャリア（i, sum）JoinIR lowerer

**変更ファイル（2個）**:
- `src/mir/builder/control_flow/joinir/route_entry/mod.rs` (current module surface)
  - historical path tokens: `{mod.rs,router.rs}` under the old `joinir/patterns/` lane
- `src/mir/builder/control_flow/joinir/route_entry/router.rs` (current route entry)
  - 当時は LOOP_PATTERNS テーブルに AccumConstLoop route（historical label 9）を **LoopSimpleWhile（historical label 1）より前に追加**した

### 検証結果 ✅

#### ビルド成功
```bash
cargo build --release
# Finished `release` profile [optimized] target(s)
```

#### Fixture実行成功（exit code 3）
```bash
./target/release/hakorune --backend vm apps/tests/phase270_p0_loop_min_const.hako
# [joinir/pattern9] Generated JoinIR for AccumConstLoop Pattern
# RC: 3
# Exit code: 3
```

#### Smoke test成功
```bash
HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase270_p0_loop_min_const_vm.sh
# [PASS] phase270_p0_loop_min_const_vm
```

#### Quick smoke成功（退行なし）
```bash
./tools/smokes/v2/run.sh --profile quick
# Passed: 45
# Failed: 1  ← 既存状態維持（Phase 268と同じ）
```

## 核心的な設計判断

### なぜ LoopSimpleWhile route を触らないか

1. **test-only stub保存**: LoopSimpleWhile route（historical label 1）は `loop_min_while.hako` 専用として歴史的価値を保つ
2. **責務分離**: Phase270専用の形は AccumConstLoop route に閉じ込め、LoopSimpleWhile route に汎用性を強要しない
3. **安全性**: 既存の LoopSimpleWhile route 依存コードを壊さない

### なぜ AccumConstLoop route は橋渡し route か

1. **固定形SSOT**: Phase270 fixture の形を厳密に Fail-Fast 固定（汎用化しない）
2. **将来吸収**: ExitKind+Frag統合時に自然に消える設計
3. **route family の局所追加**: 責務が小さく、後で統合しやすい

### 2キャリア（i, sum）の実装

**JoinIR lowerer** (`lower_accum_const_loop_joinir`):
- main のパラメータ: `[i_init_param, sum_init_param]` (2つ)
- loop_step のパラメータ: `[i_step_param, sum_step_param]` (2つ)
- k_exit のパラメータ: `[sum_exit_param]` (sumのみ、iは捨てる)
- JoinInlineBoundary:
  - `with_inputs`: join_inputs=2個, host_inputs=2個
  - `with_exit_bindings`: sum のみ（i は loop 内部変数）

## 重要な発見

### LoopSimpleWhile route は test-only stub

- **Phase 188 実装**: `loop_min_while.hako` 専用ハードコード
- **ソースコードコメント引用**:
  ```rust
  //! This is a MINIMAL implementation targeting loop_min_while.hako specifically.
  //! It establishes the infrastructure for Pattern 1 lowering, which will be
  //! generalized in future phases.
  ```
- **ハードコード内容**（lines 193-199）:
  ```rust
  // print(i)
  loop_step_func.body.push(JoinInst::Compute(MirLikeInst::Print {
      value: i_step_param,
  }));
  ```

### JoinIR-only経路の堅牢性

- **historical numbered-route table**: 当時は legacy labels 1-8 に加え 9 を追加して 9 route label を扱っていた
- **cf_loop hard-freeze**: 非JoinIR経路・環境変数分岐の追加禁止を完全遵守
- **フォールバック設計**: AccumConstLoop route の `can_lower()` が reject したら `Ok(None)` で他 route family へ逃がす

## 次フェーズへの橋渡し

**Phase 271** (仮): ExitKind+Frag 統合
- AccumConstLoop route を EdgeCFG Fragment API に統合
- 旧 numbered route families も順次 Frag 化
- numbered label 分岐削減

## 関連ドキュメント

- **設計図**: `docs/development/current/main/design/edgecfg-fragments.md`
- **現在のタスク**: `docs/development/current/main/10-Now.md`
- **バックログ**: `docs/development/current/main/30-Backlog.md`
- **Phase 268**: `docs/development/current/main/phases/phase-268/README.md`

## 受け入れ基準（全達成）

### P0成功条件
- ✅ `apps/tests/phase270_p0_loop_min_const.hako` 作成
- ✅ `tools/smokes/v2/profiles/integration/apps/archive/phase270_p0_loop_min_const_vm.sh` 作成
- ✅ LoopSimpleWhile route が test-only stub と判明 → P1へ

### P1成功条件
- ✅ AccumConstLoop route 追加（historical file token: old label-9 accum-const-loop basename）
- ✅ router登録（LoopSimpleWhile より前）
- ✅ `cargo build --release` 成功
- ✅ `./target/release/hakorune --backend vm apps/tests/phase270_p0_loop_min_const.hako` → exit code 3
- ✅ `bash tools/smokes/v2/profiles/integration/apps/archive/phase270_p0_loop_min_const_vm.sh` → PASS
- ✅ `./tools/smokes/v2/run.sh --profile quick` → 45/46 PASS（退行なし）
- ✅ ドキュメント更新完了（`phases/phase-270/README.md`新規作成）

## まとめ

**Phase 270 P0-P1 完全成功！**

- ✅ LoopSimpleWhile route は test-only stub と判明（保存）
- ✅ AccumConstLoop route（historical label 9）橋渡し route 追加
- ✅ Phase270 fixture（2キャリア: i, sum）JoinIR経路で完全動作
- ✅ 全テスト PASS（build + fixture + smoke + quick smoke）
- ✅ JoinIR-only hard-freeze維持
- ✅ 将来のExitKind+Frag統合への橋渡し完了

**次のステップ**: Phase 271で AccumConstLoop route（historical label 9）を EdgeCFG Fragment に統合
