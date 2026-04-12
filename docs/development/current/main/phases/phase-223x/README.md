# Phase 223x: same-block substring len route-window sink

Status: Landed

Purpose
- widen the folded `placement_effect_routes` proof to the next smallest MIR-side string transform

Scope
- make same-block `substring(...).length()` sinking read `placement_effect_routes` window metadata first
- keep legacy string-corridor fact matching as a compatibility fallback

Acceptance
- the same-block substring-len planner still rewrites to `nyash.string.substring_len_hii`
- the planner still collects a same-block plan when legacy string corridor facts are absent but folded route windows are present
- quick gate stays green

Follow-on
- continue the `generic placement / effect` lane with the next narrow MIR-side transform cut
