# Phase 230x: simplifycfg trivial-PHI bridge merge cut

Status: Landed

Purpose
- widen the `SimplifyCFG` bridge merge just enough to absorb a middle block that only carries trivial single-input PHIs from its sole predecessor

Scope
- keep the `pred -> middle` merge shape from `phase228x` and `phase229x`
- allow `middle` to start with PHIs only when every PHI has exactly one input from `pred`
- rewrite the trivial PHI dst uses before merging so successor PHIs and the moved terminator keep valid SSA values
- keep multi-branch SCCP, loop/self-edge cases, and non-trivial PHI merges out of scope

Acceptance
- a bridge block with trivial single-input PHIs now merges
- successor PHI incoming block ids and incoming values are rewritten to the predecessor/incoming value pair
- focused simplifycfg tests and quick gate stay green

Follow-on
- keep `semantic simplification bundle` on narrow structural cuts, then move to the first `SCCP` constant-branch cut
