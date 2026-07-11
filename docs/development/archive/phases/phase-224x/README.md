# Phase 224x: placement-effect string proof helper fold

Status: Landed

Purpose
- widen the folded `placement_effect_routes` seam from route windows to the next helper-proof consumer

Scope
- add structured string proof payload to `placement_effect_routes`
- make string publication/materialization helper proof lookup read folded routes first
- keep legacy `string_corridor_candidates` as a compatibility fallback

Acceptance
- `placement_effect_routes` exports string proof payload for string-corridor rows
- publication helper proof lookup still succeeds when legacy string-corridor candidates are absent but folded routes remain
- quick gate stays green

Follow-on
- continue the `generic placement / effect` lane with the next narrow MIR-side transform cut
