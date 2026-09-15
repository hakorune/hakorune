Task: MIR-CALL-MAP-RETURN-SITE-FLOW-I0
Parent: mir-call-map-lifecycle-consumer-i0-2026-09-15.md
NextCard: per-owner lifecycle undertaking (C5 arm of parent)
Implementation permission: pending six-line brief acceptance
---

# Return-position map-literal flow observation I0

## Six-line brief

```text
Decision: extend map-flow observation so a `%{...}` MapLiteral at a
Return-value site produces an explicit row whose destination is the
function-return boundary — never silently skipped, never faked as a
Local binding.
Source authority + canonical issuer: the existing caller-ownership walk
in resolved_semantics/home_new_prefix.rs owns the terminal-Return arm;
resolved_control_flow/map_control.rs owns the outward verification;
home_map_flow.rs owns the sealed row shape.
Non-authority: the row must not mint a BindingRef for a return, claim a
physical consumer, or imply a Map return ABI exists.
Fail-fast boundary: return-position maps that fail exact membership /
path / owner / target checks stay Unavailable or named rejects; foreign
or duplicate sites stay rejected; `[Body, ReturnValue]` path is exact —
no nested shapes admitted.
Smallest next slice: add a ReturnBoundary destination kind to
MapHomeFlow, a `map_return_outward` verification for the
`[Body, ReturnValue]` path, and the terminal-Return arm hookup that
emits the row; focused positive/negative tests.
Non-claims: Map return physical ABI, per-owner install-contract
generalization (parent C5), admission success on the merged route —
this row only makes the site visible and explicitly unconsumed.
```

`Census boundary: merged entry program -> `%{` MapLiteral in
return position; 31 live sites (MirJsonEmitBox make_*/to_json family),
plus 27 `new MapBox()`-then-return functions downstream of the same
missing contract.`

## Entry contract

Parent card pinned the failing arm: `preflight_map_install`'s first loop
requires every MapLiteral site to have a `Complete` map-flow row, but
`observe_map` runs only for `local x = %{...}` initializers —
`return %{...}` sites produce no row and fail `map-source-unavailable`.
`map_source_outward` today pins `[Body(_), Initializer(_)]` paths and a
Local destination binding.

## Acceptance

Focused tests: `return %{...}` produces a `Complete` map-flow row with a
ReturnBoundary destination carrying the exact return statement site;
wrong-owner / wrong-path / non-literal return-value sites stay rejected
or Unavailable; existing initializer-position map tests stay green.
Route evidence: the merged entry advances past the first-loop
`map-source-unavailable` arm to the next named arm (expected:
per-owner/loan gate or terminal-relation coverage).
