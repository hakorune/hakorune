---
Status: active design stop; bounded module-source census selected
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

## 2026-08-23 source-authority census receipt

The current code confirms that the normal/default gap is an ownership gap, not
an absent transport type:

| Current surface | Evidence | D0 classification |
| --- | --- | --- |
| request transport | `src/mir/compiler/normal_default_pipeline.rs` — `NormalCompileRequestV1` owns prepared Program root, source hint, imports, admission, and result contract | transport only; it does not classify declarations or issue module meaning |
| selected normal compiler | `NormalDefaultPublishedPipelineV1::compile` opens `ModuleBuilderInvocationSessionV1` and calls `complete_normal_default_program_root_catalog_lifecycle_with_target` | first Builder-effect boundary; no source-family redecision may remain below it |
| existing source-plan issuer | `NormalSourcePlanClassifierV1::seal` in `src/mir/compiler/normal_source_plan/classifier.rs` | existing issuer, but its production caller is only `runner/reference/normal_file_vm_frontdoor/source_plan_input.rs`; it is not the normal/default issuer |
| existing source inventory | `NormalSourceSurfaceInventoryV1::collect` | observes Script/top-level callable/Main/non-Main Box/unsupported rows, but currently retains AST-derived sites and rejects the residual module surface |
| existing module product | `VerifiedNormalModuleSourceV1` in `normal_source_plan/module_source.rs` | bounded Main + plain instance-box shape; no normal/default production caller, and not a general module authority |
| default public entry | `MirCompiler::compile_with_source*` -> `NormalCompileRequestV1::for_mir_mode` -> `compile_normal` | still reaches the selected normal root lifecycle without the missing Program/module source co-seal |

Therefore the missing issuer must be named in D0 as a design owner, but no new
Rust `Verified*`/`Prepared*` product is authorized yet:

```text
NormalGeneralProgramModuleSourceIssuerV1   // design name only in D0
  consumes one parser/source-backed Program plus one import/config snapshot
  co-seals exact top-level declaration, entry, and callable/member rows
  emits a finite source disposition before Builder effects
```

`NormalCompileRequestV1` remains transport, and `NormalSourcePlanClassifierV1`
must not silently become a second default classifier through an adapter. D0
must decide whether the existing parser-backed handoff can be transferred into
the normal request once, or whether the missing source owner must be opened at
the frontdoor. AST-only request construction, source-hint names, backend
strings, and `build_module` observations are not authority.

### Finite D0 disposition table

This table is a design inventory, not a runtime enum. Every row has an owner,
pre-effect behavior, continuation, and no fallback:

| State | Sole owner to be named/used | Pre-effect behavior | Continuation | Fallback |
| --- | --- | --- | --- | --- |
| `SourceAuthorityUnavailable` | parser/source handoff validator | stop; no Builder effect | typed terminal | never Legacy retry |
| `CompatibilityOutOfScope` | explicit admission constructor | leave normal classifier; preserve compatibility provenance | separate compatibility owner | never normal reclassification |
| `CanonicalCore` | existing source-plan family issuer after D0 transfer decision | source facts only | existing Script/Main0/Callable plan owner | no residual probe |
| `GeneralModuleSource` | future `NormalGeneralProgramModuleSourceIssuerV1` | source/module facts only | later function-plan slices | no bare AST descent |
| `UnsupportedNormalSurface` | source inventory owner | reject exact top-level kind before effects | typed terminal | no `Other`/`Unknown` bucket |
| `Incomplete` | module-source coverage validator | reject missing declaration/entry/callable relation | typed terminal | no default/empty row |
| `IntegrityInvalid` | source identity/co-seal validator | reject foreign/duplicate/contradictory rows | typed terminal | no relookup/reparse |

The `GeneralModuleSource` row is not accepted merely because Legacy can lower
it. It becomes admissible only after the D0 corpus and source-backed
declaration/entry/member relation are complete. Until then it is a named
design target, not a production candidate.

