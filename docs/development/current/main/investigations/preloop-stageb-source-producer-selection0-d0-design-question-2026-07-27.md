---
Status: accepted design; execution task map issued
Date: 2026-07-27
Decision: PRELOOP-STAGEB-SOURCE-PRODUCER-SELECTION0-prime-r1
Closes:
  - PRELOOP-STAGEB-SOURCE-PRODUCER-SELECTION0-D0
Blocked by:
  - none
Closes before this stop:
  - PRELOOP-STAGEB-CARRIER-CORRESPONDENCE0-P0
  - PRELOOP-OUTER-CARRIER-RESULT-CONTRACT0-S0
  - PRELOOP-STAGEB-SOURCE-ACTIVATION0-S0
  - PRELOOP-STAGEB-FUNCTION-PREPARATION0-S0
Next executable row:
  - PRELOOP-STAGEB-SOURCE-INVENTORY0-P0
Related:
  - preloop-stageb-carrier-handoff0-d0-design-question-2026-07-27.md
  - preloop-stageb-source-producer-selection0-prime-r1-task-map-2026-07-27.md
  - src/mir/preloop_stageb_carrier/activation.rs
  - src/mir/builder/module_lifecycle.rs
  - src/mir/builder/calls/instance_method_draft_preparation.rs
---

# Pre-loop Stage-B Source Producer Selection

## Accepted closeout

```text
producer:
  PreloopStageBWholeSourceProducerV1

exact production seam:
  MirCompiler::compile_request
  / MirLoweringRequestV1::Legacy arm

candidate zero:
  explicit Ordinary product

alias authority:
  compiler-supplied typed using/import snapshot

selected install:
  activation plan
    -> consuming module-activation preparation
    -> fallible preflight
    -> infallible catalog + alias install
    -> stack-owned single-use function ledger

first executable row:
  PRELOOP-STAGEB-SOURCE-INVENTORY0-P0
```

Option A1 is accepted. Direct `MirBuilder::build_module` callers remain outside
this bounded activation. Program(JSON v0), REPL compatibility, Raw publication,
and public/CLI/environment selectors remain unchanged.

The implementation task map is:

```text
docs/development/current/main/investigations/
  preloop-stageb-source-producer-selection0-prime-r1-task-map-2026-07-27.md
```

One refinement is mandatory. The activation plan must not expose a general
catalog/row tuple escape hatch. Its sole consuming terminal prepares the
module activation directly, so the same-allocation catalog and owned row cannot
be separated and reused by another caller.

## Why this is a real design stop

The bounded source proof and the behavior-neutral instance-method preparation
seam are now closed. The missing authority is not another Builder adapter. It
is the sole whole-source owner allowed to decide whether one source contains
the exact pre-loop carrier row.

Today the legacy Builder seals and installs its callable declaration catalog
inside `MirBuilder::lower_root()`. The new activation plan instead owns the
exact boxed catalog allocation beside one owned row:

```text
VerifiedPreloopStageBCarrierActivationPlanV1
  = Box<VerifiedSameModuleCallableDeclarationCatalogV1>
  + OwnedPreloopStageBCarrierRowV1
```

Those two authorities cannot be independently resealed and later compared by
value. The selected producer must create the row from the same allocation that
is eventually installed into the candidate Builder.

## Landed facts

```text
common instance-method preparation:
  skeleton / declared signature / runes / uses / receiver / params

intentionally separate:
  legacy body driver
  port-aware body driver
  legacy current-module finalizer
  port-aware short-lived header finalizer

ordinary versus port-aware parity:
  empty body green
  explicit scalar Return green

activation production consumer:
  0

Builder source-site registry:
  0

catalog reseal:
  0
```

## The source evidence that must be available together

The producer must observe one owned, resolver-visible source unit and derive:

```text
one exact declaration catalog allocation
one exact caller key
one exact root assignment
one exact outer static Call site
one structural CallArgument(1)
one exact inner same-owner MethodCall site
one exact inner Integer contract
one exact static target result requirement:
  ExactI64(required=[Argument(1)])
one prefix / selected / suffix body schedule
one typed using/import alias snapshot
```

