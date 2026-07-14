---
Status: Closed
Date: 2026-07-14
Decision: passive Ownership SSA transport only; production activation remains zero
Related:
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - mirbuilder-canonical-ownership-production-profile-2026-07-14.md
  - mirbuilder-ssa-rc0-owned-alias-materialization-design-stop-2026-07-14.md
---

# SSA-RC-A0 Passive Ownership Vocabulary

## Outcome

`CopyOwned { dst, src }` and `DestroyOwned { value }` are now passive MIR
vocabulary. They have conservative `WRITE` effects and complete structural
transport, but no executor or canonical production caller.

```text
Copy:
  ownership-neutral

CopyOwned:
  fresh destination identity
  independent-owner semantics reserved for A1a/V0

DestroyOwned:
  singleton consuming operand
  legacy ReleaseStrong alias-group behavior is not reused
```

## Closed boundaries

- printer, stable tag, destination/use query, remapper, CFG-use rewrite;
- MIR JSON schema, emitter, v0 parser, v1 bridge, and round-trip;
- representation fact propagation from source to `CopyOwned` destination;
- backend diet classification as MIR-JSON transport-only;
- opcode ledger `43 kept / 16 removed / 59 total`;
- direct JSON fail-fast witness for every ownership operand.

The direct JSON witness is deliberately narrow:

```text
value type:
  exact MirType::Box with non-empty box_type

storage class:
  exact box_ref

CopyOwned:
  source and destination types equal
```

Missing metadata, primitive/opaque storage, mismatched Box types, and
`dst == src` are rejected at ingress. Ordinary JSON without ownership opcodes
does not enter this verifier and preserves prior behavior.

## Verification

```text
ownership transport focused fixtures = 6/6
backend opcode contract fixtures = 21/21
resolved region-flow authority guard = green
cargo check = green
cargo build --release --bin hakorune = green
dev_gate quick = 66/66 green
all new or modified source/check files below 800 lines
```

## Non-claims

```text
production CopyOwned callers = 0
production DestroyOwned callers = 0
Rust interpreter ownership handlers = 0
LLVM/object ownership handlers = 0
Ownership SSA verification = not implemented
Owned Phi/Return forwarding = not implemented
canonical grammar/behavior delta = 0
```

Next is SSA-RC-A1a: implement the temporary Rust semantic-oracle handlers
without activating a canonical producer.
