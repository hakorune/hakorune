---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: USES-002A declared `uses` metadata to MIR CapabilityPlan mapping.
Related:
  - docs/development/current/main/design/uses-metadata-capsule-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-537-USES-002A-DECLARED-USES-CAPABILITY-PLAN-MAPPING.md
  - src/mir/effect_capability_plan.rs
---

# Declared Uses Capability Plan Mapping SSOT

## Decision

`USES-002A` maps already-parsed method-level `uses` metadata into canonical MIR
CapabilityPlan ids:

| Source declaration | MIR allow id | State |
| --- | --- | --- |
| `uses osvm` | `hako.osvm` | metadata only |
| `uses atomic` | `hako.atomic` | metadata only |
| `uses rawbuf` | `hako.rawbuf` | metadata only |
| `uses random` | `hako.random` | metadata only; execution unsupported |

The row keeps `verified=false`. It does not prove backend support, lower any
route, or enable execution.

## Owner

```text
src/mir/effect_capability_plan.rs
```

The owner must be the only place that maps declared `uses` spellings to
canonical `hako.*` CapabilityPlan ids.

## Stop Lines

`USES-002A` must not add:

```text
cap block syntax
source-level tls widening
random / entropy execution
backend route lowering
helper-name capability inference
reclaim execution
atomic ownership claim
remote-free drain
thread scheduling
provider activation
hooks
host allocator replacement
```

## Guard Contract

The guard must prove:

```text
declared uses osvm/atomic/rawbuf/random emit sorted canonical allow ids
CapabilityPlan source remains source_uses
CapabilityPlan verified remains false
unknown declared uses are ignored
runtime/backend route tables are not widened by this row
```
