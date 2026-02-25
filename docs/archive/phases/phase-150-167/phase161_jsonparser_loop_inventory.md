# Phase 161-impl-3: JsonParserBox ループインベントリ

## 概要

JsonParserBox (`tools/hako_shared/json_parser.hako`) に含まれるループを全て分類し、
JoinIR Pattern1-4 とのマッピングを行う。

## ループ一覧（11個）

### 1. `_parse_number` (L121-133)
```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = digits.indexOf(ch)
    if digit_pos < 0 { break }
    num_str = num_str + ch
    p = p + 1
}
```
| 特徴 | 値 |
|------|-----|
| break | ✅ あり |
| continue | ❌ なし |
| if-else PHI | ❌ なし（単純if+break） |
| ループ変数 | `p`, `num_str` (2変数) |
| **パターン** | **Pattern2 (Break)** |

---

### 2. `_parse_string` (L150-178)
```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if ch == '"' {
        // return inside loop
        return result
    }
    if ch == "\\" {
        // escape handling
        str = str + ch; p = p + 1
        str = str + s.substring(p, p+1); p = p + 1
        continue
    }
    str = str + ch
    p = p + 1
}
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし（return使用） |
| continue | ✅ あり |
| if-else PHI | ❌ なし |
| ループ変数 | `p`, `str` (2変数) |
| return in loop | ✅ あり |
| **パターン** | **Pattern4 (Continue) + return** |

---

### 3. `_parse_array` (L203-231)
```hako
loop(p < s.length()) {
    local elem_result = me._parse_value(s, p)
    if elem_result == null { return null }
    arr.push(elem)
    p = elem_result.get("pos")
    if ch == "]" { return result }
    if ch == "," { p = p + 1; continue }
    return null
}
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし |
| continue | ✅ あり |
| if-else PHI | ❌ なし |
| ループ変数 | `p`, `arr` (2変数、arrはmutate) |
| return in loop | ✅ あり（複数箇所） |
| **パターン** | **Pattern4 (Continue) + multi-return** |

---

### 4. `_parse_object` (L256-304)
```hako
loop(p < s.length()) {
    // parse key
    if s.substring(p, p+1) != '"' { return null }
    local key_result = me._parse_string(s, p)
    // parse value
    local value_result = me._parse_value(s, p)
    obj.set(key, value)
    if ch == "}" { return result }
    if ch == "," { p = p + 1; continue }
    return null
}
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし |
| continue | ✅ あり |
| if-else PHI | ❌ なし |
| ループ変数 | `p`, `obj` (2変数、objはmutate) |
| return in loop | ✅ あり（複数箇所） |
| **パターン** | **Pattern4 (Continue) + multi-return** |

---

### 5. `_skip_whitespace` (L312-319)
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
| 特徴 | 値 |
|------|-----|
| break | ✅ あり |
| continue | ❌ なし |
| if-else PHI | ✅ あり（if-else分岐でpを更新） |
| ループ変数 | `p` (1変数) |
| **パターン** | **Pattern3 (If-Else PHI) + break** |

---

### 6. `_trim` (leading whitespace, L330-337)
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
| 特徴 | 値 |
|------|-----|
| break | ✅ あり |
| continue | ❌ なし |
| if-else PHI | ✅ あり |
| ループ変数 | `start` (1変数) |
| **パターン** | **Pattern3 (If-Else PHI) + break** |

---

### 7. `_trim` (trailing whitespace, L340-347)
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
| 特徴 | 値 |
|------|-----|
| break | ✅ あり |
| continue | ❌ なし |
| if-else PHI | ✅ あり |
| ループ変数 | `end` (1変数) |
| **パターン** | **Pattern3 (If-Else PHI) + break** |

---

### 8. `_match_literal` (L357-362)
```hako
loop(i < len) {
    if s.substring(pos + i, pos + i + 1) != literal.substring(i, i + 1) {
        return 0
    }
    i = i + 1
}
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし |
| continue | ❌ なし |
| if-else PHI | ❌ なし（単純if+return） |
| ループ変数 | `i` (1変数) |
| return in loop | ✅ あり |
| **パターン** | **Pattern1 (Simple) + return** |

---

