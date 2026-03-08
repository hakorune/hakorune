---
Status: Ready
Scope: code+tests+docs（strict/dev のみ、仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/scan_with_init_strict_shadow_vm.sh
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - historical implementation file token: src/mir/builder/control_flow/plan/normalizer/pattern_scan_with_init.rs
  - src/mir/builder/control_flow/plan/facts/loop_facts.rs
---

# Phase 29ao P27: strict/dev ScanWithInit（historical label 6）“planner subset” を Facts→CorePlan で shadow adopt

Date: 2025-12-30  
Status: Ready for execution  
Goal: ScanWithInit（historical label 6）のうち **planner が Facts から一意に作れる subset** だけを strict/dev で Facts→CorePlan に寄せ、DomainPlan 経路との差分（facts/extractor/normalize のズレ）を早期検知する（release 既定挙動は不変）。

## 背景

- ScanWithInit は OK/contract/variant（reverse/matchscan 等）が混在している。
- 現状の Facts（`scan_with_init`）は “最小 subset” に寄せてあり、すべての variant を表現できるわけではない。
- そこで P27 は P26 と同様に **planner 由来のときだけ** strict/dev shadow adopt を行う：
  - planner が作れない variant は従来どおり extractor（fallback）経路で維持
  - planner subset は Facts→CorePlan で通し、ズレを fail-fast で検知

## 非目的

- ScanWithInit の reverse/matchscan など、Facts 未対応 variant を adopt 強制すること
- Facts subset の拡張（別Pで扱う）
- 新しい env var/恒常ログ追加
- release 既定経路の変更

## 実装方針

### 1) Facts→CorePlan の入口を PlanNormalizer に追加（ScanWithInit subset）

対象:
- `src/mir/builder/control_flow/plan/normalizer/mod.rs`
- historical implementation file token `pattern_scan_with_init.rs`

追加:
- `pub(in crate::mir::builder) fn normalize_scan_with_init_from_facts(...) -> Result<Option<CorePlan>, String>`

仕様:
- `CanonicalLoopFacts.facts.scan_with_init` が `Some` のときだけ `Some(CorePlan)` を返す
- それ以外は `Ok(None)`（fallback維持）
- 実装は “薄い変換” のみ（SSOT増殖禁止）:
  - `ScanWithInitFacts -> ScanWithInitPlan` を機械的に詰め替える
  - `early_return_expr` / `not_found_return_lit` / `scan_direction` / `dynamic_needle` は planner の既定（subset）に合わせる
    - 例: `early_return_expr = Variable(loop_var)`, `not_found_return_lit = -1`, `dynamic_needle = false`
  - 既存の `normalize_scan_with_init(builder, ScanWithInitPlan, ctx)` を呼ぶ

### 2) router の strict/dev shadow adopt を Pattern6 subset に追加（planner 由来のみ）

対象:
- `src/mir/builder/control_flow/joinir/route_entry/router.rs`

方針:
- strict/dev でも Pattern6 全体には強制しない（P26と同様）
- 条件:
  - 選ばれた `domain_plan` が `DomainPlan::ScanWithInit(_)`
  - かつ `outcome.plan` が `Some(DomainPlan::ScanWithInit(_))`（planner が作った subset）
- 上記を満たすときだけ adopt:
  - `facts.facts.scan_with_init` が `Some` であることを assert（fail-fast）
  - `PlanNormalizer::normalize_scan_with_init_from_facts(...)` を呼び、`Some(CorePlan)` を要求（fail-fast）
- 上記以外（fallback extractor 由来）の `DomainPlan::ScanWithInit` は従来経路のまま:
  - `lower_via_plan(builder, domain_plan, ctx)`

これにより、reverse/matchscan 等の variant で facts が無い場合も strict/dev で無駄に落ちない（互換維持）。

### 3) 回帰ゲート（SSOT）

ScanWithInit の integration smokes は既に strict で実行されている（current semantic wrapper: `scan_with_init_strict_shadow_vm.sh`）。
よって新しい smoke 追加は不要で、`phase29ae_regression_pack_vm.sh` の `scan_with_init_regression_pack_vm.sh` がそのまま adopt 経路のゲートになる。

## テスト（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P27 追加、Next 更新）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`（Current/Next 更新）

## コミット

- `git add -A`
- `git commit -m "phase29ao(p27): strict/dev adopt scan-with-init subset from facts"`
