# Phase 177: Carrier Evolution - min から Production へ

## 視覚的比較: ループ構造の進化

### Phase 174: min（1-carrier）
```
Input:  "hello world""
         ^start      ^target

loop(pos < len) {
    ch = s[pos]
    if ch == '"' → break ✓
    else         → pos++
}

Carriers: 1
- pos (ループ変数)

Pattern: 4 (Loop + If PHI)
Exit:    pos = 11
```

### Phase 175/176: min2（2-carrier）
```
Input:  "hello world""
         ^start      ^target

loop(pos < len) {
    ch = s[pos]
    if ch == '"' → break ✓
    else         → result += ch
                   pos++
}

Carriers: 2
- pos    (ループ変数)
- result (バッファ)

Pattern: 4 (Loop + If PHI)
Exit:    pos = 11, result = "hello world"
```

### Phase 177-A: Simple Case（2-carrier, Production-like）
```
Input:  "hello world""
         ^start      ^target

loop(p < len) {
    ch = s[p]
    if ch == '"' → break ✓
    else         → str += ch
                   p++
}

Carriers: 2
- p   (ループ変数)
- str (バッファ)

Pattern: 4 (Loop + If PHI)
Exit:    p = 11, str = "hello world"
```

### Phase 178: Escape Handling（2-carrier + continue）
```
Input:  "hello \"world\"""
         ^start ^escape  ^target

loop(p < len) {
    ch = s[p]
    if ch == '"'  → break ✓
    if ch == '\\' → str += ch
                    p++
                    str += s[p]
                    p++
                    continue ←← 新要素
    else          → str += ch
                    p++
}

Carriers: 2 (変わらず)
- p   (ループ変数)
- str (バッファ)

Pattern: 4? (continue 対応は未検証)
Exit:    p = 15, str = "hello \"world\""
```

### Phase 179: Full Production（2-carrier + early return）
```
Input:  "hello \"world\"""
         ^start ^escape  ^target

loop(p < len) {
    ch = s[p]
    if ch == '"'  → return MapBox {...} ✓ ←← early return
    if ch == '\\' → if p+1 >= len → return null ←← error
                    str += ch
                    p++
                    str += s[p]
                    p++
                    continue
    else          → str += ch
                    p++
}
return null  ←← ループ終端エラー

Carriers: 2 (変わらず)
- p   (ループ変数)
- str (バッファ)

Pattern: 4? (early return 対応は未検証)
Exit:    正常: MapBox, 異常: null
```

## Carrier 安定性分析

### 重要な発見
**Phase 174 → 179 を通じて Carrier 数は安定（1 → 2）**

| Phase | Carriers | 新要素 | P5昇格候補 |
|-------|----------|--------|-----------|
| 174 | 1 (pos) | - | なし |
| 175/176 | 2 (pos + result) | バッファ追加 | なし |
| 177-A | 2 (p + str) | - | なし |
| 178 | 2 (p + str) | continue | なし（確認中） |
| 179 | 2 (p + str) | early return | なし（確認中） |

### P5昇格候補の不在
**Trim と異なり、`is_ch_match` 相当は不要**

理由:
- Trim: `is_ch_match` が「次も空白か？」を決定（**次の判断に影響**）
- _parse_string: `ch == '"'` は「今終了か？」のみ（**次に影響しない**）

```
Trim の制御フロー:
  is_ch_match = (ch == ' ')
  if is_ch_match → pos++ → 次も is_ch_match を評価 ←← 連鎖

_parse_string の制御フロー:
  if ch == '"' → break → 終了
  else         → str += ch, p++ → 次は独立判断 ←← 連鎖なし
```

## JoinIR Pattern 対応予測

| Phase | Pattern 候補 | 理由 |
|-------|-------------|------|
| 177-A | Pattern4 | Loop + If PHI + break（実装済み） |
| 178 | Pattern4? | continue は Pattern4-with-continue（要実装確認） |
| 179 | Pattern5? | early return は新パターン候補（要設計） |

## まとめ

### 段階的検証戦略
1. **Phase 177-A**: min2 と同型 → P5 安定性確認
2. **Phase 178**: continue 追加 → JoinIR 拡張必要性評価
3. **Phase 179**: early return 追加 → Pattern5 設計判断

### Carrier 設計の教訓
- **最小構成で開始**: 1-carrier (Phase 174)
- **段階的拡張**: 2-carrier (Phase 175/176)
- **Production 適用**: 構造は変えず、制御フローのみ追加（Phase 177+）

→ **「Carrier 数を固定して制御フローを段階的に複雑化」が正解**
Status: Historical
