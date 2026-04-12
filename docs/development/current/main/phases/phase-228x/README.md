# Phase 228x: first simplifycfg block-merge cut

Status: Landed

Purpose
- add the first structural `SimplifyCFG` slice under the landed `semantic simplification bundle`

Scope
- merge `pred -> middle` when:
  - `pred` ends in `Jump { edge_args: None }`
  - `middle` is reachable
  - `middle` is non-entry
  - `middle` has exactly one predecessor
  - `middle` has no PHIs
- keep loop/self-edge cases, edge-arg routes, `SCCP`, and jump-threading out of scope

Acceptance
- semantic simplification bundle reports a dedicated CFG simplification counter
- single-predecessor jump-only bridge blocks disappear without widening into broader CFG rewrites
- focused simplifycfg and bundle tests stay green
- quick gate stays green

Follow-on
- keep widening `semantic simplification bundle` with one narrow structural cut at a time, then continue to `memory-effect layer`
