---
Status: Done
Date: 2026-06-05
Scope: separate mimalloc shape coverage from speed evidence so keeper candidacy cannot be decided by throughput alone.
Blocker: MIM-FMEM-016
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
  - tools/hako_check/fastmem_capability_inventory_smoke.sh
  - tools/hako_check/fastmem_check_smoke.sh
---

# 296x-429 Mimalloc Shape Coverage Score

## Purpose

`MIM-FMEM-015` made safe wrappers visible as aliases over existing FastMemory
MemOps. This row adds a keeper gate that distinguishes speed from mimalloc
shape, safety, and coverage.

## Decision

Keeper candidacy is explicit:

```text
mimalloc_keeper_candidate=0|1
mimalloc_keeper_eligible=0|1
```

Ordinary observation reports remain non-failing. `fastmem-check` applies the
stricter shape gate only when `mimalloc_keeper_candidate=1`.

## Score Families

```text
mimalloc_speed_score:
  throughput interpretation only

mimalloc_shape_score:
  structural mimalloc-shape evidence

mimalloc_safety_score:
  boundary and safety evidence

mimalloc_coverage_score:
  required coverage evidence for keeper candidacy
```

Shape components are 10 points each:

```text
page_map_bridge
typed_page_meta
tls_arena
alloc_owner
owner_check
same_owner_local_free
atomic_remote_head
safe_wrappers
no_global_lock_hot_path
no_range_scan_hot_path
```

## Boundary

```text
source_syntax_change=0
rust_only_parser_change=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
type_abi_hot_path_lookup_count=0
provider_dispatch_hot_path=0
```

Parser parity remains part of proof even though this row does not change
syntax.

## Acceptance

```text
mimalloc_keeper_candidate=1
mimalloc_shape_score>=80
mimalloc_safety_score=100
mimalloc_coverage_score>=80
mimalloc_keeper_eligible=1
mimalloc_keeper_block_reason=eligible
```

The negative fixture proves a fast/marked keeper candidate with insufficient
shape coverage fails `fastmem-check`.

Proof:

```bash
python3 -m py_compile tools/hako_check/fastmem_capability_inventory.py tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh
bash tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
mimalloc_shape_coverage_score_fields=1
mimalloc_keeper_candidate_gate=1
mimalloc_keeper_positive_fixture=shape_coverage_keeper_report.kv
mimalloc_keeper_negative_fixture=bad_shape_keeper_inventory.kv
new_smoke_script_added=0
parser_parity_checked=1
source_syntax_change=0
rust_only_parser_change=0
product_activation=0
```

Next row:

```text
MIM-FMEM-017 Product-shaped replacement front bridge
```

## Stop Line

- do not add new source syntax
- do not change only the Rust parser
- do not select keepers by throughput alone
- do not open product allocator activation, hooks, global allocator claim, or
  winner claim
- do not treat Type ABI or Provider ABI as replacement-front hot-path dispatch
