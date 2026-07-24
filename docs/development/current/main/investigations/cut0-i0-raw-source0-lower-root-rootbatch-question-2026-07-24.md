# RAW-SOURCE0 LOWER ROOT0 — ROOTBATCH0 design consultation

Status: **design stop — implementation forbidden until Q1–Q7 are answered**  
Date: 2026-07-24  
Decision: **pending**

BODY0 now returns a route-specific `RawRootBodyCompleteInvocationV1`.  The
product owns the candidate Builder session, an unpublished `main/0` draft,
the completed root-body witness, and the untouched Raw physical carrier.  It
does not admit a draft to the collector, reserve a root ledger entry, create
`condition_fn/1`, or publish a shell function.

The next boundary is ROOTBATCH0.  It must turn that one unpublished body owner
into one prepared Main/condition root batch without reviving the old
`PendingMainDraftV1` / `ModuleLoweringInvocationStateV1` authority.

## Current evidence

The following existing pieces are useful evidence, but none is an approved
ROOTBATCH0 ingress:

```text
src/mir/builder/module_draft_collector/root_batch.rs
  collector-wide preflight and infallible collector commit

src/mir/builder/raw_expansion_receipt_ledger.rs
  exact root_main / required_condition_fn requests
  complete_required_root_batch validation

src/mir/builder/root_draft_batch.rs
  generic Main/condition batch using PendingMainDraftV1 and caller policy

src/mir/builder/raw_root_completion.rs
  disconnected legacy raw root completion using the generic batch

src/mir/builder/raw_root_environment_install.rs
  CompletedRawRootBodyPhysicalV1 with private session/physical parts
```

The old generic batch is not directly reusable as the new boundary because it:

```text
accepts caller-selected ConditionFnPolicyV1
accepts a caller-supplied MirFunction condition draft
uses PendingMainDraftV1 / Main-only root state
does not retain the BODY0 route-specific owner as one aggregate
separates collector and ledger commit ownership
```

The worker inventory found one concrete identity blocker that must be answered
before implementation:

```text
BODY0 draft signature name     = "main/0"
Raw root admission identity    = "main", arity 0
MainDraftIdentityV1::root()    = "main", arity 0
ledger root_main request       = key Main, symbol "main"
collector root admission       = key Main, symbol "main"
```

This is not a spelling adapter opportunity.  ROOTBATCH0 must choose one
identity producer and make the BODY0 producer, draft signature, collector
admission, and ledger request agree.  A post-hoc `"main/0" -> "main"`
rewrite is forbidden.

There is a second authority gap: no production `condition_fn` draft producer
exists.  `module_lifecycle::finalize_module` has a development fallback that
must not be promoted.  ROOTBATCH0 must either add one exact typed Raw
condition-draft factory/recipe or split condition generation into a separately
decided prerequisite row.

`RawRootBodyCompleteInvocationV1` is the only accepted source of ROOTBATCH0
input.  Raw package, declared owner, CHILDREN0 owner, and CALLMAIN0 owner must
not open a root batch directly.

## Questions to decide

### Q1 — terminal owner

Which owner exposes the only consuming ROOTBATCH0 entry?

```text
1. RawRootBodyCompleteInvocationV1::prepare_root_batch(self)
   A compiler-visible aggregate terminal delegates a narrow Builder sibling
   terminal and keeps session, physical, body draft, and evidence together.

2. RawRootPostBodyPhysicalStateV1::prepare_root_batch(self)
   The Builder physical carrier owns the batch and the compiler re-wraps the
   result afterward.

3. PreparedRootDraftBatchV1 adapter
   Adapt the old generic batch and reconnect it to the new body product.
```

Recommendation: **1**.  The completed BODY owner already owns the session and
physical carrier as a pair.  Option 2 would require a loose session/physical
re-pairing in compiler code; option 3 would reintroduce generic Main policy.

### Q2 — `condition_fn/1` source authority

What creates the required Raw compatibility condition draft?

```text
1. A Raw-only typed Builder factory
   `RawRequiredConditionDraftV1::new()` fixes symbol condition_fn, arity 1,
   return contract, body shape, and `CanonicalRejectDuplicate` admission.

2. A neutral source recipe
   `RawRequiredConditionRecipeV1` is created with the source manifest and
   lowered by Builder at ROOTBATCH0.

3. Existing caller-supplied MirFunction / PendingConditionFnDraftV1
   Reuse the old generic draft input and validate it more strictly.
```

