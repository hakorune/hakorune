# JoinIR Loop Pattern Space (Phase 171 Ultra‑Think Memo)

このメモは「JoinIR のループパターン空間」を構造的に整理したものだよ。  
総当たりではなく、**直交する少数の軸**を組み合わせることで有限のパターンに収束することを確認する。

---

## 1. ループを構成する直交軸

ループはだいたい次の 6 軸の組み合わせで記述できる：

| 軸                | 選択肢                             | 説明                          |
|-------------------|------------------------------------|-------------------------------|
| A. 継続条件       | ① なし ② 単純 ③ 複合             | `loop(cond)` の `cond`        |
| B. 早期終了       | ① なし ② break ③ 条件付き break  | ループを「抜ける」経路         |
| C. スキップ       | ① なし ② continue ③ 条件付き cont | 次のイテレーションに「飛ぶ」  |
| D. PHI 分岐       | ① なし ② if‑PHI ③ match‑PHI      | 条件に応じて値が変わるパターン |
| E. 条件変数のスコープ | ① OuterLocal ② LoopBodyLocal     | 条件で参照される変数の定義位置 |
| F. キャリア更新   | ① 単一 ② 複数 ③ 条件付き           | ループ内での状態更新パターン（✅ Phase 176 で複数対応完了）   |

この 6 軸は、それぞれ 2〜3 通りしかないので、理論上は最大 3×3×3×3×2×3=486 通りの組み合わせになるが、  
実際に意味のあるパターンはずっと少ない（10〜20 程度）ことが分かった。

---

## 2. 現在 JoinIR が正規化済みのパターン

代表ループと対応する Pattern はだいたいこうなっている：

| Pattern        | 代表例                       | A継続   | B終了        | Cスキップ       | D‑PHI   | E変数      | F更新    |
|----------------|------------------------------|---------|--------------|-----------------|---------|------------|----------|
| P1: Minimal    | `loop_min_while.hako`        | 単純    | なし         | なし            | なし    | Outer      | 単一     |
| P2: Break      | `joinir_min_loop.hako`       | 単純    | 条件付きbreak | なし            | なし    | Outer      | 単一     |
| P3: If‑PHI     | `loop_if_phi.hako`           | 単純    | なし         | なし            | if‑PHI | Outer      | 条件付き |
| P4: Continue   | `loop_continue_pattern4`     | 単純    | なし         | 条件付きcont    | なし    | Outer      | 単一     |
| P5: Trim‑like* | `TrimTest.trim`（設計中）    | 単純    | 条件付きbreak | なし            | なし    | BodyLocal  | 複数     |

\*P5 は Phase 171 時点では「検出＋安全性判定」までだったが、Phase 172–176 で Trim / `_skip_whitespace` / `_parse_string` 最小版について JoinIR→MIR lower と複数キャリア更新まで実装済み。

ここまでで：

- OuterLocal 条件（関数パラメータや外側ローカル）を持つ基本的な while/break/continue/if‑PHI は JoinIR で正規化済み。
- LoopBodyLocal 条件（`local ch = ...; if ch == ' ' { break }`）は **LoopConditionScopeBox で Fail‑Fast** し、  
  Trim パターンだけを LoopBodyCarrierPromoter/TrimLoopHelper で特例として扱う設計にしている。

---

## 3. これから増やす候補パターン

実際の Nyash コードを見た上で、「出そうだな」と分かっているパターンをいくつか挙げておく。

### P6: break + continue 同時

```hako
loop (cond) {
    if x { continue }
    if y { break }
    ...
}
```

- A: 単純
- B: 条件付き break
- C: 条件付き continue
- E: OuterLocal 条件（が理想）

### P7: 複数キャリア + 条件付き更新

```hako
loop (i < n) {
    if arr[i] > max { max = arr[i] }
    if arr[i] < min { min = arr[i] }
    i = i + 1
}
```

- A: 単純
- B: なし
- C: なし
- D: なし or if‑PHI に相当
- F: 複数＋条件付き

### P8: ネストループ（外側ループへの break/continue）

```hako
loop (i < n) {
    loop (j < m) {
        if cond { break outer }  // labeled break
    }
}
```

※ Nyash 言語仕様で outer break をどう扱うか次第で、JoinIR 側も変わる。

### P9: match‑PHI

```hako
loop (i < n) {
    result = match state {
        "A" => compute_a()
        "B" => compute_b()
    }
}
```

これは if‑PHI を複数ケースに拡張した形。JoinIR の If/Select/PHI パスを再利用できるはず。

### P10: 無限ループ + 内部 break

```hako
loop (true) {
    if done { break }
}
```

構造としては P2 の特殊ケース（A: 継続条件なし＋B: break）として扱える。

### P11: 複合継続条件 + LoopBodyLocal

```hako
loop (i < n && is_valid) {
    local x = compute()
    is_valid = check(x)
}
```

これは LoopBodyLocal 状態を継続条件に折り込むパターン。  
BoolExprLowerer + LoopBodyCarrierPromoter の拡張で扱える可能性がある。

