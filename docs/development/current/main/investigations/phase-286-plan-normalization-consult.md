Status: Active  
Date: 2025-12-26  
Scope: Phase 286 の「Plan/Frag SSOT 収束」を進めた結果、次に見えてきた “Plan 生成の正規化” を外部（ChatGPT Pro 等）に相談するためのパケット  
Related:
- docs/development/current/main/phases/phase-286/README.md
- docs/development/current/main/design/joinir-plan-frag-ssot.md
- docs/development/current/main/design/edgecfg-fragments.md
- docs/development/current/main/investigations/phase-272-frag-plan-architecture-consult.md

# Phase 286: “Plan 生成の正規化” 相談パケット（将来設計）

## 0. 相談の意図（最重要）

この文書は「いまの実装を大きく変える提案」を求めるものではない。  
Phase 286 の目的（2本コンパイラ根治を小刻みに進める）から外れないよう、**将来の設計相談（別フェーズで設計SSOTを書いてから着手）**の材料として使う。

具体的には:

- **短期（Phase 286 継続）**: pattern ごとの Plan 化を続けつつ、共通化できる“箱”を少しずつ増やす
- **中期（別フェーズ）**: 「Plan 生成の語彙」をもっと正規化できないかを検討し、設計SSOTを確定してから移行する

## 1. 背景（いま何をしているか）

目的: 移行期間に残っている「2本の lowering」を、構造で 1 本に収束させる。

- Plan line（SSOT）: `CorePlan → Frag(compose) → emit_frag()`
- JoinIR line（移行対象）: `JoinIR → bridge → merge`

Phase 286 では JoinIR line を “第2の lowerer” として放置せず、**Plan/Frag SSOT へ吸収**していく。

## 2. 現状の到達点（PoC の事実）

Plan/Frag 側に載せられたもの（例）:

- Pattern1 (SimpleWhile) : `loop(i < N) { i = i + 1 }`
- Pattern4 (Loop with Continue) : `if (cond) { ...; continue }`
- Pattern8 (BoolPredicateScan) : predicate scan の loop + return true/false
- Pattern9 (AccumConstLoop) : `sum = sum + (const or var)` + `i = i + 1`

共通で効いた設計:

- `phi_bindings`: AST から式を lower する際、variable_map の初期値ではなく **header PHI dst を優先参照**する（SSA を閉じる）
- terminator の SSOT: `Frag + compose::* + emit_frag()`（pattern 側で terminator を直接生成しない）
- Fail-Fast 方針（段階運用）:
  - extractor が `Ok(None)` を返した: 「不適用」なので fallback 可
  - extractor が `Some` を返した後の normalize/lower が失敗: 原則 Err（silent fallback 禁止）

未完の例:

- Pattern3 (Loop with If-phi): Plan extractor は動くが normalizer は stub（現在は legacy fallback）
- Pattern2 (Loop with Break): “exit で値再接続（after_bb の PHI）” が重く、別タスク化

## 3. いま感じている “Plan 側に残る pattern 臭さ”

Pattern を Plan/Frag に載せるほど、Plan 層の中に次が残りやすい:

1. **DomainPlan の variant が増えていく**（PatternXPlan の列挙）
2. **Normalizer の骨格が似ている**のに、pattern ごとに手書きで繰り返す
3. **AST lowering の文脈（phi_bindings 等）が増える**と、引数伝播が増殖する

ここで言う「正規化」とは:

- “Pattern をやめる” のではなく、**Pattern の責務を軽くして、Plan→Frag の後段を統一する**こと

## 4. 今すぐやらない（Non-goals）

以下は綺麗だが、Phase 286 の範囲外になりやすいので別フェーズ扱い:

- Plan 語彙を `Let/If/Exit` のみにして `compile_pat(...)` で全部作る（大きい設計変更）
- Pattern の概念を言語機能（match/pattern）へ一般化する（仕様増）
- CorePlan/Frag の大規模な型変更（影響範囲が広い）

相談したいのは「次の設計SSOT（将来）」であり、いま実装にねじ込む提案ではない。

## 5. 相談したい論点（“小刻み移行”を壊さない正規化）

### Q1. 正規化の境界線はどこが安全か？

現状の分離:

- extractor: AST を見て “適用可能か” を判定し、DomainPlan を作る
- normalizer: DomainPlan を CorePlan（CFG/PHI/blocks/effects）へ変換する
- lowerer: CorePlan → Frag → emit_frag で terminator を確定する（SSOT）

