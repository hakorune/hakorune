# MIRBUILDER-EXE-ACCEPTANCE-ENV-DIRECT-RECEIVER-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D10 (accepted)
Owner: workstream row H / unified resume gate 1
Authority: D10 Decision. `get_env_method_spec` (extern_calls.rs)
  is the one spec authority — shared by the located lanes —
  and `EnvMethod`/`finish_env_value_terminal`/`emit_env_value_
  terminal_raw_v1` is the already-wired physical owner.

## Slice

1. `src/mir/builder/calls/member_route.rs`
   `plan_member_call_route`: before
   `resolve_static_receiver_box_name`, admit
   `ASTNode::Variable { name == "env" }` receivers when
   `get_env_method_spec("env", method)` returns a spec —
   `MemberCallRoutePlan::EnvMethod`. Gate on
   `!variable_map.contains_key("env")` so a bound local keeps the
   existing Standard route (mirrors the static resolver's local
   check). A bare `env.<method>` absent from the spec table gets
   a named error in the located lanes' vocabulary
   (`env method not supported`), not a StaticReceiver probe.
2. `env.<field>.<method>` (FieldAccess) — unchanged, still handled
   by `resolve_env_method_call`.

## Overlap analysis (required, source-read)

- `EnvMethod` is already wired: `execute_prepared_member_call_
  route_v1` `EnvMethod` arm → `descent.finish_env_value_terminal`
  → `emit_env_value_terminal` → `emit_env_value_terminal_raw_v1`
  → `builder.emit_extern_call_with_effects` (`ExternCall`).
  `spec.returns` decides dst vs void — no terminal change.
- Both call facades (`build_member_method_call_v1`,
  `build_member_method_call_with_claim_ingress_v1`) reach
  `EnvMethod` through the shared executor — one planning point.
- Bare `env` is dead on every current lane (VM:
  `legacy-fallback-retired`; member_route:
  `no-exact-static-target`) — admission is unblock-only; no
  working path regresses.
- Bound `local env`/`env` in `variable_map` is preserved by the
  gate; located lanes check `name == "env"` unconditionally, but
  the member-route local-first rule predates this slice and is
  kept deliberately.

## Pins (required before close)

- Positive: `env.get("K")` on the member-route lane plans
  `EnvMethod` and emits `ExternCall(env.get)` — focused
  member_route/descent test.
- Positive: `env.console.readLine()` still plans `EnvMethod` via
  the FieldAccess arm (unchanged coverage).
- Negative: `env.noSuch(...)` → named `env method not supported`
  error, never `StaticReceiver`/`no-exact-static-target`.
- Negative: `local env = ...; env.get(...)` keeps the Standard
  route (no EnvMethod shadowing of a bound local).
- Guard: extend `mirbuilder_qualified_route_scope_guard.sh` —
  pin the env-direct arm before `resolve_static_receiver_box_
  name`, the `get_env_method_spec("env", method)` authority,
  the `variable_map` gate, and the named error.
- Real app: json_stream_aggregator advances past
  `env.get/1` (`StringHelpers.starts_with`-family debug blocks
  compile) — record the next honest terminal.

## Fail-fast boundary

- Unknown bare-env methods → named error, not a catalog probe.
- `env.<field>.<method>` behavior unchanged.
- Bound `env` local → Standard route (no shadowing).

## Evidence (landed — measured)

- `cargo test --lib member_route` — 16/16 green. New pins:
  - `env_direct_receiver_uses_env_terminal_and_emits_extern_call`
    (Variable("env") + `get` → `terminal:env` event +
    `extern_name() == "env.get"` instruction emitted)
  - `env_direct_receiver_rejects_unknown_method_with_named_error`
    (`env.noSuch` → named `env method not supported`, no descent)
  - `bound_env_receiver_keeps_standard_route`
    (`variable_map["env"]` bound → Standard route, no shadowing)
  - `env_route_keeps_receiver_syntax_only_and_descends_arguments`
    unchanged — `env.<field>.<method>` FieldAccess arm intact.
- `cargo test --lib static` — 6 failures, all confirmed identical at
  baseline (stashed + re-ran): `source_bound_static_result_owner`,
  `selected_generic_static_script_box`, `script_partition`,
  `raw_invocation_port`, `refresh_module_global_call_routes`,
  `main_static_child_port` — baseline debt, unrelated.
- Guard: `mirbuilder_qualified_route_scope_guard.sh` extended —
  pins the bare-env arm (`name == "env"` + `variable_map` gate +
  `get_env_method_spec("env", method)` + named error) and all three
  new test names; new files added to the 800-line check.
- File-size: `member_route_descent_tests.rs` reached 836 lines with
  the new tests — split into `member_route_descent_testkit.rs`
  (shared fixture, 272 lines) + `member_route_env_direct_tests.rs`
  (78) + trimmed descent_tests (500). All under 800.
- Real app (json_stream_aggregator, NYASH_BIN=debug,
  `pure-first`/`compat_replay=none`): advanced past
  `static-result-ingress/no-exact-static-target` — `env.get/1`
  inside `StringHelpers` (bare block `{ local dbg = env.get(...) }`)
  now emits ExternCall. Next honest terminal:
  `callable-semantic-lowering/placement-local-missing` — the
  `local dbg` statement site inside a bare block is not in the
  `locals` declaration inventory (`declaration_sites()`).
  → D11 census.
- EXE suite (11 scripts): 3 pass / 8 fail. json failure terminal
  changed as designed; `typed_object_untyped_field_min` panic
  (return_type_strategy ValueId(15)) confirmed identical at
  baseline — pre-existing debt; other failures unchanged.

## Non-claims

- Does not admit new builtin receiver names beyond the existing
  `get_env_method_spec` table.
- Does not change `spec.returns` semantics or value-position
  policy for non-returning env methods.
- Does not touch the located lanes' env arms (same authority,
  different caller).
- Does not claim `StringHelpers`/`json_stream_aggregator` green.
