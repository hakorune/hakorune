# 296x-1649: TypeContext String Literal Leaf Projection

Status: Landed
Date: 2026-06-24
Token: TYPE-CONTEXT-STRING-LITERAL-LEAF-PROJECTION-001

## Decision

Select the live `TypeContext.string_literals` owned leaf projection as the next
direct converter slice.

```text
source:
  src/mir/builder/types/map_value.rs::string_literal

shape:
  builder.type_ctx.string_literals.get(&value).cloned()

lowering:
  StorageAccessFacts
    -> order=Unobserved
    -> ElideToLeafProjection
    -> MapGetOption
    -> OptionStringBox output
```

This slice reuses the existing immutable leaf projection rule. It must not add
a new backend route, route descriptor, or semantic operation.

## Boundary

Allowed:

```text
field type = BTreeMap<ValueId, String>
consumer = get(&ValueId).cloned()
map identity escapes = false
element reference escapes = false
order = Unobserved
value transport = ImmutableStringAtom
```

Fail fast:

```text
missing owned clone -> Deny(ReturnedReadBorrow)
returned aggregate map -> Deny(ReturnedReadBorrow)
observed ordering -> Deny(UnsupportedOrderCapability)
non-string value transport -> Deny(UnsupportedTypeTransport)
unmapped helper side effects -> Deny(UnsupportedDirectShape)
```

## Non-Claims

```text
full_emit_string_claim = 0
full_map_value_publication_claim = 0
map_value_types_claim = 0
map_literal_value_types_claim = 0
BTreeMap_iteration_order_claim = 0
general_option_payload_claim = 0
boxed_i64_payload_claim = 0
producer_side_emit_string_conversion = 0
```

## Acceptance

```text
type_context_string_literal.hako regenerates deterministically
MapGetOption is reused
new production operation kind = 0
focused guard:
  bash tools/checks/rust_lifecycle_type_context_string_literal_derived_artifact_guard.sh
focused MIR/EXE/LLVM-AOT harness green
converter matrix green
no silent hardcode guard green
current state pointer guard green
```

## Closeout

```text
implemented=1
focused_guard=green
converter_matrix=green
new_operation_kind=0
runtime_fallback=0
```
