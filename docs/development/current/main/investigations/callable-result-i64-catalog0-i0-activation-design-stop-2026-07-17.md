---
Status: design consultation required before production activation
Date: 2026-07-17
Decision: pending; Candidate A owned single-use activation plan is recommended
Baseline: fd20feedbd
Parent: callable-result-i64-catalog0-task-2026-07-17.md
Scope: exact-site same-module exact-i64 call-result activation before Builder effects
---

# Callable-result I0 activation design stop

## Current finding

Three independent worker audits and local source review agree that the closed
result catalog cannot be connected to the legacy Builder honestly by a small
emitter patch. The declaration, source-target, result, and normalized P0
products are green. The missing boundary is the production activation owner.

The exact gaps are:

1. `MirBuilder::lower_root` is the only complete pre-body hook, but it installs
   the declaration catalog into live `CompilationContext` before later
   indexing and body-lowering effects.
2. `VerifiedSourceStaticCallTargetCatalogV1` borrows that declaration catalog,
   and `VerifiedSameModuleCallableResultCatalogV1` borrows both catalogs.
   Storing all three beside the owner creates self-reference or a forbidden
   cloned authority.
3. Legacy `build_expression -> build_method_call` carries neither a canonical
   caller key nor `SourceExprSiteV1`. Exact rows cannot be claimed at emission
   without a site-aware lowering boundary.
4. The selected caller `ParserBox.static_const_parse_add/2` is an
   `InstanceBoxMethod`; the result solver proves static target declarations
   and currently rejects non-static callers during target pairing.
5. Generic call-result annotation uses function-name/module lookup after
   emission. It is not the sealed exact-site authority.

This is a new authority/lifetime contradiction discovered after P0. One
design-stop consultation is therefore permitted by the docs loop-breaker.

## Source authority

```text
owned function/program root
  -> complete declaration catalog
  -> immutable explicit-import view
  -> exact caller + SourceExprSite MethodCall inventory
  -> canonical static target rows
  -> exact-i64 result rows
  -> owned single-use plan or one scoped lowering session
  -> exact-site consumption during ordinary lowering
```

Static result inference may remain target-contract-only. Instance caller
identity is needed for source-site ownership, not as a new instance-method
result-inference family.

## Non-authorities

```text
mutable using_import_boxes
method/function/MIR symbol parsing
AST equality, source span, or encounter-order matching
current_module function signatures
final MirFunction metadata
callee-first lowering or re-lowering
legacy name-based result annotation
runtime tags or GenericLoop inference
retry or fallback
```

## Candidate A — owned single-use activation plan (recommended)

Preflight borrows declaration/target/result products only while deriving one
non-Clone owned plan. The borrowed proofs are dropped before the plan and
declaration catalog are installed into an isolated candidate Builder session.

Suggested shape:

```rust
pub(crate) struct VerifiedCallableResultActivationPlanV1 {
    declaration_catalog:
        VerifiedSameModuleCallableDeclarationCatalogV1,
    rows_by_caller:
        BTreeMap<CanonicalSameModuleCallableKeyV1,
                 Box<[VerifiedCallableResultActivationSiteV1]>>,
}

pub(crate) struct VerifiedCallableResultActivationSiteV1 {
    site: SourceExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
    required_i64_arguments: Box<[u32]>,
    result: CallableResultActivationDispositionV1,
}
```

The final fields are consultation-owned. The plan owns no AST body, import
map, target/result catalog clone, ABI/effect table, or second callable index.
It is a normalized consumption product, not another semantic solver.

Advantages:

```text
no self-reference in CompilationContext
borrowed proof lifetime ends before Builder mutation
candidate Builder owns one non-Clone plan
failure-before-live-effects boundary is explicit
```

Remaining design point: recursive AST lowering must receive the exact current
`SourceExprSiteV1` and caller key. A caller-scoped ledger may consume each row
once, but the site cursor must use the same structural-path law as the existing
projector. It may not be reconstructed from names, spans, AST equality, or
call order.

## Candidate B — stack-scoped borrowed lowering session

Keep all products on the stack and borrow them through a
`VerifiedCallableResultLoweringSessionV1<'root>` while ordinary body lowering
runs. Thread the caller key and exact source path through recursive expression
and statement lowering.

This retains the original proofs directly, but introduces wide lifetime/API
threading and still needs candidate/live Builder isolation. It is parked unless
consultation rejects Candidate A's owned projection.

## Rejected shapes

```text
store borrowed catalogs beside their owner in CompilationContext
Clone/Arc declaration, target, or result authority
rebuild target/result facts during emission
select by target name, symbol, AST equality, span, or ordinal
use mutable import state as the seal
publish through legacy name/module result annotation
lower callee first or retry a failed route
```

## Consultation questions

1. Is Candidate A the correct owner: consume the borrowed proof chain into one
   owned, non-Clone, single-use activation plan before opening the candidate
   Builder session?
2. If A is selected, should legacy lowering receive an explicit
   `SourceExprSiteV1` cursor through its recursive APIs, or should a presealed
   located-node/body view become its only input? The result must preserve one
   structural-path authority and exact repeated-site identity.
