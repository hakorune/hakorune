---
Status: locally selected and taskized; PATH0 is the next code-facing row
Date: 2026-07-18
Decision: Candidate A owned single-use activation plan plus located legacy input
Baseline: 611049a62f
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

This was a new authority/lifetime contradiction discovered after P0. A second
three-worker audit has now selected one mechanically unique implementation
shape under `LocalMechanicalSelectorAuthorityV1`; external consultation is no
longer required.

## Local selection evidence

All three worker audits converge on these facts:

```text
Candidate A:
  selected

Candidate B stack-borrowed Builder session:
  rejected

exact site producer:
  existing resolved_semantics::shadow traversal

exact lowering carrier:
  located body/statement/expression input

mutable path cursor or encounter-order ledger:
  rejected
```

`MirBuilder` and `CompilationContext` are lifetime-free owned state, and the
existing `CanonicalModuleLoweringSessionV1` already isolates an owned
candidate. Candidate B would require lifetime-parameterizing the Builder or
threading borrowed catalogs across nearly every legacy lowering API. Candidate
A instead normalizes the borrowed proofs in one lexical preflight, drops them,
then moves the sole declaration catalog and owned rows into the candidate.

The existing shadow resolver is the sole complete owner-local structural
traversal that already produces `SourcePathSegmentV1` and lexical qualified-
receiver facts. It can observe all MethodCall sites without a second walker.
Legacy lowering currently has 81 expression-entry calls across 33 files and
also creates synthetic nodes, so a mutable push/pop cursor would be fragile.
Located inputs co-seal caller, exact path, and the actual moved/borrowed node;
synthetic nodes remain explicitly unlocated and cannot claim activation rows.

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

## Candidate A — owned single-use activation plan (selected)

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

The exact private fields are implementation-owned under the laws below. The
plan owns no duplicate AST body, import map, target/result catalog clone,
ABI/effect table, or second callable index.
It is a normalized consumption product, not another semantic solver.

Advantages:

```text
no self-reference in CompilationContext
borrowed proof lifetime ends before Builder mutation
candidate Builder owns one non-Clone plan
failure-before-live-effects boundary is explicit
```

Recursive AST lowering receives located body/statement/expression inputs that
carry the caller key, exact `SourceExprSiteV1`, and syntax node. Child inputs
are constructed only through one shared child-role-to-path-segment law. A
caller-scoped ledger consumes rows by exact site, never by execution ordinal.

## Candidate B — stack-scoped borrowed lowering session

Keep all products on the stack and borrow them through a
`VerifiedCallableResultLoweringSessionV1<'root>` while ordinary body lowering
runs. Thread the caller key and exact source path through recursive expression
and statement lowering.

This retains the original proofs directly, but introduces wide lifetime/API
threading and still needs candidate/live Builder isolation. Existing ownership
and transaction structure mechanically reject it for this row.

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

## Resolved local decisions

1. Consume the borrowed proof chain into one owned, non-Clone, single-use
   activation plan before opening the candidate Builder.
2. Reuse the existing shadow traversal for complete MethodCall inventory and
   lexical facts. Share one neutral child-role-to-`SourcePathSegmentV1` policy
   with compiler source views and located legacy lowering.
3. Use located legacy inputs, not a mutable cursor. Raw legacy subtree
   delegation is allowed only when the ledger proves that no activation row
   exists under the subtree prefix.
4. Admit `InstanceBoxMethod` only as exact caller/site ownership. Targets and
   result inference remain `StaticBoxMethod`; no ParserBox result is inferred.
5. Preflight the root plus explicit imports before live mutation, lower once in
   `CanonicalModuleLoweringSessionV1`, and commit the completed candidate once.

## Task order after the decision

```text
R0-CALLABLE-RESULT-I64-CATALOG0-I0-ACTIVATION-D0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-PATH0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-A0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-SITE0-L0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-SITE0-R0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-SITE0-C0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-CUT0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0-G0
```

### PATH0 — one structural path policy and MethodCall inventory

```text
production behavior delta: 0
production result consumers: 0
```

Extract one neutral child-role-to-`SourcePathSegmentV1` policy from the
existing compiler source-view/shadow rules. Extend the sole shadow traversal
with a read-only all-MethodCall observation mode that emits exact caller/site
rows plus existing Bound/ProvenUnbound lexical dispositions. It must cover the
actual ParserBox syntax, nested/repeated calls, current-owner calls, qualified
calls, and dynamic/bound exclusions. No second AST walker is added.

### A0 — disconnected owned activation product

```text
production behavior delta: 0
production consumers: 0
```

Consume the borrowed proof chain into one owned non-Clone plan. Prove
brand correspondence, instance-caller/static-target separation, site
multiplicity, required-argument bounds, and reorder parity. Retain no AST and
no duplicate target/result authority. Every observed MethodCall receives an
explicit `SelectedExactI64` or `Unselected` disposition so missing rows never
mean legacy fallback. Borrowed target/result products are dropped before the
declaration catalog moves into the plan.

### SITE0-L0 — located legacy carrier vocabulary

```text
production behavior delta: 0
production result consumers: 0
```

Add privately constructed located body/statement/expression inputs carrying
caller key, exact structural site, and syntax node. Child construction consumes
only the neutral PATH0 role policy. Synthetic nodes are `Unlocated` and cannot
claim rows.

### SITE0-R0 — behavior-neutral located recursive descent

Use Refactor Series Mode. Begin one caller ledger from the activation plan,
lower the caller body through located inputs, and finish with exact coverage.
Raw legacy subtree delegation is allowed only for a prefix with zero active
rows. Wrong, duplicate, missing, foreign, and drifting rows reject typed. This
row changes no accepted program or result publication.

### SITE0-C0 — preserve location through call planning

Carry the exact site token through any `CoreEffectPlan`/immediate MethodCall
normalization that consumes a located call. Provenance must never be recovered
after plan creation. Production result publication remains zero.

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
3. open the existing isolated candidate Builder session
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
structural AST traversal owner = 1
child-role path policy owner = 1
exact source-site inventory = 1
source target producer = 1
result solver = 1
activation plan producer = 1
located caller ledger consumer = 1
same-module result publisher = 1
candidate commit = 1
instance result inference = 0

bare target lookup on selected path = 0
legacy result annotation on selected path = 0
final metadata reads = 0
physical/name/span/AST/encounter-order inference = 0
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

P0 remains closed and production behavior remains unchanged. Candidate A plus
located legacy input is locally selected under
`LocalMechanicalSelectorAuthorityV1`; external consultation is no longer
required. `I0-PATH0` is the next code-facing row. No emitter patch may bypass
PATH0, A0, or the SITE0 refactor series.
