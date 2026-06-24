---
Status: Done
Date: 2026-06-05
Scope: prove the replacement-front size-class mirror is tied to `.hako` `SizeClassBox` policy without changing allocator execution.
Blocker: MIM-FMEM-017B
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - lang/src/hako_alloc/memory/size_class_box.hako
  - tools/hako_check/replacement_front_report.py
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
---

# 296x-431 SizeClassBox Bridge Evidence

## Purpose

`MIM-FMEM-017A` normalized product-shaped bridge evidence and named
`SizeClassBox` as the first source truth. `MIM-FMEM-017B` makes that concrete:
the replacement-front size-class mirror must prove it is bound to the `.hako`
`SizeClassBox` policy surface.

This row is still report/check-only.

## Decision

```text
source_truth=hako_alloc.size_class_box
source_file=lang/src/hako_alloc/memory/size_class_box.hako
generated_c_behavior_change=0
source_syntax_change=0
rust_parser_change=0
hako_parser_change=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

The bridge proves only pure size-class policy:

```text
word_size
max_regular_bin
huge_bin
normalize_size
bin_size
size_to_bin
good_size
accepts
usize facades
huge sentinel = -1
```

It does not prove page metadata, TLS arena residence, remote-free behavior,
segment backing, or product activation.

## Fields

```text
replacement_front_size_class_bridge_v0=1
replacement_front_size_class_bridge_report_only=1
replacement_front_size_class_bridge_source_truth=hako_alloc.size_class_box
replacement_front_size_class_bridge_source_file=lang/src/hako_alloc/memory/size_class_box.hako
replacement_front_size_class_bridge_mirror_source=hako_size_class_box_report_mirror
replacement_front_size_class_bridge_bound=0|1
replacement_front_size_class_bridge_missing=...

replacement_front_size_class_required_method_count=12
replacement_front_size_class_required_methods_present=0|1
replacement_front_size_class_missing_methods=none|...
replacement_front_size_class_word_size=8
replacement_front_size_class_max_regular_bin=72
replacement_front_size_class_huge_bin=73
replacement_front_size_class_huge_sentinel=-1
replacement_front_size_class_usize_facades_present=0|1

replacement_front_size_class_policy_methods_covered=0|1
replacement_front_size_class_policy_constants_covered=0|1
replacement_front_size_class_policy_huge_sentinel_covered=0|1
replacement_front_size_class_policy_mirror_matches_source=0|1
```

## Acceptance

```text
replacement_front_size_class_bridge_v0=1
replacement_front_size_class_bridge_report_only=1
replacement_front_size_class_bridge_source_truth=hako_alloc.size_class_box
replacement_front_size_class_bridge_bound=1
replacement_front_size_class_bridge_missing=none
replacement_front_size_class_required_methods_present=1
replacement_front_size_class_policy_constants_covered=1
replacement_front_size_class_policy_huge_sentinel_covered=1
replacement_front_size_class_policy_mirror_matches_source=1

replacement_front_product_shaped_bridge_activation_ready=0
product_activation_ready=0
type_abi_hot_path_lookup_count=0
provider_dispatch_hot_path=0
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

## Stop Line

- do not change `SizeClassBox` behavior
- do not add or change source syntax
- do not change only the Rust parser
- do not change generated C malloc/free behavior
- do not open product allocator activation
- do not install hooks, claim global allocator ownership, or make winner claims

## Landed Evidence

```text
replacement_front_size_class_bridge_v0=1
replacement_front_size_class_bridge_source_truth=hako_alloc.size_class_box
replacement_front_size_class_bridge_mirror_source=hako_size_class_box_report_mirror
replacement_front_size_class_bridge_bound=1
replacement_front_size_class_bridge_missing=none
replacement_front_size_class_required_method_count=12
replacement_front_size_class_required_methods_present=1
replacement_front_size_class_policy_constants_covered=1
replacement_front_size_class_policy_huge_sentinel_covered=1
replacement_front_size_class_policy_mirror_matches_source=1
generated_c_behavior_change=0
source_syntax_change=0
rust_parser_change=0
hako_parser_change=0
product_activation=0
```

Next row:

```text
MIM-FMEM-017C Page-local state bridge evidence
```
