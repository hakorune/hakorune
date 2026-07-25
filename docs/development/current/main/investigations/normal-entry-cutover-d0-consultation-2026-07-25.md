---
Status: Accepted Design
Date: 2026-07-25
Scope: NORMAL-ENTRY-CUTOVER-D0 target selection and implementation boundary.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/post-s3-clean-retire-and-normal-entry-canary-task-map-2026-07-25.md
  - docs/development/current/main/investigations/raw-vm-reference-support0-s0-execution-task-2026-07-25.md
  - docs/reference/language/function-exit-and-entry-result.md
---

# NORMAL-ENTRY-CUTOVER-D0 decision

```text
Decision: NORMAL-ENTRY-CUTOVER-prime-r1
Status: accepted design

selected target:
  A-prime — promote the existing explicit Raw VM-reference route to a
  supported opt-in reference/conformance lane

normal compile_with_source cutover:
  PARK

default CLI backend cutover:
  PARK

first executable row:
  RAW-VM-REFERENCE-SUPPORT0-S0
```

This decision does not reinterpret the explicit route as the new normal
compiler. It gives the already-public, already-proven route an honest durable
role while keeping every heterogeneous normal caller on its current owner.

## Worker inventory

Six read-only audits covered runner/CLI, caller families, source/profile
capabilities, JSON/executor, LLVM/native, guards/proofs, and sunset ownership.

```text
worker_inventory = consumed
worker_inventory_scope =
  current HEAD callers, source preparation, backend/result policy,
  proof manifests, and retirement metadata
```

The worktree was clean during the audit. Existing pointer and S3 owner guards
were green.

## Exact current caller census

### Raw reference surface

```text
CLI default backend                              = mir
raw-vm-reference early selector definitions      = 1
raw-vm-reference early selector production calls = 1
run_raw_vm_reference definitions                 = 1
run_raw_vm_reference non-test calls              = 1
compile_raw_with_source definitions              = 1
compile_raw_with_source non-test calls            = 0
compile_raw_published_v1 definitions              = 1
compile_raw_published_v1 consumers                = 2
  compatibility adapter
  VM-reference execution
```

The explicit lane already owns:

```text
one file read
one Canonical parse
Raw NarrowV1 compile/publication
sealed Main/main/0 target
fresh Rust MirInterpreter
source-result decode from retained exit evidence
Canonical process projection
typed diagnostic projection
status 0 / 0..=255 / reserved fault 70
fallback zero
```

Its real-binary proof covers sixteen semantic cases plus parse, compile,
missing-file, feature-disabled, profile-conflict, decoy-entry, and default
route isolation.

### Normal compiler surface

```text
plain source-hint call sites        = 6
explicit-import call sites          = 6
normal compile adapters             = 2
direct production build_module      = 2
compile_legacy production owners    = 3
```

The plain-wrapper sites are not one semantic family:

```text
bench x3:
  current CLI path is inactive; grammar/result behavior is legacy

VM fallback:
  using text merge, macro/plugin setup, and fallback semantics occur first

Stage-1 direct:
  broad selfhost grammar, macro expansion, legacy result/status owner

minimal MIR-JSON emit:
  using is rejected, but macro expansion and artifact projection remain;
  Raw currently rejects emit routes
```

The import-aware sites span default MIR, VM keep, MIR interpreter, VM-Hako,
LLVM, and WASM. REPL and Program(JSON v0) also enter `compile_legacy`
directly. Runtime AST-JSON owns a direct `MirBuilder::build_module` bridge.

Therefore no current “no-import caller family” is an exact bounded target.

## Candidate decision

### A — selected as A-prime

Keep the exact CLI spelling:

```text
--backend raw-vm-reference
```

Reclassify it from a temporary canary into a supported opt-in
reference/conformance lane with an explicitly narrow capability contract.

This adds no production caller, grammar, backend, or result authority. It
repays the temporary canary status instead of leaving an exposed route
permanently provisional.

### B — parked

No safe existing bounded caller family was found.

The closest future candidate is `--emit-mir-json-minimal`, but it requires a
separate compile-only profile decision for:

```text
macro-off policy
exact admitted grammar
MIR-JSON artifact parity
compatibility projection
no execution/process-result claim
```

Its future design row is:

```text
RAW-MINIMAL-MIR-JSON-PROFILE-D0
```

### C — rejected for this decision

