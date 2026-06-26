---
Status: Landed
Date: 2026-06-27
Card: MIRBUILDER-MINIMAL-PATH-COMPOSED-EXECUTION-CLOSURE-001
---

# MIRBUILDER-MINIMAL-PATH-COMPOSED-EXECUTION-CLOSURE-001

## Summary

Connect the landed minimal-path derived Hako artifacts through one
prepared-state Hako call graph. This card moves from standalone artifact
evidence to same-state execution evidence.

This is an implementation card. It must not close as another docs-only design
stop.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Goal

```text
PreparedMirBuilderStateV1
  -> build_module(ASTNode::Literal(Integer(0)))
```

Rebuild the selected minimal path as a generated Hako composed execution
surface, or fail at a stable named composition red edge.

## Required Delta

At least one of the following must land:

```text
generated composed .hako artifact
Hako composed execution harness
code-facing composed route/link guard
```

Standalone artifact smoke aggregation is not accepted. The composed surface
must observe state handoff between artifacts.

## Minimum Prefix

The first implementation should consume existing artifact manifests/contracts
and attempt this prefix with shared state:

```text
MirModuleMinimalShell
  -> CoreContextApi.next_block
  -> MirFunctionConstructorShell
  -> PreparedStateInstall
  -> LiteralIntegerLowering
```

If the prefix cannot complete, the harness/guard must report the first stable
composition red edge.

## Acceptance

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
new_python_semantic_projector = 0
hako_projector_or_artifact_delta = 1
selected_existing_contracts_consumed = 1
semantic_recipe_recopy = 0
same_state_handoff_observed = 1
fallback_to_standalone_harness = 0
generated_artifact_existence_as_proof = 0
manual_next_edge_selection = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

Required checks are set by the implementation surface, but must include the
existing current-state pointer guard, no-silent-hardcode guard, converter
matrix guard, and `git diff --check`.

## Non-Claims

```text
full MirBuilder object transport = 0
full minimal-path mainline selection = 0
HakoAdopted = 0
Rust bootstrap retirement = 0
new canonical MIR instruction = 0
runtime try-Hako-then-Rust fallback = 0
```

## Next

If green, the next decision can revisit:

```text
MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-001
```

If red, the next owner is the stable composition red edge emitted by this
card's harness or guard.

## Closeout

The composed route guard landed and proved same-state handoff across the
existing prepared-state artifacts without introducing a new backend route,
ABI, or runtime fallback.
