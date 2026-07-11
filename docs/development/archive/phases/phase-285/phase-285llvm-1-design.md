# Phase 285LLVM-1: WeakRef LLVM Implementation Design

**Status**: Design phase (no implementation yet)
**Date**: 2025-12-24
**Related**: Phase 285A0 (VM WeakRef implementation), Phase 285LLVM-0 (LLVM Leak Report)

## Goal

LLVM backendでWeakRefサポートを実装し、VM parityを達成する

## Background

### 現状
- **VM**: WeakRef完全実装済み（Phase 285A0）
  - `VMValue::WeakBox(Weak<dyn NyashBox>)` 実装
  - 8テスト全てPASS
  - `apps/tests/phase285_weak_*.hako` にfixtureあり
- **LLVM**: 完全未実装
  - `src/llvm_py/instruction_lower.py` に WeakNew/WeakLoad handler存在せず
  - fallback処理もなし（未知命令は無視される）

### 制約
- **WeakRef field契約**: フロントエンド（parser/MIR builder）の責務
  - 全バックエンドで既に効いている
  - LLVM側で実装不要
- **GC統合**: WeakRefはGC不要で動作（デザイン済み）

## Runtime表現（候補）

### 候補: Option B (i64 with convention, bit 63 = weak marker)

**理由**:
- 最小FFI overhead（既存のhandle infrastructureを再利用）
- 実装が単純
- 後でtagged unionに変更可能

**注意**:
⚠️ **Phase 285LLVM-1実装前に既存LLVM値表現との整合性確認が必要**
- 既存のhandle/boxing方式と衝突しないか検証
- i64の最上位ビット（bit 63）が他の目的で使われていないか確認

### 代替案

**Option A**: Tagged Pointer `{i32 tag, i64 handle}`
- ✅ 型安全
- ❌ 実装が複雑
- ❌ FFI overhead増加

**Option C**: Opaque Pointer
- ✅ Future-proof
- ❌ Runtime support必要
- ❌ より大きな変更が必要

## weak_to_strong() 返り値

**SSOT** (`docs/reference/language/lifecycle.md:179`):
- 成功: Strong BoxRef (i64 handle > 0)
- 失敗: null (i64(0) = Void)

## FFI API

Phase 285LLVM-1で実装する3つのFFI関数（`crates/nyash_kernel/src/lib.rs` または相当箇所）:

```rust
#[no_mangle]
pub extern "C" fn nyrt_weak_new(strong_handle: i64) -> i64 {
    // Convert strong handle to weak handle
    // Implementation: Phase 285LLVM-1
    unimplemented!("Phase 285LLVM-1")
}

#[no_mangle]
pub extern "C" fn nyrt_weak_to_strong(weak_handle: i64) -> i64 {
    // Upgrade weak to strong
    // Returns: strong handle (>0) on success, 0 (Void) on failure
    unimplemented!("Phase 285LLVM-1")
}

#[no_mangle]
pub extern "C" fn nyrt_weak_drop(weak_handle: i64) {
    // Release weak reference
    unimplemented!("Phase 285LLVM-1")
}
```

## MIR Lowering

LLVM IR生成時の変換（`src/llvm_py/instruction_lower.py` で実装）:

```python
# WeakNew: strong → weak 変換
# MIR: WeakNew(dst=ValueId(10), box_val=ValueId(5))
# LLVM IR: %10 = call i64 @nyrt_weak_new(i64 %5)

elif op == "weak_new":
    box_val = lower_value(inst["box_val"])
    result = builder.call(nyrt_weak_new_fn, [box_val])
    store_value(inst["dst"], result)

# WeakLoad: weak → strong 変換（失敗時は0）
# MIR: WeakLoad(dst=ValueId(20), weak_ref=ValueId(10))
# LLVM IR: %20 = call i64 @nyrt_weak_to_strong(i64 %10)

elif op == "weak_load":
    weak_ref = lower_value(inst["weak_ref"])
    result = builder.call(nyrt_weak_to_strong_fn, [weak_ref])
    store_value(inst["dst"], result)
```

## テスト戦略

### Fixture再利用
VM fixtureを再利用（`apps/tests/phase285_weak_*.hako`）:
- `phase285_weak_basic.hako` - 基本動作
- `phase285_weak_upgrade.hako` - weak_to_strong()成功/失敗
- その他6ファイル（合計8 fixtures）

