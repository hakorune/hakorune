# Phase 181: JsonParser 残りループ設計調査

Status: Historical + Updated in Phase 42  
Note: Phase 181 時点の設計調査に基づくドキュメントだよ。最新の P2 分類と JoinIR/Normalized 状態は、このファイル内の「Phase 42 時点の P2 インベントリ」と `joinir-architecture-overview.md` を SSOT として見てね。

## 概要

JsonParser（`tools/hako_shared/json_parser.hako`）の全11ループを詳細に分析し、
どれが JoinIR パターン P1-P5 で対応可能か、どれがブロックされているかを調査した結果をまとめたもの。

**重要な発見**: 全ループが「loop条件が LoopParam/OuterLocal のみ」という制約を満たしており、
理論的に対応可能。ブロックされるループは存在しない。

## JsonParser ループ一覧（11個）

### 既に実装済み（3個）

| # | ループ | Pattern | 実装時期 | 状態 |
|----|--------|---------|---------|------|
| 1 | _skip_whitespace | P2 / P5 Trim | Phase 173 | ✅ 動作確認済み |
| 2 | _trim (leading) | P2 / P5 Trim | Phase 171 | ✅ 動作確認済み |
| 3 | _trim (trailing) | P2 / P5 Trim | Phase 171 | ✅ 動作確認済み |

### 今後実装可能（8個）

| # | ループ | Pattern | 優先度 | 難易度 | 実装時期 | 理由 |
|----|--------|---------|--------|--------|---------|------|
| 4 | _parse_number | P2 Break | 高 | 低 | Phase 182+ | indexOf パターン |
| 5 | _parse_string | P2/P4 | 高 | 中 | Phase 174-175 | string concat + キャリア複数化 |
| 6 | _atoi | P2 Break | 高 | 低 | Phase 182+ | 整数変換パターン |
| 7 | _match_literal | P1 Simple | 中 | 低 | Phase 182+ | 単純 iteration |
| 8 | _parse_array | P4 Continue | 中 | 高 | Phase 183+ | MethodCall 多数 |
| 9 | _parse_object | P4 Continue | 中 | 高 | Phase 183+ | MethodCall 多数 |
| 10 | _unescape_string | P4 Continue | 低 | 高 | Phase 184+ | 複数キャリア + flatten |
| 11 | _atof_loop | P2 Break | 低 | 低 | Phase 182+ | 浮動小数点変換 |

## 詳細分析

### _parse_number (P2 Break)

```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = digits.indexOf(ch)
    if digit_pos < 0 { break }
    num_str = num_str + ch
    p = p + 1
}
```

**特徴**:
- loop条件: `p < s.length()` (LoopParam のみ)
- キャリア: `p`, `num_str` (2個)
- break: あり（digit_pos < 0）
- 差分: indexOf パターン

**実行可能性**: ✅ Phase 182+ で実装可能（Pattern2 Break）

### _parse_string (P2/P4)

