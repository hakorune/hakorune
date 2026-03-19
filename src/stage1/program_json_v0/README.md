# Stage1 Program JSON v0 Layout

Scope: `src/stage1/program_json_v0.rs` façade と、その配下の owner-local modules。

## Responsibility Split

- `src/stage1/program_json_v0.rs`
  - façade entry
  - public authority / compat / build-surrogate API surface
  - delegates current source authority to `authority.rs`
  - keeps future-retire bridge error-prefix wrapping as a tiny crate-local facade leaf
- `src/stage1/program_json_v0/routing.rs`
  - source-shape SSOT
  - build-surrogate route selection SSOT
  - build-surrogate emission helper
- `src/stage1/program_json_v0/authority.rs`
  - current strict source authority owner
  - strict/relaxed source parse orchestration
- `src/stage1/program_json_v0/extract.rs`
  - source-text observation only
  - `using` import collection
  - `static box Main` / helper method extraction
  - dev-local alias sugar preexpand + detection
- `src/stage1/program_json_v0/lowering.rs`
  - AST subset -> Program(JSON v0) lowering
  - helper defs serialization

## Invariants

- strict parse stays crate-local (`source_to_program_json_v0_strict(...)`)
- relaxed compat keep stays owner-local (`source_to_program_json_v0_relaxed(...)`)
- future-retire `stage1_bridge` uses crate-local facade helper `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)`
- current authority source route should prefer `emit_program_json_v0_for_strict_authority_source(...)`
- the real source-authority owner is `authority.rs`; `program_json_v0.rs` is no longer the owner of the parse/lower chain itself
- future-retire bridge error-prefix wrapping is no longer mixed into `authority.rs`; it stays as a tiny facade leaf in `program_json_v0.rs`
- build-box surrogate callers use `emit_program_json_v0_for_current_stage1_build_box_mode(...)`
- current-mode build surrogate follows `crate::config::env::stage1::emit_program_json()` as env SSOT
- build-route selection (`select_program_json_v0_build_route(...)`) stays routing-local inside `routing.rs`
- build-route enum (`ProgramJsonV0BuildRoute`) stays routing-local; cross-crate callers do not read route state directly
- build-box route emission also stays routing-local; the façade only freeze-wraps owner-local `emit_program_json_v0_for_stage1_build_box(...)`
- build payload text is the only cross-crate build-surrogate result; route trace stays owner-local
- source-shape enum/info stay crate-local; cross-crate authority callers use `emit_program_json_v0_for_strict_authority_source(...)` instead of reading source-shape objects directly
- cross-crate callers use owner-1 helpers for fail-fast only; route trace stays inside `program_json_v0`
- source-shape / build-route policy lives in `routing.rs`, not in callers
- current-mode env interpretation lives in `crate::config::env::stage1`, not in callers
- `.hako` compat quarantine (`stage1-env-mir-program`) is out of scope here

## Allowed Cross-Crate Surface

- `emit_program_json_v0_for_strict_authority_source(...)`
- `emit_program_json_v0_for_current_stage1_build_box_mode(...)`

## Operation Card

- authority source caller => `emit_program_json_v0_for_strict_authority_source(...)`
- explicit compat keep => owner-local `source_to_program_json_v0_relaxed(...)` only; do not cross crate boundary
- build surrogate caller => `emit_program_json_v0_for_current_stage1_build_box_mode(...)`
- anything else => keep owner-local; do not add a new cross-crate entrypoint

## Forbidden Cross-Crate Calls

- `source_to_program_json_v0_relaxed(...)` across crate boundaries
- `source_to_program_json_v0_strict(...)` across crate boundaries
- direct `source_to_program_json_v0_strict(...)` calls from `src/runner/stage1_bridge/**`
- owner-local `emit_program_json_v0_for_stage1_build_box(...)`
- routing-local build-box emission helpers
- owner-local build-emission read model (`trace_summary()` / `into_program_json()`)
- `select_program_json_v0_build_route(...)`
- `ProgramJsonV0BuildRoute`
- source-shape enum/info internals
- parse/lower helpers and module-local orchestration

## Non-Goals

- do not add new authority routes here
- do not move shell/probe compat logic into this module cluster
- do not reopen `Program(JSON v0)` retirement work from this directory
