# 293x-515 MIMAP-NEXT-BEHAVIOR-SELECTION-001

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-NEXT-BEHAVIOR-SELECTION-001` is a planning-only BoxShape row. It chooses
the next single implementation row after the recent compiler/docs cleanup
sequence.

## Scope

- Review the durable row order in
  `docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md`.
- Review the allocator-first granularity policy in
  `docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md`.
- Decide whether the next row is:
  - one allocator behavior row,
  - one compiler acceptance sidecar,
  - or one BoxShape cleanup row.
- Produce the selected row card with explicit owner, proof app/guard or docs
  evidence, and stop lines.

## Stop Lines

- Do not implement allocator behavior in this selection row.
- Do not implement broad language/concurrency features speculatively.
- Do not combine BoxShape cleanup with BoxCount acceptance expansion.
- Keep allocator-provider activation, host allocator replacement, hooks, and
  `#[global_allocator]` inactive.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `SEL.1` | Read current taskboard and allocator-first granularity SSOT. | next candidates are concrete. | no source edits |
| `SEL.2` | Classify candidates as allocator behavior, compiler acceptance, or BoxShape cleanup. | one selected row only. | no mixed row |
| `SEL.3` | Create the selected row card and update current pointers. | current guard passes. | no implementation |
| `SEL.4` | Verify and close out. | required evidence is green. | no code edits |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row closes when the next implementation row is selected with a clear owner,
proof/guard path, and stop lines.

## Selection Result

`MIMAP-NEXT-BEHAVIOR-SELECTION-001` selects `MIMAP-042A`.

Selected row:

```text
row:
  MIMAP-042A OSVM-backed fast-path bounded purge route
owner:
  lang/src/hako_alloc/memory/osvm_fast_path_purge_route_box.hako
primary proof app:
  apps/hako-alloc-osvm-fast-path-purge-route-proof/
primary guard:
  tools/checks/k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh
```

Rationale:

- `MIMAP-041A` made the bounded purge/decommit scheduler report boundary
  record-shaped, and the later cleanup chain removed route/spec drift before
  allocator behavior resumed.
- The next narrow allocator behavior is not provider activation or host
  replacement. It is a small `.hako` route that composes the already-live
  OSVM-backed fast-path heap, the M199 state-aware decommit guard, and the M212
  bounded scheduler.
- `MIR-ROW-D` stays parked unless `MIMAP-042A` exposes a real dense queue
  field-read acceptance blocker.

Stop lines carried into `MIMAP-042A`:

```text
no direct page-source/OSVM calls in the new route owner
no unreserve / recommit / OS release
no remote-free / TLS / atomic expansion
no provider activation, hooks, host allocator replacement, or #[global_allocator]
no broad language/concurrency work
split a compiler acceptance sidecar if pure-first route preflight fails
```

Closeout:

```text
current blocker moves to MIMAP-042A OSVM-backed fast-path bounded purge route.
```
