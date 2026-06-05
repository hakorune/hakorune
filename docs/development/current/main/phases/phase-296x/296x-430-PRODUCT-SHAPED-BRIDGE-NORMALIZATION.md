---
Status: Done
Date: 2026-06-05
Scope: normalize non-activating product-shaped replacement-front bridge evidence after mimalloc shape coverage gates landed.
Blocker: MIM-FMEM-017A
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - tools/hako_check/replacement_front_report.py
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
---

# 296x-430 Product-Shaped Bridge Normalization

## Purpose

`MIM-FMEM-016` separated speed from mimalloc shape/safety/coverage. This row
adds a normalized, non-activating product-shaped bridge report so the next
implementation work cannot confuse keeper eligibility with product activation
readiness.

## Decision

`MIM-FMEM-017A` is report/check-only:

```text
generated_c_behavior_change=0
source_syntax_change=0
rust_parser_change=0
hako_parser_change=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

The first `.hako` source truth for the bridge is:

```text
replacement_front_product_shaped_bridge_source_truth=hako_alloc.size_class_box
source_file=lang/src/hako_alloc/memory/size_class_box.hako
```

Page metadata, TLS owner residence, remote-free execution, segment backing, and
activation remain later rows.

## Fields

```text
replacement_front_product_shaped_bridge_v0=1
replacement_front_product_shaped_bridge_non_activating=1
replacement_front_product_shaped_bridge_report_only=1
replacement_front_product_shaped_bridge_route=replacement_front_benchmark_to_product_ldpreload_descriptor
replacement_front_product_shaped_bridge_source_truth=hako_alloc.size_class_box
replacement_front_product_shaped_bridge_evidence_ready=0|1
replacement_front_product_shaped_bridge_activation_ready=0
replacement_front_product_shaped_bridge_block_reason=...
replacement_front_product_shaped_bridge_missing=...

replacement_front_product_shaped_bridge_shape_ok=0|1
replacement_front_product_shaped_bridge_safety_ok=0|1
replacement_front_product_shaped_bridge_coverage_ok=0|1
replacement_front_product_shaped_bridge_preflight_ok=0|1

replacement_front_product_shaped_bridge_no_type_abi_hot_lookup=0|1
replacement_front_product_shaped_bridge_no_provider_dispatch=0|1
replacement_front_product_shaped_bridge_no_global_lock_hot_path=0|1
replacement_front_product_shaped_bridge_no_range_scan_hot_path=0|1
replacement_front_product_shaped_bridge_no_host_passthrough=0|1

replacement_front_product_shaped_bridge_requires_activation_row=1
replacement_front_product_shaped_bridge_requires_product_gate_open=1
```

While product activation is closed, `replacement_front_product_shaped_bridge_missing`
must include:

```text
product_gate_open
activation_row
```

## Smoke Growth Brake

```text
new_smoke_script_added=0
extend_existing_replacement_front_report_smoke=1
extend_existing_fastmem_capability_inventory_smoke=1
extend_existing_fastmem_check_smoke=1
```

## Acceptance

```text
replacement_front_product_shaped_bridge_v0=1
replacement_front_product_shaped_bridge_evidence_ready=1
replacement_front_product_shaped_bridge_activation_ready=0
replacement_front_product_shaped_bridge_missing contains product_gate_open,activation_row
type_abi_hot_path_lookup_count=0
provider_dispatch_hot_path=0
product_activation_ready=0
```

Proof:

```bash
python3 -m py_compile tools/hako_check/replacement_front_report.py tools/hako_check/fastmem_capability_inventory.py tools/hako_check/fastmem_check.py
bash tools/hako_check/replacement_front_report_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
replacement_front_product_shaped_bridge_v0=1
replacement_front_product_shaped_bridge_source_truth=hako_alloc.size_class_box
replacement_front_product_shaped_bridge_evidence_ready=1
replacement_front_product_shaped_bridge_activation_ready=0
replacement_front_product_shaped_bridge_missing=product_gate_open,activation_row
new_smoke_script_added=0
source_syntax_change=0
rust_parser_change=0
hako_parser_change=0
```

Next row:

```text
MIM-FMEM-017B SizeClassBox bridge evidence
```

## Stop Line

- do not add or change source syntax
- do not change only the Rust parser
- do not change generated C malloc/free behavior
- do not open product allocator activation
- do not install hooks, claim global allocator ownership, or make winner claims
