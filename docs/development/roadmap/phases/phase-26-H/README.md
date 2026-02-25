# Phase 26-H — JoinIR / 関数正規化フェーズ（private 正本）

このフェーズ 26‑H の詳細な設計・タスク・ログは、まだ公開したくない内容を多く含むので、`docs/private` 側を正本として管理しているよ。

- 正本 README: `docs/private/roadmap2/phases/phase-26-H/README.md`
- 正本 TASKS: `docs/private/roadmap2/phases/phase-26-H/TASKS.md`

ここ（development 側）は公開用の入口だけ残しておいて、本文は private リポジトリ側でメンテナンスする方針だよ。

JoinIR / 関数正規化の全体方針や最終的な箱の形は、上記の private 側 README を参照してね。

- ループ = `step` を何回も呼ぶこと
- `break` = `k_exit(...)` を呼ぶこと
- `continue` = `step(...)` を呼ぶこと
- φ / LoopCarried 変数 = `step` の引数

ここでは「ループヘッダの φ で悩む」のではなく、「step の引数・k_exit の引数をどう定義するか」に責務が集中する。

### 2.2 パイプラインの再構成案

現在:

```text
AST  →  MIR / LoopForm v2  →  VM/LLVM
```

ここに 1 段挟む:

```text
AST  →  MIR / LoopForm v2  →  ★LoopFnIR(関数ループ層)  →  VM/LLVM
```

この LoopFnIR/JoinIR 層で:

- 各 LoopForm について「ループ関数(step) + 継続関数(k_exit)」を合成。
- ループの PHI / carrier / exit φ はすべて `step` / `k_exit` の引数として表現。
- 下流（VM / LLVM）は「関数呼び出し（および再帰のループ化や展開）」だけを見ればよい。

結果として:

- LoopForm v2 は「LoopFnIR を作る前段」に役割縮小。
- BodyLocal / Exit φ の詳細設計は「引数に何を持っていくか？」という関数インターフェース設計に吸収される。

---

## 4. このフェーズで実装する箱 / 概念ラベル

- 実装として増やす（26-H 内で手を動かすもの）
  - `join_ir.rs`: JoinIR 型（関数/ブロック/命令）＋ダンプ
  - LoopForm→JoinIR のミニ変換（1 ケース限定で OK）
  - 実験トグル（例: `NYASH_JOINIR_EXPERIMENT=1`）で JoinIR をダンプするフック

- 概念ラベル（27.x 以降に検討）
  - MirQuery のようなビュー層（reads/writes/succs を trait 化）
  - LoopFnLoweringBox / JoinIRBox の分割や最適化パス
  - VM/LLVM への統合

※ このフェーズでは「設計＋ミニ実験のみ」で、本線スモークは既存 MIR/LoopForm 経路を維持する。

---

## 3. 箱の数と最終形のイメージ

### 3.1 現在の PHI/Loop 周辺の箱（概略）

ざっくりカテゴリ分けすると:

- 構造:
  - `LoopFormBuilder`
  - `ControlForm`
- PHI 生成:
  - `HeaderPhiBuilder`
  - `ExitPhiBuilder`
  - `BodyLocalPhiBuilder`
  - `IfBodyLocalMergeBox`
  - `PhiBuilderBox`
  - `PhiInvariantsBox`
- 解析:
  - `LoopVarClassBox`
  - `LoopExitLivenessBox`
  - `LocalScopeInspectorBox`
  - if 解析系（IfAnalysisBox 的なもの）

関数正規化前提で進むと、最終的には:

- PHI を直接扱う箱は「LoopForm→LoopFnIR に変換する前段」に閉じ込める。
- LoopFnIR 導入後の本線では、次のような少数の箱が中心になる:
  - `LoopFnLoweringBox`（LoopForm → LoopFnIR / JoinIR）
  - `JoinIRBox`（JoinIR の保持・最適化）
  - 既存の VM/LLVM バックエンド（JoinIR からのコード生成側）

という構造に寄せられる見込み。

このフェーズ 26‑H では、「最終的にそこに寄せるための設計図」を書くところまでを目標とする。

---

## 4. 最終的に残したい「小さくて強い箱」セット

関数正規化（JoinIR / LoopFnIR）まで含めて、最終的に目指す箱の形をざっくりまとめておく。

1. フロント構造箱（構文 → 構造）
   - `ParserBox`  
     - 役割: ソース → AST 変換。制御構造はまだ構文レベル。
   - `ControlFormBox`（`ControlForm` / `LoopForm` の薄いラッパ）  
     - 役割: AST から If/Loop の「骨格」（preheader/header/body/latch/exit, then/else/merge 等）だけを抜き出す。  
     - ここでは φ/SSA は扱わない（形の SSOT）。

