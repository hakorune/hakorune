# Declared object entry and physical tagged borrow S1

Status: Decision accepted / construction selected
Execution row: MIRBUILDER-DECLARED-OBJECT-TAGGED-BORROW-PHYSICAL-S1
Scope: source-admitted DeclaredObject entry/header projection through the existing
BorrowedTaggedValue, final object view publication and C class/null transport.
Related:
- mirbuilder-declared-object-tagged-borrow-source-s0-2026-10-05.md
- mirbuilder-declared-object-formal-contract-d1-2026-10-05.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Decision

Source authority + canonical issuer: original parameter contract and S0's exact
borrowed source loan, owner/ordinal/binding/class, canonical object membership and
existing entry ValueIds. Preserve the ordinary borrowed lifetime: no moved-in Home.
Existing entry ledger retains source correspondence; do not issue a second class
registry or infer class from MirType. Loan the class from the original source at
preflight, or mechanically carry it in the same staged entry row if needed.

Selected production consumers: source_scope stages the same original source entry;
borrowed_entry preflights exact declared name/type/header/ordinal/value/carrier;
binding_materialization performs all preflight before entry installation, then
commits Integer physical representation and BorrowedTaggedValue on the existing
column. The physical Integer never creates numeric or ownership authority.
Original source kind and declared parameter metadata remain intact.

Finalized source projection lends the declared flag with the existing canonical
object view. Final object_view_values publishes when declared OR field-admitted,
with the same exact root/duplicate verification. Ignored typed formals cannot lose
their class. Existing field coverage remains independent and exact.

Wire: every borrowed param has exactly value/representation/object_view. The view
is uint canonical object for class/null, explicit null for unrestricted opaque.
Other param representations keep the existing two-key schema. No new carrier,
domain discriminator, compatibility retry or second serializer.

C schema requires that borrowed key, validates null/unique-layout uint, rejects
scalar incoming when uint, and checks known New/receiver canonical class plus
Forwarded original Copy-root/view. Nullable acquired results keep their existing
LIVE proof; their index has no producer-class row, so default object_id0 cannot
prove class. The prologue restricts uint views to tag0/payload0 or tag3 with exact
runtime class, using existing nyash.object.type_id_h and layout.runtime_type_id.
Borrowed class checking adds no retain/release and precedes callee effects.

Non-authority: Box whitelist, payload bits, runtime liveness alone, names as
target resolvers, missing optional evidence, inferred Home, fixture exceptions.
Missing/foreign source class, header, entry value, class view or carrier fails
before installation/publication/artifact at its existing owner.

External C cannot distinguish a source-inconsistent uint->null edit from a new
unconstrained function without the original AST. That correspondence negative
belongs to the source-backed final Rust boundary; C missing-key rejection is a
different obligation. Do not claim compile-time nullable producer class evidence
or recovery of erased source semantics.

## Acceptance and work order

1. Exact-source entry positives for declared object and null, alongside opaque
   regressions. Reject foreign header name/class, ordinal/binding/value/collision,
   implicit receiver, physical carrier and original type-context drift. All
   preflight errors leave entry and metadata uncommitted.
2. Same declared-original/copy -> opaque guarded field-reader through final MIR
   and JSON. Publish the ignored declared formal's uint view. Source finalization
   rejects missing/foreign declared view and source/header correspondence.
3. C rejects missing/invalid view, scalar input to uint view, known foreign class,
   wrong Forwarded root/view and foreign Copy. Class checking uses runtime type
   ID, never object ordinal. Keep nullable received-result positives and borrowed
   release negatives; no relaxation of acquired-LIVE transport.
4. Both optimization settings compile/run null/object and Normal/Fault witnesses;
   caller cleanup remains sole. Update only the handwritten borrowed-ingress
   helper to explicit object_view:null and regenerate Rust-issued witnesses.
5. Focused tests, physical boundary/JSON/C affected families, package baseline
   classification, selected pins, pointer/whitespace and source size checks.
6. Fresh quick llvm-boundary CLI and unchanged mimalloc-lite frontier probe.
   Record actual terminal; do not equate fixture success with app completion.

Replacement/deletion set: source-admitted typed entry exclusion, field-only view
publication for declared roots, optional borrowed view spelling and unrestricted
class-domain prologue for uint views. Shared generic opaque/numeric/nullable
owners stay; no backend fallback or unrelated old-edge retirement.

## Entry evidence and obligations

S0 landed/pushed c0a8bf05bd: source5/5, entry/copy18/18, package622/known3;
exact known names/causes and HEAD-identical root-test1351 guard debt are in S0.
Pointer/whitespace PASS. HEAD equals remote; no Cargo live at S1 selection.
Protected C/S0/Array/S2 WIP remains unstaged and must be preserved.

Read-only audit_cost_oct5 reviewed owner mapping, mandatory null|uint wire,
nullable class provenance gap and runtime ID distinction. Main owner integrates
the decision here; no worker editing or Cargo.

Unchanged actual app last stops artifact-unowned-lifecycle-site. Whole-definition
<= vocabulary, borrowed return/mutation, call-valued result and exact owning
Provided destination remain later mandatory obligations. This S1 does not reopen
parked ownership, rewrite production .hako or close the finite pipeline goal.
