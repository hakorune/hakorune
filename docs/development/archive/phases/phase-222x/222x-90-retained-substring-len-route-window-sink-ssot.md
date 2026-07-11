# 222x-90 Retained Substring Len Route-Window Sink SSOT

Status: Landed

Goal
- land the first MIR-side string corridor transform that reads folded `placement_effect_routes` directly

Landing
- retained cross-block `substring(...).length()` now checks `placement_effect_routes` window metadata first
- the old string-corridor fact path remains as a compatibility fallback
- `string_corridor_sink` now refreshes folded route and string kernel plan metadata after it mutates MIR

Exit
- the next generic placement/effect cut can widen another MIR-side transform without reopening ownership questions
