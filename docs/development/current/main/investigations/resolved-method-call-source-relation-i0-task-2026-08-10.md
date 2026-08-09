---
Status: ready after Dynamic dispatch D0
Date: 2026-08-10
Row: `RESOLVED-METHOD-CALL-SOURCE-RELATION-I0`
Parent: `source-bound-dynamic-method-dispatch-d0-task-2026-08-10.md`
Mode: BoxShape / neutral resolver source authority
---

# Resolved MethodCall source relation I0

## Objective

Add the missing reusable AST-free MethodCall source relation before issuing a
Dynamic target.  The existing full-body inventory proves the bounded
`skip_while/4` shape, but profile roles must not become the universal call
authority.

```text
exact resolved MethodCall
-> exact call/receiver/result sites
-> exact 0..arity ordered argument sites
-> selector spelling + checked arity as source syntax
-> resolver-owned AST-free row
```

## Product

```text
VerifiedResolvedMethodCallSourceV1
  FunctionOwnerId
  call site
  receiver site
  ordered argument rows { ordinal, site }
  result site = exact call expression site
  selector spelling
  checked arity
```

The row is sealed into the existing resolved function product and exposed by
`CallableSemanticSourceLedgerView::method_calls()`.  It is source authority,
not a call target or executable contract.

## Acceptance

```text
positive:
  substring/2 and indexOf/1 retain exact receiver/argument/result sites
  arguments are complete and ordered 0..arity
  full-body source inventory sites match the neutral rows

negative:
  missing / duplicate / gapped / reordered argument row
  receiver path mismatch
  result site mismatch
  foreign owner/source inventory
  nested callable boundary crossing
```

All failures occur before Builder effects.  New source files stay below 800
lines; split model, issuer, and tests before 760 lines.

## Nonclaims

```text
no Dynamic classification
no target catalog or dispatch key
no result value class
no effect/control/Home/suspension
no Recipe / CallSlot
no Builder / MIR / runtime
no name-based classification or fallback
```

Implementation and focused tests update `src/mir/resolved_semantics/README.md`
and the public Generic Loop stage reference in the same commit.
