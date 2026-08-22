---
Status: P0 selected; D0 accepted; implementation in progress
Task: MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-D0
Date: 2026-08-22
Priority: classify source-valid Void/unannotated callable rows without poisoning the sparse physical-header cohort
Parent: MIR-CALLABLE-PROGRAM-REGION-CONTAINMENT-P0
PreviousCard: mirbuilder-static-import-target-authority-d0-2026-08-22
NextCard: this rolling card owns the bounded P0
---

# Callable physical-header eligibility D0

## Six-line brief

```text
Decision: keep the physical-header cohort sparse. A selected callable with an unannotated or explicit Void source result is ordinary-valid but not header-eligible; it must not fail the whole package as CompletionNotValue. An explicitly admitted scalar result still requires a value-returning Completion proof.
Source authority + canonical issuer: VerifiedFinalCallableProgramSourceV1 and VerifiedResolvedCallableSemanticBatchV1 own the source result contract and opaque declaration identity; verify_function_completion_v1 owns exact exits/value classification; issue_callable_completion_seed_cohort_v1 is the sole package-scoped co-seal before issue_callable_physical_header_from_seeds_v1.
Non-authority: batch_slot alone, catalog key/name, source ordinal, AST/body/MIR inference, ResultCatalog, Builder state, fixture comments, completion.returns_value() without its declared source contract, and empty/default header rows.
Fail-fast boundary: classify each selected source row at the package pre-Builder boundary, before install, effects, Dynamic/S6C physical work, or publication; invalid explicit scalar rows reject, while source-valid no-header rows become an explicit sparse absence.
Smallest next slice: `MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-P0`; adapt only the existing completion-seed disposition so explicit accepted scalar values remain header-eligible while valid unannotated/Void rows become ordinary no-header, then add focused mixed/negative evidence. Do not change Text ABI or Builder paths.
Non-claims: no result inference, Text handle/wire, S6C activation, Dynamic publication, Completion verifier redesign, Builder/MIR/CFG/SSA, fallback/retry, production switch, legacy retirement, backend, or performance work.
```

## Observed production boundary

After `MIR-CALLABLE-PROGRAM-REGION-CONTAINMENT-P0`, the unchanged merged
production source passes the former `IfRegion(ControlContractMismatch)` and
stops before Builder effects at:

```text
[mir/callable-semantic-package/issue]
PhysicalHeader(CompletionNotValue { batch_slot: 36 })
```

The merged source contains a source-valid void-returning operator method:

```text
static box CompareOperator {
  apply(op, a, b) {
    return
  }
}
```

The read-only worker audit tied the observed row to the source anchor for
`apps/lib/std/operators/compare.hako` through the same parser semantic loan.
The batch slot is only transport evidence, not an identity. No implementation
may make `batch_slot == 36` the join key.

The current package path is:

```text
VerifiedFinalCallableProgramSourceV1
  -> complete resolved callable batch
  -> selected source-backed map
  -> exact parameter-contract rows
  -> issue_callable_completion_seed_cohort_v1
  -> issue_callable_physical_header_from_seeds_v1
  -> package install
```

The current seed issuer verifies Completion for each selected cataloged row
with a complete parameter contract, then rejects any row whose Completion is
not value-returning. The existing sparse-header README and tests state a
narrower contract: unannotated siblings are valid ordinary rows and absent
from the physical-header cohort. The open question is how the same rule must
cover a source-valid Void row without weakening explicit scalar validation.

## Worker audit and accepted boundary

The read-only audit is accepted for this D0. It found that the current
`completion_seed.rs` already classifies `Unannotated | Void` as
`result = None`, while `physical_header.rs` already treats `result == None`
as a normal sparse-header omission. The later unconditional
`!completion.returns_value()` rejection is therefore the narrow defect.

The accepted one-way classification is:

```text
source result contract
+ verified Completion
  ├─ explicit accepted scalar + value
  │    -> physical header seed
  ├─ unannotated / Void + valid Completion
  │    -> ordinary no-header seed
  └─ invalid / unsupported
       -> existing typed reject
```

This does not open a Void ABI or make `CompareOperator.apply/3` a physical
candidate. It only prevents an ordinary source-valid row from poisoning the
package's sparse physical-header cohort.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `VerifiedFinalCallableProgramSourceV1` | one parser invocation, callable membership, declaration identity, declared result spelling | physical header eligibility inferred from body/MIR |
| `VerifiedResolvedCallableSemanticBatchV1` | source-bound callable row and borrowed lowering input | batch-slot/name re-pairing or physical ABI |
| `verify_function_completion_v1` | exact exit sites, target, cleanup, and value/unit classification | deciding whether an unannotated row is selected for the sparse header |
| `issue_callable_completion_seed_cohort_v1` | one package-scoped co-seal of source row, parameter contract, identity, Completion, and optional scalar result | inventing a result or turning every selected row into a header |
| `issue_callable_physical_header_from_seeds_v1` | sparse physical-header rows for eligible source scalar results | rejecting ordinary no-header siblings or re-reading AST |
| package install/Port | move and callback-scoped loan of the already-issued cohort | a second Completion/header issuer |

The intended one-way relation is:

```text
source result contract + exact Completion
  -> HeaderEligible(scalar value)
  -> one physical-header row

source-valid Unannotated/Void
  -> OrdinaryNoHeader
  -> no physical-header row
```

An explicit scalar annotation is not equivalent to an unannotated value
return. The former can admit a header only when Completion proves a value;
the latter remains ordinary and has no physical-header result row.

## Candidate state table

