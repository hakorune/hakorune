# Raw instance MethodCall source-site carry D0

```text
Decision: RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-prime-r1
Closes: RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-D0
Status: accepted design
Opened: 2026-07-27
Parent: NESTED-INSTANCE-RESULT-EMISSION-RECONCILIATION-D1-prime-r1
First executable row: RAW-SOURCE-CURSOR0-S0
```

## Current gap

The selected pre-loop route is:

```text
lower_method_as_function
-> build_instance_method_draft_v1
-> lower_method_body
-> raw statement/expression descent
-> RawLegacyMethodCallInputV1
-> MeStandardUnified
-> UnifiedCallEmitterBox
```

The physical Call route is real, but its MethodCall input owns only:

```text
receiver AST
method spelling
argument AST rows
```

It owns no `SourceExprSiteV1`.  The exact source contract owns the desired
site, but there is no route-owned evidence that the raw MethodCall currently
being lowered is that site.

## Fixed authority boundary

```text
source site authority =
  source-owned callable body + canonical caller + structural source path

physical destination authority =
  later completed unified Call receipt

result authority =
  later sealed nested-instance result contract
```

These authorities remain separate.

The following are forbidden:

```text
AST re-walk during lowering
call ordinal matching
callee/owner/method-name matching
source span as semantic identity
Builder-wide source-site map
persistent source-site -> ValueId map
optional site field added to RawLegacyMethodCallInputV1
LocatedLegacyLoweringSessionV1 production activation as a shortcut
```

## Recommended shape

Create a catalog-backed, stack-scoped raw wrapper over the existing source
navigation kernel:

```text
SourceCursorCoreV1
  - existing SourcePathV1
  - existing SourceNode/Body/Stmt/Expr site vocabulary
  - existing ExprChildRoleV1 / BodyChildRoleV1
  - child node/site correspondence only

  ├─ existing FunctionSourceViewV1
  │    owner = FunctionOwnerIdV1
  │
  └─ VerifiedRawCallableSourceViewV1
       owner =
         exact VerifiedSameModuleCallableDeclarationCatalogV1 allocation
         + exact CanonicalSameModuleCallableKeyV1
         + exact declaration row

RawLocatedBodyInputV1
RawLocatedStmtInputV1
RawLocatedExprInputV1
  - exact node
  - exact structural site
  - same cursor identity

RawLocatedMethodCallInputV1
  - exact MethodCall syntax
  - exact SourceExprSiteV1
  - exact caller
```

Child inputs are derived while the same descent selects the child:

```text
parent input + ExprChildRoleV1 / statement index
-> child node + child structural site
```

No second traversal reconstructs the path.

`SourceCursorCoreV1` is not a new source identity issuer. It extracts the
already-existing parent-relative path/site calculation from
`FunctionSourceViewV1`; the canonical view and the new raw view delegate to
that one calculation. Canonical and raw owner identities remain distinct.

If extracting the full core would widen the first commit excessively, the raw
wrapper may delegate directly to the existing `SourcePathV1`,
`ExprChildRoleV1`, `BodyChildRoleV1`, and
`project_source_body_node_v1` operations. It may not implement another role,
path, or AST-location algorithm.

## Accepted answers

### Q1 source owner

```text
exact allocation =
  the same VerifiedSameModuleCallableDeclarationCatalogV1
  already retained by VerifiedSourceMethodCallSiteV1
  and SealedNestedInstanceResultContractV1

exact row =
  catalog.declaration(caller)
```

The raw view holds the catalog, canonical caller key from that catalog, and
exact declaration row by reference:

```rust
struct VerifiedRawCallableSourceViewV1<'catalog> {
    catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    caller: &'catalog CanonicalSameModuleCallableKeyV1,
    declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
}
```

Only this view issues the raw body root. An equal-looking catalog, caller, or
source path is foreign. Final association requires:

```text
same declaration-catalog allocation
same canonical caller key
same SourceExprSiteV1
```

