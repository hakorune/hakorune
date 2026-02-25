Status: Draft (docs-first, post self-host)  
Date: 2025-12-27  
Scope: self-host 後に “脱Rustランタイム（NyRT/.hako）” を進める前提で、MIR lifecycle/RC/weak の境界を SSOT 化するための **相談パケット（この1枚で完結）**  

# Phase 29y（future, post self-host / docs-first）: MIR lifecycle vocab freeze 相談パケット

この文書は「ソースコードや他 docs を参照しなくても回答できる」ことを最優先にしている。

## 0. 相談の意図（お願い）

この相談は「いま実装する」ためではなく、self-host 後に脱Rustを進める前提で、**設計SSOT**を固めるためのもの。

欲しいのは “最終形の完成図” ではなく:
- 小刻み（1フェーズ=小差分）で壊れにくい段階移行
- hidden root を再発させないための観測点（diagnostics）と境界

この相談で “やらない” こと:
- 所有モデルを型に埋め込む等の MIR 大改造
- self-host 前に NyRT を .hako 化する実装
- GC/finalizer のアルゴリズム追加（境界を固定するところまで）

## 1. 前提（この相談で固定したい最小意味論）

### 1.1 用語

- **strong reference**: 生存を延ばす参照。strong が 0 になると object は “死ぬ”。
- **weak reference**: 生存を延ばさない参照。生死の観測に使える。
- **weak_to_strong()**: weak から strong へ “昇格” を試みる。成功したら strong を返し、失敗したら `null` を返す。
- **explicit drop**: `x = null` のように strong を明示的に落とす操作。
- **binding scope**: `local` で宣言された変数のスコープ（ブロック/関数など）。
- **hidden root**: プログラムからは解放済みに見えるのに、実装のどこかが strong を保持し続けてしまうこと（weak_to_strong が成功して露見する）。
- **root surface（root面）**: strong を保持し得る場所の集合（例: VM のレジスタ/一時領域/フィールド表/ハンドル表など）。この集合と観測点を SSOT として固定したい。

### 1.2 いま分かった “事実”（実装からの入力として固定）

- weak は `weak_to_strong()` で生死が観測できるため、**SSA last-use = 言語寿命**のような最適化起点の寿命決定は、意味論として破綻しやすい。
- hidden root は “root面” のどこかで strong が残留すると発生する。よって **root面の定義**と **観測点**が必要。
- LLVM/harness は stdout にログが混入し得るため、smoke は **exit code SSOT**に寄せた方が安定する。

### 1.3 相談に含めたい追加の制約（運用/設計）

- 診断 ON/OFF で意味論（結果）が変わらない（観測だけが増える）。
- 環境変数を増殖させない（既存の verbose/trace と統合する）。
- 既定挙動は変えない（必要なら docs-first で境界を固定してから別フェーズで実装）。

## 2. 相談したいこと（質問は 5つだけ）

ここからが相談の本体。質問は増やさない。

### Q1. RC/weak の “実体” はどこに置くのが一番壊れにくい？

候補（推奨と段階移行の観点で教えてほしい）:
- A) **runtime（NyRT）に実体を置く**。MIR は寿命を語らず、backend は ABI を呼ぶだけ。
- B) MIR/Frag に **薄い寿命 effect（retain/release 等）**を持たせて “見える化” し、backend 差分を減らす。

求める回答:
- 破綻リスク（PHI/loop/early-exit/cleanup）と移植性（LLVM/wasm）を踏まえた推奨
- 推奨の理由（短く）

### Q2. retain/release/weak_drop の “発火点” はどこで SSOT 化すべき？

候補:
- A) `CorePlan → Frag → emit` のどこかで **1回だけ**走る “RC insertion pass” に寄せる（分散実装しない）。
- B) lowering 各所に分散（非推奨寄りだが、理由があるなら聞きたい）。

求める回答:
- PHI/loop/early-exit を踏まえた「壊れにくい置き場所」
- “1箇所” に寄せるなら、どこが一番安全か

### Q3. 関数 ABI（args borrowed / return owned）は妥当？

前提案:
- 引数: borrowed（callee は retain/release しない）
- 戻り値: owned（caller が release 責務を持つ）

求める回答:
- これで破綻しやすいパターンがあるか（例外/早期return/保存）
- 代替案があるなら “語彙が増えない最小案” を提示してほしい

### Q4. weak handle の表現と等価性（identity）をどう固定するのが安全？

論点:
- weak の同一性を何で表すべきか（token / handle / generation / pointer 等）
- ログ表示とデバッグの契約（観測点 SSOT）

求める回答:
- バックエンド差が出にくい表現のおすすめ
- compare/equals を “仕様として”どう扱うべきか（未実装なら未実装として固定すべきか）

### Q5. finalizer/GC はどの層に置くべき？（非目標を明確化）

求める回答:
- “いま固定すべき境界” と “まだ仕様にしない” 境界
- 例: 「finalizer は VM のみ」「LLVM/wasm は未対応」などの “仕様としての未対応” の置き方

## 3. 相談先に求めるアウトプット形式（短く）

- Q1–Q5 に対して各 3〜8行程度
- 段階移行（2〜4ステップ）で、各ステップの受け入れ条件（smoke/verify/contract）
- 破綻しやすい罠（PHI/loop/early-exit/cleanup/diagnostics のどれが危険か）

## 4. Done 条件（この相談パケットを “締める”）

この文書の完了条件は実装ではなく、次フェーズへ切れること。

- Q1–Q5 の回答を受けて、次の実装タスクが **3つ以内**に分割できる（例: ABI固定 / RC挿入pass設計 / 観測点SSOT）
- “やらないこと” が崩れていない（self-host 前に大改造しない）

## 5. 回答メモ（相談結果の要約, 2025-12-27）

※ この節は「相談の回答」を短く保存するためのメモで、質問（Q1–Q5）自体は増やさない。

- **Q1**: 推奨は「実体は runtime（NyRT）SSOT」。ただし hidden root を潰すために RC の “発火イベント列” は可視化したいので、MIRではなく **Frag以降の薄い effect 列（後段挿入）**として扱う（A をSSOT、Bは後段effectとして限定採用）。
- **Q2**: retain/release/weak_drop の発火点は **分散せず 1箇所**。推奨は `emit_frag()` 後〜codegen直前（CFG確定後）の **RC insertion pass**。PHI/loop/early-exit/cleanup を全部見た状態で挿入し、破綻を減らす。
- **Q3**: 関数 ABI は **args borrowed / return owned** が最小語彙で妥当。追加で「borrowed を保存/捕獲/フィールド格納/返すなら acquire（retain）を伴う」を契約として明文化すると事故が減る。
- **Q4**: weak handle は backend差を避けるため **token identity（alloc_id + generation）**を推奨。`weak_to_strong` は Alive のみ成功、Dead/Freed は `null`。ログも token を出す。
- **Q5**: finalizer/GC は Phase 29y では “境界の固定” まで。まず **RCとfiniを混ぜない**（自動finiを入れない）を強く固定し、未対応（例: LLVM/wasm）も仕様として明記する。

まとめ（次フェーズへ切るための 3 タスク案）:
1) NyRT ABI + 関数ABI + weak token identity を docs SSOT 化  
2) RC insertion pass の置き場所/規則（禁止最適化含む）を docs SSOT 化（実装しない）  
3) root surface（カテゴリ）+ 診断API（summary）+ smokeは exit code SSOT を docs SSOT 化  
