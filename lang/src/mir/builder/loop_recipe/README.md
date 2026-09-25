# lang/src/mir/builder/loop_recipe — portable LoopRecipe wire (S7A)

Owner boundary
- Owns the `.hako` side of the `LoopRecipeArtifactV1` wire:
  `{ schema_version, provenance, source_binding, recipe }`.
- `emit_loop_recipe_wire.hako` (S7A) emits one fixed minimal
  `direct_accum_v1` artifact; `emit_m8a_recurrence_wire.hako` (S7B1)
  emits the canonical M8A `variable_accum_recurrence_v1` artifact for
  the bounded profile `loop(i < 4) { acc = acc + i; i = i + 1 }` at
  root body item index 2;
  `emit_m8c_scans_wire.hako` (S7B3) emits the canonical M8C
  `LoopRecipeArtifactV2` (`schema_version` 2, `scan_with_init_v2`
  provenance) for the bounded `find_ok` profile
  (`apps/tests/scan_with_init_typed_ok_min.hako`) with the loop at
  body item index 1 — the first V2 wire row, covering `call_slot`,
  `text_eq`, `text` value class, and a `return` exit kind;
  `emit_m8d_loopcond_wire.hako` (S7B4) emits the canonical M8D
  `loop_cond_break_continue_v1` artifact for the bounded
  `loop_cond_function_for_test` profile (`local flag = 1;
  loop(flag < 2) { if flag == 1 { break } else { continue } }`)
  with the loop at body item index 1 — the first wire row covering
  a `continue` exit kind and an `If` item with a populated
  `else_block`.
  Each entry assembles its artifact from named
  string-fragment locals in fixed serde field order and prints it as
  one compact JSON line to stdout. Field names and tagged kinds mirror
  `src/mir/loop_recipe_contract/schema.rs` (V1) and `schema_v2.rs`
  (V2) exactly.
- The Rust decode/verify/normalize owner remains
  `LoopRecipeNormalizerV1` (`src/mir/loop_recipe_contract/normalize.rs`);
  the parity harness lives in
  `src/mir/loop_recipe_contract/wire_parity_tests.rs` and consumes the
  checked-in emissions at
  `src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_v1.json`,
  `hako_loop_recipe_wire_m8a_v1.json`,
  `hako_loop_recipe_wire_m8c_v2.json`, and
  `hako_loop_recipe_wire_m8d_v1.json`. The M8A arm compares against
  the artifact the real Rust producer
  `produce_variable_accum_recurrence_recipe_v1` yields for the same
  bounded source profile; the M8C arm
  compares against the artifact rebuilt through the M8C producer's
  own issuer calls (`build_recipe` -> `bind_verified_artifact`) and
  anchors `normalize_semantic` on the real
  `produce_s6c_scan_with_init_recipe_v2` product via
  `LoopRecipeNormalizerV2`; the M8D arm rebuilds the artifact
  through the producer's own issuer chain — the typed source map's
  `VerifiedLoopRootSourceV1` + `loop_cond_break_continue_recipe`
  (`pub(super)`) — and anchors `normalize_semantic` on the real
  `produce_loop_cond_break_continue_recipe_v1` product issued
  through its own policy-demand chain. The M8B variable-accum-break
  emitter, fixture, and parity arm were retired at M12-R2C with the
  caller-zero `VariableAccumBreak` producer chain (its
  `LoopBreakRecipe` attestation never matched the production owner);
  `LoopBreakRecipe` is a typed-declined row in the canonical route
  census. The M8E generic-residual
  emitter, fixture, and parity arm were retired at M12-R2B with the
  caller-zero `GenericResidual` producer chain; `GenericLoopV1` is a
  typed-declined row in the canonical route census.
- The S7G all19 closeout module
  `src/mir/loop_recipe_contract/wire_route_parity_tests.rs` adds three
  more arms covering the remaining attested-backed routes —
  `AccumConstLoop` (`emit_direct_accum_wire.hako` ->
  `hako_loop_recipe_wire_accum_direct_v1.json`, the live producer's
  `induction`/`accumulator` role-label artifact at `body_item(1)`),
  `NestedLoopMinimal` (`emit_nested_predicate_wire.hako` ->
  `hako_loop_recipe_wire_nested_v1.json`, the live producer's
  `root_0`/`root_1`/`child_0` role-label artifact) and
  `LoopTrueBreakContinue` (`emit_loop_true_wire.hako` ->
  `hako_loop_recipe_wire_loop_true_v1.json`) — each rebuilt through
  the producer's own issuer calls and anchored
  `normalize_semantic` on the real producer product, plus one
  all-route coverage census pinning every canonical route as
  wire-backed parity or typed decline against the sealed
  `ATTESTED_RECIPE_BACKED_V1` attestation and the
  `producer_id_migration` `RECEIPTS` inventory. The S7A substrate
  emission (`hako_loop_recipe_wire_v1.json`) stays pinned to the
  older-era `accum_direct_v1` golden (source-name labels,
  `body_item(0)`) as the wire-transport witness; it is not the
  AccumConstLoop cohort artifact. The `nested_predicate_v1`
  golden remains a decode-and-verify witness for the same
  reason.

