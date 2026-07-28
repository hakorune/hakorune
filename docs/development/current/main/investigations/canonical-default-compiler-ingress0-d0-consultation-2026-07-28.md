---
Status: closed bounded design census; narrower prerequisite selected
Date: 2026-07-28
Decision: NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0
ParentDecision: CANONICAL-DEFAULT-COMPILER-INGRESS0-D0
Candidate: B
Pack: COMPILER-RESIDUE0
Ceremony: T2 prerequisite design
ReplacementCell: no
ProductionCaller: 0
ProductionEdit: forbidden during D0
Parent:
  - docs/development/current/main/investigations/mirbuilder-next-edge-design-stop-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
NorthStar:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
---

# CANONICAL-DEFAULT-COMPILER-INGRESS0-D0 — Candidate B

## Decision

Candidate B remains accepted. Candidate A is not executable yet.

```text
Candidate A                         = rejected for now
Candidate B                         = accepted
aggregate future product            = VerifiedNormalGeneralProgramPlanV1
first missing authority             = NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0
MirBuilder return target            = NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0
ceremony                            = T2 bounded enabling design
production caller during D0         = 0
replacement-cell credit             = 0
tenth manifest row                  = absent
production source/test/check edit   = 0
fallback / retry / reselection      = 0
```

The direction of the parent consultation remains accepted:

```text
one normal request
-> one source-only inventory
-> one total family classification
-> one verified owner dispatch
-> one backend-neutral candidate/publication
```

The missing piece is not a thin request wrapper. The repository does not yet
have one Program-owned, pre-Builder source/catalog authority for the current
normal surface outside the exact canonical Script, Main0, and Callable
families. A whole `VerifiedNormalGeneralProgramPlanV1` may eventually aggregate
verified products, but implementing it first would create a second monolithic
MirBuilder.

Do not hide that gap behind a typed `GeneralProgram -> compile_legacy_candidate`
branch. This D0 must first define the finite accepted surface, the verified
product, and the consuming owner without adding a production route.

## Corrected current caller facts

The parent consultation contained three stale caller claims. The source at the
time of this decision establishes:

```text
CLI no-flag backend default       = mir
explicit --backend mir           = the same mainline production family
explicit --backend vm            = legacy keep/debug compatibility
strict JSON compiler call        = cfg(test)-only fixture
strict JSON runtime session      = downstream of an already-built MIR module
```

Evidence:

```text
src/cli/args.rs
  backend default_value("mir")

src/runner/dispatch.rs
  backend "mir" -> execute_mir_mode
  backend "vm"  -> BootstrapRustVmKeep orchestration

src/runner/route_orchestrator.rs
  vm family = explicit legacy keep/debug only

src/backend/mir_interpreter/strict_json_session.rs
  compile_with_source_and_imports use is inside cfg(test)
```

`execute_mir_interpreter_mode` still contains a Legacy compiler call, but its
definition has no root-reachable caller in the current bounded census. It is
not counted as a selected normal production construction site.

## Why Candidate A cannot be implemented now

### Existing substrate

The normal runner already performs most request preparation before the
compiler call:

```text
source read
-> using/prelude resolution
-> exact imports snapshot
-> parser normalization
-> parse exactly once
-> AST
```

The compiler also already has a candidate-session substrate:

```text
BuilderInvocationConfigV1
  REPL mode
  quiet diagnostics
  imports
  plugin method signatures
  source file
  core-ID seed policy

LegacyModuleCandidateSessionV1
  candidate isolation
  failure leaves the live Builder unchanged
  success commits once
```

Therefore the first missing capability is not:

```text
an imports-bearing envelope
a configuration snapshot
candidate isolation
MirCompileResult as a type
```

### Missing complete normal Program owner

The current canonical source inventory is intentionally narrow:

```text
root:
  Program only

unsupported top-level rows:
  non-Main BoxDeclaration
  Using / Import
  Enum
  Brand
  TypeAlias
  GlobalVar
  StaticConstTable

Main0:
  unique static Main
  static main
  arity 0

Script recipe:
  LinearScalar0
```

`LinearScalar0` accepts only:

```text
expressions:
  Literal / Variable / Unary / Binary

statements:
  Expr / Print / Assignment / CompoundAssignment / Local
```

It explicitly excludes general control flow such as If, Loop, Return, Break,
Continue, and ScopeBox. The exact canonical Callable owner is also a bounded
single-file/no-import family.

The current normal Legacy route accepts programs beyond those families,
including user boxes, constructors, fields, and non-Main0 entry shapes.
Consequently:

```text
current normal accepted Program
!=
Canonical Script0 + Main0 + exact Callable
```

### Rejected pseudo-cutover

This is forbidden:

