---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M52 allocator replacement hook boundary
---

# 293x-104 M52 Allocator Replacement Hook Boundary

## Decision

`M52 allocator replacement hook boundary` is live-docs.

M52 adds the allocator replacement hook SSOT and guard before any implementation.
It does not activate process allocator replacement.

Design owner:

```text
docs/development/current/main/design/allocator-replacement-hook-boundary-ssot.md
```

## Owned

- Hook boundary SSOT.
- Coverage guard:
  `tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh`
- docs/taskboard/current pointers for M52.

## Not Owned

- Process allocator replacement.
- `#[global_allocator]`.
- Runtime hook install/uninstall implementation.
- `.inc` allocator name matching.
- Hidden allocator hook environment variables.
- Pointer `fetch_add`.
- OSVM unreserve/release.
- Native pointer attr widening.

## Gate

```bash
bash tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh
bash tools/checks/k2_wide_production_allocator_port_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- the hook boundary SSOT, M52 card, taskboard, phase README, current state,
  docs index, dev_gate, and `hako_alloc` README are synchronized;
- allocator replacement hook implementation symbols have not appeared in code;
- no app/facade/policy hook matcher is present in `.inc`;
- pointer `fetch_add`, OSVM unreserve/release, and allocator hook env toggles
  remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_allocator_replacement_hook_boundary_guard.sh` passes.
