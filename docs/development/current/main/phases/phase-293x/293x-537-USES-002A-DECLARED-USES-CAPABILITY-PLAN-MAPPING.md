# 293x-537 USES-002A Declared Uses Capability Plan Mapping

Status: landed
Date: 2026-05-17

## Decision

`USES-002A` is the Hakorune core row selected by `MIMAP-051B`.

It promotes the already-parsed Stage0 `uses osvm` / `uses atomic` /
`uses rawbuf` metadata into MIR `CapabilityPlan` allow entries:

```text
uses osvm   -> hako.osvm
uses atomic -> hako.atomic
uses rawbuf -> hako.rawbuf
```

`uses random -> hako.random` stays live from `RANDOM-CAP-001`.

## Scope

- Add a small Stage1/MIR capability mapping contract for declared `uses`
  metadata.
- Keep `verified=false`; this row does not prove backend support.
- Add tests and a focused guard for canonical mappings and stop lines.
- Update the language/minimal and mimalloc task docs.

## Stop Lines

- No `cap` block syntax.
- No source-level `tls` widening.
- No random/entropy execution.
- No backend route lowering or helper-name inference.
- No reclaim execution, atomic ownership claim, remote-free drain, or thread
  scheduling.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No broad capability policy solver.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `USES-002A.1` | Write declared uses capability mapping SSOT. | canonical capability ids are fixed. | no backend gate |
| `USES-002A.2` | Extend MIR capability plan mapping. | osvm/atomic/rawbuf/random declared uses emit sorted allow ids. | verified=false |
| `USES-002A.3` | Add tests and focused guard. | guard proves mapping and stop lines. | no route lowering |
| `USES-002A.4` | Close out current pointers and select follow-up. | current pointer guard passes. | no bundle |

## Required Evidence

```text
cargo test -q --lib source_declared_uses_emit_canonical_capability_plan_ids
bash tools/checks/k2_wide_uses_capability_plan_mapping_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Return Condition

This row closes when declared low-level `uses` metadata has a canonical MIR
CapabilityPlan mapping without enabling execution.

## Implementation Result

`USES-002A` adds:

```text
SSOT:
  docs/development/current/main/design/declared-uses-capability-plan-mapping-ssot.md

owner:
  src/mir/effect_capability_plan.rs

guard:
  tools/checks/k2_wide_uses_capability_plan_mapping_guard.sh
```

Declared source `uses` metadata now maps to canonical MIR ids:

```text
uses osvm   -> hako.osvm
uses atomic -> hako.atomic
uses rawbuf -> hako.rawbuf
uses random -> hako.random
```

The row keeps:

```text
verified=false
source=source_uses
backend execution inactive
```

## Evidence

```text
cargo test -q --lib source_declared_uses_emit_canonical_capability_plan_ids
cargo test -q --lib mir_transports_low_level_declared_uses_as_capability_plan_ids
bash tools/checks/k2_wide_uses_capability_plan_mapping_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`USES-002A` selects `MIMAP-052A`.

```text
row:
  MIMAP-052A reclaim execution preflight proposal

classification:
  allocator planning / preflight row

why now:
  reclaim owner-transfer preconditions and declared capability ids are now
  visible. The next row should decide the exact fail-fast/preflight gate before
  any reclaim execution is opened.

stop lines:
  no reclaim execution
  no atomic ownership claim
  no remote-free drain
  no thread scheduling
```

Closeout:

```text
current blocker moves to MIMAP-052A.
```
