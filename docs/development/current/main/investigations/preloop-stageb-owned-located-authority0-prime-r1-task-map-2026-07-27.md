# PRELOOP-STAGEB-OWNED-LOCATED-AUTHORITY0-prime-r1

Status: accepted decision; R1-R3 closed; R4 execution order superseded
Date: 2026-07-27

## Decision

```text
Decision:
  PRELOOP-STAGEB-OWNED-LOCATED-AUTHORITY0-prime-r1

Status:
  accepted

Choice:
  A″ — one shared exact catalog allocation
       + one owned nested-result rebind witness
       + the existing borrowed located Port

First executable row:
  PRELOOP-STAGEB-SHARED-CATALOG0-S0
```

The accepted A1 whole-source producer remains unchanged. The new decision
repairs only the handoff from its owned selected row to the existing located
argument Port.

## Why the previous D1 cannot execute directly

The C4 conversion intentionally removes source borrows:

```text
PreparedPreloopLocatedArgumentV1
  -> OwnedPreloopStageBCarrierRowV1
```

The owned row retains canonical keys, structural sites, the body schedule,
and the outer Integer disposition. It does not retain the nested exact-Integer
contract needed to construct `PreparedPreloopLocatedArgumentV1`.

After C4, the catalog is also owned only by `Builder::comp_ctx`. Borrowing a
located input from that catalog and then passing the Port to `&mut MirBuilder`
creates a self-borrow conflict.

Therefore these routes are forbidden:

```text
rebuild callable-result catalogs after install
reclassify the source from sites
borrow the catalog from Builder during mutable lowering
convert selected located input to RawLegacy
add a second owned located Port/receipt family
add a ModuleLoweringInvocation collector to the selected root
```

## Accepted owner chain

```text
one Arc<VerifiedSameModuleCallableDeclarationCatalogV1>
  - created at the whole-source catalog seal
  - used by every source proof
        ↓
SealedNestedInstanceResultContractV1
        ↓ consuming projection
OwnedNestedInstanceResultRebindWitnessV1
  - exact caller
  - exact inner site
  - exact target key
  - private unconditional Integer seal
        ↓
OwnedPreloopStageBCarrierRowV1
        +
shared exact catalog
        ↓ activation install
  ├─ Builder catalog lane: Shared(Arc)
  └─ stack ledger/function ingress: same Arc
        ↓
PreparedPreloopStageBFunctionIngressV1
        ↓ existing source-view/path factories
exact outer MethodCall / CallArgument(index) / inner MethodCall
        ↓ owned witness rebind
existing SealedNestedInstanceResultContractV1
        ↓
existing PreparedPreloopLocatedArgumentV1
        ↓
existing PreloopLocatedArgumentPortV1
        ↓
existing Call receipt chain
```

`Arc` is an ownership mechanism, not a second catalog lane. `CompilationContext`
keeps one private catalog storage enum:

```text
Vacant
Exclusive(Catalog)     # existing ordinary/Raw paths
Shared(Arc<Catalog>)   # selected Stage-B path only
```

Every lookup continues through one `&Catalog` accessor. The source producer
creates the Stage-B `Arc` before any proof borrows, so source proofs, Builder,
and function ingress observe the same allocation.

## Owned rebind law

`OwnedNestedInstanceResultRebindWitnessV1` is produced only by consuming an
already-sealed nested contract. It is not a new result inference owner.

Its sole rebind terminal accepts an exact method-call site issued by the
shared catalog and checks:

```text
same shared catalog allocation
same caller canonical key
same SourceExprSiteV1
same target canonical key
same instance-method relation
```

It then returns the existing borrowed sealed contract without constructing a
callable-result catalog or reading Builder state.

Equal-looking catalogs, sites, or keys from another allocation reject.

## Function capture

The selected instance-method observation point remains the existing
instance-method loop in `module_lifecycle.rs`.

The physical function transaction uses:

```text
capture_legacy_function_pending_session_v1
existing instance-method preparation / StepTree guard
bounded prefix / selected / suffix body driver
existing PreloopLocatedArgumentPortV1 at the selected assignment only
existing current-module header lookup
existing draft finalization
existing legacy function publication
```