Recommendation: **1 unless the condition body is proven to be source-derived**.
The condition function is a Raw compatibility artifact, not a source
declaration in the accepted ScalarControl0 grammar.  Its exact semantics must
be fixed by one typed factory; a caller must not choose the body, policy, or
signature.  Option 2 is only valid if a source authority for the condition
body is identified explicitly.  Option 3 is rejected.

### Q3 — physical unpack boundary

How does ROOTBATCH0 access the unpublished `main/0` draft and collector?

```text
1. A Builder sibling terminal consumes the complete physical product and
   returns named prepared/rejected products.  Shell, collector, and ledger
   fields remain private.

2. Add a broad `into_parts()` and let compiler assemble the batch.

3. Expose collector/ledger/session tuples to the compiler for reuse of the
   existing root batch code.
```

Recommendation: **1**.  The terminal must be the only short-lived collector
loan and must not expose a raw tuple or a second physical owner.

### Q4 — prepare/commit ordering

Which atomicity law is adopted?

```text
1. Borrow-only preflight of route, brand/family, root draft, condition draft,
   collector indexes, and clean ledger; then one prepared pair reserves both
   root slots and one infallible commit records collector admissions and the
   two ledger events.

2. Reserve Main, collect Main, then reserve/collect condition_fn.

3. Reuse separate collector and ledger terminals and rely on call ordering.
```

Recommendation: **1**.  Main-only history, partial collector mutation, and
open reservation leakage must be impossible on every fallible path.  The
existing `complete_required_root_batch` validation is evidence for the lower
ledger primitive, not permission to split the outer owner.

### Q5 — publication dispositions

Which Raw-only policy is sealed into the batch?

```text
1. Required Main + Required condition_fn.  Main uses the existing Raw legacy
   whole-pair replacement disposition; condition_fn is inserted-only.

2. Caller-selected Required/Optional/Forbidden condition policy.

3. Treat condition_fn as an optimizer/materializer fallback when missing.
```

Recommendation: **1**.  Script and eligible App Raw roots share one required
compatibility condition law.  Optional/Forbidden belongs to other routes and
must not enter this Raw terminal.  Silent materializer fallback is forbidden.

### Q6 — rejection owner and failure law

What must a failed prepare retain?

```text
1. `RejectedRawRootBatchInvocationV1` retains the complete BODY0 owner,
   condition draft, prepared replacement facts, ledger/collector state,
   exact failing stage, and nested typed cause. It exposes inspection and
   `discard(self)` only.

2. Return only a collector error and let the caller reconstruct the owner.

3. Abort the ledger first and return a generic batch error.
```

Recommendation: **1**.  Retry, downgrade to Optional, replacing the body
draft, `into_parts`, and re-entry are forbidden.  Panic-to-typed rejection is
not claimed; the prepared commit must be infallible after preflight.

### Q7 — success handoff

What is the sole ROOTBATCH0 success product?

```text
1. `RawRootBatchCompleteInvocationV1::{Script, App}` owns the unchanged
   session, post-batch physical carrier, completed root-body witness, helper
   receipts, callable-Main outcome, collector-issued Main/condition receipts,
   and sealed Raw ledger.  Its only next terminal is DRAIN0.

2. Return a bare `MirModule` or `MirFunction` collection.

3. Return collector and ledger separately for later finalization.
```

Recommendation: **1**.  The root witness and receipts remain inseparable from
the physical owner until the later drain/finalizer row.

## Required non-claims

```text
BODY0 recipe/lowering changes                    = 0
legacy MainPending widening                      = 0
condition policy selection by caller             = 0
source AST re-scan / current_module inventory    = 0
collector mutation before full preflight         = 0
ledger mutation before full preflight             = 0
partial Main-only success                        = 0
condition_fn fallback or retry                   = 0
drain/finalization/postprocess/external commit    = 0
public ingress / JSON behavior                   = 0
production executor / CUT0 activation            = 0
```

## Suggested acceptance matrix after selection

```text
success: empty Script, Script scalar body, App Main.main/0
success: App helper receipts + callable-Main evidence preserved
failure: foreign brand/family, dirty shell, dirty collector index, dirty ledger
failure: wrong main symbol/arity, wrong condition symbol/arity/body contract
failure: duplicate Main replacement mismatch, duplicate condition insertion
failure: reservation overflow/poisoned ledger before collector mutation
atomicity: every prepare failure leaves BODY0 owner and physical evidence intact
one-shot: second root-batch entry, retry, fallback, and loose parts APIs = 0
```

## Recommended next row

`RAW-SOURCE0-LOWER0-ROOT0-ROOTBATCH0-CONSULT0` remains a design stop until
Q1–Q7 are answered.  No ROOTBATCH0 implementation or production consumer is
authorized by this document.
