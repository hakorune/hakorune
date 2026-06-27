---
Status: Landed
Date: 2026-06-27
Card: MIRBUILDER-MINIMAL-EXECUTION-PATH-FRONTIER-RESOLUTION-001
---

# MIRBUILDER-MINIMAL-EXECUTION-PATH-FRONTIER-RESOLUTION-001

## Summary

Resolve the explicit design-stop frontier mechanically. Consume the semantic
closure report, composed execution evidence, artifact manifests/contracts,
route selections, role/adoption SSOT, and the task-order pointer to derive
exactly one next executable owner.

This is an implementation card for a resolver/guard surface. It must not close
as another docs-only stop.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
manual_next_edge_selection = 0
```

## Goal

Build a code-facing frontier resolver that inputs the existing evidence and
emits a stable `MinimalMirBuilderExecutionPathFrontierResolutionV1` result.

The resolver must derive:

```text
decision.kind
next_slice_token
reason
owner_scope
```

without hand-picking the next owner in docs.

## Inputs

```text
semantic closure report
composed execution evidence
artifact manifests/contracts
route selections
role/adoption SSOT
task-order pointer
```

## Output

```text
MinimalMirBuilderExecutionPathFrontierResolutionV1
```

The output should be fixture-backed and machine-checkable. It may be emitted by
an analyzer, a guard, or a small resolver tool, but it must not be a docs-only
summary.

## Required Delta

At least one of the following must land:

```text
resolver tool or guard
resolver fixture
code-facing frontier resolution report generator
```

## Decision Kinds

The resolver should classify the next step as one of:

```text
ContinueComposedExecutionPrefix
FixCompositionRedEdge
DecomposeCompositeOwner
MaterializeNextLeafArtifact
RunHakoAdoptionDecision
ReadyForMinimalPathMainlinePilot
Blocked
```

## Acceptance

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
resolver_output_fixture_required = 1
existing_evidence_consumed = 1
manual_next_edge_selection = 0
stable_next_slice_token = 1
stable_reason_token = 1
first_red_edge_if_any_is_stable = 1
generated_artifact_existence_as_proof = 0
standalone_smoke_aggregation_as_proof = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

Required checks must include the current-state pointer guard, the no-silent-
hardcode guard, the converter matrix guard, and `git diff --check`.

## Non-Claims

```text
new semantic projection = 0
full minimal-path mainline selection = 0
HakoAdopted = 0
Rust bootstrap retirement = 0
new canonical MIR instruction = 0
runtime try-Hako-then-Rust fallback = 0
```

## Next

Once this card lands, follow the resolver output:

- `ContinueComposedExecutionPrefix` resumes the composed execution prefix.
- `FixCompositionRedEdge` repairs the stable state/transport/linkage mismatch.
- `DecomposeCompositeOwner` splits the composite owner before materializing.
- `MaterializeNextLeafArtifact` closes the next leaf owner.
- `RunHakoAdoptionDecision` enters the adoption lane.
- `ReadyForMinimalPathMainlinePilot` unlocks the mainline pilot gate.

Until then, the next executable owner remains intentionally unresolved.

## Closeout

The resolver is green and currently resolves the explicit design-stop frontier
as `Blocked` with `next_slice_token = MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001`.
No new executable owner is selected from the current evidence set, and manual
next-owner selection remains forbidden.
