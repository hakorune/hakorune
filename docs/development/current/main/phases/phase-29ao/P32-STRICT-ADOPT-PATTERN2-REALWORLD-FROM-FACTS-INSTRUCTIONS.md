---
Status: Ready
Scope: code+tests+docs（仕様不変・strict/devのみ拡張）
Related:
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - src/mir/builder/control_flow/plan/facts/pattern2_break_facts.rs
  - src/mir/builder/control_flow/plan/facts/pattern2_loopbodylocal_facts.rs
  - src/mir/builder/control_flow/plan/planner/build.rs
  - src/mir/builder/control_flow/plan/composer/shadow_adopt.rs
---

# Phase 29ao P32: Pattern2 real-world（phase263 seg）を planner subset に引き上げ、strict/dev で Facts→CorePlan を踏ませて SSOT 化する

Date: 2025-12-30  
Status: Ready for execution  
Goal: `apps/tests/phase263_pattern2_seg_realworld_min.hako` が JoinIR 回帰 SSOT に入っている以上、strict/dev では **必ず Facts→CorePlan shadow adopt（=tag）を踏む**状態にする。

## 背景（なぜ必要か）

`tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` は以下を回している:

- `phase263_pattern2_`（real-world）
- `phase29ab_pattern2_`（base）
- `phase29ai_pattern2_break_plan_subset_ok_min_vm`（subset）

現状、Pattern2 の strict/dev shadow adopt は “planner 由来の subset” のみが対象で、`phase263_pattern2_*` は shadow adopt を踏まない（=タグで検知できない）。

これは “CorePlan 完全移行（段階1: strict/devで差分検知可能）” の定義に対して穴になる。

## 非目的

- release 既定での経路切替（strict/dev のみ）
- by-name 分岐や一時しのぎのハードコード追加
- 新しい env var 追加
- エラーメッセージ/恒常ログ変更（strict/dev タグは例外・P28/P29で管理）

## 成功条件（Acceptance）

- `./tools/smokes/v2/run.sh --profile quick` が緑（既定挙動不変）
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が緑
- strict/dev の real-world smoke で、`phase263_pattern2_*` が **shadow adopt タグを必須で満たす**
  - 既存タグを流用: `[coreplan/shadow_adopt:pattern2_break_subset]`

## 実装方針（安全順）

### Step 1: `phase263_pattern2_seg_realworld_min.hako` が `Pattern2BreakFacts` を満たす “最小 subset” を SSOT 化

狙い:
- `phase263_pattern2_*` を “subset” として planner で一意に選べるようにする（曖昧さを `Freeze` ではなく `Ok(None)` に倒す）。

方針:
- 既存の `Pattern2BreakFacts` を “誤マッチしない範囲” でだけ拡張し、`phase263_pattern2_seg_realworld_min.hako` を拾えるようにする。
- もし `Pattern2BreakFacts` の拡張がリスク高なら、このP32では新規Facts（例: `Pattern2SegRealworldFacts`）に切り出す（ただし planner/normalizer まで含めて “1本で通る” ことが条件）。

注意:
- “通ること” より “誤マッチしないこと” を優先する（subset は狭くて良い）。

### Step 2: planner を拡張して `phase263_pattern2_*` を “planner 由来の Pattern2Break” に引き上げる

狙い:
- strict/dev shadow adopt の gate は “planner 由来のみ” を維持しつつ、real-world を planner 経由に引き上げる。

やること:
- `src/mir/builder/control_flow/plan/planner/build.rs` で、Facts が `Pattern2BreakFacts` を満たすときに `DomainPlan::Pattern2Break(..)` を候補に追加する（既存の subset 方針のまま）。
- `pattern2_loopbodylocal` facts があれば `promotion` として乗せる（既存挙動の範囲内）。

### Step 3: strict/dev shadow adopt で既存タグが出ることを保証する

狙い:
- strict/dev で “採用された” ことを SSOT タグで検知できるようにする。

方針:
- composer 側は既存の `DomainPlan::Pattern2Break` shadow adopt 経路を使い、タグは `[coreplan/shadow_adopt:pattern2_break_subset]` を流用する。
- つまり “real-world を subset に引き上げる” ことが主作業で、タグの増殖はしない。

### Step 4: 既存の real-world integration smoke を “タグ必須” に昇格

狙い:
- “strict/dev で通った” を **タグ必須で固定**し、将来の退行を機械で検知する。

対象:
- `tools/smokes/v2/profiles/integration/apps/archive/phase263_pattern2_seg_realworld_min_vm.sh`

やること:
- `HAKO_JOINIR_STRICT=1` が既に有効なので、raw output に `[coreplan/shadow_adopt:pattern2_break_subset]` が含まれることを必須にする。
- 既存の “出力が 4” の期待は維持（`filter_noise` 後の出力で判定）。

これにより、回帰パックの `phase263_pattern2_` フィルタにより **自動でゲートされる**。

## リスクと Fail-Fast

- 誤マッチ（Pattern2 じゃないのに Pattern2 と判定）:
  - subset を狭くする、`Ok(None)` に倒す、strict/dev では “対象っぽいのに曖昧” を `Freeze` で落とす（taxonomy に従う）
- 観測の揺れ:
  - タグは strict/dev のみ
  - 既存の generic smoke 出力は tag filtering の対象（P28）に従う

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
