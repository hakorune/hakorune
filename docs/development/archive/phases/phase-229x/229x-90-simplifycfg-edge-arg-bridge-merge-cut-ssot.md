# 229x-90 SimplifyCFG Edge-Arg Bridge Merge Cut SSOT

Status: SSOT

## Decision

- widen the landed `SimplifyCFG` block-merge slice by exactly one stop-line
- `pred -> middle` bridge merge now allows `pred` to carry `Jump.edge_args`
- keep the existing guards:
  - `middle` reachable
  - `middle` non-entry
  - `middle` exactly one predecessor
  - `middle` no PHIs

## Why

- `phase228x` proved the basic bridge merge owner seam
- the next smallest widening is to allow dead incoming edge-args when the middle block has no PHIs
- this still avoids PHI/loop/branch-edge-arg rewriting

## In Scope

- accept `Jump { edge_args: Some(...) }` on the predecessor bridge
- keep successor PHI predecessor rewrite unchanged
- lock focused tests for:
  - edge-arg bridge merge
  - PHI-bearing middle block guard

## Out of Scope

- `Branch.then_edge_args`
- `Branch.else_edge_args`
- middle PHI elimination
- edge-arg forwarding through PHI-bearing blocks
- loop/self-edge cases
- `SCCP`
- jump-threading

## Acceptance

1. edge-arg jump bridges merge when the middle block has no PHIs
2. PHI-bearing middle blocks still block the rewrite
3. `tools/checks/dev_gate.sh quick` stays green
