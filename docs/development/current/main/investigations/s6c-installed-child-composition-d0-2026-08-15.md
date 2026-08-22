---
Status: I0 implemented and focused-gated; next pointer is common-V2 D0
Date: 2026-08-15
Work mode: fast
Classification: T2 BoxShape
Parent: s6c-text-eq-physical-contract-d0-2026-08-15.md
---

# LOOP-S6C-INSTALLED-CHILD-COMPOSITION-D0

This row closes the ownership boundary between the existing callable package
and the caller-zero S6C semantic spine. It does not open TextEq physical
execution or a MIRBuilder session.

The BoxShape is accepted. The next bounded implementation is:

```text
LOOP-S6C-INSTALLED-CHILD-COMPOSITION-I0
```

It is caller-zero and may add only the package-private seed/child models, the
single issuer wiring, and focused positive/negative tests. It must not open a
production caller, TextFormal signature mapping, V2 envelope, Builder/session,
route, fallback, or retry.

## Six-line brief

```text
Decision: issue one package-owned S6C child from the same selected batch/catalog cohort, and move one Completion seed into that child exactly once before generic header wrapping.
Source authority + canonical issuer: existing selected-map AppMainStaticChild role, resolved semantic row/source ledger, issue_s6c_typed_input_relation_v1, canonical CoreMethod target issuer, existing S6C Facts/Recipe/Join producers, and verify_function_completion_v1, orchestrated only by issue_normal_callable_semantic_package_v1.
Non-authority: Main/name/fixture predicates, raw batch slot or Recipe key, caller-supplied Facts/Recipe/ingress, test helper issue_facts, Port-side S6C reclassification, cloned/raw Completion, TextFormal wire arguments, MIR type, Builder state, and Dynamic admission.
Fail-fast boundary: before install/Port loan, reject foreign identity, incomplete or duplicate S6C candidate, resolver/source-ledger corruption, target-pair drift, missing/foreign/duplicate Completion seed, and any S6C seed entering generic header rows.
Smallest next slice: implement the package-private seed cohort, typed candidate disposition, retained S6C child, and Port exactly-once take/lend seam in caller-zero I0.
Non-claims: no TextFormal callable-signature mapping, V2 operation/control envelope, TextEq route/residence, ReadyEntry, Builder/MIR/CFG/SSA/PHI, physical caller, fallback, retry, or legacy retirement.
```

## Authority spine

```text
VerifiedResolvedCallableSemanticBatchV1
  + VerifiedSelectedCallableBatchMapV1(AppMainStaticChild role)
  + package-owned parameter/header cohort
      -> package-private S6C candidate classifier
           -> existing typed/source-bound/Facts/Recipe/Join issuers
                -> VerifiedS6CSemanticChildV1
                     owns retained S6C source/Recipe/Join parent
                     owns the one moved VerifiedFunctionCompletionV1
                     lends header/result and Loop Return/Tail siblings
      -> generic physical-header rows for ordinary seeds only
      -> InstalledNormalCallableSemanticPackageV1
           -> NormalCallableSemanticPackagePortV1
                -> verify issued role/identity and take/lend once
```

The package issuer is the semantic issuer. The Port is only the installed
cohort loan surface. It must not re-run the S6C classifier, reconstruct a
Facts/Recipe product, or accept a slot/key from a caller.

## Candidate disposition

The package issuer scans only selected rows whose role is
`AppMainStaticChild`. It uses the existing role/identity and source-backed
batch row; it does not search by owner or method name.

For each such row, the private classifier performs this exact sequence:

```text
1. role and selected/batch identity parity
2. StaticBoxMethod mode
3. exactly one loop site in the retained resolver membership
4. issue_s6c_typed_input_relation_v1(row, loop_site)
5. issue the canonical StringBox/Text Length and Substring target pair
6. issue the exact source-bound call relation
7. consume the row's Completion seed and issue Exit/Tail source co-seal
8. issue S6C Facts -> Recipe -> logical output -> prephysical ingress
```

The result is a typed disposition, not an `Option` fallback:

```text
Candidate       -> immediately consumed into the package-owned child
TypedNonMember  -> ordinary selected child; no S6C child is issued
HardReject      -> resolver/cohort/source/target/completion corruption
```

Shape mismatch is `TypedNonMember` only for the closed S6C family (wrong
parameter coverage, no unique loop, initializer/call/binary/assignment shape
mismatch). Missing source ledger ownership, selected identity drift, foreign
source, canonical target-manifest mismatch, Completion corruption, and a
second accepted candidate are hard rejects. Zero candidates is a valid typed
ordinary package; two candidates is `DuplicateS6CCandidate`.

The candidate disposition is package-private and is consumed immediately. It
does not become a second selected map, an external profile selector, or a
caller-visible row.

The selected-map seam must be a package-private role-bearing iterator over its
own `AppMainStaticChild` rows. It must not call `main_child_selection` with a
caller-provided statement/method or expose a detached slot/key iterator. The
issuer receives the retained row identity and role witness from that seam and
joins it to the same batch declaration exactly once.

## I0 landed evidence

The caller-zero implementation now has one
`VerifiedCallableCompletionSeedCohortV1` pass. The selected
`AppMainStaticChild` row consumes exactly one matching seed into
`VerifiedS6CSemanticChildV1`; remaining ordinary seeds alone feed the generic
physical-header projection. The child retains the complete prephysical S6C
parent, while Completion parity and Loop Return/Tail remain borrowed from
that parent. `NormalCallableSemanticPackagePortV1` lends the child without a
key/slot argument, rejects generic key-only admission for the role, and
rejects a second child loan before callback execution.

Focused evidence:

