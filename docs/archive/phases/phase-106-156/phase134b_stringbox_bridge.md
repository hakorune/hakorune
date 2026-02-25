# Phase 134-B: StringBox bridge 分離

## 🎯 ゴール

**StringBox メソッド処理の分散・重複ロジックを統合モジュール化** する。

目的：
- boxcall.py:130-282 の StringBox メソッド処理（length/substring/lastIndexOf）を分離
- **180行の重い処理** を `stringbox.py` に集約
- boxcall.py から **1行の委譲呼び出し** に削減
- Phase 133 ConsoleLlvmBridge / Phase 134-A mir_call パターンを継承

```
Phase 134-A: mir_call.py unified 設計完成 ✅
        ↓
Phase 134-B: StringBox bridge 分離 ← ← ここ！
        ↓
Phase 134-C: CollectionBox bridge 分離
```

---

## 📋 スコープ（やること・やらないこと）

### ✅ やること
- boxcall.py:130-282 の StringBox メソッド処理を分析・抽出
- **StringBoxBridge 箱化モジュール** を実装（src/llvm_py/instructions/stringbox.py）
- StringBox メソッド（length, substring, lastIndexOf）の LLVM IR lowering を統一
- 複雑な最適化パス（NYASH_LLVM_FAST, NYASH_STR_CP）を stringbox.py に集約
- boxcall.py の分岐を削除、1行委譲に削減
- 既存テスト全て通過

### ❌ やらないこと
- StringBox の仕様・セマンティクス変更
- 他の Box 処理（Console/Array/Map等）への影響
- LLVM backend 全体の最適化（StringBox 処理に限定）

---

## 🏗️ 6 つのタスク

### Task 1: 設計ドキュメント作成

**ファイル**: `docs/development/current/main/phase134b_stringbox_bridge.md`（このファイル）

**書く内容**:

#### 現状整理

**boxcall.py 内の StringBox 処理**:
- 行 130-220: length/len メソッド（約90行）
  - NYASH_LLVM_FAST パス（最適化版）
  - literal folding（`"hello".length()` → `5` 定数）
  - length_cache 管理
- 行 231-282: substring メソッド（約51行）
  - NYASH_STR_CP フラグ（code point vs UTF-8 byte選択）
  - 文字列スライシング処理
- 行 284-323: lastIndexOf メソッド（約39行）
  - 文字列検索処理

**問題点**:
1. **ハードコードされたブリッジ**: `nyash.string.*` への直接 call
2. **複雑な最適化パス**: 複数の環境変数フラグで制御
3. **キャッシュ・状態管理**: length_cache, newbox_string_args の管理
4. **他の Box 処理と混在**: Console, Array, Map 処理と一緒

#### 目指す構造

**Phase 133 ConsoleLlvmBridge パターンを参考**:

```
BoxCall(StringBox, method)
    ↓
StringBoxBridge 箱化モジュール
    ↓
StringBox メソッド種別判定（length, substring, lastIndexOf）
    ↓
対応する LLVM runtime 関数群に lowering
    (@nyash.string.length, @nyash.string.substring, etc.)
```

**ファイル構成**:
```
src/llvm_py/instructions/
  ├── stringbox.py （新規、180行）
  │   ├── class StringBoxBridge
  │   ├── emit_stringbox_call()
  │   ├── _literal_fold_length()
  │   ├── _fast_strlen()
  │   └── _codepoint_mode()
  │
  └── boxcall.py （修正、-180行）
      └── StringBox判定 → stringbox.emit_stringbox_call() に委譲
```

---

### Task 2: 既存 boxcall.py の StringBox 部分の詳細棚卸し

**対象ファイル**: `src/llvm_py/instructions/boxcall.py`

**やること**:

1. **StringBox 処理の行数別カウント**:
   ```bash
   sed -n '130,323p' src/llvm_py/instructions/boxcall.py | wc -l
   ```
   - length/len: ~90行
   - substring: ~51行
   - lastIndexOf: ~39行
   - 合計: ~180行

2. **複雑な最適化パスの確認**:
   - NYASH_LLVM_FAST の使用箇所
   - NYASH_STR_CP の使用箇所
   - literal folding のロジック
   - キャッシュ管理（length_cache, newbox_string_args）

