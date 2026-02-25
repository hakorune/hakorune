# Phase 174-1: JsonParser 複雑ループ再観測

**Date**: 2025-12-08
**Purpose**: Phase 173 で未対応の複雑ループを再分析し、Phase 174 ターゲットを決定

---

## 未対応ループ分析

### 1. _parse_string (lines 150-178)

**構造**:
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
        // Escape sequence
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

**Pattern 分類**: Pattern4 候補（continue 付き）または Pattern2（基本構造）
**LoopBodyLocal**: `ch` が条件に使用
**複雑度**: ★★★☆☆（エスケープ処理あり、continue あり）

**Trim との差分**:
- ✅ **同じ**: LoopBodyLocal `ch` を break 条件に使用
- ✅ **同じ**: `local ch = s.substring(p, p+1)` パターン
- ✅ **同じ**: 単純な文字比較（`ch == "\""`）
- ⚠️ **追加**: エスケープ処理（`ch == "\\"` の次の文字を読む）
- ⚠️ **追加**: 文字列バッファへの追加処理（`str = str + ch`）
- ⚠️ **追加**: continue 使用（エスケープ処理後）
- ⚠️ **追加**: return による早期終了（成功時）

**最小化可能性**: ★★★★★（エスケープ処理・continue・return を除外すれば Trim と同型）

**最小化版**:
```hako
// エスケープ処理・文字列バッファ・continue を除外
loop(pos < len) {
    local ch = s.substring(pos, pos+1)
    if ch == "\"" {
        break  // return の代わりに break
    } else {
        pos = pos + 1
    }
}
```

この構造なら：
- ✅ TrimLoopHelper がそのまま使える
- ✅ Pattern2 Trim 特例経路が使える
- ✅ 単一キャリア `pos` のみ
- ✅ LoopBodyLocal `ch` の昇格パターンが Trim と同じ

**Phase 174 適性**: ★★★★★（最小化版で Trim に最も近い、第一候補）

---

### 2. _parse_array (lines 203-231)

**構造**:
```hako
loop(p < s.length()) {
    // Parse element
    local elem_result = me._parse_value(s, p)
    if elem_result == null { return null }

    local elem = elem_result.get("value")
    arr.push(elem)

    p = elem_result.get("pos")
    p = me._skip_whitespace(s, p)

    if p >= s.length() { return null }

    local ch = s.substring(p, p+1)
    if ch == "]" {
        // End of array - return result
        return result
    }

    if ch == "," {
        p = p + 1
        p = me._skip_whitespace(s, p)
        continue
    }

    return null
}
```

**Pattern 分類**: Pattern4 候補（continue 付き）
**LoopBodyLocal**: `ch` が条件に使用（ただし複数の他の処理も多い）
**複雑度**: ★★★★☆（複数条件、continue、ネスト、MethodCall 多数）

**Trim との差分**:
- ⚠️ **追加**: MethodCall 多数（`me._parse_value`, `me._skip_whitespace`）
- ⚠️ **追加**: MapBox/ArrayBox 操作（`elem_result.get()`, `arr.push()`）
- ⚠️ **追加**: 複数の return 文（成功・失敗パス）
- ⚠️ **追加**: continue 使用（区切り文字処理後）
- ✅ **同じ**: `local ch = s.substring(p, p+1)` パターン（限定的）
- ✅ **同じ**: 単純な文字比較（`ch == "]"`, `ch == ","`）

**最小化可能性**: ★☆☆☆☆（MethodCall と複雑な処理が本質的、最小化困難）

**Phase 174 適性**: ★★☆☆☆（複雑すぎる、Phase 175+ 推奨）

---

### 3. _parse_object (lines 256-304)

**構造**:
```hako
loop(p < s.length()) {
    p = me._skip_whitespace(s, p)

    // Parse key (must be string)
    if s.substring(p, p+1) != '"' { return null }
    local key_result = me._parse_string(s, p)
    if key_result == null { return null }

    local key = key_result.get("value")
    p = key_result.get("pos")
    p = me._skip_whitespace(s, p)

    // Expect colon
    if p >= s.length() { return null }
    if s.substring(p, p+1) != ":" { return null }
    p = p + 1

    p = me._skip_whitespace(s, p)

    // Parse value
    local value_result = me._parse_value(s, p)
    if value_result == null { return null }

    local value = value_result.get("value")
    obj.set(key, value)

    p = value_result.get("pos")
    p = me._skip_whitespace(s, p)

    if p >= s.length() { return null }

    local ch = s.substring(p, p+1)
    if ch == "}" {
        // End of object - return result
        return result
    }

    if ch == "," {
        p = p + 1
        continue
    }

    return null
}
```

**Pattern 分類**: Pattern4 候補（continue 付き）
**LoopBodyLocal**: `ch` が条件に使用（ただし他の処理も多い）
**複雑度**: ★★★★★（_parse_array と同程度以上、キー・バリューペア処理）

**Trim との差分**:
- ⚠️ **追加**: _parse_array と同様に MethodCall 多数
- ⚠️ **追加**: キー・バリューペアの複雑な処理
- ⚠️ **追加**: 複数の return 文
- ⚠️ **追加**: continue 使用
- ✅ **同じ**: `local ch = s.substring(p, p+1)` パターン（限定的）

**最小化可能性**: ★☆☆☆☆（_parse_array と同様に最小化困難）

**Phase 174 適性**: ★★☆☆☆（_parse_array と同程度の複雑さ、Phase 175+ 推奨）

