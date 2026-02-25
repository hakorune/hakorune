# Phase 273 P0: Plan Extractor (pure) + PlanLowerer SSOT — completion

Status: ✅ completed (2025-12-22)

Goal:
- Pattern を “Plan抽出プラグイン” に降格し、CFG/PHI/block/value 生成の責務を PlanLowerer に集約して、裾広がりを止める。

What changed (P0):
- Plan 語彙を最小導入（P0 は PoC として pattern-specific を 1 個だけ許容）
- Extractor は pure（builder を触らない）
- Lowerer が block/value/phi を作る唯一の場所
- terminator SSOT（`Frag → emit_frag()`）は維持

PoC target:
- Pattern6（scan with init）

Behavior / limits:
- `dynamic_needle=true` は当時 `Ok(None)` で fallthrough（P1+）
- `step_lit != 1`（reverse scan）も `Ok(None)` で fallthrough（P1+）

Regression:
- `tools/smokes/v2/profiles/integration/apps/phase254_p0_index_of_vm.sh` PASS

Files:
- `src/mir/builder/control_flow/plan/mod.rs`
- `src/mir/builder/control_flow/plan/lowerer.rs`
- `src/mir/builder/control_flow/joinir/patterns/pattern6_scan_with_init.rs`
- `src/mir/builder/control_flow/joinir/patterns/router.rs`

Next (P1+):
- Plan語彙を固定（`Seq/Loop/If/Effect/Exit`）へ畳み、pattern-specific Plan の増殖を止める。

Note (follow-up):
- その後の修正で `dynamic_needle`（Phase 258 の string index_of 系）も PlanLowerer 側で handling され、JoinIR freeze 下でも fallthrough しない経路に寄せている（詳細は Phase 273 README を参照）。
