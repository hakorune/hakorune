# JSON Artifact Loader

Scope: runner-side JSON artifact classification and `MirModule` loading in `src/runner/json_artifact/`.

Public route-family SSOT: `docs/development/current/main/design/json-v0-route-map-ssot.md`

## Responsibility Split

- `mir_loader.rs`
  - mainline `MIR(JSON)` intake
  - parsed top-level v1 bridge decides absent, supported, or unsupported
    `schema_version`; v0 is selected only when the key is absent (including
    escaped JSON key spellings)
  - no Program(JSON v0) import-bundle behavior
- `program_json_v0_loader.rs`
  - compat-only `Program(JSON v0)` intake
  - bridge lowering
  - used-import alias extraction
  - on-demand import compile + merge
  - `NYASH_JSON_V0_IMPORT_TRACE`
- `mod.rs`
  - artifact-family convergence
  - `load artifact -> MirModule`
  - shared boundary between CLI entrypoints and terminal execution

## Invariants

- `MIR(JSON)` is the mainline artifact family.
- `Program(JSON v0)` is compat/bootstrap-only and a retire target.
- `core_executor` is the terminal execution owner after a `MirModule` exists.
- `--mir-json-file` must stay on the mainline MIR loader.
- A declared v1 parse error is terminal; it must not be retried through v0.
- Schema presence is decided from parsed JSON, never from a raw substring
  search; an explicit unsupported schema is terminal before v0 parsing.
- `--json-file` is a compat umbrella intake; only the compat loader may own Program(JSON v0)-specific merge/trace behavior.
- do not reintroduce Program(JSON v0) import-bundle policy into `core_executor`.

## Non-Goals

- do not remove CLI flags here
- do not widen Program(JSON v0) semantics
- do not merge mainline MIR intake and compat Program(JSON v0) intake back into one loader
