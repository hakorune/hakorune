# 293x-539 MIMAP-052B Reclaim Execution Intent Marker Preflight

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-052B` is the fail-fast gate row selected by `MIMAP-052A`.

Before any reclaim execution row can open owner mutation, atomic claim,
remote-free drain, thread scheduling, or page-source calls, reclaim execution
intent must be MIR-visible as its own capability marker:

```text
source spelling:
  uses alloc_reclaim

MIR CapabilityPlan allow id:
  hako.alloc.reclaim

source:
  source_uses

verified:
  false
```

Pure-first route preflight must reject that marker when the explicit guard
option is enabled:

```bash
tools/checks/pure_first_route_preflight.py \
  --reject-unsupported-reclaim-execution \
  app.mir.json
```

This keeps `hako.atomic` / `hako.osvm` as generic substrate capabilities and
prevents them from becoming an implicit reclaim execution signal.

## Scope

- Extend declared `uses` capability-plan mapping with `alloc_reclaim`.
- Add explicit pure-first preflight classification for unsupported reclaim
  execution intent.
- Document the reason in the pure-first diagnostics SSOT.
- Add a focused guard that proves default preflight remains metadata-only and
  the explicit option fails before backend emission.
- Select the next row after the marker/preflight lands.

## Stop Lines

- No reclaim execution.
- No owner mutation.
- No atomic claim.
- No remote-free drain.
- No thread scheduling.
- No page-source call.
- No backend `.inc` app/name matcher.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No broad capability checker or `cap` block syntax.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `052B.1` | Write reclaim execution preflight SSOT. | marker, reason, and owner are fixed. | no execution |
| `052B.2` | Map `uses alloc_reclaim` to `hako.alloc.reclaim`. | unit/integration test observes CapabilityPlan. | no broad checker |
| `052B.3` | Add explicit preflight option and reason. | synthetic MIR fails only under explicit option. | no backend lowering |
| `052B.4` | Add guard and docs index row. | guard proves stop lines. | no default gate growth |
| `052B.5` | Close current pointers and select follow-up. | current pointer guard passes. | no bundle |

## Required Evidence

```text
cargo test -q --lib source_declared_uses_emit_reclaim_execution_capability_marker
cargo test -q --lib mir_transports_alloc_reclaim_declared_uses_as_capability_plan_id
bash tools/checks/k2_wide_reclaim_execution_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation Result

`MIMAP-052B` adds:

```text
SSOT:
  docs/development/current/main/design/reclaim-execution-preflight-ssot.md

mapping owner:
  src/mir/effect_capability_plan.rs

preflight owner:
  tools/checks/pure_first_route_preflight.py

guard:
  tools/checks/k2_wide_reclaim_execution_preflight_guard.sh
```

The row maps:

```text
uses alloc_reclaim -> hako.alloc.reclaim
```

and adds the explicit preflight option:

```text
--reject-unsupported-reclaim-execution
```

Stable failure:

```text
reason=reclaim_execution_route_unsupported
owner=capability_plans
contract=metadata.capability_plans[hako.alloc.reclaim]
```

Default preflight still accepts the metadata-only marker. Generic
`hako.atomic` / `hako.osvm` capability plans do not imply reclaim execution.

## Evidence

```text
cargo test -q --lib source_declared_uses_emit_reclaim_execution_capability_marker
cargo test -q --lib mir_transports_alloc_reclaim_declared_uses_as_capability_plan_id
bash tools/checks/k2_wide_reclaim_execution_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`MIMAP-052B` selects `MIMAP-053A`.

```text
row:
  MIMAP-053A reclaim execution support row selection

classification:
  planning-only row

why now:
  reclaim execution intent is now explicit and can fail before backend
  emission. The next row must decide whether to open a first guarded execution
  slice, add an atomic-claim contract sidecar, add a remote-free drain
  fail-fast row, or keep reclaim on read-only inventory.

stop lines:
  no reclaim execution
  no owner mutation
  no atomic claim
  no remote-free drain
  no thread scheduling
  no page-source call
  no cleanup bundle
```

Closeout:

```text
current blocker moves to MIMAP-053A.
```
