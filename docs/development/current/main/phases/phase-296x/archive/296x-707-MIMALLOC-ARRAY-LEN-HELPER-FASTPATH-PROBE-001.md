---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-ARRAY-LEN-HELPER-FASTPATH-PROBE-001
Scope: Probe the `nyash_array_length_h` / `nyash.array.slot_len_h` helper
  fast path selected by 296x-706 without changing helper ABI or product defaults.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-705-MIMALLOC-RUNTIME-BOUNDARY-DIRECT-PROBE-001.md
  - docs/development/current/main/phases/phase-296x/296x-706-MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-ARRAY-LEN-HELPER-FASTPATH-PROBE-001

## Purpose

296x-706 showed that MIR already has a clean `generic_method.len /
array_slot_len` route for the hot `ArrayBox.length` site. The remaining boundary
is the generated runtime helper:

```text
nyash.array.slot_len_h
  -> nyash_array_length_h
```

This row probes that helper's internal cost before implementation. The goal is
to decide whether a narrow helper fast path is justified.

## Scope

```text
included:
  - inspect helper assembly and call sites
  - classify cache-hit / cache-miss / host-handle lookup / Arc/drop residue
  - decide whether a helper-internal fast path is allowed

excluded:
  - MIRBuilder route changes
  - backend raw-layout direct load
  - helper ABI changes
  - product default NyRT changes
  - tracked .hako source changes
  - Arc retirement / object substrate replacement
```

## Required Output

```text
output_contract=hako-mimalloc-array-len-helper-fastpath-probe-v0
target_symbol=nyash_array_length_h
source_evidence=296x-706
mir_route_already_array_slot_len=1
helper_abi_changed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
selected_owner=<helper_cache_hit|helper_cache_miss|host_handle_lookup|arc_drop_residue|pause>
selected_owner_confidence=<low|medium|high>
implementation_allowed=<0|1>
winner_claim=0
summary=ok
```

## Probe Result

```text
output_contract=hako-mimalloc-array-len-helper-fastpath-probe-v0
target_symbol=nyash_array_length_h
source_evidence=296x-706
mir_route_already_array_slot_len=1
perf_runs=10
in_process_operation_repeat=65536
top_symbol=nyash_array_length_h
top_symbol_percent=72.06
helper_abi_changed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
selected_owner=host_handle_lookup
selected_owner_confidence=medium
implementation_allowed=1
winner_claim=0
next_task=MIMALLOC-ARRAY-LEN-HELPER-BORROWED-READY-IMPLEMENTATION-001
summary=ok
```

## Evidence

`nyash_array_length_h` is:

```text
with_array_box(handle, |arr| arr.len() as i64).unwrap_or(0)
```

`with_array_box` first checks the TLS typed handle cache, but the hot annotated
block is the miss path:

```text
handles::get(handle)
cache_store(handle, drop_epoch, obj.clone())
```

The hot instructions are in registry lock / host-handle lookup / clone-drop
residue:

```text
top_instruction_percent=63.96
top_instruction_asm=and $0xfffffffffffffff2,%rax

hot_instruction_1_percent=35.26
hot_instruction_1_asm=jne <nyash_array_length_h+0xb8>
```

The cache-hit path itself does not carry samples in this run, so a raw layout
compiler lowering is the wrong owner. The narrow implementation seam is to keep
the public helper ABI but make the `length` helper use the existing borrowed
read-only handle path instead of the clone-and-cache path.

## Stop Line

```text
do not patch tracked source .hako
do not specialize by benchmark name
do not replace ArrayBox.length with raw layout load
do not change nyash.array.slot_len_h ABI
do not change product default NyRT
do not mix Arc/object-substrate retirement into this row
```

## Acceptance

```text
source_evidence=296x-706
implementation_started=0
summary=ok
```
