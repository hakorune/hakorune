# 219x-90 Placement Effect Route Window Len Fold SSOT

Status: Landed

Goal
- keep the sibling string guardrail on the generic `placement_effect_routes` seam for the boundary len consumer

Why this cut
- `phase218x` made `placement_effect_routes` reusable across current boundary helpers
- the string len route still re-read string-specific plan windows after the generic route inventory already carried the needed window boundaries
- that duplicated route discovery was BoxShape debt on the sibling guardrail lane

Scope
- add `window_start` / `window_end` to string `PlacementEffectRoute`
- export those window fields in MIR JSON
- read route windows first in `hako_llvmc_ffi_generic_method_len_policy.inc`

Non-goals
- removing legacy helper fallbacks
- changing other string consumer routes
- adding a new string-only route family
- MIR-side lowering changes beyond the route-window metadata export

Exit
- boundary len routing hits `placement_effect_route_window` first
- the legacy plan window and `nyash.string.len_h` fallbacks remain compatibility-only
