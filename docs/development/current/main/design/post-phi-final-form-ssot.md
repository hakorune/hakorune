---
Status: SSOT
Scope: JoinIR/PlanFrag の join 値（PHI 相当）の“最終表現”と局所検証（post-phi）
Related:
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/joinir-plan-frag-ssot.md
- docs/development/current/main/design/edgecfg-fragments.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Post-PHI Final Form (SSOT)

目的: JoinIR/PlanFrag 導線における「pred 由来で値が変わる join」を、**局所的に検証できる最終表現**として固定し、emit/merge が再解析で穴埋めする余地を消す。

この文書で言う “post-phi” は、語として「PHI ノードを完全に排除した後」という意味ではない。
ここでは **“PHI 相当の join 値” が最終的にどう表現され、どこで verify されるか**を SSOT 化する。

## 1. 用語

- join 値: predecessor によって値が変わる値（if/loop の merge 点で必要になる）
- join 入力: pred → join 値の対応（どの pred がどの値を渡すか）
- layout: join 入力の順序と名前（carrier 名）を固定する SSOT

## 2. SSOT: join 値は “対応表” と “順序” を必ず持つ

### 2.1 順序の SSOT

JoinIR merge では、carrier の順序は `BoundaryCarrierLayout` を SSOT とする。

- SSOT 実装: `src/mir/builder/control_flow/joinir/merge/boundary_carrier_layout.rs`
- 入口の検証（strict/dev）: boundary hygiene / header PHI layout checks

### 2.2 対応表の SSOT

JoinInlineBoundary は join の入力を明示で持つ（“暗黙の推論”は禁止）。

- SSOT データ: `JoinInlineBoundary::join_inputs`（名前/順序は layout と一致する）
- 禁止: emit/merge が CFG/AST を覗いて join 入力を再推論すること

## 3. Invariants（局所 verify できる不変条件）

不変条件は「どこで落とすか」も含めて SSOT とする。

### 3.1 Layout 整合（入口で Fail-Fast）

- L1: `BoundaryCarrierLayout` の carrier 順序は一意で、同名が存在しない
- L2: `boundary.join_inputs` の長さ/順序/名前が layout と一致する
- L3: header PHI の順序が layout と一致する（LoopHeaderPhiInfo との整合）

検証:
- `src/mir/builder/control_flow/joinir/merge/contract_checks/boundary_hygiene.rs`（strict/dev）
- `src/mir/builder/control_flow/joinir/merge/contract_checks/header_phi_layout.rs`（strict/dev）

### 3.2 Pred 分類（entry vs latch）と値の流し先

Loop の join は “entry 値” と “latch 値” を混ぜない。

- P1: entry preds と latch preds を分類し、latch 側が初期値で上書きされない
- P2: multi-pred join では、許可された条件（例: state の intersection 等）以外で initial を採用しない（曖昧さは Freeze/Fail-Fast）

検証:
- merge の contract checks（strict/dev）
- debug assertions（debug ビルド）

### 3.3 Header PHI の安全条件（debug_assertions）

Header PHI の dst は “他の命令 dst として再利用しない”。

検証:
- `src/mir/builder/control_flow/joinir/merge/debug_assertions.rs`（debug_assertions）

## 4. “危険な失敗モード” と再発防止

### 4.1 一般 pattern が JoinIR 専用 pattern を飲み込む

例: nested loop（phase1883）が plan 側の一般 pattern に誤マッチし、JoinIR の dedicated lowerer が選ばれない。

SSOT ルール:
- より一般な extractor は、上位形（nested loop 等）を構造条件で `Ok(None)` に倒す
- 入口で by-name 特例を足さない（設計的に禁止）

## 5. Verification（SSOT）

JoinIR regression gate（VM）:

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

