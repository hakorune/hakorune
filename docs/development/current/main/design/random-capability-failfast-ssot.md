---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: RANDOM-CAP-001 `uses random` capability/fail-fast contract.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md
  - docs/development/current/main/design/mimalloc-secure-entropy-inventory-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-532-RANDOM-CAP-001-USES-RANDOM-CAPABILITY-DECISION.md
---

# Random Capability Fail-Fast SSOT

## Decision

`uses random` is a recognized source capability declaration.

For `RANDOM-CAP-001`, its only live compiler effect is metadata:

```text
source:
  uses random

MIR:
  metadata.capability_plans allow=[hako.random]
  source=source_uses
  verified=false

execution:
  unsupported
```

This row does not add entropy, random extern routes, runtime hooks, or secure
allocator hardening. It gives later route-preflight rows a stable capability
name to reject or verify before backend lowering.

## Boundary

| Surface | State | Owner |
| --- | --- | --- |
| `uses random` parse/transport | live metadata | parser / AST |
| `CapabilityPlan allow=[hako.random]` | live metadata | MIR effect/capability plan owner |
| backend random route | inactive | future `RANDOM-CAP-*` route row only |
| entropy source | inactive | future substrate row only |
| deterministic proof keys | proof/inventory only | `HakoAllocSecureEntropyInventory` |
| secure-list encode/decode hardening | inactive | future audited allocator row only |

## Fail-Fast Contract

Until a later row opens execution, any allocator behavior that needs runtime
randomness must fail before backend emission rather than falling back to a
deterministic key or raw helper name.

Future rows may consume this metadata as:

```text
uses random present
  backend route missing
    -> fail-fast unsupported random capability
```

They must not infer support from helper names, app names, proof-app names, or
`.inc` matchers.

## Stop Lines

`RANDOM-CAP-001` must not add:

```text
hako_random* / hako_entropy* extern route
OS entropy source
provider/hook/global allocator activation
secure-list behavior changes
cryptographic hardening claim
backend .inc app/name matcher
broad `uses osvm` / `uses atomic` / `uses rawbuf` checker expansion
```

## Guard Contract

The guard must prove:

```text
RANDOM-CAP-001 card is landed
this SSOT is accepted
source uses random maps only to metadata capability id hako.random
MIR tests cover the direct metadata path
runtime/backend code has no random/entropy extern route
secure-list policy remains out of this row
no .inc random app/box matcher exists
```
