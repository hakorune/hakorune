# ENTRY-RESULT-PROJECTION0-D0

Decision: `ENTRY-RESULT-PROJECTION0-D0`

Status: closed design stop; implementation authorization = `ENTRY-RESULT-PROJECTION0-S0` only

Closeout: `ENTRY-RESULT-PROJECTION0-D0-CLOSEOUT-20260725`

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

### Observed census at closeout

The required inventory was checked against the current tree. The named
`src/backend/vm_execution.rs` surface does not exist; its live equivalent is
`src/runner/modes/common_util/vm_execution.rs`.

| Surface / symbol | Observed input -> output | Current behavior | Classification |
| --- | --- | --- | --- |
| `src/backend/mir_interpreter/mod.rs` / `MirInterpreter::execute_module` | `&MirModule -> Result<Box<dyn NyashBox>, VMError>` | Performs its own `NYASH_ENTRY`, `Main.main/0`, `Main.main`, and optional top-level `main` search; returns the selected function's boxed evaluation result. | **Legacy execution authority** for entry selection and evaluation; split before canonical S0. |
| `src/runner/modes/common_util/vm_execution.rs` / `run_vm_compiled_module` | boxed VM result -> `i32` + `process::exit` | Integer is cast directly to `i32`, Bool maps to 1/0, every other Box maps silently to 0. | **Legacy process adapter**; duplicated status policy, not canonical authority. |
| `src/runner/modes/mir_interpreter.rs` / `execute_mir_interpreter_mode` | boxed interpreter result -> `i32` + `process::exit` | Repeats Integer/Bool mapping and maps all other values to 0. | **Legacy process adapter**; direct production caller. |
| `src/runner/modes/common_util/entry_selection.rs` / `select_entry_function` | `&MirModule -> String` | Selects `Main.main/0`, then arity-less `Main.main`, then optional top-level `main`; used by legacy PyVM and LLVM mock. | **Legacy selection helper**; incomplete because the interpreter duplicates it and `NYASH_ENTRY` bypasses it. |
| `src/runner/product/llvm/fallback_executor.rs` / `FallbackExecutorBox::execute` | `&MirModule -> Result<i32, LlvmRunError>` | Uses the legacy selection helper; returns mock 42 for any value-bearing Return and 0 for void/no match. | **Mock diagnostic transport**; 42/0 is not language or process semantics. |
| `src/mir/join_ir_vm_bridge_dispatch/exec_routes.rs` / `try_run_skip_ws` | `JoinValue -> i32` + optional `process::exit` | Int is cast, Bool maps to 1/0, all other JoinValues map to 0; dev mode may continue instead of exiting. | **Experimental route transport**; never a source-result authority. |
| `src/runner/hv1_inline.rs` / `run_json_v1_inline` | MIR JSON v1 -> `i32` | Test/dev inline executor returns numeric register values and uses 1 for parse/shape/step failure; shell wrappers may modulo-normalize the result. | **HV1 compatibility transport**; JSON input is outside the source-entry authority. |
| `src/runner/modes/common_util/legacy/pyvm.rs` / `run_pyvm_harness_lib` | selected entry + child status -> `Result<i32, String>` | Uses the legacy selection helper and returns the external PyVM process status. | **Historical transport**; disconnected from canonical S0. |
| `src/llvm_py/builders/entry.py` / `ensure_ny_main` | MIR/LLVM functions -> `ny_main() -> i64` | Chooses `Main.main/1` or plain `main`, manufactures an argv handle, returns integer-like results, otherwise silently returns i64 0. | **Legacy AOT entry adapter**; `ny_main` is transport ABI, not source-result authority. |
| `tools/compat/native_llvm_builder.py`, `tools/llvmlite_harness.py` | MIR/LLVM JSON -> native `ny_main` object | Emits minimal `ny_main` and defaults missing/unsupported paths to 0. | **Harness/mock transport**; no canonical status policy. |
| `src/abi/nyrt_shim.rs` / `nyrt_exec_main` | module handle -> `i32` | Current shim is a stub returning 0; it does not own source-result projection. | **Runtime stub transport**; no S0 authority. |
| `src/runner/json_v0_bridge.rs`, `src/runner/json_artifact/program_json_v0_loader.rs` | JSON/Program(JSON v0) -> `MirModule` | Parses/loads compatibility artifacts; no new Raw source-entry result is produced. | **Explicitly unchanged compatibility route**. |
| `src/runner/dispatch.rs`, `src/main.rs`, `src/runner/modes/common.rs` | runner/CLI errors -> OS status | Calls `process::exit` for parse, usage, configuration, and backend failures. | **Tool failure transport**, outside program-result projection. |

