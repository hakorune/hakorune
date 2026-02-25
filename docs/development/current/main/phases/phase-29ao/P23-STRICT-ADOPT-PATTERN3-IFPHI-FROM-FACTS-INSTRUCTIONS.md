---
Status: Ready
Scope: code+tests+docs（strict/dev のみ、仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase118_pattern3_if_sum_vm.sh
  - src/mir/builder/control_flow/joinir/patterns/router.rs
  - src/mir/builder/control_flow/plan/normalizer/pattern3_if_phi.rs
  - src/mir/builder/control_flow/plan/facts/pattern3_ifphi_facts.rs
---

# Phase 29ao P23: strict/dev Pattern3 (If‑Phi) を Facts→CorePlan で shadow adopt（DomainPlan依存を減らす）

Date: 2025-12-30  
Status: Ready for execution  
Goal: Pattern3（If‑Phi）も strict/dev では “Facts→CorePlan” を通すことで、DomainPlan 経路の二重定義/ズレを早期に検知できるようにする（既定挙動は不変）。

## 背景

- P17〜P22 で Pattern1 の strict/dev shadow adopt（Facts→CorePlan(skeleton)）を導入し、回帰ゲートで必ず踏むようにした。
- 次に意味があるのは、If‑Phi のような “join 値（post‑phi）” を伴うループ（Pattern3）でも同じ方針を適用し、
  Facts/Planner/Normalizer のズレを strict/dev で Fail‑Fast できるようにすること。

## 非目的

- release 既定経路の変更
- Pattern3 の対応範囲拡張（subset拡張はしない）
- DomainPlan の撤去（段階移行中）

## 実装方針

### 1) Facts→CorePlan の入口を PlanNormalizer に追加（Pattern3専用・最小）

対象:
- `src/mir/builder/control_flow/plan/normalizer/mod.rs`
- `src/mir/builder/control_flow/plan/normalizer/pattern3_if_phi.rs`

追加:
- `pub(in crate::mir::builder) fn normalize_pattern3_if_phi_from_facts(...) -> Result<Option<CorePlan>, String>`

仕様:
- `CanonicalLoopFacts.facts.pattern3_ifphi` が `Some` のときだけ `Some(CorePlan)` を返す
- それ以外は `Ok(None)`（fallback維持）
- 実装は `Pattern3IfPhiFacts -> Pattern3IfPhiPlan` への “薄い変換” を行い、既存の `normalize_pattern3_if_phi()` を呼ぶ
  - ここでロジックを再実装しない（SSOTを増やさない）

### 2) router の strict/dev shadow adopt を Pattern3 に拡張

対象:
- `src/mir/builder/control_flow/joinir/patterns/router.rs`

方針:
- 既存の Pattern1 strict/dev adopt と同じ方針で、Pattern3 も strict/dev では adopt を “強制” する
  - `DomainPlan::Pattern3IfPhi(_)` を選んだのに facts が無い/矛盾する場合は `Err(...)`（strict/dev のみ）

擬似コード:
```rust
if strict_or_dev && matches!(domain_plan, DomainPlan::Pattern3IfPhi(_)) {
    let facts = outcome.facts.as_ref().ok_or("...facts missing")?;
    if facts.facts.pattern3_ifphi.is_none() { return Err("...facts mismatch"); }
    let core = PlanNormalizer::normalize_pattern3_if_phi_from_facts(builder, facts, ctx)?
        .ok_or("...compose rejected")?;
    PlanVerifier::verify(&core)?;
    return PlanLowerer::lower(builder, core, ctx);
}
```

### 3) 回帰ゲート

Pattern3 の regression は既に strict で動いている:
- `tools/smokes/v2/profiles/integration/apps/phase118_pattern3_if_sum_vm.sh` は `HAKO_JOINIR_STRICT=1`

よって P23 を入れたら、`phase29ae_regression_pack_vm.sh` の Pattern3 がそのまま adopt 経路のゲートになる。

## テスト（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

（任意・局所ユニット）
- PlanNormalizer の `normalize_pattern3_if_phi_from_facts` が `Some/None` を返す境界テスト

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P23 追加、Next を更新）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p23): strict/dev adopt pattern3 from facts"`

