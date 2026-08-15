---
Status: current design stop; no implementation or physical receipt is open
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/s6c-text-eq-physical-contract-d0-2026-08-15.md`
Classification: T2 BoxShape
Authority: final callable source, exact formal-parameter issuer, Completion owner, and the branded package/Port cohort
---

# CALLABLE-PHYSICAL-HEADER-ABI-D0

This is the first bounded child of the common-V2 pre-session stop. It closes
the ownership shape for one callable header cohort: exact formal parameters,
the source-backed result/header declaration, and the existing Completion
proof must travel together through the same branded package/Port loan. It does
not implement a runtime Text handle, a C ABI, or a Builder/session route.

## Six-line brief

```text
Decision: design one branded physical-header cohort for the complete callable signature (ExactText/i64/OpaqueHandle parameter rows plus an explicit source-backed result/header row) and co-seal it with the exact Completion owner; never issue a parameter wire independently from its result/header.
Source authority + canonical issuer: VerifiedFinalCallableProgramSourceV1 and VerifiedResolvedCallableSemanticBatchV1 provide the declared formal/result source rows; issue_callable_parameter_contract_v1 remains the sole formal-row issuer, verify_function_completion_v1 remains the sole exit/result-site issuer, and one same-brand CompilationContext/InstalledNormalCallableSemanticPackageV1/Port cohort is the only transport issuer.
Non-authority: MirType or LLVM signatures, HomeDemand::Handle, StringBox::equals, body/MIR return inference, fixture expectations, catalog key/name, Dynamic-only physical headers, raw AST re-expansion, detached ResultCatalog rows, TextEq/Substring, selectors, fallback, and retry cannot issue a physical header.
Fail-fast boundary: before any runtime or Builder capability, reject foreign brand/cohort, missing or duplicate formal rows, ordinal/BindingRef/owner drift, explicit StringBox downgraded to OpaqueHandle, absent/non-i64 result annotation, foreign Completion target/site/cleanup, detached result disposition, and any parameter-only or result-only header projection.
Smallest next slice: freeze the co-sealed header shape, same-owner HRTB seam, source-annotation decision, and typed negative matrix only; keep work_mode=design_stop and issue no code, fixture, ABI, handle, or Prepared* receipt.
Non-claims: no runtime Text handle/wire, stale-generation/liveness, retain/release, BoolOrTrap, TextEq route, S6C ingress/Recipe, common V2 envelope, ReadyEntry, Builder/MIR/CFG/SSA/PHI/session, production caller, selector switch, fallback, retry, or legacy retirement.
```

## Why parameters and result stay one cohort

The semantic ExactText formal row and the same-branded package/Port transport
row are already closed. A separate physical parameter ABI would nevertheless
split one callable signature into two authorities: parameters would acquire a
wire while the callable still has no source-backed result/header issuer. The
next design therefore owns the complete signature boundary first. The
physical wire for a Text parameter, if later admitted, is a projection of this
cohort; it is not a second signature authority.

The current S6C fixture has no explicit return annotation. That is a design
stop, not permission to infer `i64` from the body, MIR, a `ResultCatalog`, or a
fixture expectation. The accepted candidate is an explicit source annotation
normalized in the final callable-source transaction, followed by the existing
Completion verifier. If the source carrier cannot provide that annotation and
its owner identity, the row remains `NoSafeSlice`.

## Ownership shape

```text
VerifiedResolvedCallableSemanticBatchV1
  -> issue_callable_parameter_contract_v1
       -> exact formal parameter rows
  -> final callable source result/header declaration
       -> verify_function_completion_v1
            -> exact explicit exits + cleanup/target parity
  -> one branded package/install transition
       -> CompilationContext catalog + InstalledNormalCallableSemanticPackageV1
            -> NormalCallableSemanticPackagePortV1
                 HRTB loan of the same parameter/header/Completion cohort
