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
- `seal/function_context.hako` is the typed internal function handoff.
- `seal/instruction_contract.hako` owns exact opcode fields and field kinds.
- `seal/instruction_facts.hako` owns passive instruction fact products.
- `seal/instruction_inventory.hako` owns block traversal and fact aggregation.
- `seal/cfg.hako`, `value_inventory.hako`, `scalar_profile.hako`, and
  `ownership.hako` consume those facts without reparsing instruction shape.
- `view/publication.hako` is the only Verified view constructor owner and is
  called only after every function and whole-document check succeeds.
- Other `view/` files expose exact admitted fields and attachment methods only.
- Published views never expose a raw node or a generic field/metadata lookup.
- No decoded instruction enum, second CFG, V0/compact translation, AST input,
  runtime handler discovery, or Rust fallback is allowed.
- Production execution callers remain zero through HMI-S0.
- Every source/check file stays below 800 lines.

`T0-L0` and `T0-S0` are closed. Current order:
`T0-P0 -> V0 -> I0 -> P0 parity`.
