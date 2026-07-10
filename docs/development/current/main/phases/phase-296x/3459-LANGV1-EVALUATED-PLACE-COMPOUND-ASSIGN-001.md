# 3459 - LANGV1-EVALUATED-PLACE-COMPOUND-ASSIGN-001

## Status

Complete. This card implemented one semantic slice: evaluated-Place compound
assignment. The next grammar decision is stopped by 3460.

## Decision

`LANGV1-SEMANTIC-KERNEL-DESIGN-STOP-001` accepted the five-Outcome kernel,
evaluated Place, source-order compound assignment, cleanup Fault precedence,
non-catchable Canonical Fault, nearest-loop control, and `NoFallthrough`.
`semantic-kernel.md` is the normative owner.

## Scope

Implement compound assignment through one evaluated Place operation:

```text
Local(slot)
Field(base_once, field)
Index(base_once, index_once)

EvalPlace -> ReadPlace -> EvalRhs -> Apply -> WritePlace
```

The implementation must not represent `P op= E` by cloning or re-evaluating
`P` as `P = P op E`.

## Required Evidence

Add focused witnesses for all Place kinds:

```text
x += rhs()
obj().field += rhs()
array()[next_index()] += make_value()
```

They must prove receiver/index/RHS each evaluate once, old-value read precedes
RHS, and the store targets the same evaluated Place. Keep Rust and Hako parser
implementations independent; compare semantic evidence rather than AST shape.

## Fail-Fast Boundary

```text
known unsupported Place/store route -> reject before operation
runtime-dependent unsupported store -> reject before store
fallback store route -> forbidden
duplicate receiver/index evaluation -> test failure
```

## Non-Claims

```text
semantic_kernel_implemented = 0
cleanup_runtime_implementation = 0
guard_let_verifier_implementation = 0
catchable_fault_activation = 0
type_contract_activation = 0
null_migration = 0
ownership_policy_change = 0
capability_verifier_activation = 0
selfhost_claim = 0
runtime_backend_fallback = 0
```

## Acceptance

```text
compound_assignment_ast_clone = 0
evaluated_place_local = 1
evaluated_place_field = 1
evaluated_place_index = 1
place_evaluation_once = 1
source_evaluation_order_fixed = 1
unsupported_store_fails_before_store = 1
```

## Closeout

The parser preserves one compound-assignment target; MIR evaluates that target
into Local, Field, or Index once; and focused parser/MIR/VM-reference tests
prove source order and fail-fast behavior. AST JSON preserves the explicit
compound-assignment shape.

## Next

`LANGV1-GRAMMAR-DESIGN-STOP-001` is the next active blocker. Do not implement
grammar changes until its decision is accepted.
