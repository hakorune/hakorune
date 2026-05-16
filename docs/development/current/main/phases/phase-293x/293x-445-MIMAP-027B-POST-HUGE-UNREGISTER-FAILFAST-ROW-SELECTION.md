# 293x-445 MIMAP-027B Post-Huge-Unregister-Failfast Row Selection

Status: ready
Date: 2026-05-16

## Decision

`MIMAP-027B` is a planning-only row. It must select exactly one next allocator
behavior row after the facade huge-release path has both:

```text
MIMAP-026A M181 success unregister
MIMAP-027A double/stale post-unregister reject diagnostics
```

The selection must keep provider activation, host allocator replacement, hooks,
and `#[global_allocator]` inactive unless the chosen row explicitly documents a
future narrow ladder step.

## Scope

- Review the post-MIMAP-027A huge unregister success + reject state.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Expected Output

This card should close with one selected next row and the following fields
filled in:

```text
row:
owner:
proof app:
guard:
reused owners:
primary proof:
stop lines:
```

It should not land code. If the chosen row needs a new capability or verifier
contract first, `MIMAP-027B` should select that contract row explicitly instead
of silently widening the allocator owner.

## Candidate Questions

- Is the next durable row a narrow OS page return / unreserve planning step, or
  should the facade first expose a verifier/CorePlan no-fallback contract for
  released huge ownership?
- Does any later row need to promote M181 page-map unregister/fail-fast facts
  out of metadata-only observers before touching OSVM release?
- Which proof app is the smallest observable contract after M181 success and
  reject diagnostics are both green?

## Selection Rubric

Prefer the next row in this order:

1. Choose a scalar `.hako` owner if it can prove the next allocator invariant
   without a new backend capability.
2. Choose a CorePlan / verifier contract row only if silent fallback or
   backend capability gating becomes the smallest blocker.
3. Do not choose OS page return / unreserve until the facade has a narrow
   page-source-backed huge allocation contract. The current huge route proves
   metadata/page-map behavior, but it does not yet attach a page-source backing
   identity suitable for later decommit.
4. Do not choose provider activation, host allocator replacement, hooks, or
   `#[global_allocator]` from this row.

## Draft Forward Rows

These are planning candidates for the sequence after `MIMAP-027B`; only the row
selected by `MIMAP-027B` becomes current.

| Row | Candidate purpose | Likely owner | Proof / guard | Stop lines |
| --- | --- | --- | --- | --- |
| `MIMAP-028A` | facade huge page-source backing route | `object_lifecycle_facade_huge_page_source_box.hako` | `apps/mimalloc-facade-huge-page-source-proof/main.hako` / `tools/checks/k2_wide_mimalloc_facade_huge_page_source_exe_guard.sh` | no release/unregister/decommit/unreserve |
| `MIMAP-028B` | post-backed-huge row selection | docs-only | pointer guard / quick | no implementation |
| `MIMAP-029A` | facade huge decommit-after-unregister success route | `object_lifecycle_facade_huge_decommit_box.hako` | `apps/mimalloc-facade-huge-decommit-proof/main.hako` / `tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh` | no unreserve, no recommit, no provider |
| `MIMAP-029B` | post-huge-decommit row selection | docs-only | pointer guard / quick | no implementation |
| `MIMAP-030A` | facade huge decommit fail-fast diagnostics | `object_lifecycle_facade_huge_decommit_failfast_box.hako` | `apps/mimalloc-facade-huge-decommit-failfast-proof/main.hako` / `tools/checks/k2_wide_mimalloc_facade_huge_decommit_failfast_exe_guard.sh` | no unreserve/recommit/provider |
| `MIMAP-030B` | post-huge-decommit-failfast row selection | docs-only | pointer guard / quick | no implementation |
| `MIMAP-031A` | OSVM unreserve capability inventory / planning row | docs/design only unless explicitly selected | pointer guard / quick | do not add `hako_osvm_unreserve*` in this planning row |

`MIMAP-028A` is the conservative default candidate: it attaches huge allocation
to an existing page-source-backed identity first. This keeps the next
implementation row small and avoids mixing huge allocation backing, release
unregister, decommit, unreserve, and provider activation.

## Stop Lines

- Do not implement allocator behavior in this planning row.
- Do not add OSVM release/unreserve/decommit, small release/free, realloc,
  alignment, purge/reclaim, remote-free, TLS, atomic, provider hook, host
  allocator replacement, or backend `.inc` matcher shortcut.
- Do not widen MIMAP-027A while selecting the next row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `027B.1` | Review MIMAP-027A landed evidence and nearby allocator backlog. | One next row is selected with owner/proof/guard names. | no implementation |
| `027B.2` | Update taskboard and granularity SSOT. | Current pointers move to the selected behavior row. | no provider activation |
| `027B.3` | Run pointer/quick gates. | Current docs are internally consistent. | no behavior widening |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
