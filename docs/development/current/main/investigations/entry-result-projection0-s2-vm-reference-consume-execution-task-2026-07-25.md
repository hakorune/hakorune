# ENTRY-RESULT-PROJECTION0-S2 VM-reference consume execution task

Decision: `ENTRY-RESULT-PROJECTION0-S2-VM-REFERENCE-CONSUME-prime-r1`
Status: implementation authorized for one disconnected compiler-internal
VM-reference carrier; production activation remains forbidden

## Objective

Consume `ProjectedSourceEntryV1` exactly once without reopening source-result
semantics or selecting a production runner:

```text
ProjectedSourceEntryV1
  -> consume_vm_reference(self)
  -> VmReferenceProcessOutcomeV1
       complete projected owner retained
       normalized status view
       exact typed fault view
       discard-only terminal
```

This is a BoxShape row. It adds no grammar, backend capability, public route,
or execution behavior.

## Internal task order

### S2-FAULT-STATUS0

- Make every `ProcessTerminationV1::Fault` carry the normalized process status.
- `ProcessExitProjectionV1` is the sole producer of reserved status 70.
- Remove process-status ownership from `ProcessFaultV1::SourceFault`.
- Add borrowed `status_code()` and `fault()` views.
- Preserve exact out-of-range value, unsupported result kind, and source
  fault code/detail.

### S2-VM-CARRIER0

- Add `src/mir/compiler/source_entry_vm_reference.rs`.
- Add one consuming `ProjectedSourceEntryV1::consume_vm_reference(self)`.
- Retain the complete projected owner by value in a non-Clone opaque carrier.
- Expose normalized status and typed fault only by borrow.
- Add `discard(self)` as the only terminal.

### S2-P0/G0

- Put focused fixtures in a separate test-only module.
- Prove exact exit 0 and 255.
- Prove every fault status is 70 with exact typed diagnostic facts.
- Prove Script/App carrier evidence remains retained.
- Add a structural guard with production consumer count zero.
- Keep every modified/new source or check file below 800 lines.

## Authority and non-authority

Authority:

```text
ProcessExitProjectionV1        = result-to-status/fault projection
ProcessTerminationV1          = normalized terminal relation
ProjectedSourceEntryV1        = carrier + termination co-owner
VmReferenceProcessOutcomeV1   = one-shot VM-reference handoff owner
```

Non-authority:

```text
SourceEntryResultV1 after projection
PhysicalSourceEntryCarrierV1 internals
module symbol / NYASH_ENTRY
legacy runner status converters
positive object-handle decoding
MirInterpreter or VM execution
```

## Acceptance matrix

```text
Script Unit
  -> status 0
  -> projected owner retained

App Integer(0)
  -> status 0

App Integer(255)
  -> status 255

Integer(-1 / 256)
  -> status 70
  -> ExitCodeOutOfRange retains exact value

Bool / Float / String / Object
  -> status 70
  -> UnsupportedProcessResult retains exact kind

source Fault
  -> status 70
  -> exact code/detail retained
```

For every row:

```text
source-result re-observation  = 0
route inference               = 0
legacy status converter       = 0
actual VM execution           = 0
public/backend caller         = 0
retry/fallback                = 0
```

## Structural guard

```text
consume_vm_reference definition                  = 1
VmReferenceProcessOutcomeV1 owns projected owner = 1
ProcessTerminationV1 normalized status owner     = 1

SourceEntryResultV1 in adapter                    = 0
ProcessExitProjectionV1 rerun                     = 0
carrier.result access                             = 0
module / NYASH_ENTRY inspection                   = 0
legacy helper/status mapping                      = 0
fault flattening / to_string                      = 0
status-zero fallback                              = 0
process::exit                                     = 0
run_vm_compiled_module                            = 0
production consume_vm_reference caller            = 0
all modified/new source/check files               < 800
```

## Closeout

After S2 is green, advance to:

```text
ENTRY-RESULT-PROJECTION0-S3-RUNTIME-ACTIVATION-DESIGN-STOP
```

Do not choose VM production, LLVM/native, public Raw ingress, or normal-entry
activation without that separate decision.

## Non-claims

```text
VM production runner
LLVM/native ny_main
public Raw ingress
normal compile_with_source
JSON / Program(JSON v0)
executor / selfhost / fastmem
legacy retirement
CUT0
```
