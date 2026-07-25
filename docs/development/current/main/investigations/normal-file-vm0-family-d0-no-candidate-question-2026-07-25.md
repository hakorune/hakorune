---
Status: Closed historical / superseded by accepted forge decision
Date: 2026-07-25
Decision needed: NORMAL-FILE-VM0-FAMILY-D0-NO-BOUNDED-CALLER
Related task: normal-file-vm0-forge-task-2026-07-25.md
Superseded by: normal-file-vm0-frontdoor-forge-task-2026-07-26.md
---

# D0 follow-up: no existing caller satisfies the fixed family

The accepted D1 decision fixed the intended family as
`NormalFileNoImportVmReferenceV1`, with D0 selecting one existing plain
source-hint caller. The read-only D0 census is now complete and contradicts
that selection premise:

```text
plain source-hint production sites = 6
candidate family                   = 0
NoBoundedCallerFamily              = sealed
new normal production caller       = 0
```

## Evidence

The six sites are:

```text
bench_vm
bench_jit
verify_outputs_match
execute_mir_json_minimal
Stage-1 binary-only direct route
VM compatibility fallback
```

Their contracts are not interchangeable:

```text
bench_vm / bench_jit / verify_outputs_match
  inline benchmark sources, discarded/timing output, mixed legacy paths

execute_mir_json_minimal
  one file read and parse, MIR JSON artifact only; no SourceEntryResult

Stage-1 direct route
  explicit compatibility bridge; bare MIR/artifact or legacy exit mapping

VM compatibility fallback
  using/preexpand/plugins and independent legacy status conversion
```

All six call the legacy `compile_with_source` / `MirBuilder::build_module`
chain. None consumes the typed Raw compile kernel, a sealed normal source
profile, an exact source-entry continuation, or `ProcessExitProjectionV1`.

The nearest shape, `--emit-mir-json-minimal`, is explicitly artifact-only and
must remain a separate `RAW-MINIMAL-MIR-JSON-PROFILE-D0` lane.

## Decision question

Choose exactly one next policy:

### A — New front-door owner, no legacy caller mapping

Keep `NormalFileNoImportVmReferenceV1`, but reinterpret D0 as a future-owner
forge row rather than a selection from existing callers. Create no production
caller yet. Define the typed `NormalFileRequestV1` / source profile / exact Raw
handoff / reuse proof, then return to D2 for activation authorization.

### B — Select a compile-only artifact lane

Select `execute_mir_json_minimal` only for an independent
`RAW-MINIMAL-MIR-JSON-PROFILE-D0`. It may prove one file read, canonical parse,
NoImports, and MIR artifact parity, but it cannot claim normal execution,
SourceEntryResult, process status, VM parity, or `compile_with_source` parity.

### C — Keep the normal family parked

Keep D0 closed as `NoBoundedCallerFamily` and do not forge a new front door
until a later design decision names the caller owner. Existing legacy callers
remain unchanged; no normal implementation row opens.

## Required answer

```text
Decision: NORMAL-FILE-VM0-FAMILY-D0-{short-name}
Status: accepted | provisional | rejected
Choice: A | B | C
Caller/profile authority:
First executable row:
Required proof:
Retirement/sunset:
Non-claims:
```

Do not choose a legacy caller by convenience, occurrence count, or because it
already reaches `build_module`. Do not add fallback, source rewriting,
`NYASH_ENTRY` search, generic status conversion, or a production caller inside
this D0 closeout.
