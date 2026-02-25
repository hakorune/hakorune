# Phase 173-1: JsonParser ループ再観測

**Date**: 2025-12-08
**Purpose**: Trim P5 パイプラインを JsonParser に展開するための予備調査

---

## 観測対象ループ

### 1. _trim - Leading Whitespace

**File**: `tools/hako_shared/json_parser.hako`
**Lines**: 330-337

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

**Pattern 分類**: Pattern2 (break付き)
**LoopBodyLocal**: `ch` (substring定義、break条件で使用)
**Trim 類似度**: ★★★★★ (完全同型 - 既に実装済み)

**JoinIR 実行結果**: ✅ SUCCESS
```
[pattern2/check] Analyzing condition scope: 3 variables
[pattern2/check]   'ch': LoopBodyLocal
[pattern2/promoter] LoopBodyLocal 'ch' promoted to carrier 'is_ch_match'
[pattern2/trim] Safe Trim pattern detected, implementing lowering
[joinir/pattern2] Generated JoinIR for Loop with Break Pattern (Phase 170-B)
```

**状態**: ✅ Phase 171 で既に P5 パイプライン実装済み

---

### 2. _trim - Trailing Whitespace

**File**: `tools/hako_shared/json_parser.hako`
**Lines**: 340-347

```hako
loop(end > start) {
    local ch = s.substring(end-1, end)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        end = end - 1
    } else {
        break
    }
}
```

**Pattern 分類**: Pattern2 (break付き)
**LoopBodyLocal**: `ch` (substring定義、break条件で使用)
**Trim 類似度**: ★★★★★ (完全同型 - 既に実装済み)

**JoinIR 実行結果**: ✅ SUCCESS
```
[pattern2/check] Analyzing condition scope: 3 variables
[pattern2/check]   'ch': LoopBodyLocal
[pattern2/promoter] LoopBodyLocal 'ch' promoted to carrier 'is_ch_match'
[pattern2/trim] Safe Trim pattern detected, implementing lowering
[joinir/pattern2] Generated JoinIR for Loop with Break Pattern (Phase 170-B)
```

**状態**: ✅ Phase 171 で既に P5 パイプライン実装済み

---

### 3. _skip_whitespace

**File**: `tools/hako_shared/json_parser.hako`
**Lines**: 310-321

```hako
_skip_whitespace(s, pos) {
    local p = pos
    loop(p < s.length()) {
        local ch = s.substring(p, p+1)
        if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
            p = p + 1
        } else {
            break
        }
    }
    return p
}
```

**Pattern 分類**: Pattern2 (break付き)
**LoopBodyLocal**: `ch` (substring定義、break条件で使用)
**Trim 類似度**: ★★★★★ (Trim の leading whitespace と完全同型)

**構造的特徴**:
- `loop(p < s.length())` - 長さチェック（Trim の `start < end` と同等）
- `local ch = s.substring(p, p+1)` - 1文字抽出（完全一致）
- `if ch == " " || ch == "\t" || ch == "\n" || ch == "\r"` - 空白判定（完全一致）
- `p = p + 1` vs `break` - 進行 vs 脱出（完全一致）

**JoinIR 実行可能性**: ✅ 既存の Trim パイプラインで即座に対応可能
- `TrimLoopHelper::is_safe_trim()` が true を返すはず
- `LoopBodyCarrierPromoter::try_promote()` で carrier 昇格成功するはず

**Phase 173-2 での選定**: ★★★★★ 第一候補（Trim と完全同型）

---

### 4. _parse_string

**File**: `tools/hako_shared/json_parser.hako`
**Lines**: 150-178

```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)

    if ch == '"' {
        // End of string
        local result = new MapBox()
        result.set("value", me._unescape_string(str))
        result.set("pos", p + 1)
        result.set("type", "string")
        return result
    }

    if ch == "\\" {
        // Escape sequence (workaround: flatten to avoid MIR nested-if bug)
        local has_next = 0
        if p + 1 < s.length() { has_next = 1 }

        if has_next == 0 { return null }

        str = str + ch
        p = p + 1
        str = str + s.substring(p, p+1)
        p = p + 1
        continue
    }

    str = str + ch
    p = p + 1
}
```

