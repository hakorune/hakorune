---
Status: active design consultation
Date: 2026-07-28
Decision: CANONICAL-DEFAULT-COMPILER-INGRESS0-D0
Pack: COMPILER-RESIDUE0
Ceremony: T2 design stop
ReplacementCell: no
ProductionEdit: forbidden during D0
Parent:
  - docs/development/current/main/investigations/mirbuilder-next-edge-design-stop-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
NorthStar:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
---

# CANONICAL-DEFAULT-COMPILER-INGRESS0-D0

## Decision boundary

Nine production replacement cells are closed. Do not select a tenth cell yet.
The next design responsibility is the largest remaining north-star break:
normal/default compilation still enters the Legacy compiler route.

```text
normal/default source
-> compile_with_source_hint*
-> MirCompiler::compile_with_source*
-> compile_legacy_request
-> compile_legacy_candidate
-> MirBuilder::build_module
```

Existing canonical compiler entrypoints accept narrower, explicit families and
have no normal/default production caller. They cannot replace the route by a
T0 edge switch.

The D0 question is:

> What one typed normal compile request can consume the already-parsed AST,
> exact source identity, explicit imports, compiler configuration, and
> normal-admission provenance once; classify the complete normal source family
> before Builder effects; dispatch to existing canonical family owners once;
> return the backend-neutral `MirCompileResult`; and atomically replace the
> selected normal/default Legacy ingress without probing, retry, or fallback?

If that product is not yet implementable, the consultation must identify the
first missing capability and a smallest prerequisite slice with a real
production caller. A disconnected canonical route is not replacement credit.

## Why this outranks local cleanup

A six-worker bounded census found:

```text
live CALL-OBJECT child-recursion breaks             = 0
next live DESCENT break                             = non-Program root
normal/default typed canonical ingress callers     = 0
normal/default Legacy ingress callers               > 0
dead CALL-OBJECT facade cleanup                     = about -85..-95 LOC
dead body/statement shell cleanup                   = at least -60 LOC
bounded proof consolidation                         = -131 test LOC
```

The cleanups are valid debt repayment, but they switch no production authority.
The non-Program root is a real associated-descent break, yet it sits inside the
Legacy `build_module` route. Selecting it before the compiler-front decision
would make the old default ingress cleaner without resolving the final
pipeline's single canonical entry requirement.

Therefore:

```text
current authority = CANONICAL-DEFAULT-COMPILER-INGRESS0-D0
tenth cell         = not selected
production edit    = 0
manifest row delta = 0
```

## Exact current caller census

### Legacy public fronts

```text
MirCompiler::compile_with_source
  definition                    = 1
  non-test executable callers   = 3
    source_hint wrappers        = 2
    MirCompiler::compile        = 1

MirCompiler::compile_with_source_and_imports
  definition                    = 1
  non-test executable callers   = 3
    source_hint wrappers        = 2
    strict JSON session         = 1

MirCompiler::compile_legacy
  definition                    = 1
  non-test executable callers   = 3
    compile_with_source         = 1
    Program JSON v0             = 1
    REPL compatibility          = 1
```

The shared source-hint wrappers have twelve non-test runner call sites across
normal MIR, ordinary VM, MIR interpreter, LLVM, Wasm, VM-Hako bridge,
benchmarks, minimal MIR JSON, and Stage1/direct compatibility flows. Both
wrapper families currently end at `compile_with_source*`, hence Legacy.

### Normal/default and ordinary MIR-compiler examples

```text
explicit --backend mir
-> compile_with_source_hint_and_imports
-> compile_with_source_and_imports
-> compile_legacy_request

ordinary --backend vm
-> BootstrapRustVmKeep
-> compile_with_source_hint_and_imports
-> compile_with_source_and_imports
-> compile_legacy_request
```

These are the minimum caller family the D0 must classify. It must decide
whether the other source-hint consumers switch through the same atomic front
or remain separately named compatibility/reference authorities.

The no-flag CLI default is `backend=interpreter`, not MIR. The caller table must
classify that default explicitly as selected, separate, or out of scope with
evidence; it must not relabel explicit `--backend mir` as the CLI default.

### Existing canonical fronts

