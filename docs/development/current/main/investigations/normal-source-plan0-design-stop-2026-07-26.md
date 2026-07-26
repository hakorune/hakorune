---
Status: accepted decision and durable task order
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Closes: NORMAL-SOURCE-PLAN0-D0
Classification: source-owned one-shot sealed plan
First executable row: NORMAL-SOURCE-PLAN0-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/normal-source-plan0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-entry-cutover-d2-consultation-2026-07-26.md
  - docs/development/current/main/investigations/mirbuilder-core-complete0-proof-task-2026-07-26.md
  - docs/reference/language/function-exit-and-entry-result.md
---

# NORMAL-SOURCE-PLAN0-prime-r1

The bounded MirBuilder core and the explicit narrow normal-file VM-reference
lane are closed. The next layer classifies one parsed normal source exactly
once, then hands the sealed family to existing Script, function, callable, and
entry-result owners. It does not create another compiler.

## Accepted authority

```text
owned parsed normal source
  -> NormalSourcePlanClassifierV1
  -> SealedNormalSourcePlanV1
       ScalarRoot::Script
       ScalarRoot::Main0
       CallableModule
```

The classifier owns source-family truth only. Profile admission is deliberately
separate:

```text
SealedNormalSourcePlanV1
+ SealedNormalEntryProfileV1
  -> NormalSourcePlanAdmissionV1
  -> AdmittedNormalSourcePlanV1
```

This split corrects a potential authority overlap in the consultation draft.
`NormalSourcePlanClassifierV1` never matches a profile, backend, result carrier,
or route. `NormalSourcePlanAdmissionV1` never reclassifies source.

The profile relation is fixed:

```text
NormalFileNoImportVmReferenceV1
  = frozen narrow reference profile

NormalFileCanonicalCoreVmReferenceV1
  = separate name, evidence, activation, and CLI request
```

In-place growth or semantic reinterpretation of the existing narrow profile is
forbidden.

## Source-family law

| Source surface | Sealed result |
| --- | --- |
| no declarations | `ScalarRoot::Script` |
| static `Main.main/0` as the only callable | `ScalarRoot::Main0` |
| `Main.main/0` plus one or more additional callable sites | `CallableModule` |
| ordinary function(s), no Main | `MissingSourceEntry` |
| Script statements mixed with Main/functions | `MixedSourceFamilies` |
| duplicate Main | `DuplicateMain` |
| instance Main | `MainMustBeStatic` |
| Main without `main` | `MainMethodMissing` |
| `Main.main` arity other than zero | `MainArityMismatch` |
| unsupported top-level declaration | `UnsupportedTopLevelSurface` |

The sealed product owns the original parsed source. It exposes no `ast()`,
`into_ast()`, source-text, reclassification, retry-as-scalar, or
retry-as-callable terminal.

Main-box method inventory is deterministic. The AST stores methods in a map,
so source-plan evidence uses the top-level Main site plus sorted method keys;
it does not claim lexical method order or duplicate-method detection.

## Existing owners retained

```text
Script result:
  RawScriptResultContractV1
  -> RawScriptBodyRecipeV1

Main/function result:
  VerifiedFunctionCompletionV1
  -> SealedFunctionExitContractV1
  -> PreparedFunctionDraftSealV1
  -> CompletedFunctionDraftV1

helper catalog/graph:
  VerifiedCallableHeaderSourceUnitV1
  -> VerifiedResolvedCallableModuleV1
  -> VerifiedCallableGraphInventoryV1
  -> VerifiedAcyclicCallableGraphV1
  -> later VerifiedCallableSccPartitionV1

publication:
  one generalized canonical module batch
  -> ModuleDraftCollectorV1
  -> atomic candidate-module commit

execution/result:
  one backend-neutral published source-entry invocation
  -> exact VM-reference execution
  -> SourceEntryResultV1
  -> ProcessExitProjectionV1
```

The normal classifier does not use `VerifiedRawRootExpansionV1` as its family
authority: that product has only Script/App vocabulary, does not enforce all
normal Main laws, and lives under the Builder layer.

