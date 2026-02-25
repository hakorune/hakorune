# ChatGPT Pro 質問状（CorePlan SSOT 収束 / Hakorune JoinIR→Plan→CorePlan）

目的: Hakorune の制御フロー正規化（JoinIR→PlanFrag→CorePlan）を「CorePlan = 唯一のSSOT」として収束させる方針が最適か、また残っている“表現力の穴”を最小の語彙追加でどう埋めるのが綺麗か、設計レビューをお願いしたい。

## 0. 先に欲しい回答（期待フォーマット）

- 結論（推奨方針）: 3〜7行
- 重要な設計判断: 箇条書き 5〜10
- 次フェーズの実行可能なTODO（小粒度）: 3〜8項目
- “やらない方が良い” 落とし穴: 3〜8項目

## 1. 現状の到達点（要約）

- JoinIR 側の legacy loop table は撤去済みで、入口は `Plan/Composer → CorePlan` へ収束した。
- 回帰ゲート（SSOT）はこれで固定:
  - quick: `./tools/smokes/v2/run.sh --profile quick`
  - joinir gate: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- stdlib（json_native の StringUtils）について、主要なループ系 API は subset を fixture+smoke で固定し、gate に組み込み済み:
  - trim_start/trim_end, index_of/last_index_of, index_of_string, split, to_upper/to_lower, join, parse_integer（派生含む）など
- 一方で、loop 内 early return を多用する形（例: `is_integer`）や、escape/unescape のように制御が濃いものは「無理筋」と判断し、unsupported を SSOT 化して保留している。

## 2. いまの基本方針（設計の骨）

### 2.1 SSOTの置き場所（目標）

- SSOT（真実）: `CorePlan`（emit/merge は CorePlan/Frag 以外を“再解析”しない）
- 派生: Facts / Planner / DomainPlan
  - Facts: 観測・正規化（lossless寄り）
  - Planner: 0/1/2+ の候補集合から一意化（Ambiguous なら Freeze）
  - DomainPlan: “意図/recipe” として残ってもよいが、最終の verify/emit 契約は CorePlan で固定する

### 2.2 フォールバックの意味（固定）

- `Ok(None)`: 対象外（構造が無い/Plan化不要）
- `Freeze`: 対象っぽいのに曖昧/矛盾/禁止形（strict/dev では Fail-Fast で検知）

### 2.3 制約（絶対）

- 既定挙動不変（releaseの意味論/恒常ログ/エラー文字列は変えない）
- by-name ハードコード禁止（パターン名での一時しのぎ分岐禁止）
- silent fallback 禁止（strict/devで検知可能にする）
- 新しい env var は原則追加しない（必要なら docs へ SSOT 付きで最小限）

## 3. 参照SSOT（この質問状が依拠する設計）

（必要に応じて回答側が読む前提。リンクは repo 内パス。）

- 移行道筋 SSOT: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- Done 判定 SSOT: `docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md`
- post-phi 最終形 SSOT: `docs/development/current/main/design/post-phi-final-form-ssot.md`
- effect 分類 SSOT: `docs/development/current/main/design/effect-classification-ssot.md`
- cleanup/ExitKind 契約 SSOT: `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md`
- composer v0/v1 境界 SSOT: `docs/development/current/main/design/coreloop-composer-v0-v1-boundary-ssot.md`

## 4. 相談したい “残っている穴”（設計の争点）

### 4.1 loop 内 early return（return-heavy）をどう扱うのが綺麗か？

例: `StringUtils.is_integer(s)` は `loop(i < s.length()) { if not is_digit(...) { return false } ... } return true` のような形。

現状は subset 外として unsupported 固定しているが、将来の収束のために「最小語彙」で吸収したい。

質問:
- CorePlan 側で “loop 内 return” を表現する最小の設計は？
  - 例: `ExitKind::Return` を ExitMap に寄せる／Frag境界で表現する／Loop.body の語彙を拡張する、等
- 既存の “Loop.body effect-only” 制約（Fail-Fast）を崩さずに実現するなら、どの構造が最適？
- `Ok(None)` / `Freeze` の境界はどう置くべき？

### 4.2 match / BranchN（多分岐）を skeleton に入れるタイミング

質問:
- skeleton として `If2/Loop` に加えて `BranchN` を早期に追加した方が綺麗？
- 追加するなら、Facts/Planner/Composer の責務分離はどうするのが良い？

### 4.3 Unwind / 例外系（未実装でも設計だけ先に置くべきか）

質問:
- `ExitKind::Unwind` を先に SSOT として置くべき？（未実装でも）
- cleanup/defer の走り方を ExitKind 経由に一本化する設計の最小単位は？

### 4.4 DomainPlan の将来（残す/縮める/撤去）

質問:
- DomainPlan を “意図/recipe” として残す場合の綺麗な境界は？
- どの情報は CorePlan に落とし、どれは DomainPlan に残すべき？
- “pattern爆発” を防ぐ設計（Skeleton + Feature 合成）を徹底するなら、DomainPlan はどれくらい必要？

### 4.5 observability（strict/dev のタグ・Freeze taxonomy）

質問:
- strict/dev のみで出す “安定タグ” のスキーマ設計（最小で壊れない形）は？
- `Freeze::{Ambiguous, Inconsistent, Unstructured, Unsupported, BugInvariant}` の粒度は適切？

## 5. 次フェーズの提案（こちらの暫定案）

こちらの案が “筋が良い” か確認したい。

1. まず stdlib subset 拡張（facts/composeの純変換を厚くする）を継続（Phase 29aq 系）
2. そこで止まった「return-heavy」だけを、CorePlan語彙の最小追加（別Phase）で吸う
3. すべて gate に fixture+smoke で固定し、Ok(None)/Freeze 境界を SSOT 化してから進める

質問:
- 1→2 の順（A→B運用）は最適？
- 2（語彙追加）をするなら “最初に足すべき最小の語彙” は何？

## 6. 回答者への補足（重要）

- コードは既に大規模に動いているため、提案は「小さく・可逆・SSOTが増える」形が嬉しい。
- “とりあえず通す” のハードコード（by-name分岐・隠しトグル・黙ってfallback）は禁止。

