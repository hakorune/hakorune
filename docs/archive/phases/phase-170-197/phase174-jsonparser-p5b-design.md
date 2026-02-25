# Phase 174-3: JsonParser `_parse_string` P5B 設計

**Date**: 2025-12-08
**Target**: `_parse_string` ループ（エスケープ処理付き文字列パース）
**Purpose**: Trim P5 パイプラインを複雑ループに拡張する設計

---

## ターゲットループ構造

### 完全版（Phase 175+ のターゲット）

```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)

    if ch == "\"" {  // 終端クォート
        // End of string
        local result = new MapBox()
        result.set("value", me._unescape_string(str))
        result.set("pos", p + 1)
        result.set("type", "string")
        return result
    }

    if ch == "\\" {  // エスケープ開始
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

### 最小化版（Phase 174-4 PoC）

```hako
// エスケープ処理・文字列バッファ・continue・return を除外
loop(pos < len) {
    local ch = s.substring(pos, pos+1)
    if ch == "\"" {
        break
    } else {
        pos = pos + 1
    }
}
```

---

## Pattern Space 分析

### 軸 A〜F での分類（最小化版）

| 軸 | 値 | 説明 |
|----|-----|------|
| **A. 継続条件** | 単純 | `pos < len` |
| **B. 早期終了** | 条件付き break | `if ch == "\"" { break }` |
| **C. スキップ** | なし | continue 使用なし |
| **D. PHI 分岐** | なし | if 分岐あるが PHI 不要 |
| **E. 条件変数のスコープ** | **LoopBodyLocal** | `ch` が条件に使用 |
| **F. キャリア更新** | 単純 | `pos` のみ更新 |

**Pattern 分類**: **P5**（LoopBodyLocal 条件）+ **Pattern2**（break 付き）

---

## Trim との差分

### ✅ 同じ部分（P5 パイプライン再利用可能）

1. **LoopBodyLocal 変数の使用**:
   - Trim: `local ch = s.substring(start, start+1)`
   - Parse String: `local ch = s.substring(pos, pos+1)`
   - → **完全一致**（変数名が異なるだけ）

2. **文字比較パターン**:
   - Trim: `if ch == " " || ch == "\t" || ch == "\n" || ch == "\r"`
   - Parse String: `if ch == "\""`
   - → **構造は同じ**（比較対象が異なるだけ）

3. **Pattern2 構造**:
   - Trim: `loop(start < end)` + `{ start = start + 1 } else { break }`
   - Parse String: `loop(pos < len)` + `{ pos = pos + 1 } else { break }`
   - → **完全同型**

4. **キャリア更新**:
   - Trim: `start = start + 1`（単一キャリア）
   - Parse String: `pos = pos + 1`（単一キャリア）
   - → **完全一致**

### ⚠️ 追加部分（完全版、Phase 175+ で必要）

1. **複数キャリア**:
   - `pos`: ループカウンタ
   - `str`: 文字列バッファ
   - **課題**: 既存の TrimLoopHelper は単一キャリア想定

2. **エスケープ処理**:
   - `if ch == "\\" { ... }` → 追加の文字読み込み
   - **課題**: LoopBodyLocal の多重使用（`ch` と `escaped`）

3. **continue**:
   - エスケープ処理後に continue
   - **課題**: Pattern4 対応が必要

4. **return による早期終了**:
   - 成功時の return
   - **課題**: ExitLine とループ外リターンの分離

---

## 箱の再利用 vs 追加

### 再利用可能な既存箱（Phase 174-4）

| 箱名 | 再利用可能性 | 理由 |
|-----|-----------|------|
| **LoopConditionScopeBox** | ✅ 100% | `ch` を LoopBodyLocal と分類可能 |
| **LoopBodyCarrierPromoter** | ✅ 100% | `ch` の昇格検出（Trim と同じ） |
| **TrimPatternInfo** | ✅ 100% | 基本構造は再利用可能 |
| **TrimLoopHelper** | ✅ 100% | 単一キャリア版で完全対応 |
| **CarrierInfo** | ✅ 100% | `pos` キャリア情報 |
| **ExitLine** | ✅ 100% | break による終了 |
| **Boundary** | ✅ 100% | ループ境界情報 |

**結論**: 最小化版では**既存の箱をそのまま使える**！

### 追加が必要な箱（Phase 175+ で検討）

1. **MultiCarrierTrimHelper**（将来）:
   - TrimLoopHelper の拡張版
   - 複数キャリア（`pos` + `str`）対応
   - **Phase 175 で設計・実装**

2. **EscapeSequenceHandler**（将来）:
   - エスケープ処理の追加ロジック
   - `ch == "\\"` 時の特殊処理
   - **Phase 176 で設計・実装**

3. **ContinuePatternHelper**（将来）:
   - Pattern4（continue 付き）対応
   - **Phase 176 で設計・実装**

---

## Phase 174 戦略

### ステップ1: 最小化版で試験（Phase 174-4）

**最小化方針**:
- ✅ エスケープ処理を**除外**
- ✅ 文字列バッファ（`str`）を**除外**
- ✅ continue を**除外**
- ✅ return を break に**置換**

**最小化版**:
```hako
// Phase 174-4 PoC
loop(pos < len) {
    local ch = s.substring(pos, pos+1)
    if ch == "\"" {
        break
    } else {
        pos = pos + 1
    }
}
```

**この構造の利点**:
- ✅ TrimLoopHelper がそのまま使える
- ✅ Pattern2 Trim 特例経路が使える
- ✅ 単一キャリア `pos` のみ
- ✅ 既存コードの変更**ゼロ**

**期待される動作**:
1. `LoopConditionScopeBox::analyze()`
   - `ch`: LoopBodyLocal（substring でループ内定義）
   - `pos`, `len`: OuterLocal（ループ外定義）

2. `LoopBodyCarrierPromoter::try_promote()`
   - `ch` を carrier `is_ch_match` に昇格
   - 昇格理由: break 条件に使用

3. `TrimLoopHelper::is_safe_trim()`
   - true を返す（Trim と完全同型）

4. Pattern2 lowerer
   - Trim 特例経路で JoinIR 生成
   - `[pattern2/trim] Safe Trim pattern detected`

### ステップ2: エスケープ処理の追加（Phase 175+）

最小化版が成功したら、段階的に追加：

1. **Phase 175**: 文字列バッファ追加（複数キャリア対応）
   ```hako
   loop(pos < len) {
       local ch = s.substring(pos, pos+1)
       if ch == "\"" {
           break
       } else {
           result = result + ch  // ← 追加
           pos = pos + 1
       }
   }
   ```

2. **Phase 176**: エスケープ処理追加（continue 対応）
   ```hako
   loop(pos < len) {
       local ch = s.substring(pos, pos+1)
       if ch == "\"" {
           break
       } else if ch == "\\" {
           // エスケープ処理
           pos = pos + 1
           result = result + s.substring(pos, pos+1)
           pos = pos + 1
           continue  // ← Pattern4 対応が必要
       } else {
           result = result + ch
           pos = pos + 1
       }
   }
   ```

3. **Phase 177**: return 処理追加（ExitLine 拡張）
   ```hako
   loop(pos < len) {
       local ch = s.substring(pos, pos+1)
       if ch == "\"" {
           return create_result(result, pos)  // ← ExitLine 拡張
       }
       // ... 以下同じ
   }
   ```

---

## 命名の汎用化（Phase 175+ で検討）

### 現在の命名（Trim 特化）

- `TrimLoopHelper` - Trim 専用のような命名
- `is_safe_trim()` - Trim 専用のような命名

### 汎用化案（Phase 175+ で検討）

**案1: CharComparison**
- `CharComparisonLoopHelper`
- `is_safe_char_comparison()`
- **利点**: 文字比較ループの汎用名
- **欠点**: やや長い

**案2: SingleCharBreak**
- `SingleCharBreakLoopHelper`
- `is_safe_single_char_break()`
- **利点**: 構造を正確に表現
- **欠点**: 長い、やや複雑

**案3: P5Pattern（軸E準拠）**
- `P5LoopHelper`
- `is_safe_p5_pattern()`
- **利点**: Pattern Space 軸E と一貫性
- **欠点**: P5 の意味が不明瞭

**Phase 174 での方針**: **命名変更なし**（既存コード保持）
- Phase 175+ で複数キャリア対応時に再検討
- 既存の TrimLoopHelper は Trim 専用として保持
- 新しい汎用版を別途作成する可能性

---

## テスト戦略

### Phase 174-4 テストケース

**ファイル**: `local_tests/test_jsonparser_parse_string_min.hako`

```hako
static box JsonParserStringTest {
    s: StringBox
    pos: IntegerBox
    len: IntegerBox

    parse_string_min() {
        me.s = "hello world\""  // 終端クォート付き
        me.pos = 0
        me.len = me.s.length()

        // 最小化版: エスケープ処理なし、終端クォート検出のみ
        loop(me.pos < me.len) {
            local ch = me.s.charAt(me.pos)
            if ch == "\"" {
                break
            } else {
                me.pos = me.pos + 1
            }
        }

        print("Found quote at position: ")
        print(me.pos)
    }

    main() {
        me.parse_string_min()
        return "OK"
    }
}
```

**期待される出力**:
```
[pattern2/check] 'ch': LoopBodyLocal ✅
[pattern2/promoter] promoted to carrier 'is_ch_match' ✅
[pattern2/trim] Safe Trim pattern detected ✅
Found quote at position: 11
```

**実行コマンド**:
```bash
NYASH_JOINIR_CORE=1 NYASH_LEGACY_LOOPBUILDER=0 \
  ./target/release/hakorune local_tests/test_jsonparser_parse_string_min.hako
```

---

## 結論

### Phase 174-4 では

- ✅ 既存の TrimLoopHelper をそのまま再利用
- ✅ 最小化版（`ch == "\""`のみ）で P5 パイプライン検証
- ✅ 既存コードの変更**ゼロ**
- ❌ エスケープ処理・複数キャリア・continue は Phase 175+ に延期

### Phase 175+ で検討

- **MultiCarrierTrimHelper** の設計
- **エスケープ処理**の統合（Pattern4 対応）
- **命名の汎用化**（Trim → CharComparison）
- **return 処理**の ExitLine 拡張

### 技術的価値

**Phase 174-4 で実証すべきこと**:
1. ✅ Trim パイプラインが文字比較対象が異なるだけで機能すること
2. ✅ `"\"`（終端クォート）という異なる文字でも昇格パターンが機能すること
3. ✅ TrimLoopHelper の汎用性（空白文字以外にも対応可能）

**成功すれば**:
- Trim P5 パイプラインが「Trim 専用」ではなく「文字比較ループ汎用」であることが証明される
- Phase 175+ での拡張（複数キャリア・continue 等）の基盤が確立される
Status: Historical
