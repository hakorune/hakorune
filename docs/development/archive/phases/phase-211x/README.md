# Phase 211x: generic placement/effect owner seam

Status: Landed

Purpose
- fold the landed string corridor, sum placement, and thin-entry pilots into one generic placement/effect route inventory
- keep the first code cut inspection-only and behavior-preserving

Scope
- add a MIR-side `placement_effect_routes` owner seam
- wire the owner seam into semantic refresh
- export the folded routes through MIR JSON
- keep the current string / sum / thin-entry pilot metadata intact underneath the folded view

Non-goals
- no MIR rewrite or lowering widening
- no new backend consumer switch in this cut
- no DCE / simplification-bundle widening
- no closure / array / map semantic widening

Acceptance
- `placement_effect_routes` exists as a MIR-side owner seam
- semantic refresh populates the folded route inventory
- MIR JSON exports the folded route inventory
- `git diff --check`

Follow-on
- broader `generic placement / effect` fold-up remains backlog under the same roadmap row
- the immediate next layer may move independently from this owner seam cut
