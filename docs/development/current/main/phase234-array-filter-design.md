# Phase 234: ArrayFilter / Pattern3 設計フェーズ

目的: ArrayExtBox.filter 系 3 テスト（Phase 49-4）の役割と期待値を整理し、Pattern3 if-PHI として正式サポートするか、当面は Fail-Fast を仕様として維持するかを設計レベルで決めるフェーズだよ（コード変更なし）。

---

## 1. ArrayExtBox.filter テスト仕様（Task 234-1）

対象: `src/tests/joinir/mainline_phase49.rs`

### 1.1 phase49_joinir_array_filter_smoke

- 目的:
  - dev フラグ `HAKO_JOINIR_ARRAY_FILTER_MAIN=1` で JoinIR Frontend mainline を経由したときに、ArrayExtBox.filter のコンパイルがクラッシュしないことを確認する。
- filter 実装の形:
  - `filter(arr, pred)` で `out: ArrayBox`, `i`, `n` をローカルとして持つ。
  - ループ本体:
    - `loop(i < n) {`
      - `v = arr.get(i)`
      - `if pred(v) { out.push(v) }`
      - `i = i + 1`
    - `return out`
- 期待しているのは:
  - 「Route B (JoinIR Frontend mainline) で compile が `Ok` を返すこと」（実行結果まではテストしていない）。

### 1.2 phase49_joinir_array_filter_fallback

- 目的:
  - dev フラグ OFF（`HAKO_JOINIR_ARRAY_FILTER_MAIN` 未設定）のときは、従来の（JoinIR mainline を通らない）経路でコンパイルが成功することを確認する。
- filter 実装の形:
  - ソースは smoke と同一（同じ filter 実装）。
- 期待しているのは:
  - dev フラグ OFF の Route A が「従来経路で compile `Ok`」になること。
  - JoinIR mainline を切った状態での後方互換性チェック。

### 1.3 phase49_joinir_array_filter_ab_comparison

- 目的:
  - ArrayExtBox.filter に対して Route A（legacy）と Route B（JoinIR Frontend）を A/B 比較し、両経路とも compile だけは成功しつつ、生成される MIR ブロック数をログで観察する。
- filter 実装の形:
  - ソースは前二つと同一。
- 期待しているのは:
  - Route A: `HAKO_JOINIR_ARRAY_FILTER_MAIN` OFF で compile `Ok`。
  - Route B: `HAKO_JOINIR_ARRAY_FILTER_MAIN=1` で compile `Ok`。
  - 実行結果ではなく、コンパイル成功と大まかなブロック構造（ブロック数）の健全性を確認するテスト。

---

## 2. 既存 Pattern / Box との対応付け（Task 234-2）

### 2.1 パターン構造の分類

- loop 形:
  - `loop(i < n) { ... i = i + 1 }` → ループ条件 `i < n` + カウンタ更新 `i = i + 1` は既存の Pattern2（break）/Pattern3（if-PHI）と同型。
- 条件分岐:
  - `if pred(v) { out.push(v) }`
    - `pred` はラムダ（関数）で、ループ内から呼び出される。
    - `out.push(v)` は ArrayBox への蓄積（Accumulation）だが、ループの「キャリア」としては `out` をどう扱うかが問題になる。
- 変数役割（ざっくり）:
  - ループインデックス: `i`（既存 CounterLike）
  - ループ境界: `n`（captured const 相当）
  - 出力配列: `out`（Array 型 accumulator）
  - 一時変数: `v`（body-local）

### 2.2 Pattern3 If-PHI との関係

- 既存の P3 if-sum:
  - `if (cond) { sum = sum + x }` という「条件付き Accumulation」を、Counter (`i`) + Accumulator (`sum`) の 2 キャリアで扱う。
  - LoopUpdateSummary + CarrierInfo + ExitLine で Counter/Accumulator を JoinIR の PHI / Exit に接続する。
- ArrayFilter の if:
  - `if pred(v) { out.push(v) }` は、`sum = sum + x` の配列版（filter 型 Accumulation）に近い。
  - ただし:
    - Accumulator の更新が「BoxCall（push）」であり、純粋な数値加算ではない。
    - `out` はループ外でも使われるオブジェクト（ループ終了後に return）で、スカラ accumulator と扱いが異なる。
- まとめ:
  - **構造的には P3 if-PHI と似ているが、更新が BoxCall ベースであり、現在の数値 if-sum パイプラインとは別系統の扱いが必要**。

