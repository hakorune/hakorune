# ENTRY-RESULT-PROJECTION0-S2 runtime consumer design stop

Decision: `ENTRY-RESULT-PROJECTION0-S2-DESIGN-STOP`
Status: design consultation required; no runtime/backend implementation
authorized

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

## Questions to close

### Q1 — first runtime consumer

Choose exactly one:

```text
A (recommended): VM/reference-only adapter consuming ProcessTerminationV1
B: LLVM/native ny_main adapter
C: explicit public Raw ingress
D: normal compile_with_source cutover
```

Recommendation: A. It provides a semantic reference without changing native
ABI or public routing. B and C require backend/public parity gates first; D is
the broadest migration and must not be the first consumer.

### Q2 — legacy mapping boundary

Does the selected runtime adapter consume only `ProcessTerminationV1`, with
legacy status converters remaining disconnected, or may it inspect
`SourceEntryResultV1` again?

Recommendation: status-only. Route, source result, object handles, and module
symbols must not be reinterpreted after projection.

### Q3 — fault transport

Choose whether a typed `ProcessTerminationV1::Fault` reaches the reference
runner as a structured diagnostic/status pair or is flattened to legacy zero.

Recommendation: structured fault with reserved status 70. Silent zero is
forbidden in the canonical adapter; legacy zero remains a named compatibility
path only.

### Q4 — activation scope

Choose the first permitted caller:

```text
S2 disconnected VM fixture/adapter only (recommended)
VM production runner
LLVM/native harness
public Raw ingress
```

Recommendation: disconnected fixture/adapter only. Production callers,
JSON, executor, selfhost, legacy retirement, and CUT0 remain zero until a
separate parity/cutover decision.

## Non-authority

```text
SourceEntryResultV1 after projection
module symbols or NYASH_ENTRY
legacy VM/LLVM/runner status helpers
positive object-handle decoding
normal compile_with_source
JSON/Program(JSON v0)
```

Candidate row after acceptance:

```text
ENTRY-RESULT-PROJECTION0-S2-VM-REFERENCE-CONSUME0
```

This card is a design stop. Do not add a runtime caller until Q1–Q4 are
accepted and the fault/parity matrix is written.
