# 293x-205: C199 Compound Assignment Surface

Status: Complete

## Decision

Promote compound assignment to the default `.hako` source surface for ordinary
assign targets.

Accepted operators:

- `+=`
- `-=`
- `*=`
- `/=`

Accepted targets:

- local variables
- field accesses
- index accesses

## Semantics

Compound assignment is syntax sugar only.

```hako
target += rhs
```

lowers to the existing canonical assignment shape:

```hako
target = target + rhs
```

The same rule applies to `-=`, `*=`, and `/=` with their corresponding binary
operators. C199 does not add overflow behavior, atomic read-modify-write
semantics, allocator-specific meaning, or a backend route selector.

## Implementation

- The parser accepts compound assignment for `Variable`, `FieldAccess`, and
  `Index` assignment targets.
- The AST remains canonical `Assignment { value: BinaryOp { ... } }`.
- `apps/compound-assignment-surface-proof/` fixes VM behavior for local, field,
  and index targets.

## Acceptance

```bash
bash tools/checks/k2_wide_compound_assignment_surface_guard.sh
```