3. **内部依存関係の確認**:
   - StringBox メソッドが他のメソッドを呼んでいるか
   - 他の Box メソッドが StringBox を参照しているか

4. **既存テスト確認**:
   ```bash
   rg "StringBox|string.*length|substring" src/llvm_py/tests/ --type python
   ```

**結果記録**: phase134b ドキュメントの「実装計画」に記載

---

### Task 3: StringBoxBridge 箱化モジュールの実装

**実装方針**: Phase 133 ConsoleLlvmBridge パターンを継承

**ファイル**: `src/llvm_py/instructions/stringbox.py`（新規、~180行）

**責務**:
1. StringBox メソッド種別を判定（length, substring, lastIndexOf等）
2. 文字列引数を LLVM IR 形式に変換
3. 対応する LLVM runtime 関数呼び出しを生成

**実装パターン**:

```python
# src/llvm_py/instructions/stringbox.py

class StringBoxBridge:
    """
    StringBox メソッド処理を箱化した専用モジュール

    Phase 133 ConsoleLlvmBridge パターンを継承
    """

    STRINGBOX_METHODS = {
        "length": 410,        # TypeRegistry slot
        "len": 410,           # alias
        "substring": 411,
        "lastIndexOf": 412,
        # ... etc
    }

    @staticmethod
    def emit_stringbox_call(builder, module, method_name, receiver, args):
        """
        StringBox メソッド呼び出しを LLVM IR に lowering

        Args:
            builder: LLVM IRBuilder
            module: LLVM Module
            method_name: メソッド名（"length", "substring" etc.）
            receiver: StringBox インスタンス（i64 handle または i8* pointer）
            args: メソッド引数リスト

        Returns:
            LLVM Value（メソッド戻り値）
        """

        if method_name in ("length", "len"):
            return StringBoxBridge._emit_length(builder, module, receiver, args)

        elif method_name == "substring":
            return StringBoxBridge._emit_substring(builder, module, receiver, args)

        elif method_name == "lastIndexOf":
            return StringBoxBridge._emit_lastindexof(builder, module, receiver, args)

        else:
            raise ValueError(f"Unknown StringBox method: {method_name}")


    @staticmethod
    def _emit_length(builder, module, receiver, args):
        """
        StringBox.length() / StringBox.len() を LLVM IR に lowering

        Supports:
        - NYASH_LLVM_FAST: Fast path optimization
        - literal folding: "hello".length() → 5
        """

        # Phase 134-A より移動: boxcall.py:130-220 のロジック
        # - literal 判定
        # - length_cache 参照
        # - fast path vs normal path

        pass


    @staticmethod
    def _emit_substring(builder, module, receiver, args):
        """
        StringBox.substring(start, end) を LLVM IR に lowering

        Supports:
        - NYASH_STR_CP: Code point vs UTF-8 byte mode
        """

        # Phase 134-A より移動: boxcall.py:231-282 のロジック
        pass


    @staticmethod
    def _emit_lastindexof(builder, module, receiver, args):
        """
        StringBox.lastIndexOf(needle) を LLVM IR に lowering
        """

        # Phase 134-A より移動: boxcall.py:284-323 のロジック
        pass


    @staticmethod
    def _literal_fold_length(literal_str):
        """
        Literal StringBox の length を compile-time に計算

        例: "hello".length() → 5
        """

        # literal folding ロジック抽出
        pass


    @staticmethod
    def _fast_strlen(builder, module, receiver):
        """
        NYASH_LLVM_FAST パスでの高速 strlen 実装
        """

        # fast path 抽出
        pass


    @staticmethod
    def _codepoint_mode():
        """
        NYASH_STR_CP フラグから code point / UTF-8 byte モードを判定
        """

        # フラグ判定ロジック抽出
        pass
```

---

### Task 4: boxcall.py から StringBox 処理を削除・委譲に変更

**やること**:

1. **現在の分岐を確認**:
   ```python
   # boxcall.py 内のStringBox判定 (例)
   if box_id == CoreBoxId::StringBox or box_name == "StringBox":
       # 行 130-323: StringBox メソッド処理
       ...
   ```

