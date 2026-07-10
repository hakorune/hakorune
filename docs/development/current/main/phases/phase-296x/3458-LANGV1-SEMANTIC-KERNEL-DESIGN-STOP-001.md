# 3458 - LANGV1-SEMANTIC-KERNEL-DESIGN-STOP-001

## Status

Design consultation stop. Do not implement parser, MIR, runtime, verifier, or
backend changes from this card.

## Established Basis

`LANGV1-CONSTITUTION-001` is accepted. Its exactly-once and source-order law
exposes a concrete current risk: compound assignment clones an lvalue AST into
both the assignment target and read expression. Index and field receiver
sub-expressions can therefore be evaluated more than once.

## Decision Required

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

The decision must answer all of these before implementation:

1. Exact source-order for compound assignment: Place evaluation, old-value
   read, RHS evaluation, operator application, store.
2. Whether `Fault` can be caught, and cleanup precedence when body and cleanup
   both produce non-Normal outcomes.
3. Whether `Break`/`Continue` need labels or depth in the v1 kernel.
4. `guard let ... else` rule: require `NoFallthrough` outcome rather than a
   broad static `Never` type, or choose a different contract.
5. Canonical normal form boundary: semantic operations over values/Places,
   rather than AST text rewriting.

## Candidate Decisions

```text
A: five-Outcome kernel + Place + NoFallthrough
   Fault is non-catchable in Canonical mode; cleanup runs and a cleanup Fault
   wins after required remaining cleanup.

B: five-Outcome kernel + Place + explicit catchable Fault subset
   Requires a closed Fault taxonomy and catch boundary in this same decision.

C: defer Outcome/Fault distinction and fix compound assignment locally
   Rejected unless the Constitution is amended: it leaves evaluation and
   cleanup semantics without one owner.
```

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

## Implementation Slice After Decision

One semantic slice only:

```text
compound assignment through evaluated Place
side-effect fixture proves receiver/index/RHS each run once in source order
Rust and Hako parser witness remains aligned for the accepted surface
unsupported backend rejects before store
```

## Non-Claims

```text
semantic_kernel_implemented = 0
compound_assignment_fix = 0
fault_taxonomy_finalized = 0
catch_policy_finalized = 0
type_contract_activation = 0
null_migration = 0
ownership_policy_change = 0
selfhost_claim = 0
```