### 9. `_unescape_string` (L373-431)
```hako
loop(i < s.length()) {
    local ch = s.substring(i, i+1)
    // flatten workaround for nested-if bug
    local is_escape = 0
    if ch == "\\" { is_escape = 1 }
    if process_escape == 1 {
        // multiple if-continue patterns
        if next == "n" { result = result + "\n"; i = i + 2; continue }
        if next == "t" { result = result + "\t"; i = i + 2; continue }
        // ... more cases
    }
    result = result + ch
    i = i + 1
}
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし |
| continue | ✅ あり（多数） |
| if-else PHI | ✅ あり（フラットworkaround済み） |
| ループ変数 | `i`, `result` (2変数) |
| **パターン** | **Pattern4 (Continue) + multi-PHI** |
| **複雑度** | **高**（フラット化workaround済み） |

---

### 10. `_atoi` (L453-460)
```hako
loop(i < n) {
    local ch = s.substring(i, i+1)
    if ch < "0" || ch > "9" { break }
    local pos = digits.indexOf(ch)
    if pos < 0 { break }
    v = v * 10 + pos
    i = i + 1
}
```
| 特徴 | 値 |
|------|-----|
| break | ✅ あり（複数箇所） |
| continue | ❌ なし |
| if-else PHI | ❌ なし |
| ループ変数 | `i`, `v` (2変数) |
| **パターン** | **Pattern2 (Break)** |

---

## パターン別サマリ

| パターン | ループ数 | 対象 |
|----------|----------|------|
| **Pattern1 (Simple)** | 1 | `_match_literal` (return in loop) |
| **Pattern2 (Break)** | 2 | `_parse_number`, `_atoi` |
| **Pattern3 (If-Else PHI) + break** | 3 | `_skip_whitespace`, `_trim`×2 |
| **Pattern4 (Continue)** | 5 | `_parse_string`, `_parse_array`, `_parse_object`, `_unescape_string`, + return variations |

## JoinIR 対応状況

### ✅ 現状で対応可能（Pattern1-2）
- `_match_literal` - Pattern1 + return
- `_parse_number` - Pattern2
- `_atoi` - Pattern2

### 🟡 拡張が必要（Pattern3）
- `_skip_whitespace` - **if-else + break の組み合わせ**
- `_trim` (leading) - **if-else + break の組み合わせ**
- `_trim` (trailing) - **if-else + break の組み合わせ**

### 🔴 大きな拡張が必要（Pattern4+）
- `_parse_string` - continue + return in loop
- `_parse_array` - continue + multi-return
- `_parse_object` - continue + multi-return
- `_unescape_string` - continue + multi-continue + if-else

## 推奨アクション

### 短期（Pattern3強化）
1. **Pattern3 + break 対応**: `_skip_whitespace`, `_trim`×2 を動かす
2. これで JsonParserBox の一部メソッドが動作可能に

### 中期（Pattern4基本）
1. **continue サポート**: `_parse_string`, `_match_literal` の return in loop 対応
2. **return in loop → break 変換**: 内部的に return を break + 値保存に変換

### 長期（Pattern4+完全対応）
1. **multi-return, multi-continue**: `_parse_array`, `_parse_object`
2. **複雑なフラット化パターン**: `_unescape_string`

## 備考

- `_unescape_string` は既に「MIR nested-if bug workaround」としてフラット化されている
- `_parse_value` 自体はループなし（再帰呼び出しのみ）
- ProgramJSONBox はループなし（getter のみ）

---

---

# BundleResolver ループインベントリ

## 対象ファイル

`lang/src/compiler/entry/bundle_resolver.hako`

## ループ一覧（8個）

### 1. Alias table parsing - outer loop (L25-45)
```hako
loop(i < table.length()) {
    local j = table.indexOf("|||", i)
    local seg = ""
    if j >= 0 { seg = table.substring(i, j) } else { seg = table.substring(i, table.length()) }
    // ... process seg ...
    if j < 0 { break }
    i = j + 3
}
```
| 特徴 | 値 |
|------|-----|
| break | ✅ あり |
| continue | ❌ なし |
| if-else PHI | ✅ あり（seg への代入） |
| ループ変数 | `i`, `seg` (2変数) |
| **パターン** | **Pattern3 (If-Else PHI) + break** |

---

### 2. Alias table parsing - inner loop for ':' search (L33)
```hako
loop(k < seg.length()) { if seg.substring(k,k+1) == ":" { pos = k break } k = k + 1 }
```
| 特徴 | 値 |
|------|-----|
| break | ✅ あり |
| continue | ❌ なし |
| if-else PHI | ❌ なし（単純if+break+代入） |
| ループ変数 | `k`, `pos` (2変数) |
| **パターン** | **Pattern2 (Break)** |

---

### 3. Require mods env alias check - outer loop (L52-71)
```hako
loop(i0 < rn0) {
    local need = "" + require_mods.get(i0)
    local present = 0
    // inner loop for bundle_names check
    // ... if present == 0 { ... }
    i0 = i0 + 1
}
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし |
| continue | ❌ なし |
| if-else PHI | ❌ なし |
| ループ変数 | `i0` (1変数) |
| **パターン** | **Pattern1 (Simple)** |

---

### 4. Require mods env alias check - inner loop (L57)
```hako
loop(j0 < bn0) { if ("" + bundle_names.get(j0)) == need { present = 1 break } j0 = j0 + 1 }
```
| 特徴 | 値 |
|------|-----|
| break | ✅ あり |
| continue | ❌ なし |
| if-else PHI | ❌ なし |
| ループ変数 | `j0`, `present` (2変数) |
| **パターン** | **Pattern2 (Break)** |

