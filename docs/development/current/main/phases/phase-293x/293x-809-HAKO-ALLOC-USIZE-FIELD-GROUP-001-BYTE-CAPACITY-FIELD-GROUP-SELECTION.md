# 293x-809 HAKO-ALLOC-USIZE-FIELD-GROUP-001 Byte/Capacity Field-Group Selection

Status: selected current
Date: 2026-05-19

## Decision

Select the first exact-`usize` allocator field-group pilot after the
release-candidate closeout.

This row is a BoxShape / numeric-contract planning row. It must choose one
owner-local group of non-negative byte/capacity fields and leave all reason,
status, token, id, and sentinel-bearing fields on `i64`.

## Context

MIMAP-282A closed the modeled allocation-ledger release-candidate family. The
next cleanup should start reducing the `i64` overuse without collapsing the
sentinel and reason-code lanes into exact unsigned storage.

Existing SSOT:

```text
docs/development/current/main/design/usize-semantic-foundation-ssot.md
lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
```

## Candidate Direction

Prefer a narrow byte/capacity group from the segment arena backing
release-candidate/report family, for example fields such as:

```text
source_capacity
source_committed_bytes
source_uncommitted_bytes
padded_bytes
slot_capacity
planned_backing_bytes
planned_committed_bytes
applied_backing_bytes
applied_committed_bytes
remaining_source_bytes
```

The selected migration row should update `NUMERIC_FIELDS.md` first and migrate
only the selected owner-local group.

## Stop Lines

- No broad `i64` to `usize` rewrite.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields in this selection row.
- No migration of `-1` sentinel-bearing fields.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No new backend route or `.inc` matcher.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selected Row

TBD by this row:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-002
  migrate one owner-local byte/capacity field group only
```