### 2.3 既存箱でどこまで表現可能か

- PatternPipelineContext / CarrierInfo / ExitLine だけで無理なく扱えるか？
  - ループキャリアとして `out` を登録し、ExitLine で `out` を JoinIR の戻り値に接続すること自体は可能そう。
  - しかし、現行の P3 if-sum は「スカラ accumulator の PHI 合成」を前提にしており、配列の push を Accumulation として静的に扱うための契約（SideEffect・Alias・BoxCall 許容範囲）が決まっていない。
- ラムダ内のループかどうか？
  - 今回の ArrayExtBox.filter 実装は「ラムダの中にループがある」のではなく、「filter 本体にループがあり、ラムダ `pred` をコールする」形。
  - したがって「ラムダ内のループ」という意味での特別扱いは不要だが、「Box メソッドとして公開されるループ」という観点では selfhost/JsonParser とは少し違うレイヤ。

---

## 3. 方針決定（Task 234-3）

ここでは **Phase 234 時点での方針** を決めるだけで、コードはまだ触らないよ。

### Option A: P3 If-PHI の正式対象としてサポートする

- やることのイメージ（将来フェーズで）:
  - P3 if-sum パイプラインを一般化して「BoxCall ベースの Accumulation（push など）」も扱えるようにする。
  - CarrierInfo に「配列 accumulator」を追加し、ExitLine で `out` を JoinIR 経由で返り値に接続。
  - Pattern3 lowerer に「ArrayFilter パターン」用のサブ箱を足すか、既存 Accumulation 判定を拡張する。
- 必要な投資:
  - BoxCall の side-effect モデル（純粋な Accumulation として扱って安全か）を整理する。
  - JsonParser / selfhost よりも優先する理由が必要（現状は数値／JsonParser ラインが主戦場）。

### Option B: 当面は Fail-Fast を仕様として維持する（Phase 234 の結論）

- ここまでの整理から見えること:
  - 現在の FAIL は `[cf_loop/pattern3] Accumulator variable 'sum' not found in variable_map` という **PoC lowerer の制約に当たっての Fail-Fast** であり、JoinIR core（PHI/ValueId/ExitLine）の崩れではない。
  - JsonParser / selfhost / ExprLowerer ラインがまだ道半ばで、P3 if-sum も「数値 accumulator」を中心に設計されている。
  - ArrayFilter のような「BoxCall ベースの Accumulation」を安全に一般化するには、BoxCall/side-effect モデルと LoopPattern の交差設計が必要で、Phase 234 の守備範囲を超える。
- したがって Phase 234 の結論としては:
  - **ArrayExtBox.filter 系 3 テストは、当面「P3 if-PHI の外側にある PoC 領域」として Fail-Fast を仕様として維持する**。
  - 将来フェーズ（例: Phase 24x）で「Box ベース Accumulation / ArrayFilter パターン」を正式に設計するまでは、
    - 今のエラーメッセージと FAIL は「未対応領域を示すラベル」として残す。

---

## 4. FAIL の位置づけ更新（Task 234-4）

- Phase 232 では array_filter 3 件を P1/P2 候補として挙げていたが、Phase 234 の結論に従って:
  - **分類: P2（意図的 Fail-Fast / 将来拡張候補）**
  - ラベル: 「ArrayExtBox.filter / BoxCall ベース Accumulation パターン（将来の P3 拡張候補）」。
- `phase232-failing-tests-inventory.md` では:
  - loop_update_summary 系 4 件は Phase 233 で解消済み。
  - 残る array_filter 3 件は「Phase 234 の結論により、当面は Fail-Fast を仕様として維持する P2」として位置づける。

---

## 5. 次フェーズ候補（メモ）

- Phase 24x: ArrayFilter / BoxCall Accumulation の設計フェーズ
  - ねらい:
    - P3 if-PHI の「accumulator」概念を、数値 + 配列（BoxCall push）まで拡張できるか検討する。
  - 具体案:
    - CarrierInfo に「Box ベース accumulator」を追加するかどうか。
    - LoopPatternSpace に「filter パターン」を追加するか、既存 P3 のバリエーションとして扱うか。
    - JsonParser / selfhost で類似パターンが出てきたときに、ArrayFilter と同じ箱で扱えるか評価する。

このフェーズ（Phase 234）では、ここまでの設計メモだけに留めて、実装やテスト期待値の変更は行わないよ。***
Status: Active  
Scope: Array filter 設計（ExprLowerer ライン）
