# Hako Alloc Wide Report Argument Cleanup SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: ARG-DATA-001

## Decision: accepted

Insert a short BoxShape sidecar before MIMAP-454A to reduce wide report and
long positional argument pressure in allocator comparison/report owners.

The immediate problem is not primarily the lack of more integer types. The
problem is wide, flat data:

- one report owns many unrelated fields;
- owner-local `ReportFields` records mirror report boxes field-for-field;
- `makeReport` / `reject` / `record...` methods move long positional argument
  lists;
- stop-line evidence, runner evidence, counters, and decision state are mixed
  in one flat payload.

## Cleanup Order

1. Structure first.
   Split wide reports into conceptual groups before adding source syntax.

   ```text
   decision:
     accepted / reason

   runner_evidence:
     runner invoked / output present / result code / run counters

   memory_evidence:
     requested bytes / peak rss / memory evidence present

   counters:
     total / accepted / rejected / reason counters

   forbidden_actions:
     process replacement / hook / backend matcher / global allocator /
     hidden discovery / provider package / worker thread
   ```

2. Replace long positional argument lists with owner-local context records.

   Preferred shape:

   ```hako
   record SomeRunnerEvidence {
       explicit_runner_invoked: i64
       output_present: i64
       result_code: i64
       run_count: i64
   }

   record SomeStopLineEvidence {
       process_replacement_executed: i64
       hook_installed: i64
       backend_matcher_added: i64
       global_allocator_installed: i64
   }

   recordSomeEvidence(diagnostic, runner, stop_lines)
   ```

3. Keep the current `new Box { field: expr }` initializer canonical.
   It is useful as a field-set contract even when it does not reduce line
   count.

4. Use record construction ergonomics only for owner-local data shaping.
   ARG-DATA-003 accepted record field defaults, empty record literals,
   same-name shorthand, and record-only `with` updates. These are Stage1
   source ergonomics for tracked local records, not runtime record
   materialization and not an automatic record-to-box copy.

5. Do not introduce wide-copy sugar yet.
   The following remain parked until a later language row proves exact
   semantics and fail-fast behavior:

   - record spread / `...fields`;
   - named arguments;
   - automatic record-to-box copy.

6. Do not bulk-convert every historical diagnostic owner just to make the
   source look uniform. The measured ARG-DATA-008 batch showed little line-count
   reduction when only defaults and `with` are applied. Future owner work should
   use the accepted record ergonomics for new or touched owners, and should use
   data-shape decomposition when actual line-count or argument-pressure
   reduction is required.

## Near-Term Pilot

The first implementation row should pick one recent wide owner and only do
argument-shape cleanup:

```text
ARG-DATA-002:
  C mimalloc explicit runner report argument object pilot
```

Recommended owner:

```text
lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako
```

Target:

- keep output lines unchanged;
- keep report box fields unchanged;
- introduce narrow context records for runner evidence and stop-line evidence;
- reduce repeated long positional arguments in owner-local `makeReport` /
  `reject` flows;
- keep the public
  `recordAllocatorComparisonCMimallocExplicitRunnerExecution(...)` entry
  positional for now, because record-local carriers must not cross owner/API
  boundaries until a future language row defines that contract;
- do not change language syntax.

## ARG-DATA-002 Outcome

ARG-DATA-002 landed the first cleanup pilot in:

```text
lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako
```

It added owner-local context records:

```text
HakoAllocAllocatorComparisonCMimallocExplicitRunnerRunEvidence
HakoAllocAllocatorComparisonCMimallocExplicitRunnerMemoryEvidence
HakoAllocAllocatorComparisonCMimallocExplicitRunnerStopLineEvidence
```

The public evidence entry remains positional for compatibility and to avoid
record-local carrier escape across owner boundaries. Internally, the owner now
constructs context records once and passes them through `reject` / `makeReport`,
reducing repeated argument transport without adding new syntax.

## ARG-DATA-003 to ARG-DATA-008 Outcome

ARG-DATA-003 accepted the narrow Stage1 record construction ergonomics surface:

```hako
record ReportFields {
    accepted: i64 = 0
    reason: i64 = 0
}

local fields = ReportFields {}
fields = fields with {
    accepted: 1,
    reason
}
```

ARG-DATA-004 through ARG-DATA-008 applied that shape to selected
allocator-comparison diagnostic owners. The useful effect was structural:

- ReportFields defaults are visible at the owner-local record boundary;
- omitted scalar fields use the record default rather than repeated explicit
  zeroes;
- `with` makes the update step explicit without mutating the base record;
- ordinary report boxes still use explicit `new Report { field: fields.field }`
  copy helpers.

This is not a broad line-count cleanup by itself. The current task boundary is:

```text
new or touched diagnostic owner:
  use ReportFields defaults and record-only with

large historical owner with real argument pressure:
  split data shape first, then use record ergonomics

parked:
  spread, named args, automatic record-to-box copy, box with
```

## Stop Lines

- No new source syntax in ARG-DATA-001.
- No `...fields` / spread syntax.
- No named argument syntax.
- No automatic record-to-box copy semantics.
- No ordinary-box `with` copy/update.
- No runtime record object materialization.
- No backend route additions.
- No process allocator replacement, hooks, backend matcher additions,
  `#[global_allocator]`, provider package generation, or hidden discovery.

## Validation

ARG-DATA-001 is docs-only and uses a static guard.

ARG-DATA-002 should preserve the existing MIMAP-451A / MIMAP-452A output and
MIR contracts:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh --level L2
```
