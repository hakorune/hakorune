---
Status: Active
Date: 2026-06-28
Card: MIRBUILDER-MINIMAL-PATH-COMPOSED-PREFIX-ADVANCE-001
---

# MIRBUILDER-MINIMAL-PATH-COMPOSED-PREFIX-ADVANCE-001

## Summary

Advance the same-state composed execution prefix mechanically. Consume the
semantic closure report, the landed composed continuation evidence, artifact
manifests/contracts, route selections, and the explicit design-stop pause
contract to classify the next unconsumed edge as `LeafArtifact`,
`CompositeOwner`, `Closed`, or `Unknown` without hand-picking a new semantic
owner.

This is an implementation card. It must not close as another docs-only stop.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
manual_next_owner_selection = 0
```

## Goal

Build a code-facing prefix-advance surface that keeps the same
prepared/module/function state handoff and mechanically resolves the next
unconsumed edge from existing evidence.

## Inputs

```text
semantic closure report
composed continuation evidence
artifact manifests/contracts
route selections
design-stop pause contract
task-order pointer
```

## Output

```text
MinimalMirBuilderExecutionPathComposedPrefixAdvanceV1
```

## Required Delta

At least one of the following must land:

```text
resolver tool or guard extension
prefix-advance fixture
code-facing composed prefix / route / link guard
```

Standalone artifact smoke aggregation is not accepted. The composed surface
must observe shared state handoff, not isolated per-artifact replay.

## Minimum Prefix

Use the existing composition prefix as the baseline:

```text
MirModuleMinimalShell
  -> CoreContextApi.next_block
  -> MirFunctionConstructorShell
  -> PreparedStateInstall
  -> LiteralIntegerLowering
```

The prefix advance must either expose a stable first composition red edge or
prove that the prefix remains green under the same state handoff.

## Acceptance

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
existing_contracts_consumed = 1
resolver_output_fixture_required = 1
semantic_recipe_recopy = 0
new_semantic_projection = 0
manual_next_owner_selection = 0
same_state_handoff_observed = 1
next_unconsumed_edge_classified = 1
stable_next_slice_token = 1
stable_reason_token = 1
first_red_edge_if_any_is_stable = 1
standalone_smoke_aggregation_as_proof = 0
generated_artifact_existence_as_proof = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
hako_adopted = 0
full_minimal_path_mainline_selected = 0
```

## Non-Claims

```text
full minimal-path mainline = 0
HakoAdopted = 0
Rust bootstrap retirement = 0
new canonical MIR instruction = 0
runtime try-Hako-then-Rust fallback = 0
semantic owner re-selection = 0
standalone smoke aggregation = 0
```

## Next

If the prefix advance classifies the next edge as `LeafArtifact`, continue the
composed prefix mechanically. If it classifies the edge as `CompositeOwner`,
decompose the owner before materializing. If it reaches `Closed`, run the
adoption or mainline-readiness decision next. If it is `Unknown`, keep the
design stop explicit and fail fast.
