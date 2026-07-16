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
A-prime no-result mutation of caller-owned register storage
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
MapBox-returning storage mutation helpers
register/snapshot storage replacement after birth
raw register or snapshot MapBox accessors
```

Every file imports its direct dependencies. No file may rely on test import
order. Every source/check file remains below 800 lines.

Register storage keeps only exact scalar payloads. The sealed function view
owns the corresponding `i64` / `i1` kind facts. Storage helpers mutate an
ordinary formal in place, return no MapBox, and never acquire field ownership.
