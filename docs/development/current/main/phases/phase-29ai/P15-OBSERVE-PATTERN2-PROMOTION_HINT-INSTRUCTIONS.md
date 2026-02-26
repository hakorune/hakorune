# Phase 29ai P15: Observe Pattern2 promotion hint (strict/dev)

Date: 2025-12-29  
Status: Ready for execution  
Scope: Pattern2 LoopBodyLocal promotion hint を strict/dev 限定で観測可能にする（仕様不変）  
Goal: promotion hint が付与されている事実を stable tag で固定し、次の Plan/Frag 吸収フェーズへ進む前提を揃える

## Objective

- Pattern2 LoopBodyLocal facts が取れるときだけ、strict/dev 限定で安定タグを出して観測できるようにする
- 既定（非 strict）ではログ増やさない・挙動/エラー文字列は不変

## SSOT / Preconditions

- strict 判定は既存の `HAKO_JOINIR_STRICT=1` / `NYASH_JOINIR_STRICT=1` のみ（新 env var 禁止）
- `filter_noise()` は `^[joinir/` や `^[trace:` を落とすため、タグは `[plan/` で出す

## Implementation Steps

### Step 1: タグ出力を追加（planner outcome 参照）

ファイル:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

実装位置:
- `try_build_domain_plan()` の `if let Some(domain_plan) = plan_opt { ... }` 直前付近

補足:
- planner outcome は `build_plan_with_facts()` の結果を使い、facts 直抽出はしない

ガード条件（全部満たすときだけ出す）:
- `entry.kind` が `RuleKind::Pattern2`
- `crate::config::env::joinir_dev::strict_enabled()` が true
- `build_plan_with_facts()` の outcome から `Pattern2LoopBodyLocalFacts` が取れる

出力するタグ（stderr 推奨、1行固定）:
- TrimSeg: `[plan/pattern2/promotion_hint:TrimSeg]`
- DigitPos: `[plan/pattern2/promotion_hint:DigitPos]`

注意:
- `trace::trace()` は `filter_noise()` で落ちるので `eprintln!` を使う
- facts 直抽出は禁止（planner outcome だけを参照する）

### Step 2: integration smoke をタグ検証に昇格

対象ファイル（既存2本の強化）:
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_min_vm.sh`

変更内容:
- 既存の `RC=2`/出力チェックは維持
- 追加で `OUTPUT_CLEAN` にタグが含まれることを必須条件にする

期待:
- seg_min: `[plan/pattern2/promotion_hint:TrimSeg]`
- digit_pos_min: `[plan/pattern2/promotion_hint:DigitPos]`

### Step 3: Docs + CURRENT_TASK 更新

- `docs/development/current/main/phases/phase-29ai/README.md`（P15 完了、タグ仕様をSSOT化）
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
