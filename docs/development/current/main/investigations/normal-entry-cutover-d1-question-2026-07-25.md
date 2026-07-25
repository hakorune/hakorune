# NORMAL-ENTRY-CUTOVER-D1 consultation question

- Status: Historical / superseded before implementation
- Date: 2026-07-25
- Related decision stop: `NORMAL-ENTRY-CUTOVER-D1-DESIGN-STOP-20260725`
- Related card: `normal-entry-cutover-d1-consultation-2026-07-25.md`
- Accepted successor: `normal-file-vm0-frontdoor-forge-task-2026-07-26.md`
- Purpose: freeze the admissible normal-entry family and its cutover contract before implementation

> Historical question only. The accepted successor for normal-file work is
> `NORMAL-FILE-VM0-FAMILY-D0-FORGE-FRONTDOOR`; it creates a new typed front
> door with no legacy caller mapping and no production caller before D2.

## Question for ChatGPT Pro

Please make a **design decision**, not an implementation patch, for the next
normal-entry lane of hakorune-selfhost. The repository is intentionally stopped
at `NORMAL-ENTRY-CUTOVER-D1-DESIGN-STOP-20260725`; no normal-entry code, public
cutover, JSON change, or legacy retirement should be authorized until this
decision is accepted.

The central question is:

> Is there one bounded, semantically coherent normal-entry caller family that
> can be admitted next, or should normal-entry cutover remain parked while the
> missing language/result/backend contracts are designed first?

Do not choose `compile_with_source` merely because it is the existing public
API. Treat its current path as evidence of legacy ownership, not as proof that
it is the correct next production family.

## Current evidence

The existing normal source path is:

```text
compile_with_source
  -> compile_legacy
  -> compile_request(Legacy)
  -> compile_with_source_internal
  -> MirBuilder::build_module(ast)
```

The worker census found no single bounded normal caller family matching the
already-proven Raw VM-reference contract:

```text
plain source-hint callers       = 6
import-aware callers            = 6
normal adapters                 = 2
direct build_module bridges     = 2
other heterogeneous routes     = REPL / JSON / Stage1 / VM fallback / LLVM / WASM
bounded Raw NarrowV1 family     = 0
```

The closest candidate is a compile-only MIR-JSON route, but it requires a
separate profile decision and must not claim source-entry execution or process
status semantics.

The current Raw VM-reference support lane is deliberately narrower and already
has a closed contract:

```text
explicit default-off backend/profile = raw-vm-reference / RawVmReferenceCanonicalV1
source origin                         = BareAst
imports/using                         = unsupported in NarrowV1
callable Main selection               = Omitted
exact target                          = sealed Main.main/0
interpreter                           = fresh Rust MirInterpreter
entry discovery                       = no NYASH_ENTRY, no module scan, no fallback
source result                         = typed SourceEntryResultV1
process projection                    = ProcessExitProjectionV1
Unit                                  = status 0
Integer 0..=255                      = exact status
out-of-range/unsupported/Fault        = typed fault, reserved status 70
conformance                           = 16-case matrix green
```

That Raw contract is not automatically a contract for the normal entry. The
normal lane still has these unresolved or mixed authorities:

```text
legacy finalizer / Builder last-ValueId return behavior
Script final-expression versus statement-tail classification
ordinary Function/Main explicit-return and Unit-fallthrough semantics
NYASH_ENTRY and module-scan entry discovery
LLVM helper/fallback result mapping
JSON and Program(JSON v0) direct bridges
legacy runner status conversion
```

The normative function-exit design says ordinary functions and `Main.main` use
explicit-return-only semantics, Script has a source-classified final-expression
result, and process status is a separate projection. Any compatibility behavior
must be named, bounded, default-off, and have a retirement condition.

## Decisions required

### Q1 — admissible caller family

Choose exactly one, or state a narrower replacement:

**A. Compile-only MIR-JSON family**

Admit one explicit artifact-only profile (for example a minimal MIR JSON
emitter). It produces an artifact and does not claim `SourceEntryResult`, VM
execution, process status, normal public entry, or legacy parity.

**B. Exact no-import execution family**

Admit one explicit source profile with a bounded grammar and exact entry/result
contract, then execute only through an explicitly selected backend. This is
allowed only if all missing Function/Main/Script/result/process contracts are
closed in the same decision chain.

**C. Park normal-entry cutover** *(recommended unless an exact family is proven)*

Admit no new normal production caller. Continue only with design/contract rows
that establish a bounded profile and its parity evidence. Existing callers stay
legacy and disconnected from the new Raw production lane.