The normal classifier also does not manufacture
`SourceCallableDeclarationSiteV1`. It passes one owned Program and sealed
normal sites to the callable-catalog owner, which verifies and converts them
inside its own boundary. No helper-only cloned Program is created.

## First canonical-core capability

The first vertical profile is intentionally narrow:

```text
Script:
  existing scalar-and-Unit Script result

Main:
  static Main.main/0
  no receiver/capture/attributes
  call-free until NORMAL-MAIN-DIRECT-CALL0-S0
  zero or one terminal root Return
  Unit / Integer / Bool / Float
  unannotated / :void / exact :i64

helpers:
  top-level free static functions
  exact i64 parameters and result
  singleton/call-free helper set
  deterministic acyclic graph

entry:
  one source Main function
  one synthetic physical main thunk
  one exact co-sealed relation
```

String function results, Main-box helpers, instance methods, receiver/capture,
dynamic carriers, nested/multiple/all-path Return, cleanup-bearing Return, and
imports remain typed capability rejections.

Main remains outside the helper catalog. Existing keys retain their roles:

```text
source Main draft  = FunctionDraftKeyV1::CanonicalResolvedOwner
helper draft       = FunctionDraftKeyV1::CanonicalCallable
physical thunk     = FunctionDraftKeyV1::Main
```

The Raw `Main` slot replacement policy is not reused for the canonical thunk.

## Failure and reuse law

Source classification rejection retains the complete input:

```rust
struct RejectedNormalSourcePlanV1 {
    owner: PreparedNormalSourcePlanInputV1,
    stage: NormalSourcePlanStageV1,
    error: NormalSourcePlanErrorV1,
}
```

S0 stages are source-only:

```text
RootSurface
SourceEntry
FamilyClosure
```

Profile exclusion belongs to `RejectedNormalSourcePlanAdmissionV1`, not this
classifier.

Rejections expose inspection plus `discard(self)` only. They do not expose the
owner, retry, resume, fallback, another profile, or another source family.

Every pre-execution rejection preserves:

```text
live Builder mutation      = 0
partial module publication = 0
profile reselection        = 0
Legacy retry               = 0
```

After VM execution begins, VM Fault remains a normal terminal path through
`SourceEntryResultV1::Fault`.

## Corrected executable order

The accepted umbrella is:

```text
NORMAL-CANONICAL-CORE0
```

Worker inventory found four necessary structural seams. They are part of the
accepted umbrella and do not reopen D0:

1. source classification and profile admission are separate;
2. Main/helpers/thunk need one heterogeneous canonical batch owner;
3. Main plus one helper needs a singleton/call-free helper-plan row before the
   existing multi-function DAG plan;
4. the sole `NormalSourcePlanCompilerV1::consume()` dispatch must be an
   explicit row before profile activation.

### A. Source plan

```text
NORMAL-SOURCE-PLAN0-S0
-> NORMAL-SOURCE-PLAN0-INPUT0-S0
-> NORMAL-SOURCE-PLAN0-G0
```

S0 is disconnected and source-only. INPUT0 adds one consuming projection from
`PreparedNormalFileSourceV1`; the existing narrow production terminal remains
unchanged. G0 freezes classifier=1, reclassification=0, production consumer=0.

### B. Main-only canonical module

```text
NORMAL-MAIN0-SOURCE0-S0
-> NORMAL-MAIN0-F1-PLAN0-S0
-> NORMAL-MODULE-TX0-L0
-> NORMAL-MAIN0-THUNK0-S0
-> NORMAL-CANONICAL-MODULE-BATCH0-S0
-> NORMAL-MAIN0-TX0-I0
```

`NORMAL-MODULE-TX0-L0` defines one common transaction schema.
`NORMAL-CANONICAL-MODULE-BATCH0-S0` adds the canonical heterogeneous
Main/helpers/thunk manifest, receipt, and drain. Main-only is its two-draft
specialization; callable modules later consume the same owner.

