---
Status: Design stop accepted; D0 execution authorized
Date: 2026-08-04
Decision: evaluate exactly one-level nested pure fallthrough If with explicit
  outer/inner else and one shared i64 binding; keep the existing one-If V1
  profile immutable
Exception: genuine next-shape design consultation after the completed
  implicit Call-RHS D0/D1/D2 row
ParentCurrentCard: docs/development/current/main/investigations/joinir-if-recipe-call-branch-implicit-d0-d2-execution-task-2026-08-04.md
Related:
  - joinir-if-recipe-shape-envelope-d0-design-stop-2026-08-04.md
  - joinir-if-recipe-call-branch-implicit-design-stop-2026-08-04.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/recipe-first-entry-contract-ssot.md
---

# One-level Nested If recipe — design stop

## Selected shape

The next candidate is one recursive control node, not a general nested
language activation:

```hako
local x = 0
if x < 10 {
    if x < 5 {
        x = 1
    } else {
        x = 2
    }
} else {
    x = 3
}
return x
```

The exact envelope is:

```text
resolved-trivial function
  one outer If and one inner If at depth one
  both explicit else, pure fallthrough only
  one shared outer i64 binding assigned once per leaf branch
  one post-outer-merge read/return
  no calls, effects, returns in branches, loops, short-circuit, records,
  match, or additional bindings
```

The inner merge result is the value entering the outer then edge. The outer
merge has the inner result and the outer else value as its two inputs.

## Audit conclusion

The existing canonical CFG/SSA primitives already prove the physical shape:
the resolved control analyzer recursively visits nested If, and existing
tests prove that the inner actual exit (not its then-entry) is an outer
predecessor and that both PHI sets use actual predecessors. The existing
whole-compile unpublished candidate and candidate-abort boundary are reusable.

The current one-If recipe is intentionally closed by `ifs.len() != 1` and the
selected lowerer rejects nested sites. This must become a named nested profile,
not a silent relaxation of `ResolvedTrivialExplicitElse` or
`ResolvedTrivialImplicitElse`.

The proposed owner chain is:

```text
same-pass recursive facts
  -> Nested If recipe/artifact with child composition
  -> one node JoinSig per If plus a physical-ID-free composition witness
  -> one selected canonical If physicalizer (recursive adapter)
  -> existing Binding SSA / CanonicalCfg / PhiTxn owner
```

No detached Builder, second transaction, rollback journal, route retry, or
second PHI/SSA owner is permitted.

## D0 contract boundary

The D0 row must freeze:

* exact depth-one membership and deterministic preorder source ownership;
* outer and inner `IfJoinSigV1` rows and the composition law
  `inner merge -> outer then predecessor`;
* a portable recursive child artifact that carries no physical IDs, raw AST,
  callable headers, or Builder state;
* exact rejection for depth greater than one, implicit nested else, unsupported
  child operations, missing branch transfer, missing continuation read, path
  mismatch, and multiple bindings;
* a separate nested profile/product identifier so the existing one-If V1
  remains unchanged.

The child operation algebra remains leaf-only. Nested control is represented by
the recipe's recursive block/node structure, never hidden in a Call or other
operation.

## D1 ownership census

Before any production wiring, record:

```text
nested facts producer                  = 1
nested mapper/artifact producer       = 1
node JoinSig/composition witness      = 1 owner chain
selected physicalizer caller          = 1
CanonicalCfg/BindingSSA/PhiTxn owners = existing only
new route/transaction/retry           = 0
```

Raw IfForm, A+, CorePlan/JoinIR, located legacy, and JSON-v0 paths stay in
non-selected columns. The census must prove exact preorder coverage and zero
Builder effects in facts/recipe production.

## D2 evidence boundary

Use one production-shaped fixture and the existing candidate abort seam:

* inputs that choose inner-then, inner-else, and outer-else produce 1, 2, and
  3 respectively;
* MIR contains two PHIs, with the inner merge output used as the outer-then
  predecessor/value input;
* predecessor/value pairs equal both sealed JoinSigs and interpreter output;
* a late failure after inner and outer CFG/PHI work discards the unpublished
  candidate, leaves live Builder/module/function/ID fingerprint unchanged,
  and permits fresh reuse.

If the existing seam cannot inject failure after both nested PHIs, stop at D2
design. Do not add a new fault API just to manufacture the proof.

## Explicit non-claims

This row does not activate implicit nested If, nested calls/effects/returns,
loops, short-circuit, records, match, multiple bindings, global PHI/SSA
sole-writer status, legacy route retirement, grammar changes, JSON-v0
widening, Home/ownership, or property retirement.

## Stop conditions

Return to design before implementation if nested composition needs a new CFG or
PHI owner, more than one recursive level, a second physicalizer, raw/name
lookup after the recipe boundary, Option/retry/fallback, or a touched file
over 800 lines.

The design stop is accepted for the bounded D0 row. D1/D2 remain gated on
green membership/composition/rejection evidence; no broader nested shape is
authorized.
