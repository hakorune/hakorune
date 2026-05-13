---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: M214 read-only allocator options/defaults inventory surface.
Related:
  - docs/development/current/main/design/mimalloc-next-row-selection-ssot.md
  - docs/development/current/main/design/mimalloc-port-remaining-inventory-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/options_inventory_box.hako
  - apps/hako-alloc-options-inventory-proof/
---

# Hako Alloc Options Inventory SSOT

## Decision

M214 adds a read-only `.hako` options/defaults inventory surface for the
mimalloc port.

The owner names stable option/default/init facts so future rows can reason
about option vocabulary without adding mutable runtime configuration.

## Owner

```text
lang/src/hako_alloc/memory/options_inventory_box.hako
```

Responsibilities:

```text
classify known allocator option ids
report stable default values
report that defaults are static/read-only
report that mutable options and env toggles are inactive
report that provider/hook/replacement/reclaim remain inactive
```

Non-responsibilities:

```text
parsing environment variables
loading config files
changing allocation policy
changing size-class/page/queue behavior
selecting providers
installing hooks
replacing the process allocator
executing reclaim
unreserve or OS release
```

## Proof surface

```text
apps/hako-alloc-options-inventory-proof/
tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh
```

Required report facts:

```text
options_inventory_present = 1
known_option_count > 0
mutable_options_enabled = 0
env_toggles_added = 0
would_change_allocation_policy = 0
would_select_provider = 0
would_install_hook = 0
would_replace_process_allocator = 0
would_execute_reclaim = 0
```

## Stop line

M214 must stay read-only. Any row that wants mutable options, process/env
configuration, allocator policy changes, provider activation, hooks,
replacement, or reclaim execution must open a new accepted design card first.
