# Phase 229x: simplifycfg edge-arg bridge merge cut

Status: Landed

Purpose
- widen the first `SimplifyCFG` slice just enough to merge jump bridges that carry edge-args into a middle block with no PHIs

Scope
- keep the `pred -> middle` bridge merge shape from `phase228x`
- allow `pred` to be `Jump { edge_args: Some(...) }`
- only when `middle` still has no PHIs
- keep branch edge-args, loop/self-edge cases, and PHI-bearing middle blocks out of scope

Acceptance
- jump-edge-arg bridge blocks merge when the middle block has no PHIs
- middle blocks with PHIs still block the rewrite
- focused simplifycfg tests and quick gate stay green

Follow-on
- keep widening `semantic simplification bundle` one structural cut at a time, then hand off to `memory-effect layer`
