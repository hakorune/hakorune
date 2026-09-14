---
Status: closed__design__2026-09-14
Task: MIR-CALL-NORMAL-PIPELINE-LIFECYCLE-ROUTE-RECOVERY-D0
Date: 2026-09-14
Priority: resolve the two ParentPassCurrentFail normal-ingress route regressions
Parent: mir-call-normal-pipeline-red-recovery-d0-2026-09-14.md
NextCard: mir-call-normal-ingress-ordinary-invoke-lifecycle-i0-2026-09-14.md
Implementation permission: false for this design card; the bounded I0 below is selected
---

# Normal-ingress lifecycle route recovery design stop

## Six-line brief

```text
Decision: preserve the two current-change reds as a selected-normal route blocker; do not change the expected CanonicalTyped route or add a compatibility retry until the lifecycle/fault-frame cause is mapped to an existing physical owner.
Source authority + canonical issuer: source-backed normal callable package and normal default finalization issue the call/body facts; PublishedMirBackendView and the selected-C lifecycle consumer own physical route admission.
Non-authority: test expectation, generic compatibility view, AST/name/arity, A3 Mixed admission, resolver If support, VM/LLVM parity, and any inferred route from a function count.
Fail-fast boundary: a selected module containing an unsupported lifecycle or non-lifecycle instruction remains UnsupportedBeforeObject with its first named physical terminal; no fallback or silent reclassification is permitted.
Smallest next slice: inventory the exact unsupported instruction/call rows for static Main + Main.helper and static Main + top-level helper, then select one existing lifecycle or call owner for a positive/negative recovery guard.
Non-claims: whole lifecycle promotion, all static/free calls, publication cutover, old-edge deletion outside this pair, VM parity, Windows evidence, or full-lib green.
```

## Finite regression inventory

| Source shape | Parent result | Current result | Named terminal |
| --- | --- | --- | --- |
| `static box Main { helper...; main() { return helper(2) } }` | `CanonicalTyped` | `UnsupportedBeforeObject` | published view route |
| top-level `function helper...` + `static box Main { main() { return helper(2) } }` | `CanonicalTyped` | `UnsupportedBeforeObject` | published view route |

The current source-backed MIR dump shows a `fault.frame.enter`/`invoke` pair
around the direct free-function call. This is an observation to map to the
existing lifecycle physical owner, not permission to remove fault cleanup or
to reclassify the call by name.

## Ordered bounded tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Instruction census | For both source shapes, record every `fault.frame`, `invoke`, call callee, lifecycle row, and selected-C eligibility from the published module. |
| 2 | Owner decision | Choose the existing lifecycle physical consumer or call-route owner that can losslessly consume the rows; if no issuer/consumer exists, keep `NoSafeSlice` and name the missing owner. |
| 3 | Route contract | Define the accepted source-to-physical relation and explicit rejects for unsupported lifecycle, instance, legacy, or mixed rows. No assertion-only retargeting. |
| 4 | Focused guards | Add positive guards for both finite source shapes and a negative guard for the first unsupported row; keep the two known baseline `published_consumer_*` failures outside this slice. |
| 5 | Implementation | Implement one owner-local change only after the design stop closes, with the selected old route/retry edge named for deletion. |
| 6 | Recheck | Run both exact reds plus the parent baseline inventory; only then decide whether to return to resolver-If expressivity. |

## Explicit states

| State | Meaning | Action |
| --- | --- | --- |
| `SourceCallRowsReady` | source package and exact call rows are co-sealed | continue to selected physical owner |
| `LifecycleRouteReady` | existing lifecycle consumer accepts the fault/invoke relation | eligible for owner-local implementation |
| `UnsupportedBeforeObject` | no lossless consumer for a row | stop before artifact; no fallback |
| `LegacyCallObserved` | selected normal path still carries LegacyCallV0 | reject named site before publication |
| `OwnerMissing` | no existing issuer/consumer can accept the relation | remain `NoSafeSlice`; do not invent a receipt |

The two current reds remain `CutoverBlockerOpen` until one of the ready
states is proved. The known parent baseline is recorded separately and must
not be used to justify a route assertion rewrite.

## Owner decision receipt

The lifecycle-route audit selected the existing normal finalization owner; it
did not select fixture isolation or generic-view promotion. Since `dcb33c91ad`,
ordinary static/free calls are represented as `MirInstruction::Invoke`, and
`PublishedMirBackendView::try_new` intentionally treats every `Invoke` as a
lifecycle instruction. The two red tests are older generic-view observers:
the parent SHA predates that physical shape, so the fixtures remain valid.

The canonical chain is:

```text
NormalDefaultRootCatalogLifecycle
 -> FinalizedRootHandoffV1 (CallReturn)
 -> compile_normal_with_published()
 -> bind_finalized_root_handoff()
 -> lifecycle_admission::admit_lifecycle()
 -> issue_lifecycle_physical_program()
 -> issue_lifecycle_compiled_entry_contract()
```

The generic `UnsupportedBeforeObject` terminal remains strict. The next
bounded slice removes only the tests' generic re-entry through
`compile_normal() -> PublishedMirBackendView::try_new(result.module)` and
observes the selected published callback instead. Both fixtures must prove
`CanonicalTyped`, `CallReturn`, one ordinary call (`Main.helper/1` or
`helper/1`), and its `OrdinaryI64` physical function. A negative guard keeps
the generic view at `UnsupportedBeforeObject` and forbids compatibility retry.

This is a route-observer correction at the existing lifecycle owner; it does
not promote the generic view, add fallback, alter fixture semantics, or claim
whole-lifecycle coverage. The selected next card is
`mir-call-normal-ingress-ordinary-invoke-lifecycle-i0-2026-09-14.md`.
