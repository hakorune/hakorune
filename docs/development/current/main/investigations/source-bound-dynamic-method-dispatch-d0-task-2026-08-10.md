---
Status: accepted target/source boundary; prerequisite source relation I0 next
Date: 2026-08-10
Row: `SOURCE-BOUND-DYNAMIC-METHOD-DISPATCH-D0`
Parent: `generic-loop-dynamic-full-body-closure-d0-task-2026-08-10.md`
Mode: BoxShape / compiler acceptance repair
---

# Source-bound Dynamic method dispatch contract

## Decision boundary

The prerequisite `GENERIC-LOOP-DYNAMIC-FULL-BODY-COVERAGE-I0` is closed. Its
source inventory retains both exact MethodCall syntax rows without assigning
receiver/result semantics. The external and code audits close this D0 with one
correction: target/source identity and execution semantics are separate
products. No Recipe or physical implementation proceeds until both are
sealed.

The unchanged production `ParserScanLoopBox.skip_while/4` contains:

```hako
src.substring(i, i + 1)
pred_chars.indexOf(ch)
```

Both receivers are source-backed `Dynamic`. The current resolver target
catalog admits exact declared instance/static targets only; it cannot honestly
mint an exact declaration target for either call. Treating the method spelling
as a Box/type classifier, fabricating an instance target, or falling through to
the legacy Builder writer is forbidden.

The missing compiler authority is a route-disjoint arm in the one source-call
target catalog:

```text
exact declared receiver
  -> exact declaration target capability

source-backed Dynamic receiver
  -> opaque DynamicMemberTarget
  -> exact source-bound Dynamic dispatch contract
```

This row designs the second arm. It does not weaken the first arm, create a
second target catalog, or put runtime dispatch identity into the neutral
Recipe wire.

## Accepted target/source products

The first implementation issues one non-`Clone`, AST-free relation per exact
source call site:

```text
VerifiedDynamicMemberTargetV1
  DynamicMemberDispatchKeyV1
    selector spelling
    checked arity

VerifiedSourceBoundDynamicMemberCallV1
  exact function owner / source call site
  source-backed Dynamic receiver relation
  ordered source argument relations
  exact result expression/source destination site
  DynamicMemberTarget
  same source/resolver provenance
```

The method spelling is only the as-written runtime dispatch key. It is not
authority for receiver Box/type, result refinement, Home, provider identity,
physical ABI, backend route, or special compiler behavior.

The first product is a late-bound message target, not an exact provider or
declaration target. It says only that the runtime receiver selects one member
identified by selector and arity. It owns no Dynamic result class, Home,
effect, suspension, Fault, ABI, provider, or executable route.

The Recipe `CallSlot` later keeps only receiver/argument/result logical keys.
An item-keyed sibling relation co-seals the source target, Dynamic value rows,
and the separate execution envelope exactly once; owner Loop/block placement
remains derivable from Recipe membership.

## Source authority inventory

Existing reusable evidence:

```text
call syntax and exact source sites:
  ResolvedFunctionBodyShapeProductV1

receiver/argument BindingRefs and source membership:
  CallableSourceLedgerV1

source-backed Dynamic origin:
  SourceBackedDynamicCallableIssuerV1
  CallableDynamicOriginLoweringStateV1 (migration physical witness only)
```

Missing canonical issuer:

```text
source call site
+ Dynamic receiver origin
+ ordered argument source relations
+ method symbol/arity dispatch key
-> one verified Dynamic dispatch relation
```

The audit also found that no reusable AST-free arbitrary MethodCall row owns
receiver, ordered arguments, result site, selector, and arity together.
`VerifiedSourceMethodCallSiteV1` borrows AST and a Builder declaration catalog;
the bounded full-body inventory is profile-specific. Therefore
`RESOLVED-METHOD-CALL-SOURCE-RELATION-I0` must land before the Dynamic target
I0. It adds the neutral row to the resolved function/ledger rather than
promoting profile roles into universal authority.

