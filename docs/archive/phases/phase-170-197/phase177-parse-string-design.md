# Phase 177: _parse_string 本体 JoinIR 適用設計

## 目的
Production `JsonParserBox._parse_string` メソッドに JoinIR P5 パイプラインを適用する。
Phase 174/175/176 で確立した「1-carrier → 2-carrier」検証の成果を、実際の JSON パーサーに展開する。

## 本番ループ構造（lines 144-181）

### 前提条件
- 開始位置 `pos` は `"` を指している（line 145 でチェック）
- ループ開始時 `p = pos + 1`（引用符の次の文字から開始）

### ループ本体（lines 150-178）
```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)

    if ch == '"' {
        // End of string (lines 153-160)
        local result = new MapBox()
        result.set("value", me._unescape_string(str))
        result.set("pos", p + 1)
        result.set("type", "string")
        return result  // ← Early return (通常の exit)
    }

    if ch == "\\" {
        // Escape sequence (lines 162-174)
        local has_next = 0
        if p + 1 < s.length() { has_next = 1 }

        if has_next == 0 { return null }  // ← Early return (エラー)

        str = str + ch
        p = p + 1
        str = str + s.substring(p, p+1)
        p = p + 1
        continue  // ← ループ継続
    }

    str = str + ch  // 通常文字の追加
    p = p + 1
}

return null  // ← ループ終端に到達（エラー）
```

### 制御フロー分岐
1. **終端クォート検出** (`ch == '"'`): 成功 return
2. **エスケープ検出** (`ch == "\\"`) + continue: 2文字消費してループ継続
3. **通常文字**: バッファ蓄積 + 位置進行
4. **ループ終端**: 引用符が閉じていない（エラー）

## Carriers 分析

| 変数 | 初期値 | 役割 | 更新式 | P5 昇格対象？ | 備考 |
|------|--------|------|--------|---------------|------|
| `p` | `pos + 1` | カウンタ（位置） | `p = p + 1` | **N（ループ変数）** | 条件式に使用 |
| `str` | `""` | バッファ（蓄積） | `str = str + ch` | **N（通常キャリア）** | エスケープ時2回更新 |
| `has_next` | - | 一時変数（フラグ） | - | - | ループ内のみ有効 |
| `is_escape` | - | （潜在的フラグ） | - | - | 明示的変数なし |

### 重要な発見
- **エスケープ処理は continue 経由**: `p` と `str` を 2 回更新してから continue
- **Early return が 2 箇所**: 成功 return (line 159) とエラー return (line 167)
- **通常キャリアのみ**: P5 昇格対象（`is_ch_match` 相当）は**不要**

## min ケースとの差分

### 共通点
| 項目 | min (Phase 174) | min2 (Phase 175) | 本番 (Phase 177) |
|------|-----------------|------------------|------------------|
| ループ変数 | `pos` | `pos` | `p` |
| バッファ | なし | `result` | `str` |
| 終端条件 | `ch == '"'` → break | `ch == '"'` → break | `ch == '"'` → return |
| 通常処理 | `pos++` | `result += ch; pos++` | `str += ch; p++` |

### 差分
| 項目 | min/min2 | 本番 |
|------|----------|------|
| 終端処理 | `break` のみ | Early `return` (MapBox 返却) |
| エスケープ処理 | なし | `continue` を使った複雑な分岐 |
| エラー処理 | なし | ループ内・外に `return null` |

## 方針決定: 段階的アプローチ

### Phase 177-A: Simple Case（今回）
**対象**: エスケープ**なし**・終端クォート検出のみ
- **ループ構造**: min2 と完全同型（`p` + `str` の 2-carrier）
- **終端処理**: `break` → 直後に MapBox 構築（return 代替）
- **目的**: P5 パイプラインが「2-carrier + break」で動作することを確認

```hako
// Simplified for Phase 177-A
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if ch == '"' {
        break  // P5: Pattern4 対応（Loop+If PHI merge）
    } else {
        str = str + ch
        p = p + 1
    }
}
// break 後: MapBox 構築
```

### Phase 178: Escape Handling（次回）
**追加要素**:
- `continue` 分岐（エスケープ処理）
- Early return（エラーハンドリング）
- P5 昇格キャリア候補の検討（`is_escape` フラグ？）

### Phase 179: Full Production（最終）
**統合**:
- `_unescape_string()` 呼び出し
- 完全なエラーハンドリング
- 本番同等の MapBox 返却

## Phase 177-A 実装計画

### Test Case: `test_jsonparser_parse_string_simple.hako`
```hako
// Phase 177-A: Production-like simple case
static box JsonParserStringTest3 {
    parse_string_simple() {
        local s = "hello world\""
        local p = 0  // pos + 1 相当（簡略化のため 0 から開始）
        local str = ""
        local len = s.length()

        // 2-carrier loop (min2 と同型)
        loop(p < len) {
            local ch = s.substring(p, p+1)
            if ch == "\"" {
                break
            } else {
                str = str + ch
                p = p + 1
            }
        }

        // Post-loop: 結果出力（MapBox 構築の代替）
        print("Parsed string: ")
        print(str)
        print(", final pos: ")
        print(p)
    }

    main() {
        me.parse_string_simple()
        return "OK"
    }
}
```

### 期待される MIR 構造
- **Pattern4 検出**: Loop + If PHI merge（Phase 170 実装済み）
- **2 carriers**: `p` (ループ変数) + `str` (バッファ)
- **Exit PHI**: ループ後の `p` と `str` が正しく伝播

### 検証項目
1. ✅ P5 パイプライン通過（JoinIR → MIR）
2. ✅ 2-carrier の正しい伝播（`p` と `str`）
3. ✅ `break` 後の変数値が正しく使用可能

## 成功基準
- [ ] `test_jsonparser_parse_string_simple.hako` が実行成功
- [ ] MIR ダンプで Pattern4 検出確認
- [ ] 出力: `Parsed string: hello world, final pos: 11`

## 次のステップ（Phase 178）
- `continue` 分岐の追加（エスケープ処理）
- P5 昇格キャリア候補の検討（必要性を再評価）
- Early return 対応（JoinIR での処理検討）

## まとめ
**Phase 177-A では、min2 と同型の Simple Case を Production 環境に適用する。**
エスケープ処理は Phase 178 以降に回し、まず「2-carrier + break」の動作確認を優先する。
Status: Historical
