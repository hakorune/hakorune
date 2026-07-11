# 293x-380 MIMAP-020A OSVM/Page-Source Capability Pilot

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-020A` is the next allocator row after `MIMAP-019A`. It may start a
capability-gated OSVM/page-source pilot only after the in-memory facade route
and read-only purge/reclaim policy route are stable.

## Scope

- Select one existing page-source / OSVM owner that already has an executable
  guard.
- Add or extend one narrow proof app and one guard only if the selected row
  needs a mimalloc-facing acceptance seam.
- Keep unsupported capability paths fail-fast; do not silently fall back to an
  in-memory page model.

## Planned Task Order

MIMAP-020A is intentionally an adoption/closeout row first. Do not start by
adding a new OSVM abstraction.

| Step | Task | Output | Stop line |
| --- | --- | --- | --- |
| `020A.1` | Adopt the existing M49 page-source owner as the selected capability pilot. | `HakoAllocPageSourcePolicy` and `HakoAllocProductionFacade.pageSource*` are named as the MIMAP-020A owners. | no new `.hako` owner |
| `020A.2` | Re-run the existing executable proof. | `bash tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh` proves reserve/commit/decommit through MIR-owned OSVM extern routes. | no new guard unless the existing proof misses a MIMAP-specific invariant |
| `020A.3` | Record closeout and advance the current pointer. | MIMAP-020A card lands with selected owner, evidence, and next decision point. | no provider/hook/global allocator activation |

Selected first owner:

```text
lang/src/hako_alloc/memory/page_source_policy_box.hako
  HakoAllocPageSourcePolicy.reservePage(bytes)
  HakoAllocPageSourcePolicy.commitPage(base, bytes)
  HakoAllocPageSourcePolicy.decommitPage(base, bytes)

lang/src/hako_alloc/memory/allocator_facade_box.hako
  HakoAllocProductionFacade.pageSourceReserve(bytes)
  HakoAllocProductionFacade.pageSourceCommit(base, bytes)
  HakoAllocProductionFacade.pageSourceDecommit(base, bytes)
```

Existing proof/guard to adopt:

```text
apps/hako-alloc-page-source-policy-proof/main.hako
tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
```

## Closeout

`MIMAP-020A` closes as an existing-owner adoption row. The selected M49
page-source owner already proves the reserve/commit/decommit route through the
facade-owned page-source methods, so no new `.hako` owner, proof app, guard, or
OSVM abstraction is needed for this card.

Evidence:

```text
bash tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
# [m49-mir-json] ok
# [k2-wide-hako-alloc-page-source-policy-exe] ok
```

No split trigger fired:

- `MIMAP-020B`: not needed; the existing M49 proof is sufficient for the
  selected mimalloc-facing page-source facade seam.
- `USES-002A`: not needed for this closeout; method-level `uses osvm`
  capability metadata can remain a future verifier contract row.
- provider ladder reopen: not triggered; provider hooks, host allocator
  replacement, and `#[global_allocator]` stay inactive.

## Split Rules

Only split MIMAP-020A if one of these blockers appears:

| Split | Trigger | Return condition |
| --- | --- | --- |
| `MIMAP-020B` | The existing M49 proof is green but a mimalloc-facing facade acceptance seam is still missing. | One narrow proof app/guard proves that seam without adding provider activation. |
| `USES-002A` | Method-level `uses osvm` capability metadata must become verifier-active before the row can safely close. | Unsupported capability fails fast and supported OSVM route remains guarded. |
| provider ladder reopen | The task requires provider selection, hook install, host allocator replacement, or `#[global_allocator]`. | Stop MIMAP work and explicitly reopen the optional allocator-provider ladder. |

If none of these triggers fire, MIMAP-020A should close as an existing-owner
adoption row rather than adding code.

## Post-020 Decision Point

After MIMAP-020A lands, do not implicitly continue into provider activation.
The next work item must be selected explicitly from the current SSOTs:

- continue `.hako` / `hako_alloc` allocator behavior work with a new
  MIMAP card;
- open a compiler/language sidecar only if the allocator row is blocked;
- reopen the optional provider ladder only by explicit instruction.

## Stop Lines

- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No production allocator activation.
- No atomic/TLS/remote-free expansion.
- No page-map lookup unless the selected owner already owns that seam.
- No broad OSVM API surface beyond the selected capability row.
- No backend matcher shortcut; supported routes must remain MIR-owned route
  metadata / existing CoreMethodContract paths.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