```text
NormalCompileClassifier
  -> exact canonical family: canonical owner
  -> otherwise: compile_legacy_candidate
```

Even without retry, the residual branch would still be:

```text
bare AST
-> Legacy build_module
-> Lower-side route and semantic redecision
```

A typed enum around that branch does not remove the old authority.

## Accepted first prerequisite responsibility

```text
NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0
```

Definition:

> For one selected normal `Program`, co-seal source identity, import admission,
> exact top-level declaration rows, the entry site and arity, and exact
> user-box/member callable identities and declaration sites before Builder
> effects.

`GeneralProgram` does not mean “all other AST.” It means only:

```text
an explicitly enumerated current-normal family
whose source correspondence and lowering obligations are complete
before Builder effects
```

Anything not in that finite table must produce a typed preflight rejection.

## Product order

The target owner graph is deliberately staged:

```text
OwnedNormalProgramSourceV1
  AST + exact source identity
  imports/config/admission retained

-> NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0
   exact Program/declaration/entry/callable source facts
   typed rejection before Builder effects

-> GENERAL-FUNCTION-PLAN0 family slices
   Resolve -> Facts -> Recipe -> Verify
   one semantic vocabulary at a time

-> aggregate VerifiedNormalGeneralProgramPlanV1
   module facts + heterogeneous verified function plans
   entry/publication obligations

-> existing DraftSeal / Collector / atomic module transaction

-> current-normal MirCompileResult parity owner

-> one infallible commit

-> MirCompileResult
```

The aggregate product is not the next implementation row. It must not expose
an unbounded raw-AST escape hatch or re-own every statement/expression family.

## D0 execution task

This is a design and census task. It does not implement the products above.

### 1. Freeze the selected normal corpus

Build a bounded, repository-backed table of current normal programs for:

```text
Script:
  scalar
  If
  Loop
  Return
  call / new / field / index

Main:
  Main.main/0
  Main.main(args)
  helper methods
  user boxes
  fields / constructors

Callable:
  top-level helper
  Main helper
  acyclic call graph
  bounded recursive call graph

Module:
  imports
  user boxes
  currently accepted declaration rows
```

For every row, record:

```text
current normal acceptance evidence
source identity/import profile
existing canonical owner, if any
missing verified obligation
expected MirCompileResult surface
unsupported backends and exact fail-fast
```

Do not create one document or one guard per fixture. Keep the table and the
decision in this rolling card; use existing corpus/test files as evidence.

### 2. Define the finite residual boundary

Partition the corpus into:

```text
CanonicalCore
  Script
  Main0
  Callable
    topology = Acyclic | Recursive

finite named residual rows
TypedReject
```

The partition must be total and pairwise-disjoint before Builder effects.
Canonical failure may not cause movement to another branch.

`RecursiveCallable` is not a fourth source family. The existing Callable owner
seals SCC topology and chooses Acyclic or Recursive internally. For residual
rows, enumerate the exact source vocabulary and first rejection stage. An
`Other`, `Unknown`, or residual Legacy variant is forbidden.

### 3. Select the first module-source vocabulary

The first source-backed residual is:

```text
non-Main user Box
static Main.main(args)
constructor / field / method declarations
```

The first red is the top-level non-Main `BoxDeclaration` inventory rejection,
before body lowering. Therefore the first bounded owner is exact user-box
schema/declaration facts plus module correspondence, not a broad body lowerer.

The owner must carry:

```text
source identity and import admission
exact declaration rows and sites
entry site and arity
user-box/member callable identities and sites
typed rejection for unlisted rows
```

Body grammars remain later `GENERAL-FUNCTION-PLAN0` slices. Do not invent a
second recipe, function-seal, collector, or module-publication authority.

### 4. Define later function-plan slices

Each accepted residual body vocabulary is a separate verified-plan slice:

```text
Main0 + If
Main0 + Loop
Return / call / new / field / index
Main(args)
user-box birth / field / method
Enum-bearing modules
static-const-table modules
imports-bearing modules
top-level function main(args)
```

Each slice must reuse the existing semantic owner and preserve exact source
coverage. It must not call:

```text
compile_legacy_candidate
MirBuilder::build_module(ASTNode)
build_expression(raw AST root)
build_statement(raw AST root)
```

If one slice would re-own multiple unrelated statement/expression semantic
families, split it again. Do not turn `GeneralProgram` into a second
monolithic MirBuilder.

### 5. Freeze current-normal publication parity

The final normal route must preserve the current normal result contract, not
silently adopt the explicit canonical-reference terminal policy.

The D0 must name parity evidence for:

