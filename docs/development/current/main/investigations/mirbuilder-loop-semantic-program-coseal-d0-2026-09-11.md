---
Status: active__NoSafeSlice__LoopSemanticProgramCoSeal__2026-09-11
Task: LOOP-SEMANTIC-PROGRAM-COSEAL-R0
Date: 2026-09-11
Priority: close the missing source-relation and entry-coverage design before any shared semantic receipt
Parent: mirbuilder-final-acceptance-scope-d0-2026-09-11
NextCard: receipt-authority design review
---

# Loop semantic-program co-seal D0

## Six-line brief

```text
Decision: retain the existing Callable I0 as caller-zero evidence; do not implement the all-family co-seal yet.
Source authority + canonical issuer: each family keeps its resolver/source parent; Callable I0 is the only complete issuer in this slice.
Non-authority: route_loop, Generic physical admission, Dynamic-only envelope, MIR/CFG observations, and matching owner/key values.
Fail-fast boundary: reject before a shared semantic receipt can be issued when source relations, coverage, family parent, or lineage are absent.
Smallest next slice: design the exact source-backed issuer and receipt contract for LoopNode/source, item/carrier, context, and complete entry coverage.
Non-claims: no new Verified*/Prepared* product, production selection or switch, physical cutover, fallback, retry, or legacy deletion.
```

## Worker-audited authority census

The read-only audit on 2026-09-11 found the following finite boundary:

| family | current source authority | current canonical issuer | current status |
| --- | --- | --- | --- |
| Callable | `CallableSemanticSourceLedgerView` plus `VerifiedCallableSingleLoopSourceMapV1` | `issue_callable_single_loop_recipe_v1` then `issue_callable_semantic_program_v1` | complete Callable-only caller-zero parent |
| Generic | `ResolvedFunctionLoweringInputV1` plus selection evidence | `issue_generic_g0_source_parent_v1` | parent exists; shared source-relation/coverage receipt does not |
| Dynamic | `VerifiedDynamicLoopFullBodySourceInventoryV1` | `issue_dynamic_full_loop_semantic_program_v2` | profile-specific V2 envelope; not a shared V1 issuer |
| M8/M9 | VariableAccum Facts/Recipe or versioned portable wire projection | no same-parent shared issuer identified | outside this safe slice |

The selected Callable production edge is
`lower_app_main_static_child` → prepared operation and terminates at
`lower_normal_cataloged_static_box_method_with_callable_single_loop_program_v1`
before draft-seal/pending commit/publication. This proves the bounded Callable
consumer only; `route_loop` remains the old scheduler and is not promoted.

## Missing design, not an implementation permission

The current products do not yet co-seal all required relations in one
source-backed parent:

1. exact `LoopNodeKey` → resolver Loop-source relation;
2. complete item/carrier source relations;
3. resolver-capability-backed semantic context;
4. opaque complete-entry-source coverage receipt;
5. one same-parent issuer for every admitted family;
6. one production caller, fail-fast terminal, and exclusive old-edge delete-set.

Generic `entries` and physical `entry_coverage_ok` are local checks, not the
opaque receipt required by the SSOT. Existing source claims similarly prove
structural shape, not source identity. Adding a guessed aggregate, default
receipt, or physical projection would create a second semantic authority, so
this row remains `NoSafeSlice` in `design_stop`.

## Required next Decision

The next design-only review must name, for each admitted family, the source
issuer and exact consumer for the four missing relation groups. It must also
record cardinality, lineage/frame/version checks, duplicate/foreign/missing
negative cases, the first production switch, typed terminal, and the finite
old-edge deletion set. Until that Decision is accepted, do not add a shared
`VerifiedLoopSemanticProgramV1`, alter `route_loop`, widen Callable, or open
M10b/M11/M12.

## Non-claims and acceptance boundary

This card does not claim Generic/M8/M9 coverage, repository-wide semantic
program issuance, CFG/SSA/PHI correctness, backend parity, selfhost parity,
production Loop selection, old-route retirement, or whole-MirBuilder
completion. Acceptance for this card is the recorded authority census and an
accepted bounded receipt-design Decision; no Cargo or source edit is required.
