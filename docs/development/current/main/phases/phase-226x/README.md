# Phase 226x: placement-effect string scheduling owner cut

Status: Landed

Purpose
- move landed string-corridor transform scheduling fully under the top-level `placement_effect_transform` owner seam

Scope
- let `placement_effect_transform` own module iteration and pre/post-DCE scheduling
- keep `string_corridor_sink` focused on function-local family logic
- keep behavior unchanged in this cut

Acceptance
- optimizer-visible placement/effect scheduling flows through the generic owner seam
- landed string corridor rewrites still behave the same
- quick gate stays green

Follow-on
- continue widening `generic placement / effect` from this owner seam, then hand off to `semantic simplification bundle`
