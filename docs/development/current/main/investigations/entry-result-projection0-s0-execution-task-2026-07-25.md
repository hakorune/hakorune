# ENTRY-RESULT-PROJECTION0-S0

Decision: `ENTRY-RESULT-PROJECTION0-S0`

Status: `CONTRACT0` closed; next sub-row is `ENTRY-SELECTION0`

## Objective

Introduce the typed source-entry/process-result vocabulary and one disconnected
prepare-only transport seam. This row must not change the normal compiler
entry, JSON routes, executor wiring, or native publication behavior.

## Authority chain

```text
SelectedSourceEntryV1
  -> SourceEntryThunkV1
  -> SourceEntryResultV1
  -> ProcessExitProjectionV1::project(CanonicalProcessExitV1)
  -> ProcessTerminationV1
```

Only `SourceEntryResultV1` owns source evaluation results. Only
`ProcessExitProjectionV1` may convert a sealed source result to process status.
Existing VM/MIR-interpreter/JoinIR/LLVM/PyVM conversions remain named legacy
adapters and are not widened by this row.

## Contract vocabulary

```rust
enum SourceEntryResultV1 {
    Unit(UnitOriginV1),
    Integer(i64),
    Bool(bool),
    Float(f64),
    String(Box<str>),
    Object(SealedObjectResultV1),
    Fault(SealedSourceFaultV1),
}

enum ProcessTerminationV1 {
    Exit(ProcessExitCodeV1),
    Fault(ProcessFaultV1),
}

enum CanonicalProcessExitV1 {
    UnitZero,
    IntegerByte,
    FaultReserved70,
}
```

The exact public Rust shape may use private sealed carriers, but it must retain
the same authority split. `ProcessExitCodeV1` is bounded to `0..=255`; no
wrapping or positive-handle interpretation is allowed.

Canonical projection:

```text
Unit/Void              -> Exit(0)
Integer 0..=255        -> Exit(exact value)
Integer outside range  -> Fault(ExitCodeOutOfRange)
Bool/Float/String/obj  -> Fault(UnsupportedProcessResult)
source Fault           -> Fault(reserved=70, diagnostic retained)
```

The legacy runner profile is `LegacyRunnerExitProjectionV1`, an explicitly
named compatibility fixture only; it is never selected implicitly by the new
seam.

## S0 implementation order

### CONTRACT0

Add the private contract module and its unit tests. Define Unit provenance,
bounded process status, typed unsupported/out-of-range faults, and the profile
name. Do not call `process::exit` here.

### ENTRY-SELECTION0

Add one compiler-internal producer of `SelectedSourceEntryV1`. It consumes the
already selected route evidence and seals the function identity once. Do not
re-read module symbols in a backend. Existing `NYASH_ENTRY` and helper-based
selection remain disconnected compatibility paths.

### SOURCE-ENTRY0

Create a source-entry thunk contract that transports the selected callable's
sealed result as `SourceEntryResultV1`. The thunk must not infer route from
`main`, `Main.main`, module name, or Box shape. No public ingress is added.

### PHYSICAL-THUNK0

Add one Builder/backend-neutral handoff from the source-result carrier to a
physical entry carrier. It may be opaque and non-Clone; it must not expose a
bare mutable `MirModule` or a process status before projection.

### VM-REFERENCE0

Add reference fixtures for Unit, byte-range Integer, out-of-range Integer,
unsupported scalar/object, and Fault. These fixtures consume the new pure
projection only; they do not replace `run_vm_compiled_module`.

### EXE-AOT0

Add an explicit capability adapter boundary for `ny_main`. It receives only
`ProcessExitCodeV1`/normalized status. LLVM mock 42/0 behavior and existing
`ensure_ny_main` behavior remain unchanged and are recorded as legacy.

### PARITY-G0

Add structural checks for one projection owner, one selection producer, zero
new backend status converters, zero fallback from unsupported results, and
zero changes to JSON/normal entry callers.

## Failure and ownership law

All projection failures are typed and occur before any OS exit or live Builder
publication. Rejected owners expose inspection plus `discard(self)` only.
There is no retry, fallback to status 0, positive-handle decoding, or legacy
re-entry. The adapter may convert a typed rejection to an existing diagnostic
transport only after consuming the rejection.

## Acceptance matrix

```text
Unit/Void              -> Exit(0)
Integer(0)             -> Exit(0)
Integer(255)           -> Exit(255)
Integer(-1/256)        -> typed range Fault
Bool/Float/String/obj  -> typed unsupported Fault
source Fault           -> reserved 70 Fault

same compiler reuse:
  success -> success
  reject  -> success

normal compile_with_source caller delta = 0
JSON/Program(JSON v0) caller delta       = 0
executor/selfhost/fastmem/CUT0 delta     = 0
legacy converter new callers             = 0
```

## Guard and file budget

Recommended files:

```text
src/mir/compiler/source_entry_result.rs
src/mir/compiler/source_entry_selection.rs
src/mir/compiler/source_entry_thunk.rs
tools/checks/lib/entry_result_projection0_s0_guard.py
```

Keep each new/modified source and check file below 800 lines. The guard must
prove the single authority chain, typed rejection, no process exit in the
contract module, and no new legacy/backend caller.

## Non-claims

```text
normal compile_with_source cutover
public compile_raw_with_source activation
JSON/Program(JSON v0) changes
executor/selfhost/fastmem activation
LLVM mock normalization
old Raw-chain retirement
App compatibility parity
dynamic result carrier activation
native OS ABI rewrite
CUT0
```

## CONTRACT0 closeout

Implemented in `src/mir/compiler/source_entry_result.rs` with the module
registration in `src/mir/compiler/mod.rs`. The pure projection has no process
exit or backend caller. Focused evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib source_entry_result -- --test-threads=1
  5 passed

python3 tools/checks/lib/entry_result_projection0_contract0_guard.py
  one_projection=1 typed_faults=1 no_exit=1 legacy_callers=0 below_800=1
```

`ENTRY-SELECTION0` is the next implementation sub-row. It must consume sealed
route evidence once and must not re-use the backend-local selection helpers as
a new authority.

## ENTRY-SELECTION0 closeout

`SelectedSourceEntryV1` now consumes `RawRootEnvironmentManifestV1` once and
seals exactly `Script` or `AppMain0`. The manifest remains inside the selected
owner for the next thunk handoff. No module scan, `NYASH_ENTRY`, backend entry
helper, or process exit is reachable from this producer.

Focused evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib source_entry_selection -- --test-threads=1
  2 passed

python3 tools/checks/lib/entry_result_projection0_selection0_guard.py
  one_producer=1 sealed_manifest=1 no_backend_scan=1 no_exit=1 below_800=1
```

`SOURCE-ENTRY0` is the next implementation sub-row. It must transport this
typed selection to a source result without reopening route selection.
