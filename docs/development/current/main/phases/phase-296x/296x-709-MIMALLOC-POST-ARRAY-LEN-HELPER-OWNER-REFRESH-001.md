---
Status: Active
Date: 2026-06-15
Task: MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001
Scope: Classify the remaining `nyash_array_length_h` residue after the
  borrowed-ready helper keeper.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-708-MIMALLOC-ARRAY-LEN-HELPER-BORROWED-READY-IMPLEMENTATION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001

## Purpose

Refresh the owner after 296x-708. The helper switch reduced the measured
`nyash_array_length_h` share but did not remove the helper boundary.

The remaining samples are inside the handle-registry / typed-object boundary:

```text
registry read lock / OnceCell-ready path
typed handle TLS lookup
Arc / trait-object downcast residue
drop-epoch guard residue
```

This row decides whether a narrow next implementation is still justified, or
whether the remaining gap belongs to a broader closed-world object/handle
substrate lane.

## Inputs

```text
source_evidence=296x-708
target_symbol=nyash_array_length_h
body_elapsed_ns_after=53000000
top_symbol_percent_after=68.13
```

## Required Output

```text
output_contract=hako-mimalloc-post-array-len-helper-owner-refresh-v0
source_evidence=296x-708
target_symbol=nyash_array_length_h
body_elapsed_ns=<n>
top_symbol_percent=<pct>
remaining_owner=<owner|none>
remaining_owner_confidence=<low|medium|high>
implementation_allowed=<0|1>
closed_world_route_required=<0|1>
object_substrate_required=<0|1>
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not change helper ABI
do not add direct raw ArrayBox layout lowering
do not add benchmark-specific or helper-name-specific compiler branches
do not change MIRBuilder or LLVM lowering
do not change tracked .hako source
do not change Arc/object substrate
do not change product defaults
```

## Acceptance

```text
source_evidence=296x-708
implementation_started=0
summary=pending
```
