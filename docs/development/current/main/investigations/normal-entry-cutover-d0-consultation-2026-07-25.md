# NORMAL-ENTRY-CUTOVER-D0 consultation

Decision: `NORMAL-ENTRY-CUTOVER-D0` — design stop opened after the explicit
Raw VM-reference canary parity closeout. No default-route or normal-entry
implementation is authorized by this document.

## Current evidence

The explicit `--backend raw-vm-reference` canary is a bounded, default-off
reference lane. Its real-binary proof family is green for the accepted
NarrowV1 grammar and proves:

```text
16 semantic fixtures
parse and compile rejection status mapping
missing-source and feature-disabled status mapping
profile conflicts
sealed Main target under NYASH_ENTRY=decoy
default mir route preservation
```

The canary does not prove parity for the normal compiler surface. NarrowV1
still rejects imports, macros, REPL, JSON, helper widening, control flow,
calls, objects, async, and fastmem. The normal source inventory also spans
MIR, VM, VM-hako, LLVM, WASM, bench, Stage-1, selfhost, and JSON bridges.

## The one question to answer before implementation

What does “normal-entry cutover” mean for this repository?

Choose exactly one target surface:

```text
A  keep the canary as a supported opt-in reference lane
B  reroute one bounded no-import caller family
C  change compile_with_source
D  change the default CLI backend
E  park cutover and keep the canary as explicit evidence only
```

The choice must name the exact caller set, grammar/profile capability,
source preparation owner, entry-result policy, backend owner, fallback law,
and retirement condition. A canary result alone does not authorize B, C, or
D because it does not cover the broader normal source and backend contract.

## Required D0 inventory

Before any executable row is selected, record:

```text
normal compile_with_source production callers
no-import versus import-aware callers
compile_raw_with_source callers
JSON/Program(JSON v0) callers
VM/VM-hako/LLVM/WASM/Stage-1/selfhost callers
grammar and source-preparation differences
entry-result and process-exit differences
fallback or legacy status conversions
one candidate bounded target, or an explicit PARK decision
```

The following remain forbidden during D0:

```text
default backend change
compile_with_source body change
JSON or executor widening
LLVM/native/ny_main activation
legacy fallback
caller-selected hidden policy
```

## Exit conditions

```text
one explicit cutover target is named
source/profile/backend/entry-result authorities are separate
unsupported callers fail before effects
no fallback is structurally proven
caller census is reproducible
parity matrix and retirement row are written
implementation authorization is explicitly granted in a new decision
```

Until these conditions are met, the correct status is `PARK` and the explicit
canary remains the only production-shaped Raw consumer.