3. May the activation inventory admit an `InstanceBoxMethod` caller while
   retaining static-only target result contracts, or should caller ownership
   use a separate source-owner key product?
4. Is the atomic boundary correctly placed before live Builder mutation:
   preflight every product, open a candidate Builder, lower once, and commit
   the completed candidate only?

Recommendation: select Candidate A, keep static-only result inference, admit
instance identity only as exact-site caller ownership, and thread one explicit
site-aware lowering context.

## Task order after the decision

```text
R0-CALLABLE-RESULT-I64-CATALOG0-I0-ACTIVATION-D0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-A0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-SITE0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-CUT0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-G0
```

### A0 — disconnected owned activation product

```text
production behavior delta: 0
production consumers: 0
```

Consume the borrowed proof chain into one owned non-Clone plan. Prove
brand correspondence, instance-caller/static-target separation, site
multiplicity, required-argument bounds, and reorder parity. Retain no AST and
no duplicate target/result authority.

### SITE0 — exact legacy lowering site ledger

```text
production behavior delta: 0
production result consumers: 0
```

Add one caller-scoped site cursor and monotonic exact-once ledger. Repeated and
nested calls remain distinct. Wrong, duplicate, missing, foreign, or drifting
rows reject typed. Name/span/AST/order reconstruction remains zero.

### CUT0 — atomic production activation

```text
production behavior delta:
  selected same-module exact-i64 call results become available during lowering
production result publisher: exactly 1
```

Required order:

```text
1. preflight root plus explicit imports before live Builder mutation
2. seal declaration, site/target, result, and activation products once
3. open an isolated candidate Builder session
4. install declaration catalog plus activation plan once
5. lower each body once with its caller/site ledger
6. claim the exact row before selected call-route mutation
7. lower arguments in source order
8. require exact Integer at every required ordinal
9. emit sealed Callee::Global(target.mir_symbol_projection())
   through a no-resolver/no-fallback entry
10. only after successful Call emission publish dst as MirType::Integer
11. finish and commit the candidate once
```

The selected path must not invoke legacy name/module result annotation.

### G0 — guards and closeout

Use a small I0 checker or extend the existing small same-module inventory. Do
not extend the roughly 765-line S0 checker.

```text
declaration catalog preflight producer/install = 1
immutable import view = 1
exact source-site inventory = 1
source target producer = 1
result solver = 1
activation plan producer = 1
site-ledger consumer = 1
same-module result publisher = 1

bare target lookup on selected path = 0
legacy result annotation on selected path = 0
final metadata reads = 0
physical/name/span/AST inference = 0
retry/re-lowering/fallback = 0
second catalog/type/owner map = 0
source/check files >= 800 lines = 0
```

## Failure-before-effects law

```text
preflight/brand/route/result mismatch:
  candidate/live Builder instruction and type effects = 0

site/target/required-argument mismatch:
  Call emission = 0
  result type publication = 0

Call emission failure:
  result type publication = 0

later candidate lowering failure:
  live compiler/module publication = 0

rejected compile followed by valid compile:
  pass on the same compiler instance
```

Argument-expression effects may precede a required-representation mismatch.
CUT0 must not claim whole-call rollback; whole-compilation discard belongs to
the isolated candidate session.

## Required fixtures

```text
forward/reverse generic proof apps both execute 6
repeated and nested exact sites consume once each
qualified imported alias and current-owner calls
instance caller -> static target call

ParserBox.static_const_parse_add/2
  -> ParserStringUtilsBox.skip_ws/2
  -> StringHelpers.skip_ws/2

required argument is exact Integer
result is exact before GenericLoop verification
actual parser fixture has no MissingTransientType

unknown/non-i64 required argument -> no Call/result publication
missing/duplicate/wrong/foreign site -> typed reject
failed Call -> result type delta 0
rejected compile -> later valid compile succeeds
```

## Implementation may claim after CUT0

```text
one pre-body exact activation producer
one exact-site, exact-once selected call consumer
instance caller identity with a static target result contract
successful sealed Call followed by exact Integer publication
declaration-order and repeated-site stability
candidate-session discard on compilation failure
```

## Implementation must not claim

```text
general callable or instance-method result inference
non-i64 result inference
general legacy Builder source-site conversion
callee-first or two-pass lowering
whole-call argument-effect rollback
GenericLoop inference
fallback or retry
```

## Stop conditions

1. A second AST walker, callable catalog, target catalog, or result solver.
2. Site reconstruction from names, symbols, spans, AST equality, or order.
3. Mutable imports as seal authority.
4. Self-referential/Clone/Arc catalog storage.
5. Widening result inference to instance bodies merely to admit a caller.
6. Current-module/final-metadata/runtime-tag result lookup.
7. Callee-first publication, retry, re-lowering, or fallback.
8. A second persistent `ValueId -> type/owner` map.
9. Unrelated route behavior changes in the candidate transaction.
10. A source/check file reaching 800 lines.

## Decision lock

P0 remains closed and production behavior remains unchanged. I0 code is not
authorized until consultation selects the owned-plan versus scoped-session
shape and exact source-site carrier. Candidate A is recommended. The next
code-facing owner after selection is `I0-A0`; no emitter patch may bypass A0
and SITE0.