```text
mir::normal_callable_semantic_package::s6c_child_tests = 2 passed
mir::normal_callable_semantic_package = 29 passed
cargo check = green (inherited warning census only)
```

No Builder/session/MIR/physical caller, TextEq route, fallback, or retry was
opened. The next design stop is the parent common-V2 pre-session contract.

## D0 gates before I0

The following four type boundaries are mandatory; naming them is not yet an
implementation receipt.

1. **Seed identity and coverage.** Every seed carries the package/catalog
   brand, source identity, selected role, owner, and exact result projection.
   The private seed cohort proves `seed set == explicit-result selected rows`,
   `ordinary ∩ s6c == ∅`, and exactly one seed per owner. A raw batch slot is
   never the public lookup key.
2. **One Completion owner.** `VerifiedS6CSemanticChildV1` stores the retained
   S6C parent and header/result projection only. Its Completion/Loop Return/Tail
   view is borrowed from that retained owner; no separate `completion` field,
   clone, comparison, or second verification exists.
3. **Typed consumer split.** The generic header issuer consumes only ordinary
   seeds and never re-runs `verify_function_completion_v1`. The S6C seed is
   consumed by the child issuer before header wrapping. The installed Port
   exposes a private child loan alongside the existing exactly-once ledger;
   generic selected-input entry points reject the S6C role.
4. **Result contract before child issue.** The candidate must have an explicit
   `i64` result annotation and Completion declared-result parity before any
   child is issued. Missing/unsupported result is `TypedNonMember` during the
   closed candidate classification; a result/Completion mismatch after that
   point is `HardReject`, never a header `Option` or late fallback.

## One Completion seed, two typed consumers

The current physical-header issuer and S6C Exit/Tail issuer each verify and
own `VerifiedFunctionCompletionV1`. That is not a valid final ownership shape.
The seam is:

```text
one package-private CompletionSeedCohort
  -> S6C seed: move into VerifiedS6CSemanticChildV1
  -> ordinary seeds: move into generic physical-header rows
```

The generic header issuer receives only ordinary seeds. It never receives an
exclusion list or a raw S6C slot, and it never calls
`verify_function_completion_v1` a second time. The S6C child lends a narrow
header/result projection and a sibling Completion/Loop Return/Tail view from
the same retained owner.

Suggested private shape (names are design targets, not landed APIs):

```text
VerifiedCallableCompletionSeedCohortV1
  ordinary: Box<[VerifiedCallableHeaderCompletionSeedV1]>
  s6c: Option<VerifiedS6CCompletionSeedV1>

VerifiedS6CSemanticChildV1
  retained: VerifiedS6CPrephysicalIngressV2
  header: owner + exact scalar result projection
  completion: retained source/control authority, not a clone
```

No raw seed getter, `Clone`, `into_parts`, or detached Completion product is
allowed. The child and ordinary header rows are moved into the verified
package; after install, the Port can lend them only inside its HRTB callback.

## Acceptance matrix

Positive:

```text
one exact AppMainStaticChild -> one package-owned S6C child
one accepted Completion seed -> one child owner
ordinary seeds -> generic header rows only
S6C header/result view and Loop Return/Tail view share the child owner
zero exact candidates -> typed ordinary package with no S6C child
```

Negative before any Builder/session:

```text
Main.main or Required root treated as child
ordinary/Dynamic row treated as S6C
foreign selected/batch/source identity
zero or multiple loop sites classified as TypedNonMember for the closed family
resolver/source-ledger corruption classified as HardReject
missing or duplicate candidate
Length/Substring target or source relation drift
Completion seed missing, foreign, cloned, or consumed twice
S6C seed appears in generic header cohort
caller supplies Facts/Recipe/ingress/key/slot/fixture
Port reclassifies S6C or opens a second semantic issuer
borrow escapes the Port callback
```

## Ordered DAG

```text
LOOP-S6C-INSTALLED-CHILD-COMPOSITION-D0   [accepted BoxShape]
  -> LOOP-S6C-INSTALLED-CHILD-COMPOSITION-I0  [current caller-zero BoxCount]
  -> LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
       TextFormal callable mapping
       one Completion owner at the common boundary
       generic operations vs If/Exit control, exact S6C 15-set coverage
  -> LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
  -> LOOP-S6C-COMMON-V2-PRESESSION-I0       [caller-zero, Builder-free]
  -> route admission / canonical session / bounded cutover
```

## I0 acceptance

```text
positive:
  exact S6C fixture issues one child from one package cohort
  ordinary selected rows remain ordinary header rows
  child header/result and retained Completion sibling views agree
  package/Port child loan is exactly once and callback-scoped

negative:
  zero or duplicate candidate
  foreign role/identity/brand/source
  result annotation or Completion parity drift
  S6C seed reaches generic header
  second seed consume, raw Completion clone, detached key/slot, or callback escape

guards:
  production caller = 0
  Builder/session/MIR IDs = 0
  V1/Dynamic adapter and S6C physicalizer = 0
  fallback/retry = 0
  all new source files < 760 lines
```

## NoSafeSlice conditions

Keep this row in `NoSafeSlice::MissingInstalledS6CChildCompositionIssuer`
if any of the following is required:

```text
caller-supplied batch slot, key, Facts, Recipe, ingress, or fixture
name/owner lookup as membership authority
Port-side reclassification or a second typed issuer
Completion clone, comparison, or second verification
generic header exclusion list for S6C
detached child/header/Completion re-pairing
V1/Dynamic adapter, S6C physicalizer, Builder/session, physical ID
fallback, retry, or runtime route arbitration
```

The parent card remains the owner of TextFormal mapping and the generic V2
operation/control envelope. This child card does not reopen either design.
