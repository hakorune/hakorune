# Phase 195x: optimization roadmap layer regroup

Status: Landed

Purpose
- regroup the optimization roadmap from feature-pilot rows into design-layer rows
- keep this cut docs-only

Scope
- add one optimization-layer SSOT
- update `CURRENT_TASK.md`, `10-Now.md`, `15-Workstream-Map.md`, and `phase-163x`
- keep the then-current code next as DCE lane A2 while changing the long-range roadmap wording

Non-goals
- no code change
- no DCE widening
- no string / sum / user-box / array / map semantics change

Result
- top-level roadmap now reads as generic layers
- pilot surfaces are demoted under those layers
- `DSE` is explicitly owned by the memory-effect layer, not the simplification bundle
