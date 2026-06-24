# 296x-1012 ARRAY-TEXT-STORE-LOCAL-MUTATION-BACKEND-CONSUMER-001

Status: Landed
Date: 2026-06-17
Scope: C ABI backend consumer for cross-array len-only array text edit route

## Contract

```text
output_contract=hako-array-text-store-local-mutation-backend-consumer-v0
source_evidence=296x-1011,target/fresh-compiler-owner-selection-1008
row_kind=implementation

target_front=kilo_meso_substring_concat_array_set
selected_owner_family=array_text_slot_insert_store_boundary

consumed_proof=array_get_lenhalf_insert_mid_dest_slot_len_only
same_slot_proof_preserved=1
backend_consumer_added=1
backend_consumer=array_text_edit_lenhalf_dest_len_only

consumer_capability_required=sink_store_len_only
consumer_capability_fallback_fact_enabled=0
destination_store_emitted=0
destination_store_skipped=1
insert_helper_emitted=0
store_helper_emitted=0
result_len_lowering=slot_len_plus_middle_len

ny_main_calls_hako_array_text_slot_len=1
ny_main_calls_kernel_slot_insert_hisi=0
ny_main_calls_kernel_slot_store_hi=0
ny_main_hot_loop_shape=slot_len_plus_add_const

c_kernel_cycles=182726
ny_kernel_cycles=200265
c_kernel_instr=901308
ny_kernel_instr=565498
ratio_kernel_cycles=0.91
ratio_kernel_instr=1.59

product_arraybox_storage_changed=0
product_stringbox_storage_changed=0
benchmark_name_branch_count=0
source_name_branch_count=0
helper_name_inference_count=0
mirbuilder_object_management_enabled=0

next_task=FRESH-COMPILER-OWNER-SELECTION-002
summary=ok
```

## Purpose

Consume the cross-array len-only route added in 296x-1011 without changing
product ArrayBox / StringBox storage.

The selected front computes:

```text
line = src.get(row)
out = line.substring(0, len/2) + "xx" + line.substring(len/2, len)
dst.set(row, out)
total += out.length()
```

The active result only observes `out.length()`.  The destination array contents
are not read by the benchmark body before the final length check, so this row
consumes the route as a length-only sink:

```text
result_len = hako.array_text.slot_len(src, row) + 2
```

## Implementation

The C ABI route reader now accepts only the explicit proof:

```text
proof=array_get_lenhalf_insert_mid_dest_slot_len_only
consumer_capabilities=["sink_store_len_only"]
destination_array_value != array_value
result_len_value > 0
publication_boundary=none
```

The backend consumer emits:

```llvm
%arrtext_edit_len_<block>_<inst> =
  call i64 @"hako.array_text.slot_len"(i64 %src_array, i64 %row)
%r<result_len_value> =
  add i64 %arrtext_edit_len_<block>_<inst>, <middle_byte_len>
```

It marks the matched substring / concat / destination-store instruction window
as skipped.  The existing same-slot store consumer remains tied to
`array_get_lenhalf_insert_mid_same_slot`.

## Measurement

Commands:

```bash
cargo test -q array_text_edit_plan --lib
cargo check -q --release --bin hakorune
bash tools/perf/build_perf_release.sh
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh \
  kilo_meso_substring_concat_array_set ny_main 1
bash tools/perf/bench_micro_c_vs_aot_lanes.sh \
  kilo_meso_substring_concat_array_set 1 3 100
```

`ny_main` hot loop after the consumer:

```asm
and    $0x3f,%esi
mov    %rbx,%rdi
call   hako.array_text.slot_len
add    %rax,%r15
add    $0x2,%r15
inc    %r12
cmp    $0x2bf20,%r12
jne    ...
```

The previous `nyash.array.kernel_slot_insert_hisi` and
`nyash.array.kernel_slot_store_hi` calls are gone from the loop.

Lane result:

```text
c_kernel_instr=901308
c_kernel_cycles=182726
ny_kernel_instr=565498
ny_kernel_cycles=200265
ratio_kernel_instr=1.59
ratio_kernel_cycles=0.91
aot_status=ok
```

## Stop Line

```text
do not generalize this to arbitrary cross-array store elision
do not consume fallback evidence as a fast-path Fact
do not change product ArrayBox / StringBox storage
do not branch by benchmark/source/helper name
do not claim broader string/array store semantics
```

## Next

```text
FRESH-COMPILER-OWNER-SELECTION-002
```

Select the next front / owner from current measured evidence.  If another
array-text route is chosen, it must start from route reachability evidence, not
from helper-symbol inference.