```hako
loop(p < s.length()) {
    local ch = s.substring(p, p+1)

    if ch == '"' {
        // return with result
        return result
    }

    if ch == "\\" {
        // escape処理
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

**特徴**:
- loop条件: `p < s.length()` (LoopParam のみ)
- キャリア: `p`, `str` (2個)
- PHI: if-else 分岐（3路）
- control_flow: return, continue
- string concat あり

**実行可能性**: ✅ Phase 174-175 で実装可能（string concat対応後）

### _skip_whitespace (P2/P5 既実装)

**状態**: ✅ Phase 173 で既に実装済み・動作確認済み

### _trim leading/trailing (P2/P5 既実装)

**状態**: ✅ Phase 171 で既に実装済み・動作確認済み

### _match_literal (P1 Simple)

```hako
loop(i < len) {
    if s.substring(pos + i, pos + i + 1) != literal.substring(i, i + 1) {
        return 0
    }
    i = i + 1
}
```

**特徴**:
- loop条件: `i < len` (LoopParam + OuterLocal)
- キャリア: `i` (1個)
- 副作用: なし（return のみ）

**実行可能性**: ✅ P1 simple, Phase 182+ で可能

### _parse_array (P4 Continue)

```hako
loop(p < s.length()) {
    local elem_result = me._parse_value(s, p)
    // ... validation ...

    arr.push(elem)

    if ch == "]" {
        // return with result
        return result
    }

    if ch == "," {
        p = p + 1
        continue
    }

    return null
}
```

**特徴**:
- loop条件: `p < s.length()` (LoopParam のみ)
- キャリア: `p`, `arr` (2個)
- MethodCall: _parse_value(), _skip_whitespace() など多数
- control_flow: return, continue

**実行可能性**: ✅ Phase 183+ で実装可能（MethodCall 複数対応後）

### _parse_object (P4 Continue)

**特徴**: _parse_array と同様だが複雑性が更に高い（key-value 処理）

**実行可能性**: ✅ Phase 183+ で実装可能

### _unescape_string (P4 Continue)

**特徴**:
- loop条件: `i < s.length()` (LoopParam のみ)
- キャリア: `result`, `i` (2個、`i` は条件付きで +1 または +2)
- PHI: 複数（escape種別で分岐）
- MethodCall: なし
- flatten workaround: あり

**実行可能性**: ✅ Phase 184+ で実装可能（複数キャリア完全対応後）

### _atoi (P2 Break)

```hako
loop(i < len) {
    local ch = s.substring(i, i + 1)
    local digit_pos = digits.indexOf(ch)
    if digit_pos < 0 { break }
    result = result * 10 + digit_pos
    i = i + 1
}
```

**特徴**:
- loop条件: `i < len` (LoopParam + OuterLocal)
- キャリア: `i`, `result` (2個)
- break: あり（digit_pos < 0）
- 差分: indexOf + 算術演算

**実行可能性**: ✅ Phase 182+ で実装可能（Pattern2 Break）

### _atof_loop (P2 Break)

```hako
loop(i < len) {
    local ch = s.substring(i, i + 1)
    local digit_pos = digits.indexOf(ch)
    if digit_pos < 0 { break }
    result = result * 10 + digit_pos
    i = i + 1
}
```

**特徴**:
- loop条件: `i < len` (LoopParam + OuterLocal)
- キャリア: `i`, `result` (2個)
- break: あり（digit_pos < 0）
- 差分: _atoi と同型（浮動小数点版）

**実行可能性**: ✅ Phase 182+ で実装可能（Pattern2 Break、_atoi の後継）

## Phase 42 時点の P2 インベントリ（JsonParser）

Phase 42 では、上の 11 ループについて「P2 としてどこまで JoinIR / Normalized で扱えているか」を棚卸しして、P2-Core / P2-Mid / P2-Heavy の 3 クラスに整理したよ。

### 1. 分類ポリシー

- **P2-Core**: 既に Normalized→MIR(direct) まで実装され、Phase 41 で canonical route（既定経路）として扱っているループ群。
- **P2-Mid**: JoinIR(Structured) には載っているが、Normalized はこれから本格対応する「次候補」のループ群。
- **P2-Heavy**: MethodCall 多数・複雑キャリアなどの理由で、Normalized 対応は Phase 43 以降に送っている重めのループ群。

### 2. JsonParser ループの現在ステータス（2025‑12 時点）

| # | ループ | Pattern | P2 クラス | JoinIR 状態 | Normalized 状態 | 備考 |
|----|--------|---------|-----------|-------------|-----------------|------|
| 1 | _skip_whitespace | P2 / P5 Trim | P2-Core | ✅ JoinIR OK（Phase 173, 197, 245 系） | ✅ Normalized→MIR(direct) / canonical（Phase 37, 41） | mini / real の両方をフィクスチャ化して dev / canonical で比較済み |
| 2 | _trim (leading) | P2 / P5 Trim | P2-Heavy | ✅ JoinIR OK（TrimLoopHelper 経由） | 未対応（P5 専用経路のまま） | Trim/P5 専用 lowerer で処理。Normalized 対応は将来検討 |
| 3 | _trim (trailing) | P2 / P5 Trim | P2-Heavy | ✅ JoinIR OK | 未対応 | leading と同様に Trim/P5 ラインで運用 |
| 4 | _parse_number | P2 Break | P2-Mid | ✅ JoinIR OK（Phase 245-EX） | ✅ dev Normalized→MIR(direct)（Phase 43-C、フィクスチャ `jsonparser_parse_number_real`。num_str は現状仕様のまま据え置き） | header/break/p 更新は JoinIR 経路に載せ済み。数値正規化は Phase 43 以降で拡張予定 |
| 5 | _parse_string | P2/P4 | P2-Heavy | 部分的に JoinIR 対応（Pattern3/4 拡張後に対象） | 未対応 | return/continue・複数キャリアを含むため heavy クラス扱い |
| 6 | _atoi | P2 Break | P2-Mid | ✅ JoinIR OK（Phase 246-EX） | ✅ dev Normalized→MIR(direct)（mini + 本体符号あり/なし、Phase 43-A） | P2-Core には `_atoi` mini fixture が入っている。本体は Phase 43 以降で canonical 化予定 |
| 7 | _match_literal | P1 Simple | （P1） | ✅ JoinIR OK | Normalized 対応は P1 ラインで別途管理 | P1 simple なので P2 クラス分類の対象外。Phase 197 で JoinIR E2E 検証済み |
| 8 | _parse_array | P4 Continue | P2-Heavy | ⚠️ Deferred（複数 MethodCall） | 未対応 | continue + MethodCall 多数のため heavy クラス。ConditionEnv/MethodCall 拡張後に扱う |
| 9 | _parse_object | P4 Continue | P2-Heavy | ⚠️ Deferred | 未対応 | _parse_array と同種の heavy ループ |
| 10 | _unescape_string | P4 Continue | P2-Heavy | ⚠️ Deferred | 未対応 | 複数キャリア + flatten を含む。Pattern3/4 拡張後の対象 |
| 11 | _atof_loop | P2 Break | P2-Mid | JoinIR 対応候補（_atoi と同型） | 未対応 | `_atoi` 後継として P2-Mid 候補に分類。Phase 43 以降で `_atoi` 本体と一緒に扱う想定 |

最新の canonical / dev Normalized 経路や Shape 判定ロジックの詳細は `joinir-architecture-overview.md`（Phase 35–41 セクション）を参照してね。

## Pattern × Box マトリクス（JsonParser全体）

```
                      | P1 Simple | P2 Break | P3 If-PHI | P4 Continue | P5 Trim-like
