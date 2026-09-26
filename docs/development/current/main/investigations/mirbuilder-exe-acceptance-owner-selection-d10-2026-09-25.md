# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D10

Status: open__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-HANDLE-ARG-S0
  (landed — handle-typed ordinary-new arguments admitted via
  `SelectedNewArgumentKindV1::Handle` + observation-preserving
  fallback coseal; `new UserStats(user)` claim rows proven; json
  advanced past `ordinary-new/argument-source-unavailable`)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D9 selection lineage.

## Problem

ORDINARY-NEW-HANDLE-ARG-S0 landed. The app advances to the next
named terminal:

```text
lexical scope body failed: lexical scope body failed:
[freeze:contract][static-result-ingress/no-exact-static-target]
   … at `env.get/1`
```

Emitted at `member_route.rs` `StaticReceiver` arm: a `StaticReceiver`
route plan probed `take_static_result_publication_ingress_v1`, the
caller classified `Cataloged` (`StringHelpers` static box method —
a `using`-imported module), and the publication owner returned
`NoExactStaticTarget` — `env` is not a declared box in the
declaration catalog. The call site is
`lang/src/shared/common/string_helpers.hako:128,146`
(`local dbg = env.get("HAKO_STAGEB_DEBUG")` inside `StringHelpers`),
reached because `apps/json-stream-aggregator/main.hako` has
`using selfhost.shared.common.string_helpers as StringHelpers` and
`JsonLine.intField` calls `StringHelpers.to_i64/1`.

`env` is a builtin/host service receiver, not a same-module
declared box. `NoExactStaticTarget` is a designed named stop for
exactly this class (no declaration => no target authority).

## Census questions (design_stop)

1. Which owner classifies `env`/builtin receivers today — is there
   an existing builtin-service publication lane, or is `env.get`
   expected on a different route plan entirely (Variable receiver,
   host extern, RetainedUnavailable)?
2. Does the same-module declaration catalog scope already include
   `using`-imported boxes (is `StringHelpers.to_i64` itself
   resolvable), or does every imported-module static call need a
   cross-module authority?
3. Finite inventory of builtin receiver names admitted by the
   route planner (`env`, `Math`, …) and which have declaration or
   physical-builtin authorities.
4. Smallest bounded slice: which ONE receiver class gets a real
   target authority, and what stays a named stop.

## Boundary

This census covers the `StaticReceiver` -> `no-exact-static-target`
edge only: route plan classification -> declaration catalog lookup
-> publication take. It does not touch the `me.`/DeclaredInstance
lane, the ordinary-new claim lane, or NamedArray/callable-loop
families.

## Exit

- [ ] Finite receiver inventory + owning authority named (or
      NoSafeSlice with reopen trigger).
- [ ] One bounded S-card emitted; pointers synced.
