---
Status: closed
Date: 2026-08-10
Row: `SOURCE-BOUND-DYNAMIC-METHOD-DISPATCH-I0`
Parent: `source-bound-dynamic-method-dispatch-d0-task-2026-08-10.md`
Mode: BoxShape / route-neutral target admission
---

# Source-bound Dynamic MethodCall dispatch I0

## Objective

Consume the landed neutral MethodCall source row and the existing
source-backed Dynamic receiver origin to issue one exact route-disjoint
DynamicMember target/source relation for the unchanged production
`skip_while/4` calls.

```text
VerifiedResolvedMethodCallSourceV1
+ exact receiver lexical BindingRef
+ VerifiedSourceBackedDynamicCallableV1 origin
        -> Candidate DynamicMember source relation
```

This row widens compiler acceptance. It must not narrow or rewrite the source,
classify by selector spelling, or infer semantics from `MirType::Unknown`.

## Structural change

Generalize the existing static-only source target catalog in place:

```text
VerifiedSourceCallTargetCatalogV1
  key = CanonicalSameModuleCallableKeyV1 + exact call site

VerifiedSourceCallTargetV1
  Static(existing target)
  DynamicMember(new source-bound relation)
```

Do not add a second Dynamic catalog. Temporary static-only projections are
compat adapters with an explicit removal condition. Static and DynamicMember
for the same caller/site are a typed duplicate rejection.

The resolved MethodCall row is branded by `FunctionOwnerIdV1`, while the
existing catalog is branded and keyed by `CanonicalSameModuleCallableKeyV1`.
Before inserting either target arm, this I0 must issue exactly one relational
bridge:

```text
cataloged VerifiedNormalCallableSemanticLoanV1
  + exact resolved function product
        -> VerifiedCatalogCallableOwnerLinkV1
             CanonicalSameModuleCallableKeyV1
             <-> FunctionOwnerIdV1
```

The link issuer co-seals the catalog allocation, exact callable source, and
resolved owner. It must not recover either side from symbol/name/arity or from
equal-looking numeric identity. The target issuer consumes this link; callers
cannot supply the two identities independently. Migrating the catalog itself
to an owner key is outside this I0 and must be a separate all-static-row
cutover if chosen later.

The landed link also owns the exact source ingress. The Dynamic target issuer
derives the resolver ledger, neutral MethodCall rows, and source-backed Dynamic
origin internally from that ingress. Supplying those products independently
was rejected during implementation review because it would reopen a
post-verification re-pairing seam.

## DynamicMember product

The new relation owns only message/source identity:

```text
caller `VerifiedCatalogCallableOwnerLinkV1`
exact neutral MethodCall source row
exact receiver BindingRef
source-backed Dynamic origin receipt
dispatch key = selector spelling + checked arity
ordered argument source rows
exact result source site
```

It owns no result value class, effect, Fault behavior, suspension, Home ABI,
provider, executable function pointer, Recipe item, or MIR value.

## Admission

```text
Candidate:
  neutral MethodCall row is complete
  exact catalog-callable/FunctionOwner link is sealed
  receiver site resolves to exact Bound lexical BindingRef
  BindingRef belongs to the same source-backed Dynamic origin
  caller/source/catalog identities agree
  target catalog has no existing row for the call site

Declined:
  complete CurrentOwner/proven-unbound static call
  complete declared-instance call
  complete non-Dynamic lexical receiver

Unresolved:
  receiver lexical/source-backed Dynamic evidence is unavailable

Rejected:
  foreign owner/catalog/source
  duplicate or colliding target arm
  receiver-site/BindingRef contradiction
  argument/result relation contradiction
  selector or arity supplied independently of the neutral source row
```

`NoSafeSlice` is a development state when the canonical issuer is absent; it
is not a source disposition.

## Acceptance

```text
positive:
  unchanged skip_while/4 issues DynamicMember rows for substring/2 and indexOf/1
  one exact catalog-callable/FunctionOwner link is consumed for each caller
  both rows retain exact neutral receiver/argument/result sites
  dispatch keys come only from the neutral MethodCall rows
  one route-neutral target catalog contains disjoint target arms

negative:
  foreign owner or source-backed Dynamic origin
  foreign/equal-looking catalog callable without the exact owner link
  non-Dynamic receiver
  receiver source mismatch
  duplicate same-site arm
  static/DynamicMember collision
  independently substituted selector/arity

guards:
  zero Builder/MIR/Recipe/runtime effect
  no selector-name semantic branch
  no symbol/name/arity reconstruction of caller identity
  no second catalog
  all new source files below 800 lines
  owner README and public MIR reference updated in the same commit
```

## Nonclaims and stop line

```text
no Dynamic result class
no Pure / Readonly / NonSuspending / NonControl default
no Fault or ordinary-result contract
no Home/lifetime contract for ch
no Recipe / CallSlot
no Builder / CFG / SSA / PHI
no provider or runtime route
no retry/fallback
```

After this I0, stop at
`DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-D0`. Recipe work remains closed until the
selector-independent execution envelope has an accepted canonical issuer.

## Landed receipt

The production normal/default lifecycle now extends the existing static
target catalog in place when the callable semantic batch is complete. Every
cataloged callable consumes one exact catalog/owner/source-ingress link; the
issuer walks the neutral MethodCall rows internally and publishes only
route-disjoint `DynamicMember` rows. Existing static consumers receive a
temporary static projection from the same catalog.

The unchanged full
`lang/src/compiler/parser/scan/parser_scan_loop_box.hako` source is the positive
acceptance fixture. `ParserScanLoopBox.skip_while/4` publishes exact
`substring/2` and `indexOf/1` rows without selector classification or source
narrowing. This exposed and repaired one compiler-wide acceptance gap: an
unbound bare MethodCall receiver is now sealed as a neutral
`QualifiedUnbound` receiver source rather than rejected as an ordinary
unresolved variable. Lexical receivers retain their exact resolver relation;
the qualified/static observer's explicitly requested coverage remains
unchanged.

Focused evidence:

```text
source_call_target::dynamic_member_tests             5/5
source_call_target::*                               60/60
resolved_semantics::body_shape_tests                  5/5
builder::normal_callable_semantic_source::tests       9/9
cargo check --lib                                    green
```

The root-lifecycle focused suite is classified red only in the pre-existing
duplicate-Box parser fixture: the current parser rejects that source before
the lifecycle test can assert its older CatalogSeal-stage expectation. This
row does not change that parser boundary.
