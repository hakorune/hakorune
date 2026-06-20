# CURRENT_TASK

Status: SSOT pointer
Date: 2026-06-20
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read the `latest_card_path` named in `CURRENT_STATE.toml`.
3. Check the worktree:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

4. Run heavier gates only when the current code slice is ready:

```bash
tools/checks/dev_gate.sh quick
```

## Current Task

Read these fields in `docs/development/current/main/CURRENT_STATE.toml`:

- `active_lane`
- `active_phase`
- `latest_card_path`
- `current_blocker_token`

Current blocker:

```text
RUSTC-SEMIR-ADAPTER-TOOLCHAIN-COMPAT-PREFLIGHT-001
```

Purpose:

```text
Add a diagnostic-only toolchain compatibility preflight for the standalone
rustc semantic adapter tool. Do not extract lifecycle facts in this row.
```

Lifecycle converter boundary:

```text
The answer to "can the converter translate Rust ownership into .hako?" is
documented in docs/development/current/main/design/rust-lifecycle-projection-ssot.md.
The practical reference manual is:
docs/reference/architecture/rust-to-hako-lifecycle-projection.md.

Short form:
  yes, but only as rustc facts -> HakoLifecyclePlan -> verifier -> emitter.

The converter/emitter renders verified plans. It does not choose ownership,
borrow, move, or Drop policy directly from Rust syntax.
```

Current evidence:

