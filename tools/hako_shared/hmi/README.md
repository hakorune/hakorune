# HMI semantic-reference substrate

Decision: accepted for HMI-S0.

This directory owns the disconnected `.hako` MIR semantic-reference reader.
Its sole future carrier is a Rust-emitted MIR JSON V1 document accepted by the
`HMI-MIR-JSON-V1-STRICT` profile.

```text
raw JSON text
  -> json_native strict policy
  -> one JsonNode tree
  -> whole-document HMI seal
  -> bounded read-only views over that same tree
```

Layer rules:

- `strict_ingress.hako` is the only JSON parser caller.
- `seal/` owns MIR admission contracts, never JSON grammar.
- `view/` exposes exact admitted fields only.
- Published views never expose a raw node or a generic field/metadata lookup.
- No decoded instruction enum, second CFG, V0/compact translation, AST input,
  runtime handler discovery, or Rust fallback is allowed.
- Production execution callers remain zero through HMI-S0.
- Every source/check file stays below 800 lines.

Current order: `T0-L0 -> T0-S0 -> T0-P0 -> V0 -> I0 -> P0 parity`.
