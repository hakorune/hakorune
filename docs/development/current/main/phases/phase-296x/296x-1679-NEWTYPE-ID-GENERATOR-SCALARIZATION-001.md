# 296x-1679: Newtype ID Generator Scalarization

Status: Complete
Date: 2026-06-25
Token: NEWTYPE-ID-GENERATOR-SCALARIZATION-001

## Decision

Select the focused CoreContext ID-generator scalarization slice.

```text
source owner:
  ValueIdGenerator / BasicBlockIdGenerator scalar state

live consumer:
  CoreContext::next_value
  CoreContext::next_block
  CoreContext::peek_next_value
  CoreContext::peek_next_block
```

This is not generator-object transport. The selected representation is owned
scalar counter state plus nominal i64 ID transports.

## Selected Source Slice

```text
ValueIdGenerator::new
ValueIdGenerator::next
ValueIdGenerator::peek_next

BasicBlockIdGenerator::new
BasicBlockIdGenerator::next
BasicBlockIdGenerator::peek_next

CoreContext::next_value
CoreContext::next_block
CoreContext::peek_next_value
CoreContext::peek_next_block
```

`CoreContext::new` initializes the scalar generator state fields in the
generated box.

## Authority

```text
live Rust generator facts
  -> GeneratorStateFacts
       state_field=next_id
       mutation=PostIncrement | ReadOnly
       range=u32
  -> NominalIdTransportPlan
       ValueId       -> ValueIdAsI64
       BasicBlockId  -> BasicBlockIdAsI64
  -> CoreContextFieldCapabilityPlan
       value_gen -> ValueIdGeneratorState
       block_gen -> BasicBlockIdGeneratorState
  -> generated CoreContext scalar fields
```

`ValueIdAsI64` and `BasicBlockIdAsI64` share the physical i64 lane but remain
different semantic transports.

## Non-Authority

The following must not be claimed or used as proof in this slice:

```text
generator object identity
raw i64 interchangeability between ValueId and BasicBlockId
MirBuilder::next_value_id
reserved ValueId skipping
function-local allocation policy
ValueId::INVALID sentinel semantics
overflow behavior at u32::MAX
```

## Implementation Boundary

Selected generated fields:

```text
CoreContext.value_next_id: i64
CoreContext.block_next_id: i64
```

Selected API methods:

```text
CoreContextApi.next_value(ctx): i64
CoreContextApi.peek_next_value(ctx): i64
CoreContextApi.next_block(ctx): i64
CoreContextApi.peek_next_block(ctx): i64
```

The return lane is i64, but manifests and verifier metadata must record the
nominal transport:

```text
next_value / peek_next_value:
  return_transport=ValueIdAsI64

next_block / peek_next_block:
  return_transport=BasicBlockIdAsI64
```

No backend route, ABI, canonical MIR instruction, runtime fallback, or
MirBuilder allocation policy is selected.

## Acceptance

Generated behavior:

```text
next_value(ctx)      -> 0
next_value(ctx)      -> 1
peek_next_value(ctx) -> 2
next_value(ctx)      -> 2

next_block(ctx)      -> 0
peek_next_block(ctx) -> 1
next_block(ctx)      -> 1
```

Independence:

```text
next_value does not change block_next_id
next_block does not change value_next_id
next_binding remains independent
next_temp_slot remains independent
next_debug_join remains independent
```

Manifest/verifier:

```text
previous excluded_methods no longer include:
  CoreContext::next_value
  CoreContext::next_block
  CoreContext::peek_next_value
  CoreContext::peek_next_block

transport_notes:
  value_id_transport=ValueIdAsI64
  basic_block_id_transport=BasicBlockIdAsI64
  generator_object_transport=0
  invalid_id_claim=0
  reserved_value_id_skipping_claim=0
```

Gates:

```text
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family core-context --check
bash tools/checks/rust_lifecycle_core_context_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Fail-Fast Boundary

```text
Generator object identity observed
  -> Deny(UnsupportedTypeTransport)
     detail=GeneratorObjectTransportRequired

next_id field not scalar u32/i64
  -> Deny(UnsupportedTypeTransport)
     detail=GeneratorStateNotScalar

ValueId and BasicBlockId transport conflated
  -> Deny(UnsupportedTypeTransport)
     detail=NominalIdTransportMismatch

reserved ID skipping required
  -> Deny(DefaultSemanticMismatch)
     detail=ReservedValueIdPolicyRequired

MirBuilder allocation policy required
  -> Deny(UnsupportedDirectShape)
     detail=FunctionLocalAllocationPolicyRequired
```

## Non-Claims

```text
full CoreContext conversion = 0
full MirBuilder crate claim = 0
MirBuilder::next_value_id conversion = 0
reserved ValueId skipping = 0
ValueId::INVALID semantics = 0
BasicBlockId invalid sentinel semantics = 0
arbitrary generator object transport = 0
generator reset support = 0
overflow parity at u32::MAX = 0
function-local allocation policy = 0
source selfhost claim = 0
mainline_selected = 0
new backend route = 0
new ABI = 0
runtime fallback = 0
```

## Closeout Evidence

```text
generator_state_facts=green
nominal_id_transport_plan=green
CoreContext.value_next_id=green
CoreContext.block_next_id=green
CoreContextApi.next_value=green
CoreContextApi.peek_next_value=green
CoreContextApi.next_block=green
CoreContextApi.peek_next_block=green
ValueIdAsI64_and_BasicBlockIdAsI64_not_raw_equivalent=green
generator_object_transport=0
invalid_id_claim=0
reserved_value_id_skipping_claim=0
MirBuilder_allocation_policy_claim=0
runtime_try_hako_then_rust_fallback=0
ny_llvmc_emit_exe_lib_backend_ready_refresh=green
```

Validated with:

```text
cargo build --release --bin hakorune
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family core-context --check
bash tools/checks/rust_lifecycle_core_context_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

The next blocker is a design stop before opening
`MIRBUILDER-DERIVED-CONTEXT-BUNDLE-V1-001`.
