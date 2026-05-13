# hako-alloc-decommit-recommit-reuse-exe-hardening-proof

Purpose: M210 proof for decommit/recommit/reuse EXE hardening.

The app composes the existing M195-M209 allocator lifecycle path in one proof:
bounded decommit, duplicate decommit blocking, bounded recommit, marker
generation transition, page-local reactivation, lifecycle observation, reuse
priority, and lifecycle stats snapshots.

It intentionally avoids:

- new allocator owners
- unreserve or OS release behavior
- provider activation, hooks, or process allocator replacement
- backend-specific shortcuts or `.inc` matchers
- VM-only completion claims

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_decommit_recommit_reuse_exe_hardening_guard.sh
```
