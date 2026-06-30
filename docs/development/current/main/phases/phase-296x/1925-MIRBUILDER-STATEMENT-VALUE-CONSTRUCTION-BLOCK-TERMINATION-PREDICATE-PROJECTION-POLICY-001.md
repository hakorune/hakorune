# 1925 - MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BLOCK-TERMINATION-PREDICATE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BLOCK-TERMINATION-PREDICATE-PROJECTION-POLICY-001
```

## Purpose

Resolve the `BlockTerminationPredicate` subcluster selected after the
StatementValueConstruction diagnostic helper policy.

The selected surface is a read-only predicate:

```text
is_current_block_terminated() -> bool
```

It observes the current block and current function, returns `false` when the
context is missing, and does not mutate builder state. This card records it as
a read-only predicate descriptor, not as a standalone Hako projection owner.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_statement_value_construction_block_termination_predicate_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-statement-value-construction-block-termination-predicate-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_statement_value_construction_block_termination_predicate_projection_policy_guard.sh
```

## Decision

```text
policy = ReadOnlyPredicateDescriptor
owner_edge = mirbuilder::statement_value_construction_block_termination_predicate
projection_surface_selected = 0
registry_descriptor_selected = 0
mutation_owner_selected = 0

next_card =
  MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-PROJECTION-POLICY-001
```

## Evidence

```text
source_count = 1

reads:
  MirBuilder.current_block
  ScopeContext.current_function
  Function.blocks[current_block]
  BasicBlock.terminated

mutates:
  none

markers:
  Check if the current basic block is terminated
  self.current_block
  self.scope_ctx.current_function
  function.get_block(block_id)
  block.is_terminated()
  false
```

## Acceptance

```text
policy = ReadOnlyPredicateDescriptor
access = ReadOnly
mutates = []
default_when_context_missing = false
projection_surface_selected = 0
registry_descriptor_selected = 0
mutation_owner_selected = 0
runtime_or_projection_policy_by_name = 0
manual_family_selection = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no standalone Hako projection surface
no mutation owner selection
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
