Task: MIR-CALL-MAP-CALL-ARG-FLOW-I0
Parent: mir-call-map-local-entry-source-i0-2026-09-15.md
NextCard: MIR-CALL-MAP-CONTAINED-DESCENDANT-FLOW-I0
(mir-call-map-contained-descendant-flow-i0-2026-09-15.md)
Implementation permission: landed
---

# Map call-argument-position flow I0

## Entry contract

Entry-class coverage is complete: every walked map literal's entries
now classify (String / BorrowedHandle / MapLocal / NestedMap /
NestedArray / scalar leaves). Pinned arm evidence on the merged route
(2026-09-15, temporary instrumentation — removed):

```text
[tmp/preflight-loop1] site=[Body(1), Value, Argument(0)]
    err=[freeze:contract] map-source-unavailable
-> [callable-semantic-package/install] MapLifecycleConsumerMissing
```

The first install failure is a **call-argument-position** `%{...}` —
`MirJsonEmitBox.to_json(%{"functions" => [main]})` — whose MapLiteral
body-shape row demands a flow row at preflight loop1 but is never
observed: `scan_new_home_flow` only walks `local x = %{...}`
initializers and terminal `return %{...}` values. The `Argument(u32)`
path segment and `ExprChildRoleV1::CallArgument(u32)` already exist as
sealed relations (`source_site.rs:52`, `source_path_policy.rs:102`).

`Census boundary: merged entry program -> `%{...}` sites at
`Argument(ordinal)` children of call/new/method-call expressions;
includes `to_json(%{...})` and `%{"functions" => [main]}` shapes;
excludes receiver positions and call-result initializers.`

Census result (2026-09-15): **exactly 2 sites**, both
`return MirJsonEmitBox.to_json(%{...})` (merged L14676, L14680) —
map literals as `Argument(0)` of a call in return-value position.
No expression-statement or local-initializer call-arg `%{...}` exists
in merged; no non-zero argument ordinal carries a map literal.

## Open design questions (worker audit — pending)

1. Walk coverage: which statement positions can contain a call with a
   `%{...}` argument — expression statements (`foo(%{...})`), local
   initializers (`local x = foo(%{...})`), return values
   (`return foo(%{...})`)? How does `scan_new_home_flow`/the caller
   enumerate expression children — is there a sealed way to find all
   `Argument(ordinal)` map children of a call expression site?
2. Destination shape: `MapDestinationV1::CallArgument { call: <site>,
   ordinal }` — what does the outward verifier check (call-site row
   membership, exact `parent.node + Argument(ordinal)` path tail,
   sealed call relation)?
3. Ownership semantics: a `%{...}` passed as a call argument — is the
   map constructed and transferred into the callee, or does the caller
   retain it? What does the existing call/argument Facts model say
   (affine Call rows, `home_local_call_flow`)?
4. Which statements trigger observation: does the walk need to scan
   every statement's expression children for call sites, or is there
   an existing per-statement call enumeration?

## Worker audit result (2026-09-15, read-only)

1. `scan_new_home_flow` walks only the root body's statement list;
   the terminal arm already reaches `ReturnValue` children — the scan
   for `Argument(ordinal)` map children belongs inside the `Ok(value)`
   arm and must not be gated on `terminal_call`/`return_scalar`.
2. Merged census is exactly 2 sites, both `return
   MirJsonEmitBox.to_json(%{...})` — return-value MethodCall
   `Argument(0)` only. Expression-statement/local-initializer call args
   stay uncovered (pin `declared_root_unissued_map_sites_stop_before_install`
   relies on this).
3. `MapDestinationV1::CallArgument { call: OwnedExprSiteV1, ordinal }`
   mirrors `EntrySlot`; `map_argument_outward` verifies owner/ptr
   equality, MapLiteral membership, `call.node + Argument(ordinal)`
   path tail, exactly one sealed relation row, a sealed method-call or
   direct-call row linking the ordinal to the site, and the
   scope/target triple.
4. No existing Facts arm models a caller-passed map — the row records
   destination evidence only; transfer semantics are a non-claim.
5. `observe_map` is destination-agnostic; sibling arg maps share one
   `used` set and thread `homes = remaining` in ordinal order.

Next-arm warning (acceptance target): `map_install_owners` still
rejects owners whose map entries are not TransferHome/scalar-Value —
`%{"functions"=>[main]}`'s `NestedArray` entry and `to_json`'s non-i64
terminal will trip the NEXT arm, which is this card's stated goal.

## Six-line brief

```text
Decision: admit `%{...}` at `Argument(ordinal)` children of a terminal
return value as `MapDestinationV1::CallArgument { call, ordinal }` —
destination evidence only.
Source authority + canonical issuer: `observe_map` / `home_map_flow` +
`map_argument_outward` beside `map_entry_outward`, driven from the
terminal `Ok(value)` arm's sealed `Argument(ordinal)` relations.
Non-authority: no physical call-arg Map ABI, no emission claim.
Fail-fast boundary: expression-statement and local-initializer call
args stay unobserved (pin holds); non-call parents, foreign owners,
and missing call rows reject in `map_argument_outward`.
Smallest next slice: CallArgument destination + verifier + terminal-arm
walk + focused tests.
Non-claims: physical arg-passing ABI, C5 per-owner arm, Map return ABI,
production switch.
```

## Acceptance

Landed: a `%{...}` at `Argument(ordinal)` of a terminal return value
produces a Complete flow row with exact `CallArgument` destination
evidence (`call` owned site + `ordinal`), verified by
`map_argument_outward` (owner/ptr, MapLiteral membership, path tail,
exactly one sealed relation, sealed method-call or direct-call row
linking ordinal→site, scope/target triple). Sibling argument maps
share one `used` set and thread `homes = remaining` in ordinal order;
a live Home entry inside an argument map consumes it exactly like an
entry-slot transfer.

Focused: 3 new tests in `map_call_argument_flow_tests.rs`
(destination row, live-Home transfer, outward-verifier negatives);
package scope 202/202 quick-profile green. `map_home_flow_tests.rs`
stays at its pre-existing 904 lines; the new tests live in the split
module per the 800-line hard stop.

Route evidence (2026-09-15, instrumentation removed): loop1's first
failure advanced past `[Body(1), Value, Argument(0)]` to
`[Body(0), Initializer(0), Element(0), Argument(1), Element(3)]` —
a `%{...}` nested inside an array element of a call argument inside an
array element of a local initializer
(`local blocks = [me._block(0, [..., %{...}])]`, merged L14990+). The
outer label stays `MapLifecycleConsumerMissing` — honestly recorded.
The residual family is **map literals contained in arbitrary
expression subtrees** (`Element`/`Argument` roles at any depth, plus
unwalked statement positions), not the direct call-argument child this
card covered.

Five unrelated `map_`-filtered reds (`map_write_timing_tests`,
`global_call_route_plan`, `mir_corebox_router_unified::map`) reproduce
identically on parent `d59fc6821f` — classified `known baseline debt`,
disjoint subsystem.
