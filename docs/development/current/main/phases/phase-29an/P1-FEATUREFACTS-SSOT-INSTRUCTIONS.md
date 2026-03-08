---
Status: Active
Scope: code（仕様不変、未接続のSSOT足場）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P1: FeatureFacts SSOT（ExitMap/ValueJoin/Cleanup の材料）

Date: 2025-12-29  
Status: Ready for execution  
Scope: Facts に “Feature（特徴）” の材料を追加する（未接続、仕様不変）

## Objective

- “complete route-family 列挙” を増やさず、**Skeleton に直交する Feature** を Facts 側へ寄せる足場を作る
- まずは最小の **ExitUsage（break/continue/return の存在）** を SSOT として定義し、将来の ExitMap 合成へ繋げる

## Non-goals

- ルーティング順序・観測・エラー文字列の変更
- 既存 planner-first / historical extractor lane の削除
- 新 env var / 恒常ログ追加
- “対象っぽいのに不整合” を Ok(None) で隠す（P1 は **Freeze を増やさない**）

## Implementation（構造優先）

### Step 1: FeatureFacts 型を追加（SSOT）

Add:
- `src/mir/builder/control_flow/plan/facts/feature_facts.rs`

Suggested vocabulary（最小・後拡張可能）:

- `LoopFeatureFacts`
  - `exit_usage: ExitUsageFacts`（実装する）
  - `value_join: Option<ValueJoinFacts>`（P1 は placeholder = `None`）
  - `cleanup: Option<CleanupFacts>`（P1 は placeholder = `None`）
- `ExitUsageFacts { has_break, has_continue, has_return }`

Extraction（保守的）:
- `break/continue/return` の **存在**だけ見る（位置/対応付けは P2+）
- ネストした loop の exit は外側に数えない（外側の ExitMap を誤推論しない）
- `if` の then/else の中は再帰で見る（ただし “未知ノード” は無視）

方針:
- `ExitUsageFacts` は `Default` を持ち、使う側が簡単に埋められるようにする
- ここは **Facts層**なので、Plan/Frag/emit を import しない

### Step 2: facts/mod.rs に module 登録

Update:
- `src/mir/builder/control_flow/plan/facts/mod.rs`

Add:
- `pub(in crate::mir::builder) mod feature_facts;`

### Step 3: LoopFacts に optional で接続（既定挙動は不変）

Update:
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Add:
- `pub features: Option<LoopFeatureFacts>`

Rules:
- `Ok(None)` の gate（“何も取れないなら None”）は **そのまま**
- 既存の route-specific facts が 1 つでも取れた場合だけ `features: Some(...)` を埋める
  - “features だけ取れた” で `Ok(Some)` にしない（既定挙動を変えない）

### Step 4: planner/build.rs の unit tests を調整

Update:
- `src/mir/builder/control_flow/plan/planner/build.rs`

If tests construct `LoopFacts` directly, add:
- `features: None`（または `Some(LoopFeatureFacts::default())` を許可するならそれでもよいが、P1 は `None` 推奨）

## Tests（最低限）

Add unit tests:
- `break/continue/return` が loop body（if を含む）にあると `ExitUsageFacts` が立つ
- nested loop 内の break/continue は外側の `ExitUsageFacts` に影響しない

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p1): add loop feature facts ssot (exit usage)"`
