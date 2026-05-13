# 293x-250 M206 Reuse Proof Closeout

Status: Complete

## Scope

M206 closes the current purge/recommit reuse proof ladder. It adds an explicit
proof app and local guard showing that the landed M199 and M205 owners compose
into a reusable page loop:

1. allocate from an OSVM-backed heap page
2. release the handle and retire the page
3. decommit through the state-aware duplicate guard
4. block duplicate decommit before another source call
5. reject direct page acquire while still decommitted
6. recommit and reactivate the page
7. select and acquire from the same heap page again
8. repeat decommit/recommit for a second marker generation

## Non-Goals

- no new allocator owner
- no new page-source reserve/decommit/recommit primitive
- no unreserve or OS release
- no provider activation, hook install, or process allocator replacement
- no public allocator API change
- no object-return allocator API parity expansion

## Acceptance

- `apps/hako-alloc-reuse-proof-closeout-proof/main.hako` exits 0 through the
  pure-first EXE route.
- `tools/checks/k2_wide_hako_alloc_reuse_proof_closeout_guard.sh` is listed in
  `docs/tools/check-scripts-index.md` and remains local-run by default.
- The guard confirms no closeout-specific owner or matcher leaked into
  `lang/src/hako_alloc/memory` or `lang/c-abi/shims`.
