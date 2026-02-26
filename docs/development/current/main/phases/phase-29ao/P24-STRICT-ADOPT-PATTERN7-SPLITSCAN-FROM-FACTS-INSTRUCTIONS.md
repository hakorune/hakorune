---
Status: Ready
Scope: code+tests+docs（strict/dev のみ、仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern7_splitscan_ok_min_vm.sh
  - src/mir/builder/control_flow/joinir/patterns/router.rs
  - src/mir/builder/control_flow/plan/normalizer/pattern_split_scan.rs
  - src/mir/builder/control_flow/plan/facts/loop_facts.rs
---

# Phase 29ao P24: strict/dev Pattern7 (SplitScan) を Facts→CorePlan で shadow adopt

Date: 2025-12-30  
Status: Ready for execution  
Goal: Pattern7（SplitScan）も strict/dev では “Facts→CorePlan” を通し、DomainPlan 経路のズレ（fallback/近似マッチ）を早期に検知できるようにする（release 既定挙動は不変）。

## 背景

- P17/P23 で Pattern1/3 を strict/dev のみ Facts→CorePlan へ寄せた。
- Pattern7 は ValueJoin（block_params → PHI）実装の代表で、回帰ゲートにも含まれているため、ここを shadow adopt 対象にすると効果が大きい。

## 非目的

- release 既定経路の変更
- Pattern7 の対応範囲拡張（facts subset の拡張）
- 新しい env var/ログ追加
- DomainPlan の撤去（段階移行中）

## 実装方針

### 1) Facts→CorePlan の入口を PlanNormalizer に追加（Pattern7専用・薄い変換）

対象:
- `src/mir/builder/control_flow/plan/normalizer/mod.rs`
- `src/mir/builder/control_flow/plan/normalizer/pattern_split_scan.rs`（または隣接の新ファイルを追加して mod.rs に登録）

追加:
- `pub(in crate::mir::builder) fn normalize_pattern7_split_scan_from_facts(...) -> Result<Option<CorePlan>, String>`

仕様:
- `CanonicalLoopFacts.facts.split_scan` が `Some` のときだけ `Some(CorePlan)` を返す
- それ以外は `Ok(None)`（fallback維持）
- 実装は “薄い変換” のみ:
  - `SplitScanFacts -> SplitScanPlan` を機械的に詰め替える
  - 既存の `normalize_split_scan(builder, SplitScanPlan, ctx)` を呼ぶ
  - SplitScan のロジックを再実装しない（SSOTを増やさない）

擬似コード:
```rust
pub(in crate::mir::builder) fn normalize_pattern7_split_scan_from_facts(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopPatternContext,
) -> Result<Option<CorePlan>, String> {
    let Some(split) = facts.facts.split_scan.as_ref() else { return Ok(None); };
    let parts = SplitScanPlan {
        s_var: split.s_var.clone(),
        sep_var: split.sep_var.clone(),
        result_var: split.result_var.clone(),
        i_var: split.i_var.clone(),
        start_var: split.start_var.clone(),
    };
    Ok(Some(Self::normalize_split_scan(builder, parts, ctx)?))
}
```

### 2) router の strict/dev shadow adopt を Pattern7 に拡張

対象:
- `src/mir/builder/control_flow/joinir/patterns/router.rs`

方針:
- 既存の Pattern1/3 strict/dev adopt と同じ方針で、Pattern7 も strict/dev では adopt を “強制” する
  - `DomainPlan::SplitScan(_)` を選んだのに facts が無い/矛盾する場合は `Err(...)`（strict/dev のみ）

擬似コード:
```rust
if strict_or_dev && matches!(domain_plan, DomainPlan::SplitScan(_)) {
    let facts = outcome.facts.as_ref().ok_or("pattern7 strict/dev adopt failed: facts missing")?;
    if facts.facts.split_scan.is_none() {
        return Err("pattern7 strict/dev adopt failed: facts mismatch".to_string());
    }
    let core = PlanNormalizer::normalize_pattern7_split_scan_from_facts(builder, facts, ctx)?
        .ok_or("pattern7 strict/dev adopt failed: compose rejected")?;
    PlanVerifier::verify(&core)?;
    return PlanLowerer::lower(builder, core, ctx);
}
```

注意:
- “新ログ” は増やさない（route ログも既存のまま）
- adopt は strict/dev のみ（release 既定は従来どおり `lower_via_plan(builder, domain_plan, ctx)`）

## 回帰ゲート（SSOT）

Pattern7 の integration smokes は既に strict で実行されているため、追加の smoke は不要。

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
  - filter `phase29ab_pattern7_` が P24 adopt 経路のゲートになる

## テスト（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P24 を追記、Next を更新）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p24): strict/dev adopt pattern7 split-scan from facts"`
