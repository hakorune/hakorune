MirBuilder Internals — Toggle Aggregation

- Use `builder_config_box.hako` (`hako.mir.builder.internal.builder_config`) to read all `HAKO_MIR_BUILDER_*` toggles.
- Use `program_json_input_contract_box.hako` (`hako.mir.builder.internal.program_json_input_contract`) as the dedicated owner for Program(JSON) null/header validation and text coercion on the outer entry contract.
- Use `func_defs_gate_box.hako` (`hako.mir.builder.internal.func_defs_gate`) as the dedicated owner for the func-def pre-lowering gate (`toggle -> coerce -> FuncLoweringBox.lower_func_defs(...)`).
- Use `loop_force_route_box.hako` (`hako.mir.builder.internal.loop_force_route`) as the dedicated owner for the dev-only loop-force gate, minimal loop MIR assembly, and finalize-chain handoff.
- Use `unsupported_tail_box.hako` (`hako.mir.builder.internal.unsupported_tail`) as the dedicated owner for ternary detect, unsupported-reason selection, and fail-tag handoff.
- Use `registry_authority_box.hako` (`hako.mir.builder.internal.registry_authority`) as the dedicated owner for the normal registry-first `Program(JSON v0) -> MIR(JSON)` authority block.
- Use `fallback_authority_box.hako` (`hako.mir.builder.internal.fallback_authority`) as the dedicated owner for the non-registry/internal fallback chain that still belongs to `.hako` authority.
- Use `delegate_provider_box.hako` (`hako.mir.builder.internal.delegate_provider`) as the dedicated owner for the selfhost builder delegate gate and provider emit call.
- Use `delegate_finalize_box.hako` (`hako.mir.builder.internal.delegate_finalize`) as the dedicated owner for delegate-side `user_box_decls` MIR finalize and its handoff into the shared outer normalization chain.
- Use `finalize_chain_box.hako` (`hako.mir.builder.internal.finalize_chain`) as the dedicated owner for the shared outer finalize order (`inject funcs -> methodize -> normalize`) and its stable fail tags.
- Do not call `env.get` directly in lowers; prefer helper methods like:
  - `trace_enabled()`, `debug_enabled()`
  - `internal_on()`, `delegate_on()`, `selfhost_no_delegate_on()`, `registry_on()`, `funcs_on()`, `registry_only()`
  - `loop_jsonfrag_on()`, `jsonfrag_normalize_on()`, `skip_loops_on()`
  - `loop_adapter_return_mode()` → `string` (default) or `map`

Notes
- JsonFrag emission is kept default OFF and used for structural observation only. Semantics are prioritized by the normal path.
- `MirBuilderBox.hako` should keep checked handoff + route sequencing; Program(JSON) entry validation now lives in `BuilderProgramJsonInputContractBox`, generic unsupported/no-match decision now lives in `BuilderUnsupportedTailBox`, and source-entry compat now lives in `MirBuilderSourceCompatBox`.
- `MirBuilderSourceCompatBox` should keep the source-entry compat seam instead of widening `MirBuilderBox` again.
- If the outer Program(JSON) input contract needs to grow, extend `program_json_input_contract_box.hako` before widening the outer box again.
- If the dev-only loop-force route needs to grow, extend `loop_force_route_box.hako` before widening the outer box again.
- If the generic unsupported/no-match tail needs to grow, extend `unsupported_tail_box.hako` before widening the outer box again.
- If the normal registry-first mainline needs to grow, extend `registry_authority_box.hako` before widening the outer box again.
- If the non-registry/internal fallback chain needs to grow, extend `fallback_authority_box.hako` before widening the outer box again.
- If the delegate/provider compat lane needs to grow, extend `delegate_provider_box.hako` before widening the outer box again.
- If the delegate-side local finalize needs to grow, extend `delegate_finalize_box.hako` before widening the outer box again.
- If the shared outer finalize chain needs to grow, extend `finalize_chain_box.hako` before widening the outer box again.
- If the func-def pre-lowering gate needs to grow, extend `func_defs_gate_box.hako` before widening the outer box again.