### Bounded D0 task cells

```text
D0.1 corpus census
  Script: scalar, If, Loop, Return, call/new/field/index
  Main: main/0, main(args), helpers, user boxes, fields, constructors
  Callable: top-level/Main helpers, acyclic and bounded recursive topology
  Module: imports and currently evidenced declaration rows

D0.2 source relation census
  source identity + parser invocation + imports/config
  top-level declaration site/cardinality
  entry site/arity
  user-box/member callable identity and declaration site
  first rejection stage and current-normal result parity

D0.3 issuer decision
  choose one Program-owned issuer location and one move/loan chain
  keep request transport, parser source authority, and Builder session distinct
  record NoSafeSlice if parser provenance cannot reach the issuer exactly once

D0.4 acceptance packet
  finite/disjoint table, no residual Other/Unknown/Legacy state,
  Builder-effect count = 0 during classification, production caller = 0,
  fallback/retry/reselection = 0, and all touched design artifacts named
```

No implementation, fixture, guard-as-permission, production switch, or
`VerifiedNormalGeneralProgramPlanV1` aggregate belongs to these D0 cells.

## 2026-08-23 D0.1/D0.2 corpus and relation receipt

Status: D0.1/D0.2 census recorded; D0.3 accepted; D0.4 records NoSafeSlice for full ingress; Box declaration syntax D0 is next; no implementation permission.

This is a read-only census of the current normal/default surface. It separates
three kinds of evidence so that parser fixtures and disconnected source-plan
tests are not misreported as a production switch:

```text
A = direct `compile_normal` acceptance/parity evidence
B = source-plan or source-backed product evidence with no normal/default caller
C = parser/example syntax evidence only; not a normal acceptance claim
```

| Corpus row | Current evidence | D0 state | Missing source obligation |
| --- | --- | --- | --- |
| Script scalar and ordinary expression/statement rows | A: `normal_script_semantic_source_tests.rs`, `normal_script_*_tests.rs`, and `legacy_candidate_session_tests.rs::real_integer_zero_fixture_uses_the_selected_normal_request` | existing Script core where its source owner is complete | one Program source identity and total Script source coverage; no module reclassification |
| Script control/return and selected declaration rows | A: `normal_script_semantic_source_tests.rs`, `normal_script_root_return_tests.rs`, `normal_script_match_tests.rs`, `normal_script_enum_*_tests.rs` | existing Script corridor or explicit deferred row; not a GeneralModule row | retain source role/site facts without using declaration-name or AST re-probe as a new authority |
| Plain `Main.main/0` | A: `normal_default_root_catalog_lifecycle_tests.rs::verified_expansion_disposition_reaches_script_and_app_root_lowering`; B: `normal_source_plan/tests.rs::main_zero_only_is_a_scalar_main_root` | `CanonicalCore` candidate after source transfer is decided | exact Main declaration/entry relation co-sealed with the Program source authority |
| `Main.main(args)` and Main helpers | A: `normal_default_pipeline_tests.rs::normal_ingress_materializes_required_callable_main_without_changing_script`; A: `legacy_candidate_session_tests.rs::normal_pipeline_matches_legacy_compatibility_for_general_module` | `GeneralModuleSource` first bounded residual | entry arity, helper identities, and callable/member rows before the Builder session |
| Source-backed callable family | A/B: `normal_default_pipeline_tests.rs` callable request tests and `normal_default_root_catalog_lifecycle_tests.rs::source_backed_selected_callable_uses_the_installed_package_port`; B: `normal_source_plan/*callable*` | existing Callable core; topology remains `Acyclic | Recursive` inside its owner | raw normal ingress must not manufacture a second callable authority or lose parser provenance |
| Non-Main instance Box with fields/constructor/methods | A: `legacy_candidate_session_tests.rs::normal_pipeline_matches_legacy_compatibility_for_general_module`; B: `normal_source_plan/tests.rs::main_with_plain_instance_box_seals_module_source` | `GeneralModuleSource` | exact Box/member declaration rows, constructor/field/method identity, and source correspondence |
| Imports/Using plus configuration snapshot | A: `legacy_candidate_session_tests.rs::explicit_imports_commit_only_with_the_finished_normal_candidate`; request imports live in `normal_default_pipeline.rs` | `GeneralModuleSource` | source import rows, source identity, admission/config snapshot, and one co-seal before effects; empty imports are not an absence proof |
| Enum/Brand/TypeAlias/Global/StaticConst/Record declarations | A for selected Script enum/record rows; B: `program_declaration_facts.rs` observes declarations inside the session | finite named residual rows, not `Other` | declaration site/cardinality and demand relation must be source-owned; `PreparedNormalProgramDeclarationFactsV1` is not silently promoted to the Program issuer |
| BuildGate, nested Program, duplicate/mixed/foreign rows | B/C: `normal_source_plan/inventory.rs`, `module_source.rs`, parser source-seal tests, and root lifecycle rejection tests | typed `UnsupportedNormalSurface`, `Incomplete`, or `IntegrityInvalid` | exact first rejection stage and source witness; no repair, reparse, or Legacy retry |