---

### 5. Duplicate names check - outer loop (L76-87)
```hako
loop(i < n) {
    local name_i = "" + bundle_names.get(i)
    local j = i + 1
    loop(j < n) { ... }
    i = i + 1
}
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし |
| continue | ❌ なし |
| if-else PHI | ❌ なし |
| ループ変数 | `i` (1変数) |
| **パターン** | **Pattern1 (Simple)** |

---

### 6. Duplicate names check - inner loop (L79-85)
```hako
loop(j < n) {
    if ("" + bundle_names.get(j)) == name_i {
        print("[bundle/duplicate] " + name_i)
        return null
    }
    j = j + 1
}
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし（return使用） |
| continue | ❌ なし |
| if-else PHI | ❌ なし |
| ループ変数 | `j` (1変数) |
| return in loop | ✅ あり |
| **パターン** | **Pattern1 (Simple) + return** |

---

### 7. Required modules check - outer loop (L92-101)
```hako
loop(idx < rn) {
    local need = "" + require_mods.get(idx)
    local found = 0
    // inner loop
    if found == 0 { print("[bundle/missing] " + need) return null }
    idx = idx + 1
}
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし |
| continue | ❌ なし |
| if-else PHI | ❌ なし |
| ループ変数 | `idx` (1変数) |
| return in loop | ✅ あり（条件付き） |
| **パターン** | **Pattern1 (Simple) + return** |

---

### 8. Required modules check - inner loop (L97)
```hako
loop(j < bn) { if ("" + bundle_names.get(j)) == need { found = 1 break } j = j + 1 }
```
| 特徴 | 値 |
|------|-----|
| break | ✅ あり |
| continue | ❌ なし |
| if-else PHI | ❌ なし |
| ループ変数 | `j`, `found` (2変数) |
| **パターン** | **Pattern2 (Break)** |

---

### 9. Merge bundle_srcs (L107)
```hako
loop(i < m) { merged = merged + bundle_srcs.get(i) + "\n" i = i + 1 }
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし |
| continue | ❌ なし |
| if-else PHI | ❌ なし |
| ループ変数 | `i`, `merged` (2変数) |
| **パターン** | **Pattern1 (Simple)** |

---

### 10. Merge bundle_mod_srcs (L111)
```hako
loop(i2 < m2) { merged = merged + bundle_mod_srcs.get(i2) + "\n" i2 = i2 + 1 }
```
| 特徴 | 値 |
|------|-----|
| break | ❌ なし |
| continue | ❌ なし |
| if-else PHI | ❌ なし |
| ループ変数 | `i2`, `merged` (2変数) |
| **パターン** | **Pattern1 (Simple)** |

---

## BundleResolver パターン別サマリ

| パターン | ループ数 | 対象 |
|----------|----------|------|
| **Pattern1 (Simple)** | 5 | L52, L76, L92, L107, L111 |
| **Pattern1 + return** | 2 | L79, L92 (条件付きreturn) |
| **Pattern2 (Break)** | 3 | L33, L57, L97 |
| **Pattern3 (If-Else PHI) + break** | 1 | L25 |

## BundleResolver JoinIR 対応状況

### ✅ 現状で対応可能（Pattern1-2）
- ほとんどのループが **Pattern1 または Pattern2** で対応可能
- 10個中9個が単純なパターン

### 🟡 拡張が必要（Pattern3）
- **L25-45 (Alias table outer loop)**: if-else で seg に代入するパターン
- これは Pattern3 の if-else PHI + break 対応が必要

---

## 統合サマリ

### JsonParserBox + BundleResolver 合計

| パターン | JsonParserBox | BundleResolver | 合計 |
|----------|---------------|----------------|------|
| Pattern1 | 1 | 5 | **6** |
| Pattern1 + return | 0 | 2 | **2** |
| Pattern2 | 2 | 3 | **5** |
| Pattern3 + break | 3 | 1 | **4** |
| Pattern4 | 5 | 0 | **5** |
| **合計** | **11** | **10** | **21** |

### 優先度順の対応方針

1. **Pattern1-2 強化** (11ループ対応可能)
   - return in loop の対応が必要（break変換）

2. **Pattern3 + break** (4ループ)
   - if-else PHI と break の組み合わせ
   - `_skip_whitespace`, `_trim`×2, Alias table outer loop

3. **Pattern4 (continue系)** (5ループ)
   - `_parse_string`, `_parse_array`, `_parse_object`, `_unescape_string`
   - JsonParserBox のコア機能

---

## 更新履歴

- 2025-12-06: Phase 161-impl-3 Task 161-3-1 完了 - ループインベントリ作成
- 2025-12-06: Phase 161-impl-3 Task 161-3-3 完了 - BundleResolver 棚卸し追加
Status: Historical
