Status: Active  
Date: 2025-12-22  
Scope: Phase 272 の設計相談（この Markdown だけで完結する要約 + 質問集）  
Audience: 外部相談（ChatGPT Pro 等）向け  

# Phase 272: Pattern 裾広がりを止める設計相談（Frag + Plan 収束）

## 相談の目的（結論）

`Frag + emit_frag()`（terminator SSOT）は導入できたが、上流が “Pattern 群” のままだと **loop 形が増えるたびに pattern 実装が裾広がり**になりやすい。  
Phase 272（Pattern6/7 を Frag へ段階移行）のタイミングで、**pattern を「Plan 抽出」に降格**し、lowering の本線を収束させたい。

この文書は、外部相談先に渡して「設計の方向性」「最小の収束案」「段階移行の手順」をレビューしてもらうためのもの。

---

## 現状（2025-12-22）

### 参照SSOT（コード/設計）

- 設計（Structured→CFG 合成SSOT候補）: `docs/development/current/main/design/edgecfg-fragments.md`
- Phase 272（実装チェックリスト）: `docs/development/current/main/phases/phase-272/README.md`
- loop pattern router（優先順位SSOT）: `src/mir/builder/control_flow/joinir/patterns/router.rs`
- Frag API（型/emit SSOT）: `src/mir/builder/control_flow/edgecfg/api/`
  - `emit_frag()`（terminator SSOT）: `src/mir/builder/control_flow/edgecfg/api/emit.rs`
- 参照実装（Frag 経路が既に動作）:
  - Pattern8: `src/mir/builder/control_flow/joinir/patterns/pattern8_scan_bool_predicate.rs`
  - Pattern8 emission: `src/mir/builder/emission/loop_predicate_scan.rs`

### Phase 269（完了）: Pattern8 が Frag 経路で動作中（参照実装）

Pattern8 は “pattern 層で block/value を構築” し、terminator は `Frag + emit_frag()` に集約している。

- block を明示割り当て（`next_block_id()`）
- PHI は `insert_phi_at_head_spanned()` で header 先頭へ（SSA を閉じる）
- wiring は emission 層で `Frag` を組み、`emit_frag()` で Branch/Jump/Return を落とす

### Phase 270（完了）: Pattern9 を bridge として追加

Pattern1（simple_while_minimal）が test-only stub のため、JoinIR-only minimal loop を通す橋として `Pattern9_AccumConstLoop` を追加。  
Phase 271（docs-only）で撤去条件を SSOT 化済み（`edgecfg-fragments.md` の Bridge patterns セクション）。

### Phase 272 P0（進行中）: Pattern6→Pattern7 の順で Frag 化

- Pattern6: `index_of` 形（scan with init）
  - fixture/smoke: `apps/tests/phase254_p0_index_of_min.hako` / `tools/smokes/v2/profiles/integration/apps/phase254_p0_index_of_vm.sh`
- Pattern7: `split` 形（tokenization with variable step + side effects）
  - fixture/smoke: `apps/tests/phase256_p0_split_min.hako` / `tools/smokes/v2/profiles/integration/apps/phase256_p0_split_vm.sh`

---

## 問題意識（相談ポイント）

### 1) Frag の入口が小さくても、pattern 群が増えれば裾広がりになる

Frag/emit は SSOT 化できたが、上流の loop 処理が “pattern番号の列挙” を中心に残ると:

- 新しい loop 形（reverse, dynamic needle, extra side effects, nested if 等）が出るたびに pattern が増える
- “どのパターンで通すか” が本質になり、Structured→CFG 合成則（ExitKind/Frag）が中心に戻らない
- 結果としてフローが読みにくくなり、設計が収束しにくい

### 2) compiler flow（責務分離）がまだ不安定

収束させたい本線は以下:

`AST(loop)` → **Plan（正規化された抽出結果）** → `Frag（wires/branches）` → `emit_frag()`（terminator SSOT）

pattern は “検出して Plan を返すだけ” に降格し、CFG の組み立て・配線の本体は Plan→Frag の共通 lowerer に集約したい。

---

## 現状の実装形（要約）

### EdgeCFG Frag（terminator SSOT）

- `Frag` は “entry + branches + wires (+ exits)” を持つ
- `emit_frag()` が以下を保証（Fail-Fast）:
  - 1 block = 1 terminator（wire/branch の衝突禁止、複数wire禁止）
  - `set_jump_with_edge_args` / `set_branch_with_edge_args` を使用し successors/preds を同期
  - Return は `target=None` を許可（意味を持たない）