```

The installed package does not become the catalog owner by itself: installation
moves the source-backed catalog into the existing `CompilationContext` owner.
The physical-header seam must retain the catalog brand and installed package
state as one cohort, and the Port may lend only that same-brand cohort. A
caller-supplied parameter row, result catalog, raw batch slot, or S6C ingress
is not a valid input.

The proposed shape is one non-splittable header cohort with sibling views:

```text
formal parameters: exact ordinal + BindingRef + owner/origin + formal ABI kind
result/header:     explicit declared result + source identity + target function
completion:        exact explicit return set + target + cleanup disposition
cohort seal:       same catalog brand + installed batch/selection identity
```

The TextEq leaf may later borrow its parameter and result views from this
parent, but this parent must not reissue TextEq source meaning or inspect a
Recipe item. The result representation catalog may be a downstream
publication projection only after it is tied to the same source identity and
Completion; it cannot be the header authority by itself.

## Required design acceptance

The D0 is accepted only when the design names all of the following without
implementation:

```text
1. one source-backed result/header issuer, with explicit annotation policy;
2. one same-brand package/install/Port transport seam;
3. one complete formal-parameter set, including ExactText StringBox-as-Text;
4. one Completion/return-site parity check owned by verify_function_completion_v1;
5. one typed reject for missing/foreign/duplicate/ambiguous header data;
6. zero body/MIR/fixture/name/key inference paths;
7. zero physical handle/wire, Builder, session, selector, fallback, and retry claims.
```

The source-shape decision may require an explicit `: i64` result annotation
for the S6C helper. That is a reference/source decision and must be recorded
before an I0; it is not a fixture edit permitted by this D0.

## Negative matrix

```text
foreign CompilationContext/catalog/package brand       -> ForeignCohort
missing or duplicate formal row                        -> HeaderCoverage
ordinal or BindingRef swap                              -> FormalBindingMismatch
StringBox downgraded to OpaqueHandle                    -> FormalKindMismatch
i64 reclassified as ExactText                           -> FormalKindMismatch
missing explicit result annotation                      -> MissingSourceResultHeader
non-i64 or foreign declared result                      -> ResultHeaderMismatch
body/MIR/ResultCatalog-only result inference             -> InferenceForbidden
Completion target/function mismatch                     -> CompletionTargetMismatch
Completion exit set or cleanup mismatch                 -> CompletionCoverage
detached result disposition or Dynamic-only header      -> ForeignHeaderAuthority
parameter-only physical wire                            -> IncompleteCallableHeader
raw AST/key/name/batch-slot caller input                -> API unavailable / reject
TextEq/Substring/physical handle inferred here          -> ScopeViolation
fallback, retry, or alternate physical target            -> FallbackForbidden
```

## Ordered DAG after this design stop

```text
CALLABLE-PHYSICAL-HEADER-ABI-D0             (current; design_stop)
  -> CALLABLE-PHYSICAL-HEADER-TRANSPORT-R0  (same-brand projection only)
  -> S6C-INSTALLED-BATCH-CHILD-COMPOSITION-D0
  -> LOOP-SEMANTIC-PROGRAM-COSEAL-R0
  -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
  -> LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
       13 operations
       + Recipe/JoinSig/Layout-owned If + Exit control
       + exact 15-placement coverage
  -> LOOP-S6C-COMMON-V2-PRESESSION-I0
  -> common V2 physical session
```

The existing `CALLABLE-TEXT-PARAMETER-COHORT-I0` is semantic/cohort evidence,
not a physical wire. It is a prerequisite projection for this D0, not a
replacement for it. Result/header design and physical Text parameter design
must not be opened as independent signature owners.

## Parked whole-builder convergence tasks

The following findings are real repository debt, but none is a reason to
invent a physical owner inside this D0. They are parked until the active Loop
cutover has a production caller and exact-HEAD integration evidence:

```text
MIRBUILDER-BRANCH-INTEGRATION-CLOSEOUT-R0
  main remains the canonical integration target; this work branch is not a
  second code SSOT. Merge/verify/pointer-sync before claiming repository-wide
  closure.

MIRBUILDER-WHOLE-BUILDER-TYPED-INGRESS-D0
  replace root/ordinary source_ast()->raw AST work-plan escape with one
  source-backed typed root/program loan; compatibility adapters remain one
  explicit boundary and may not become a second semantic issuer.

MIRBUILDER-COMMON-FINISH-CONVERGENCE-D0
  converge Canonical/Legacy/SelectedDynamic finish schedules behind the
  canonical session/DraftSeal owner after caller-zero proof; do not merge
  schedules by name or add fallback.

MIRBUILDER-WARNING-SURFACE-CENSUS-R0
  reproduce lib/test warning JSON at an exact HEAD and parent, classify
  current-change vs inherited by owner, then narrow module-level allows in
  bounded rows; no blanket allow removal or warning-driven semantic change.

MIRBUILDER-PHYSICAL-STRUCTURE-CLEANUP-D0
  after cutover, compact stale mirrors/archive/docs and split near-limit
  modules by owner without crossing 760/800 lines or moving authority.
```

These rows are not current execution rows. They do not authorize raw-AST
replacement, finish-route deletion, warning cleanup, docs deletion, or main
integration while this D0 is active.

## Stop conditions

Return to `NoSafeSlice` if any implementation proposal requires:

```text
issuing a parameter wire before a complete callable header exists;
inferring a result from MIR/body/ResultCatalog/key/name/fixture;
keeping a detached result catalog or parameter row and re-pairing by key;
using Dynamic-only physical header metadata for ordinary callables;
opening the S6C child from a test-built ingress or raw batch slot;
constructing a second Completion/return-site authority;
adding TextEq ABI/residence/route, V2 transport, Builder/session, fallback, or retry;
claiming main integration, whole-builder typed ingress, or finish convergence from local focused green.
```

## Current evidence classification

Worker audits agree that the semantic ExactText formal row, same-branded
parameter cohort, and Main static-child selected membership are closed. The
physical callable header is not: the source fixture lacks an explicit result
annotation, the installed package has no ordinary callable header, and the
existing result catalog is built later as a publication projection. The
current branch therefore remains `design_stop`; no physical parameter/header
receipt, S6C production caller, Builder/session, or main integration is
claimed.