```text
compile_resolved
  normal/default production callers = 0
  accepted family                    = one resolved FunctionDeclaration

compile_resolved_callable_module
  normal/default production callers = 0
  accepted family                    = exact acyclic callable Program

explicit recursive callable ingress
  normal/default production callers = 0
  accepted family                    = bounded recursive callable Program
```

The closest source-plan substrate is:

```text
PreparedNormalSourcePlanInputV1
-> NormalSourcePlanClassifierV1
-> Script | Main0 | Callable
-> CanonicalCoreSourcePlanCompileRequestV1
-> compile_canonical_core_source_plan
```

Its live caller is an explicit default-off VM-reference profile. It is not a
normal/default compiler ingress and does not yet prove normal imports, complete
accepted grammar, backend-neutral result parity, or compatibility provenance.

## Decisions this D0 must close

### 1. Declared production caller family

At minimum, enumerate and classify:

```text
no-flag default backend=interpreter
explicit backend=mir
ordinary explicit backend=vm
MIR interpreter
LLVM
Wasm
VM-Hako bridge
benchmark and minimal-MIR routes
Stage1/direct routes
strict JSON
REPL
Program JSON v0
explicit canonical/Raw VM-reference profiles
```

Each family must be one of:

```text
selected normal/default or ordinary compiler cutover caller
explicit compatibility authority with an exact removal condition
explicit reference authority
out of scope with evidence
```

### 2. One typed request owner

The selected request must own or borrow exactly:

```text
already-parsed AST / Program
exact source identity and source hint
explicit imports snapshot
compiler invocation/configuration snapshot
normal-admission provenance
total source-family classification input
```

It must not reread or reparse source, reconstruct AST, drop imports, or infer
provenance from a string after classification. REPL, JSON, and reference
compatibility stay behind separate typed requests or explicit authorities;
their provenance is not merged into the normal request.

### 3. Total accepted-family table

The consultation must compare Legacy-normal accepted behavior with canonical
owners for:

```text
Script
Main0
callable Main/module
top-level functions
acyclic calls
recursive calls
currently accepted Box roots
currently accepted non-Program roots
imports / Using handling
```

FunctionDeclaration coverage is not Program coverage. Acyclic coverage is not
recursive coverage. Missing rows must fail closed before Builder effects.

### 4. One classifier and internal owner graph

The final candidate must have:

```text
normal/default route selection       = exactly 1
source-family classification         = exactly 1
Builder effects before classification= 0
canonical owner dispatch             = exactly 1
canonical rejection -> Legacy        = 0
```

Existing `compile_resolved*`, callable-module, recursive-module, and
NormalSourcePlan implementations may remain semantic owners, but must become
branches behind the one typed classifier rather than competing default fronts.

### 5. Result and publication parity

The selected front must return the same backend-neutral compiler product and
preserve:

```text
MirCompileResult contract
module candidate isolation
function draft collection
success-only atomic publication
source diagnostics and identity
imports
compiler configuration
compiler reuse after failure
```

VM execution belongs after compilation. A VM-reference front that reads the
file and executes it is not itself the backend-neutral compiler owner.

### 6. Compatibility boundary

The D0 must explicitly decide the future of:

```text
REPL compatibility
Program JSON v0 compatibility
strict JSON session
explicit Raw VM-reference profiles
explicit canonical-core VM-reference profile
```

Do not merge compatibility provenance into the normal typed request. Keeping
an explicit compatibility entry is allowed only with an exact typed contract
and removal or permanent-support condition.

### 7. Atomic old-edge delete set

Before any executable row is selected, name the exact old call-shaped edges
that become zero in the same implementation commit. The minimum candidates are:

```text
normal/default compile_with_source -> compile_legacy
normal/default compile_with_source_and_imports
  -> compile_legacy_request
normal/default source_hint wrappers -> Legacy request
```

Do not promise deletion of explicit REPL/JSON/reference APIs until their
caller classification is accepted.

## Candidate outcomes

### Candidate A — one total normal/default typed ingress

Accept only if the census proves one request can cover the complete selected
normal caller family and dispatch every accepted source family before Builder
effects.

Expected ceremony:

