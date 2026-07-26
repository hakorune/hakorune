# Raw instance MethodCall source-site carry D0

```text
Decision: RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-D0
Status: design stop
Opened: 2026-07-27
Parent: NESTED-INSTANCE-RESULT-EMISSION-RECONCILIATION-D1-prime-r1
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

Create a source-owned, stack-scoped structural cursor family:

```text
VerifiedRawCallableBodyCursorV1
  - exact canonical caller
  - exact owned/borrowed callable body authority
  - root SourceNodeSiteV1

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

## Decision questions

1. Which existing source owner supplies the exact callable body, canonical
   caller, and root source site to the first production candidate cursor?
2. Should the sole `raw_expression_dispatch` matcher become generic over a
   raw input view, with the existing AST-only route retained as a thin facade?
3. What is the minimum expression/statement surface that must carry the cursor
   to reach `Body(3).Value.Argument(1)` without delegating through an unlocated
   subtree?
4. Where is the isolated candidate draft boundary so cursor/lowering failure
   leaves the live Builder and publication unchanged?
5. What exact explicit production request selects the candidate cursor route
   while the default legacy raw route remains unchanged?

## Recommended task order after acceptance

```text
RAW-SOURCE-CURSOR0-S0
  source-only cursor/input/rejection vocabulary

RAW-EXPRESSION-DISPATCH-CURSOR0-S0
  split the current near-limit matcher into a module folder

RAW-EXPRESSION-DISPATCH-CURSOR0-S1
  generic body/statement/expression input view

RAW-EXPRESSION-DISPATCH-CURSOR0-S2
  structural child-site propagation for the admitted pre-loop prefix

RAW-EXPRESSION-DISPATCH-CURSOR0-S3
  legacy AST facade parity and single matcher guard

RAW-LOCATED-INSTANCE-METHOD-INPUT0-S0
  exact site-aware MethodCall input and private association factory

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
  3-5 buildable commits

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