```text
STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001 is closed by 296x-1305.
PHI-INPUT-REMAT-OPERAND-MEMO-001 is closed by 296x-1306.
STRING-CORRIDOR-STABLE-LENGTH-HINT-FALLBACK-RETIRE-001 is closed by 296x-1307.
RUST-SUBSET-APP-FRONT-LOOP-TRUE-BREAK-CONTINUE-SMOKE-CLOSEOUT-001 is closed by
296x-1308. `apps/rust-subset-to-hako/smoke.sh` reports `summary=ok`.
RUST-SUBSET-SYN-ADAPTER-SMOKE-ENTRY-001 is closed by 296x-1309.
RUST-SUBSET-SYN-ADAPTER-INDEX-EXPRESSION-001 is closed by 296x-1310.
RUST-SUBSET-SYN-ADAPTER-BREAK-CONTINUE-UNSUPPORTED-HANDOFF-001 is closed by
296x-1311.
RUST-SUBSET-SYN-ADAPTER-GENERIC-FUNCTION-SKELETON-001 is closed by 296x-1312.
RUST-SUBSET-SYN-ADAPTER-UNSUPPORTED-TRAIT-HANDOFF-HARDENING-001 is closed by
296x-1313.
RUST-SUBSET-SMOKE-FIXTURE-TABLE-REFACTOR-001 is closed by 296x-1314.
RUST-SUBSET-CRATE-HANDOFF-INVENTORY-001 is closed by 296x-1315.
RUST-SUBSET-MODULE-SCHEMA-VALIDATION-PARITY-001 is closed by 296x-1316.
RUST-SUBSET-PATH-NAME-NORMALIZATION-001 is closed by 296x-1317.
RUST-SUBSET-CRATE-MANIFEST-V0-001 is closed by 296x-1318.
RUST-SUBSET-SYN-ADAPTER-MULTI-MODULE-PROBE-001 is closed by 296x-1319.
RUST-SUBSET-CRATE-HANDOFF-MIR-ACCEPTANCE-001 is closed by 296x-1320.
HAKO-ORDERED-MAP-BOX-SSOT-001 is closed by 296x-1321.
HAKO-ORDERED-MAP-BOX-V0-001 is closed by 296x-1322.
MIRBUILDER-BINDING-CONTEXT-ORDERED-MAP-PROBE-001 is closed by 296x-1323.
CONSTRUCTOR-LIFECYCLE-FIELD-INIT-BIRTH-PROBE-001 is closed by 296x-1324.
FIELD-INITIALIZER-LIBRARY-ROUTE-PROBE-001 is closed by 296x-1325.
IMPORTED-FIELD-INIT-BIRTH-MERGE-FIX-001 is closed by 296x-1326.
GLOBAL-CALL-UNKNOWN-CALLEE-DIAGNOSTIC-001 is closed by 296x-1327.
CREAT-SUBSET-PILOT-SELECTION-001 is closed by 296x-1328.
HAKORUNE-BOX-CORE-RUSTSUBSET-PILOT-001 is closed by 296x-1329.
RUST-SUBSET-GENERATED-FUNCTION-MIR-ACCEPTANCE-001 is closed by 296x-1330.
RUST-SUBSET-NEXT-CRATE-PILOT-SELECTION-001 is closed by 296x-1331.
HAKORUNE-MIR-CORE-RUSTSUBSET-PILOT-001 is closed by 296x-1332.
RUST-SUBSET-CRATE-WRAPPER-EXE-PURE-ROUTE-UNBLOCK-001 is closed by 296x-1333.
PURE-ROUTE-UNSUPPORTED-SHAPE-DIAGNOSTIC-001 is closed by 296x-1334.
RUST-SUBSET-CRATE-WRAPPER-EXE-SMOKE-001 is closed by 296x-1335.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001 is closed by 296x-1336.
HAKORUNE-MIR-CORE-ID-MODULES-RUSTSUBSET-PILOT-001 is blocked by 296x-1337.
RUST-SUBSET-TUPLE-STRUCT-CONSTRUCTOR-SKELETON-001 is closed by 296x-1338.
RUST-SUBSET-COMPOUND-ASSIGN-SKELETON-SAFETY-001 is closed by 296x-1339.
RUST-SUBSET-SELF-QUALIFIED-CALL-SKELETON-SAFETY-001 is closed by 296x-1340.
HAKORUNE-MIR-CORE-ID-MODULES-MATERIALIZATION-001 is closed by 296x-1341.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-002 is closed by 296x-1342.
HAKORUNE-MIR-CORE-VALUE-KIND-MATERIALIZATION-001 is closed by 296x-1343.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-003 is closed by 296x-1344.
RUST-SUBSET-ENUM-VARIANT-VALUE-SKELETON-SAFETY-001 is closed by 296x-1345.
RUST-SUBSET-VEC-NEW-CALL-SKELETON-SAFETY-001 is closed by 296x-1346.
HAKORUNE-MIR-CORE-EFFECT-MATERIALIZATION-001 is closed by 296x-1347.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-004 is closed by 296x-1348.
HAKORUNE-MIR-BUILDER-BINDING-CONTEXT-MATERIALIZATION-001 is closed by 296x-1349.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-005 is closed by 296x-1350.
HAKORUNE-MIR-BUILDER-VARIABLE-CONTEXT-MATERIALIZATION-001 is closed by 296x-1351.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-006 is closed by 296x-1352.
RUST-SUBSET-ASSOCIATED-FUNCTION-CALL-SKELETON-SAFETY-001 is closed by 296x-1353.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-007 is closed by 296x-1354.
RUST-SUBSET-ASSOCIATED-CONST-VALUE-SKELETON-SAFETY-001 is closed by 296x-1355.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-008 is closed by 296x-1356.
HAKORUNE-MIR-BUILDER-CORE-CONTEXT-MATERIALIZATION-001 is closed by 296x-1357.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-009 is closed by 296x-1358.
HAKORUNE-MIR-BUILDER-CONTEXT-MATERIALIZATION-001 is closed by 296x-1359.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-010 is closed by 296x-1360.
HAKORUNE-MIR-DEFS-CALL-UNIFIED-MATERIALIZATION-001 is closed by 296x-1361.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-011 is closed by 296x-1362.
RUST-SUBSET-GENERIC-IMPL-TARGET-SKELETON-SAFETY-001 is closed by 296x-1363.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-012 is closed by 296x-1364.
RUST-SUBSET-REFERENCE-TYPE-SPELLING-SKELETON-SAFETY-001 is closed by 296x-1365.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-013 is closed by 296x-1366.
HAKORUNE-MIR-BUILDER-TYPE-CONTEXT-MATERIALIZATION-001 is closed by 296x-1367.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-014 is closed by 296x-1368.
RUST-SUBSET-SELF-VALUE-SKELETON-SAFETY-001 is closed by 296x-1369.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-015 is closed by 296x-1370.
RUST-SUBSET-OPTION-CONSTRUCTOR-SKELETON-SAFETY-001 is closed by 296x-1371.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-016 is closed by 296x-1372.
HAKORUNE-MIR-BUILDER-METADATA-CONTEXT-MATERIALIZATION-001 is closed by 296x-1373.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-017 is closed by 296x-1374.
HAKORUNE-MIR-BUILDER-CRATE-ROOT-MATERIALIZATION-001 is closed by 296x-1375.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-018 is closed by 296x-1376.
HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-001 is blocked by 296x-1377
after the reusable FileBox helper reaches MIR emit but fails EXE pure-route
lowering. Hand-unrolled 7-module wrapper fallback remains forbidden.
CRATE-BUNDLE-FILE-ROUTE-HELPER-EXE-SHAPE-001 is closed by 296x-1379 as a
boundary decision: FileBox remains Main/input-route owned; helper-owned
FileBox is not implemented in that row.
FILEBOX-DYNAMIC-PATH-LOOP-EXE-SHAPE-001 is closed by 296x-1380; the focused
Main-owned dynamic FileBox path loop reaches MIR emit and EXE.
HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-RESUME-001 is blocked by
296x-1382 after the first wrapper implementation reaches MIR emit but fails
EXE on loop-carried string accumulation with an undefined LLVM value in
nyash.string.concat3_hhh.
STRING-CONCAT-LOOP-CARRIED-EXE-SHAPE-001 is closed by 296x-1383; deferred
concat pairs are materialized before later concat_hh / concat3 use.
HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-RESUME-001 is closed by
296x-1382; the manifest-driven 7-module `hakorune_mir_builder` crate-bundle
wrapper reaches adapter crate-mode golden, wrapper EXE parity, and
fixture-only aggregate generated-skeleton MIR emit.
RUST-LIFECYCLE-PROJECTION-SSOT-001 is closed by 296x-1381; the boundary is
fixed as rustc adapter facts -> Hako lifecycle resolver -> verifier ->
converter/emitter of verified plans.
RUST-LIFECYCLE-FACTS-VOCAB-000 is closed by 296x-1384; passive
RustLifecycleFacts-v0 vocabulary is documented without behavior changes.
HAKO-LIFECYCLE-PLAN-VOCAB-000 is closed by 296x-1385; passive
HakoLifecyclePlan-v0 vocabulary is documented without resolver, verifier,
emitter, or pilot behavior.
RUST-TO-HAKO-LIFECYCLE-EMITTER-CONTRACT-000 is closed by 296x-1386; the
converter/emitter contract is fixed as rendering verified plans only.
MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001 is closed by 296x-1387;
BindingContext lifecycle facts/plan fixtures and guard are green.
MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-ORACLE-PARITY-001 is closed by 296x-1388;
the BindingContext lifecycle plan matches Rust oracle vectors and promotion is
limited to BindingContext only.
RUST-LIFECYCLE-NEXT-OWNER-SELECTION-001 is closed by 296x-1389; A-lite is
selected as VariableContext lifecycle gap inventory before any pilot.
VARIABLE-CONTEXT-LIFECYCLE-GAP-INVENTORY-001 is closed by 296x-1390; the next
slice is VariableContext simple map only, excluding returned map and
snapshot/restore behavior.
VARIABLE-CONTEXT-LIFECYCLE-SIMPLE-MAP-PILOT-001 is closed by 296x-1391;
simple-map facts/plan fixtures and guard are green.
VARIABLE-CONTEXT-LIFECYCLE-SIMPLE-MAP-ORACLE-PARITY-001 is closed by
296x-1392; simple-map plan matches oracle vectors without claiming returned
map, snapshot/restore, carrier, or PHI behavior.
RUST-LIFECYCLE-POST-VARIABLE-SIMPLE-MAP-OWNER-SELECTION-001 is closed by
296x-1393; returned borrow boundary inventory is selected as the next owner.
VARIABLE-CONTEXT-RETURNED-BORROW-BOUNDARY-INVENTORY-001 is closed by
296x-1394; variable_map() consumers are classified, variable_map_mut() has no
external callsites but remains Deny(ReturnedMutableBorrow), and follow-up rows
are named.
VARIABLE-CONTEXT-POST-RETURNED-BORROW-OWNER-SELECTION-001 is closed by
296x-1395; immutable map BorrowView probe is selected as the next owner.
VARIABLE-CONTEXT-IMMUTABLE-MAP-BORROWVIEW-PROBE-001 is closed by 296x-1396;
owner-carrying read BorrowView fixtures and guard are green while mutable map,
snapshot/restore, carrier/PHI, and resolver claims remain disabled.
VARIABLE-CONTEXT-POST-BORROWVIEW-OWNER-SELECTION-001 is closed by 296x-1397;
snapshot/restore ownership is selected as the next owner.
VARIABLE-CONTEXT-SNAPSHOT-RESTORE-OWNERSHIP-001 is closed by 296x-1398;
CloneOwnedMap and ReplaceOwned fixtures and guard are green while mutable map,
carrier/PHI, and resolver claims remain disabled.
VARIABLE-CONTEXT-POST-SNAPSHOT-RESTORE-OWNER-SELECTION-001 is closed by
296x-1399; mutable-map Deny closeout is selected as the next owner.
VARIABLE-CONTEXT-MUTABLE-MAP-DENY-CLOSEOUT-001 is closed by 296x-1400;
variable_map_mut() has no external Rust callsites and stays
Deny(ReturnedMutableBorrow).
VARIABLE-CONTEXT-POST-MUTABLE-DENY-OWNER-SELECTION-001 is closed by 296x-1401;
carrier/PHI lifecycle inventory is selected as the next owner.
VARIABLE-CONTEXT-CARRIER-PHI-LIFECYCLE-INVENTORY-001 is closed by 296x-1402;
CarrierInfo and region-observer map consumers are inventoried, with carrier
snapshot probes and PHI consumer inventory named as follow-ups.
POST-CARRIER-PHI-INVENTORY-OWNER-SELECTION-001 is closed by 296x-1403;
CarrierInfo::from_variable_map snapshot is selected as the next owner.
VARIABLE-CONTEXT-CARRIER-SNAPSHOT-PLAN-PROBE-001 is closed by 296x-1404;
CarrierInfo::from_variable_map is fixture-guarded as
CarrierSnapshotFromBorrowView without PHI join_id or resolver claims.
POST-CARRIER-SNAPSHOT-OWNER-SELECTION-001 is closed by 296x-1405; explicit
carrier snapshot is selected as the next owner.
VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-PROBE-001 is closed by 296x-1406;
CarrierInfo::with_explicit_carriers is fixture-guarded as
ExplicitCarrierSnapshotFromBorrowView with missing-carrier fail-fast preserved.
POST-EXPLICIT-CARRIER-SNAPSHOT-OWNER-SELECTION-001 is closed by 296x-1407;
PHI carrier lifecycle consumer inventory is selected before a general
resolver skeleton.
PHI-CARRIER-LIFECYCLE-CONSUMER-INVENTORY-001 is closed by 296x-1408;
join_id, promoted_body_locals, trim_helper, merge_from, and read-only
CarrierInfo consumers are inventoried.
POST-PHI-CARRIER-CONSUMER-INVENTORY-OWNER-SELECTION-001 is closed by
296x-1409; join_id producer inventory is selected before any probe or
resolver.
PHI-CARRIER-JOIN-ID-LIFECYCLE-PRODUCER-INVENTORY-001 is closed by 296x-1410;
production CarrierVar.join_id has no Some(ValueId) producer and is None-only
outside tests/fixtures.
POST-JOIN-ID-PRODUCER-INVENTORY-OWNER-SELECTION-001 is closed by 296x-1411;
CarrierInfo::merge_from lifecycle probing is selected as the next live mutation
boundary.
CARRIER-INFO-MERGE-FROM-LIFECYCLE-PROBE-001 is closed by 296x-1412;
merge_from is fixture-guarded as OwnedCarrierInfoMerge without join_id producer
or resolver claims.
POST-MERGE-FROM-LIFECYCLE-OWNER-SELECTION-001 is closed by 296x-1413;
read-only resolver skeleton is selected as the next diagnostic-only owner.
HAKO-LIFECYCLE-RESOLVER-READONLY-SKELETON-001 is closed by 296x-1414;
diagnostic AllowPlan / DenyUnresolvedBoundary reporting is fixture-guarded
without verifier, emitter, backend, or selection-owner claims.
POST-READONLY-RESOLVER-OWNER-SELECTION-001 is closed by 296x-1415; passive
verifier result vocabulary is selected before emitter probing.
LIFECYCLE-VERIFIER-RESULT-VOCAB-000 is closed by 296x-1416; a bounded
CarrierInfo::merge_from VerifiedPlan result is fixture-guarded without
emission, backend, resolver-selection, or wide parity claims.
POST-VERIFIER-RESULT-VOCAB-OWNER-SELECTION-001 is closed by 296x-1417; one
verified-plan emitter probe is selected as the next owner.
RUST-TO-HAKO-LIFECYCLE-EMITTER-PROBE-001 is closed by 296x-1418; one
CarrierInfo::merge_from lifecycle surface is fixture-guarded without
executable-program, backend, or converter-core rewrite claims.
POST-LIFECYCLE-EMITTER-PROBE-OWNER-SELECTION-001 is closed by 296x-1419;
join_id vocabulary decision is selected before expanding emitter/resolver
coverage.
PHI-CARRIER-JOIN-ID-VOCABULARY-DECISION-001 is closed by 296x-1420;
CarrierVar.join_id is parked as test-fixture/stale vocabulary for the lifecycle
lane, with resolver/verifier/emitter deny rules intact.
```

