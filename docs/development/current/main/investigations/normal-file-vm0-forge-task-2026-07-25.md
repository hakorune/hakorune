---
Status: Closed historical; superseded by NORMAL-FILE-VM0-FAMILY-D0-FORGE-FRONTDOOR
Date: 2026-07-25
Decision: NORMAL-ENTRY-CUTOVER-D1-PARK-AND-FORGE-NORMAL-FILE-VM0
Scope: one bounded normal-file VM-reference family, from D0 evidence to one-caller cutover
Superseded by: normal-file-vm0-frontdoor-forge-task-2026-07-26.md
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/language-v1-convergence-current.md
  - docs/development/current/main/investigations/normal-entry-cutover-d1-consultation-2026-07-25.md
  - docs/reference/language/function-exit-and-entry-result.md
---

# NormalFileNoImportVmReferenceV1 task order

## Decision and boundary

D1 accepts **C: park normal-entry cutover**, but fixes the next family instead
of leaving the lane open-ended:

```text
NormalFileNoImportVmReferenceV1
  D0 evidence -> proof P0 -> D2 activation decision
  -> one caller cutover -> import-aware family -> legacy fence
```

This card is the execution SSOT after the D1 consultation. It does not make
`compile_with_source` the next owner and does not widen the already-supported
opt-in `raw-vm-reference` route.

The first executable row is documentation/read-only evidence:

```text
NORMAL-FILE-VM0-FAMILY-D0
```

No production normal caller is added until D2 accepts the bounded family.

## Fixed profile

```text
profile                         = NormalFileNoImportVmReferenceV1
profile owner                   = SealedNormalEntryProfileV1
source origin                   = SingleFileUtf8V1
source read                     = exactly once
parser                          = canonical parser exactly once
AST transport                   = internal BareAst only
grammar                         = exact NarrowV1 matrix, no silent widening
imports / using                 = NoImports; source using is typed rejection
macros / @local / plugins       = unsupported; no source rewrite or preexpand
REPL / script arguments / JSON  = unsupported
optimization                   = CanonicalDefaultOptimizedV1
artifact contract               = none in this family
execution backend               = fresh Rust MirInterpreter only
entry                           = sealed source entry and exact physical target
process profile                 = CanonicalProcessExitV1::V1
fallback / retry                = zero
```

The normal front door only seals file/profile facts and hands off to existing
typed Raw kernels. It does not own Builder lowering, Return finalization, entry
search, VM status conversion, or diagnostics policy.

## Existing kernel correspondence

The family must reuse this chain; a second compiler is forbidden:

```text
NormalFileRequestV1
  -> SealedNormalEntryProfileV1
  -> PreparedNormalFileSourceV1
  -> RawPublishedCompileRequestV1
  -> MirCompiler::compile_raw_published_v1
  -> RawPublishedInvocationV1
  -> exact VM-reference activation
  -> SourceEntryResultV1
  -> ProcessExitProjectionV1
  -> VmReferenceProcessOutcomeV1
  -> RawVmReferenceRunReportV1
```

`compile_raw_published_v1` remains the sole typed Raw compile kernel. The
existing explicit `raw-vm-reference` lane remains default-off and supported;
this task only proves whether a normal-file front door can feed it safely.

## D0 — `NORMAL-FILE-VM0-FAMILY-D0`

### Purpose

Select exactly one existing plain source-hint caller family and prove that it
has one source preparation, one profile, and one observable output. Do not use
string occurrence counts as the selection authority.

### Required evidence

```text
caller family is one route-scoped request owner
source is one UTF-8 file read once
profile is fixed before source effects
NoImports/no-macro/no-REPL surface is explicit
output is execution report, not artifact-only output
normal caller uses the existing Raw compile/VM kernels
no legacy fallback, retry, or backend widening
legacy/new behavior differences are enumerated
```

### D0 product

```rust
struct SealedNormalFileVmCandidateV1 {
    caller: VerifiedNormalCallerFamilyV1,
    source: SealedNormalFileSourceProfileV1,
    compile: RawPublishedCompileProfileV1,
    execution: RawVmReferenceExecutionProfileV1,
    result: CanonicalProcessExitV1,
    legacy_sunset: NormalEntryLegacySunsetV1,
}
```

The constructor is read-only evidence. `new normal production caller = 0`.

### D0 rejection

If no plain caller satisfies the family, record `NoBoundedCallerFamily` and
keep the lane parked. Do not broaden the grammar, add a fallback, or select all
six source-hint callers together.

### D0 closeout — `NoBoundedCallerFamily` (2026-07-25)

The read-only census inspected every plain `compile_with_source_hint` production
site. No site is an admissible NormalFileNoImportVmReferenceV1 caller:

