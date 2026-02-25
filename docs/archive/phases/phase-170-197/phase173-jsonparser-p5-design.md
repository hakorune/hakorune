# Phase 173-3: JsonParser P5 Design

**Date**: 2025-12-08
**Purpose**: Trim P5 パイプラインを JsonParser に展開する設計

---

## 基本方針

> **「Trim パターンをそのまま再利用」** - JsonParser の `_skip_whitespace` は Trim と完全同型なので、既存の TrimPatternInfo / TrimLoopHelper をそのまま使用できる。

### 重要な発見

Phase 173-1 の観測で判明した事実：

✅ **JsonParser の `_trim` メソッドは既に P5 対応済み！**

```
[pattern2/trim] Safe Trim pattern detected, implementing lowering
[pattern2/trim] Carrier: 'is_ch_match', original var: 'ch', whitespace chars: ["\r", "\n", "\t", " "]
[joinir/pattern2] Generated JoinIR for Loop with Break Pattern (Phase 170-B)
```

これは、Phase 171 で実装した P5 パイプラインが、以下の範囲で既に機能していることを意味する：

- ✅ `LoopConditionScopeBox::analyze()` - LoopBodyLocal 検出
- ✅ `LoopBodyCarrierPromoter::try_promote()` - Carrier 昇格
- ✅ `TrimLoopHelper::is_safe_trim()` - Trim パターン検証
- ✅ Pattern2 lowerer - Trim 特例経路で JoinIR 生成

---

## 必要な対応

### 1. 命名の汎用化（将来対応、今回は不要）

現在の `TrimLoopHelper` は「Trim」という名前だが、実際は「LoopBodyLocal 変数を bool carrier に昇格する」汎用的なパターン。将来的には以下の名前変更を検討：

- `TrimPatternInfo` → `CharComparisonPatternInfo`
- `TrimLoopHelper` → `CharComparisonLoopHelper`

**ただし Phase 173 では名前変更しない**（挙動不変を優先）。

**理由**:
1. **動作実績**: Phase 171 で Trim として実装し、既に JsonParser._trim で動作確認済み
2. **リスク回避**: 名前変更は破壊的変更のリスクあり
3. **段階的実装**: まず JsonParser で動作確認してから、汎用化を検討

---

### 2. JsonParser 用のテストケース追加

**目的**: `_skip_whitespace` が P5 パイプラインで動作することを確認

**ファイル**: `local_tests/test_jsonparser_skip_whitespace.hako`（NEW）

**内容**:
```hako
static box JsonParserTest {
    s: StringBox
    pos: IntegerBox
    len: IntegerBox

    skip_whitespace(input_str, start_pos) {
        me.s = input_str
        me.pos = start_pos
        me.len = me.s.length()

        local p = me.pos
        loop(p < me.len) {
            local ch = me.s.substring(p, p+1)
            if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
                p = p + 1
            } else {
                break
            }
        }

        return p
    }

    main() {
        print("=== JsonParser skip_whitespace Test ===")

        local result = me.skip_whitespace("  \t\n  hello", 0)
        print("Skipped to position: ")
        print(result)

        if result == 5 {
            print("PASS: Correctly skipped 5 whitespace characters")
            return 0
        } else {
            print("FAIL: Expected position 5, got ")
            print(result)
            return 1
        }
    }
}
```

**期待される動作**:
1. `loop(p < me.len)` - Pattern2 として検出
2. `local ch = me.s.substring(p, p+1)` - LoopBodyLocal として検出
3. `if ch == " " || ...` - Trim パターンとして認識
4. Carrier `is_ch_match` に昇格
5. JoinIR 生成成功

---

### 3. Pattern2 での Trim 特例判定拡張（必要なら）

現在の Pattern2 は「Trim」という名前で判定しているが、実際は構造判定なので、JsonParser の `_skip_whitespace` も同じ経路を通るはず。

**確認事項**:
- ✅ `TrimLoopHelper::is_safe_trim()` が JsonParser ループでも true になるか？
- ✅ `substring()` メソッドの検出が機能するか？
- ⚠️ `charAt()` メソッドの検出が必要か？（`_skip_whitespace` は `substring()` を使用）

**結論**: Phase 173-4 で実行時に確認

---

## データフロー（JsonParser 版）

```
JsonParser._skip_whitespace (AST)
      ↓
LoopConditionScopeBox::analyze()
      ↓ has_loop_body_local() == true (ch が LoopBodyLocal)
LoopBodyCarrierPromoter::try_promote()
      ↓ Promoted { trim_info }
TrimPatternInfo::to_carrier_info()
      ↓
CarrierInfo::merge_from()
      ↓
TrimLoopHelper::is_safe_trim() → true
      ↓
Pattern2 lowerer (Trim 特例経路)
      ↓
JoinIR 生成（bool carrier: is_ch_match）
```

**Trim との違い**: なし（完全同型）

---

## 実装方針

### Phase 173-4 で確認すべきこと

1. **LoopBodyLocal 検出**: JsonParser の `_skip_whitespace` が `LoopBodyCarrierPromoter` で検出されるか？
   - 期待: `local ch = s.substring(p, p+1)` が LoopBodyLocal として認識される

2. **Trim パターン認識**: `TrimLoopHelper::is_safe_trim()` が true になるか？
   - 期待: 空白文字判定パターンが検出される