### Pattern6/7（現状）: JoinIRConversionPipeline 依存

現時点では `JoinIRConversionPipeline` に依存しており、
JoinModule → MirModule → merge… という暗黙の変換で terminator が作られる。
これが “terminator SSOT” を弱め、pattern 増殖も誘発しやすい。

---

## 制約（ポリシー）

- by-name ハードコード禁止（Box名文字列一致で分岐など）
- 環境変数トグル増殖禁止（今回の相談では新設しない）
- Fail-Fast 原則（fallback は原則避け、`Ok(None)` の “不適用” と `Err` の “契約違反” を分ける）
- 大規模設計変更は避け、段階移行（P0/P1…）で可逆に進める

---

## 収束のための案（たたき台）

### 案A: “Pattern = Plan Extractor” へ降格（推奨）

pattern を増やすのではなく、**Plan の種類を少数に固定**し、pattern は Plan 抽出だけを担当する。

例（概念）:

- `LoopPlan::ScanEq`（Pattern6の本質）
  - `i`（loop var）, `s`（haystack）, `needle`
  - `step`（P0は 1 のみ、逆走は P1 で追加）
  - `found_exit` / `not_found_exit`（Return / afterへ落とす等）
  - `effects`: なし（P0）
- `LoopPlan::SplitScan`（Pattern7の本質）
  - carriers: `i`, `start`
  - invariants: `s`, `sep`, `result`
  - side-effects: `result.push(segment)`（順序固定）

Plan→Frag の lowerer を共通化:

1. block/value の生成（pattern or lowerer）
2. PHI insertion（`insert_phi_at_head_spanned`）
3. wiring（emission で `Frag` を組む）
4. `emit_frag()`（terminator SSOT）

### 案B: Plan は 1 種に寄せ、差分は “語彙” に寄せる

Scan/Predicate/Split を全部 “Loop + If + Step + Exit” の語彙に落とし、
Plan は「どの基本語彙をどう繋ぐか」だけにする。

利点: Plan 種類が増えにくい  
欠点: 設計が抽象化しすぎると P0 の実装が重くなる

---

## 相談したい質問（ChatGPT Pro への問い）

### Q1. Plan の粒度

Pattern6/7/8 の裾広がりを止めるために、Plan の型はどの粒度が適切か？

- `ScanPlan` / `SplitPlan` のような “中粒度” がよいか
- もっと小さく `LoopPlan { header, body, step, exits }` に寄せるべきか
- Plan 種類を増やさず “パラメータ” で吸収する設計案はあるか

### Q2. 責務分離（フォルダ/モジュール）

どこに Plan を置くべきか？

- 候補: `src/mir/builder/control_flow/` 配下に `plans/`（Extractor/Plan/Lowerer）
- pattern フォルダは “extractor” 専用へ縮退させるべきか
- emission 層は “wiring only” を守るべきか（Pattern8 と同様）

### Q3. `Ok(None)` と `Err` の境界（Fail-Fast）

「不適用」は `Ok(None)` で通常loweringへ戻すとして、`Err` にすべき契約違反は何か？

- 例: extractor が “形は一致” と判断した後に、必要な var が存在しない等
- “close but unsupported” を Err（Fail-Fast）にし、形が違うだけなら Ok(None) にする方針は妥当か

### Q4. side effects（Pattern7）の扱い

副作用（`result.push`）を含む loop を Plan/Frag で表す際、
評価順・SSA・ブロック配置の設計で注意すべき点は？

### Q5. bridge patterns（Pattern9）の扱い

bridge pattern は撤去条件SSOTを作ったが、設計としてどこまで許すべきか？
（例: “bridge を増やさない運用” の現実的なルール）

---

## 期待する回答フォーマット（外部相談用）

1. 推奨する収束アーキテクチャ（1ページ図 + 箇条書き）
2. Phase 272 以降の段階移行手順（P0→P1→撤去）
3. Plan 型の提案（最小フィールド、増殖しない理由）
4. Fail-Fast の境界（Ok(None)/Err のガイドライン）
5. 副作用を含む loop の設計チェックリスト

---

## Non-goals（今回やらない）

- 言語仕様の拡張（大きな機能追加は一時停止中）
- merge/EdgeCFG plumbing の広域改変
- 新しい環境変数トグル追加

