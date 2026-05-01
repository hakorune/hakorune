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
- `--hako-emit-program-json`
  - retired hako-prefixed Program(JSON v0) public alias
  - removed in P6; do not use as a live compatibility entry
  - explicit Program(JSON) work stays on raw compat flags or dedicated probes
- `--json-file <program.json>`
  - compat umbrella intake
  - accepts mixed JSON artifacts, not just Program(JSON v0)
  - route path: `runner::pipe_io::try_run_json_v0_pipe` -> `core_executor::execute_json_artifact` -> `json_artifact::load_json_artifact_to_module` -> `program_json_v0_loader::load_program_json_v0_to_module` -> `compile_program_json_v0_imports_bundle`
  - this is the route that owns the `NYASH_JSON_V0_IMPORT_TRACE` observation
- `--program-json-to-mir <out> --json-file <program.json>`
  - retired in phase-29ci P16
  - use `env.mirbuilder.emit` / `tools/selfhost/lib/program_json_mir_bridge.sh`
    for explicit Program(JSON)->MIR helper work
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

## Compat Capsule Model

A `Program(JSON v0)` compat capsule is an explicit owner that keeps a bounded
compatibility seam alive while the mainline stays MIR-first.

Capsule invariants:

- named entrypoint in docs
- clear input/output boundary
- no `selfhost_build.sh` facade shortcut
- no promotion to mainline proof
- delete or archive only after MIR-first replacement exists

Current capsule classes:

| Capsule | Entrypoints | Boundary | Reading |
| --- | --- | --- | --- |
| Stage-B artifact diagnostic | `tools/dev/program_json_v0/stageb_artifact_probe.sh`, `tools/lib/program_json_v0_compat.sh` | source `.hako` -> Program(JSON v0) file | explicit artifact capture only |
| Program(JSON)->MIR bridge | `tools/selfhost/lib/program_json_mir_bridge.sh`, `tools/selfhost_exe_stageb.sh` (`stageb-delegate`), `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` | Program(JSON v0) -> MIR(JSON) -> optional ny-llvmc proof | compat conversion capsule, not primary proof |
| Stage1 contract | `tools/selfhost/lib/stage1_contract.sh`, `tools/selfhost/compat/run_stage1_cli.sh` | Stage1 CLI env contract -> Program/MIR compatibility payloads | explicit contract pin |
| Fixture contract | `tools/smokes/v2/lib/stageb_helpers.sh`, phase29bq JoinIR/MirBuilder pins, Stage-B/Core fixture smokes | Program(JSON v0) fixture -> smoke contract assertions | fixture-only compatibility |

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
    - keep `--hako-emit-mir-json` as the Stage-1 MIR launcher for `stage1-env-mir-source`
    - `--hako-emit-program-json` is retired as the first duplicate public Program(JSON) alias
   - demote `--json-file` / `--emit-program-json-v0` to explicit compat guidance
   - current explicit keepers that still block hard delete are:
     - `tools/dev/program_json_v0/stageb_artifact_probe.sh`
     - `tools/lib/program_json_v0_compat.sh`
     - `tools/selfhost/lib/program_json_mir_bridge.sh`
     - `tools/selfhost_exe_stageb.sh` (`stageb-delegate` bridge capsule)
     - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
     - `tools/selfhost/lib/stage1_contract.sh`
     - `tools/selfhost/compat/run_stage1_cli.sh`
     - `tools/smokes/v2/lib/stageb_helpers.sh`
     - Rust/public delete-last surface:
       `src/runtime/deprecations.rs`, `src/stage1/program_json_v0*`,
       `src/runner/stage1_bridge/**`
    - retire `--program-json-to-mir` after caller inventory reaches zero (landed in P16)
    - hard delete only after the compat caller inventory reaches zero

## Bridge Caller Ownership Split

`tools/selfhost/lib/program_json_mir_bridge.sh` remains live while the callers
below still need explicit Program(JSON v0) -> MIR(JSON) conversion.

| Caller | Ownership | Delete posture |
| --- | --- | --- |
| `tools/selfhost_exe_stageb.sh` default or `HAKORUNE_STAGE1_EMIT_ROUTE=direct` | MIR-first route | not a Program(JSON v0) bridge blocker |
| `tools/selfhost_exe_stageb.sh` with explicit `HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate` | bridge compat capsule | P104 replacement proof for standalone bridge-to-EXE probe; keep explicit until bridge archive coverage is complete |
| `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` | Stage2 bootstrap PHI/LLVM verification proof | P105 guards against the reduced run-only `stage1-cli` artifact; P106 keeps this as a bridge capsule until `stage1_contract_exec_mode ... emit-mir`, plan-backed `env.get/1`, and MIR dominance are green |

