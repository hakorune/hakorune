# Phase 219x: placement-effect route window len fold

Status: Landed

Purpose
- keep the sibling string guardrail on the generic placement/effect seam
- fold `substring(...).length()` routing through `placement_effect_routes` first

Scope
- add route window fields to string placement/effect routes in MIR metadata
- export those windows in MIR JSON
- read `placement_effect_route_window` first in the len policy C shim
- keep legacy `known_string_concat_chain_len` / plan window / `nyash.string.len_h` fallbacks intact

Non-goals
- no concat planner rewrite
- no fallback removal
- no new string-only route family
- no MIR-side transform outside the route window export needed for this proof

Acceptance
- smoke hits `placement_effect_route_window`
- lowered IR keeps `nyash.string.len_h` / `nyash.string.substring_len_hii` out of the direct route
- current exact keeper remains green

Follow-on
- reuse the same route-window seam for the next sibling string consumer only if the same generic placement/effect proof is already exported
