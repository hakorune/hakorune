# Generic Loop (v0)

Responsibility:
- Recognize a minimal loop body subset (facts)
- Normalize to CorePlan using only Loop + leaf effects + ExitIf/IfEffect

Non-goals:
- No carriers/value-join
- No nested control-flow or else-branches
- No route-specific semantics beyond the subset

SSOT:
- Condition canon (analysis-only view): `plan/canon/generic_loop/condition.rs`
- Update canon (analysis-only view): `plan/canon/generic_loop/update.rs`
- Step canon (extract + placement): `plan/canon/generic_loop/step.rs`
- Facts: `facts.rs`
- Normalizer: `normalizer.rs`
- Reject reasons: `plan/facts/reject_reason.rs` (log format: `[plan/reject]`)

Step extract order (SSOT, no behavior change):
- `extract_loop_increment_plan` (legacy helper fast path)
- `generic_loop_canon/step_extract/var_step.rs` (`i = i + step_var` and related top-level patterns)
- `generic_loop_canon/step_extract/next_step.rs` (`next_i = i + 1; i = next_i` style)
- `generic_loop_canon/step_extract/complex_step.rs` (`i = (i - x) / k` style)
- `facts/canon/generic_loop/step/extract.rs`: compatibility facade only

Step placement split (SSOT, no behavior change):
- `control_flow/generic_loop_canon/step_placement/facts.rs`: increment/conditional step の形マッチだけ担当
- `control_flow/generic_loop_canon/step_placement/plan.rs`: `RejectReason` を含む placement 判定だけ担当
- `facts/canon/generic_loop/step/placement/*`: compatibility facade only

Condition split (SSOT, no behavior change):
- `control_flow/generic_loop_canon/condition/candidates.rs`: loop_var candidate 観測だけ担当
- `control_flow/generic_loop_canon/condition/bound.rs`: BoundExpr 観測だけ担当
- `facts/canon/generic_loop/condition.rs`: compatibility facade only

Update canon split (SSOT, no behavior change):
- `control_flow/generic_loop_canon/update/literal_match.rs`: update 式の shape match だけ担当
- `control_flow/generic_loop_canon/update/literal_step.rs`: `UpdateCanon` の literal step 生成だけ担当
- `facts/canon/generic_loop/update.rs`: compatibility facade only

Type split (SSOT, no behavior change):
- `canon/generic_loop/types.rs`: Condition/Update/Step の観測型定義

Related docs:
- `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`
- `docs/development/current/main/design/compiler-expressivity-first-policy.md`
