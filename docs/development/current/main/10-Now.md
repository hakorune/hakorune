Status: SSOT mirror
Date: 2026-06-20
Scope: one-screen current dashboard. Do not store landed history here.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Now

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`

## Active Blocker

```text
CREAT-SUBSET-PILOT-SELECTION-001
```

The scan-methods focused timeout slice is closed by 296x-1304, the touched
string-corridor regression is closed by 296x-1305, PHI input rematerialization
identity is closed by 296x-1306, and string-corridor stable-length hint fallback
retirement is closed by 296x-1307. The rust-subset app-front smoke reopen is
closed by 296x-1308; `JsonParser.parse_array/0` is owned by
loop_true_break_continue via the recipe-first ExitAllowed path. A dedicated
syn-adapter smoke wrapper is closed by 296x-1309. RustSubset `Index`
expression transport is closed by 296x-1310. Loop `break` / `continue` is
fixture-guarded as an explicit Unsupported handoff by 296x-1311. Generic
function skeleton transport is closed by 296x-1312. Unsupported trait handoff
hardening is closed by 296x-1313. Rust-subset smoke fixture handling is
table-driven by 296x-1314. Crate/multi-file handoff boundaries are inventoried
by 296x-1315. Module schema validation parity is closed by 296x-1316.
RustSubset path/name normalization is closed by 296x-1317 without adding
`.hako` syntax. The crate manifest v0 contract is accepted by 296x-1318. The
synthetic multi-module adapter probe is closed by 296x-1319. Crate handoff MIR
acceptance is closed by 296x-1320. OrderedMapBox is documented as a `.hako`
library deterministic-map boundary by 296x-1321. OrderedMapBox v0 and its
focused EXE/AOT smoke are closed by 296x-1322. The focused BindingContext-style
OrderedMapBox probe is closed by 296x-1323. Constructor lifecycle is guarded
separately by 296x-1324 without moving meaningful birth logic into field
initializers. Field-initializer library route probing is closed by 296x-1325:
same-file initializer routes are green. Imported static factory field
initializer routes are fixed by 296x-1326: App-mode import bundle lowering
now defers non-Main static methods until instance box constructors are lowered,
so `new Box(args)` can inject `Box.birth/arity`. The active blocker returns to
creat subset pilot selection.

## Next

1. Inventory candidate creat/source modules without adding new `.hako` syntax.
2. Count unsupported RustSubset families and graph blockers.
3. Select a 2-3 module pilot slice.
4. Keep Rust parser/crate graph discovery in the external adapter boundary.
5. Do not reopen fastpath or constructor lifecycle work without a new blocker.
6. Run:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

7. Update current pointers when the implementation row closes.

## Recently Closed

- `IMPORTED-FIELD-INIT-BIRTH-MERGE-FIX-001`
  - imported static factory field-initializer routes preserve `new -> birth`
  - App-mode static method lowering is deferred until instance constructors are lowered
  - focused field-initializer, OrderedMap, and constructor-lifecycle smokes are green
- `STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001`
  - semantic string-corridor benchmark contract
  - read-only `MethodCallOperandView`
  - no benchmark/source/function-name branches
- `PHI-INPUT-REMAT-OPERAND-MEMO-001`
  - predecessor-local rematerialization memo
  - receiver-prefixed substring remat identity preserved
  - accepted remat shapes unchanged
- `STRING-CORRIDOR-STABLE-LENGTH-HINT-FALLBACK-RETIRE-001`
  - string-corridor planning reads typed stable-length relations
  - diagnostic stable-length hints remain output-only
  - hint parsing as correctness evidence retired
- `RUST-SUBSET-APP-FRONT-LOOP-TRUE-BREAK-CONTINUE-SMOKE-CLOSEOUT-001`
  - `parse_array`-class loop(true) shapes use loop_true_break_continue
  - effectful continue-prelude branches are accepted through ExitAllowed
  - full rust-subset app-front smoke is green
- `RUST-SUBSET-SYN-ADAPTER-SMOKE-ENTRY-001`
  - dedicated wrapper for `RUST_SUBSET_RUN_ADAPTER=1`
  - converter core remains input-route agnostic
- `RUST-SUBSET-SYN-ADAPTER-INDEX-EXPRESSION-001`
  - Rust `xs[i]` lowers to RustSubset `Index`
  - Python reference, syn adapter, and `.hako` EXE/AOT parity are green
  - Array storage/bounds semantics remain compiler/runtime-owned
- `RUST-SUBSET-SYN-ADAPTER-BREAK-CONTINUE-UNSUPPORTED-HANDOFF-001`
  - loop `break` / `continue` lowers to an explicit Unsupported handoff
  - compiler Recipe/CorePlan semantics are unchanged
  - Python reference, syn adapter, and `.hako` EXE/AOT parity are green
- `RUST-SUBSET-SYN-ADAPTER-GENERIC-FUNCTION-SKELETON-001`
  - generic Rust function type spellings are preserved in skeleton output
  - no type-parameter model or generic semantics are claimed
  - Python reference, syn adapter, and `.hako` EXE/AOT parity are green
- `RUST-SUBSET-SYN-ADAPTER-UNSUPPORTED-TRAIT-HANDOFF-HARDENING-001`
  - unsupported trait handoff now has RustSubset JSON and `.hako` EXE/AOT parity
  - no trait semantics or new schema node are introduced
  - Python reference, syn adapter, and `.hako` EXE/AOT parity are green
- `RUST-SUBSET-SMOKE-FIXTURE-TABLE-REFACTOR-001`
  - smoke fixture handling is table-driven
  - behavior, converter core, schema, and input routes are unchanged
  - Python reference, smoke, and adapter smoke are green
- `RUST-SUBSET-CRATE-HANDOFF-INVENTORY-001`
  - current syn adapter is documented as single-file only
  - crate handoff is scoped toward manifest plus per-module artifacts
  - crate graph discovery remains external to `converter_core.hako`
  - path/name normalization is identified as P0 before crate pilot
- `RUST-SUBSET-MODULE-SCHEMA-VALIDATION-PARITY-001`
  - module schema validation boundaries match the Python reference where active
  - schema_version/root kind and unknown item kinds fail fast in `.hako`
  - Python reference also fail-fasts unknown statement/expression kinds
  - known Unsupported nodes still emit TODO comments
- `RUST-SUBSET-PATH-NAME-NORMALIZATION-001`
  - syn adapter owns source_name/emitted_name and Hako-safe emitted identifiers
  - tuple fields normalize to `_0`, `_1`; reserved names normalize to `rust_*`
  - converter prints emitted names and does not resolve Rust paths
- `RUST-SUBSET-CRATE-MANIFEST-V0-001`
  - crate-wide handoff is a manifest plus per-module RustSubsetModule artifacts
  - manifest is a transport index, not a semantic AST or name resolver
  - multi-file adapter output is still a follow-up row
- `RUST-SUBSET-SYN-ADAPTER-MULTI-MODULE-PROBE-001`
  - synthetic mini-crate emits crate-manifest.json plus three module artifacts
  - converter_core manifest/FileBox ownership remains 0
  - adapter smoke verifies the generated artifact set
- `RUST-SUBSET-CRATE-HANDOFF-MIR-ACCEPTANCE-001`
  - `.hako` crate wrapper validates the synthetic manifest and module ids
  - module artifacts are read through FileBox without converter_core ownership
  - generated skeleton MIR emit is smoke-guarded
- `HAKO-ORDERED-MAP-BOX-SSOT-001`
  - OrderedMapBox v0 is documented as a `.hako` library utility
  - v0 owns deterministic String-key order only
  - `MapBox`, ring0, ring1 provider registration, and MirBuilder remain unchanged
- `HAKO-ORDERED-MAP-BOX-V0-001`
  - OrderedMapBox v0 is implemented in `apps/lib/collections/ordered_map.hako`
  - focused EXE/AOT smoke verifies deterministic order and update behavior
  - `MapBox`, ring0, ring1 provider registration, and MirBuilder remain unchanged
- `MIRBUILDER-BINDING-CONTEXT-ORDERED-MAP-PROBE-001`
  - focused EXE/AOT probe verifies OrderedMapBox as a BindingContext-style name-to-id map
  - deterministic binding snapshot, duplicate update, and missing lookup are fixed
  - MirBuilder, BindingContext, MapBox, ring0, and ring1 remain unchanged
- `CONSTRUCTOR-LIFECYCLE-FIELD-INIT-BIRTH-PROBE-001`
  - constructor lifecycle is guarded separately from OrderedMapBox API work
  - birth(value), constructor args, and per-instance ArrayBox defaults are smoke-checked
  - OrderedMapBox keeps explicit v0 create-time initialization as a route-compatibility choice
- `FIELD-INITIALIZER-LIBRARY-ROUTE-PROBE-001`
  - same-file direct new, static factory new, and birth(value) field-initializer routes are green
  - imported static factory probes stop at unsupported_pure_shape before runtime
  - MIR route metadata reports `unknown_global_callee` for the focused app-library factory

Closeout evidence:

```bash
cargo test -q operand_view
cargo test -q phi_input_materializer
cargo test -q string_corridor_relation
cargo test -q string_corridor_sink
cargo test -q string_kernel_plan
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
bash apps/rust-subset-to-hako/smoke.sh
bash apps/rust-subset-to-hako/smoke_adapter.sh
```

## Paused Lanes

- exact-AOT fastpath optimization is paused until a fresh measured owner
  appears.
- VM product-route app validation is retired; app/selfhost validation uses
  EXE/AOT unless a semantic-reference VM task explicitly opts in.
- build crate split planning is available but not the active blocker.
