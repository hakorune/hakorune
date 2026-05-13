# hako-alloc-recommit-failfast-proof

Purpose: M201 proof for the recommit fail-fast entry.

The app observes an unmarked OSVM-backed heap page as requiring no recommit,
then uses the M199 guard to decommit and mark the page. The M201 recommit entry
then reports the page as blocked with `requires_recommit=1`, while unknown page
ids return missing-page facts.

This proof is pure-first EXE focused because the setup uses the existing OSVM
decommit leaf path through M199. The M201 owner itself intentionally keeps
actual recommit, page-source calls, unreserve, OS release, marker clearing, and
heap/page mutation closed.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_recommit_failfast_guard.sh
```