It must not recover any of these from Builder state, callee spelling, Box
spelling, emitted MIR, or runtime values.

## Q1 — Which whole-source owner produces the optional activation?

### Option A1 — `MirCompiler::compile_request` Legacy arm

One audit recommends the sole legacy route-selection site:

```text
MirLoweringRequestV1::Legacy
  -> whole-source selection
  -> Ordinary: existing compile_with_source_internal
  -> Selected: exact Stage-B candidate ingress
```

Advantages:

```text
exact production selector caller = 1
compile_with_source and compile_with_source_and_imports already converge here
selected profile cannot leak into direct MirBuilder callers
candidate zero can use the unchanged build_module route
```

Required follow-up:

```text
Selected needs a new typed Builder ingress that consumes the prepared install
build_module / lower_root must not reseal the selected catalog
```

### Option A2 — `MirBuilder::build_module` before `prepare_module()`

Another audit recommends the earliest common Builder entry:

```text
Raw-root expansion preflight
-> whole-source selection
-> prepare_module
```

Advantages:

```text
all direct build_module callers see one source owner
actual legacy AST and configured import aliases are already together
selection still precedes prepare_module effects
```

Risk:

```text
the bounded Stage-B capability becomes active for every direct Builder caller
candidate-zero and unsupported-inventory law must preserve every such caller
production consumer census is wider than one compiler Legacy arm
```

### Option B — Builder-owned selection inside `lower_root()`

`MirBuilder::lower_root()` receives the bare AST, seals the catalog, derives
the optional activation, installs the catalog, and continues.

Advantages:

```text
close to the current catalog install
small caller surface
```

Risks:

```text
Builder becomes source-policy owner
classification and rejection occur after module preparation has opened
activation proof construction is mixed into legacy orchestration
harder to retain the complete source owner on rejection
```

### Option C — Raw published-compile source-facts owner

The existing Raw source-facts/package chain produces the activation.

Advantages:

```text
typed pre-physical failure chain already exists
source facts already own one callable catalog
```

Risks:

```text
the current Stage-B frontier is the legacy whole-source route
Raw NarrowV1 admission is not the same source profile
using Raw only as a catalog factory would import Raw lifecycle authority
```

## Q2 — What does candidate zero mean?

Choose one:

```text
A. explicit Ordinary product
   source was inventoried and no candidate was selected

B. Option::None
   absence alone selects the unchanged ordinary route

C. typed rejection
   this production ingress accepts only the exact selected fixture family
```

Candidate one must own exactly one activation. Candidate many, ambiguous
source relations, or a selected row that drifts before install must be typed
pre-Builder rejection. No path may try selected lowering and then retry the
ordinary route.

## Q3 — Which alias/import authority is admitted?

The producer needs target identity across the real whole-source input. Select
the exact source of aliases:

```text
A. compiler-supplied typed using/import snapshot
B. merged canonical AST only
C. ambient Builder CompilationContext lookup
```

The accepted choice must state whether `.hako` text-merge has already
normalized the source before this owner. Callee and Box names are diagnostic
only.

## Q4 — What is the sole consuming install terminal?

The activation plan currently exposes read-only row evidence only. D0 must
select one consuming terminal with this shape:

```text
VerifiedPreloopStageBCarrierActivationPlanV1
  -> PreparedPreloopStageBModuleActivationV1
     - exact declaration catalog moves into Builder install
     - exact row stays in a module-owned single-use ledger
  -> selected caller capture once
```

The terminal must prove:

```text
borrowed source evidence before install = 0
catalog reseal                          = 0
duplicate install                       = typed rejection
selected caller consumption             = exactly one
unobserved selected caller               = typed rejection
selected caller retry                    = 0
ordinary function behavior delta         = 0
```

## Q5 — Which production ingress is allowed to call it?