```text
T2
one bounded responsibility-design commit
one immediately-following atomic I0/R0 implementation commit
```

The implementation commit must add the typed owner and remove the selected
Legacy/default edges together. No temporary dual default route is allowed.

### Candidate B — prerequisite capability first

Select B if exactly one missing capability prevents A, for example:

```text
imports-bearing canonical request
backend-neutral canonical compile result
total source-family preflight
normal/compatibility provenance partition
```

The prerequisite may become a production cell only when it has a named
existing production caller and deletes an old authority atomically. Otherwise
it is a bounded enabling task and receives no replacement credit.

### Rejected candidate C — probe then fall back

```text
try canonical
-> unsupported/rejected
-> retry through Legacy
```

Reject unconditionally. It creates two default authorities, changes diagnostic
and effect timing, and hides incomplete accepted-family coverage.

## Structural boundary

Current closed ratchet:

```text
source files / ceiling = 952 / 952
source LOC   / ceiling = 182452 / 182452
test files   / ceiling = 139 / 139
test LOC     / ceiling = 40809 / 40826
```

The next five-cell rolling production LOC base is `-141`, so a tenth cell could
be at most `+141` under that independent rule.

The four-metric ratchet above measures the fixed MirBuilder roots only. It does
not measure `src/mir/compiler` or runner files and therefore cannot, by itself,
bound this compiler-ingress implementation. Those MirBuilder-root ceilings
must remain unchanged. In addition, this D0 requires the eventual atomic
implementation to have non-positive physical production-Rust LOC across every
`.rs` file in its diff, including compiler and runner ingress files.

```text
MirBuilder source/test ratchet delta      = 0
new source/test/check files               = 0
new per-cell guard                        = 0
whole implementation production Rust LOC <= 0
all touched source/check                  < 800 lines
```

Proof consolidation or dead-facade cleanup may later ratchet the ceilings
downward. Their deletion is not headroom that authorizes unrelated growth.

## Required evidence before implementation

```text
exact non-test caller census                  = complete
selected normal/default caller family         = exact
selected typed production callers             = exact N
typed request fields and owner                = exact
accepted source-family table                  = total
imports/config/provenance transport           = exact
canonical internal owner graph                = one
Builder effects before classification         = 0
backend-neutral result contract               = exact
compatibility/reference boundary              = exact
atomic old-edge delete set                    = exact
selected compile_with_source* -> Legacy edges = 0
compatibility/reference caller delta          = 0
canonical rejection -> Legacy                 = 0
full normal corpus/backend parity gate        = named
failure / compiler reuse / atomic publish     = named
MirBuilder ratchet delta                       = 0
whole implementation production Rust LOC      <= 0
```

Only after all rows are accepted may a tenth replacement manifest row be
created.

## Hard stop

Return to a narrower design consultation if:

```text
normal accepted corpus cannot be enumerated
imports do not cross the typed boundary
compiler options or provenance are inferred late
source is reread/reparsed or AST is rewritten
family selection occurs after Builder effects
canonical owner returns a VM-specific terminal instead of MirCompileResult
normal and compatibility callers cannot be separated
canonical rejection requires Legacy retry
one atomic old-edge delete set cannot be named
full parity requires unrelated language/runtime/backend semantics
source/test/check structural ceilings would grow
```

## Explicit non-claims

This D0 does not authorize:

```text
production source, test, guard, or manifest edits
tenth replacement-cell credit
canonical probe with Legacy fallback
default backend selection changes
language, grammar, runtime, backend, or result-policy changes
source reread/reparse, AST rewrite, or import re-resolution
blanket deletion of REPL/JSON/reference entrypoints
non-Program root descent implementation
Stage-B or Ownership activation
selfhost migration
proof consolidation or dead-facade cleanup
```

## Consultation request

Please return one of:

```text
A. Candidate A accepted
   exact typed request
   exact caller family
   total accepted-family table
   internal canonical owner graph
   atomic old-edge delete set
   parity/failure/reuse gates
   structural repayment

B. Candidate B accepted
   first missing capability only
   exact production caller, if any
   exact owner and delete set
   proof that no fallback/second default router is introduced

C. Hard stop
   first unresolved authority boundary
   evidence required for the next bounded census
```
