# 3385 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CALLER-ORIENTATION-AUTHORITY-DESIGN-STOP-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CALLER-ORIENTATION-AUTHORITY-DESIGN-STOP-001
```

## Purpose

Stop after the ScalarKnown fastpath-connected shadow-consume closeout.

The current state is:

```text
fastpath_connected_closeout = 1
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
caller_orientation_runtime_path = 0
source_selfhost_claim = 0
```

All known ScalarKnown fast-path surfaces now have checked-in generated typed
`.hako` artifacts consumed as shadow evidence at Rust fast-path decision points.
The next step would approach the long-term `.hako` caller orientation where Rust
becomes host oracle / compatibility checker, so authority boundaries must be
decided by design consultation before implementation continues.

## Consultation Question

```text
We have completed the scoped ScalarKnown fastpath-connected closeout:

- 6/6 known ScalarKnown fast-path surfaces consume checked-in generated typed
  .hako artifacts as shadow evidence at the live Rust fast-path decision point.
- Rust remains route authority.
- No runtime .hako source parsing remains in the shadow consumer.
- fastpath_connected_closeout = 1.
- hako_runtime_route_authority = 0.
- rust_fastpath_rewired = 0.
- source_selfhost_claim = 0.

What is the next safe authority step?

A. Keep Rust as route authority and add a fail-fast mismatch gate over all
   ScalarKnown shadow-consumed generated artifacts before any authority switch.

B. Select one narrow ScalarKnown surface, likely MapLoadScalarI64Routes or
   MapStoreI64, and define a .hako-authoritative route decision pilot while Rust
   remains oracle/compat checker.

C. Switch to .hako caller orientation for ScalarKnown as a whole, with Rust
   host/oracle compatibility checks.

D. Do not switch authority yet. Park ScalarKnown after shadow closeout and return
   to wider Source Selfhost lane selection.

Please decide:

1. Should the next step be mismatch-gate hardening, one-surface authority pilot,
   whole ScalarKnown caller orientation, or park/return?
2. If one-surface pilot is allowed, which surface is the safest first authority
   candidate and what proof axis selects it?
3. Which claims are allowed, and which must remain 0?
```

## Non-Claims

```text
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
build_rs_hako_compiler_invocation = 0
live_hako_authority = 0
caller_orientation_runtime_path = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0
```