### Q2 navigation

The sole raw matcher becomes input-view generic. The AST-only legacy route is
a thin facade over that matcher. Source navigation is delegated to the
existing path/site/child-role machinery; no second navigation engine is
created.

### Q3 bounded surface

The first route carries exact located inputs only through the admitted
pre-loop prefix needed to reach:

```text
Body(3).Value.Argument(1)
```

Every parent/child transition in that prefix uses the same structural child
selection to derive both node and site. A subtree containing a selected site
may not delegate through an unlocated input. Other raw surfaces continue
through the unchanged legacy facade.

### Q4 failure boundary

The explicit candidate ingress lowers into an isolated unpublished draft.
Cursor/navigation/lowering failure retains or discards that candidate and
leaves the live Builder and module publication unchanged. Commit is allowed
only after source-cursor coverage, raw-lowering completion, and draft
verification are green.

### Q5 route selection

One crate-private explicit candidate request owns the verified raw source view
and selects the cursor route once. It does not modify
`lower_method_as_function`, the default legacy raw route, or
`LocatedLegacyLoweringSessionV1`. Failure never retries another route.

## Recommended task order after acceptance

```text
RAW-SOURCE-CURSOR0-S0
  source-only cursor/input/rejection vocabulary

RAW-EXPRESSION-DISPATCH-CURSOR0-I0
  one 3-5 commit BoxShape refactor series:
    split the current near-limit matcher into a module folder
    generic body/statement/expression input view
    legacy AST facade parity and single matcher guard

  closed:
    the source-only Raw cursor remains the sole located child-site carrier.
    RawLocated inputs must not implement the legacy transport views because
    `into_legacy_*` intentionally erases their sites. The candidate-only
    physical descent belongs after an exact located MethodCall input exists.

RAW-LOCATED-INSTANCE-METHOD-INPUT0-S0
  exact site-aware MethodCall input and private source-only factory

RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-I0
  one explicit source-owned candidate route

RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-P0
  exact site/AST/physical-route/failure/reuse parity

RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-G0
```

## Proof budget

```text
ceremony:
  T1/T2 BoxShape series

accepted grammar/result delta:
  0

production default-route delta:
  0

new source identity issuer:
  0
  existing SourceExprSiteV1 structural vocabulary only

new source-location transport owner:
  1

commit budget after D0:
  3-5 buildable commits for the complete carry umbrella

new per-row shell guard:
  0
  extend the existing callable-result/raw-route guard family
```

## Acceptance

```text
exact pre-loop MethodCall site arrives from route-owned structural descent
same node/site/caller cursor identity is retained
foreign/equal-looking cursor rejects
unselected MethodCall cannot borrow the selected association

AST rewrite/re-walk                    = 0
ordinal/name-based reconstruction      = 0
Builder persistent source state        = 0
LocatedLegacy production activation    = 0
physical Call behavior delta           = 0
MirType/type_ctx write delta           = 0
fallback/retry                         = 0
all modified/new source/check files    < 800 lines
```

## Non-claims

```text
unified Call physical receipt implementation
nested-instance receipt implementation
Integer result publication
loop-refresh production activation
loop selected-call publisher migration
general all-AST located lowering completion
default raw route cutover
parser, grammar, runtime, backend change
```

## Required closeout

```text
Decision:
  RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-prime-r1

Status:
  accepted

source owner:
  same VerifiedSameModuleCallableDeclarationCatalogV1 allocation
  + exact caller
  + exact declaration

source navigation:
  existing SourcePath/site/child-role machinery
  one extracted/delegated SourceCursorCoreV1

raw owner:
  thin catalog-backed VerifiedRawCallableSourceViewV1

second source navigation engine:
  forbidden

raw matcher:
  one input-view-generic matcher
  existing legacy AST facade preserved

candidate route:
  one explicit crate-private request
  isolated unpublished draft
  default raw route unchanged

first executable row:
  RAW-SOURCE-CURSOR0-S0
```
