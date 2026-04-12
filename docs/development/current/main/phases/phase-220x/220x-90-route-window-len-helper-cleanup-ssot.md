# 220x-90 Route Window Len Helper Cleanup SSOT

Status: Landed

Goal
- keep the boundary len route-window branch thin and shared

Why this cut
- `phase219x` proved the route-window proof on `placement_effect_routes`
- the branch itself still carried local helper duplication in the len policy
- that duplication is BoxShape debt, not behavior work

Scope
- factor the route-window emission into a helper inside `hako_llvmc_ffi_generic_method_len_policy.inc`

Non-goals
- changing lookup order
- changing the trace tag
- changing any route payload

Exit
- the len policy still reads `placement_effect_route_window` first
- emitted behavior stays unchanged
