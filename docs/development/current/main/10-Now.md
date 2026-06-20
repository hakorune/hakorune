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
POST-VERIFIER-RESULT-VOCAB-OWNER-SELECTION-001
```

Read `docs/development/current/main/CURRENT_STATE.toml` for the complete active
lane status. BindingContext and VariableContext simple-map lifecycle pilots are
closed. 296x-1394 inventoried returned map borrows and keeps
`variable_map_mut()` denied as a returned mutable alias boundary. 296x-1395
selected immutable `variable_map()` BorrowView as the next owner. 296x-1396
closed that BorrowView probe with fixture and guard parity. 296x-1398 closed
snapshot/restore ownership with CloneOwnedMap and ReplaceOwned fixtures.
296x-1400 closed `variable_map_mut()` as Deny(ReturnedMutableBorrow) with no
external Rust callsites. 296x-1402 inventoried carrier-sensitive map reads.
296x-1404 fixture-guards CarrierInfo::from_variable_map as
CarrierSnapshotFromBorrowView. 296x-1406 fixture-guards
CarrierInfo::with_explicit_carriers as ExplicitCarrierSnapshotFromBorrowView.
296x-1407 selects PHI carrier lifecycle consumer inventory before a general
resolver. 296x-1408 inventories join_id, promoted_body_locals, trim_helper,
merge_from, and read-only CarrierInfo consumers.
296x-1409 selects join_id producer inventory before any probe or resolver.
296x-1410 records that production `CarrierVar.join_id` has no `Some(ValueId)`
producer and is currently None-only outside tests/fixtures.
296x-1411 selects `CarrierInfo::merge_from` lifecycle probing as the next live
mutation boundary. 296x-1412 fixture-guards merge_from as
OwnedCarrierInfoMerge without join_id producer or resolver claims.
296x-1413 selects a read-only resolver skeleton. 296x-1414 fixture-guards
diagnostic AllowPlan / DenyUnresolvedBoundary reporting without verifier,
emitter, backend, or selection-owner claims.
296x-1415 selects passive verifier result vocabulary before emitter probing.
296x-1416 fixture-guards a bounded CarrierInfo::merge_from VerifiedPlan result
without emission, backend, resolver-selection, or wide parity claims.

The active row is 296x-1417. It selects the next lifecycle owner after passive
verifier result vocabulary.

## Next

1. Read 296x-1417.
2. Choose one next lifecycle owner.
3. Park non-selected owners explicitly.
4. Keep implementation_started=0 in this selection row.
5. Run:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Recently Closed

- `IMPORTED-FIELD-INIT-BIRTH-MERGE-FIX-001`
  - imported static factory field-initializer routes preserve `new -> birth`
  - App-mode static method lowering is deferred until instance constructors are lowered
  - focused field-initializer, OrderedMap, and constructor-lifecycle smokes are green
- `GLOBAL-CALL-UNKNOWN-CALLEE-DIAGNOSTIC-001`
  - `reason=unknown_global_callee` remains stable
  - MIR route metadata adds `reason_detail` and `reason_hint`
  - route selection and backend lowering are unchanged
- `CREAT-SUBSET-PILOT-SELECTION-001`
  - `crate_inventory.py` inventories manifest bundles without parsing Rust
  - `hakorune_box_core` is selected as the first real 3-module crate pilot
  - `Use` remains explicit Unsupported handoff; no new `.hako` syntax is added
- `HAKORUNE-BOX-CORE-RUSTSUBSET-PILOT-001`
  - `hakorune_box_core` manifest bundle is checked in
  - empty struct and impl receiver skeleton output are made parser-safe
  - full skeleton parse and leaf-module MIR emit are verified
- `RUST-SUBSET-GENERATED-FUNCTION-MIR-ACCEPTANCE-001`
  - top-level generated functions lower as standalone MIR declarations
  - runtime script statements exclude top-level FunctionDeclaration nodes
  - `hakorune_box_core_expected.hako` emits MIR
- `RUST-SUBSET-NEXT-CRATE-PILOT-SELECTION-001`
  - candidate crates are inventoried through manifest bundles
  - `hakorune_mir_core` is selected with modules `control_ids` and `types`
  - next blocker is materializing that selected slice
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