---

### 4. _parse_number (lines 121-133)

**構造**:
```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = digits.indexOf(ch)

    // Exit condition: non-digit character found
    if digit_pos < 0 {
        break
    }

    // Continue parsing: digit found
    num_str = num_str + ch
    p = p + 1
}
```

**Pattern 分類**: Pattern2（break 付き、continue なし）
**LoopBodyLocal**: `ch`, `digit_pos` が条件に使用
**複雑度**: ★★☆☆☆（Trim に近いが、indexOf 使用）

**Trim との差分**:
- ✅ **同じ**: `local ch = s.substring(p, p+1)` パターン
- ✅ **同じ**: break での終了
- ✅ **同じ**: 単一キャリア `p` の更新
- ⚠️ **追加**: `digits.indexOf(ch)` による範囲チェック（OR chain の代わり）
- ⚠️ **追加**: 文字列バッファへの追加処理（`num_str = num_str + ch`）

**最小化可能性**: ★★★★☆（文字列バッファを除外すれば Trim に近い）

**最小化版**:
```hako
// 文字列バッファを除外
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = digits.indexOf(ch)
    if digit_pos < 0 {
        break
    } else {
        p = p + 1
    }
}
```

**Phase 174 適性**: ★★★☆☆（候補だが、_parse_string のほうが Trim に近い）

---

## Phase 174-2 選定結果

**選定ループ**: `_parse_string`（最小化版）

**選定理由**:

1. ✅ **Trim に最も近い構造**:
   - `local ch = s.substring(pos, pos+1)` パターン
   - `if ch == "\"" { break }` - 単純な文字比較
   - 単一キャリア `pos` のみ（最小化版）
   - LoopBodyLocal `ch` の使用パターンが Trim と同じ

2. ✅ **最小化可能性が高い**:
   - エスケープ処理を除外 → 基本的な文字比較ループ
   - 文字列バッファを除外 → 単一キャリア
   - continue を除外 → Pattern2 構造維持
   - return を break に置換 → Pattern2 Trim 特例経路利用可能

3. ✅ **実用性**:
   - JSON 文字列パースの核心部分
   - 成功すれば将来的にエスケープ処理等を追加可能

4. ✅ **既存実装で対応可能**:
   - `TrimLoopHelper::is_safe_trim()` がそのまま使える
   - `LoopBodyCarrierPromoter::try_promote()` で carrier 昇格可能
   - Pattern2 Trim 特例経路で JoinIR 生成可能

**構造的類似度**:
- Trim/_skip_whitespace: ★★★★★（100% 一致）
- _parse_string（最小化版）: ★★★★★（99% 一致、`"\""`の代わりに空白文字）
- _parse_number（最小化版）: ★★★★☆（95% 一致、indexOf 使用が差分）

**Phase 174 目標**:
- 基本的な文字比較部分を P5 パイプラインで処理（最小化版）
- エスケープ処理・文字列バッファ・continue は Phase 175+ に延期

**次点候補**: `_parse_number`（最小化版）
- Trim に近いが、indexOf 使用がやや複雑
- _parse_string 成功後の次のターゲット候補

**Phase 175+ 候補**: `_parse_array`, `_parse_object`
- 複雑すぎる、MethodCall 多数、最小化困難
- P5 パイプライン拡張後に検討

---

## Phase 174-3 準備: 最小化版の設計方針

**最小化方針**:
1. **エスケープ処理を除外**: `if ch == "\\"` ブロック全体を削除
2. **文字列バッファを除外**: `str = str + ch` を削除、`str` 変数を削除
3. **continue を除外**: エスケープ処理がないので continue も不要
4. **return を break に置換**: 成功時の return を break に変更

**最小化版の構造**:
```hako
// Phase 174-4 PoC版（最もシンプルな形）
loop(pos < len) {
    local ch = s.substring(pos, pos+1)
    if ch == "\"" {
        break
    } else {
        pos = pos + 1
    }
}
```

この構造は Trim/_skip_whitespace と **完全に同型**（文字比較の対象が異なるだけ）。

**期待される動作**:
1. `LoopConditionScopeBox::analyze()` - `ch` を LoopBodyLocal として検出
2. `LoopBodyCarrierPromoter::try_promote()` - carrier `is_ch_match` に昇格
3. `TrimLoopHelper::is_safe_trim()` - true を返す
4. Pattern2 lowerer - Trim 特例経路で JoinIR 生成

**Phase 174-4 で実証すべきこと**:
- ✅ Trim パイプラインが文字比較対象が異なるだけで機能すること
- ✅ `"\""`（終端クォート）という異なる文字でも昇格パターンが機能すること
- ✅ TrimLoopHelper の汎用性（空白文字以外にも対応可能）

---

## 結論

**Phase 174 戦略**:
- ✅ _parse_string の最小化版をターゲット
- ✅ 既存の TrimLoopHelper をそのまま再利用
- ✅ Pattern2 Trim 特例経路で JoinIR 生成
- ❌ エスケープ処理・複数キャリア・continue は Phase 175+ に延期

**Phase 175+ への道筋**:
1. Phase 174: _parse_string 最小化版（Trim と同型）
2. Phase 175: 文字列バッファ追加（複数キャリア対応）
3. Phase 176: エスケープ処理追加（continue 対応）
4. Phase 177: _parse_number 対応（indexOf パターン）
5. Phase 178+: _parse_array/_parse_object（複雑ループ）
Status: Historical