3. **Trim 特例経路**: Pattern2 の Trim 特例経路を通るか？
   - 期待: `[pattern2/trim] Safe Trim pattern detected` が出力される

4. **JoinIR 生成**: JoinIR → MIR lowering が成功するか？
   - 期待: `[joinir/pattern2] Generated JoinIR for Loop with Break Pattern` が出力される

5. **実行成功**: 生成された MIR が正しく実行されるか？
   - 期待: `PASS: Correctly skipped 5 whitespace characters` が出力される

---

### 追加実装が必要な場合

**可能性のある拡張**:

1. **charAt() メソッドの検出**
   - 現在: `substring()` のみ検出
   - 拡張: `charAt()` も検出（JsonParser の一部のコードで使用）
   - 実装箇所: `src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs`
   - 方法: `is_substring_method_call()` を `is_char_extraction_method()` に拡張

2. **メソッド名の汎用化**
   - 現在: "substring" ハードコード
   - 拡張: "charAt", "substring" の両方に対応
   - リスク: 低（既存動作を壊さない追加）

**Phase 173-4 での判断基準**:
- ✅ `_skip_whitespace` が `substring()` のみ使用 → 追加実装不要
- ⚠️ 他の JsonParser ループが `charAt()` 使用 → Phase 174+ で対応

---

## 構造比較

### Trim (test_trim_main_pattern.hako)

```hako
loop(start < end) {
    local ch = s.substring(start, start+1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        start = start + 1
    } else {
        break
    }
}
```

### JsonParser._skip_whitespace

```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        p = p + 1
    } else {
        break
    }
}
```

### 構造的一致点

| 要素 | Trim | JsonParser | 一致 |
|------|------|------------|------|
| ループ条件 | `start < end` | `p < s.length()` | ✅ (比較演算) |
| LoopBodyLocal | `local ch = s.substring(...)` | `local ch = s.substring(...)` | ✅ (完全一致) |
| 空白判定 | `ch == " " \|\| ch == "\t" \|\| ...` | `ch == " " \|\| ch == "\t" \|\| ...` | ✅ (完全一致) |
| 進行処理 | `start = start + 1` | `p = p + 1` | ✅ (加算代入) |
| 終了処理 | `break` | `break` | ✅ (完全一致) |

**結論**: 100% 構造的に一致 → 既存の P5 パイプラインで完全対応可能

---

## リスク分析

### 低リスク要因

1. **既存実装の活用**:
   - `TrimLoopHelper` は既に JsonParser._trim で動作確認済み
   - `LoopBodyCarrierPromoter` は既に Trim で動作確認済み
   - Pattern2 Trim 特例経路は既に実装済み

2. **構造的一致**:
   - `_skip_whitespace` と Trim は完全同型
   - 新しいパターン認識ロジック不要

3. **独立性**:
   - `_skip_whitespace` は独立した helper method
   - 他のコードへの影響なし

### 潜在的リスク

1. **MethodCall 検出の差異**:
   - Trim: static box method 内のループ
   - JsonParser: helper method 内のループ
   - 影響: 低（AST レベルでは同じ構造）

2. **変数スコープの差異**:
   - Trim: `start`, `end` が method local
   - JsonParser: `p` が method local、`s` が parameter
   - 影響: 低（LoopConditionScopeBox は parameter も OuterLocal として扱う）

**結論**: 既存実装で対応可能、追加実装不要の見込み

---

## 次のステップ (Phase 173-4)

### 実装タスク

1. **テストケース作成**:
   - `local_tests/test_jsonparser_skip_whitespace.hako` 作成
   - Trim と同じ構造、JsonParser の文脈でテスト

2. **JoinIR モード実行**:
   ```bash
   NYASH_JOINIR_CORE=1 NYASH_LEGACY_LOOPBUILDER=0 \
     ./target/release/hakorune local_tests/test_jsonparser_skip_whitespace.hako
   ```

3. **期待される出力確認**:
   ```
   [pattern2/check] Analyzing condition scope: 3 variables
   [pattern2/check]   'ch': LoopBodyLocal
   [pattern2/promoter] LoopBodyLocal 'ch' promoted to carrier 'is_ch_match'
   [pattern2/trim] Safe Trim pattern detected, implementing lowering
   [joinir/pattern2] Generated JoinIR for Loop with Break Pattern
   PASS: Correctly skipped 5 whitespace characters
   ```

4. **charAt() 対応確認**:
   - 必要なら `LoopBodyCarrierPromoter` を拡張
   - Phase 173-4 の実行結果で判断

5. **テスト実行**:
   ```bash
   cargo test --release --lib loop_body_carrier_promoter
   cargo test --release --lib pattern2_with_break
   ```

---

## 成功基準

- ✅ `test_jsonparser_skip_whitespace.hako` が JoinIR で成功
- ✅ `[pattern2/trim] Safe Trim pattern detected` 出力
- ✅ `PASS: Correctly skipped 5 whitespace characters` 出力
- ✅ 既存の Trim テストが引き続き PASS
- ✅ ユニットテスト全て PASS

**ドキュメント成果物**:
- `phase173-jsonparser-p5-impl.md` - 実装結果レポート
- `CURRENT_TASK.md` - Phase 173 成果記録
Status: Historical
