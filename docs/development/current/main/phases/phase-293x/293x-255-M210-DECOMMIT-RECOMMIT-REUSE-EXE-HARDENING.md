# 293x-255 M210 Decommit/Recommit/Reuse EXE Hardening

Status: Complete

## Purpose

M210 hardens the existing M195-M209 decommit/recommit/reuse lifecycle path
under pure-first EXE. It keeps lifecycle proof completion out of VM-only or
silent fallback routes without adding allocator behavior.

## Decision

Decision: accepted.

Add a proof-only app and guard:

```text
apps/hako-alloc-decommit-recommit-reuse-exe-hardening-proof/
tools/checks/k2_wide_hako_alloc_decommit_recommit_reuse_exe_hardening_guard.sh
```

No new allocator owner is added.

## Row Contract

The proof composes:

- M195/M196 bounded decommit through page-source decommit adapter
- M199 duplicate guard and M198/M204 generation-counted marker
- M202/M203 bounded recommit through page-source recommit adapter
- M205 page-local reactivation
- M207 lifecycle observer
- M208 heap reuse priority policy
- M209 lifecycle stats observer surface

The guard requires pure-first EXE output, MIR JSON owner-function presence, and
direct OSVM route emit logs for reserve, commit, and decommit.

## Stop Lines

- Do not add allocator behavior or a new hako_alloc owner.
- Do not add unreserve, OS release, scheduler, provider activation, hooks, or
  process allocator replacement.
- Do not add backend-specific `.inc` app/name matchers.
- Do not claim VM-only completion.

## Acceptance

- The proof app exits 0 through the pure-first EXE route.
- The proof covers two decommit/recommit/reuse generations and a blocked
  decommitted-page direct acquire.
- The proof observes M207 lifecycle states, M208 reuse priority, and M209 stats
  from the same EXE path.
- The guard confirms no app/name matcher leaked into `.inc`, no forbidden
  provider/hook/OS release vocabulary appears in the proof, and the guard stays
  local-run / index-listed.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_decommit_recommit_reuse_exe_hardening_guard.sh
```
