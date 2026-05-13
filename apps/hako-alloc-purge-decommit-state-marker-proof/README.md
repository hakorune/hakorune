# hako-alloc-purge-decommit-state-marker-proof

Purpose: M198 proof for purge decommit state marker ownership.

The app creates an OSVM-backed heap page, proves that a live/ineligible
decommit report is not marked, then releases the page-local block, executes
decommit through the M197 integration, and records the successful page id
through `HakoAllocPurgeDecommitStateMarker`.

This proof is pure-first EXE focused because it uses the OSVM leaf execution
path. It intentionally keeps heap/page mutation outside the marker owner and
keeps unreserve / OS release closed.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_purge_decommit_state_marker_guard.sh
```