| State | Source evidence | Package effect | Next |
| --- | --- | --- | --- |
| `HeaderEligible` | explicit supported scalar annotation + exact value Completion + complete formal contract | one seed and one sparse header row | install/loan |
| `OrdinaryNoHeader` | unannotated or explicit Void contract with source-valid Completion | package continues; no header row | ordinary selected route |
| `OrdinaryValueWithoutHeader` | unannotated source contract + exact value Completion | package continues; seed may retain Completion but result remains absent | ordinary route |
| `ExplicitResultCompletionInvalid` | explicit supported scalar annotation but Completion is missing/unit/foreign | typed package reject before effects | discard |
| `UnsupportedResultAnnotation` | explicit result annotation outside the accepted scalar cohort | typed package reject | discard |
| `FormalCoverageInvalid` | missing/duplicate/foreign parameter contract | typed package reject | discard |
| `SourceIdentityInvalid` | missing, foreign, duplicate, or ordinal-repaired identity | typed package reject | discard |

`OrdinaryNoHeader` is not a `default`, empty catalog, or fallback. It is an
explicit source-backed absence of physical-header admission. The cohort stays
always present; only its row lookup is optional, as the existing package
contract requires.

## Required D0 census

Before code, record the following using the existing source/batch owners:

```text
parser invocation witness for the observed row
opaque callable declaration identity
declared result contract: Unannotated / Void / Annotated(name)
Completion disposition and returns-value classification
parameter-contract identity and coverage
selected role: ordinary or AppMainStaticChild
whether a physical-header row is expected
whether S6C child admission observes the row
```

The census must include these source shapes:

```text
1. explicit : i64 + terminal value Return       -> HeaderEligible
2. unannotated + terminal value Return          -> OrdinaryValueWithoutHeader
3. unannotated + bare/void Return               -> OrdinaryNoHeader
4. explicit : void + unit completion            -> OrdinaryNoHeader
5. explicit unsupported result                  -> UnsupportedResultAnnotation
6. explicit : i64 + unit/missing value          -> ExplicitResultCompletionInvalid
7. foreign/duplicate identity                   -> SourceIdentityInvalid
```

The production merged probe must identify the observed Void row through the
same parser identity relation. A manually counted ordinal, method name, or
catalog key is only diagnostic evidence.

## Fail-fast boundary and no-effects law

All row classification happens before:

```text
CompilationContext install
Dynamic admission effects
S6C child/physical ingress
Builder function/session work
module collector or publication
```

The accepted sequence is:

```text
selected source row
  -> exact identity/owner/parameter coverage
  -> verify_function_completion_v1
  -> classify source result contract
  -> HeaderEligible or OrdinaryNoHeader
  -> co-seal package cohort
```

There is no transition:

```text
CompletionNotValue -> infer result from body/MIR
CompletionNotValue -> empty/default header
CompletionNotValue -> legacy retry/fallback
```

If a package-level invariant is invalid, the unpublished compilation session
remains the only discard authority. A source-valid ordinary Void row must not
poison unrelated eligible rows in the sparse cohort.

## Focused acceptance

Positive:

```text
explicit : i64 value row still lends one header
unannotated value row remains ordinary and has no header
unannotated Void row remains ordinary and has no header
explicit : void row remains ordinary and has no header
mixed package keeps eligible scalar rows when Void siblings are present
production merged probe passes the observed Void row and stops at the next
  already-owned source/semantic boundary
```

Negative:

```text
explicit unsupported result remains a typed PhysicalHeader error
explicit : i64 with unit/missing value remains a Completion error
foreign parser identity cannot be repaired by batch slot/name/order
duplicate parameter contract remains a typed coverage error
missing formal coverage remains a typed coverage error
no Builder/session/effect occurs on a rejected explicit row
```

Structural guards:

```text
physical-header issuer does not infer result from AST/body/MIR/ResultCatalog
CompletionNotValue is not the ordinary no-header terminal after D0
batch_slot is not exposed as a public join key
empty/default header construction = 0
fallback/retry = 0
new Text/ABI/Builder/physical receipt = 0
source files remain <760 lines; 800 is hard stop
```

## Ordered task queue

```text
MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-D0
  source/identity/role census; freeze the finite state and no-effects law

MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-P0
  only after D0 acceptance: adapt the existing seed disposition so
  source-valid Unannotated/Void rows are sparse absence, preserve explicit
  scalar fail-fast, add focused mixed/negative tests, and update one reusable
  guard; no Builder or ABI change

MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-R0
  rerun the merged production probe, prove the observed row identity and
  next blocker, then close the package-level evidence before any activation
```

## P0 execution brief — MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-P0

```text
change:
  reuse issue_callable_completion_seed_cohort_v1 and preserve its existing
  source result/Completion/parameter identity co-seal

accept:
  explicit accepted scalar + value -> existing header seed
  valid Unannotated/Void -> ordinary seed with no physical-header result
  unsupported or invalid explicit rows -> existing typed reject

files:
  completion_seed.rs, focused physical-header/package tests, one reusable guard

forbidden:
  body/MIR/ResultCatalog inference, new receipt, Builder/ABI change,
  fallback/retry, source reparse, ordinal/name pairing
```

P0 closes only after the focused mixed/negative tests, cargo check, source-size
guard, and the merged production probe all pass or the next blocker is
recorded. The next probe result is not a license to widen this row.

## NoSafeSlice conditions

Remain in `design_stop` if any proposal requires:

```text
body/MIR/ResultCatalog inference for an absent result;
batch slot/name/ordinal as the source identity;
re-running the resolver or reparsing the source;
making every selected Cataloged row a physical-header candidate;
dropping Completion ownership or creating a second Completion verifier;
using empty/default/Option merging to hide partial header state;
changing Text handle/ABI, S6C, Dynamic physical, Builder, or publication
  ownership to make this row pass;
falling back or retrying after package/header rejection;
accepting the production probe without exact source-bound row evidence.
```
