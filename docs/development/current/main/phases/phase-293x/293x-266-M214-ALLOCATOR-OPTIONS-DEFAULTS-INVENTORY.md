# 293x-266 M214 Allocator Options Defaults Inventory

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

M214 adds a read-only `.hako` owner for allocator options/defaults inventory.

The row names stable option/default facts without adding mutable runtime
options, environment toggles, allocation behavior changes, reclaim execution,
provider activation, hooks, or process allocator replacement.

## Landed files

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/hako-alloc-options-inventory-ssot.md` | M214 options inventory SSOT. |
| `lang/src/hako_alloc/memory/options_inventory_box.hako` | Read-only options/defaults inventory owner. |
| `apps/hako-alloc-options-inventory-proof/` | VM / pure-first EXE proof app for M214. |
| `tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh` | Local-run guard for the M214 proof and stop lines. |

## Fixed decisions

M214 records only read-only facts:

```text
options_inventory_present
known_option_count
mutable_options_enabled = 0
env_toggles_added = 0
would_change_allocation_policy = 0
would_select_provider = 0
would_install_hook = 0
would_replace_process_allocator = 0
would_execute_reclaim = 0
```

## Stop lines

Do not add:

```text
mutable runtime options
environment option toggles
option parsing from env/files
allocation policy changes
thread scheduling
reclaim execution
unreserve
OS release
provider activation
hooks
process allocator replacement
backend app/name matchers
language syntax implementation
selfhost migration
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh
```

## Next blocker

```text
D208 mimalloc migration closeout check
```
