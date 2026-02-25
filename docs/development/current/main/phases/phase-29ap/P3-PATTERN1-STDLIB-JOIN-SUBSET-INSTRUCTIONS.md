---
Status: Ready
Scope: code+tests+docs (Pattern1 stdlib join subset via Plan SSOT)
Related:
  - docs/development/current/main/phases/phase-29ap/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - apps/lib/json_native/utils/string.hako
  - src/mir/builder/control_flow/plan/facts/pattern1_array_join_facts.rs
  - src/mir/builder/control_flow/plan/normalizer/pattern1_array_join.rs
  - src/mir/builder/control_flow/joinir/patterns/router.rs
---

# Phase 29ap P3: Pattern1 stdlib join subset via Plan SSOT

Date: 2025-12-30  
Status: Ready for execution  
Goal: StringUtils.join の loop を Plan/Facts/Normalizer で受理し、JoinIR Pattern1 依存を撤去する（既定挙動は不変）。

## 非目的

- Pattern1 の一般拡張（任意の body を許す）
- 新しい env var / 恒常ログ追加
- 既存の release/strict 挙動やエラー文字列の変更

## 実装方針（SSOT）

### 1) Facts: stdlib join 形状を SSOT 化

- 新規: `pattern1_array_join_facts.rs`
  - 形状（超保守）:
    - condition: `i < arr.length()` のみ
    - body: `if i > 0 { result = result + separator }` / `result = result + arr.get(i)` / `i = i + 1`
    - break/continue/return/if-else は不可
  - 失敗は `Ok(None)`（fallback 維持）

### 2) Planner: DomainPlan を追加

- 新規: `DomainPlan::Pattern1ArrayJoin`
- Facts から 1 候補だけ生成（ambiguous は Freeze）

### 3) Normalizer: CorePlan へ拡張

- 新規: `normalize_pattern1_array_join`
  - 2 PHI（loop var + result）
  - `if i > 0` は body_bb→sep_bb/step_bb 分岐に展開
  - join は `Frag.block_params + EdgeArgs(layout=ExprResultPlusCarriers)` で表現

### 4) Smoke: join を gate に入れる

- 新規 fixture: `apps/tests/phase29ap_stringutils_join_min.hako`
- 新規 smoke: `tools/smokes/v2/profiles/integration/joinir/phase29ap_stringutils_join_vm.sh`
- gate へ追加: `phase29ae_regression_pack_vm.sh`
- docs: `phase-29ae/README.md` に追記

### 5) JoinIR router の Pattern1 を撤去

- `LOOP_PATTERNS` から Pattern1 を外す
- stdlib join/to_lower が Plan 経路で通ることを smoke で確認

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git commit -m "phase29ap(p3): route stdlib join via plan subset"`
