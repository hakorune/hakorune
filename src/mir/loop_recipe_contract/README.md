# Portable Loop Recipe Contract

This directory owns the Builder-free, selfhost-portable semantic contract for
Loop lowering.

## Authority

- `LoopRecipeArtifactV1` owns schema version, source provenance, producer route
  trace, and one `LoopRecipeV1`.
- `LoopRecipeV1` owns one closed recursive control algebra represented on the
  wire as ordered arenas with recipe-local keys.
- Every Loop node owns a typed source path. External/pre-loop values are named
  explicitly by `inputs`; every other value has exactly one operation result.
- A carrier entry must be available before its owning Loop is entered. Backedge
  and exit joins remain the later JoinSig elaboration authority.
- `LoopRecipeVerifierV1` consumes only `LoopRecipeV1`. It cannot select or retry
  a route and cannot inspect the producer trace.

## Forbidden dependencies

This subtree must not import AST nodes, `MirBuilder`, `CorePlan`, physical
`ValueId`/`BasicBlockId`, `Frag`, route composers, callbacks, retry, or legacy
mutation-family policy.

The control tree is the sole source of connectivity. Logical CFG/JoinSig and
physical MIR are later elaborations; they are not duplicated in this wire
contract.

Arena rows and recursive traversal both use canonical preorder. Source paths
use closed semantic steps (`function_body`, `body`, `loop_body_root`, and
`loop_body`) rather than untyped integer ordinals.

## Extension rule

Start with the Accum-ready operation vocabulary. Add one typed operation only
when a route migration supplies a counterexample and fixtures. Never add opaque
AST/statement payloads or legacy-emitter escape hatches.
