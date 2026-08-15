---
Status: current design_stop — bounded child of the common V2 pre-session D0
Date: 2026-08-15
Parent: `docs/development/current/main/investigations/s6c-text-eq-physical-contract-d0-2026-08-15.md`
Authority: `VerifiedMainExpansionV1`, parser final callable source loan, and the source-backed callable catalog/selected-map issuer
Classification: T2 BoxShape; later bounded I0 is BoxCount only after this role boundary is accepted
---

# MAIN-STATIC-CHILD-SELECTED-MEMBERSHIP-D0

This card closes one missing membership boundary for the common V2 cohort. It
does not make every `Main` method a selected callable. The root `Main.main`
terminal, ordinary `Main` static children, and Dynamic eligibility are three
different roles and must not be inferred from the owner name.

## Six-line design brief

```text
Decision: co-seal only AppMainStaticChild rows into the existing selected batch map, while keeping AppMainRoot/Main.main on the existing root terminal and keeping Dynamic eligibility explicit and disjoint.
Source authority + canonical issuer: VerifiedMainExpansionV1::static_children() supplies the typed child role; the same VerifiedFinalCallableProgramSourceV1 HRTB supplies the exact InitialCallableFinalSlotV1::BoxMethod and parser CallableDeclarationIdentityV1; one source-backed catalog/selected-map issuer co-seals the pair.
Non-authority: owner == "Main", method == "main", method names, AST pointers, fixture spelling, canonical keys alone, raw batch slots, Main.main compatibility, and Dynamic shape/parameter contracts cannot issue selected membership or repair a foreign row.
Fail-fast boundary: before Port loan, reject missing/foreign/duplicate child identity, duplicate selected key or batch slot, Main.main smuggling, child-role/slot drift, non-static or non-direct child, and any AppMainStaticChild that reaches the Dynamic candidate gate.
Smallest next slice: freeze a role-bearing selected-inventory/map shape and one co-seal issuer; then implement only the existing Main static-child -> package Port loan with focused positive/negative coverage. No result/header, S6C Facts, V2 envelope, Builder session, or route code is opened.
Non-claims: no universal Main selection, no Main-name exception in a later consumer, no Dynamic candidate admission, no runtime ABI, no ResultCatalog/header, no S6C child, no common V2 transport, no MIR/CFG/SSA/PHI/session, no fallback or retry.
```

## Current evidence and decision

`VerifiedMainExpansionV1::static_children()` is already the sole typed source
role for independently lowerable static methods under the root `Main` box.
`callable_main_compat()` is a separate compatibility/root role and must not be
placed in the same selected coverage set. The existing
`VerifiedSourceBackedSameModuleCallableCatalogV1` owns declaration bodies and
the parser-issued selected identities; `issue_selected_callable_batch_map_v1`
is the only batch-slot joiner.

The current source-backed catalog drops every `Main` method by owner name. That
is too coarse for the production root path: `decls.rs` already sends each
`Main` static child through `lower_cataloged_static_box_method`, while the
installed Port cannot loan it because the selected map has no row. The fix is
not to delete the Main expansion or to add a second Main ledger. The existing
catalog/selected-map issuer must receive a role-bearing projection from the
same final-source loan:

```text
AppMainRoot
  -> root/optional callable-main compatibility terminal

AppMainStaticChild
  -> existing Cataloged key + parser identity + batch slot
  -> ordinary package/Port loan exactly once
  -> Dynamic eligibility = false unless a later explicit Decision changes it

NormalSelectedCallable
  -> existing selected map and existing Dynamic gate policy
```

The `AppMainStaticChild` row is identified by the parser final slot and opaque
identity that were issued together. A name or AST pointer may be used only as
an internal navigation check inside the same issuer; it is not a downstream
authority. The selected map may therefore contain the child for ordinary
lowering without making it a Dynamic candidate. Its role must be carried as a
typed disposition or equivalent sealed field, and `contains_dynamic_batch_slot`
must not mean “all selected rows”.

## Ownership shape

```text
VerifiedMainExpansionV1
  static_children() -> typed AppMainStaticChild source roles

VerifiedFinalCallableProgramSourceV1::with_callable_semantic_syntax(...)
  exact BoxMethod final slot + parser identity

issue_main_static_child_selected_inventory_v1(...)
  -> one role-bearing source-backed projection

issue_selected_callable_batch_map_v1(...)
  -> disjoint ordinary/Main-child rows, exact batch-slot coverage

InstalledNormalCallableSemanticPackageV1 / Port
  -> existing scoped lowering loan; no detached Main row
```

The co-seal must verify that each child role has exactly one parser row, that
the final slot is a direct `BoxMethod` in the same `Main` expansion, and that
the resulting canonical key is not already present in the ordinary selected
set. `Main.main` is consumed by the root materialization terminal, never by
the selected map. No `Main`-specific production route is added to the Port.

## Negative matrix

```text
Main.main presented as AppMainStaticChild       -> MainRootRoleConflict
Main child missing parser identity              -> MissingCallableIdentity
foreign final slot/identity                     -> ForeignMainChildCohort
duplicate child identity or canonical key       -> DuplicateSelectedKey
duplicate batch slot                            -> DuplicateBatchSlot
ordinary selected row collides with Main child  -> SelectedRoleOverlap
non-static/instance Main child                  -> MainExpansion reject
Main child reaches Dynamic eligibility gate     -> DynamicRoleLeak
raw batch slot or caller-supplied child         -> API unavailable / reject
Main owner/name changed while slot is foreign   -> identity/slot reject
```

Existing `Main` Dynamic-negative fixtures remain meaningful: a Main child can
be present in the ordinary selected coverage while still being excluded from
the Dynamic candidate set. They must not be “fixed” by silently allowing the
Dynamic parameter contract or by renaming the fixture.

## Bounded implementation row after acceptance

```text
MAIN-STATIC-CHILD-SELECTED-MEMBERSHIP-I0
  - add the role-bearing co-seal to the existing source-backed issuer
  - extend the existing selected batch map with disjoint role coverage
  - make Port loan the same Main child row once
  - preserve Main.main/root materialization and Dynamic=false
  - add positive ordinary Main-child loan plus foreign/duplicate/role-leak negatives
  - reuse existing package/catalog/Loop guards; no new top-level guard
```

The I0 is not authorized while the parent remains at design stop. It must not
touch the result/header issuer, S6C ingress, V2 operation/control envelope,
canonical session constructor, or physicalizer.

## NoSafeSlice conditions

Return to the parent `NoSafeSlice` if the implementation requires any of:

* selecting `Main.main` together with its children;
* a second catalog, selected map, Main side ledger, or caller-supplied batch slot;
* matching by owner/method name, AST address, fixture position, or Recipe shape;
* automatically turning every Main child into a Dynamic candidate;
* changing `Main` expansion to deferred/compatibility ownership;
* moving result/header ABI, S6C Facts, or physical session meaning into this row.

## Non-claims and handoff

This card only makes the existing production Main-child lowering edge
admissible to the same package/Port cohort. It does not claim that the S6C
`find_ok` body is selectable, that its unannotated result has a physical ABI,
or that any V2 operation can reach `CanonicalSsaFunctionSessionV2`. Those
remain parent-D0 blockers and must be co-sealed later from the same branded
source/batch cohort.