```text
postprocess schedule:
  RC insertion = Run
  verification = ReportPreTransformOnly
  pre-transform verifier Err remains reportable in MirCompileResult

request/config:
  exact imports and source identity
  quiet/plugin signatures/optimize decision
  ContinueLive core-ID policy
  successful no-import compile clears prior imports

function set
entry symbol and arity
MIR instructions
types and origins
metadata
diagnostic category/text
verification_result
imports
source identity
success-only publication
compiler reuse after each failure stage
```

The explicit canonical policy (`RequireFinal`, family-specific RC skip,
fresh/quiet-only candidate state) is not parity-compatible with the current
normal/default contract and must not be reused unchanged.

The publication boundary is:

```rust
pub(crate) struct NormalCompilePublicationV1 {
    candidate: CompletedNormalCompileCandidateV1,
    contract: CurrentNormalCompileResultContractV1,
}
```

This is compiler publication policy, not backend execution policy.

### 6. Freeze the future total ingress handoff

After this prerequisite closes, Candidate A may use:

```rust
pub(crate) struct NormalCompileRequestV1 {
    source: OwnedNormalSourceV1,
    imports: NormalImportSnapshotV1,
    config: NormalCompilerConfigSnapshotV1,
    admission: NormalAdmissionV1,
}
```

Required distinctions:

```text
source identity:
  Named { display_name, source_hint }
  Anonymous { exact caller provenance }

imports:
  Explicit(snapshot)
  EmptyByContract(MinimalNoImportsSealV1)

admission:
  PreparedSourceFile
  MinimalMirJsonNoImports

REPL:
  structurally separate compatibility request

backend name:
  absent from the backend-neutral compile request
```

The final classifier must use one observation product and one exhaustive
match:

```rust
pub(crate) enum VerifiedNormalCompileDispatchV1 {
    CanonicalCore(CanonicalCoreSourcePlanCompileRequestV1),
    GeneralProgram(BoundNormalGeneralProgramRequestV1),
}
```

It must not “try” the narrow classifier and use `GeneralProgram` after
rejection.

## Caller authority table for future Candidate A

### Selected normal/default

```text
execute_mir_mode
  no-flag backend=mir
  explicit backend=mir
  dump/verify/full MIR JSON variants

execute_mir_json_minimal
  MinimalMirJsonNoImports admission

LLVM source compiler
  backend terminal runs after MirCompileResult

Wasm source compiler
  backend terminal runs after MirCompileResult
```

These are four source compiler construction sites, not four independent
source-family classifiers.

### Explicit compatibility

```text
explicit backend=vm / BootstrapRustVmKeep
vm-compat-fallback
Stage1 binary-only direct route
MirCompiler::compile(ASTNode) bare-AST API
REPL
Program JSON v0 import compilation
```

They require separate typed provenance. Candidate A may not infer their class
from source hints or backend strings inside a generic wrapper.

### Explicit reference

```text
VM-Hako bridge
Raw VM-reference
canonical-core VM-reference
benchmark compiler profiles
```

They may later share a source-neutral compile kernel, but their constructors
remain separate from normal admission.

### Out of scope

```text
strict JSON runtime session
  consumes a MIR module
  compiler call exists only in cfg(test)

execute_mir_interpreter_mode
  no current root caller

backend execution after MirCompileResult
```

## Accepted-family table

| Source family | Current canonical owner | Required final disposition |
| --- | --- | --- |
| LinearScalar0 Script | existing | `CanonicalCore::Script` |
| general Script with control/calls/new/field/index | none complete | enumerated `GeneralProgram` plan |
| exact static `Main.main/0` | existing | `CanonicalCore::Main0` |
| Main args/helpers/user boxes/fields/constructors | none complete | enumerated `GeneralProgram` plan |
| exact acyclic callable module | existing | canonical Callable, Acyclic topology |
| exact bounded recursive callable module | existing | same canonical Callable, Recursive topology |
| non-Main user boxes | canonical inventory rejects | first finite module-source vocabulary |
| Enum | canonical inventory rejects; production-normal evidence exists | later finite residual row |
| Static table | canonical inventory rejects; production-normal evidence exists | later finite residual row |
| Brand/TypeAlias/Global | no production-normal acceptance evidence | typed reject; no acceptance claim |
| imports-bearing source | transport exists; exact canonical callable is no-import | preclassified imports-aware owner |
| residual Using/Import AST after preparation | not accepted | pre-Builder typed rejection |
| non-Program root | outside normal-file family | `BareAstCompatibility` |

This table is a design inventory. D0 closeout must replace “accepted rows” with
exact repository-backed rows; it may not promote an unverified declaration
family merely because Legacy contains a branch for it.

## Candidate A reopen conditions

Candidate A remains parked until all are true:

