---
Status: Design stop — source-backed helper reach authority is unresolved
Date: 2026-09-12
Decision: LOOP-G0-HELPER-BACKEND-REACH-D0
Parent: mirbuilder-loop-g0-source-to-exe-acceptance-i0-2026-09-12.md
NextCard: none__G0HelperBackendReach__PendingAuthorityDecision
---

# Generic G0 helper backend reach D0

## Six-line brief

```text
Decision: keep this row at design_stop / NoSafeSlice until a source-backed selected-program membership and call authority is named; do not synthesize a Call or scan the module by name.
Source authority + canonical issuer: existing VerifiedFinalCallableProgramSourceV1 -> NormalRootExecutionConsumerV1 -> VerifiedNormalCallableSemanticPackageV1 -> existing physical-program issuer, with the existing G0 selector/Recipe/Single owner reused after admission.
Non-authority: module function-name scans, source_ast() re-resolution, MIR/backend metadata, runner entry choice, test-only emitters, synthetic Calls, fallback, retry, and a second semantic receipt.
Fail-fast boundary: source owner/forest/header, selected-program membership, G0 policy, and helper ABI/call relation must be co-sealed before LLVM/C artifact construction or runtime execution.
Smallest next slice: decide whether an existing source-backed Call-bearing program can legitimately select generic_g0/2, or whether the source/Call authority must be expanded in a separate Call/R7 row; then name the exact physical-program input and runtime oracle.
Non-claims: no helper LLVM-input reach, helper runtime result, LLVM18 success, backend parity, new fixture, Call/R7 completion, legacy retirement, or whole-MIRBuilder completion.
```

## Bounded census

The boundary starts at the existing normal-package source program and ends at
the existing `PublishedLifecyclePhysicalProgramV1` membership issued to the
LLVM physical backend. It includes the published `generic_g0/2` helper, the
ordinary `Main.main/0` root, the existing top-level G0 selector, its Recipe,
the Generic G0 physical cohort, the Single lifecycle, and the existing
runtime/EXE oracle once a legitimate selected program is defined.

It excludes module-wide function discovery, a synthetic root-to-helper Call,
changing the current I0 fixture only to manufacture reach, a test-only
physical emitter, backend fallback/retry, a new semantic `Verified*` or
`Prepared*` product, and any deletion or retirement. The I0 evidence remains
exactly package publication plus root-EXE coexistence; its root returns 30 and
does not call `generic_g0/2`. The LLVM18-unavailable run was skipped, not a
helper execution success.

## Worker premise audit (read-only, 2026-09-12)

The consulted worker found that the physical-program issuer currently collects
the root plus ordinary callees that are actually called from that root:
`src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program.rs`
(`issue_lifecycle_physical_program`, around lines 221 and 277). A helper being
present in the module therefore does not place it in LLVM input. The current
fixture's `Main` has no G0 Call, so no legitimate runtime route exists within
this boundary.

The source-backed G0 chain is reusable once the admission is designed:
`src/mir/builder/normal_callable_semantic_loan_port/generic_g0.rs`, existing
`generic_g0_recipe`, `VerifiedGenericRecipeProductG0`,
`GenericG0PhysicalOperationCohortV1`,
`lower_resolved_generic_g0_function_draft`, and the Single lifecycle. The
worker found no safe implementation slice that can make the helper execute
without first deciding the source-backed membership/Call authority.

## Required future acceptance

The positive must show that the source-backed selected program contains
`generic_g0/2` in the physical ABI input, that LLVM receives that exact
membership, and that an existing legitimate call path executes the helper with
its expected result. Negatives must mutate helper membership, source owner,
header, arity, or call relation and reject before LLVM/C artifact publication.
The reusable guard must reject module-name re-search, `source_ast()`
re-resolution, synthetic Call creation, route-loop entry, test-only emitter
use, fallback/retry, and a second issuer/receipt.

This card authorizes only the next design decision. It does not authorize code,
fixture, production switch, backend route, or semantic receipt changes.
