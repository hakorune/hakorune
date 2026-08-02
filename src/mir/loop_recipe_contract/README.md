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
- External/pre-loop values are named explicitly by `inputs`; every other value
  has exactly one operation result.
- A carrier entry must be available before its owning Loop is entered. Backedge
  and exit joins remain the later JoinSig elaboration authority.
- `LoopRecipeVerifierV1` consumes only `LoopRecipeV1`. It cannot select or retry
  a route and cannot inspect source ownership or the producer receipt.
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
