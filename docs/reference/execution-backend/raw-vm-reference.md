---
Status: reference contract
Decision: accepted
Date: 2026-07-25
---

# Raw VM-reference lane

`raw-vm-reference` is the explicit, feature-gated semantic reference and
conformance lane for the typed Raw compiler owner chain. It is supported as an
opt-in development/runtime lane; it is not the default compiler route, a
product/native backend, or a distribution artifact.

## Invocation

Build the optional capability and select the backend explicitly:

```bash
cargo build --release --features vm-reference
target/release/hakorune --backend raw-vm-reference program.hako
```

The backend remains default-off. Without `--features vm-reference`, the
explicit selection reports a feature-unavailable usage error. The default
`--backend mir` route and JSON/LLVM routes are unchanged.

## Fixed profile

The production request seals one profile before source I/O or compilation:

```text
grammar       = Canonical
source        = NarrowV1
imports       = None
callable Main = Omitted
execution     = fresh VM, CanonicalProcessExitV1
fallback      = forbidden
```

The lane reads one source file, parses it once, compiles through the shared
typed Raw publication kernel, selects the sealed `Main.main/0` target, and
executes that target in a fresh VM reference instance. It never scans
`NYASH_ENTRY`, performs module-symbol discovery, or re-enters the legacy
compiler path.

## Result and status contract

Source result and process status are separate contracts. Unit/Void maps to
status `0`; Integer values in `0..=255` map exactly to that status. Integer
values outside that range, Bool/Float/String/object results, result ABI
mismatches, and VM faults are typed process faults with reserved status `70`.
They are never silently converted to success or reconstructed from a VM value
after projection.

Usage/profile errors return status `2`; invocation/source/compile/activation
errors return status `1`. Program diagnostics are rendered once from the typed
process fault and do not change the already-sealed status.

## Scope and non-claims

This lane is a semantic reference/conformance owner only. It does not activate
the normal `compile_with_source` cutover, `ny_main`/LLVM/native publication,
JSON or Program(JSON v0), executor/selfhost/fastmem wiring, legacy Raw-chain
retirement, or CUT0. Those are separate decisions and gates.