For the selected option, state why the other options are rejected.

### Q2 — source and profile authority

Define one named profile and its authority for each axis:

```text
source origin (BareAst / file / REPL / JSON)
grammar and admitted statement forms
imports / using / macros / plugins
optimization and builder configuration
script arguments / REPL display behavior
artifact versus execution result
backend and exact execution mode
unsupported-before-effects rule
```

Do not allow caller-selected booleans, ambient environment, module symbols,
or backend fallback to become semantic authority.

### Q3 — compile-only versus execution

If A is selected, explicitly separate the artifact row from any later execution
row. A compile-only result must not silently imply:

```text
SourceEntryResult
process exit code
VM/LLVM parity
normal `compile_with_source` parity
```

If B is selected, identify the exact execution owner, source-entry owner, and
typed handoff to `ProcessExitProjectionV1`.

### Q4 — language/result parity

Freeze the required matrix before normal cutover. At minimum cover:

```text
ordinary function: explicit value return, explicit Unit, fallthrough Unit
Main.main: same rule as ordinary method; no entry-specific implicit tail
Script: final source expression versus Print/Local/Assignment statement tails
empty body and explicit void
annotated and unannotated returns
Integer / Bool / Float / String / dynamic or unsupported carriers
helper plus Main
```

State which behavior is canonical, which is compatibility-only, and where a
typed rejection occurs. Builder-produced last `ValueId` must not be accepted as
language authority by itself.

### Q5 — exact entry target and physical handoff

Specify how the selected source entry is sealed and carried to execution or
artifact publication. Require an exact typed target such as `Main.main/0` where
applicable. Explicitly forbid:

```text
NYASH_ENTRY-driven discovery
module/function scans
symbol-based route inference
generic Box or handle downcasts
legacy fallback or retry
```

If the profile is artifact-only, state that no entry target is produced.

### Q6 — process/result projection

If the selected family executes, name the single owner that converts
`SourceEntryResult` to process termination. Decide at least:

```text
Unit/Void
Integer range and out-of-range behavior
Bool
Float/String/object/dynamic values
source/VM faults
diagnostic versus status ownership
```

Require canonical projection parity across every backend included by the
profile. Legacy mappings such as modulo conversion, silent String-to-zero, or
heuristic handle decoding must be either excluded or placed behind a named
compatibility profile with a sunset.

### Q7 — failure, mutation, and compiler reuse

Define the failure owner and prove:

```text
unsupported profile/grammar fails before effects
typed rejection retains the exact owner/evidence
failed compile does not mutate the live Builder
failed execution becomes a typed source/process fault only after execution starts
same compiler can run success -> success and rejection -> success
retry/fallback/partial publication = 0
new production caller count = exactly 1
```

### Q8 — retirement and boundary conditions

State the exact conditions for later normal-entry cutover and retirement of old
routes. Address separately:

```text
compile_with_source default behavior
old Raw finalizer and run_raw callers
legacy runner status conversion
JSON / Program(JSON v0)
LLVM/native/ny_main
REPL/Stage1/WASM
executor/selfhost/fastmem
```

Do not claim repository-wide lexical removal when the actual condition is
production caller zero. Every compatibility profile needs a stable sunset ID,
an owner, and a measurable retirement row.

## Required answer format

Please answer in this exact shape:

```text
Decision: NORMAL-ENTRY-CUTOVER-D1-{short-name}
Status: accepted | provisional | rejected

Q1 caller family:
Q2 source/profile authority:
Q3 compile-only versus execution:
Q4 language/result parity:
Q5 exact entry target:
Q6 process/result projection:
Q7 failure/reuse/caller isolation:
Q8 retirement and boundaries:

first executable row:
conditional task order:
blockers:
non-claims:
```

If no bounded family is ready, choose **C / park** and give only the smallest
design rows needed to make one family admissible. Do not authorize code edits,
public cutover, JSON changes, executor wiring, or legacy retirement from a
parity discussion alone.

## Local acceptance boundary

This question is a design-stop artifact. Until the answer is accepted:

```text
normal-entry implementation                 = 0
compile_with_source cutover                 = 0
new normal production caller                = 0
JSON / Program(JSON v0) changes             = 0
LLVM/native/ny_main activation              = 0
legacy retirement                           = 0
executor/selfhost/fastmem/CUT0              = 0
```

The current implementation may continue only on already-authorized Raw
VM-reference support work and documentation/guard maintenance that does not
cross this D1 boundary.
