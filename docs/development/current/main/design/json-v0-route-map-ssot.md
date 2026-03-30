# JSON Artifact Route Map SSOT

Status: current
Scope: `Program(JSON v0)` compat routes and `MIR(JSON)` mainline routes
Related:
- docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md

## Artifact Families

- `MIR(JSON)`
  - mainline artifact family
  - preferred public emit/intake surface
  - current reading: v1-preferred, v0-read-fallback
- `Program(JSON v0)`
  - compat/bootstrap-only artifact family
  - retire target
  - keep only as explicit bridge / convert / umbrella-intake residue
  - file-level placement / authority cleanup is tracked separately in
    `selfhost-authority-facade-compat-inventory-ssot.md`

## CLI Route Table

- `--emit-program-json-v0`
  - stage1 bridge emit route
  - source `.hako` -> Program(JSON v0)
  - compat-only / future-retire public surface
  - does **not** exercise the import-bundle execute path
- `--json-file <program.json>`
  - compat umbrella intake
  - accepts mixed JSON artifacts, not just Program(JSON v0)
  - route path: `runner::pipe_io::try_run_json_v0_pipe` -> `core_executor::execute_json_artifact` -> `json_artifact::load_json_artifact_to_module` -> `program_json_v0_loader::load_program_json_v0_to_module` -> `compile_program_json_v0_imports_bundle`
  - this is the route that owns the `NYASH_JSON_V0_IMPORT_TRACE` observation
- `--program-json-to-mir <out> --json-file <program.json>`
  - explicit compat bridge route
  - convert Program(JSON v0) to MIR(JSON) and write a file
  - keep lane, not the primary mainline emit path
- `--mir-json-file <mir.json>`
  - direct MIR execute route
  - mainline intake route

## Internal Convergence Model

- target internal model is `load artifact -> MirModule -> execute`
- `MIR(JSON)` loader should be the mainline intake owner
- `Program(JSON v0)` loader should own all compat-only behavior
  - import-bundle alias collection
  - on-demand import compile + merge
  - bridge-local lowering residue
- `core_executor::execute_json_artifact(...)` is the terminal route owner once the artifact has been lowered to `MirModule`

## Migration Order

1. docs lock
   - fix artifact-family reading and CLI route names
   - status: landed
2. internal API split
   - separate `load_mir_json(...)`, `load_program_json_v0(...)`, `load_json_artifact_to_module(...)`, and `execute_json_artifact(...)`
   - status: landed
3. compat isolation
   - keep Program(JSON v0) import-bundle behavior entirely behind the compat loader
   - status: landed
4. archive/delete readiness
   - sync `phase-29ci` / `phase-29cj` delete order around the compat loader boundary
   - rewrite direct MIR file callers away from `--json-file`
   - archive-ready monitor/probe/docs bucket is archive-only evidence
   - mixed route probe helper split is explicit now; keep it inside compat-loader routing, not as a separate cleanup bucket
5. public-surface cleanup
   - keep public mainline docs on `--emit-mir-json` / `--mir-json-file`
   - demote `--json-file` / `--program-json-to-mir` / `--emit-program-json-v0` to explicit compat guidance
   - hard delete only after the compat caller inventory reaches zero

## Caller Reduction Rule

- if a caller already feeds `MIR(JSON)` directly, it should use `--mir-json-file`
- `--json-file` should remain only for:
  - Program(JSON v0) intake
  - compat loader probes
  - v1/v0 fallback or downconvert coverage
  - import-bundle / bridge diagnostics

## Trace Ladder

- `NYASH_JSON_V0_IMPORT_TRACE=1`
  - emits stable one-line summary traces at info level
  - summary lines show route identity and high-level result:
    - `phase=enter`
    - `phase=skip`
    - `phase=merge.done`
    - `phase=fail`
- `NYASH_RING0_LOG_LEVEL=DEBUG`
  - additionally emits detailed bundle traces
  - detail lines show guard/restore and merge boundaries:
    - `phase=merge.begin`
    - `phase=guard.set`
    - `phase=restore`
- default runs should stay readable; deep bundle internals are intentionally debug-only

## Current Observation Recipe

- summary-only:
  - `NYASH_JSON_V0_IMPORT_TRACE=1 target/release/hakorune --json-file <program.json>`
- deep debug:
  - `NYASH_JSON_V0_IMPORT_TRACE=1 NYASH_RING0_LOG_LEVEL=DEBUG target/release/hakorune --json-file <program.json>`

## Non-Goals

- do not reopen `Program(JSON v0)` as a mainline artifact family
- do not jump directly to CLI removal or hard delete before `phase-29ci` / `phase-29cj` caller inventory is empty
