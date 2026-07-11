# 293x-472 MIMAP-039B Post-Remote-Free-Retry-Bound Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-039B` is the planning-only row after `MIMAP-039A`.

It must select exactly one next row after the remote-free retry-bound cleanup
lands.

It must not land code.

## Candidate Set

```text
candidate:
  pick a narrow allocator behavior row if cleanup no longer blocks the next
  mimalloc completeness seam
candidate:
  park object-lifecycle queue loop cleanup behind a compiler acceptance sidecar
candidate:
  continue narrow cleanup if another concrete hardcoded allocator shape remains
candidate:
  switch to a language/compiler sidecar only if the next allocator row exposes
  an acceptance blocker
```

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Selection Result

`MIMAP-039B` selects `MIR-ROW-C`.

Rationale:

- The next object-lifecycle page queue cleanup replaces fixed `page0` /
  `page1` / `page2` selection with a loop-carried selected page object.
- MIR emit and route preflight already pass for the minimized shape, but
  same-module route metadata currently publishes the returned selected page as
  `void_sentinel_i64_zero`.
- The caller then loses the typed object owner for `selected.page_id` /
  `selected.acquire(...)`, so pure-first EXE stops with
  `typed_object_field_get_plan_missing`.
- This is a compiler acceptance sidecar, not allocator behavior.

Selected row:

```text
row:
  MIR-ROW-C nullable user-box object return
owner:
  src/mir/user_box_method_route_plan
proof app:
  apps/userbox-nullable-loop-return-proof/main.hako
guard:
  tools/checks/k2_wide_userbox_nullable_loop_return_guard.sh
primary proof:
  same-module method returning null-or-selected object publishes
  return_shape=object_handle and target_result_box_name
stop lines:
  no allocator behavior change
  no object-lifecycle page queue source rewrite in this row
  no backend .inc matcher shortcut
  no app/box-name classifier
```

Closeout:

```text
current blocker moves to MIR-ROW-C nullable user-box object return.
```
