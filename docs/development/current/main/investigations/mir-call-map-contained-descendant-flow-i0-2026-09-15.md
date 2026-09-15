Task: MIR-CALL-MAP-CONTAINED-DESCENDANT-FLOW-I0
Parent: mir-call-map-call-arg-flow-i0-2026-09-15.md
NextCard: MIR-CALL-MAP-LIFECYCLE-CONSUMER-I0 (C5 per-owner contract —
reachable only after every sealed MapLiteral row has a flow row)
Implementation permission: pending six-line brief + worker audit
---

# Map contained-descendant flow I0

## Entry contract

The call-argument card landed `MapDestinationV1::CallArgument` for
direct `Argument(ordinal)` children of a terminal return value.
Merged-route loop1 evidence (2026-09-15) shows the next uncovered
family is **deeper containment**:

```text
[tmp/preflight-loop1]
site=[Body(0), Initializer(0), Element(0), Argument(1), Element(3)]
err=[freeze:contract] map-source-unavailable
```

`local blocks = [ me._block(0, [ ..., %{...}, ... ]) ]`
(merged L14990+, `emit_array_push_sequence`): the map literal's parent
chain is `Initializer -> Element -> Argument -> Element`. Neither the
direct-initializer LocalBinding arm nor the return-value CallArgument
walk reaches it — the local initializer is itself `PrefixNotCovered`
(array literal), yet every sealed `MapLiteral` row still needs a flow
row for preflight loop1.

`Census boundary: merged entry program -> `%{...}` sites whose parent
role chain is not (Initializer|Value|EntryValue|direct-Argument-of-
return-value); includes array elements at any depth, call arguments in
non-terminal statement positions, nested bodies; excludes sites already
issued via LocalBinding/ReturnBoundary/EntrySlot/CallArgument.`

Census correction note: the call-arg card's "exactly 2 sites" counted
only direct `(%{` patterns — maps nested inside containers inside call
args were undercounted. Re-census with path-prefix classification.

## Open design questions (worker audit — pending)

1. Generic destination vs per-role variants: is
   `MapDestinationV1::ContainedIn { parent: OwnedExprSiteV1,
   role: SourcePathSegmentV1 }` the right shape — one outward verifier
   (`parent.node + role` path tail + exactly one sealed relation row +
   scope/target triple) covering `Element`/`Argument`/future roles? Or
   keep per-parent variants? EntrySlot/CallArgument stay as-is (no
   refactor in this slice).
2. Enumeration order: for each walked statement, find all MapLiteral
   rows under `[Body(i)]` via path prefix on `shape.expressions()`,
   sorted in path order — is that the sealed source order? How do
   nested-body (if/loop) expression rows appear in the shape inventory?
3. Double-issuance: map sites already issued as LocalBinding /
   ReturnBoundary / EntrySlot / CallArgument children must be skipped —
   is there an issued-site set or does the walk need to subtract
   covered subtrees? A `%{...}` inside an already-observed map is an
   EntryValue child handled by `observe_map` recursion, not by this
   scan.
4. `locals`/`homes`/`used` state: maps inside a `PrefixNotCovered`
   initializer still consume live Homes in source order — does
   threading the running state into every descendant match the entry
   walk's semantics? What happens to `unavailable` gating (the
   LocalBinding arm is gated on `unavailable.is_none()` but descendant
   rows must be issued regardless)?
5. Unwalked statement positions: expression statements, if/loop bodies
   — pin test `declared_root_unissued_map_sites_stop_before_install`
   expects `Helpers.consume(%{})` (expression statement) and
   `if value { local m = %{} }` (nested body) to keep stopping install.
   Do we observe those maps too (row exists, pin's outer label
   survives via a different arm) or does the pin need scoping?

## Six-line brief

```text
Decision: pending worker audit — likely a generic `ContainedIn`
destination + per-statement subtree scan over sealed MapLiteral rows.
Source authority + canonical issuer: `observe_map` /
`scan_new_home_flow` + one `map_contained_outward` verifier.
Non-authority: no physical container/arg ABI, no emission claim.
Fail-fast boundary: sites already issued keep their specific
destination; unsealed paths stay SourceMismatch.
Smallest next slice: pending — generic destination + subtree
enumeration + focused tests.
Non-claims: physicalization, C5 per-owner arm, unwalked-statement
coverage unless the audit says loop1 requires it, production switch.
```

## Acceptance

Pending: merged loop1's first failure advances past the
`Element(0), Argument(1), Element(3)` containment site — ideally to the
`map_install_owners` arm (C5 territory) once every sealed MapLiteral
has a flow row.
