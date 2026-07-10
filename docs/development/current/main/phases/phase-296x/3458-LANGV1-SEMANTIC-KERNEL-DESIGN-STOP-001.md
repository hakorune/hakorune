# 3458 - LANGV1-SEMANTIC-KERNEL-DESIGN-STOP-001

## Status

Decision accepted. This card records the consultation closeout; implementation
is authorized only by `LANGV1-EVALUATED-PLACE-COMPOUND-ASSIGN-001`.

## Established Basis

`LANGV1-CONSTITUTION-001` is accepted. Its exactly-once and source-order law
exposes a concrete current risk: compound assignment clones an lvalue AST into
both the assignment target and read expression. Index and field receiver
sub-expressions can therefore be evaluated more than once.

## Accepted Decision

Choose the Language v1 semantic kernel for:

```text
Outcome:
  Normal(value_or_unit)
  Return(value_or_unit)
  Break
  Continue
  Fault(reason)

Place:
  Local(slot)
  Field(base_once, field)
  Index(base_once, index_once)
```

Decision A is accepted and is now normative in
`docs/reference/language/semantic-kernel.md`:

1. Compound assignment is `EvalPlace -> ReadPlace -> EvalRhs -> Apply ->
   WritePlace`.
2. Canonical `Fault` is non-catchable; cleanup always runs and the first
   cleanup Fault wins after remaining cleanup runs.
3. `Break` and `Continue` target the nearest loop; labels/depth are deferred.
4. `guard let ... else` requires `NoFallthrough`, not a public `Never` type.
5. Canonical normal form uses semantic Value/Place/Outcome operations, not AST
   text rewriting.

## Source Authority

```text
language laws = semantic-contract-charter.md
current grammar = EBNF.md
current cleanup semantics = scope-exit-semantics.md
current type/lifecycle semantics = topic SSOTs
implementation evidence = Rust/Hako parser, MIR, VM, EXE
```

## Non-Authority

```text
AST clone shape alone
current backend behavior alone
source file path
test count
generated agreement without an independent witness
legacy compatibility syntax
```

## Fail-Fast Boundary

```text
no compound-assignment rewrite that duplicates Place evaluation
no catch behavior inferred from legacy try implementation
no cleanup precedence inferred from backend accident
no AST text substitution as semantic proof
no runtime/backend fallback for an unmodeled Outcome
```

## Authorized Implementation Slice

One semantic slice only:

```text
compound assignment through evaluated Place
side-effect fixture proves receiver/index/RHS each run once in source order
Rust and Hako parser witness remains aligned for the accepted surface
unsupported backend rejects before store
```

## Non-Claims

```text
semantic_kernel_decision_accepted = 1
compound_assignment_implementation_authorized = 1
semantic_kernel_implemented = 0
compound_assignment_fix = 0
canonical_fault_catchable = 0
type_contract_activation = 0
null_migration = 0
ownership_policy_change = 0
selfhost_claim = 0
```
