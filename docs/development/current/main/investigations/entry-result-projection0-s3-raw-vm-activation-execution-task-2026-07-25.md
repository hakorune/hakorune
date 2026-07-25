# ENTRY-RESULT-PROJECTION0-S3 Raw VM activation execution task

Decision: `ENTRY-RESULT-PROJECTION0-S3-RAW-VM-ACTIVATION-prime-r1`
Status: S3-ENTRY-CARRY0, S3-EXECUTION0, and S3-OWNER0 implemented and guarded;
S3-PARITY0/G0 is the active closeout row. The production route remains
explicit, typed, and isolated from legacy VM/LLVM/public callers.

## Owner chain

```text
RawPublishedInvocationV1
  -> prepare_vm_reference_activation(self)
  -> PreparedRawVmReferenceActivationV1
  -> execute(self)
  -> CompletedRawVmReferenceExecutionV1
       exact SourceEntryResultV1 sealed once
  -> complete_source_entry(self)
  -> ProcessExitProjectionV1 (Canonical V1)
  -> VmReferenceProcessOutcomeV1
  -> prepare_diagnostic(self)
  -> typed RawVmReferenceRunReportV1
```

No stage reopens source AST, route symbols, module inventory, or status policy.

## Internal task order

### S3-ENTRY-CARRY0

- Extract one narrow `SelectedSourceEntryContinuationV1` at the manifest to
  physical transition.
- Retain only brand, Script/App route, typed Main key/symbol/arity target, and
  pairing seal; do not carry the complete manifest/catalog.
- Thread the continuation through post-install, root witness, postprocess,
  and `RawPublishedInvocationCoreV1` by value.
- Make route mismatch a typed preflight failure; route reconstruction after
  manifest selection is forbidden.

### S3-DIAGNOSTIC0

- Add `VmReferenceProcessDiagnosticAdapterV1` in the compiler layer.
- Consume `&ProcessFaultV1` once into a structured report with stable tags:
  `[process/exit-code-out-of-range]`, `[process/unsupported-result]`, and
  `[process/source-fault]`.
- Do not write status, perform I/O, retry, fallback, or flatten to zero.
- Keep exact source-fault code/detail and unsupported kind.

### S3-EXECUTION0

- Add an opaque, consuming VM execution terminal in `src/mir/compiler`.
- Derive exact target from `RawRootBatchSlotV1::Main.contract()`:
  key `Main`, symbol `main`, arity `0`.
- Validate target presence before execution; missing target is activation
  rejection, not a program SourceFault.
- Create a fresh `MirInterpreter` and call only
  `execute_function_with_args(module, "main", &[])`.
- Match `VMValue` directly against a sealed decode plan. Never call
  `to_nyash_box`, `as_integer`, `as_bool`, or downcast Box handles.
- Derive the plan from the retained root exit witness. `ScriptUnitValue`
  (Print/Local/Assignment/CompoundAssignment), empty Unit, and App Unit must
  remain Unit even when the physical payload is an Integer.
- Convert VM errors to typed `SourceEntryResultV1::Fault`; process status is
  assigned only later by `ProcessExitProjectionV1`.
- Exhaustively classify `VMError` variants into stable source-fault codes;
  do not flatten execution faults into a legacy status.
- Gate backend implementation on `vm-reference`; unavailable capability is a
  typed rejection with no fallback.

### S3-OWNER0

- Extract one private `compile_raw_published_v1` kernel from the current full
  `compile_raw_with_source` chain.
- Keep all existing stage owners and typed rejected owners; split error
  vocabulary from orchestration to preserve the 800-line boundary.
- Keep `compile_raw_with_source` as a compatibility adapter that maps rejected
  owners to its existing String transport and consumes the compatibility
  envelope only after publication.
- Add one explicit Raw VM-reference production entry that consumes the typed
  kernel and never calls the legacy compile/build route.
- Keep opaque module opening inside the compiler execution terminal. Runner
  receives only a typed report/status surface.

### S3-PARITY0/G0

- Run actual Raw compile plus fresh exact VM execution for Script/App through
  the shared `compile_raw_published_v1` owner.
- Cover empty, explicit void, integer 0/255, out-of-range integer, Bool,
  Float, String, Print, Local, assignment, compound assignment, VM faults,
  and decode ABI mismatch.
- Prove decoy `NYASH_ENTRY` and extra callable symbols do not alter Main/main/0
  selection.
- Prove compile reuse after success, compile rejection, process fault, and VM
  execution fault.
- Add structural guard and exact caller census.
- Keep every modified/new source and check file below 800 lines.

Current evidence (partial closeout): actual execution tests now cover empty
Script, empty App Main, integer 0/255, integer range faults, Bool/Float/String
unsupported process results, Print-as-Unit, division VM faults, and compiler
reuse after success or entry rejection. The full matrix, decode ABI mismatch,
decoy-entry census, and final caller census remain open under this row.

## Failure law

```text
compile/activation rejection
  -> exact owner + stage + typed cause + discard(self)

VM execution error
  -> SourceEntryResultV1::Fault
  -> canonical ProcessTerminationV1::Fault(status=70)
```

These are distinct: failure to start execution is a rejection; a running
program fault is a typed source result. No retry, fallback, status rewrite, or
legacy re-entry exists on either path.

## Structural guard contract

```text
compile_raw_published_v1 definition                 = 1
compatibility adapter authority erasure             = 1
run_raw_vm_reference production entry               = 1
selected-entry continuation producer                = 1
exact execute_function_with_args caller              = 1
VmReferenceProcessOutcomeV1 production consumer      = 1
diagnostic adapter status writes                     = 0

execute_module / NYASH_ENTRY in new route            = 0
module scan / route reconstruction                   = 0
legacy status helper / build_module fallback         = 0
VMValue direct result inference                      = 0
Box coercion / handle decoding                       = 0
normal compile / JSON / LLVM / ny_main widening      = 0
process::exit in compiler adapter                    = 0
all modified/new source/check files                  < 800
```

## Non-claims

```text
general VM/MIR runner status-law replacement
LLVM/native ny_main activation
normal compile_with_source cutover
JSON / Program(JSON v0)
executor / selfhost / fastmem
legacy runner retirement
object/handle/dynamic result ABI
interpreter-session reuse
CUT0
```

After S3 closeout, stop at the next separately measured backend/public
activation decision; do not widen the existing runner implicitly.
