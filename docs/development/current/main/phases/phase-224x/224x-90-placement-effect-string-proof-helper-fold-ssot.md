# 224x-90 Placement-Effect String Proof Helper Fold SSOT

Status: Landed

Goal
- make the next helper-proof consumer read folded generic placement/effect metadata instead of depending directly on string-family candidate storage

Landing
- `placement_effect_routes` now carries structured string proof payload for string-corridor rows
- publication/materialization helper proof lookup now reads folded routes first
- legacy `string_corridor_candidates` remain as the compatibility fallback during the bridge period

Exit
- the next generic placement/effect cut can widen another string helper consumer without inventing a second proof owner
