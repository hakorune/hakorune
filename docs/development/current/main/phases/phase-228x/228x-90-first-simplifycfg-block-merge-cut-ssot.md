# 228x-90 First SimplifyCFG Block-Merge Cut SSOT

Status: SSOT

## Decision

- the first structural widening under `semantic simplification bundle` is a narrow block merge
- this cut only merges `pred -> middle` when:
  - `pred` ends in `Jump { edge_args: None }`
  - `middle` is reachable
  - `middle` is not the entry block
  - `middle` has exactly one predecessor
  - `middle` has no PHI instructions
- loop/self-edge cases stay out of scope

## Why

- `phase206x` already locked the DCE / `SimplifyCFG` handoff boundary
- the next safe move is structural CFG cleanup that does not widen into edge-arg, PHI, or loop-carried rewriting
- block-merge is a canonical `SimplifyCFG` starter slice and gives the bundle a real structural owner without reopening family-specific passes

## In Scope

- add a dedicated `simplify_cfg` pass under the semantic simplification owner seam
- add one dedicated `cfg_simplified` counter to optimizer stats
- rewrite successor PHI predecessor ids when the merged-away block appears as an incoming edge

## Out of Scope

- edge-arg forwarding
- branch threading
- loop/backedge collapse
- PHI elimination in the merged target
- `SCCP`
- jump-threading

## Acceptance

1. `semantic_simplification` runs one structural simplifycfg slice before DCE/CSE
2. stats report CFG simplifications separately from `intrinsic` / `reorder`
3. focused tests lock:
   - linear single-predecessor jump merge
   - successor PHI predecessor rewrite
   - edge-arg guard
4. `tools/checks/dev_gate.sh quick` stays green