The census makes two duplicated decisions explicit:

1. Entry selection is split between `MirInterpreter::execute_module`, the
   common helper, LLVM's `ensure_ny_main`, and stage-1 environment injection.
2. Process status conversion is split between VM/MIR-interpreter, JoinIR,
   LLVM mock/AOT, and child-process transports. Their current 0/1/42/cast
   behavior is compatibility evidence, not a second language specification.

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

### D0 profile decision

The profile vocabulary is now fixed before S0:

```text
CanonicalProcessExitV1
  Unit/Void                  -> status 0
  Integer 0..=255            -> exact status
  Integer outside the range -> ProcessExitOutOfRange Fault
  Bool/Float/String/object   -> UnsupportedProcessResult Fault
  program Fault              -> reserved status 70 plus diagnostic

LegacyRunnerExitProjectionV1
  retains current VM/MIR-interpreter compatibility while its callers are
  measured and retired; it is never selected implicitly by canonical S0.
```

No modulo wrapping, Box-to-positive-handle inference, or non-integer silent
success is allowed in the canonical profile. The existing 0/1/42 behavior is
named in the census and remains unchanged until a later adapter/cutover row.

## Q4 — physical entry boundary

`physical main` is a typed source-result transport role, not an independent
language-semantic owner. The D0 artifact must decide whether the first S0 uses
one source-entry thunk or an existing route-specific transport, and must show
how `ny_main() -> i64` receives only normalized process status. Native `main`
may perform only checked OS adaptation.

### D0 thunk decision

S0 selects one typed source-entry thunk boundary:

```text
SelectedSourceEntryV1
  -> SourceEntryThunkV1
  -> SourceEntryResultV1
  -> ProcessExitProjectionV1::project(CanonicalProcessExitV1)
  -> normalized ny_main status
  -> checked native OS adaptation
```

The thunk is the only canonical transport from a selected source entry to a
backend-facing result. Existing VM, JoinIR, LLVM, and PyVM paths remain
legacy adapters until their callers are explicitly migrated. `ny_main` never
receives a generic Box or a positive handle; it receives only the normalized
process status.

Entry selection itself is sealed once as `SelectedSourceEntryV1`. The current
module-scanning helpers and `NYASH_ENTRY` branches are not allowed to infer a
new source route after that seal.

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

### S0 implementation scope

`ENTRY-RESULT-PROJECTION0-S0` is authorized only in this order:

```text
CONTRACT0
  SourceEntryResultV1 / ProcessTerminationV1 / profile vocabulary

ENTRY-SELECTION0
  one SelectedSourceEntryV1 producer; no backend re-selection

SOURCE-ENTRY0
  typed source-entry thunk and Unit/scalar/Fault transport

PHYSICAL-THUNK0
  one backend-neutral thunk handoff; no public cutover

VM-REFERENCE0
  reference projection fixture; legacy runner stays disconnected

EXE-AOT0
  explicit capability adapter; no LLVM mock normalization

PARITY-G0
  caller census, legacy profile, and no-silent-fallback guard
```

The S0 row must not change parser grammar, App policy, ordinary
`compile_with_source`, JSON/Program(JSON v0), executor wiring, selfhost,
fastmem, old Raw retirement, or CUT0. Unsupported process projections must
reject at the process boundary with typed evidence; they must not fall back to
the legacy runner or to status 0.

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
The observed census, profile decision, and thunk decision above close D0.
No Rust behavior change is included in this closeout; the next executable row
is the separately tracked S0 task below.

## Next order

```text
ENTRY-RESULT-PROJECTION0-D0 (closed)
  -> ENTRY-RESULT-PROJECTION0-S0 (next)
       CONTRACT0 -> ENTRY-SELECTION0 -> SOURCE-ENTRY0
       -> PHYSICAL-THUNK0 -> VM-REFERENCE0 -> EXE-AOT0 -> PARITY-G0
```

Still parked: App compatibility parity, normal-entry cutover, JSON changes,
executor/selfhost/fastmem activation, old Raw retirement, and CUT0.

## Next executable task

`docs/development/current/main/investigations/entry-result-projection0-s0-execution-task-2026-07-25.md`
