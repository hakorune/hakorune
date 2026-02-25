---
Status: Active
Scope: docs-first（仕様不変）
Related:
- docs/development/current/main/phases/phase-29al/README.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/design/joinir-plan-frag-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29al P1: Post-PHI final form SSOT（docs-first）

Date: 2025-12-29  
Status: Ready for execution  
Scope: join 値（PHI 相当）の最終表現と局所 verify を SSOT 化する（仕様不変）

## Objective

- “pred によって値が変わる join” を、**暗黙推論なし**で表現できることを SSOT 化する
- layout / mapping / pred 分類 / verify の責務境界を 1 枚に固定し、再解析や if 地獄の余地を消す

## Non-goals

- PHI の完全排除（これは別フェーズ）
- 既存の error 文字列の変更
- 新 env var 追加
- 既定挙動の変更（release は不変）

## Steps

### Step 1: SSOT を 1 枚に固定

Add:
- `docs/development/current/main/design/post-phi-final-form-ssot.md`

Must include:
- “post-phi” の定義（ここでは PHI 排除ではなく、join 値の最終表現/verify の SSOT）
- `BoundaryCarrierLayout` と `JoinInlineBoundary::join_inputs` の関係
- 検証点（contract_checks / debug_assertions）の一覧
- 危険な失敗モード（一般 pattern が専用 pattern を飲む）と SSOT ルール

### Step 2: 参照導線を追加

Update:
- `docs/development/current/main/design/planfrag-ssot-registry.md`（SSOT 参照の追加）
- 必要なら `docs/development/current/main/design/joinir-plan-frag-ssot.md` の “関連ドキュメント” に追加

### Step 3: Phase 入口を更新

Update:
- `docs/development/current/main/phases/phase-29al/README.md` に P1 を追加（P0→P1 の流れを明確化）

### Step 4: Now/Backlog/CURRENT_TASK を更新

Update:
- `docs/development/current/main/10-Now.md`（Current Focus と Next を更新）
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Verification

- docs-only のため `cargo build` は必須ではない
- ただし Gate の SSOT を維持するため、次は任意で実行:
  - `./tools/smokes/v2/run.sh --profile quick`
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "docs(phase29al): post-phi final form ssot"`

