MirBuilder Internals — Toggle Aggregation

- Use `builder_config_box.hako` (`hako.mir.builder.internal.builder_config`) to read all `HAKO_MIR_BUILDER_*` toggles.
- Use `registry_authority_box.hako` (`hako.mir.builder.internal.registry_authority`) as the dedicated owner for the normal registry-first `Program(JSON v0) -> MIR(JSON)` authority block.
- Do not call `env.get` directly in lowers; prefer helper methods like:
  - `trace_enabled()`, `debug_enabled()`
  - `internal_on()`, `delegate_on()`, `selfhost_no_delegate_on()`, `registry_on()`, `registry_only()`
  - `loop_jsonfrag_on()`, `jsonfrag_normalize_on()`, `skip_loops_on()`
  - `loop_adapter_return_mode()` → `string` (default) or `map`

Notes
- JsonFrag emission is kept default OFF and used for structural observation only. Semantics are prioritized by the normal path.
- `MirBuilderBox.hako` should keep route sequencing and compat tails; if the normal registry-first mainline needs to grow, extend `registry_authority_box.hako` before widening the outer box again.