質問:

- この構造を維持したまま、**どの層で共通化すると破壊的変更になりにくいか**？
  - 例: normalizer 内に “LoopSkeletonBox（PHI+blocks+frag wiring）” を導入するのは安全か
  - 例: DomainPlan を “Pattern 名” ではなく “LoopRecipe” に寄せるのは中期の適切な設計か

### Q2. DomainPlan の “型の増殖” をどう抑えるべきか？

現状: `DomainPlan::Pattern1SimpleWhile(...)` のように variant が増える。

相談:

- 増殖を許容して “normalizer を共通化” する方が安全か（variant は残す）
- ある段階で DomainPlan を “構造語彙” に寄せて、variant を減らすべきか

求める回答の形:

- いきなり理想形ではなく、**段階移行のマイルストーン**（フェーズ分割）で示してほしい

### Q3. “Loop の正規語彙” はどこまで必要か？

現状の PoC を見る限り、いくつかの共通骨格がある:

- Pattern1/9: `preheader → header(PHI) → body/step → back-edge → after`
- Pattern4: header PHI は同じだが、body 内で continue 分岐が入る
- Pattern8: loop 途中に return（found/after の2 exit）を持つ
- Pattern3: body 内の if-else が merge PHI（carrier）を必要とする

相談:

- “LoopRecipe” の最小語彙は何か？（例: blocks, carriers, loop_cond, step_effects, exits, optional merge）
- それを CorePlan に落とす normalizer の責務境界はどこが良いか？

### Q4. Fail-Fast と fallback の運用（設計SSOT）

段階運用（今の方針）:

- extractor が `Ok(None)`: legacy fallback OK
- extractor が `Some`: 以降の失敗は Err（silent fallback 禁止）

相談:

- この Fail-Fast を「設計SSOT」として固定するなら、どの地点で verify すべきか
  - 例: DomainPlan 生成直後に “必須フィールドが揃っているか” を verify
  - 例: CorePlan 生成直後に “SSA が閉じているか / terminator が1つか” を verify

## 6. いま欲しい回答（ChatGPT Pro への依頼文：コピペ用）

以下の条件で提案してください。

### コンテキスト（読まずに分かる要約）

- 目的: 移行中の JoinIR line を Plan/Frag SSOT に吸収し、「2本の lowering」を根治したい
- いまは pattern ごとに extractor→DomainPlan→normalizer→CorePlan→Frag→emit_frag を追加している
- Pattern1/4/8/9 は Plan/Frag に載った（PoC とスモークで固定済み）
- Pattern3 は extractor まで動くが normalizer が stub（次の対象）
- Pattern2 は “break exit の値再接続（after PHI）” が重いので deferred

### 制約

- 大規模設計変更は避ける（Phase 286 の範囲外）。やるなら別フェーズで設計SSOTを先に書く。
- by-name ハードコード禁止（Box名文字列一致で分岐などは禁止）
- 環境変数トグル増殖禁止（既存 debug gate の範囲で）
- Fail-Fast 原則（silent reroute を避ける）
- 既定挙動を壊さない（quick smoke は常に green が前提）

### 質問

1. 今の構造（DomainPlan→CorePlan→Frag）を保ったまま “pattern 臭さ” を減らす最小の正規化は何か？
   - 具体例: normalizer の共通骨格（LoopSkeletonBox/BlockLayoutBox/PhiBindingsBox）を導入する順序
2. DomainPlan variant の増殖を抑えるのはいつ/どうやるのが安全か？
   - “variant は許容して normalizer を共通化” vs “variant を減らす設計へ移行”
3. 長期的に “Plan の語彙をさらに正規化” したい場合、別フェーズの設計SSOTとして何を先に決めるべきか？
   - 例: LoopRecipe の最小語彙、不変条件、verify 地点、段階移行手順

### 期待する出力

- いきなり最終形ではなく、**段階移行（1フェーズ=小差分）**のマイルストーン
- 各フェーズの **受け入れ基準**（smoke/verify/contract）
- 破壊的変更になりやすいポイント（やらない方が良い罠）も列挙

## 7. 追記（この相談の位置づけ）

この文書は Active（相談パケット）であり、SSOT ではない。  
結論が固まったら `docs/development/current/main/design/` 配下に “設計SSOT” を新規作成し、Phase 文書はそこへリンクする。