### P12: early return 内包

```hako
loop (i < n) {
    if error { return null }  // ループを抜けて関数終了
}
```

ループ exit と関数 exit が混ざるパターン。ExitLine とは別に「関数全体の戻り値ライン」とどう噛ませるかの設計が必要。

---

## 4. 収束性について

- 各軸が有限個の状態しか取らないこと、
- 多くの組み合わせが意味を持たない／禁止パターンであること（無限ループ、進まないループなど）

から、JoinIR で真面目に扱うべきループ形は **高々数十種類** に収束する。

実運用上の優先順としては：

1. P1–P4: 基本パターン（すでに実装済み）
2. P5: Trim / JsonParser で現実に必要な LoopBodyLocal 条件の一部（昇格可能なもの）  
3. P6, P7, P12: パーサ／集計／エラー処理で頻出
4. P8, P9, P11: 言語仕様や実アプリのニーズを見ながら段階的に

という順で Box を増やしていけば、「パターン爆発」にはならずに済む想定だよ。

---

## 5. Pattern × Box マトリクス（Phase 200 追加）

各 Pattern がどの Box を使うかを一覧にしたよ。
これで Claude / ChatGPT / 将来の自分が「このパターンはどの箱を使うか」をすぐ把握できる。

| Box / Module                         | P1 Minimal | P2 Break | P3 If‑PHI | P4 Continue | P5 Trim‑like* |
|--------------------------------------|------------|----------|-----------|-------------|---------------|
| **ConditionEnv**                     | ✅          | ✅        | ✅         | ✅           | ✅             |
| **BoolExprLowerer**                  | ✅          | ✅        | ✅         | ✅           | ✅             |
| **LoopConditionScopeBox**            | ✅          | ✅        | ✅         | ✅           | ✅             |
| **LoopScopeShapeBuilder**            | ✅          | ✅        | ✅         | ✅           | ✅             |
| **ConditionEnvBuilder**              | ✅          | ✅        | ✅         | ✅           | ✅             |
| **LoopHeaderPhiBuilder**             | ✅          | ✅        | ✅         | ✅           | ✅             |
| **ExitMeta / ExitMetaCollector**     | ✅          | ✅        | ✅         | ✅           | ✅             |
| **ExitLine / ExitLineReconnector**   | ✅          | ✅        | ✅         | ✅           | ✅             |
| **JoinInlineBoundary**               | ✅          | ✅        | ✅         | ✅           | ✅             |
| **BreakConditionAnalyzer**           | ❌          | ✅        | ❌         | ❌           | ✅             |
| **Pattern4CarrierAnalyzer**          | ❌          | ❌        | ❌         | ✅           | ❌             |
| **ContinueBranchNormalizer**         | ❌          | ❌        | ❌         | ✅           | ❌             |
| **LoopUpdateAnalyzer**               | ❌          | ❌        | ❌         | ✅           | ❌             |
| **LoopBodyCarrierPromoter**          | ❌          | ❌        | ❌         | ❌           | ✅             |
| **TrimLoopHelper**                   | ❌          | ❌        | ❌         | ❌           | ✅             |
| **TrimPatternValidator**             | ❌          | ❌        | ❌         | ❌           | ✅             |
| **TrimPatternLowerer**               | ❌          | ❌        | ❌         | ❌           | ✅             |
| **BlockAllocator** (merge)           | ✅          | ✅        | ✅         | ✅           | ✅             |
| **ValueCollector** (merge)           | ✅          | ✅        | ✅         | ✅           | ✅             |
| **InstructionRewriter** (merge)      | ✅          | ✅        | ✅         | ✅           | ✅             |
| **ExitPhiBuilder** (merge)           | ✅          | ✅        | ✅         | ✅           | ✅             |

**凡例**:
- ✅: 使用する（必須）
- ❌: 使用しない
- 🔮: 将来的に使用予定（設計段階）

**備考**:
- P5 (Trim‑like) は Phase 171‑172 で validation + JoinIR lowering まで完了。
- P6〜P12（break+continue 同時、複数キャリア、match‑PHI など）は将来パターンとして識別済み。
- すべての Pattern は **LoopConditionScopeBox** で条件変数のスコープを分類する（Fail‑Fast の要）。
- **TrimLoopHelper / TrimPatternValidator / TrimPatternLowerer** は P5 専用（他パターンには影響しない）。

---

## 6. 関連ドキュメント

- `joinir-architecture-overview.md`  
  JoinIR 全体の箱と契約。Loop/If/ExitLine/Boundary/条件式ラインの全体図。
- `phase33-16-design.md`, `PHASE_33_16_SUMMARY.md`  
  Loop header PHI / ExitLine / Boundary 再設計の詳細。
- `phase166-jsonparser-loop-recheck.md`  
  JsonParserBox / Trim 系ループのインベントリと、どの Pattern に入るかの観測ログ。
- `phase171-pattern5-loop-inventory.md`  
  Trim/JsonParser 向け Pattern5（LoopBodyLocal 条件）設計の進捗。