2. 関数正規化箱（LoopFnIR / JoinIR）
   - `LoopFnLoweringBox`（LoopForm → LoopFnIR/JoinIR）  
     - 役割: LoopForm/ControlForm を入力に、`step(i, k_exit)` / `join_after_if(x, k_exit)` のような関数＋継続の形に落とす。  
     - φ/Exit/Carrier/BodyLocal をすべて「関数の引数」に吸収する。
   - `JoinIRBox`  
     - 役割: JoinFunction/JoinInst を保持・ダンプし、将来的には JoinIR 上の最適化もここにまとめる。  
     - 制御は Call/Jump/Ret だけに集約される。

3. 解析箱（最小セット）
   - `JoinIrQueryBox`（MirQuery/JoinQuery 相当）  
     - 役割: read/write/succs を返すビュー層。ExitLiveness や GC root 判定の入力に使う。
   - `LoopVarClass/IfAnalysisBox`（統合して 1 箱でもよい）  
     - 役割: どの変数が loop-carried か（Carrier/Pinned/BodyLocal）・exit 後に使われるかを表で決める箱。  
     - JoinIR 観点では「関数引数として持つべき変数集合」を返す責務に縮退する。

4. 実行箱（バックエンド）
   - `VmBackendBox`（Rust VM / PyVM）  
     - 役割: JoinIR から VM 実行用コードに落とす。JoinIR の Call/Jump/Ret を関数呼び出しと分岐に写す。
   - `LlvmBackendBox`  
     - 役割: JoinIR から LLVM IR/AOT への変換。関数＋基本ブロックへの再投影を行う。

このセットを「最終形」として意識しつつ、26‑H ではまず JoinIR/LoopFnIR 周りの設計とミニ実装だけを進め、PHI/Loop 周辺の既存箱は徐々にこの形に寄せていく。

---

## 4. 26-H でやること（スコープ）

- JoinIR / LoopFnIR の設計ドキュメント作成
  - 命令セット（call / ret / jump / 継続）の最小定義。
  - if / loop / break / continue / return を JoinIR に落とす書き換え規則。
  - φ = 関数引数、merge = join 関数、loop = 再帰関数＋exit 継続、という対応表。
- 最小 1 ケースの手書き変換実験（MIR → JoinIR）
  - ループ＋break を含む簡単な関数を 1 例だけ JoinIR に落とし、形を確認。
- MirQueryBox 経由で必要な MIR ビュー API の確認
  - reads/writes/succs など、JoinIR 変換に必要な情報がすでに `MirQuery` で取れるかチェック。
- すべてトグル OFF で行い、本線（MIR/LoopForm ルート）のスモークには影響させない。

---

## 5. やらないこと（26-H では保留）

- 既存ルート（MIR/LoopForm/VM/LLVM）を JoinIR で置き換える。
- スモーク / CI のデフォルト経路変更。
- Loop/PHI 既存実装の削除（これは 27.x 以降の段階で検討）。

---

## 6. 実験計画（段階）

1. **設計シート**  
   - `docs/development/architecture/join-ir.md` に命令セット・変換規則・対応表を記述（φ=引数, merge=join, loop=再帰）。

2. **ミニ変換実験 1 ケース**  
   - 最小ループ（例: `loop(i < 3) { if i >= 2 { break } i = i + 1 } return i`）を MIR → JoinIR へ手書き変換し、テストでダンプを確認。VM/LLVM 実行までは行わない。

3. **トランスレータ骨格**  
   - `src/mir/join_ir.rs` などに型定義だけ追加（未配線、トグル OFF）。MirQueryBox（reads/writes/succs）で必要なビューが揃っているか確認。

4. **トグル付き実験**  
   - `NYASH_JOINIR_EXPERIMENT=1` などのトグルで最小ケースを JoinIR 変換・ダンプするルートを作る（デフォルト OFF でスモーク影響なし）。

---

## 7. 受け入れ基準（このフェーズ）

- docs に JoinIR / LoopFnIR の設計と変換規則が明記されている。
- 最小 1 ケースの JoinIR 変換がテストでダンプできる（join/step/k_exit の形になっている）。
- 本線スモーク（既存 MIR ルート）は影響なし（トグル OFF）。

---

## 8. 次フェーズへの橋渡し

- 26-H のスコープは「設計＋最小 JoinIR ダンプ＋ JoinIrMin 向け自動変換（トグル付き）」まで。
- 27.x では、次のような範囲を候補とする:
  - JoinIR 変換器を拡張し、FuncScanner / Stage‑B など本番寄りのループを 1〜2 個 JoinIR で通す（トグル付き）。
  - ExitLiveness や BodyLocal PHI の一部を LoopFnIR 側に吸収し、PHI/Loop 周辺の箱を徐々に減らす。
  - VM/LLVM 実行経路に JoinIR を統合するのは 27.x 以降を想定し、当面は「設計＋ミニ実験」に留める。