Non-goals (must not grow here)
- No producer cohort, Facts, RoutePolicy, JoinSig elaboration, verifier,
  CFG/PHI, physical MIR, or default authority.
- No Program(JSON v0) producer input, no `route_loop`/registry wiring, no
  production caller — the subtree is caller-zero.
- No ASTNode/synthetic AST, borrowed Facts/frame, MirBuilder internals,
  CorePlan, ValueId, BasicBlockId, Frag, callbacks, traits, Rust lifetimes,
  selector capability, retry, suffix, or fallback state on the wire.
- No import of `lang/src/compiler/mirbuilder/**` (compat surface) or
  `lang/src/selfhost/mir_builder/**` (scaffold). No hostbridge or host
  callback; `tools/checks/hako_mirbuilder_no_hostbridge.sh` covers this
  subtree.
- The `LoopRecipeArtifactV2` row landed at S7B3 as wire coverage only:
  `emit_m8c_scans_wire.hako` emits the canonical artifact; no `.hako`
  V2 producer, Facts, or verifier exists here.

Executable-subset boundary (S7A D1 finding)
- On current HEAD the plain `.hako` execution paths (`--backend vm`,
  `vm-hako`, `llvm`, `hakorune-compat`) admit only scalar/string locals,
  `+` concat, loop/if, print, and return inside `Main.main()`: user method
  calls (`me.*`, static, instance, top-level `fn`), MapBox/ArrayBox
  literals, and `env.get` fail with
  `[freeze:contract][static-call/legacy-fallback-retired]` or
  `[vm-reference/legacy-call/global-stopped]` /
  `NewBox intrinsic-target-unsupported`. The legacy compat entry
  `compat/emit_mir_json_v0.hako` itself no longer runs (`env.get`
  retirement) — baseline debt observed during S7A.
- Because of this boundary the S7A slice is one file: the DTO/emitter
  split (`loop_recipe_wire_box.hako` + `loop_recipe_wire_emit_box.hako`)
  sketched in the D0 card is deferred to the first row whose card names an
  executable mechanism for `.hako` method calls (S7B lane decision).
- S7B rows therefore land as wire-coverage cohorts: each entry emits its
  family's canonical artifact; the Facts/RoutePolicy/JoinSig producer
  half of M9 stays deferred until a `.hako` execution-mechanism row
  lands in its owning lane (S7B1 card, "deferred claim").

Regenerating the checked-in emissions
```bash
./target/debug/hakorune --backend vm \
  lang/src/mir/builder/loop_recipe/emit_loop_recipe_wire.hako \
  > src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_v1.json
./target/debug/hakorune --backend vm \
  lang/src/mir/builder/loop_recipe/emit_m8a_recurrence_wire.hako \
  > src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_m8a_v1.json
./target/debug/hakorune --backend vm \
  lang/src/mir/builder/loop_recipe/emit_m8c_scans_wire.hako \
  > src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_m8c_v2.json
./target/debug/hakorune --backend vm \
  lang/src/mir/builder/loop_recipe/emit_m8d_loopcond_wire.hako \
  > src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_m8d_v1.json
./target/debug/hakorune --backend vm \
  lang/src/mir/builder/loop_recipe/emit_direct_accum_wire.hako \
  > src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_accum_direct_v1.json
./target/debug/hakorune --backend vm \
  lang/src/mir/builder/loop_recipe/emit_nested_predicate_wire.hako \
  > src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_nested_v1.json
./target/debug/hakorune --backend vm \
  lang/src/mir/builder/loop_recipe/emit_loop_true_wire.hako \
  > src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_loop_true_v1.json
```
Each emission is one compact JSON line; the harness compares
`decode_and_verify` + `normalize_*` products, not raw formatting.
