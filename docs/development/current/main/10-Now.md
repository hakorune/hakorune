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
MIRBUILDER-REPLACE-HARDCODED-FAMILY-GENERATORS-001
```

Read `docs/development/current/main/CURRENT_STATE.toml` for the complete active
lane status. The current task-order SSOT is
`docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md`.
BindingContextNative, VariableContextNative simple-map, VariableContext
snapshot/restore artifact ownership transfer, the shared MirBuilder emitter,
the shared family generator helper and driver, the shared family manifest
helper, and the MirBuilder converter matrix guard are green. The next
implementation task is replacing hard-coded family generators behind that
matrix, without new route/card churn.

Historical context follows.

296x-1510 extracted BindingContext MIR lifecycle facts from
optimized MIR without Hako plan, `.hako`, backend, or authority-promotion
claims. 296x-1511 selected the Derived-to-Native Hako Artifact Model:
generated Hako is a selected execution artifact, while native Hako adoption is
the later source-selfhost gate. 296x-1512 now opens the BindingContext
behavioral derived-artifact pilot without mainline selection. 296x-1512A closes
`OrderedMapBox.remove` and `OrderedMapBox.clear` as library-owned behavior, so
1512 no longer needs a converter/emitter workaround for those operations.
296x-1512 closes the BindingContext DerivedShadow artifact with deterministic
regeneration, generated Hako parse/MIR/EXE gates, and Rust oracle behavior
parity. 296x-1513 records a DerivedMainline candidate route manifest for
BindingContext only while keeping selected_on_mainline=0, Rust
bootstrap/oracle retained, and runtime fallback forbidden. 296x-1514 now
selects wait_for_route_seam rather than premature HakoAdopted source
adoption. 296x-1515 now designs the smallest explicit BindingContext
derived-mainline route seam, records BindingContext as not selected because no
selfhost family-artifact route seam exists yet, and keeps runtime fallback
forbidden. 296x-1516 now defines that route seam as an SSOT before any
generated artifact is selected. 296x-1517 now applies that seam to
BindingContext only and selects it as the DerivedMainline family route.
296x-1518 now selects the next derived-artifact pilot.
It selects VariableContext simple-map only; returned borrow, snapshot/restore,
and carrier-sensitive behavior stay out of scope. 296x-1519 now generates
that bounded artifact. 296x-1520 now decides its derived route selection.
296x-1521 selects immutable `variable_map()` BorrowView as the next bounded
owner and records the mini-model task ladder through carrier snapshots.
296x-1522 closes the immutable BorrowView derived artifact pilot.
296x-1523 selects that generated artifact as a derived_hako family route.
296x-1524 closes the snapshot/restore derived artifact pilot.
296x-1525 now decides whether that generated artifact is selected as a
derived_hako family route.
BindingContext and VariableContext simple-map lifecycle pilots are
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
296x-1417 selects one verified-plan emitter probe. 296x-1418 fixture-guards
one CarrierInfo::merge_from lifecycle surface without executable-program,
backend, or converter-core rewrite claims.
296x-1419 selects join_id vocabulary decision before expanding emitter or
resolver coverage. 296x-1420 parks CarrierVar.join_id as test-fixture/stale
vocabulary for the lifecycle lane, with resolver/verifier/emitter deny rules
intact. 296x-1421 selects ownership-aware converter reference documentation
before resuming lifecycle owner probing. 296x-1422 documents the converter as
a verified-plan renderer, not an ownership policy owner. 296x-1423 selects
trim_helper lifecycle inventory as the next owner. 296x-1424 inventories
trim_helper as route-specific metadata with resolver/verifier/emitter deny
boundaries intact. 296x-1425 selects TrimRouteInfo::to_carrier_info producer
probing. 296x-1426 fixture-guards TrimRouteInfo::to_carrier_info as
TrimHelperCarrierProducer without trim route lowering, promoted_body_locals
ownership, join_id producer, or general resolver claims. 296x-1427 selects
promoted_body_locals inventory. 296x-1428 inventories promoted_body_locals
producers, merge behavior, and join_id-dependent consumers without claiming
join_id production or route lowering. 296x-1429 selects promoted_body_locals
producer probing. 296x-1430 fixture-guards trim and DigitPos
promoted_body_locals producers as name recorders only, without join_id
producer, route lowering, resolver, or emitter claims. 296x-1431 selects
promoted-name resolution deny closeout. 296x-1432 closes promoted-name
resolution as denied until a production join_id producer exists. 296x-1433
selects bounded lifecycle emitter parser/MIR surface probing. 296x-1434
makes the existing CarrierInfo::merge_from emitter surface parser/MIR-checkable
without generated-program, backend, or converter-core claims. 296x-1435
selects trim route lowering inventory as the next owner. 296x-1436 inventories
the trim route lowering boundary without backend, generated-program,
rustc-adapter, or resolver-Allow claims. 296x-1437 selects a read-only trim
route lowering decision probe. 296x-1438 fixture-guards trim route metadata as
a metadata candidate while executable route lowering remains denied by
MissingPromotedCarrierIdentity. 296x-1439 selects promoted carrier identity /
join_id design inventory. 296x-1440 inventories keep-denied,
CarrierVar.join_id producer, and ConditionBinding identity options without
implementing any producer. 296x-1441 selects condition-binding identity as the
promoted carrier identity policy. 296x-1442 records that decision without
rewriting resolution, adding a join_id producer, or emitting trim route
lowering. 296x-1443 selects a read-only condition-binding promoted identity
proof probe. 296x-1444 fixture-guards AllowIdentityCandidate and missing /
mismatched deny vectors without resolution rewrite or trim lowering. 296x-1445
selects condition-binding resolution rewrite design. 296x-1446 chooses an
additive adapter design that keeps legacy `resolve_promoted_join_id` intact.
296x-1447 selects the additive adapter probe. 296x-1448 implements the adapter
as a read-only `CarrierInfo` query without scope-manager wiring or trim route
lowering. 296x-1449 selects scope-manager wiring design. 296x-1450 documents
the explicit condition_bindings input and lookup order without changing code.
296x-1451 selects the scope-manager condition-bindings input probe.
296x-1452 adds the input and focused lookup tests without trim route lowering.
296x-1453 selects trim route lowering proof update.
296x-1454 refreshes the proof: identity is available, executable lowering
remains denied by missing implementation readiness.
296x-1455 selects executable trim route lowering implementation design. 296x-1456
documents a readiness gate before backend lowering.
296x-1457 selects the trim route lowering readiness gate.
296x-1458 implements the read-only readiness decision without backend lowering.
296x-1459 selects readiness integration inventory. 296x-1460 inventories the
boundary/route-lowering seam as the first valid callsite.
296x-1461 selects a route-boundary trim readiness integration probe.
296x-1462 adds the read-only JoinInlineBoundary readiness probe without backend
lowering.

296x-1464 documents the Rust-to-Hako lifecycle projection reference: converter
emission is allowed only from verified HakoLifecyclePlan input, not direct Rust
syntax ownership rewriting.

296x-1465 selects RustLifecycleFacts adapter inventory for
BindingContext/VariableContext as the next owner before resolver, verifier, or
emitter implementation.

296x-1502 adds diagnostic-only rustc semantic adapter toolchain compatibility
preflight. Local stable toolchain reports
`rustc_private_readiness=requires_nightly_or_bootstrap`. 296x-1503 selects
toolchain setup / override design before HIR inventory. 296x-1504 selects
pinned-date nightly plus rustc-dev and compile/link/run proof as the formal
standalone adapter route; `RUSTC_BOOTSTRAP` is diagnostic_untrusted only.
296x-1505 implements the adapter-local pinned nightly preflight and verifies
`rustc_driver` compile/link/run without facts or backend changes. 296x-1506
adds HIR item/provenance inventory only, without extracting THIR/MIR or
lifecycle facts. 296x-1507 is the current selection row: choose the next rustc
semantic adapter owner.

296x-1466 inventories the first BindingContext / VariableContext fact
requirements and keeps the adapter policy-free.

296x-1467 selects the compact BindingContext adapter fact fixture as the first
post-inventory owner.

296x-1468 adds the target-neutral BindingContext adapter facts fixture and
guard, with Hako policy spellings excluded.

296x-1469 selects a passive verifier-result fixture over BindingContext adapter
facts and the existing lifecycle plan.

296x-1470 adds a passive verifier-result fixture over BindingContext adapter
facts and the existing lifecycle plan without enabling emission.

296x-1471 selects the VariableContext adapter fact fixture as the next owner.

296x-1472 adds the target-neutral VariableContext adapter facts fixture and
guard.

296x-1473 selects a passive verifier-result fixture over VariableContext
adapter facts and existing plan fixtures.

296x-1474 adds the passive VariableContext adapter verifier fixture without
enabling emission.

296x-1475 selects a fixture-only verifier skeleton as the next owner.

296x-1476 adds the fixture-only lifecycle checker over checked-in
BindingContext and VariableContext JSON fixtures.

296x-1477 selected the ownership-aware converter boundary tasking before
returning to implementation. 296x-1478 records the converter task sequence as
verified HakoLifecyclePlan rendering, not direct Rust syntax ownership
rewriting. 296x-1479 defines the lifecycle-aware converter two-input
boundary. 296x-1480 selects and renders one verified lifecycle plan fixture
into a bounded `.hako` surface. The active row is 296x-1481. It compares that
surface against the Rust oracle for the selected family only. The active row is
296x-1482. It selects BindingContext as the first rustc semantic
lifecycle-facts adapter probe. The active row is 296x-1483. It probes the
BindingContext adapter shape as target-neutral RustLifecycleFacts-v0 output.
296x-1484 selects the minimal external rustc adapter harness design as the
next owner. The active row is 296x-1485. It designs the BindingContext
lifecycle-facts adapter harness boundary. The active row is 296x-1486. It adds
the first minimal BindingContext rustc semantic adapter harness probe.
296x-1487 selects the minimal rustc adapter toolchain preflight as the next
owner. The active row is 296x-1488. It adds a diagnostic-only toolchain
preflight without extracting lifecycle facts. The active row is 296x-1489. It
selects BindingContext lifecycle facts extraction as the next owner. The
active row is 296x-1490. It extracts focused BindingContext facts from the
selected Rust source slice and verifies them against the target-neutral
adapter fixture. The active row is 296x-1491. It selects the next owner after
that extraction pilot. The active row is 296x-1492. It pilots focused
VariableContext lifecycle facts extraction. The active row is 296x-1493. It
selects extracted-facts verifier parity as the next owner. The active row is
296x-1494. It verifies extractor-produced facts through the existing lifecycle
verifier path. The active row is 296x-1495. It selects the next owner after
verifier parity. The active row is 296x-1496. It documents the rustc-internal
semantic adapter boundary before implementation.
296x-1496 documents the rustc semantic adapter boundary. The active row is
296x-1497. It selects the next owner after that design. The active row is
296x-1498. It documents the rustc semantic adapter tool boundary and preflight
contract. The active row is 296x-1499. It selects the next owner after that
design. The active row is 296x-1500. It adds the diagnostic-only adapter tool
preflight skeleton and guard. The active row is 296x-1501. It selects the next
owner after preflight. The active row is 296x-1502. It adds diagnostic-only
rustc toolchain compatibility classification before HIR extraction.

## Next

1. Read 296x-1502.
2. Add diagnostic-only toolchain compatibility preflight.
3. Keep lifecycle facts generation disabled.
4. Run:

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