----------------------|-----------|----------|-----------|-------------|-------------
_parse_number         |           |    ✅    |           |             |
_parse_string         |           |    ✅    |           |     ✅      |
_parse_array          |           |    ✅    |           |     ✅      |
_parse_object         |           |    ✅    |           |     ✅      |
_skip_whitespace      |           |    ✅    |           |             |     ✅
_trim (leading)       |           |    ✅    |           |             |     ✅
_trim (trailing)      |           |    ✅    |           |             |     ✅
_match_literal        |    ✅     |          |           |             |
_unescape_string      |           |    ✅    |           |     ✅      |
_atoi                 |           |    ✅    |           |             |
_atof_loop            |           |    ✅    |           |             |
```

## ブロック分析

**重要な発見**: ブロックされるループは存在しない。

**理由**:
1. 全ループの loop条件が「LoopParam のみ」または「LoopParam + OuterLocal」
2. LoopBodyLocal が loop条件に含まれない
3. Phase 170-D の LoopConditionScopeBox がループ「条件式」のみを検査

**例外**: Trim パターンは LoopBodyLocal (`ch`) を「キャリア昇格」処理
- LoopBodyCarrierPromoter が `ch` を carrier に昇格
- TrimLoopHelper で安全性検証済み

## 実装ロードマップ

### Phase 182: 基本パターン（3-4個）

**目標**: 高優先度・低難易度のループを実装

- [ ] _parse_number (P2 Break, indexOf)
- [ ] _atoi (P2 Break)
- [ ] _match_literal (P1 Simple)
- [ ] _atof_loop (P2 Break, _atoi の後継)

### Phase 183: 中級パターン（2-3個）

**目標**: MethodCall を含むループに対応

- [ ] _parse_string 完全版 (P2/P4, string concat + escape)
- [ ] _parse_array (P4 Continue, MethodCall 複数)
- [ ] _parse_object (P4 Continue, MethodCall 複数)

### Phase 184+: 高級パターン（1個）

**目標**: 複雑なキャリア処理に対応

- [ ] _unescape_string (P4 Continue, 複数キャリア + flatten)

## 次フェーズへの提案

### Phase 182 実装スタート前にすること

1. Phase 164/165 の代表ケース（loop_min_while など）を確認
2. Phase 177 の PHI/LoopHeaderPhiInfo の multi-carrier 対応を確認
3. Phase 176 の CarrierUpdateLowerer の複数キャリア対応を確認

### Phase 182 実装の戦略

**段階的アプローチ**:
1. _parse_number だけで Pattern2 Break パターンを再確認
2. _atoi で P2 break パターン複数候補を検証
3. _match_literal で P1 simple パターンの拡張性を確認
4. _atof_loop で _atoi のパターン再利用性を確認

## 重要な設計的含意

### Loop条件の制約は本質的に安全

**発見**: JsonParser の全11ループが「LoopParam/OuterLocal のみ」という制約を満たす。

**理由**:
- JsonParser のループは文字列走査型が多い（`p < s.length()` など）
- LoopBodyLocal を条件に含むパターンは Trim など特殊なケースのみ
- Trim パターンは LoopBodyCarrierPromoter で昇格可能

**結論**: LoopConditionScopeBox の設計は正しかった。
　　　　　Trim 以外のほとんどのループはブロックされない。

### Pattern × Box の組み合わせ戦略

**現状**:
- P1 Simple: _match_literal（1個）
- P2 Break: _parse_number, _atoi, _atof_loop（3個）
- P4 Continue: _parse_array, _parse_object, _unescape_string（3個）
- P5 Trim: _skip_whitespace, _trim（既実装3個）

**戦略**:
1. P2 Break を優先（高優先度、低難易度）
2. P4 Continue は MethodCall 対応後に実装
3. P1 Simple は汎用性確認のために早めに試す

## 関連ドキュメント

- `loop_pattern_space.md` - JoinIR Loop Pattern Space 定義
- `phase174-jsonparser-loop-inventory-2.md` - 前回の詳細分析
- `phase173-jsonparser-loop-recheck.md` - Phase 173 検証結果
- `joinir-architecture-overview.md` - JoinIR アーキテクチャ全体図
- `phase171-c1-carrier-promoter-design.md` - LoopBodyCarrierPromoter 設計
- `phase180-trim-module-design.md` - TrimLoopLowerer モジュール化

---

**作成日**: 2025-12-08
**Phase**: 181（JsonParser 残りループの設計調査）
**分析対象**: tools/hako_shared/json_parser.hako（全11ループ）
**主な発見**: ブロックなし、理論的に全対応可能
**実装優先度**: P2 Break（3個） > P1 Simple（1個） > P4 Continue（3個） > P5 Trim（既実装3個）
