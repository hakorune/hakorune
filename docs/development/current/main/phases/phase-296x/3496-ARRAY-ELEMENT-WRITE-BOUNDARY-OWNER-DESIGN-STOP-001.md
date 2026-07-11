# 3496 - ARRAY-ELEMENT-WRITE-BOUNDARY-OWNER-DESIGN-STOP-001

## Status

Active design consultation stop. Do not change Array acceptance, MIR
vocabulary, runtime methods, planner lowering, or type-contract activation
until the write owner and identity law are accepted.

Decision: pending.

## Objective

Select one behavior-preserving owner for every accepted Array element write
before Typed `Array<T>` semantic activation.

```text
array literal append
ArrayBox.push(value)
ArrayBox.set(index, value)
ArrayBox.insert(index, value)
planner-produced equivalents
```

Current `builder/types/array_element.rs` only observes calls and updates
receiver-local `MirType::Array` facts. It is representation evidence, not a
semantic contract or write owner.

## Required Decisions

1. Should canonical MIR retain an explicit `ArrayElementWrite` operation, or
   should a typed call-owned recipe remain visible until backend lowering?
2. Which owner resolves literal/push/set/insert into the canonical boundary
   without method-name checks being duplicated across builder and planner?
3. What stable identity follows one Array object through aliases, Copy, PHI,
   loop carriers, parameters, fields, and returns without using ValueId alone?
4. Is literal append represented by the same operation with
   `LiteralAppend`, or by construction rows plus final publication?
5. What is the exact evaluation order for receiver, index, and value, and
   where does publication occur?
6. Which existing planner-produced calls bypass the ordinary method-call
   observation path, and how are they structurally forced through one owner?
7. Which verifier evidence proves every accepted write path converged before
   Typed Array activation?

## Candidate Shape

```text
ArrayElementWrite:
  boundary = LiteralAppend | Push | Set | Insert
  receiver
  optional index
  value
  stable array identity evidence

order:
  evaluate receiver once
  -> evaluate index once when present
  -> evaluate value once
  -> publish through one write owner
```

The candidate is not accepted yet. A generic `Call` metadata sidecar is an
alternative only if it remains durable across CFG/SSA refresh and cannot be
silently dropped by planner/backend consumers.

## Source Authority

```text
canonical Array literal and method-call grammar rows
ArrayMethodId name/arity registry
semantic evaluation-order law
current accepted builder and planner producers
```

## Non-Authority

```text
MirType::Array
homogeneous literal inference
value_types or value origin
storage kind and fast-path plans
method name alone
helper name
successful VM execution
```

## Fail-Fast Boundary

```text
accepted write path without selected owner -> fail
planner bypasses owner -> fail
identity lost at Copy/PHI/loop/field/call boundary -> fail
representation fact used as contract proof -> fail
evaluation duplicated or reordered -> fail
backend silently treats unknown write as generic call -> fail
```

## Minimum Implementation Slice After Acceptance

```text
1. one canonical write owner/API
2. literal, push, set, insert producer convergence
3. planner-produced call convergence
4. Copy/PHI/loop identity evidence
5. MIR JSON observation
6. verifier completeness guard
7. behavior-preserving VM fixtures
8. typed_array_contract_activation = 0
```

Do not combine BoxShape convergence with source-owned `Array<T>` carrier or
runtime element checks. Typed Array activation is a later card.

## Non-Claims

```text
array_element_write_owner_decided = 0
array_element_write_owner_implemented = 0
typed_array_contract_activation = 0
array_identity_contract_decided = 0
new_array_acceptance = 0
backend_array_lowering = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```
