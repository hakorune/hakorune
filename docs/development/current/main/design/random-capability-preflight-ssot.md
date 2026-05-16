---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: RANDOM-CAP-002 unsupported random route preflight.
Related:
  - docs/development/current/main/design/random-capability-failfast-ssot.md
  - docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-533-RANDOM-CAP-002-RANDOM-UNSUPPORTED-ROUTE-PREFLIGHT.md
---

# Random Capability Preflight SSOT

## Decision

`uses random` remains legal as metadata-only capability transport.

Unsupported random execution is rejected only when a guard/build lane
explicitly requests random execution capability checking:

```bash
tools/checks/pure_first_route_preflight.py \
  --reject-unsupported-random \
  app.mir.json
```

When the option is enabled, any main-reachable function with
`metadata.capability_plans allow=[hako.random]` fails before backend emission
unless a later row replaces this diagnostic with a supported random route.

## Reason Contract

```text
reason:
  random_capability_route_unsupported

layer:
  route-preflight

owner:
  capability_plans

contract:
  metadata.capability_plans[hako.random]
```

Default preflight behavior stays unchanged: metadata-only `uses random` is
allowed when no execution row asks for a random route.

## Stop Lines

`RANDOM-CAP-002` must not add:

```text
hako_random* / hako_entropy* extern route
OS entropy source
secure-list encode/decode behavior change
cryptographic hardening claim
provider activation
backend .inc app/name matcher
```

## Guard Contract

The guard must prove:

```text
default preflight accepts metadata-only hako.random capability plans
--reject-unsupported-random fails with random_capability_route_unsupported
non-random capability plans are unaffected
random/entropy route vocabulary remains absent from runtime/backend code
```