| Site | Observable contract | D0 result |
| --- | --- | --- |
| `bench_vm` | inline benchmark source, result discarded, timing only | reject |
| `bench_jit` | inline benchmark source, JIT env, result discarded | reject |
| `verify_outputs_match` | mixed legacy compile paths, string comparison | reject |
| `execute_mir_json_minimal` | one file read, MIR JSON artifact, no execution result | reject; separate artifact lane |
| Stage-1 direct route | explicit compatibility bridge, bare MIR/artifact or legacy exit | reject |
| VM compatibility fallback | using/preexpand/plugins and independent legacy status mapping | reject |

All six ultimately enter the legacy `compile_with_source` / `build_module`
chain; none consumes `compile_raw_published_v1`, a sealed normal profile, an
exact source-entry continuation, or `ProcessExitProjectionV1`. The closest
shape is `--emit-mir-json-minimal`, but it is artifact-only and remains the
independent `RAW-MINIMAL-MIR-JSON-PROFILE-D0` lane.

Therefore D0 closes with:

```text
candidate family                  = 0
NoBoundedCallerFamily             = sealed evidence
new normal production caller      = 0
fallback / retry                  = 0
normal-file D2 activation         = blocked by owner-selection decision
```

This is a source-evidence update, not permission to create a caller in the D0
row. The next decision must choose whether the named family is a future new
front-door owner (with no existing caller mapped), an artifact-only lane, or a
continued park. It must not silently reinterpret one of the six legacy sites.

## P0 — proof before activation

P0 is one proof package with four sections; it is not permission to cut over.

### `NORMAL-FILE-VM0-SOURCE-PROFILE-D0`

Seal:

```text
profile axes and conflict rejection before file read
one source read and one canonical parse
exact NarrowV1 grammar and using/import rejection
Builder effects = 0 on profile/source/parse rejection
CanonicalDefaultOptimizedV1 without caller booleans
```

The prepared source owner is move-only:

```rust
struct PreparedNormalFileSourceV1 {
    source_file: Box<str>,
    ast: ASTNode,
    profile: SealedNormalEntryProfileV1,
    _seal: PreparedNormalFileSourceSealV1,
}
```

### `NORMAL-FILE-VM0-CORRESPONDENCE-P0`

Prove that the normal candidate is only a typed front-door correspondence to
the existing Raw published compile and VM-reference activation. It must not
introduce:

```text
independent Builder path
independent Return finalizer
NYASH_ENTRY or module scan
independent process conversion
diagnostic status mutation
legacy compile fallback
```

### `NORMAL-FILE-VM0-SEMANTIC-MATRIX-P0`

Every row is either accepted or a typed reject before D2. The required matrix
is:

```text
ordinary function explicit value / explicit Unit / Unit fallthrough
Main.main/0 explicit return / Unit fallthrough / no implicit tail
Script final expression Value
Script final Print / Local / Assignment / CompoundAssignment = Unit
empty body and explicit void/null
annotation and unannotated result relations
Integer / Bool / Float / String
Object / Box / Future / WeakRef = typed first-profile rejection
helper plus Main
non-Main entry candidates = never scanned or retried
```

Canonical authority is `ExplicitReturnOnly` for functions/Main,
`ScriptLastExpressionOrUnit` for Script, and `ProcessExitProjectionV1` for
process status. A Builder last `ValueId` is never language authority.

### `NORMAL-FILE-VM0-CALLER-G0`

The census must be reproducible and route-scoped:

```text
new normal production caller = 0 before D2
candidate family             = exactly 1
fallback / retry             = 0
unclassified candidate       = 0
legacy caller identity       = recorded for later retirement
```

## D2 — `NORMAL-ENTRY-CUTOVER-D2`

D2 is a second design/activation decision. It chooses whether the proven
candidate may become one production caller. D2 must reject activation if any
P0 row is a gap, not silently add a capability in the cutover row.

Required D2 inputs:

```text
SealedNormalFileVmCandidateV1
semantic matrix green or typed-reject complete
exact entry-target correspondence
canonical process projection evidence
compiler reuse evidence
real binary status/diagnostic evidence
caller census and sunset owner
```

Possible outcomes:

```text
accepted       -> NORMAL-FILE-VM0-REQUEST0-S0
provisional    -> remain parked with named missing proof
rejected       -> return to the specific capability owner
```

## Gate B — one-caller production activation

Only after D2 `accepted`:

