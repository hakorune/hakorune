# Portable Loop Recipe Contract

This directory owns the Builder-free, selfhost-portable semantic contract for
Loop lowering.

## Authority

- `LoopRecipeArtifactV1` owns schema version, a required source wire claim,
  producer route receipt, and one `LoopRecipeV1`.
- The source wire claim names one declared-function body by compilation-unit
  and function ordinals, then maps the ordered Loop arena exactly 1:1 to unique
  typed paths.
- `LoopRecipeV1` owns one closed recursive control algebra represented on the
  wire as ordered arenas with recipe-local keys. It contains no source or route
  authority.
- The semantic Loop node is the single long-term loop shape: `Always` is a
  degenerate predicate, and a loop with no explicit `break`/`continue`/`return`
  is the same frame with fewer exit rows. Nested loops recurse through the
  same `LoopRecipeItemV1::Loop` node; they are not a second semantic family.
  `LoopV0`, `LoopTrue`, and `LoopCond` names belong to current producers or
  legacy physical adapters, not to the portable semantic SSOT.
- External/pre-loop values are named explicitly by `inputs`; every other value
  has exactly one operation result.
- A carrier entry must be available before its owning Loop is entered. The
  caller-zero `LoopJoinSigElaboratorV1` elaborates the bounded Accum logical
  edges and visible ancestor-carrier payloads; its verified product is
  non-`Clone` and has no production caller. Full dominance/predecessor and
  wider nested-exit closure remain later M6 work.
- `LoopRecipeVerifierV1` consumes only `LoopRecipeV1`. It cannot select or retry
  a route and cannot inspect source ownership or the producer receipt.
- `LoopRecipeVerifierV1` owns structural recipe preconditions; the JoinSig
  elaborator owns logical dataflow/edge rows. Do not merge these authorities.
- Artifact verification proves only the wire claim's internal structure:
  canonical source-key order, exact coverage, unique paths, root entry through
  `body_item`, and direct-child path grammar. It does not prove that the named
  function or AST sites exist, nor that they produced this recipe.
- `StructurallyVerifiedLoopRecipeSourceClaimV1` is therefore an internal,
  non-`Clone` validation capability. Its wire DTO remains intentionally
  `Clone`; neither type is source authority.

## Forbidden dependencies

This subtree must not import AST nodes, `MirBuilder`, `CorePlan`, physical
`ValueId`/`BasicBlockId`, `Frag`, route composers, callbacks, retry, or legacy
mutation-family policy.

The control tree is the sole source of connectivity. Logical CFG/JoinSig and
physical MIR are later elaborations; they are not duplicated in this wire
contract.

Arena rows and recursive traversal both use canonical preorder. Artifact source
paths use only the closed steps `body_item`, `scope_body_item`, and
`loop_body_item`. A root path starts with exactly one `body_item`; later steps
may describe outer `scope_body_item`/`loop_body_item` ancestry. A semantic child
is exactly its parent's path plus one `loop_body_item` and zero or more
`scope_body_item` steps. A second `body_item` or `loop_body_item` cannot skip an
intermediate semantic Loop.

Normalization has three deliberate views: full artifact, source-bound
(source + semantics, without route receipt), and semantic-only (without source
or route). Schema V1 is still caller-zero and pre-production, so this is a V1
contract correction with no compatibility adapter or V2 alias.

## Extension rule

Start with the Accum-ready operation vocabulary. Add one typed operation only
when a route migration supplies a counterexample and fixtures. Never add opaque
AST/statement payloads or legacy-emitter escape hatches.

## Post-cutover convergence gate

After the portable producer has one production caller and the canonical
session is the physical lifecycle owner, the remaining family adapters are a
temporary implementation detail. The cleanup target is:

```text
frame producers (LoopV0 / LoopTrue / LoopCond)
  -> one general frame adapter (condition + typed exit rows)
Nested
  -> recursive use of the same frame adapter
Generic
  -> classified and removed; no post-effect retry
```

The gate is semantic and evidence-based, not a rename: all fixtures must have
the same verified Recipe/JoinSig winner, CFG/PHI/value parity, and no legacy
family production caller. Before the one recursive physicalizer can absorb all
frame producers, JoinSig must also admit the shared logical branch/merge
obligations for exits that currently reject as a branch-merge mismatch. Do not
attempt this convergence during D5 caller-zero physical-input work; it is a
post-cutover refactor gate.
