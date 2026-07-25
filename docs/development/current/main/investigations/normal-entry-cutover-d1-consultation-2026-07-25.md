---
Status: Closed historical; superseded by NORMAL-FILE-VM0-FAMILY-D0-FORGE-FRONTDOOR
Date: 2026-07-25
Scope: Fresh D1 decision for a possible normal-source entry cutover.
Superseded by: normal-file-vm0-frontdoor-forge-task-2026-07-26.md
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/language-v1-convergence-current.md
  - docs/development/current/main/investigations/normal-entry-cutover-d0-consultation-2026-07-25.md
  - docs/reference/language/function-exit-and-entry-result.md
---

# NORMAL-ENTRY-CUTOVER-D1 consultation

This was the D1 design stop. Its accepted decision authorizes only the
documentation/read-only D0 row in
`normal-file-vm0-forge-task-2026-07-25.md`; it still authorizes no normal
production caller, default cutover, backend, JSON, or executor widening. The previous D0 decision only promoted the explicit
`raw-vm-reference` lane; it did not identify a safe normal caller. A later
accepted forge decision creates a new front door without mapping any legacy
caller; this card remains historical evidence for the earlier park decision.

## Read-only worker inventory

```text
plain source-hint callers       = 6
explicit-import callers         = 6
normal adapters                 = 2
direct build_module bridges    = 2
additional REPL/JSON/Stage1/
  VM fallback/LLVM/WASM routes  = heterogeneous
bounded Raw NarrowV1 family    = 0
```

The caller families mix source preparation, imports, macros, plugins, result
transport, artifact emission, and backend ownership. The closest candidate is
`--emit-mir-json-minimal`, but it is compile-only artifact work and must be a
separate `RAW-MINIMAL-MIR-JSON-PROFILE-D0`, not a normal-entry cutover.

## Evidence boundaries

Current normal source APIs still enter the legacy owner chain:

```text
compile_with_source
  -> compile_legacy
  -> compile_with_source_internal
  -> MirBuilder::build_module
```

The canonical F1 function/Main semantics, Script
`ScriptLastExpressionOrUnit` classifier, selected source-entry continuation,
and typed `SourceEntryResultV1` are not transported through that chain.

Runtime backends also retain independent authorities: VM entry discovery and
status conversion, LLVM/native helper selection and fallback, JSON bridges,
and legacy process adapters. Their current behavior is compatibility evidence,
not normal-entry parity evidence.

## D1 questions to answer

### Q1 — caller family

Choose exactly one route-scoped production caller family, or choose park:

```text
A  exact compile-only MIR-JSON family
B  exact no-import source execution family
C  no admissible family; keep normal cutover parked (recommended)
```

No family may be selected from a repo-wide token count. It must have one
source-preparation owner, one profile, and one observable output contract.

### Q2 — source/profile authority

Fix one typed profile for grammar, source origin, imports, macros, plugins,
REPL, optimization, and unsupported-before-effects behavior. Profile
reconstruction and legacy fallback must be zero.

### Q3 — semantic result scope

Decide whether the candidate is compile-only artifact output or execution.
Compile-only rows make no `SourceEntryResult` or process-status claim.
Execution rows must use the canonical Function/Main/Script result contracts,
exact selected entry, and `ProcessExitProjectionV1`.

### Q4 — function/Main/Script parity

Before cutover, define and pass rows for:

```text
ordinary explicit return and Unit fallthrough
Main explicit return and no implicit tail
Script final expression versus Print/Local/Assignment Unit
empty/void/annotation and unsupported dynamic results
```

Builder-returned last `ValueId`, Print payload, Local binding, or assignment
result is not semantic authority.

### Q5 — entry/backend boundary

Seal one exact source-entry target and physical handoff. `NYASH_ENTRY`, module
scans, LLVM helper selection, generic Box downcasts, and mock/fallback status
conversions are non-authorities. Decide explicitly whether VM, LLVM/AOT, or
artifact-only output is in scope; do not combine them implicitly.

### Q6 — acceptance and failure law

An admissible candidate requires:

```text
route-scoped production caller count             = 1
source/profile authority                         = 1
grammar/backend/artifact-or-result parity       = complete
unsupported-before-effects proof                = 1
compiler reuse: success->success and reject->success = green
fallback/retry                                  = 0
caller census                                   = reproducible
```

Failure must retain the exact unpublished owner and leave the live Builder,
JSON state, and backend state unchanged.

### Q7 — retirement and normal cutover

Select a separate retirement owner for the candidate's old route. Normal
`compile_with_source` and default backend cutover require a later explicit
decision; they are never implied by a green opt-in or compile-only proof.

## Conditional implementation order

Only if D1 accepts a concrete family, issue rows in this order:

```text
CUTOVER-CALLER0
-> SOURCE-PROFILE0
-> FUNCTION-MAIN0
-> SCRIPT-TAIL0
-> ENTRY-TARGET0
-> PROCESS-PROJECTION0
-> BACKEND-PARITY0
-> FAILURE-REUSE0
-> RETIRE-G0
```

Until then, no executable row is selected. `RAW-MINIMAL-MIR-JSON-PROFILE-D0`
is an independent consultation if compile-only MIR JSON is chosen.

## Explicit non-claims

```text
compile_with_source cutover
default backend change
general VM/MIR status-law replacement
LLVM/native/ny_main activation
JSON / Program(JSON v0) changes
executor/selfhost/fastmem activation
App any-statement-tail promotion
old Raw retirement
CUT0
```

The supported `raw-vm-reference` lane remains the only active opt-in runtime
reference lane. It has no temporary sunset; removal requires a new T2 decision,
a successor with exact-entry/result parity, and unique-role caller zero.
