Task: MIR-CALL-MAP-CONTAINED-DESCENDANT-FLOW-I0
Parent: mir-call-map-call-arg-flow-i0-2026-09-15.md
NextCard: MIR-CALL-MAP-ARRAY-ELEMENT-CONTAINER-SOURCE-I0
(array-entry leaf contract must admit nested container elements such as
`"incoming" => [[4, 1], [5, 2]]` — merged route's residual
`MapCandidateNotCovered` class), then MIR-CALL-MAP-LIFECYCLE-CONSUMER-I0
Status: landed — ContainedIn destination + interleaved descendant sweep
---

# Map contained-descendant flow I0

## Entry contract

The call-argument card landed `MapDestinationV1::CallArgument` for
direct `Argument(ordinal)` children of a terminal return value.
Merged-route loop1 evidence (2026-09-15) showed the next uncovered
family was **deeper containment**:

```text
[tmp/preflight-loop1]
site=[Body(0), Initializer(0), Element(0), Argument(1), Element(3)]
err=[freeze:contract] map-source-unavailable
```

`local blocks = [ me._block(0, [ ..., %{...}, ... ]) ]`: the map
literal's parent chain is `Initializer -> Element -> Argument ->
Element`. Neither the LocalBinding arm nor the return-value
CallArgument walk reached it — yet every sealed `MapLiteral` row still
needs a flow row for preflight loop1.

`Census boundary: merged entry program -> `%{...}` sites whose parent
role chain is not (Initializer|Value|EntryValue|direct-Argument-of-
return-value); includes array elements at any depth, call arguments in
non-terminal statement positions, nested bodies; excludes sites already
issued via LocalBinding/ReturnBoundary/EntrySlot/CallArgument.`

## Decision (worker audit integrated)

1. **Generic destination**: `MapDestinationV1::ContainedIn { parent,
   role }` — one outward verifier for arbitrary sealed child positions.
   Stronger evidence is preserved: `EntryValue` under a MapLiteral
   parent still issues `EntrySlot`, `Argument` under a sealed
   method/direct-call parent still issues `CallArgument`.
2. **Enumeration**: `shape.expressions()` MapLiteral rows filtered by
   statement-path prefix (`segments.starts_with(statement) + at least
   one extra segment`). Source paths are append-only descendants.
3. **Double-issuance**: skip sites already present in `maps` —
   `map_flow` rejects duplicates anyway; the issued-set check keeps
   EntrySlot-recursion children and specific-arm rows unique.
4. **State**: the sweep is interleaved inside the statement walk at
   three points (terminal arm before `break`, non-Local continue arm,
   after the Local `for ordinal` loop) and inherits the running
   `locals`/`homes`; a fresh `used` set per statement; `homes =
   remaining` threading. Descendant rows are issued regardless of the
   enclosing statement's `unavailable`.
5. **Unwalked coverage**: nested-body (`LoopBody`/`IfThen`/`IfElse`)
   maps are swept but their scope triple fails the outward verifier →
   `Unavailable` row (pins `declared_root_unissued_map_sites_stop_
   before_install` semantics: row exists, install still stops).
   A tail pass issues `Unavailable` for any sealed literal the walk
   never reached — keeps the one-row-per-literal invariant fail-closed.

## Six-line brief

```text
Decision: generic ContainedIn destination + per-statement sealed-row
prefix sweep, interleaved with the existing walk at program points.
Source authority + canonical issuer: `observe_descendant_maps`
(home_map_descendant_flow.rs) + `map_contained_outward` verifier.
Non-authority: no physical container/arg ABI, no emission claim.
Fail-fast boundary: EntrySlot/CallArgument keep stronger evidence;
unverifiable paths and nested-body scopes stay Unavailable.
Smallest next slice: array-entry container elements (EntryValue ->
Element -> non-leaf) — the residual MapCandidateNotCovered class.
Non-claims: physicalization, C5 per-owner arm, production switch.
```

## Acceptance evidence

- Focused: `map_contained_descendant_flow_tests.rs` 5/5 — ContainedIn
  destination + exact parent/role, merged-shape completion
  (`Element -> Argument -> Element`), stronger-destination
  preservation (EntrySlot kept, no re-issue), outward rejections
  (foreign owner, non-expression parent, wrong role), nested-body
  Unavailable pin.
- Package scope: 207/207 quick-profile green. One pin update:
  `array_entry_rejects_home_and_container_elements` now locates the
  outer return-map row by site — the outer map still stays
  Unavailable, but a contained descendant inside it now completes.
- Merged route advanced: the missing-row site
  `[Body(0), Initializer(0), Element(0), Argument(1), Element(3)]` is
  resolved. Loop1's first failure moved to
  `[Body(0), Initializer(0), Element(3), Argument(1), Element(0)]`
  (`emit_if_merge_local_return_var`, merged L14775+) with
  `has_row=true` — the ContainedIn row is issued; observation is
  Unavailable because `"incoming" => [[4, 1], [5, 2]]` puts a nested
  array at `EntryValue(2) -> Element(0)`, which the array-entry
  leaf-element contract does not admit (`MapCandidateNotCovered`).
  Outer label `MapLifecycleConsumerMissing` unchanged.
- Non-claims: no physical Map arg/return ABI, no C5 owner arm, no
  nested-body scope coverage, no production switch.
