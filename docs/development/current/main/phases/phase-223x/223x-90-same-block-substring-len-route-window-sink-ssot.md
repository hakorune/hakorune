# 223x-90 Same-Block Substring Len Route-Window Sink SSOT

Status: Landed

Goal
- land the next folded route-window widening on the existing string corridor sink pipeline

Landing
- same-block `substring(...).length()` planning now reads folded `placement_effect_routes` window metadata first
- legacy string-corridor facts remain as a compatibility fallback

Exit
- the next generic placement/effect cut can widen another string sink transform without adding a new proof owner