2. **分岐を1行委譲に置き換え**:
   ```python
   if box_id == CoreBoxId::StringBox:
       # Phase 134-B: StringBoxBridge に委譲
       return stringbox.StringBoxBridge.emit_stringbox_call(
           builder, module, method_name, receiver, args
       )
   ```

3. **import 追加**:
   ```python
   from .stringbox import StringBoxBridge
   ```

4. **既存コードの削除**:
   - 行 130-323 の StringBox 処理ブロックを削除

---

### Task 5: 既存テスト実行・確認

**やること**:

1. **既存テスト実行**:
   ```bash
   cargo test --release 2>&1 | grep -E "StringBox|string.*length"
   ```

2. **stringbox.py のテスト**:
   - 新規 src/llvm_py/tests/test_stringbox.py を追加（オプション）
   - または既存テストで動作確認

3. **全テスト PASS 確認**:
   ```bash
   cargo test --release 2>&1 | tail -5
   ```

---

### Task 6: ドキュメント & CURRENT_TASK 更新

**やること**:

1. **phase134b_stringbox_bridge.md に追記**:
   ```markdown
   ## Phase 134-B 実装結果

   ### 修正ファイル
   - `src/llvm_py/instructions/stringbox.py`: StringBoxBridge 新規作成
   - `src/llvm_py/instructions/boxcall.py`: StringBox 処理を委譲に変更

   ### テスト結果
   - StringBox 関連テスト: ✅ PASS
   - 全テスト: ✅ PASS

   ### 成果
   - boxcall.py: 481 → 301行 (37%削減)
   - StringBox メソッド処理を stringbox.py に一元化
   - Phase 134-C CollectionBox 分離の準備完了
   ```

2. **CURRENT_TASK.md 更新**:
   ```markdown
   ### Phase 134-B: StringBox bridge 分離 ✅

   **完了内容**:
   - boxcall.py:130-282 の StringBox メソッド処理を分離
   - StringBoxBridge 箱化モジュール実装
   - 複雑な最適化パス (NYASH_LLVM_FAST, NYASH_STR_CP) を集約

   **修正箇所**:
   - src/llvm_py/instructions/stringbox.py (新規、180行)
   - src/llvm_py/instructions/boxcall.py (-180行)

   **テスト結果**: 全テスト PASS

   **成果**:
   - boxcall.py: 481 → 301行 (37%削減)
   - StringBox メソッド処理を stringbox.py に一元化
   - 次分割での拡張が容易に

   **次フェーズ**: Phase 134-C - CollectionBox bridge 分離
   ```

---

## ✅ 完成チェックリスト（Phase 134-B）

- [ ] boxcall.py の StringBox 処理の詳細棚卸し
- [ ] 複雑な最適化パス（NYASH_LLVM_FAST, NYASH_STR_CP）の確認
- [ ] StringBoxBridge 箱化モジュール実装
- [ ] StringBox メソッド別の lowering 関数実装
  - [ ] _emit_length()
  - [ ] _emit_substring()
  - [ ] _emit_lastindexof()
- [ ] 最適化ヘルパー実装
  - [ ] _literal_fold_length()
  - [ ] _fast_strlen()
  - [ ] _codepoint_mode()
- [ ] boxcall.py から StringBox 処理を削除
- [ ] StringBox判定 → stringbox.emit_stringbox_call() に委譲
- [ ] import 追加確認
- [ ] 既存テスト実行 & 全て PASS 確認
- [ ] phase134b_stringbox_bridge.md に実装結果追記
- [ ] CURRENT_TASK.md 更新
- [ ] git commit で記録

---

## 所要時間

**4〜5 時間程度**

- Task 1-2 (設計 & 棚卸し): 45分
- Task 3 (StringBoxBridge 実装): 1.5時間
- Task 4 (boxcall.py 修正): 45分
- Task 5-6 (テスト・ドキュメント): 1.5時間

---

## 次のステップ

**Phase 134-C: CollectionBox bridge 分離**
- boxcall.py:325-375 の Array/Map メソッド処理を collectionbox.py に分離
- Phase 133/134-A/134-B の箱化パターンを継承