### C. Shared VM-reference terminal

```text
SOURCE-ENTRY-VMREF-NEUTRAL0-L0
-> SOURCE-ENTRY-VMREF-RAW-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF0-P0
```

L0 creates no consumer. The Raw adapter proves existing exact-target,
status, and diagnostic parity before the Main adapter connects. VM execution,
process projection, and diagnostic owners remain singular.

### D. Main plus helpers

```text
NORMAL-CALLABLE-SOURCE0-S0
-> NORMAL-MAIN-DIRECT-CALL0-S0
-> NORMAL-HELPER-MODULE-PLAN0-S0
-> NORMAL-CALLABLE-MODULE0-A0-S0
-> NORMAL-CALLABLE-MODULE0-TX0-I0
```

The source row generalizes the existing function-only header unit to one owned
Program plus an exact verified helper-site set. The helper-plan row admits a
singleton/call-free helper set; the existing acyclic owner remains the
deterministic multi-helper graph authority.

Recursive SCC support is deliberately not an initial production blocker:

```text
NORMAL-CALLABLE-MODULE0-R0-S0
```

It follows canonical-core G0 as a separate capability/parity row.

### E. Sole dispatch and separate profile

```text
NORMAL-SOURCE-PLAN0-ADMISSION0-S0
-> NORMAL-SOURCE-PLAN0-DISPATCH0-I0
-> NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a
-> NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0
-> NORMAL-FILE-CANONICAL-CORE0-CALLER0-I0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0b
-> NORMAL-FILE-CANONICAL-CORE0-G0
-> MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
```

The dispatcher consumes one admitted plan and selects exactly one already
sealed owner. It never tries Raw, then canonical, then Legacy.

Current progress:

```text
NORMAL-SOURCE-PLAN0-S0        = closed
NORMAL-SOURCE-PLAN0-INPUT0-S0 = active
```

### F. Promotion and completion

```text
NORMAL-CALLABLE-MODULE0-R0-S0
-> NORMAL-ENTRY-PRODUCT-BACKEND-D0
-> NORMAL-DEFAULT-CALLER-CENSUS0-P0
-> NORMAL-ENTRY-PROMOTION-D3
-> NORMAL-IMPORT-BUNDLE0
-> MIRBUILDER-LEGACY-FENCE0
-> MIRBUILDER-NORMAL-COMPLETE0
-> MIRBUILDER-COMPLETE0-G0
```

Canonical-core green never automatically changes the default backend or an
existing caller.

## Reconsult only on three contradictions

Open a new design stop only if implementation evidence proves one of these:

1. Main cannot remain under the original Program owner while entering the
   existing function-source projection;
2. Main direct calls cannot reuse the existing callable index/call-row owner;
3. Raw and canonical publication cannot share one neutral exact-entry VM owner.

Heterogeneous batch and singleton-helper gaps are known BoxShape tasks inside
this umbrella, not new consultation triggers.

## Proof lifecycle

```text
sunset_id:
  NORMAL-SOURCE-PLAN0-PROOF-SUNSET-001

retirement owner:
  NORMAL-SOURCE-PLAN0-G0

retire_when:
  canonical-core production consumer = 1
  + source-plan fixtures absorbed by the reusable normal route guard
  + disconnected source-plan proof consumer = 0

target evidence:
  NORMAL-FILE-CANONICAL-CORE0-G0
```

Do not create one shell guard per row. Grow one `normal-source-plan0` family
guard, then merge its durable assertions into the normal route guard.

## Non-authority and non-claims

```text
last Builder ValueId
module/function symbol scan
CLI backend string after typed selection
Legacy compile retry
AST/source text rewrite
process status as source-result classification
default backend/caller cutover
compile_with_source change
imports / using
JSON / REPL / Stage1
LLVM/native/ny_main
dynamic/object result carrier
Main-box helper methods
instance methods / receiver
cleanup activation
old Raw retirement
executor / selfhost / fastmem
CUT0
```