Indirect callers of `tools/selfhost_exe_stageb.sh` are owned by the selected
emit route. They are not separate bridge-helper callers unless they call
`program_json_mir_bridge_emit()` directly.

## Stage1 Contract Caller Ownership Split

`tools/selfhost/lib/stage1_contract.sh` is a live shell contract owner, not a
dead Program(JSON v0) helper. It remains a delete-last blocker while the caller
classes below are active.

| Caller class | Examples | Reading |
| --- | --- | --- |
| build/bootstrap | `tools/selfhost/mainline/build_stage1.sh` | capability probes and direct emit checks |
| identity/proof | `tools/selfhost/lib/identity_routes.sh`, `tools/selfhost_identity_check.sh` | exact route validation |
| compatibility wrapper | `tools/selfhost/compat/run_stage1_cli.sh`, `tools/selfhost/run_stage1_cli.sh` | wrapper/shim around the Stage1 shell contract |
| dev/probe | phase29ch/phase29cg probes | diagnostics; still direct contract callers |
| smoke | `phase29bq_selfhost_stage1_contract_smoke_vm.sh` | contract pin requiring a prebuilt stage1-cli artifact |

`tools/selfhost/run_stage1_cli.sh` is only a top-level shim to the compat
wrapper. Count it under `tools/selfhost/compat/run_stage1_cli.sh`, not as a
separate Program(JSON v0) keeper.

## Fixture Caller Ownership Split

`tools/smokes/v2/lib/stageb_helpers.sh` is broader than the phase29bq
MirBuilder fixture pins. It owns Stage-B compiler fixture helpers for current
smoke lanes and therefore keeps Program(JSON v0) fixture support live.

| Helper surface | Current callers | Reading |
| --- | --- | --- |
| `stageb_emit_program_json_v0_fixture()` | `phase29bq_hako_program_json_contract_pin_vm.sh`, `phase29bq_hako_mirbuilder_*` | direct Stage-0 Program(JSON v0) fixture emit for .hako MirBuilder pins |
| `stageb_compile_to_json*()` | `integration/stageb/*`, `integration/core_direct/*`, budget/core quick fixture smokes | Stage-B compiler stdout -> Program(JSON v0) fixture capture |
| `stageb_json_nonempty()` / `stageb_gatec_expect_rc()` | Stage-B fixture execution smokes | Program(JSON v0) fixture assertions |
| `stageb_export_vm_compile_env()` | bundle/require negative smokes | shared Stage-B compiler env contract |

P65 pruned the unused Rust MIR fallback helper from `stageb_helpers.sh`; do not
reintroduce a fallback route there without an active caller and a named owner.

## Rust Public Delete-Last Surface Split

The Rust/public delete-last bucket has two separate compat surfaces.

| Surface | Owner | Reading |
| --- | --- | --- |
| `--emit-program-json-v0` parse/deprecation | `src/cli/args.rs`, `src/runtime/deprecations.rs` | public compat emit flag, keep until shell/tool emit callers reach zero |
| `--emit-program-json-v0` execution | `src/runner/emit.rs`, `src/runner/stage1_bridge/program_json_entry/`, `src/runner/stage1_bridge/program_json/` | bridge-local file read -> Program(JSON v0) payload -> writeback |
| Program(JSON v0) payload authority | `src/stage1/program_json_v0.rs`, `src/stage1/program_json_v0/**` | shared Rust owner for strict authority source, `host_providers/mir_builder` handoff, stage1 bridge payload, and BuildBox/module-string bootstrap support |
| `--json-file` Program(JSON v0) intake | `src/runner/json_artifact/program_json_v0_loader.rs` | compat umbrella intake; separate from the emit flag |
| Program(JSON v0) fixture env transport | `src/main.rs` (`HAKO_PROGRAM_JSON_FILE` -> `HAKO_PROGRAM_JSON`) | `.hako` MirBuilder fixture handoff; separate from CLI emit and `--json-file` intake |

Do not delete `src/stage1/program_json_v0*` just because the public emit flag
is retired; it also serves non-CLI Rust callers. Do not delete the
Program(JSON v0) loader until `--json-file` Program(JSON v0) intake callers are
also replaced or archived.

## Caller Reduction Rule

- if a caller already feeds `MIR(JSON)` directly, it should use `--mir-json-file`
- `--json-file` should remain only for:
  - Program(JSON v0) intake
  - compat loader probes
  - v1/v0 fallback or downconvert coverage
  - import-bundle / bridge diagnostics
- compat capsule callers must not be reused as primary proof gates; pair them
  with an explicit MIR-first proof when validating mainline behavior

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
- do not treat the remaining explicit probe/fixture keepers as proof that public
  compat deletion is ready
