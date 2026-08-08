# AST JSON transport

This module owns descriptive AST transport only. It does not resolve names,
issue source capabilities, select providers, or reconstruct missing source
order.

```text
joinir_compat
  = legacy payload shape; imports are CompatibilityOnly

roundtrip facade
  = root schema dispatch and public encode/decode entry

roundtrip_decoder
  = one recursive DecodeMode with nested-failure receipt

box_inventory_v2
  = ordered method row encoding/decoding and provenance validation
```

The v2 root marker selects strict recursive v2 mode exactly once. A malformed
nested v2 node rejects the complete root; it must not fall back to v1 or be
silently dropped by `filter_map`. Legacy or unmarked JSON is compatibility
mode and can only produce `CompatibilityOnly(LegacyJsonV1)` method rows.

The AST transport transaction validates all Box method rows before committing
one ordered inventory. Resolver-grade source truth remains a later parser
seal, and Builder/MIR consumers are outside this module.
