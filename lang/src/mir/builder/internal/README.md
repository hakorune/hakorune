MirBuilder Internals — Toggle Aggregation

- Use `builder_config_box.hako` (`hako.mir.builder.internal.builder_config`) to read all `HAKO_MIR_BUILDER_*` toggles.
- Use `registry_authority_box.hako` (`hako.mir.builder.internal.registry_authority`) as the dedicated owner for the normal registry-first `Program(JSON v0) -> MIR(JSON)` authority block.
- Use `fallback_authority_box.hako` (`hako.mir.builder.internal.fallback_authority`) as the dedicated owner for the non-registry/internal fallback chain that still belongs to `.hako` authority.
- Use `delegate_provider_box.hako` (`hako.mir.builder.internal.delegate_provider`) as the dedicated owner for the selfhost builder delegate gate and provider emit call.
- Use `delegate_finalize_box.hako` (`hako.mir.builder.internal.delegate_finalize`) as the dedicated owner for delegate-side `user_box_decls` MIR finalize and its handoff into the shared outer normalization chain.
- Use `finalize_chain_box.hako` (`hako.mir.builder.internal.finalize_chain`) as the dedicated owner for the shared outer finalize order (`inject funcs -> methodize -> normalize`) and its stable fail tags.
- Do not call `env.get` directly in lowers; prefer helper methods like:
  - `trace_enabled()`, `debug_enabled()`
  - `internal_on()`, `delegate_on()`, `selfhost_no_delegate_on()`, `registry_on()`, `registry_only()`
  - `loop_jsonfrag_on()`, `jsonfrag_normalize_on()`, `skip_loops_on()`
  - `loop_adapter_return_mode()` → `string` (default) or `map`

Notes
- JsonFrag emission is kept default OFF and used for structural observation only. Semantics are prioritized by the normal path.
- `MirBuilderBox.hako` should keep route sequencing and generic unsupported/no-match decision; source-entry compat now lives in `MirBuilderSourceCompatBox`.
- `MirBuilderSourceCompatBox` should keep the source-entry compat seam instead of widening `MirBuilderBox` again.
- If the normal registry-first mainline needs to grow, extend `registry_authority_box.hako` before widening the outer box again.
- If the non-registry/internal fallback chain needs to grow, extend `fallback_authority_box.hako` before widening the outer box again.
- If the delegate/provider compat lane needs to grow, extend `delegate_provider_box.hako` before widening the outer box again.
- If the delegate-side local finalize needs to grow, extend `delegate_finalize_box.hako` before widening the outer box again.
- If the shared outer finalize chain needs to grow, extend `finalize_chain_box.hako` before widening the outer box again.
