# Phase 29ai P14: Planner support（Pattern2 LoopBodyLocal promotion subset）

Date: 2025-12-29  
Status: Ready for execution  
Scope: Pattern2 LoopBodyLocal promotion（TrimSeg / DigitPos）の “要求” を Plan に載せる（仕様不変）  
Goal: promotion の判定・形状解析（Facts）と、採用/非採用（Planner）と、実際の実行（既存 JoinIR 経路）を分離する

## Objective

P12 で Pattern2 LoopBodyLocal の shape を Facts として抽出できるようになった。
P14 では、その Facts を Planner で解釈し「この Pattern2BreakPlan は promotion-required である」という情報を DomainPlan に載せる。

この P14 は **挙動を変えない**。promotion-required を Plan に載せるだけで、実行は従来の JoinIR/Pattern2 の箱群に任せる。
（PlanLowerer/Normalizer を新規に拡張しない。）

## Non-goals

- promotion を Plan/Frag 経路に実装し直す（大改造なので後続フェーズ）
- host binding / derived slot emitter / boundary hygiene の契約変更
- freeze を実行経路で増やす（曖昧なら planner は Ok(None) に倒す）
- 既存 fixture の期待値変更

## Design (Minimal)

### Add-on metadata on Pattern2BreakPlan

`Pattern2BreakPlan` に “promotion の要求” を付加する（デフォルト None）。

例:
- `promotion: Option<Pattern2PromotionHint>`
- `Pattern2PromotionHint::LoopBodyLocal(Pattern2LoopBodyLocalFacts)`（Facts をそのまま載せる）

このフィールドは **planner でのみセット**し、legacy extractor 由来の plan は None のまま。

## Implementation Steps（Critical Order）

### Step 1: Plan vocab に promotion hint を追加（構造だけ）

ファイル:
- `src/mir/builder/control_flow/plan/mod.rs`

やること:
- `Pattern2BreakPlan` に `promotion: Option<Pattern2PromotionHint>` を追加（既定 None）。
- `Pattern2PromotionHint` を追加。

注意:
- 既存の `PlanNormalizer` / `PlanLowerer` が `Pattern2BreakPlan` をどこで使っているか確認し、未参照ならコンパイルのみでOK。
- 参照がある場合は「使わないフィールド追加」だけで済むようにする（match exhaustiveness に注意）。

### Step 2: Planner が hint を埋める（subset のみ）

ファイル:
- `src/mir/builder/control_flow/plan/planner/build.rs`

やること:
- `facts.facts.pattern2_break` が Some かつ `facts.facts.pattern2_loopbodylocal` が Some のときだけ、
  `DomainPlan::Pattern2Break(Pattern2BreakPlan { promotion: Some(...), .. })` を候補として push。
- hint を付与しない普通の Pattern2Break は従来どおり候補生成（P11）を維持。

方針:
- hint 付き/無しの 2候補が同時に立つと CandidateSet が ambiguous になるので、
  “hint があるときは hint 付きのみ候補を出す” を推奨。

### Step 3: unit tests（Facts→Planner 境界）

ファイル:
- `src/mir/builder/control_flow/plan/planner/build.rs`（または facts 側）

やること:
- TrimSeg/DigitPos の Facts を与えたとき `Pattern2BreakPlan.promotion.is_some()` を固定。

### Step 4: Docs / Tracking 更新

更新:
- `docs/development/current/main/phases/phase-29ai/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`

書くこと（最小）:
- P14 は “挙動不変・Planにメタ情報を載せるだけ” を明記。
- Next（P15）候補: promotion hint を JoinIR 側の orchestrator が観測できるように配線（ただし挙動不変）。

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

