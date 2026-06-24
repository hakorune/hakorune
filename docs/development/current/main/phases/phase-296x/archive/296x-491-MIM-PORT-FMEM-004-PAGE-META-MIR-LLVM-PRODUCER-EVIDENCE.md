---
Status: Done
Date: 2026-06-06
Scope: MIM-PORT-FMEM-004.
Related:
  - docs/development/current/main/phases/phase-296x/296x-490-MIM-PORT-FMEM-003-PAGE-META-TABLEINDEX-PROOF-SURFACE.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-491 MIM-PORT-FMEM-004 PageMeta MIR-to-LLVM Producer Evidence

## Decision

Add a narrow producer evidence seam for the PageMeta fastmem pilot:

```text
.hako PageMeta fastmem body
  -> MIR FastMemRegion / MemOp / verified access plans
  -> Python LLVM producer object compile
  -> producer-neutral KV evidence
  -> fastmem-check --inventory
```

This row does not add new lowering policy. It records that the existing
MIR-to-LLVM producer can consume the verifier-owned PageMeta `TableIndex`,
`FieldLoad`, and `FieldStore` plans without route selection, Type ABI lookup,
Provider ABI dispatch, or Python-template C semantic fallback.

## Implemented

```text
src/llvm_py/instructions/copy.py:
  propagates FastMemory LayoutRef carriers through Copy aliases without placing
  raw LayoutRef values in the ordinary LLVM value map.

src/llvm_py/tests/test_fastmem_memop_layoutref.py:
  adds a regression that verifies Copy keeps LayoutRef in
  resolver.fastmem_layout_refs and out of vmap before a field load consumes it.

tools/hako_check/fastmem_mir_to_llvm_producer_report.py:
  compiles a MIR JSON file through the existing Python LLVM producer, then
  emits observation-only KV evidence from verified FastMemory access-plan
  metadata.

tools/hako_check/fastmem_mir_to_llvm_producer_report_route_rows.py:
  accepts the CLI profile spelling layout-table as the layout-table producer
  profile, so route rows no longer fall back to stale atomic-remote defaults.

tools/hako_check.sh:
  exposes the report as:
    fastmem-mir-to-llvm-producer-report --mir-json mir.json [--out report.kv]

tools/hako_check/manifests/fastmem_source_syntax_smoke.toml:
  extends the existing FastMemory smoke so the PageMeta pilot now checks:
    source inventory
    MIR metadata inventory
    MIR-to-LLVM producer evidence
    fastmem-check over that evidence

  The manifest also records newly reached producer boundaries where the Copy
  LayoutRef fix advances old `expected-layout-ref` failures to either
  producer success, check failure, or a later fail-fast reason.
```

## Evidence Shape

Expected producer evidence:

```text
output_contract=hako-check-fastmem-mir-to-llvm-producer-report-v0
replacement_front_producer=mir_to_llvm_lowering
replacement_front_backend_artifact=object
replacement_front_producer_transition_state=final_primary
replacement_front_next_producer_slice=layout_table_producer_pilot
replacement_front_selected_memop_kinds=TableIndex,FieldLoad,FieldStore
replacement_front_deferred_memop_kinds=CurrentAllocOwnerId,OwnerEq
mir_fmem_008b_layout_table_producer_pilot=1
memop_table_index_lowered_count=1
memop_field_load_lowered_count=3
memop_field_store_lowered_count=1
memop_current_alloc_owner_id_lowered_count=0
memop_owner_eq_lowered_count=0
memop_atomic_remote_head_lowered_count=0
fastmem_raw_pointer_in_ordinary_vmap_count=0
fastmem_layout_ref_escape_count=0
fastmem_lowering_recomputed_layout_offset_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
summary=ok
```

The report is already an inventory-shaped producer evidence file. Use
`fastmem-check --inventory` for this direct producer report. The older
`fastmem-check --report` path remains for benchmark/replacement-front reports
that need normalization first.

## Still Closed

```text
DirectArray/free-list lowering
block_used storage mutation
local_free_head mutation
remote_head / AtomicRemoteHead lowering
CurrentAllocOwnerId / OwnerEq in the PageMeta body
owner field mutation
TLS backing transfer
same-owner / remote-owner routing policy
Python-template C diagnostic payload deletion/archive
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```bash
NYASH_FEATURES=stage3,rune target/release/hakorune --backend mir \
  --emit-mir-json /tmp/page_meta_pilot.mir.json \
  lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako

bash tools/hako_check.sh fastmem-mir-to-llvm-producer-report \
  --mir-json /tmp/page_meta_pilot.mir.json \
  --out /tmp/page_meta_pilot.llvm.report.kv

bash tools/hako_check.sh fastmem-check \
  --inventory /tmp/page_meta_pilot.llvm.report.kv \
  --format kv \
  --out /tmp/page_meta_pilot.llvm.check.kv

bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_producer_parity_smoke.sh
.venv/bin/pytest -q src/llvm_py/tests/test_fastmem_metadata_loader.py \
  src/llvm_py/tests/test_fastmem_memop_layoutref.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Verified on 2026-06-12:

```text
.venv/bin/pytest -q src/llvm_py/tests/test_fastmem_metadata_loader.py \
  src/llvm_py/tests/test_fastmem_memop_layoutref.py
  -> 31 passed

bash tools/hako_check/fastmem_source_syntax_smoke.sh
  -> [TEST/OK] fastmem_source_syntax

bash tools/hako_check/fastmem_capability_inventory_smoke.sh
  -> [TEST/OK] fastmem_capability_inventory

bash tools/hako_check/fastmem_check_smoke.sh
  -> [TEST/OK] fastmem_check

bash tools/hako_check/fastmem_producer_parity_smoke.sh
  -> [TEST/OK] fastmem_producer_parity

bash tools/checks/current_state_pointer_guard.sh
  -> ok

cargo check --release --bin hakorune
  -> finished release profile

git diff --check
  -> ok
```

## Next

```text
MIM-PORT-FMEM-005:
  choose the next hako_alloc body slice after scalar PageMeta producer evidence.
  Candidate owner: PageMeta local/free-list field group with DirectArray /
  free-list mutation still behind explicit proof gates.
```
