# Callable parameter source

This directory is the parser-side physical owner for callable parameter source
syntax. It exists so the transfer-source catalog can grow without adding a
second authority to the near-limit `source_seal.rs` or mixing parameter
semantics into the general resolver handoff.

Current R0 boundary:

- `model.rs` owns the existing AST-free neutral name/type row;
- `issuer.rs` owns the existing `ParamDecl` fallback projection;
- `tests.rs` fixes typed, untyped, and legacy-name ordering behavior;
- no transfer classification, parser provenance, declaration identity, Home
  demand, ABI, Recipe key, MIR value, fallback, or production activation.

The next I0 may add a complete sibling parameter-source catalog here. It must
issue exact parameter ordinals and typed `Ordinary` rows from parser authority;
it must not infer `Ordinary` from missing metadata or extend
`ParserBoxSourceSealV1` as a storage shortcut.