Changing `compile_with_source` would simultaneously affect heterogeneous
bench, fallback, Stage-1, selfhost, and adapter callers. Raw NarrowV1 does not
cover their source preparation or grammar.

### D — rejected for this decision

Changing the default CLI backend would send normal using, macro, JSON,
emit/build, plugin, object, call, control-flow, and backend requests into the
early NarrowV1 fail-fast boundary. The default remains `mir`.

### E — not selected

Pure parking is safe but leaves an already-public production-shaped route and
its sunset debt labeled provisional. A-prime provides a truthful durable role
without widening it.

## Supported lane contract

```text
entry:
  explicit --backend raw-vm-reference only

build capability:
  --features vm-reference required
  feature remains default-off

source:
  one file
  Canonical parser
  Raw NarrowV1
  no text merge or AST rewrite

imports/macros/REPL/JSON:
  unsupported

callable Main:
  Omitted policy

backend:
  fresh Rust MirInterpreter
  exact sealed target; no NYASH_ENTRY or module-scan selection

process:
  CanonicalProcessExitV1

optimization:
  existing --no-optimize snapshot only

fallback:
  forbidden

default route:
  unchanged mir
```

Unsupported profile facts reject before file/source effects at the selector.
Unsupported source shapes reject in the source-only eligibility/manifest
preflight before the isolated physical session opens. No rejection retries
Legacy compilation.

## Status contract

```text
profile/capability/missing-source failure = 2
parse/compile/activation failure          = 1
program Unit                              = 0
program Integer 0..=255                   = exact status
program range/unsupported/source fault    = 70 + typed diagnostic
diagnostic delivery failure               = status unchanged
```

## Source authority

```text
CLI selector:
  selects one typed Raw VM-reference profile

Canonical parser:
  owns source syntax

Raw source/eligibility/manifest:
  owns NarrowV1 admission and source-derived body recipe

selected-entry continuation:
  owns exact source entry identity

Raw postprocess/publication:
  owns opaque module and evidence chain

VM-reference execution:
  owns exact VM target and source-result decode

ProcessExitProjectionV1:
  owns process status
```

## Non-authorities

```text
backend spelling
function/module symbol scan
NYASH_ENTRY
Builder last ValueId
physical return payload alone
compatibility-erased MirCompileResult
JSON module inventory
legacy runner status helpers
```

## Sunset decision

Close:

```text
sunset_id = NORMAL-ENTRY-CANARY-SUNSET-001
retirement_owner = RAW-VM-REFERENCE-SUPPORT0
sunset_row = RAW-VM-REFERENCE-SUPPORT0-S0 / CANARY-SUNSET0
```

The prior task map already allowed repayment when an accepted reference-runner
decision removed the route from canary status. The selector, runner shell, and
subprocess matrix are retained and renamed/reclassified as durable supported
conformance evidence.

```text
new proofs = 0
retired or renamed proofs = existing canary proof family
net proof delta = 0
```

The supported reference lane itself has no temporary sunset. Removing it later
requires a new T2 decision, a successor with exact-entry/canonical-result
parity, and zero unique conformance role.

## Fixed task order

```text
NORMAL-ENTRY-CUTOVER-D0                    closed by this decision
  -> RAW-VM-REFERENCE-SUPPORT0-S0          next executable row
  -> normal/default cutover                PARK

future independent fronts:
  RAW-MINIMAL-MIR-JSON-PROFILE-D0
  ENTRY-RESULT-AOT-D0
  JSON-SOURCE-RESULT-D0
  PROGRAM-JSON-IMPORT-BUNDLE-D0
  GENERAL-RUNNER-STATUS-D0

future normal cutover:
  NORMAL-ENTRY-CUTOVER-D1
  only after one exact caller family and complete parity exist
```

## Reopen conditions for normal/default cutover

```text
one exact production caller family
one source-preparation owner
exact grammar/profile parity
backend capability parity
accepted result/status policy
unsupported-before-effects proof
fallback zero
reproducible caller census
retirement/sunset owner
fresh explicit decision
```

## Non-claims

```text
compile_with_source cutover
compile_with_source_and_imports cutover
default backend change
general VM/MIR status-law replacement
LLVM/native/ny_main activation
JSON / Program(JSON v0) changes
executor / selfhost / fastmem activation
Raw grammar widening
helper widening
macro/using/REPL activation
CUT0
```
