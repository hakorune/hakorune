# Phase 220x: placement-effect route-window helper cleanup

Status: Landed

Purpose
- keep the string len route-window seam thin after phase219x
- factor the route-window branch inside the len policy into one helper

Scope
- move the `placement_effect_route_window` emission branch behind a shared helper in the len policy C shim
- keep behavior identical

Non-goals
- no new route kind
- no widening
- no fallback removal
- no MIR-side transform

Acceptance
- same smoke remains green
- same route trace remains green
- no behavior change in emitted IR

Follow-on
- generic placement/effect returns to the actual next lane after this BoxShape-only polish
