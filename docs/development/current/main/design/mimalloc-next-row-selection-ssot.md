---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: D207 selection of the next single mimalloc implementation row after D206.
Related:
  - docs/development/current/main/design/mimalloc-port-remaining-inventory-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/design/post-m213-next-lane-selection-ssot.md
---

# Mimalloc Next Row Selection SSOT

## Decision

D207 selects exactly one next implementation row:

```text
M214 allocator options/defaults inventory
```

This row is selected because the main allocator algorithm lane, packed metadata
lane, stats/lifecycle/purge/reclaim inventory lane, and provider stop line are
all stable after D206. The remaining safe mimalloc-shaped surface is read-only
option/default/init vocabulary.

## Selected row contract

| Field | Value |
| --- | --- |
| Row token | `M214 allocator options/defaults inventory` |
| Behavior class | read-only inventory |
| Primary owner | `lang/src/hako_alloc/memory/options_inventory_box.hako` |
| Suggested proof app | `apps/hako-alloc-options-inventory-proof/` |
| Suggested guard | `tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh` |
| Current pointer after D207 | `M214 allocator options/defaults inventory` |

## M214 scope

M214 may add a `.hako` owner that names allocator option/default/init facts and
returns stable structured reports.

Allowed facts:

```text
options inventory exists
known option names are classified
known defaults are observable
option source is static/read-only
allocation behavior is unchanged
provider/hook/replacement are inactive
reclaim execution is inactive
```

Recommended report fields:

```text
options_inventory_present
known_option_count
mutable_options_enabled
env_toggles_added
would_change_allocation_policy
would_select_provider
would_install_hook
would_replace_process_allocator
would_execute_reclaim
```

## Stop lines

M214 must not add:

```text
mutable runtime options
environment option toggles
option parsing from process env/files
allocation policy changes
size-class policy changes
page queue behavior changes
remote-free behavior changes
purge/decommit/recommit behavior changes
thread scheduling
atomic ownership claim
reclaim execution
unreserve
OS release
provider activation
hooks
process allocator replacement
backend app/name matchers
language syntax implementation
selfhost parser/mirbuilder changes
```

## Guard expectations

The M214 guard should prove all of these:

```text
VM proof exits 0
pure-first EXE proof exits 0 if the existing backend route supports the shape
report says mutable_options_enabled = 0
report says env_toggles_added = 0
report says would_change_allocation_policy = 0
report says would_select_provider = 0
report says would_install_hook = 0
report says would_replace_process_allocator = 0
report says would_execute_reclaim = 0
no provider/hook/replacement env vars are introduced
```

Unsupported backend behavior must fail fast or stay out of the M214 acceptance
surface. Silent fallback is not allowed.

## Why not the other candidates

| Candidate | Decision |
| --- | --- |
| thread heap owner-token inventory | park until options/defaults vocabulary is named |
| secure entropy inventory | park until capability design can avoid cryptographic overclaim |
| page-source release/unreserve proposal | park; touches OS lifetime and must stay explicit |
| reclaim execution proposal | reject for now; requires threads, atomics, ownership transfer, and remote-free drain |
| language minimal-surface row | park until a selected allocator row needs it |
| selfhost restart inventory | park until allocator/language seam is stable or a lane switch card selects it |

## Completion condition

D207 is complete when current state points at M214 and no allocator behavior has
changed.
