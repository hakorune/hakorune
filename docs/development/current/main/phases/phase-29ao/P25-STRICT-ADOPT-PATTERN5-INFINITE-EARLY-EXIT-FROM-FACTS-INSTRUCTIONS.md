---
Status: Ready
Scope: code+tests+docs（strict/dev のみ、仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_strict_shadow_vm.sh
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - src/mir/builder/control_flow/plan/facts/loop_true_early_exit_facts.rs
---

# Phase 29ao P25: strict/dev LoopTrueEarlyExit を Facts→CorePlan で shadow adopt（historical label 5）

Date: 2025-12-30  
Status: Ready for execution  
Goal: LoopTrueEarlyExit（historical label 5, `loop(true) + early exit`）も strict/dev では “Facts→CorePlan” を通し、DomainPlan 経路との差分（facts/extractor/normalize のズレ）を早期検知できるようにする（release 既定挙動は不変）。

## 背景

- P17/P23/P24 で Pattern1/3/7 を strict/dev のみ Facts→CorePlan へ寄せた。
- LoopTrueEarlyExit は回帰ゲート（phase29ae pack）に含まれているが、現状は strict/dev で Facts→CorePlan を踏んでいない。
- P16 で LoopTrueEarlyExit の exit join を `Frag.block_params + EdgeArgs` へ移しているため、ここを adopt 対象にすると “CorePlan化の実地” が進む。

## 非目的

- release 既定経路の変更
- Pattern5 の対応範囲拡張（facts subset の拡張）
- 新しい env var/恒常ログの追加
- DomainPlan の撤去（段階移行中）

## 実装方針

### 1) Facts→CorePlan の入口を PlanNormalizer に追加（LoopTrueEarlyExit専用・薄い変換）

対象:
- `src/mir/builder/control_flow/plan/normalizer/mod.rs`
- historical implementation file token: `pattern5_infinite_early_exit.rs`

追加:
- `pub(in crate::mir::builder) fn normalize_loop_true_early_exit_from_facts(...) -> Result<Option<CorePlan>, String>`

仕様:
- `CanonicalLoopFacts.facts.loop_true_early_exit` が `Some` のときだけ `Some(CorePlan)` を返す
- それ以外は `Ok(None)`（fallback維持）
- 実装は “薄い変換” のみ:
  - `LoopTrueEarlyExitFacts -> LoopTrueEarlyExitPlan` を機械的に詰め替える
  - historical implementation file token `pattern5_infinite_early_exit.rs` の既存 normalizer を呼ぶ
  - LoopTrueEarlyExit のロジックは再実装しない（SSOTを増やさない）

擬似コード:
```rust
pub(in crate::mir::builder) fn normalize_loop_true_early_exit_from_facts(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopPatternContext,
) -> Result<Option<CorePlan>, String> {
    let Some(p5) = facts.facts.loop_true_early_exit.as_ref() else { return Ok(None); };
    let parts = LoopTrueEarlyExitPlan {
        loop_var: p5.loop_var.clone(),
        exit_kind: p5.exit_kind,
        exit_condition: p5.exit_condition.clone(),
        exit_value: p5.exit_value.clone(),
        carrier_var: p5.carrier_var.clone(),
        carrier_update: p5.carrier_update.clone(),
        loop_increment: p5.loop_increment.clone(),
    };
    Ok(Some(Self::normalize_loop_true_early_exit(builder, parts, ctx)?))
}
```

### 2) router の strict/dev shadow adopt を LoopTrueEarlyExit に拡張

対象:
- `src/mir/builder/control_flow/joinir/route_entry/router.rs`

方針:
- `DomainPlan::LoopTrueEarlyExit(_)` を選んだとき、strict/dev では adopt を “強制” する
  - facts が無い/矛盾する場合は `Err(...)`（strict/dev のみ）
  - release 既定は従来通り `lower_via_plan(builder, domain_plan, ctx)` へ

擬似コード:
```rust
if strict_or_dev && matches!(domain_plan, DomainPlan::LoopTrueEarlyExit(_)) {
    let facts = outcome.facts.as_ref().ok_or("loop_true_early_exit strict/dev adopt failed: facts missing")?;
    if facts.facts.loop_true_early_exit.is_none() {
        return Err("loop_true_early_exit strict/dev adopt failed: facts mismatch".to_string());
    }
    let core = PlanNormalizer::normalize_loop_true_early_exit_from_facts(builder, facts, ctx)?
        .ok_or("loop_true_early_exit strict/dev adopt failed: compose rejected")?;
    PlanVerifier::verify(&core)?;
    return PlanLowerer::lower(builder, core, ctx);
}
```

注意:
- 新ログは増やさない（route ログも既存のまま）

### 3) strict/dev gate を regression pack に追加（新 smoke 1 本）

historical replay basename `phase286_pattern5_break_vm.sh` は strict を付けていない replay lane なので、P25 の adopt 経路が current gate で踏まれない。
そこで “strict shadow adopt” 専用の smoke を追加して pack に組み込む。

追加ファイル:
- `tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_strict_shadow_vm.sh`

内容:
- current semantic fixture alias: `apps/tests/loop_true_early_exit_min.hako`
- historical fixture pin token: `phase286_pattern5_break_min.hako`
- `NYASH_DISABLE_PLUGINS=1` + `HAKO_JOINIR_STRICT=1` で VM 実行
- 期待: 出力が `3`（または `RC: 3`）を含む

pack へ追加:
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に
  `run_filter "loop_true_early_exit_strict_shadow_vm" "loop_true_early_exit_strict_shadow_vm"` を追加

docs 更新:
- `docs/development/current/main/phases/phase-29ae/README.md` に pack 項目として追記

## テスト（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P25 を追記、Next を更新）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- （必要なら）`docs/development/current/main/design/coreplan-migration-roadmap-ssot.md` の “Current/Next” も更新

## コミット

- `git add -A`
- `git commit -m "phase29ao(p25): strict/dev adopt loop-true-early-exit from facts"`