**Pattern 分類**: Pattern4 候補（continue付き）
**LoopBodyLocal**: `ch` (substring定義、複数条件で使用)
**Trim 類似度**: ★★☆☆☆ (構造が複雑)

**構造的差異**:
- ✅ `local ch = s.substring(p, p+1)` - Trim と同じ
- ❌ 複数の条件分岐（`ch == '"'`, `ch == "\\"`）
- ❌ return 文での早期終了
- ❌ continue での反復継続
- ❌ 中間状態の蓄積（`str = str + ch`）

**LoopBodyLocal 使用**: `ch` を複数箇所で使用
**Phase 173 での対応**: ⚠️ 除外（複雑すぎる、Phase 174+ で検討）

---

### 5. _parse_array

**File**: `tools/hako_shared/json_parser.hako`
**Lines**: 189-230+

```hako
loop(p < s.length()) {
    // Parse value
    local val_result = me._parse_value(s, p)
    if val_result == null { return null }

    local val = val_result.get("value")
    arr.push(val)
    p = val_result.get("pos")

    p = me._skip_whitespace(s, p)

    if p < s.length() {
        local next_ch = s.substring(p, p+1)
        if next_ch == "," {
            p = p + 1
            p = me._skip_whitespace(s, p)
            continue
        }
        if next_ch == "]" {
            // End of array
            p = p + 1
            local result = new MapBox()
            result.set("value", arr)
            result.set("pos", p)
            result.set("type", "array")
            return result
        }
    }

    return null
}
```

**Pattern 分類**: Pattern4 候補（continue付き）
**LoopBodyLocal**: `next_ch` (substring定義、条件で使用)
**Trim 類似度**: ★☆☆☆☆ (構造が大きく異なる)

**構造的差異**:
- ❌ MethodCall 多数（`me._parse_value`, `me._skip_whitespace`）
- ❌ 複数の return 文（成功・失敗パス）
- ❌ 配列操作（`arr.push(val)`）
- ❌ MapBox からの値取得
- ✅ `local next_ch = s.substring(p, p+1)` - Trim と同じパターン
- ✅ `if next_ch == ","` - 単純な等価比較

**LoopBodyLocal 使用**: `next_ch` のみ（限定的）
**Phase 173 での対応**: ⚠️ 除外（構造が複雑、Phase 175+ で検討）

---

### 6. _unescape_string

**File**: `tools/hako_shared/json_parser.hako`
**Lines**: 373-410+

```hako
loop(i < s.length()) {
    local ch = s.substring(i, i+1)

    // Workaround: flatten to avoid MIR nested-if bug
    local is_escape = 0
    local has_next = 0

    if ch == "\\" { is_escape = 1 }
    if i + 1 < s.length() { has_next = 1 }

    local process_escape = 0
    if is_escape == 1 {
        if has_next == 1 {
            process_escape = 1
        }
    }

    if process_escape == 1 {
        // ... complex escape handling
    }

    result = result + ch
    i = i + 1
}
```

**Pattern 分類**: Pattern1 候補（break/continue なし）
**LoopBodyLocal**: `ch`, `is_escape`, `has_next`, `process_escape`
**Trim 類似度**: ☆☆☆☆☆ (全く異なる)

**構造的差異**:
- ❌ break/continue なし（自然終了のみ）
- ❌ 複数の LoopBodyLocal 変数
- ❌ ネストした条件分岐
- ❌ エスケープシーケンス処理

**Phase 173 での対応**: ❌ 除外（Pattern1、LoopBodyLocal の問題なし）

**JoinIR 実行結果**: ❌ FAILED
```
[ERROR] ❌ MIR compilation error: [joinir/freeze] Loop lowering failed:
JoinIR does not support this pattern, and LoopBuilder has been removed.
```

**失敗理由**: Pattern1 でも JoinIR 未対応の構造あり（Phase 173 対象外）

---

## 選定候補

### Phase 173-2 で選ぶべきループ: `_skip_whitespace`

**選定理由**:

1. ✅ **Trim と完全同型**: 既存の P5 パイプラインがそのまま使える
   - `local ch = s.substring(p, p+1)` パターン
   - `if ch == " " || ch == "\t" || ch == "\n" || ch == "\r"` 空白判定
   - `break` での終了

2. ✅ **独立した関数**: 他の複雑なロジックに依存しない

3. ✅ **実用的**: JsonParser で頻繁に使われる（7箇所で呼び出し）

4. ✅ **テスト容易**: 単純な入出力でテスト可能

5. ✅ **既存実装の活用**:
   - `TrimLoopHelper::is_safe_trim()` - そのまま使える
   - `LoopBodyCarrierPromoter::try_promote()` - そのまま使える
   - Pattern2 lowerer - Trim 特例経路がそのまま機能する

**次点候補**: なし（他のループは複雑すぎるか、LoopBodyLocal 問題がない）

---

## 重要な発見

### ✅ JsonParser の _trim は既に P5 対応済み！

Phase 171 で実装した Trim パイプラインが、JsonParser の `_trim` メソッドで **既に完全動作** していることを確認：

```
[pattern2/trim] Safe Trim pattern detected, implementing lowering
[joinir/pattern2] Generated JoinIR for Loop with Break Pattern (Phase 170-B)
```

これは、Phase 173 の目的が達成済みであることを意味する。

### 🎯 Phase 173 の真の価値

**既存実装の検証**: Trim パイプラインが JsonParser でも機能することを実証
- ✅ `TrimLoopHelper` の汎用性確認
- ✅ Pattern2 Trim 特例の堅牢性確認
- ✅ LoopBodyLocal 昇格の実用性確認

**次のステップ**: `_skip_whitespace` で独立関数パターンの検証
- Trim は static box のメソッド内ループ
- _skip_whitespace は helper method 内ループ
- 両方で機能すれば、P5 パイプラインの汎用性が完全証明される

---

## 次のステップ (Phase 173-2)

### 選定ループ: `_skip_whitespace`

**選定基準確認**:
- ✅ LoopBodyLocal 変数を break 条件に使用
- ✅ substring でループ内定義
- ✅ OR chain での比較（空白文字判定）
- ✅ Pattern2 構造（break 付き、continue なし）

**期待される動作**:
1. `LoopConditionScopeBox::analyze()` - `ch` を LoopBodyLocal として検出
2. `LoopBodyCarrierPromoter::try_promote()` - carrier `is_ch_match` に昇格
3. `TrimLoopHelper::is_safe_trim()` - true を返す
4. Pattern2 lowerer - Trim 特例経路で JoinIR 生成

**Phase 173-4 で実装**: ミニテストケースでの動作確認

---

## Phase 173-2 選定結果

**選定ループ**: `_skip_whitespace` (line 310-321)

**選定理由**:

1. **Trim と完全同型**: 構造が完全に一致
   - `loop(p < s.length())` ← Trim の `loop(start < end)` と等価
   - `local ch = s.substring(p, p+1)` ← 完全一致
   - `if ch == " " || ch == "\t" || ch == "\n" || ch == "\r"` ← 完全一致
   - `{ p = p + 1 } else { break }` ← Trim の `start = start + 1` と同パターン

2. **独立関数**: helper method として独立（テスト容易）

3. **実用性**: JsonParser で 7箇所で使用される頻出パターン

4. **既存実装で対応可能**:
   - `TrimLoopHelper::is_safe_trim()` がそのまま使える
   - `LoopBodyCarrierPromoter::try_promote()` で carrier 昇格可能
   - Pattern2 Trim 特例経路で JoinIR 生成可能

5. **検証価値**:
   - ✅ `_trim` メソッド内ループで既に成功（static box method）
   - 🎯 `_skip_whitespace` で helper method パターンを検証
   - → 両方成功すれば P5 パイプラインの汎用性が完全証明される

**次点候補**: なし

**理由**:
- `_parse_string` - 複雑すぎる（return/continue 混在、Phase 174+）
- `_parse_array` - MethodCall 多数、構造複雑（Phase 175+）
- `_unescape_string` - Pattern1、LoopBodyLocal 問題なし（対象外）

**Phase 173-3 での設計方針**: 既存の Trim パイプラインをそのまま活用
Status: Historical