## One target catalog

Do not add a second Dynamic catalog. In the target I0 series, generalize the
existing static-only names to the route-neutral vocabulary:

```text
VerifiedSourceCallTargetCatalogV1
  key = exact caller + exact call site

VerifiedSourceCallTargetV1
  Static(existing qualified/current-owner target)
  DynamicMember(VerifiedSourceBoundDynamicMemberCallV1)
```

Existing static sealing semantics remain unchanged. Static and Dynamic arms
for the same caller/site are a typed duplicate rejection. Temporary static
projection adapters must state their removal condition; equal names or keys
from a foreign catalog/function owner are not authority.

The issuer belongs in a neutral pre-Builder source/semantic layer. Builder
modules may consume the product later but must not become its semantic owner.

## Outcome rules

```text
Candidate:
  exact supported source MethodCall, Dynamic receiver origin, complete ordered
  argument relations, exact source membership, and one result relation

Declined:
  completely observed call is not the Dynamic-dispatch family

Unresolved:
  source/resolver evidence required to decide the source call is unavailable

Rejected:
  foreign owner/provenance, duplicate call site, missing/duplicate argument,
  receiver-origin mismatch, or contradictory result relation

NoSafeSlice:
  the canonical issuer itself is not implemented; this is a development state,
  not a source disposition
```

## Closed D0 decisions

1. No current durable resolver product owns complete ordered MethodCall source
   membership. The neutral source-relation I0 is a prerequisite.
2. No current neutral call-result Dynamic class owner exists. V2 Dynamic value
   and call relation D0/I0 owns it later; `MirType::Unknown` is never reverse
   authority.
3. No honest complete effect/control/Home/suspension issuer exists. The
   selector-independent `DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-D0/I0` is a
   separate prerequisite to Recipe readiness. Target I0 does not fake it.
4. A later item-keyed sibling CallSlot relation holds target/value/envelope
   refs; Recipe remains the owner of item order and placement.
5. Runtime uses actual receiver class plus selector/arity against one immutable
   registry snapshot, selects one executable plan, and invokes once. Missing or
   failed selection errors without fallback.

## Implementation row

After the neutral source relation lands,
`SOURCE-BOUND-DYNAMIC-METHOD-DISPATCH-I0` must:

```text
issue exact relations for substring/2 and indexOf/1 in unchanged skip_while/4
retain source-backed Dynamic receiver and exact argument/result source lineage
reject missing / duplicate / foreign / reordered source rows
perform zero Builder/MIR effect
expose no name-based semantic classifier
update owner README and public MIR reference in the same commit
keep every new source file below 800 lines
```

Candidate requires an exact Bound lexical receiver whose `BindingRef` belongs
to the source-backed Dynamic origin catalog. A variable receiver must not be
misrepresented as a shadow `Dynamic` syntax variant. Qualified/current-owner
static targets and fully observed non-Dynamic receivers are Declined for this
arm; foreign identity, route collision, argument gaps/duplicates/order repair,
or receiver-origin mismatch are Rejected.

## Corrected task order

```text
1. RESOLVED-METHOD-CALL-SOURCE-RELATION-I0
2. SOURCE-BOUND-DYNAMIC-METHOD-DISPATCH-I0
3. DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-D0
4. DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-I0
5. LOOP-V2-DYNAMIC-VALUE-AND-CALL-RELATION-D0/I0
```

The execution-envelope row is the hard stop before Recipe. In particular,
Recipe-local SSA for `ch` does not prove the Home/lifetime of the Dynamic call
result.

## Nonclaims

```text
no exact declaration target for a Dynamic receiver
no Text/String refinement
no method-name special case
no Dynamic result class in target I0
no Pure/Readonly/NonSuspending/NonControl/Home default
no Recipe or CallSlot implementation
no Builder/MIR/CFG/PHI
no runtime invocation or provider selection
no retry/fallback
no source annotation/rewrite/narrow fixture
```
