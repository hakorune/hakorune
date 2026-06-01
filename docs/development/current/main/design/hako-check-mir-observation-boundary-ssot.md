---
Status: Active
Date: 2026-05-27
Scope: boundary between hako_check source perf-surface and MIR-level method shape observation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-80-HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-81-HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH.md
  - docs/development/current/main/design/hako-optimization-toolbox-usability-ssot.md
  - tools/hako_check/README.md
---

# hako_check / MIR Observation Boundary

## Decision

`hako_check perf-surface` remains a source-level observation tool. MIR-level
shape observation is normally a separate adapter/app surface.

Exception: `hako_check fastpath-explain` may consume an already-emitted MIR JSON
artifact to print direct-memory diagnostic coverage. This exception is read-only
and exists so future `direct {}` / FastPath obligation failures have one
developer-facing explanation surface. It must not emit MIR, rewrite source,
select keepers, or own lowering policy.

```text
hako_check perf-surface:
  source-level risk and keeper suggestion

MIR method shape adapter:
  actual lowered MIR shape for selected methods

hako_check fastpath-explain:
  MIR JSON diagnostic adapter for DirectArray / Span / RequiredFastPath metadata

keeper diff adapter:
  before/after source report + MIR report + measurement evidence
```

## Rationale

`hako_check` should not become an optimizer or a MIR analyzer. Its job in the
mimalloc parity lane is to identify suspicious `.hako` source surfaces and
suggest one next keeper candidate.

MIR observation answers a different question: whether a selected source risk
actually lowers into costly calls, field operations, copies, PHIs, branches, or
runtime checks.

Keeping these contracts separate prevents hako_check from accumulating backend
responsibility and keeps each row narrow.

The fastpath-explain exception is intentionally narrower than the general MIR
method-shape adapter: it reads named metadata fields that are already part of
the direct-memory diagnostic contract and reports missing/passing obligations.
It does not infer new method shape, route ownership, or performance keepers.

## Planned Surfaces

### Source Perf-Surface v1

Owner: `tools/hako_check`.

```text
output_contract=hako-check-perf-surface-v1
loop_field_get_count
loop_field_set_count
loop_array_get_count
loop_array_length_count
allocation_like_in_loop_count
suggested_next_kind=box_count|box_shape|mir_diagnostic|none
confidence=low|medium|high
summary=ok
```

### MIR Method Shape v0

Owner: `tools/mir_check` initially.

```text
output_contract=hako-mir-method-shape-v0
input_kind=mir_json
selected_method
mir_instruction_count
call_count
field_get_count
field_set_count
array_get_call_count
array_length_call_count
phi_count
copy_count
branch_count
return_count
summary=ok
```

The first implementation should be Python. `.hako` migration comes only after
the contract and fixture expectations stabilize.

### Keeper Diff v0

Owner: adapter surface, not hako_check core.

```text
output_contract=hako-mimalloc-keeper-before-after-diff-v0
keeper_id
before_source_surface
after_source_surface
before_mir_shape
after_mir_shape
measurement_before
measurement_after
keeper_effect=accepted|no_effect|regressed|inconclusive
summary=ok
```

### FastPath Explain v0

Owner: `tools/hako_check`, read-only MIR JSON adapter.

```text
output_contract=hako-check-fastpath-explain-v0
input_kind=mir_json
tool_surface=hako_check_fastpath_explain
observation_only=1
rewrite_executed=0
direct_array_access_plan_count
direct_array_checked_plan_count
direct_array_proved_unchecked_plan_count
span_access_plan_count
required_fastpath_region_count
fastpath_obligation_count
fastpath_obligation_failed_count
missing_fastpath_plan_count
clean=0|1
summary=ok|failed
```

## Stop Line

- `hako_check` does not rewrite source.
- `hako_check fastpath-explain` does not emit MIR; it consumes a caller-provided
  MIR JSON file only.
- MIR method shape does not select keepers by itself.
- Diff adapter does not implement keepers.
- Provider activation, process allocator replacement, hooks, globals, and
  winner claims remain closed unless a separate decision row opens them.