Acceptance for the current slice:

```bash
bash tools/checks/rust_lifecycle_trim_route_lowering_inventory_guard.sh
bash tools/checks/rust_lifecycle_trim_route_lowering_decision_guard.sh
bash tools/checks/rust_lifecycle_promoted_carrier_identity_inventory_guard.sh
bash tools/checks/rust_lifecycle_promoted_carrier_identity_policy_guard.sh
bash tools/checks/rust_lifecycle_condition_binding_promoted_identity_guard.sh
bash tools/checks/rust_lifecycle_condition_binding_resolution_design_guard.sh
bash tools/checks/rust_lifecycle_condition_binding_resolution_adapter_guard.sh
bash tools/checks/rust_lifecycle_scope_manager_condition_binding_wiring_design_guard.sh
bash tools/checks/rust_lifecycle_scope_manager_condition_binding_input_guard.sh
bash tools/checks/rust_lifecycle_trim_route_lowering_proof_update_guard.sh
bash tools/checks/rust_lifecycle_executable_trim_route_lowering_design_guard.sh
bash tools/checks/rust_lifecycle_trim_route_lowering_readiness_gate_guard.sh
bash tools/checks/rust_lifecycle_trim_route_lowering_readiness_integration_inventory_guard.sh
bash tools/checks/rust_lifecycle_route_boundary_trim_readiness_probe_guard.sh
bash tools/checks/rust_lifecycle_emitter_surface_mir_guard.sh
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Task Order

1. Read 296x-1477.
2. Choose one next lifecycle / route owner.
3. Park non-selected owners explicitly.
4. Keep implementation_started=0 in this selection row.
5. Keep converter core and backend behavior unchanged.

Recommended next row:

```text
RUST-LIFECYCLE-FACTS-ADAPTER-CONTEXT-INVENTORY-001
```

Current lifecycle SSOT:

```text
docs/development/current/main/design/rust-lifecycle-projection-ssot.md
```
Current card:

```text
docs/development/current/main/phases/phase-296x/296x-1466-RUST-LIFECYCLE-FACTS-ADAPTER-CONTEXT-INVENTORY-001.md
```

Task sequence:

```text
1. POST-LIFECYCLE-FIXTURE-VERIFIER-SKELETON-OWNER-SELECTION-001
2. choose one of:
   A. return to trim route fixture selection
   B. VariableContext returned mutable borrow API replacement design
   C. lifecycle-aware emitter pilot design
```

## Pointers

- Current state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Latest phase card: read `latest_card_path` in `CURRENT_STATE.toml`
- Current docs policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Restart mirror: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Thin dashboard: `docs/development/current/main/10-Now.md`
