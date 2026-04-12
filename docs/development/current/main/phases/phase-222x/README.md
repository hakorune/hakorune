# Phase 222x: retained substring len route-window sink

Status: Landed

Purpose
- connect the first folded `placement_effect_routes` proof to an existing MIR-side string transform

Scope
- make retained `substring(...).length()` sinking read `placement_effect_routes` window metadata first
- keep legacy string-corridor fact matching as a compatibility fallback
- keep folded route and string kernel plan metadata refreshed after string corridor sink rewrites

Acceptance
- retained cross-block substring-len rewriting still lands on `nyash.string.substring_len_hii`
- the planner can still collect the retained-len plan when legacy string corridor facts are absent but folded route windows are present
- quick gate stays green

Follow-on
- continue the `generic placement / effect` lane with the next narrow MIR-side transform cut