```text
selected normal corpus                          = finite and named
source-family partition                         = total and disjoint
module source/catalog authority                 = closed
required GENERAL-FUNCTION-PLAN0 slices          = closed
VerifiedNormalGeneralProgramPlanV1 aggregate    = closed
existing DraftSeal/Collector reuse              = closed
Lower-side source-family redecision             = 0
current-normal MirCompileResult parity          = named and green
selected normal construction sites              = exactly 4
compatibility/reference constructors            = separately typed
atomic selected old-edge delete set              = exact
fallback / retry / reselection                   = 0
```

Only then may the tenth replacement row be selected.

The future Candidate A atomic delete set is:

```text
selected normal compile_with_source -> compile_legacy
selected normal compile_with_source_and_imports -> compile_legacy_request
selected normal source-hint wrappers -> Legacy request
selected normal MirLoweringRequestV1::Legacy construction
```

Compatibility/reference Legacy edges are not part of that deletion unless a
separate accepted removal condition covers them.

## Accepted task order

Closing this census does not authorize Candidate A. The accepted order is:

```text
M0. this bounded census closes
    return target = NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0

D1. repository artifact lifecycle interlude
   R1 archive substrate recovery
   -> R2 global phase resolver
   -> R3 two-file phase-296x pilot
   -> R4 first bounded nested-archive batch only

D2. DOCS-MEANING-RECOVERY-RETURN0
   strict lifecycle gate
   current-state pointer guard
   link/reference closure
   measured current-doc recount

M1. NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0
    close exact Program/declaration/entry/callable source authority

M2. GENERAL-FUNCTION-PLAN0
    implement one finite semantic vocabulary per row

M3. aggregate VerifiedNormalGeneralProgramPlanV1
    aggregate only already-verified module and function products

M4. reuse DraftSeal / Collector / atomic module publication

M5. close CurrentNormalCompileResultContractV1 parity

M6. Candidate A re-evaluation
    accept one total typed ingress or stop at the first remaining capability

M7. Candidate A atomic cutover
    only after M1-M6 are green
```

The docs interlude is bounded. R5 stale-phase cohorts and R6
design/investigation retirement remain later repository-lifecycle work and do
not block the MirBuilder return after the first R4 batch.

Do not mix compiler source changes with R1 through RETURN0. The current
compiler card remains the return target; no new consultation or task document
is created for the handoff.

## D0 acceptance

```text
corrected caller table                           = source-backed
current normal representative corpus             = finite and source-backed
first rejection stage per representative row     = exact
unclassified representative corpus rows          = 0
GeneralProgram catch-all branch                  = 0
first module-source authority                     = exact
verified-plan raw-AST route escape               = 0
Builder effects during classification            = 0
Lower-side family reclassification               = 0
compile_legacy_candidate in target owner          = 0
MirBuilder::build_module(ASTNode) in target owner = 0
normal publication/result parity contract        = exact
future Candidate A caller/delete set              = exact
production caller during D0                       = 0
production source/test/check edit                 = 0
replacement manifest row delta                    = 0
fallback / retry / reselection                    = 0
```

## Structural boundary

The MirBuilder structural observation remains unchanged during this D0:

```text
source files / baseline = 952 / 952
source LOC   / baseline = 182452 / 182452
test files   / baseline = 139 / 139
test LOC     / baseline = 40809 / 40826
```

These numbers are measured consequences, not an implementation-permission
gate. Compiler-ingress implementation may legitimately add a named owner and
must report compiler/runner files and LOC before/after/delta. It may not add a
generic “misc” module, a per-cell guard, or a second default router.

## Hard stop

Stop and return to a narrower owner design if:

```text
the current normal accepted corpus cannot be enumerated
GeneralProgram needs an Other/Unknown/Legacy residual
the plan exposes bare AST for route redecision
one compile owner must duplicate all expression/statement semantics
imports or source identity are reconstructed after classification
current normal MirCompileResult parity cannot be stated
canonical rejection is needed to discover the GeneralProgram branch
compatibility/reference callers cannot be separated from normal admission
the future atomic old-edge delete set cannot be named
```

## Explicit non-claims

This accepted task does not authorize:

```text
production source, test, guard, or manifest edits
tenth replacement-cell credit
Candidate A implementation
a GeneralProgram fallback owner
canonical probe followed by Legacy
default backend selection changes
language/grammar/runtime/backend/result-policy changes
source reread/reparse or AST rewrite
REPL/JSON/reference removal
non-Program root descent
Stage-B or Ownership activation
selfhost migration
proof consolidation or dead-facade cleanup
```

## One-line handoff

```text
Do not build the typed default front yet.

First run the bounded docs interlude. Then close
NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0, add verified function-plan
vocabularies one at a time, aggregate only closed products, preserve the
current-normal result contract, and only then reopen Candidate A.
```