The corpus table is intentionally not a claim that every A row already has a
canonical source product. A rows prove current-normal behavior only. B rows
prove reusable source machinery only. C rows remain outside the acceptance
corpus until a normal caller and source authority are named.

### D0.2 relation boundary observed in the current code

The current request and root lifecycle expose the precise missing seam:

```text
MirCompiler::compile_with_source*
  -> NormalCompileRequestV1::for_mir_mode
  -> PreparedNormalDefaultProgramRootV1::seal(AST)
  -> ModuleBuilderInvocationSessionV1::open_for_token
  -> root expansion / declaration facts / catalog / lower
```

The request currently carries a source hint (`Named | Anonymous`), imports,
admission, and result contract. The raw `for_mir_mode` constructor does not
carry a parser invocation witness or a source-backed declaration handoff.
The existing `NormalSourcePlanClassifierV1::seal` and
`VerifiedNormalModuleSourceV1::seal` are not a substitute: their production
caller is the reference frontdoor, and the module product is the bounded
`Main0WithPlainInstanceBoxes0` shape with no normal/default caller.

Inside the selected normal lifecycle, `VerifiedRawRootExpansionV1::from_program`
and `PreparedNormalProgramDeclarationFactsV1::collect` are currently invoked
after the Builder session has been opened. The compatibility branch also seals
the callable declaration catalog after `prepare_normal_default_module`. These
are useful observations and existing source checks, but they are not one
Program/source authority before the effect boundary.

Therefore the D0 issuer contract is now concrete:

```text
one parser/source-backed Program owner
 + one exact source/import/admission/config snapshot
 + top-level declaration rows and cardinality
 + Main/entry site and arity
 + user-box/member callable identities and sites
 -> one finite disposition before ModuleBuilderInvocationSession::open
 -> no AST-only reclassification below that boundary
```

If the raw normal frontdoor cannot transfer parser provenance exactly once,
that is `SourceAuthorityUnavailable`/`NoSafeSlice`, not permission to use a
source-file name, AST ordinal, Builder catalog, or compatibility retry.

### D0.1/D0.2 completion conditions

```text
1. Each A/B/C row above has one named evidence owner and one D0 state.
2. GeneralModuleSource rows are finite and disjoint by top-level/declaration
   vocabulary; no Other/Unknown/Legacy residual remains.
3. Source identity, parser invocation, imports/config, declaration rows,
   entry relation, and member callable relation have one planned co-seal.
4. The first rejection stage is named before Builder effects; current
   lifecycle stages are not reused as a semantic authority by name alone.
5. The current-normal result contract remains a parity obligation, not a
   source admission shortcut.
6. No Rust product, fixture, guard permission, fallback, or production caller
   is added by this receipt.
```

