# ENTRY-RESULT-PROJECTION0-S2 runtime consumer design stop

Decision: `ENTRY-RESULT-PROJECTION0-S2-VM-REFERENCE-CONSUME-prime-r1`
Status: accepted; implementation is authorized only through the disconnected
VM-reference consumer described below

`ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME0` is complete as a
disconnected carrier-retaining projection. `ProjectedSourceEntryV1` now owns
the physical source-entry carrier and the canonical `ProcessTerminationV1`.
No VM, LLVM/native, public ingress, normal compiler entry, or JSON route has
consumed it.

## Why execution stops here

The next step would be an execution consumer, not another compiler value
carrier. Existing VM, MIR interpreter, LLVM, native `ny_main`, and public
runner paths intentionally have different legacy status mappings. Selecting
one as the first consumer changes activation authority and requires a parity
contract; wiring one opportunistically would create a second process-status
policy.

## Accepted answers

### Q1 — first runtime consumer

Q1 selects exactly one:

```text
A (recommended): VM/reference-only adapter consuming ProcessTerminationV1
B: LLVM/native ny_main adapter
C: explicit public Raw ingress
D: normal compile_with_source cutover
```

Decision: A. It provides a semantic reference without changing native
ABI or public routing. B and C require backend/public parity gates first; D is
the broadest migration and must not be the first consumer.

### Q2 — legacy mapping boundary

Q2: the selected runtime adapter consumes only `ProcessTerminationV1`, with
legacy status converters remaining disconnected, or may it inspect
`SourceEntryResultV1` again?

Decision: status-only. Route, source result, object handles, and module
symbols must not be reinterpreted after projection.

### Q3 — fault transport

Q3: a typed `ProcessTerminationV1::Fault` reaches the reference consumer as a
structured diagnostic/status pair and is never flattened to legacy zero.

Decision: structured fault with reserved status 70. Silent zero is
forbidden in the canonical adapter; legacy zero remains a named compatibility
path only.

### Q4 — activation scope

Q4 permits only:

```text
S2 disconnected VM fixture/adapter only (recommended)
VM production runner
LLVM/native harness
public Raw ingress
```

Decision: disconnected fixture/adapter only. Production callers,
JSON, executor, selfhost, legacy retirement, and CUT0 remain zero until a
separate parity/cutover decision.

## Normalized fault-status authority

The adapter must not invent status 70. `ProcessExitProjectionV1` remains the
sole projection authority and issues a normalized termination:

```text
Exit {
  status
}

Fault {
  status = reserved process-fault status 70
  fault  = exact typed ProcessFaultV1
}
```

Every fault kind carries a normalized status at the termination layer:
out-of-range integer, unsupported process result, and retained source fault.
`ProcessFaultV1` retains diagnostic facts only; it does not independently own
process-status policy.

## One-shot carrier boundary

```text
ProjectedSourceEntryV1
  -> consume_vm_reference(self)
  -> VmReferenceProcessOutcomeV1
       owns the complete ProjectedSourceEntryV1 by value
       exposes normalized status by borrow
       exposes typed fault by borrow
       exposes discard(self)
```

The VM-reference outcome is not an execution engine and has no retry,
fallback, module access, source-result access, or public publication terminal.
`Fault` is a terminal outcome, not an adapter rejection, so the consuming
transition is infallible.

## Non-authority

```text
SourceEntryResultV1 after projection
module symbols or NYASH_ENTRY
legacy VM/LLVM/runner status helpers
positive object-handle decoding
normal compile_with_source
JSON/Program(JSON v0)
```

Accepted execution row:

```text
ENTRY-RESULT-PROJECTION0-S2-VM-REFERENCE-CONSUME0
```

Implementation order:

```text
S2-FAULT-STATUS0
  -> S2-VM-CARRIER0
  -> S2-P0/G0
  -> S3-RUNTIME-ACTIVATION-DESIGN-STOP
```

No production runtime caller may be added in S2.
