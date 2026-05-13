# hako-alloc-purge-state-aware-duplicate-guard-proof

Purpose: M199 proof for purge state-aware duplicate decommit prevention.

The app creates an OSVM-backed heap page, proves a live page attempt stays
source-inactive, then releases the page-local block and executes one eligible
decommit through the M199 guard. A repeated attempt for the same page is
blocked by the marker before the page-source adapter can run again.

This proof is pure-first EXE focused because it uses the OSVM leaf execution
path. It keeps heap/page mutation outside the guard owner and keeps unreserve /
OS release closed.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_purge_state_aware_duplicate_guard.sh
```