It does not call the current whole-body
`build_instance_method_draft_with_port_v1`: that sibling fixes
`ExpressionInput = ASTNode`, and its statement/body descent delegates directly
to the ordinary Port. It also does not use `RawInvocationChildPortV1`, because
that owner requires a module collector/invocation lifecycle that the selected
Legacy root does not own.

The root loop only issues an exact canonical-key observation to the stack
ledger. No Builder field, source-site map, name-selected policy, retry, or
fallback is added.

## Task series

### R1 — `PRELOOP-STAGEB-SHARED-CATALOG0-S0`

Status: closed

Implement:

```text
source producer seals Arc<Catalog> before proof borrows
activation plan retains the Arc
CompilationContext private Exclusive | Shared storage
Stage-B atomic context install accepts Shared(Arc)
ledger retains the same Arc
```

Acceptance:

```text
catalog semantic producer                         = existing 1
catalog storage lane                              = 1
Stage-B shared allocation producer                = 1
Arc::ptr_eq(Builder, ledger)                      = green
ordinary catalog install behavior                 = unchanged
catalog reseal                                    = 0
production caller                                 = 0
```

Landed evidence:

```text
source catalog allocation                          = Arc::new before proof borrows
activation / selected / rejection retention        = same Arc
Builder catalog storage                            = private Exclusive | Shared
Builder + installed stack ledger                   = Arc::ptr_eq green
ordinary and Raw installs                          = Exclusive, unchanged
focused carrier / inventory / selection /
context-install / module-activation tests          = green
cargo check --lib                                  = green
```

### R2 — `PRELOOP-STAGEB-NESTED-REBIND0-S0`

Status: closed

Implement:

```text
OwnedNestedInstanceResultRebindWitnessV1
consuming projection from sealed nested contract
owned witness retention in activation row
exact shared-catalog rebind terminal
```

Focused matrix:

```text
same catalog/caller/site/target -> existing borrowed contract
equal-looking foreign catalog   -> reject
caller/site/target drift        -> reject
result-catalog construction     -> 0
Builder reference               -> 0
```

Landed evidence:

```text
owned rebind witness producer                    = existing sealed contract only
retained identity                                = catalog/caller/site/target
unconditional Integer authority                  = private consumed seal
same shared catalog/caller/site/target            = rebind green
equal-looking foreign catalog                    = typed reject
caller/site/target drift                         = typed reject
callable-result catalog construction during rebind= 0
Builder reference                                = 0
activation row retention                         = 1
```

### R3 — `PRELOOP-STAGEB-FUNCTION-INGRESS-RECIPE0-S0`

Status: closed

Implement:

```text
PreparedPreloopStageBFunctionIngressV1
existing structural source-view descent
owned witness rebind
existing PreparedPreloopLocatedArgumentV1 construction
```

The prepared borrowed owner lives only inside the execution terminal while
the external catalog `Arc` remains alive. It never borrows from Builder.

Landed evidence:

```text
sole recipe producer                              = installed ledger parts
owned authority                                   = shared Arc + exact row
source descent                                    = existing factories only
owned nested result                               = exact rebind terminal
borrowed located owner                            = HRTB callback-local
Builder / RawLegacy / new Port / ledger delta     = 0
production caller / Call receipt / type write     = 0
```

### R4 — `PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-I0`

The original R4-and-later order is superseded by:

```text
PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION-RECONCILIATION0-prime-r1
```

Read:

```text
docs/development/current/main/investigations/
  preloop-stageb-instance-function-session-reconciliation0-prime-r1-task-map-2026-07-27.md
```

The R1-R3 products and owner decisions in this card remain authoritative. Do
not execute the historical R4 ordering below: read-only audits proved that
complete declaration metadata, located outer completion, outer physical
receipt, assignment correspondence, and outer Integer publication must close
before suffix descent can finish the selected function.

Add one Builder-owned selected instance-function session sibling. Reuse the
existing function preparation, StepTree guard, configured function context,
finalizer, and unpublished pending-session authorities, but execute the
already-sealed body schedule explicitly:

```text
prefix statements   -> ordinary Raw descent
selected assignment -> existing PreloopLocatedArgumentPortV1
suffix statements   -> ordinary Raw descent
```

