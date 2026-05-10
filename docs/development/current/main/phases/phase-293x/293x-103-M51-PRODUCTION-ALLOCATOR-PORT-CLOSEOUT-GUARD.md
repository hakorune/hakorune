---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M51 production allocator port closeout guard
---

# 293x-103 M51 Production Allocator Port Closeout Guard

## Decision

`M51 production allocator port closeout guard` is live-narrow.

M51 closes the first production allocator port slice by inventorying M46-M50:

```text
M46 facade boundary
M47 local page policy proof
M48 remote-free policy proof
M49 OSVM page-source proof
M50 production-facade stress parity
```

It adds no app fixture, route row, NyRT export, `.inc` lowering behavior, pointer
`fetch_add`, OSVM unreserve/release row, native pointer attr, or allocator
replacement hook.

## Owned

- coverage guard:
  `tools/checks/k2_wide_production_allocator_port_closeout_guard.sh`
- docs/taskboard/current pointers for M51.

## Not Owned

- Process allocator replacement.
- Allocator hook design.
- New substrate capability rows.
- New native/LLVM attrs.
- Removing lower-seam regression apps.

## Gate

```bash
bash tools/checks/k2_wide_production_allocator_port_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh
bash tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- M46-M50 apps, cards, guards, docs index, and dev_gate entries are present;
- `hako_alloc` exports the production facade and policy boxes;
- `.inc` does not branch on production allocator app/facade/policy names;
- pointer `fetch_add`, OSVM unreserve/release, and allocator replacement rows
  remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_production_allocator_port_closeout_guard.sh` passes.