---

## 進捗

- ✅ Phase 130-133: JoinIR → LLVM 第3章完全クローズ
- ✅ Phase 134-A: mir_call.py unified 設計完成
- ✅ Phase 134-B: StringBox bridge 分離（← **完了！**）
- 📋 Phase 134-C: CollectionBox bridge 分離（予定）
- 📋 Phase 135: LLVM フラグカタログ化（予定）

---

## Phase 134-B 実装結果 ✅

### 実装日時
2025-12-04 (Claude Code 実装)

### 修正ファイル
1. **新規作成**: `src/llvm_py/instructions/stringbox.py` (466行)
   - StringBoxBridge 箱化モジュール
   - length/len, substring, lastIndexOf メソッド lowering 実装
   - 最適化パス統合 (NYASH_LLVM_FAST, NYASH_STR_CP)
   - literal folding, length_cache 等の高度な最適化実装

2. **修正**: `src/llvm_py/instructions/boxcall.py` (481 → 299行)
   - StringBox メソッド処理 (lines 130-323, ~180行) を削除
   - 1行の委譲呼び出しに置き換え: `emit_stringbox_call()`
   - import 追加: `from instructions.stringbox import emit_stringbox_call`

### 実装内容詳細

#### StringBoxBridge モジュール構造
```python
class StringBoxBridge:
    STRINGBOX_METHODS = {
        "length": 410,
        "len": 410,  # Alias
        "substring": 411,
        "lastIndexOf": 412,
    }

    # Main dispatcher
    emit_stringbox_call()  # 全 StringBox メソッドの entry point

    # Method-specific handlers
    _emit_length()         # length/len 処理 (literal folding, cache, fast path)
    _emit_substring()      # substring 処理 (NYASH_STR_CP mode)
    _emit_lastindexof()    # lastIndexOf 処理

    # Helper functions
    _literal_fold_length() # Compile-time length 計算
    _fast_strlen()         # NYASH_LLVM_FAST 最適化パス
    _codepoint_mode()      # NYASH_STR_CP フラグ判定
    get_stringbox_method_info()  # Diagnostic helper
```

#### 最適化パス統合
1. **NYASH_LLVM_FAST パス**:
   - literal folding: `"hello".length()` → `5` (compile-time)
   - length_cache: 計算済み長さをキャッシュ
   - string_ptrs: ポインター直接アクセスで高速化
   - newbox_string_args: StringBox 生成時の引数追跡

2. **NYASH_STR_CP パス**:
   - Code point mode vs UTF-8 byte mode 切り替え
   - substring, length 計算でモード考慮

3. **Handle-based vs Pointer-based パス**:
   - i64 handle: nyash.string.*_hii 系関数
   - i8* pointer: nyash.string.*_sii 系関数

### テスト結果
- ✅ Python import テスト: PASS
  - `from instructions.stringbox import emit_stringbox_call` 成功
  - `from instructions.boxcall import lower_boxcall` 成功
- ✅ 既存テスト: 変更前と同じ結果 (47 failed は pre-existing, VM関連)
- ✅ LLVM backend: インポートエラーなし、構文エラーなし

### 成果
- **boxcall.py 削減**: 481 → 299行 (**37.8% 削減, 182行減**)
- **StringBox 処理の一元化**: 全メソッド処理が stringbox.py に集約
- **Phase 133 パターン継承**: ConsoleLlvmBridge と同じ設計
- **拡張性向上**: Phase 134-C CollectionBox 分離の準備完了

### 設計原則の踏襲
- ✅ Phase 133 ConsoleLlvmBridge パターンを完全継承
- ✅ 箱化モジュール化: 1 Box type = 1 dedicated module
- ✅ 最適化パスの統合: 環境変数フラグを module 内で管理
- ✅ Diagnostic helpers: get_stringbox_method_info() 実装

### 次のステップ
**Phase 134-C: CollectionBox bridge 分離**
- boxcall.py:143-193 の Array/Map メソッド処理を分離
- get, push, set, has メソッドを collectionbox.py に集約
- Phase 133/134-B パターンを継承
Status: Historical
