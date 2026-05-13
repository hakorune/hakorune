# hako-alloc-decommitted-page-reuse-precondition-proof

Purpose: M200 proof for decommitted page reuse precondition.

The app observes an unmarked OSVM-backed heap page as reusable, then uses the
M199 guard to decommit and mark the page. The same page is then classified as
blocked with `requires_recommit=1`, while unknown page ids return missing-page
facts.

This proof is pure-first EXE focused because it uses the OSVM leaf execution
path. It intentionally keeps recommit, unreserve, OS release, and heap/page
mutation outside the precondition owner.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_decommitted_page_reuse_precondition_guard.sh
```
