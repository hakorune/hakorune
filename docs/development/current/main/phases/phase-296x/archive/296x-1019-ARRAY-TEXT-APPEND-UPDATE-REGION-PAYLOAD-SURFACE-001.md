Status: Done
Date: 2026-06-17
Scope: passive MIR-owned executor-contract payload for the indexOf append/update front.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1018-ARRAY-TEXT-APPEND-UPDATE-PRODUCER-SHAPE-DIAGNOSTIC-001.md

# ARRAY-TEXT-APPEND-UPDATE-REGION-PAYLOAD-SURFACE-001

## Decision

The append/update observer front may attach a passive single-region executor
contract when MIR proves this exact shape:

- `indexOf(const_utf8)` feeds a found predicate.
- The true arm stores `source + const_suffix` back to the same array row.
- The row index is `loop_index % const`.
- The stored updated text feeds `length()`.
- The length result feeds the scalar accumulator add.
- Publication boundary remains `none`.

This row is metadata only. It does not enable backend lowering, runtime helper
lowering, or source rewriting.

## Evidence

Target front:

```text
benchmarks/bench_kilo_meso_indexof_append_array_set.hako
```

Report command:

```bash
cargo build -q --release --bin hakorune
HAKO_STAGE1_MODE=emit-mir HAKO_EMIT_MIR_JSON=1 STAGE1_EMIT_MIR_JSON=1 \
  target/release/hakorune --emit-mir-json \
  target/array-text-append-update-region-payload-1019/indexof_append_array_set.mir.json \
  benchmarks/bench_kilo_meso_indexof_append_array_set.hako
python3 tools/hako_check/state_explain.py \
  --mir-json target/array-text-append-update-region-payload-1019/indexof_append_array_set.mir.json \
  --topn 3
```

Observed:

```text
array_text_observer_route_count=1
array_text_observer_executor_contract_count=1
array_text_observer_route_0_executor_contract_effects=observe.indexof,store.cell,length_result_carry,scalar_accumulator
array_text_observer_route_0_executor_contract_consumer_capabilities=compare_only,sink_store_len_sum
array_text_observer_route_0_region_mapping_row_modulus_const=128
array_text_observer_route_0_region_mapping_length_result_value=41
array_text_observer_route_0_region_mapping_accumulator_phi_value=33
array_text_observer_route_0_region_mapping_accumulator_next_value=43
```

## Implementation Notes

- `array_text_observer_region_contract` now reads CFG terminals from either
  `BasicBlock::terminator` or instruction-tail terminators.
- The matcher derives predecessor edges from terminal successors instead of
  requiring `BasicBlock::predecessors` to be populated.
- Store-only and store+len-sum contracts share the same region contract family,
  but store+len-sum has explicit effect/capability tags.
- hako_check reports the passive payload fields for auditability.

## Stop Lines

- Do not infer this shape in the C backend from raw MIR.
- Do not enable backend lowering in this row.
- Do not add runtime helper calls in this row.
- Do not claim a perf win from metadata reachability.
- Do not move this logic into MIRBuilder.

## Next

```text
ARRAY-TEXT-APPEND-UPDATE-BACKEND-READER-SURFACE-001
```

Add a backend reader/guard surface for the new executor-contract payload while
keeping lowering disabled.
