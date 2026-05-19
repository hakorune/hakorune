# 293x-842 HAKO-ALLOC-USIZE-FIELD-GROUP-034 Next Field-Group Selection

Status: landed
Date: 2026-05-19

## Decision

Select the next allocator exact-`usize` stored field group after the
arena-backing geometry count / page-size chain has been closed out.

`HAKO-ALLOC-USIZE-FIELD-GROUP-002` through `033` migrated and closed the
byte/capacity, diagnostic mirror, and arena-backing geometry chain from
release-candidate back through readiness. This row is a planning/selection row:
it must choose one narrow owner-local group before any additional migration.

## Selection Criteria

The selected group must be:

- stored production `hako_alloc` state under `lang/src/hako_alloc/memory/`;
- non-negative by local invariant;
- owner-local and narrow enough for one migration row plus one closeout row;
- free of `-1` sentinel semantics;
- free of reason/status/flag vocabulary semantics;
- free of pointer/id/generation semantics unless a dedicated id/index row
  explicitly owns that choice;
- backed by an existing L2/L3 guard pair or paired with a narrow guard update.

## Stop Lines

- No field migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of counters, ids, alignments, flags, or sentinels without a
  selected owner/group and a dedicated follow-up card.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Selected the segment-map accepted-readiness modeled consume ledger block/count
  report group as the next exact-`usize` migration group.
- Kept the selection downstream-first: migrate the consume-ledger report group
  before moving upstream to guarded readiness composition or scalar lookup
  owners.
- Did not migrate any field in this row.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-035` as the concrete migration row for the
segment-map accepted-readiness modeled consume ledger block/count report group.
