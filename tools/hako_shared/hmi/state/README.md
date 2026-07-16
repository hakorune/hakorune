# HMI disconnected scalar state

This directory owns the opcode-neutral state substrate used by the future HMI
scalar handlers.

Allowed:

```text
tagged i64/i1 values
typed state errors
explicit outcomes and predecessors
harness-only step budget
typed scalar registers and entry snapshots
bounded-view anchored execution sessions
```

Forbidden:

```text
raw JsonNode access
object_get / array_get / root_for_seal
opcode names or dispatch
decoded MIR/CFG products
VMValue / MirModule / BoxRef
handler registries
fallback / retry / V0 conversion
production callers
```

Every file imports its direct dependencies. No file may rely on test import
order. Every source/check file remains below 800 lines.
