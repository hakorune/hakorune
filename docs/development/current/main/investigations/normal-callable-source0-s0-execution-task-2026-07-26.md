---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-CALLABLE-SOURCE0-S0
Scope: convert one sealed CallableModule source family into one Program-owned Main-plus-top-level-helper source unit without AST clone or catalog identity issuance
ceremony_tier: T1 source-owner generalization
proof_inventory_before: sealed normal CallableModule source sites plus existing function-only CAT0 header source unit
new_proofs: exact-site source-unit correspondence and function-only facade parity
retired_or_merged_proofs: none
net_proof_delta: one durable exact-site constructor proof
sunset_budget: existing function-only constructor becomes a thin facade
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
Related:
  - docs/development/current/main/investigations/normal-source-plan0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-main0-source0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-main0-vmref0-p0-execution-task-2026-07-26.md
---

# NORMAL-CALLABLE-SOURCE0-S0

## Outcome

Consume:

```text
SealedNormalCallableModuleSourceV1
```

once and produce:

```rust
pub(crate) struct VerifiedNormalCallableSourceUnitV1 {
    // one owned original Program
    // exact Main box/method sites
    // exact top-level helper declaration sites
}
```

The Program is not split, cloned, rewritten, or normalized. Main and helpers
remain views into one immutable owner.

## First profile boundary

Admit in this row:

```text
one exact static Main.main/0 site
one or more additional top-level FunctionDeclaration sites
```

Reject:

```text
Main-box additional methods
instance methods
receiver-bearing callables
non-function additional sites
duplicate/out-of-range sites
site/type drift
```

Main-box helper methods belong to:

```text
NORMAL-STATIC-METHOD-CATALOG0
```

They must not block the first canonical-core profile.

## One Program owner

Generalize the existing callable header source unit:

```rust
impl VerifiedCallableHeaderSourceUnitV1 {
    fn seal_header_surface(program: ASTNode) -> Result<Self, ...>;

    fn seal_exact_sites(
        program: ASTNode,
        sites: Box<[SourceCallableDeclarationSiteV1]>,
    ) -> Result<Self, ...>;
}
```

`seal_exact_sites` validates:

```text
root is Program
site set non-empty
every site is in range
every site is unique
every selected statement is FunctionDeclaration
declaration-site order is deterministic
```

It does not require every top-level statement to be a function. Unselected
statements remain owned but are not callable catalog rows.

The existing function-only constructor must collect its exact sites and call
the same internal exact-site verifier. Two independent header validation
implementations are forbidden.

## Normal owner projection

The normal source plan already owns:

```text
original PreparedNormalSourcePlanInputV1
exact Main box site
exact Main.main/0 method site
additional callable sites
```

Add one consuming internal split inside `normal_source_plan`; do not expose:

```text
into_ast()
ast()
source_text()
clone_program()
```

The projection maps only:

```text
NormalAdditionalCallableSiteV1::TopLevel
  -> SourceCallableDeclarationSiteV1
```

`NormalAdditionalCallableSiteV1::MainMethod` is a typed rejection before
catalog sealing.

The output should retain the generalized
`VerifiedCallableHeaderSourceUnitV1` as its sole Program owner plus exact Main
site evidence. It must not keep a second AST owner.

## Borrowed views

Required bounded access:

```text
exact Main function view
exact helper declaration sites
existing callable header/body lookup by sealed site
source identity/diagnostic receipt
```

No consumer can enumerate arbitrary top-level statements and decide a new
source family.

`CallableFunctionSyntaxViewV1` remains the generic exact function header/body
view. Raw Main expansion/source-facts owners are forbidden because they would
reselect the source family.

## Failure retention

```rust
pub(crate) struct RejectedNormalCallableSourceV1 {
    owner: SealedNormalCallableModuleSourceV1,
    stage: NormalCallableSourceStageV1,
    error: NormalCallableSourceErrorV1,
}
```

Stages:

```text
MainRelation
HelperSiteProjection
HeaderSourceUnit
FamilyClosure
```

Errors retain exact site/index/kind facts. Public terminals:

```text
stage()
error()
discard(self)
```

No owner recovery, AST extraction, retry as ScalarRoot, retry as Raw,
fallback, or partial catalog.

## Implementation order

```text
CS-A EXACT-SITE0
  SourceCallableDeclarationSiteV1 checked constructor
  exact-site header source-unit verifier
  function-only facade parity

CS-B NORMAL-PROJECTION0
  consuming normal CallableModule source projection
  Main-method helper typed rejection

CS-C OWNER0
  VerifiedNormalCallableSourceUnitV1
  one Program owner + Main/helper evidence

CS-D FIXTURE/G0
  exact source/site/rejection matrix
  catalog/Builder/MIR/runtime callers zero
```

## Fixture matrix

Success:

```text
Main.main/0 + one top-level static helper
Main.main/0 + multiple top-level static helpers
helper declaration reorder preserves exact selected meaning
existing function-only Program facade parity
repeated borrowed lookup retains the same AST identity
```

Rejection:

```text
Main-box additional helper method
empty helper set
duplicate helper site
out-of-range helper site
selected site is not FunctionDeclaration
Main site/helper site overlap or drift
non-static/non-profile helper is retained for later catalog rejection,
  unless the source-family contract itself excludes it
```

This row validates source ownership and exact sites. Exact-i64 callable profile,
duplicate callable key/symbol, owner issuance, call graph, and body semantics
remain the existing downstream catalog/plan authorities.

## Structural gate

```text
owned Program count                                = 1
exact-site header verifier                         = 1
function-only facade implementation                = 1 thin

AST clone/rewrite                                  = 0
bare AST/source accessor                           = 0
source-family reclassification                     = 0
Raw Main/source-facts use                          = 0

callable owner issuance                            = 0
catalog sealing                                    = 0
Builder/MIR/backend/runtime reference              = 0
production runner/CLI caller                       = 0
fallback/retry                                     = 0

all modified/new source/check files                < 800 lines
```

Extend an existing source/callable authority guard where practical. No
per-row shell wrapper.

## Immediate continuation

```text
NORMAL-CALLABLE-SOURCE0-S0
-> NORMAL-MAIN-DIRECT-CALL0-S0
-> NORMAL-CALLABLE-MODULE0-A0-S0
```

`NORMAL-MAIN-DIRECT-CALL0-S0` seals Main call sites against the one complete
helper catalog. This source row does not inspect calls.

## Non-claims

```text
callable catalog owner issuance
Main direct-call activation
acyclic/SCC plan activation
module transaction/publication
Main-box helper support
instance methods/receiver
imports/using
String/object/dynamic result
runner/default/product cutover
Legacy or Raw retirement
```
