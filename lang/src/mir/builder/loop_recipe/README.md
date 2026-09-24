# lang/src/mir/builder/loop_recipe — portable LoopRecipe wire (S7A)

Owner boundary
- Owns the `.hako` side of the `LoopRecipeArtifactV1` wire:
  `{ schema_version, provenance, source_binding, recipe }`.
- `emit_loop_recipe_wire.hako` (S7A) emits one fixed minimal
  `direct_accum_v1` artifact; `emit_m8a_recurrence_wire.hako` (S7B1)
  emits the canonical M8A `variable_accum_recurrence_v1` artifact for
  the bounded profile `loop(i < 4) { acc = acc + i; i = i + 1 }` at
  root body item index 2; `emit_m8b_break_wire.hako` (S7B2) emits the
  canonical M8B `variable_accum_break_v1` artifact for the bounded
  profile `loop(i < 10) { if(i == 5) { sum += 10; break }; sum += 1;
  i += 1 }` at root body item index 2 — covering the `If` item, the
  `Exit` item, and a non-empty `exits` table with one break row.
  Each entry assembles its artifact from named
  string-fragment locals in fixed serde field order and prints it as
  one compact JSON line to stdout. Field names and tagged kinds mirror
  `src/mir/loop_recipe_contract/schema.rs` exactly.
- The Rust decode/verify/normalize owner remains
  `LoopRecipeNormalizerV1` (`src/mir/loop_recipe_contract/normalize.rs`);
  the parity harness lives in
  `src/mir/loop_recipe_contract/wire_parity_tests.rs` and consumes the
  checked-in emissions at
  `src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_v1.json`,
  `hako_loop_recipe_wire_m8a_v1.json`, and
  `hako_loop_recipe_wire_m8b_v1.json`. The M8A arm compares against
  the artifact the real Rust producer
  `produce_variable_accum_recurrence_recipe_v1` yields for the same
  bounded source profile; the M8B arm compares against
  `produce_variable_accum_break_recipe_v1` likewise.

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
- No `LoopRecipeArtifactV2` wiring (V2 belongs to the S7B ScanWithInit
  cohort row).

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
  lang/src/mir/builder/loop_recipe/emit_m8b_break_wire.hako \
  > src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_m8b_v1.json
```
Each emission is one compact JSON line; the harness compares
`decode_and_verify` + `normalize_*` products, not raw formatting.
