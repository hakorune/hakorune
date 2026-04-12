# Phase 227x: semantic simplification owner seam

Status: Landed

Purpose
- add the first top-level MIR owner seam for the `semantic simplification bundle`

Scope
- route the already-landed DCE and CSE passes through one bundle owner
- keep behavior unchanged in this cut
- leave `SimplifyCFG`, `SCCP`, and jump-threading for later follow-on slices

Acceptance
- optimizer no longer wires DCE and CSE directly
- bundle stats still report the same DCE/CSE counters
- quick gate stays green

Follow-on
- widen the semantic simplification bundle with the first structural simplification cut, then continue to `memory-effect layer`