### Parity検証
VM/LLVM出力parity検証:
- stdout比較（同じ出力であることを確認）
- exit code比較（同じ終了コードであることを確認）

### スモークテスト
`tools/smokes/v2/profiles/quick/lifecycle/phase285_weak_basic_llvm.sh`:
- 現在SKIP中（Phase 285LLVM-1 message）
- 実装後にSKIP解除

## 実装チェックリスト

### ファイル変更/新規作成

- [ ] **新規**: `src/llvm_py/instructions/weak.py`
  - WeakNew/WeakLoad handler実装
  - `instruction_lower.py` から分離されたモジュール

- [ ] **変更**: `src/llvm_py/instruction_lower.py`
  - `elif op == "weak_new"` handler追加
  - `elif op == "weak_load"` handler追加
  - `weak.py` をimport

- [ ] **変更**: `crates/nyash_kernel/src/lib.rs` (または相当箇所)
  - `nyrt_weak_new()` 実装
  - `nyrt_weak_to_strong()` 実装
  - `nyrt_weak_drop()` 実装

- [ ] **変更**: `tools/smokes/v2/profiles/quick/lifecycle/phase285_weak_basic_llvm.sh`
  - `test_skip` 削除
  - 実際のテスト実装（VM版と同じパターン）

### 検証ステップ

1. [ ] **Runtime表現の整合性確認**
   - 既存LLVM値表現（handle/boxing方式）と衝突しないか検証
   - bit 63が他で使われていないか確認

2. [ ] **FFI関数単体テスト**
   - `nyrt_weak_new()` が正しいweak handleを返すか
   - `nyrt_weak_to_strong()` が成功時にstrong handle、失敗時に0を返すか
   - `nyrt_weak_drop()` が正しくリソースを解放するか

3. [ ] **MIR lowering検証**
   - WeakNew命令が正しくLLVM IRに変換されるか
   - WeakLoad命令が正しくLLVM IRに変換されるか
   - 生成されたLLVM IRが実行可能か

4. [ ] **VM/LLVM parity検証**
   - 8 fixtureでVM/LLVMの出力が一致するか
   - exit codeが一致するか

5. [ ] **スモークテスト**
   - `phase285_weak_basic_llvm.sh` PASS確認
   - 回帰テスト（既知FAIL以外増えないか）

## 前提条件

- ✅ VM WeakRef完全実装済み（Phase 285A1）
- ⚠️ Runtime表現候補選定済み（Option B、実装前に既存LLVM値表現との整合性確認必要）
- ✅ FFI signatures定義済み（本ドキュメント）
- ✅ Phase 285LLVM-0完了（LLVM Leak Report実装済み）

## 実装時の注意点

1. **Fail-Fast原則**: エラーは早期に明示的に失敗させる
   - weak_to_strong()失敗時はnull (i64(0))を返す
   - フォールバック処理は入れない

2. **SSOT準拠**: `docs/reference/language/lifecycle.md` の仕様に厳密に従う
   - weak_to_strong()の返り値は仕様通り
   - WeakRef semanticsは変更しない

3. **テスト駆動**: VM fixtureで動作確認してから進める
   - 実装→テスト→修正のサイクルを回す
   - VM parityを常に確認

## 成功基準

**Phase 285LLVM-1 完了時**:
- ✅ WeakNew/WeakLoad命令のLLVM IR生成が動作
- ✅ FFI関数3つ（nyrt_weak_new, nyrt_weak_to_strong, nyrt_weak_drop）実装済み
- ✅ 8 fixtureでVM/LLVM parity達成（stdout + exit code一致）
- ✅ `phase285_weak_basic_llvm.sh` スモークテストPASS
- ✅ 回帰なし（既知FAIL以外増えない）

## 参考資料

- VM実装: `src/runtime/execution/vm/value.rs` (`VMValue::WeakBox`)
- WeakRef SSOT: `docs/reference/language/lifecycle.md:170-180`
- Fixtures: `apps/tests/phase285_weak_*.hako`
- Phase 285A0: VM WeakRef実装（完了）
- Phase 285LLVM-0: LLVM Leak Report実装（完了）
