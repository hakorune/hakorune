# Stage1 Bridge Stub Emit

Scope: stub emit helpers under `src/runner/stage1_bridge/stub_emit/`.

## Sections

- `../stub_emit.rs`: facade (`run_capture(...)`, child timeout/exit handling, mode selection) with helper-local timeout/nonzero/parse-write orchestration
- `parse.rs`: stdout parse / validation for MIR(JSON) and Program(JSON) via helper-local line extraction and parse-error formatting
- `writeback.rs`: writeback policy, optional MIR dump, file/stdout emission via helper-local output routes

## Forbidden

- child spawn / timeout policy outside `../stub_emit.rs`
- duplicate output-path resolution here (use `../emit_paths.rs`)
- route planning / route execution here
