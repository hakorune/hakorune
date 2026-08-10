# Normal callable Program source

This parser boundary retains the complete callable anchor set after the total
parser postpass and verifies one whole-file transform before normal MIR
compilation.

Authority:

- the parser chooses `SourceBacked` or a typed compatibility cohort;
- a source-backed transform consumes the initial product exactly once;
- final callable slots and declarations must remain an exact set; and
- the verified final product owns the transformed Program and retained opaque
  anchors atomically.

Non-authority:

- names, arity, spans, ordinals, and AST addresses never recreate identity;
- a failed exact transform never retries through compatibility;
- this module does not resolve callables or own Builder, Recipe, Home, ABI, or
  physical lowering; and
- macro transform policy remains owned by `src/macro/`.

The first cohort accepts exact callable-preserving transforms. Added, removed,
reordered, or changed callable declarations reject. Broader transform receipts
must be added as explicit source transactions rather than AST reconstruction.
