Task: MIR-CALL-MAP-CALL-ARG-FLOW-I0
Parent: mir-call-map-local-entry-source-i0-2026-09-15.md
NextCard: MIR-CALL-MAP-LIFECYCLE-CONSUMER-I0 (C5 per-owner contract —
reachable only after call-arg rows exist)
Implementation permission: pending six-line brief + worker audit
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

## Six-line brief

```text
Decision: pending worker audit.
Source authority + canonical issuer: `observe_map` / `home_map_flow` +
a `map_argument_outward` verifier beside `map_entry_outward`.
Non-authority: no physical call-arg Map ABI, no emission claim.
Fail-fast boundary: non-map arguments and unobserved call families
stay as today; missing relation rows stay SourceMismatch.
Smallest next slice: pending — the CallArgument destination + walk
coverage for the observed statement shapes + focused tests.
Non-claims: physical arg-passing ABI, C5 per-owner arm, Map return ABI,
production switch.
```

## Acceptance

Pending: a `%{...}` at `Argument(ordinal)` produces a Complete flow
row with exact call-argument destination evidence; merged route's loop1
advances past `Body(1), Value, Argument(0)` to the next arm.
