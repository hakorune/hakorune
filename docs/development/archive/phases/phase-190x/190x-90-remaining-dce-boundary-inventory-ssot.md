# 190x-90: remaining DCE boundary inventory SSOT

Goal
- freeze the remaining DCE backlog into explicit lanes after the landed local-field widening ladder

Lanes
1. loop/backedge local-field partial DCE
   - still local-root based
   - requires dedicated iteration-safety proofs
2. generic memory DCE
   - `Store` / `Load`
   - requires memory/observer ownership instead of local-box-only heuristics
3. observer/control cleanup
   - `Debug`
   - terminators and control-flow anchors

Policy
- keep lane 1 separate from lane 2
- keep lane 2 separate from lane 3
- no single phase may widen local-field backedge reasoning and generic `Store` / `Load` together
