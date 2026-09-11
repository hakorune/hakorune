---
Status: closed__DecisionAccepted__G0HelperBackendReach__2026-09-12
Date: 2026-09-12
Decision: LOOP-G0-HELPER-BACKEND-REACH-D0
Parent: mirbuilder-loop-g0-source-to-exe-acceptance-i0-2026-09-12.md
NextCard: LOOP-G0-HELPER-BACKEND-REACH-I1
---

# Generic G0 helper backend reach D0

## Six-line brief

```text
Decision: accept one bounded backend-reach implementation through the existing source-backed Global Call and physical-program issuer; do not synthesize a Call or scan the module by name.
Source authority + canonical issuer: existing VerifiedFinalCallableProgramSourceV1 -> NormalRootExecutionConsumerV1 -> VerifiedNormalCallableSemanticPackageV1 -> existing physical-program issuer, with the existing G0 selector/Recipe/Single owner reused after admission.
Non-authority: module function-name scans, source_ast() re-resolution, MIR/backend metadata, runner entry choice, test-only emitters, synthetic Calls, fallback, retry, and a second semantic receipt.
Fail-fast boundary: source owner/forest/header, selected-program membership, G0 policy, and helper ABI/call relation must be co-sealed before LLVM/C artifact construction or runtime execution.
Smallest next slice: use the existing normal-package `Callee::Global` form with `Main.main -> generic_g0(0, 0)`; assert exact root/helper physical membership and use the existing EXE route with result `3` when LLVM18 is available.
Non-claims: no all-family Loop reach, backend parity, LLVM18 success on unavailable hosts, Call/R7 completion, legacy retirement, or whole-MIRBuilder completion.
```

## Decision accepted

The source-backed Call authority is now concrete. The parser/resolver and
normal callable publisher already issue a `Callee::Global` for a top-level
FreeFunction call. `PublishedMirBackendView::try_new` records the same relation
as a `PublishedFreeFunctionCallRef`; the existing
`issue_lifecycle_physical_program` then consumes the root's typed Call through
`collect_ordinary_calls`, `ordinary_callable_key`, and the canonical definition
table. This is the sole selected-program membership path.

The bounded fixture shape is:

```text
static function generic_g0(i: i64, j: i64): i64 { <existing G0 loop> }
static box Main { main() { return generic_g0(0, 0) } }
```

`generic_g0/2` is therefore selected by the source Call, not by module-name
presence. Its two logical arguments match the two physical formal lanes and
its integer result is the existing ordinary-call ABI. For the current loop
body, the runtime oracle is exit code `3`; the prior Pair exit `30` is not a
G0 execution claim and must not be reused for this witness.

The next implementation may change only the focused acceptance fixture/test
and assertions around the existing issuer/emitter. It may not add a source
resolver, semantic receipt, module scan, synthetic Call, second issuer,
fallback, or backend route.

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

This card authorizes the bounded I1 implementation in
`mirbuilder-loop-g0-helper-backend-reach-i1-2026-09-12.md`. It does not
authorize all-family reach, backend parity, or a second semantic/physical
owner.
