---
Status: closed
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-MAIN-DIRECT-CALL0-S0
Scope: seal one complete top-level helper catalog and resolve call-free or finite exact Main.main/0 direct calls against it before Builder effects
ceremony_tier: T1 source/catalog semantic plan
proof_inventory_before: one Program-owned normal callable source plus existing CAT0 catalog and finite direct-call owners
new_proofs: Main-role catalog correspondence and exact Main-to-helper call rows
retired_or_merged_proofs: none
net_proof_delta: one durable Main-role consumer of existing catalog/call vocabulary
sunset_budget: no temporary second catalog or Main-only call resolver
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
Related:
  - docs/development/current/main/investigations/normal-callable-source0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-main0-f1-plan0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
---

# NORMAL-MAIN-DIRECT-CALL0-S0

## Outcome

Consume one normal callable source and seal:

```text
one complete immutable top-level helper catalog
+ one exact embedded Main.main/0 semantic/source unit
+ zero-or-more exact Main -> helper direct-call rows
+ one retained helper-catalog resolver continuation
```

The output is Builder-free:

```rust
pub(crate) struct VerifiedNormalMainDirectCallPlanV1 {
    main: VerifiedNormalMainFunctionPlanV1,
    helpers: CallableCatalogSealOutcomeV1,
    calls: Box<[VerifiedNormalMainDirectCallRowV1]>,
    _seal: VerifiedNormalMainDirectCallPlanSealV1,
}
```

The concrete ownership layout may use a single combined owner rather than the
illustrative fields, but it must preserve one Program, one helper catalog, and
one still-usable helper resolver continuation.

## One catalog authority

Reuse:

```text
VerifiedCallableHeaderSourceUnitV1 exact helper sites
-> VerifiedOwnerFreeCallableCatalogSourceUnitV1
-> CallableCatalogSealOutcomeV1
```

Do not create:

```text
Main catalog
entry catalog
second helper index
symbol-only lookup table
AST-cloned function-only Program
```

Main is not inserted as a helper catalog row. Its source owner remains the
canonical Main F1 owner; helpers retain `CanonicalCallableKeyV1` identities.

## Main semantic resolution

Generalize the existing embedded Main resolver with one explicit borrowed
callable-index input:

```text
resolve embedded Main root/forest
using its exact reserved Main owner
+ borrow complete helper index for FunctionCall resolution
```

The no-call Main path remains a thin facade over the same implementation with
an explicit empty-call capability. It must not build a dummy catalog.

Forbidden:

```text
resolve_function(ASTNode) consuming clone
from_exact_parts_without_callable on a call-bearing Main
source name lookup during lowering
physical symbol lookup as source authority
retry call-free after call-plan failure
```

## First direct-call profile

Admit:

```text
caller =
  static Main.main/0
  receiver/capture/uses/contracts/attrs absent

target =
  top-level free static helper
  exact existing i64 parameter/result profile

calls =
  zero or more finite direct FunctionCall sites
  nested expression calls allowed when existing finite-call observation admits
  exact arity
  exact helper key/ref/symbol relation
```

Reject:

```text
Main self recursion
unresolved target
physical-symbol spelling at source
arity mismatch
MethodCall/receiver
Main-box helper
instance helper
String/object/dynamic argument or result
Lambda/capture/Outbox
unsupported backend capability
```

Helper-to-helper graph analysis remains the following A0/R0 rows. This row
only seals Main's outgoing entry edges and the complete helper catalog they
reference.

## Main zero-parameter capability

Current generic direct-call preflight rejects call-bearing zero-parameter
functions. Add one named role policy:

```text
CanonicalFunctionRolePolicyV1::NormalMainDirectCall0
```

It may relax only caller arity for the already-sealed static Main role. It does
not change target signature, result carrier, return semantics, receiver, or
ordinary function admission.

The same source-call observation and resolved call-row owner used by ordinary
callables must be reused. No Main-specific AST call walker.

## Failure retention

Preparation order:

```text
1. exact helper-site/header validation
2. owner-free helper catalog validation
3. helper owner/index sealing
4. embedded Main resolution with borrowed helper index
5. Main F1 completion/result preflight
6. Main call-row/index correspondence
7. issue verified combined plan
```

