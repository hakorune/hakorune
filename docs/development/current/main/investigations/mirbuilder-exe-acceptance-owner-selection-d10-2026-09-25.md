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

## Census results (main-investigator, 2026-09-25)

1. **Builtin receiver owner already exists**: `get_env_method_spec`
   (extern_calls.rs) is the shared spec table — the located lanes
   (`helpers_value/lower.rs:269`, `loop_body_lowering_associated_
   input.rs:197`) already emit `CoreEffectPlan::ExternCall` for
   `Variable("env")` receivers through it. The member-route lane
   has `MemberCallRoutePlan::EnvMethod` wired at
   `member_route.rs:229-233` → `finish_env_value_terminal` →
   `emit_env_value_terminal_raw_v1` → `ExternCall` (returns flag
   handled: dst=None + void for non-returning methods).
2. **`using`-imported boxes already resolve**: `resolve_imported_
   static_box` maps the alias, and imported box declarations are in
   the same-module declaration catalog — `StringHelpers.to_i64/1`
   resolved fine (the freeze is only at `env.get/1`, not the
   imported static call).
3. **Receiver inventory on the route planner**:
   - `env.<field>.<method>` (FieldAccess) → `resolve_env_method_
     call` → `EnvMethod` (works today).
   - Bare `env.<method>` (Variable) → claimed by `resolve_static_
     receiver_box_name` first → `StaticReceiver` →
     `NoExactStaticTarget`. On the default VM lane the same call
     dies at `static-call/legacy-fallback-retired`. **Bare `env`
     is dead on every current lane** — admission is unblock-only.
   - `Math.<method>` → `qualified_math_compatibility_owner`
     inside `handle_static_method_call_with_descent` (untouched).
   - Bound `local env` → `is_local_var` → Standard receiver route
     (must keep).
4. **The gap is planner order + shape coverage only**:
   `plan_member_call_route` checks `resolve_static_receiver_box_
   name` before `resolve_env_method_call`, and the env check only
   accepts the `env.<field>.<method>` FieldAccess shape.

## Decision (accepted 2026-09-25)

```text
Decision: admit bare `env.<method>` receivers into the existing
          `EnvMethod` route before static-receiver classification.
Source authority + canonical issuer:
          `get_env_method_spec` table — one authority shared with
          the located lanes. Physical owner: `EnvMethod` route →
          `finish_env_value_terminal` → `emit_env_value_terminal_
          raw_v1` → `ExternCall` (already wired).
Non-authority: static-result publication ingress (`env` is not a
          declared box), `handle_static_method_call_with_descent`
          (Math-scoped), name-hardcoding outside the spec table.
Fail-fast boundary: bare `env.<method>` absent from the spec
          table → named error in the located lanes' vocabulary;
          bound `local env` keeps the Standard receiver route
          (variable_map gate mirrors the static resolver's local
          check).
Smallest next slice:
          MIRBUILDER-EXE-ACCEPTANCE-ENV-DIRECT-RECEIVER-S0 —
          `plan_member_call_route` admits `Variable("env")` +
          `get_env_method_spec("env", method)` → `EnvMethod`
          before `resolve_static_receiver_box_name`; pins:
          `env.get` → ExternCall plan+emission, `env.<field>.
          <method>` unchanged, bound `env` stays Standard,
          unknown env method → named error.
Non-claims: no imported-catalog change, no new builtin receiver
          names, no `env.set`-in-value-position policy change
          (spec.returns stays the terminal's concern).
```

## Exit

- [x] Finite receiver inventory + owning authority named:
      `EnvMethod`/`get_env_method_spec`/`emit_env_value_terminal_
      raw_v1`; bare `env` is the single unowned shape.
- [x] One bounded S-card emitted:
      MIRBUILDER-EXE-ACCEPTANCE-ENV-DIRECT-RECEIVER-S0.
