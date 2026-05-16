---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-052B reclaim execution intent marker and unsupported preflight.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-owner-transfer-contract-ssot.md
  - docs/development/current/main/design/declared-uses-capability-plan-mapping-ssot.md
  - docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-539-MIMAP-052B-RECLAIM-EXECUTION-INTENT-MARKER-PREFLIGHT.md
---

# Reclaim Execution Preflight SSOT

## Decision

Reclaim execution intent must be explicit metadata. It must not be inferred
from generic substrate capabilities such as `hako.atomic` or `hako.osvm`.

`MIMAP-052B` adds the metadata-only marker:

| Source declaration | MIR allow id | State |
| --- | --- | --- |
| `uses alloc_reclaim` | `hako.alloc.reclaim` | metadata only; execution unsupported |

The marker remains `verified=false` and `source=source_uses`. It does not
prove atomic ownership claim, owner mutation, remote-free drain, page-source
calls, or backend lowering.

## Preflight Contract

Default route preflight allows the metadata marker, matching other metadata
transport rows.

When a guard/build lane explicitly asks for reclaim execution support, the
pure-first route preflight rejects any main-reachable function with
`metadata.capability_plans allow=[hako.alloc.reclaim]`:

```bash
tools/checks/pure_first_route_preflight.py \
  --reject-unsupported-reclaim-execution \
  app.mir.json
```

Stable failure contract:

```text
reason:
  reclaim_execution_route_unsupported

layer:
  route-preflight

owner:
  capability_plans

contract:
  metadata.capability_plans[hako.alloc.reclaim]
```

## Stop Lines

`MIMAP-052B` must not add:

```text
reclaim execution
owner mutation
atomic ownership claim
remote-free drain
thread scheduling
page-source call
OSVM unreserve / release
backend .inc app/name matcher
provider activation
hooks
host allocator replacement
cap block syntax
```

## Guard Contract

The guard must prove:

```text
declared uses alloc_reclaim emits hako.alloc.reclaim CapabilityPlan
default preflight accepts metadata-only hako.alloc.reclaim
--reject-unsupported-reclaim-execution fails with reclaim_execution_route_unsupported
unreachable reclaim metadata does not fail pure-first route preflight
generic hako.atomic / hako.osvm capabilities are not treated as reclaim execution
runtime/backend route tables are not widened by this row
```