Name one exact caller. Do not answer with “normal compiler”, “legacy route”,
or “Raw route” as a family. The decision must fix:

```text
source request owner
one constructor
one consumer
feature/profile visibility
candidate-zero behavior
error/status projection
retirement condition
```

## Recommended decision criteria

Prefer the option that satisfies all of these without a second source engine:

```text
source classification once
same declaration catalog allocation
Builder effects after selection only
one consuming catalog install
one selected function capture
ordinary route unchanged
typed 0 / 1 / many disposition
typed alias snapshot
failure owner retention
fallback / retry zero
```

## Audit synthesis and recommendation

Both read-only audits agree on the internal architecture:

```text
VerifiedWholeSourceStaticCallTargetInventoryV1
  location: src/mir/source_call_target/whole_source_inventory.rs

PreloopStageB source selection
  location: src/mir/preloop_stageb_carrier/source_selection.rs

catalog:
  Box-sealed once
  borrowed within one lexical proof scope
  moved after every source borrow ends

0:
  explicit Ordinary + bounded unavailable disposition

1:
  Selected exact activation

many / selected drift:
  typed pre-Builder rejection
```

They disagree only on the external seam:

```text
A1 compile_request Legacy arm:
  strongest exact-one production caller boundary

A2 build_module pre-prepare:
  strongest all-direct-Builder-entry uniformity
```

Recommended for this bounded row: **A1**.

The reason is scope, not convenience. The accepted Stage-B carrier decision
authorizes one selected production ingress while requiring ordinary route
delta zero. `compile_request` is already the sole route-selection authority,
and both legacy public compiler facades converge there. Direct
`MirBuilder::build_module` callers are not silently reclassified.

This recommendation is conditional on one strong rule:

```text
Selected does not call ordinary build_module and then override it.
Selected consumes a typed install request whose catalog vacancy is preflighted,
then commits catalog + activation ledger infallibly before declaration/body
lowering. lower_root does not reseal.
```

## Superseded provisional task series

The dependency order below led to the accepted task map. Execute the linked
task map rather than treating this historical sketch as the live queue.

```text
PRELOOP-STAGEB-SOURCE-PRODUCER-SELECTION0-D0-CLOSEOUT

-> PRELOOP-STAGEB-SOURCE-INVENTORY0-P0
   exact whole-source producer / alias / 0-1-many correspondence

-> PRELOOP-STAGEB-SOURCE-SELECTION0-S0
   disconnected one-shot selection product

-> PRELOOP-STAGEB-MODULE-ACTIVATION0-S0
   consuming same-allocation catalog install preparation

-> PRELOOP-STAGEB-FUNCTION-INGRESS0-I0
   exact selected instance-method capture

-> PRELOOP-STAGEB-FUNCTION-INGRESS0-P0
   selected / ordinary / drift / reuse matrix

-> PRELOOP-STAGEB-FUNCTION-INGRESS0-G0

-> UNIFIED-CALL-OUTER-CARRIER-RECEIPT0-S0
-> PRELOOP-OUTER-CARRIER-RECEIPT0
-> PRELOOP-OUTER-CARRIER-TYPE-I0
-> CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
```

## Non-claims

```text
production Stage-B activation
default Raw cutover
whole port-aware Raw cutover
general instance-method result inference
loop-refresh activation
GenericLoop type publication
ownership grammar
Alias / View language semantics
parser / VM / backend changes
fallback / retry
```

## Required answer

This answer is now closed by
`PRELOOP-STAGEB-SOURCE-PRODUCER-SELECTION0-prime-r1`.

```text
Decision:
  PRELOOP-STAGEB-SOURCE-PRODUCER-SELECTION0-prime-r1

Status:
  accepted | provisional | rejected

Q1 producer:
  exact owner and exact code seam

Q2 candidate zero:
  Ordinary | None | reject

Q3 alias authority:
  exact typed source

Q4 consuming install:
  exact owner chain and failure retention

Q5 production ingress:
  exact one caller or caller-zero continuation

first executable row:
  exact row
```