## 2026-08-23 D0.3 issuer decision

Status: design decision accepted; implementation remains forbidden in this D0.
The canonical shape is a parser-owned source authority plus one normal-ingress
co-seal issuer. The existing parser authority is the substrate, not a reason
to add a second scanner:

```text
one parser invocation
  -> ParserNormalProgramSourceAuthorityDispositionV1
  -> NormalGeneralProgramModuleSourceIssuerV1
  -> source disposition carried by move
  -> NormalCompileRequestV1 transport
  -> ModuleBuilderInvocationSessionV1::open
```

`ParserNormalProgramSourceAuthorityDispositionV1` remains the parser owner of
the invocation witness and ProgramBody rows. A later bounded parser slice must
co-seal the existing `ParserBoxSourceSealV1` and `PreparedCallableSourceV1`
relations into the module-source product, adding only the exact entry row;
the ingress must not scan the AST or rebuild rows from names/ordinals. The design-only
`NormalGeneralProgramModuleSourceIssuerV1` is the sole normal-ingress issuer
that co-seals that parser product with the exact normalized source identity,
imports, admission, configuration snapshot, top-level declaration rows,
Main/entry relation, and user-box/member callable rows. It must run before the
Builder session opens; it may not reparse or reconstruct rows from a request
name, AST ordinal, or Builder catalog.

The request remains transport. `NormalSourcePlanClassifierV1` remains the
bounded reference issuer. `PreparedNormalProgramDeclarationFactsV1`, raw root
expansion, `build_module`, and the compatibility catalog remain observations or
lowering inputs, not the missing source authority.

```text
source authority unavailable / foreign / incomplete / contradictory
  -> typed terminal before Builder effects
source-backed finite module rows
  -> one source disposition
compatibility admission
  -> explicit compatibility owner; never normal retry
```

The move contract is one-way and exact:

```text
prepared source/import snapshot
  + parser source authority
  -> issuer consumes both once
  -> request transports the resulting disposition once
  -> Builder opens only after classification
```

No parallel `Option` source fact, AST-only request constructor, second source
planner, or legacy fallback is part of this design. If parser provenance cannot
reach the issuer exactly once, the result is `SourceAuthorityUnavailable` /
`NoSafeSlice`, not a name-based or Builder-based repair.

### D0.3 acceptance and next task

The issuer choice is accepted only as a design boundary. D0.4 must still write
the finite/disjoint disposition table and an evidence packet proving:

```text
issuer definition/cardinality                 = 1
parser witness transfer                       = exactly once
declaration/entry/member relation co-seal     = before Builder open
AST reconstruction below the boundary         = 0
Builder effects during classification         = 0
normal production caller during D0             = 0
fallback / retry / reselection                = 0
GeneralProgram catch-all / empty defaults     = 0
```

This closes the issuer-selection cell but does not authorize a Rust product, fixture, guard-as-permission, production switch, or aggregate
`VerifiedNormalGeneralProgramPlanV1`. Full-ingress D0.4 is recorded as
`NoSafeSlice`; the next bounded design card is the parser-owned Box declaration
syntax slice, which must remain disconnected from normal production ingress.

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

## Frontier after Script semantic-shelf R0

The Script direct-static semantic-shelf R0 is a completed BoxShape refactor;
it does not widen this design or authorize a default-ingress implementation.
The repository-artifact interlude is also closed at `RETURN0`, so this existing
card is now the current design frontier. The next work is still census-only:
freeze the finite normal corpus, name the Program/module source authority and
issuer, and prove the total pre-effect family boundary before any code,
fixture, fallback, aggregate, or production switch.

A read-only worker review on 2026-08-23 selected this row over
`dynamic_full_body_recipe` relocation. The latter remains compiler-owned and
its Builder move is `NoSafeSlice`; its existing source/Recipe boundary is not
to be reopened by this D0.