```text
NORMAL-FILE-VM0-REQUEST0-S0
  one typed normal runner request owner

NORMAL-FILE-VM0-SOURCE0-S0
  one file read + one parse + prepared source owner

NORMAL-FILE-VM0-COMPILE0-S0
  one consuming handoff to compile_raw_published_v1

NORMAL-FILE-VM0-ENTRY0-S0
  selected source entry + exact Main.main/0/root target transport

NORMAL-FILE-VM0-RESULT0-S0
  SourceEntryResult -> ProcessExitProjectionV1

NORMAL-FILE-VM0-PARITY0-P0
  actual source/function/Main/Script/status/diagnostic matrix
  real binary green

NORMAL-FILE-VM0-CALLER0-I0
  exactly one normal production caller

NORMAL-FILE-VM0-G0
  caller=1, fallback=0, other normal callers unchanged

NORMAL-FILE-VM0-LEGACY-CALLER-RETIRE0-S0
  selected family's old production caller=0
```

### Activation failure law

```text
profile/source/parse rejection -> Builder mutation 0
compile rejection              -> live Builder replacement 0
activation rejection           -> published owner retained, no execution
VM fault after execution       -> SourceEntryResult::Fault, status 70
success -> success             -> green
reject -> success              -> green
```

Every rejection is a typed owner with `stage()`, `error()`, bounded report,
and `discard(self)` only. No retry, fallback, owner escape, or partial result.

## Gate C — import-aware family

This gate is deliberately after the no-import family:

```text
NORMAL-IMPORT-BUNDLE-D0
  source + imported files + aliases authority

NORMAL-IMPORT-BUNDLE0-S0
  one sealed bundle, no ambient using

NORMAL-FILE-IMPORT0-PROFILE0
  named import-aware profile

NORMAL-FILE-IMPORT0-CORRESPONDENCE0
  same Raw compile/publication kernel correspondence

NORMAL-FILE-IMPORT0-CUTOVER0
  one import-aware caller family

NORMAL-FILE-IMPORT0-RETIRE0
  corresponding legacy caller zero
```

Import-aware work cannot be smuggled into `NoImports`.

## Gate D — MirBuilder normal completion

MirBuilder core completion is separate from all backend/integration migration:

```text
MIRBUILDER-CORE-COMPLETE0
  canonical function exit owner = 1
  canonical Script result owner = 1
  canonical entry/result owner = 1
  atomic draft/module publication = 1
  supported Raw VM lane = 1
  bounded normal file lane = 1
```

Normal compiler completion is later:

```text
MIRBUILDER-LEGACY-FENCE0-S0
  direct build_module only inside named LegacyCompatibility profiles

MIRBUILDER-NORMAL-CALLER-CENSUS0-P0
  plain/import-aware/adapters/direct bridges reclassified

MIRBUILDER-NORMAL-COMPLETE0-P0
  unclassified normal caller zero

MIRBUILDER-COMPLETE0-G0
  normal completion declaration
```

JSON, REPL, Stage1, WASM, LLVM/AOT, executor, selfhost, and fastmem are
separate integration families and are not blockers for the bounded normal VM
family.

## Ownership and no-go rules

```text
source/profile authority       = SealedNormalEntryProfileV1
compile authority              = compile_raw_published_v1
entry authority                = selected typed source-entry continuation
VM target authority            = exact sealed Main.main/0/root target
result authority               = SourceEntryResultV1
process status authority       = ProcessExitProjectionV1
diagnostic formatting          = VmReferenceProcessDiagnosticAdapterV1
legacy process mapping         = named LegacyRunnerExitProjectionV1 only
```

Forbidden in every row:

```text
compile_with_source default cutover before D2
caller-selected semantic booleans
source rewrite or @local pre-expansion
ambient import/using configuration
NYASH_ENTRY or module scan
Builder last-ValueId return inference
legacy fallback/retry
generic Box/handle downcast
silent unsupported-to-zero conversion
JSON/LLVM/native/executor widening
```

## Sunset records

### Normal family legacy sunset

```text
sunset_id        = NORMAL-FILE-NOIMPORT-LEGACY-SUNSET-001
retirement_owner = NORMAL-FILE-VM0-RETIRE0
retirement_row   = NORMAL-FILE-VM0-LEGACY-CALLER-RETIRE0-S0
retire_when      = accepted profile + parity green + one caller + fallback 0
                   + selected family's legacy production caller 0
```

### Direct Builder bridge fence

```text
sunset_id        = MIRBUILDER-DIRECT-BUILD-MODULE-SUNSET-001
retirement_owner = MIRBUILDER-LEGACY-FENCE0
retirement_row   = MIRBUILDER-LEGACY-FENCE0-S0
retire_when      = every remaining production caller is typed canonical or
                   explicitly named LegacyCompatibility; unclassified caller 0
```

## Non-claims

```text
compile_with_source default cutover
default backend change
general VM/MIR status-law replacement
LLVM/native/ny_main activation
JSON / Program(JSON v0) changes
REPL / Stage1 / WASM cutover
executor / selfhost / fastmem activation
old Raw-chain retirement
App AnyStatement-tail promotion
repository-wide lexical legacy removal
CUT0
```
