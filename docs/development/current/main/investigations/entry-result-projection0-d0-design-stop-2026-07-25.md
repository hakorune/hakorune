# ENTRY-RESULT-PROJECTION0-D0

Decision: `ENTRY-RESULT-PROJECTION0-D0`

Status: accepted design stop; implementation authorization = none

## Purpose

This card is the required design stop after `SCRIPT-RESULT-TAIL0-S0`.
It inventories every existing source-entry and process-status producer before
any shared exit-code helper, physical entry thunk, or normal-entry cutover is
implemented. Evaluation results and process status must remain different
typed products.

## Q1 — result authority

The semantic boundary is fixed as:

```text
SelectedSourceEntryV1
  -> SourceEntryResultV1
  -> ProcessExitProjectionV1
  -> ProcessTerminationV1
  -> normalized ny_main() status
  -> native OS status
```

`SourceEntryResultV1` may retain Unit, exact scalar values, or a typed Fault.
`ProcessTerminationV1` owns only normalized process status or process Fault.
No generic `Box -> exit code`, positive-handle heuristic, or module-symbol
inference may become a new authority.

## Q2 — mandatory caller census

The D0 artifact must classify each producer and direct caller, not merely grep
for a helper name. At minimum the inventory covers:

| Surface | Required evidence |
| --- | --- |
| `src/backend/vm_execution.rs` | source-result extraction and status conversion |
| standalone MIR interpreter | `MirInterpreter::execute_module` entry selection and result mapping |
| quiet-MIR runner | modulo/truncation/Float/String behavior and caller |
| `src/runner/modes/common_util/entry_selection.rs` | every duplicated entry-selection rule |
| JoinIR bridge | source result or status ownership |
| HV1 inline runner | result transport and status conversion |
| native `ny_main` / NyRT | positive-`i64` handle heuristic and normalized status |
| LLVM harness and mock | integer return path and 42/0 fallback |
| historical PyVM | status transport, marked non-authoritative if disconnected |
| public JSON / Program(JSON v0) | explicit unchanged route and zero new Raw authority |

Each row records path, symbol, input type, output type, fallback behavior,
production/test scope, and whether it is an authority or a transport adapter.

## Q3 — selected policy owner

The eventual S0 owner is one pure `ProcessExitProjectionV1::project` over a
sealed `SourceEntryResultV1` and an explicitly selected process profile. It is
not a function of backend, module symbol, or runtime Box shape.

The D0 decision must select and name the profile vocabulary, including:

```text
Unit/Void disposition
Integer range and out-of-range Fault
Bool/Float/String/object disposition
program Fault status
legacy runner compatibility profile and sunset
```

Until that profile is accepted, no production status conversion is changed.

## Q4 — physical entry boundary

`physical main` is a typed source-result transport role, not an independent
language-semantic owner. The D0 artifact must decide whether the first S0 uses
one source-entry thunk or an existing route-specific transport, and must show
how `ny_main() -> i64` receives only normalized process status. Native `main`
may perform only checked OS adaptation.

## Q5 — parity and fail-fast

The inventory must separate:

```text
source/function/Main completion semantics
Script result semantics
process-exit projection
backend capability
```

Unsupported process projections fail explicitly at the process boundary.
Existing JSON, executor, selfhost, and normal `compile_with_source` behavior
remain unchanged until a later selected profile/canary row.

## Deliverable and acceptance

D0 produces one checked-in inventory/design artifact and no Rust behavior
change. It closes only when:

```text
all required caller surfaces have one census row
duplicated entry selection is explicitly listed
authority vs transport is decided for every producer
legacy modulo/handle/mock behavior is named, not silently normalized
ProcessExitProjection owner and profile vocabulary are selected
physical thunk boundary is selected
S0 implementation scope and non-claims are explicit
```

Required checks are docs layout/index/pointer checks and `git diff --check`.
No new executable row may start from this card alone.

## Next order

```text
ENTRY-RESULT-PROJECTION0-D0
  -> ENTRY-RESULT-PROJECTION0-S0
       CONTRACT0 -> ENTRY-SELECTION0 -> SOURCE-ENTRY0
       -> PHYSICAL-THUNK0 -> VM-REFERENCE0 -> EXE-AOT0 -> PARITY-G0
```

Still parked: App compatibility parity, normal-entry cutover, JSON changes,
executor/selfhost/fastmem activation, old Raw retirement, and CUT0.