The current whole-body `build_instance_method_draft_with_port_v1` is not the
consumer: its trait fixes `ExpressionInput = ASTNode`, and its body/statement
descent delegates to the ordinary Port. R4 therefore adds one thin bounded
body driver plus a generic selected-session payload/error sibling over the
same function session. It does not add a second general function compiler.

```text
new function compiler                            = 0
new module collector                             = 0
second whole-body dispatcher                     = 0
prefix/suffix ordinary descent                   = 1
selected statement located Port consumer         = 1
RawLegacy conversion                             = 0
selected failure -> ordinary retry               = 0
```

### R5 — `PRELOOP-STAGEB-FUNCTION-ACTIVATION-LEDGER0-P0/G0`

Only now extend the C4 ledger with states that have real producers:

```text
Armed(ingress)
  -> InFlight(ingress)
  -> Completed(exact function receipt)
  |  Rejected(retained ingress + cause)
```

Matrix:

```text
exact key observed once       -> Completed
selected key never observed   -> SelectedCallerNotObserved
selected key observed twice   -> SelectedCallerConsumedTwice
selected lowering failure     -> retained typed rejection
failure -> fresh compile      -> green
```

No payloadless `Consumed`, `Poisoned`, `reset`, `rearm`, or row escape
terminal is allowed.

### R6 — `PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-I0`

This is the first production behavior change.

```text
compile_legacy_request
  -> LegacyWholeSourceCompileRequestV1
  -> compile_request Legacy arm
  -> select once
     ├─ Ordinary
     │    -> install typed aliases after selection
     │    -> existing Legacy build
     └─ Selected
          -> shared-catalog activation
          -> exact selected function transaction
          -> existing Legacy finish
```

`compile_with_source_and_imports` moves an `Explicit` typed snapshot into the
request. `compile_legacy` moves `None`. Neither mutates Builder aliases before
selection.

### R7 — `PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-P0/G0`

Focused matrix:

```text
Ordinary None clears stale aliases only after selection
Ordinary Explicit preserves existing alias behavior
ProgramV0 / REPL remain explicit Ordinary
ambiguous/selection reject leaves Builder unchanged
Selected direct/alias candidate consumes exactly once
Selected failure never retries Ordinary
failed Selected -> fresh Ordinary success
direct MirBuilder / JSON / Raw caller delta = 0
```

## Downstream series

After R7, continue without reopening the already accepted outer-carrier
design:

```text
UNIFIED-CALL-OUTER-CARRIER-RECEIPT0-S0
-> PRELOOP-OUTER-CARRIER-RECEIPT0-I0
-> PRELOOP-OUTER-CARRIER-ASSIGNMENT0-S0
-> PRELOOP-OUTER-CARRIER-RECEIPT0-P0/G0

-> PRELOOP-OUTER-CARRIER-TYPE-I0-S0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-I0/P0/G0

-> CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
-> PRELOOP-INNER-TYPE-PROOF-RETIRE0-S0
```

Only after the real Stage-B guard is green may the next actual frontier be
selected. Alias/View language work, loop-refresh activation, and ownership
grammar remain parked and are not precommitted by this series.

## Structural gate

```text
whole-source catalog semantic producer             = 1
whole-source Stage-B shared allocation producer    = 1
Builder callable catalog truth lane                = 1
Builder + ledger exact shared allocation           = 1

owned nested rebind witness producer               = 1
callable-result catalog rebuild during ingress     = 0
source reclassification during ingress             = 0

existing borrowed located Port consumer            = 1
second owned located Port                          = 0
RawLocated -> RawLegacy conversion                 = 0

selected function session owner                    = 1
new module collector/invocation lifecycle          = 0
Builder source-site registry                       = 0
persistent SourceExprSite -> ValueId map            = 0

exact production selector caller                   = 1 after R6
direct build_module / JSON / Raw selector caller   = 0
fallback / retry / route reselection               = 0

outer Call receipt before downstream series        = 0
outer type publication before downstream series    = 0
all modified/new source/check files                < 800 lines
```

## Non-claims

```text
general shared catalog API
general owned located lowering
general instance-method result inference
whole port-aware Raw cutover
Raw publication redesign

outer Call receipt in R1-R7
outer carrier type publication in R1-R7
GenericLoop publisher
loop-refresh activation

Alias / View language semantics
ownership grammar
parser / VM / LLVM / backend changes
default backend cutover
fallback / retry
```