All fallible source/header/profile work must occur before consuming away the
only recoverable normal source owner. If an existing catalog terminal drops
the Program on rejection, introduce a borrow-only preparation product rather
than cloning or weakening rejection retention.

```rust
pub(crate) struct RejectedNormalMainDirectCallPlanV1 {
    owner: OpenNormalCallableSourcePlanV1,
    stage: NormalMainDirectCallStageV1,
    error: NormalMainDirectCallErrorV1,
}
```

Stages:

```text
HelperCatalog
MainResolution
MainFunction
CallRows
Correspondence
```

Only `stage()`, `error()`, and `discard(self)`. No retry, fallback, owner
recovery, partial catalog, or Builder entry.

## Implementation order

```text
MC-A CATALOG-INPUT0
  consuming split of VerifiedNormalCallableSourceUnitV1
  one Program/helper-site/Main-evidence owner

MC-B CATALOG-PREP0
  borrow-only exact-i64 helper catalog preparation
  rejection retains complete normal source

MC-C MAIN-RESOLVE0
  embedded Main resolution with borrowed complete helper index

MC-D CALL-ROWS0
  existing finite direct-call observation/profile
  exact Main-to-helper rows

MC-E PLAN/G0
  one combined verified plan
  Builder/MIR/backend callers zero
```

## Fixture matrix

Success:

```text
call-free Main + one helper
Main calls one helper
Main calls forward-declared helper
Main calls backward-declared helper
Main calls multiple helpers
nested finite helper calls in one Main expression
helper declaration reorder preserves normalized meaning
```

Rejection:

```text
missing helper
arity mismatch
physical-symbol source spelling
Main self call
MethodCall
helper header outside exact-i64 profile
duplicate helper key
duplicate helper physical symbol
call-row/catalog cardinality drift
```

Compiler/semantic-owner reuse:

```text
success -> rejection -> success
```

## Structural gate

```text
normal Program owner count                         = 1
helper catalog producer                            = 1
Main catalog membership                            = 0
complete helper index borrowed by Main resolver    = 1

Main-specific AST call walker                      = 0
source/AST clone or rewrite                        = 0
lowering-time name/symbol lookup                    = 0
retry/fallback                                     = 0

Builder/MIR/module publication                     = 0
VM/runner/CLI caller                               = 0
default/product route delta                        = 0
all modified/new source/check files                < 800 lines
```

## Immediate continuation

```text
NORMAL-MAIN-DIRECT-CALL0-S0
-> NORMAL-CALLABLE-MODULE0-A0-S0
-> NORMAL-CALLABLE-MODULE0-R0-S0
-> NORMAL-CALLABLE-MODULE0-TX0-S0
```

Open a design consultation only if the existing catalog/resolver continuation
cannot be retained across both Main resolution and later helper-module
resolution without a second catalog or AST clone.

## Non-claims

```text
helper body graph/lowering activation
acyclic/SCC module plan
atomic normal callable module transaction
Main-box helper
instance method/receiver
imports/using
String/object/dynamic carrier
runner/default/product cutover
Legacy or Raw retirement
```

## Closeout

```text
Status:
  closed

Landed:
  043274ac6c refactor: prepare callable catalog seals
  8cf87877be feat: seal normal Main direct call plan

Owner chain:
  one Program-owned normal callable source
  -> borrow-only owner-free helper candidate preparation
  -> borrow-only helper owner/catalog preparation
  -> infallible catalog commit
  -> same resolver continuation issues Main owner
  -> complete helper index resolves Main FunctionCall sites
  -> call-free or finite direct-call Main capability proof

Structural result:
  helper catalog count        = 1
  Main catalog membership     = 0
  resolver session count      = 1
  AST clone/rewrite           = 0
  call-free retry             = 0
  Builder/MIR/backend caller  = 0

Focused evidence:
  normal_source_plan tests    = green
  Main direct-call plan       = 4 green
  callable catalog tests      = 15 green
  normal source-plan guard    = green
  vm-reference library check  = green
  current-state pointer guard = green
  touched source/check files  < 800 lines

Next:
  NORMAL-CALLABLE-MODULE0-A0-S0
```
