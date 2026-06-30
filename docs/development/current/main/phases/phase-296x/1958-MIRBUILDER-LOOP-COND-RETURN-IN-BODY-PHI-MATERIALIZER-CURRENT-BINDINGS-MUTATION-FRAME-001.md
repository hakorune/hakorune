# 1958 - MIRBUILDER-LOOP-COND-RETURN-IN-BODY-PHI-MATERIALIZER-CURRENT-BINDINGS-MUTATION-FRAME-001

## Token

```text
MIRBUILDER-LOOP-COND-RETURN-IN-BODY-PHI-MATERIALIZER-
CURRENT-BINDINGS-MUTATION-FRAME-001
```

## Purpose

Materialize the bounded owner mutation-frame descriptor selected for
`LoopCondReturnInBodyPhiMaterializer.current_bindings`.

This card contracts the replacement for the denied raw returned mutable borrow.
It does not emit Hako, select a HakoShadow projector, materialize a native
source seed, adopt a family, or claim Source Selfhost.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-return-in-body-phi-materializer-
    current-bindings-mutation-frame-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_loop_cond_return_in_body_phi_materializer_
    current_bindings_mutation_frame.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_return_in_body_phi_materializer_
    current_bindings_mutation_frame_guard.sh
```

## Acceptance

```text
input policy = BoundedWithMapOperation
strict_raw_borrow_policy = Deny
frame_kind = BoundedOwnerMutationFrame
owner = LoopCondReturnInBodyPhiMaterializer
owned_field = current_bindings
entry_surface = current_bindings_mut
bounded_callsite = lower_return_in_body_block

state_outputs include:
  LoopCondReturnInBodyPhiMaterializer.current_bindings
  carrier_updates
  builder.variable_ctx.variable_map
  lowered body plans

mutation_order includes:
  EnterBoundedCurrentBindingsFrame
  IterateRecipeItemsInSourceOrder
  DispatchStatementAst
  LowerAssignmentThroughCarrierMerge
  LowerLocalInitThroughCarrierMerge
  LowerMethodCallEffectsWithCurrentBindings
  LowerFunctionCallEffectsWithCurrentBindings
  LowerPrintEffectsWithCurrentBindings
  LowerIfWithJoinedBranchBindings
  RecordCarrierUpdatesFromJoinedCarrierPhis
  LowerReturnWithCurrentBindings
  ReturnLoweredBodyPlans
  ExitFrameWithoutAliasEscape

forbidden_operations include:
  ReturnMutableMapAlias
  StoreMutableBorrow
  CallerOwnedMutableAlias
  RustLifetimeSyntaxTransport
  RuntimeFallback

source_order_markers are present and ordered
bounded_mutation_frame_contract_ready = 1
raw_mutable_alias_selected = 0
returned_mutable_borrow_allowed = 0
stored_borrow_allowed = 0
caller_owned_mutable_alias = 0
hako_generation = 0
hako_shadow_projector_selected = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Result

```text
decision:
  SelectNextDiagnosticClusterResolution

reason_token:
  BoundedCurrentBindingsMutationFrameContractReady

selected_next_card:
  MIRBUILDER-MULTI-AXIS-DIAGNOSTIC-CLUSTER-RESOLUTION-001
```

## Non-Claims

```text
no raw mutable alias transport
no returned &mut Hako surface
no Rust lifetime syntax transport
no Hako generation
no HakoShadow projector
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
