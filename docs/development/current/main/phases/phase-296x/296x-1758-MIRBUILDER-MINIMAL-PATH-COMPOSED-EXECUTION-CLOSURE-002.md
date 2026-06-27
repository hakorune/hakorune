---
Status: Active
Date: 2026-06-27
Card: MIRBUILDER-MINIMAL-PATH-COMPOSED-EXECUTION-CLOSURE-002
---

# MIRBUILDER-MINIMAL-PATH-COMPOSED-EXECUTION-CLOSURE-002

## Summary

Continue the composed execution prefix as the integration-only follow-up to
the explicit design-stop frontier. This card consumes the landed semantic
closure report, the same-state composed execution evidence, artifact
manifests/contracts, and route selections to derive a stable first
composition red edge or a green prefix without hand-picking a new semantic
owner.

This is an implementation card. It must not close as a docs-only follow-up.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
manual_next_edge_selection = 0
```

## Goal

Build a code-facing continuation surface that keeps the same prepared/module/
function state handoff and mechanically resolves the next composed step from
existing evidence.

## Inputs

```text
semantic closure report
composed execution evidence
artifact manifests/contracts
route selections
design-stop pause contract
task-order pointer
```

## Output

```text
MinimalMirBuilderExecutionPathComposedExecutionContinuationV2
```

## Required Delta

At least one of the following must land:

```text
composed continuation resolver tool or guard
composed-evidence fixture
code-facing composed route/link guard
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

The continuation must either expose a stable first composition red edge or
prove that the prefix remains green under the same state handoff.

## Acceptance

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
existing_contracts_consumed = 1
semantic_recipe_recopy = 0
new_semantic_projection = 0
manual_next_edge_selection = 0
same_state_handoff_observed = 1
stable_next_slice_token = 1
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
```

## Next

If the composed continuation exposes a stable red edge, follow the
analyzer-derived child owner. If it stays green, keep the integration card
narrow and let the resolver derive the next slice mechanically.

