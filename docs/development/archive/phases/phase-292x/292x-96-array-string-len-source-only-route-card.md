---
Status: Landed
Date: 2026-04-22
Scope: A2c implementation card for moving `array_string_len_window` source-only direct-set reuse from `.inc` analysis to MIR metadata.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
  - docs/development/current/main/phases/phase-292x/292x-95-array-string-len-keep-live-route-card.md
---

# 292x-96: `array_string_len_window` Source-Only Route Card

## Problem

A2a and A2b moved len-only and keep-live source reuse into MIR metadata. The
remaining `array_string_len_window` C owner is the direct-set reuse branch:

```text
array.get(i) -> copy* -> length
                         -> substring pair / piecewise concat
                         -> same-array set
```

The legacy C analyzer still recognizes this by scanning raw MIR JSON for
insert-mid and piecewise concat shapes. That is the last blocker before
`analyze_array_string_len_window_candidate` can be deleted.

## Decision

MIR must emit an explicit source-only route tag for direct-set reuse. `.inc`
must consume only the route tag and must not rediscover the substring/concat/set
shape.

## Fixture Inventory

Pinned by metadata route after this card:

- `apps/tests/mir_shape_guard/array_string_len_insert_mid_source_only_min_v1.mir.json`
- `apps/tests/mir_shape_guard/array_string_len_piecewise_concat3_source_only_min_v1.mir.json`
- `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh`
- `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh`

Current route expectation:

```text
stage=array_string_len_window result=hit reason=mir_route_metadata
keep_get_live=0
source_only_insert_mid=1
proof=array_get_len_source_only_direct_set
```

## Metadata Vocabulary

Route id stays:

```text
array.string_len.window
```

Mode to add:

```text
source_only_insert_mid
```

Proof vocabulary:

```text
array_get_len_source_only_direct_set
```

Required field expectation:

- `keep_get_live: false`
- `source_only_insert_mid: true`
- `effects: ["load.cell", "observe.len", "publish.source.ref"]`
- skip indices cover the metadata-emitted len instruction; copy aliases are not
  skipped because later direct-set routes need the alias state

## Implementation Notes

- Do not port the full C analyzer line-for-line.
- Reuse existing MIR-owned string corridor / kernel plan metadata when possible.
- If MIR cannot prove same-array direct-set without another SSOT, stop and add
  the missing route metadata first.
- The static fixture metadata includes `value_consumer_facts` for the final set
  value so downstream direct-set windows do not fall back to public substring
  materialization.
- Delete `analyze_array_string_len_window_candidate` in the follow-up card after
  both source-only fixture smokes require `reason=mir_route_metadata`.

## Verification

```bash
cargo test -q array_string_len_window
cargo test -q build_mir_json_root_emits_array_string_len_window_routes
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh
```
