---
Status: Active workstream
Date: 2026-07-30
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
North star:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
Task map:
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
---

# MirBuilder In-Place Replacement Workstream

## Goal

最終production authorityを次の一本へ収束させる。

```text
Resolve -> Observe -> Facts -> Recipe -> Verify
        -> Lower -> Seal -> Collect -> Atomic Publish
```

現在のMirBuilderを稼働させたまま、競合する責務ownerを一つずつ交換する。
第二MirBuilder、production consumer 0のroute拡張、Legacy fallbackは作らない。
cell数、pack数、LOCは観測値であり、完成条件ではない。

## Current state

```text
Parent:        RAW-ENTRY-MATERIALIZATION-CONTRACT0-D0
Latest landed: NORMAL-INSTANCE-CONSTRUCTOR-CALLABLE-IDENTITY0-I0-R0
Result:        selected Script direct Box raw compatibility is retired
Latest landed:  `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-DISPOSITION0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS24-D0` — closed
Latest landed:  `NORMAL-SCRIPT-PRINT-DIRECT-OWNER0-I0-R0`
Latest design:  `NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-I0-R0`
Latest design:  `NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS25-D0` — closed
Latest design:  `NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS26-D0` — closed
Latest design:  `NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS27-D0` — closed
Latest design:  `NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS28-D0` — closed
Latest design:  `NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS29-D0` — closed
Latest design:  `NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS30-D0` — closed
Latest design:  `NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS31-D0` — closed, NoSafeSlice
Latest design: `RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-D0` — closed
Latest landed: `RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS33-D0` — closed
Latest landed: `RAW-ROOT-LEGACY-BRANDED-TERMINAL-RESIDUE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS34-D0` — closed
Latest design: `RAW-STATIC-MAIN-COMPAT-BATCH-DISPOSITION0-D0` — RETAIN-FENCED
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS35-D0` — closed
Latest landed: `JOINMODULE-PHI-OBSERVER-RETIRE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS36-D0` — closed
Latest landed: `JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS37-D0` — closed
Latest landed: `LLVM-JOINMODULE-EXPERIMENT-ROUTE-RETIRE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS38-D0` — closed
Latest landed: `JOINIR-FRONTEND-FUNC-META-RETIRE0-RET0`
Next stop:     `MIRBUILDER-LIVE-EDGE-CENSUS39-D0`
History:       Git history and the short landed tail below
```

## R4 fence / residual registry

This is the sole current list for R4 disposition.  A prose `fenced` or
`separate` label is not a registered fence.  R4 Complete requires every active
or unregistered row below to be closed, rehomed, or retained with its complete
activation and sunset contract.

| State | Ledger key / family | Exact surface | Activation / normal-default | Target disposition | Release row / condition |
| --- | --- | --- | --- | --- | --- |
| retain-fenced | `RAW-STATIC-MAIN-COMPAT-BATCH-SUNSET-001` | `PreparedRawStaticMainBoxCompatibilityV1` prepared raw static-Main batch: sorted helpers followed by root Main with legacy entry policy | raw dispatcher static `Main` -> `RawLegacyChildLoweringPortV1`; selected normal verified App Main = 0 | RETAIN-FENCED: live arbitrary-AST raw route, no exact Program/source locator, helper-first and `LegacyEnvironment` coupling | fresh named release D0 only when one raw located-source + entry-materialization contract can atomically delete dispatcher -> static-Main, RawLegacy -> prepared batch, prepared helper -> raw static method, and prepared root -> legacy Main policy edges |
| closed | `NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003` | selected Script non-Box runtime compatibility, ending with the exact 9 unsupported kinds LoopRange, Break, Continue, ImportStatement, BuildGate, EnumDeclaration, BrandDeclaration, TypeAliasDeclaration, GlobalVar | selected normal Script only; raw/reference and nested body descent remain separate | REOWN | retired by `NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-I0-R0`: exact 9 -> direct shared guarded diagnostic; selected Script `RawCompatibility` execution = 0 |
| closed | `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001` | selected Program immediate instance constructors, plus selected Script plain-instance runtime-prefix constructors | every selected Program instance Box has one immediate demand; plain Script adds its second `InstancePrefixCompatibility` demand; non-plain Script's later raw runtime lifecycle is the separate row below | REOWN | retired by `NORMAL-INSTANCE-CONSTRUCTOR-CALLABLE-IDENTITY0-I0-R0`: one source occurrence -> unchanged physical LegacySymbol admission per existing demand |
| closed | `NORMAL-TOPLEVEL-FUNCTION-CALLABLE-COMPAT-SUNSET-003` | selected Program top-level `FunctionDeclaration` raw LegacyChild admission | selected normal only; raw/reference remains separate | REOWN | retired by `NORMAL-TOPLEVEL-FUNCTION-CALLABLE-IDENTITY0-I0-R0`: source-order receipt -> unchanged legacy physical collector admission |
| closed | `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-COMPAT-SUNSET-001` | selected Script runtime's plain non-Main static/instance Box ordinary-method direct raw admission | selected normal Script only; constructors, static Main, non-plain/nested/raw-reference Box descent excluded | REOWN | retired by `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-ADMISSION0-I0-R0`: selected direct raw method edges = 0 |
| closed | `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-COMPAT-SUNSET-002` | selected Script direct non-plain `BoxDeclaration` raw statement admission | selected normal Script only; raw/reference and non-Box Script statements remain separate | REOWN | retired by `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-DISPOSITION0-I0-R0`: static/instance selected lifecycles and exact sync rejection preserve legacy parity; direct Box -> raw statement driver = 0 |
| retain-fenced | `JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0` | two direct normalized-shadow executions and strict/dev StepTree observer | explicit dev/debug; default normal = 0 | RETAIN-FENCED | fresh named normalized-shadow release D0: verified Recipe/CorePlan loop owner, strict/dev parity, independent observer disposition |
| retain-fenced | `VM-BRIDGE-COMPAT-SUNSET-001` | `join_ir_vm_bridge_dispatch` Exec and LowerOnly targets | explicit VM keep / vm-reference with `NYASH_JOINIR_VM_BRIDGE=1`; default MIR and vm-fallback = 0 | RETAIN-FENCED | fresh named VM-bridge release D0: dispatcher caller = 0 or one explicit-lane execution owner replaces the lane |
| closed | `RAW-DRAFT-DISCONNECTED-PROOF-SUNSET-001` | `RawDraftInvocationV1`, its two cfg(test) callers, compiler `begin_raw_draft`, and dedicated guard | production caller = 0; disconnected proof owner only | RET0 | retired by `RAW-DRAFT-DISCONNECTED-PROOF-RETIRE0-RET0`: complete owner/test/compiler/guard surface = 0 |
| closed | `RAW-ROOT-STATIC-CHILD-DRAFT-COMPAT-SUNSET-001` | former `InvocationPhysicalStateV1::complete_raw_static_child` direct `LegacyChildDraftAdmissionV1` issuer shared by static helpers and callable Main | explicit raw public / VM-reference route; default normal = 0 | REOWN | retired by `RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0`: one existing locator+role admission now reaches the unchanged collector projection; direct legacy-symbol issuer = 0 |
| closed | `RAW-ROOT-LEGACY-BRANDED-TERMINAL-SUNSET-001` | former caller-zero `complete_legacy_child_branded` and `commit_legacy_pending_branded` adapters from `LegacyChildDraftAdmissionV1` to branded collector receipt | activation = 0 after `RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0`; definitions only | RET0 | retired by `RAW-ROOT-LEGACY-BRANDED-TERMINAL-RESIDUE0-RET0`; unbranded, symbol-keyed, resolved, and nested-live terminals retained |
| closed | `LLVM-JOINMODULE-EXPERIMENT-ROUTE-SUNSET-001` (promotes `R4-UNREGISTERED-LLVM-EXPERIMENT-001`) | former LLVM runner `JoinIrExperimentBox`: `Main.skip/1` MIR -> JoinModule -> MIR replacement plus original-MIR return on lowering/bridge failure | activation and complete LLVM-only owner/hook/env surface = 0 | RET0 | retired by `LLVM-JOINMODULE-EXPERIMENT-ROUTE-RETIRE0-RET0`; shared JoinModule lowering, VM bridge, normalized-shadow fence, and shared experiment flag remain |
| closed | `JOINIR-FRONTEND-FUNC-META-SUNSET-001` (promotes `R4-UNREGISTERED-FRONTEND-METADATA-001`) | former `frontend::func_meta`, public `JoinFuncMeta`/`JoinFuncMetaMap`, bridge metadata observation and `*_with_meta` APIs | metadata types, non-empty issuers, observation, and old APIs = 0 | RET0 metadata authority; conversion REOWNED into crate-bounded `module_converter` and boundary-aware bridge | retired by `JOINIR-FRONTEND-FUNC-META-RETIRE0-RET0`; converter output, aliases, normalized boundary, AST analysis, and VM bridge preserved |
| unregistered | `R4-UNREGISTERED-CARRIER-BOUNDARY-001` — carrier boundaries | `JumpArgsLayout` and `JoinInlineBoundary` families | live/fenced consumer mapping not yet registered | undecided; named D0 required before C0 | CorePlan/MIR rehome D0, RET0, or RETAIN-FENCED |
| closed | `JOINMODULE-PHI-OBSERVER-SUNSET-001` (promotes `R4-UNREGISTERED-PHI-OBSERVER-001`) | former `verify_phi_reserved` global collector/report, three debug observation hooks, dedicated builder/module tests, exports, README and generated owner-inventory row | production decision consumer = 0 before deletion; complete asset now absent | RET0 | retired by `JOINMODULE-PHI-OBSERVER-RETIRE0-RET0`: complete observer/test/hook/wiring/docs surface = 0 and existing native-owner inventory regenerated |
| unregistered | `R4-UNREGISTERED-AST-FRONTEND-001` — legacy AST frontend | `AstToJoinIrLowerer` plus its fixture/dev-flag closure | production caller = 0; test/reference closure remains | undecided; named D0 required before C0 | `JOINMODULE-AST-FRONTEND-LEGACY-DISPOSITION0-D0` |
| closed | `JOINMODULE-TEST-HANDLER-LANE-SUNSET-001` (promotes `R4-UNREGISTERED-TEST-HANDLER-001`) | former cfg(test)-only `block_finalizer`, `handlers/**`, `merge_variable_handler`, and `terminator_builder` legacy VM-bridge handler lane | production conversion remains solely in `joinir_block_converter/**`; deleted lane and registrations = 0 | RET0 | retired by `JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0`: 14 files / 3743 lines, four cfg(test) module declarations, obsolete README section, stale PHI seam row, and generated inventory rows deleted |
| retain-fenced | `NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001` (promotes `R4-UNREGISTERED-NESTED-BOX-RAW-BODY-001`) | recursive `RawInvocationChildPortV1` -> nested static method and shared instance constructor/method `LegacyChildDraftAdmissionV1` issuers | selected normal function body is live; nested Main stays root-only reject; raw/reference are separate | RETAIN-FENCED: no exact source occurrence reaches the raw port | fresh `RAW-LOCATED-BODY-TRANSPORT0-D0` may select REOWN only when one function-relative located transport deletes a named production edge; otherwise forced disposition at `MIRBUILDER-R4-FINAL-CONFORMANCE0-C0` |
| unregistered | `R4-UNREGISTERED-JOINMODULE-REMAINDER-001` — JoinModule model/lowering/JSON/format remainder | old-IR reference scope outside rows above | normalized-dev Builder execution and explicit VM route have live consumers; LLVM experiment consumer is retired; exact model/lowering/JSON/format partition remains unregistered | undecided; named D0 required before C0 | partition live normalized-dev/VM owners from caller-zero reference scope, then delete or RETAIN-FENCED with exact owner, activation, retire_when |

The registry has four registered R4 fences, zero active compatibility residuals,
zero active retirements, twelve closed residuals, and three unregistered R4 family
rows.  This is an honest registry state, not a claim that eleven fences are
already registered.
`LegacyChildDraftAdmissionV1` occurrence count is a separate census metric
(`30` occurrences in `6` `src/mir` files at the latest exact census). The
selected-Script residual is closed; raw static Main is an explicit retained
fence above. Nested raw body descent was
promoted from its immutable unregistered audit key to
`NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001` and is now retain-fenced; it is no
longer an unregistered family.

Every census updates this table before selecting the next row. Every unregistered
family has an immutable audit key, but is not a fence or sunset until a D0
records its exact owner/edge, activation, and retire condition. A future active
fence must then receive its stable sunset ID and either its own named release D0
or the forced `MIRBUILDER-R4-FINAL-CONFORMANCE0-C0` decision here; an
unregistered family may not become active on a generic prose promise. No second
fence ledger is permitted.

Before R4 final conformance, one mandatory
`MIRBUILDER-R4-LEGACY-CHILD-ADMISSION-DISPOSITION0-D0` census must map every
live `LegacyChildDraftAdmissionV1` site to exactly one registry family and one
final disposition: `RET0`, `REOWN`, or `RETAIN-FENCED`.  Each retained site must
name its owner, activation, sunset ID, and release row/condition in this sole
registry.  The occurrence count alone is not a disposition, and R4 Complete is
forbidden while any site lacks this crosswalk.  A newly introduced fence is
invalid unless its release row/condition is recorded here in the same commit.

## Disposition closeout

`MIRBUILDER-LIVE-EDGE-CENSUS38-D0` — read-only census, closed

```text
Registry:
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=11, unregistered=3.

Four-family result:
  frontend metadata = empty-only production observation; RET0 selected.
  carrier boundary = JumpArgsLayout neutral REOWN plus JoinInlineBoundary
    subordinate to the existing normalized-shadow fence; later D0.
  AST frontend = caller-zero but 5560-line semantic evidence closure; later
    RETAIN-FENCED D0 after evidence crosswalk.
  JoinModule remainder = live normalized-dev/VM core; broad RET0 rejected.

Selected:
  JOINIR-FRONTEND-FUNC-META-RETIRE0-RET0, T2 atomic RET0 with bounded
  converter REOWN.

Next:
  execute the selected RET0, then return to a fresh live-edge census.
```

`JOINIR-FRONTEND-FUNC-META-RETIRE0-RET0` — T2 atomic RET0, closed

```text
Deleted:
  frontend func_meta module and public types; metadata observation and
  *_with_meta APIs; two metadata-only Phase40 tests and stale status prose.

Reowned:
  unchanged Structured JoinModule conversion into crate-bounded
  module_converter; boundary-aware bridge no longer accepts an empty metadata
  map.  Existing function aliasing and normalized-shadow boundary remain.

Evidence:
  cargo check --lib and vm-reference hakorune = green;
  Phase40 analysis 6/6, VM bridge 7/7, Stage-B body/FuncScanner, and
  normalized-shadow 90/90 = green; old metadata symbols/APIs = 0; focused
  rustfmt, diff check, and pointer guard = green.

Measured:
  metadata/tests net deletion = 250 lines before neutral converter move;
  new source/test/check files = 0 (meta.rs responsibility renamed/reowned);
  largest touched source/check file = 297 lines; replacement credit = 0.

R4:
  JOINIR-FRONTEND-FUNC-META-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=12, unregistered=3.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS39-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS37-D0` — read-only census, closed

```text
Registry:
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=10, unregistered=4.

LegacyChildDraftAdmissionV1:
  30 occurrences / 6 src/mir files.  The two live issuers both map to
  NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001; all other occurrences are
  support/test-only.  No unregistered live admission site remains.

Selected:
  LLVM-JOINMODULE-EXPERIMENT-ROUTE-RETIRE0-RET0, T2 atomic RET0.
  The LLVM-only opt-in Main.skip/1 mutation is a live competing production
  authority and silently returns the original MIR on lowering/bridge failure.

Preserve:
  shared JoinModule model/lowering; VM bridge; normalized-shadow dev fence;
  NYASH_JOINIR_EXPERIMENT and its shared accessor.

Next:
  execute the selected RET0, then return to a fresh live-edge census.
```

`LLVM-JOINMODULE-EXPERIMENT-ROUTE-RETIRE0-RET0` — T2 atomic RET0, closed

```text
Deleted:
  JoinIrExperimentBox and its source file; LLVM runner hook; pipeline plan and
  report fields; LLVM-only environment accessors/key; current inventory,
  runtime-report, reference-doc, and hako_check observations.

Preserved:
  LLVM compilation/execution order outside the hook; shared JoinModule
  lowering/model; VM bridge; normalized-shadow dev route;
  NYASH_JOINIR_EXPERIMENT; historical archive records.

Evidence:
  cargo check --lib, --bin hakorune, and llvm-harness+vm-reference = green;
  LLVM pipeline inventory and runtime report smokes = green;
  shared skip_ws, VM bridge, Stage-B body, and FuncScanner focused tests =
  green; retired current symbols/env/fallback owner = 0; pointer guard and
  diff check = green.

Measured:
  retirement surface excluding current pointer/closeout = 13 files,
  5 insertions / 232 deletions including one 124-line source file;
  new source/test/check files = 0; largest touched source/check file =
  274 lines; replacement credit = 0.

R4:
  LLVM-JOINMODULE-EXPERIMENT-ROUTE-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=11, unregistered=4.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS38-D0.
```

`JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0` — T1 detached RET0, closed

```text
Deleted:
  join_ir_vm_bridge/block_finalizer.rs;
  join_ir_vm_bridge/handlers/**;
  join_ir_vm_bridge/merge_variable_handler.rs;
  join_ir_vm_bridge/terminator_builder.rs;
  four cfg(test) module registrations and the obsolete README lane section.

Synchronized:
  canonical SSA seam inventory removed the retired bridge-PHI row;
  PHI publication caller inventory removed the retired caller;
  native-owner and failure-outcome control-flow inventories were regenerated
  with their existing generators.

Preserved:
  joinir_block_converter/** as the sole production conversion owner;
  bridge integration, VM execution, JoinModule model/metadata;
  MIR, grammar, route, diagnostics, and runtime behavior.

Evidence:
  cargo check --lib = green;
  joinir_block_converter 1/1 and bridge integration 7/7 = green;
  resolved Binding SSA contract = green;
  native-owner and failure-outcome control-flow inventory checks = green;
  retired source/module/path references = 0;
  current-state pointer guard and git diff check = green.
  The broad PHI type-publication inventory reaches a pre-existing LocalSSA
  anchor failure; the same failure reproduces at pre-change HEAD 01a4c553c7.

Measured:
  retired lane = 14 source/test files / 3743 lines;
  new source/test/check files = 0;
  largest retained touched source/check file = 645 lines;
  replacement credit = 0.

R4:
  JOINMODULE-TEST-HANDLER-LANE-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=10, unregistered=5.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS37-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS36-D0` — read-only census, closed

```text
R4 registry:
  sole ledger is consistent; no duplicate or missing live fence.
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=9, unregistered=5.

LegacyChildDraftAdmissionV1:
  30 occurrences / 6 src/mir files.
  Both live issuers are nested Box body descent and map exactly to
  NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001.
  No unregistered live admission site exists.

Six-family disposition:
  LLVM experiment       = explicit env/feature live; later RETAIN-FENCED D0.
  frontend metadata     = bridge/normalized-dev live; API census required.
  carrier boundary      = normal CFG plus bridge live; partitioned REOWN.
  AST frontend          = production zero, broad fixture closure; later RET0.
  test-handler lane     = cfg(test)-only, exact RET0 selected.
  JoinModule remainder  = normalized-dev/VM/LLVM live; partition required.

Selected:
  JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0, T1 detached RET0.

Exact delete:
  join_ir_vm_bridge block_finalizer.rs;
  handlers/**;
  merge_variable_handler.rs;
  terminator_builder.rs;
  four cfg(test) module declarations and obsolete README lane section.

Preserve:
  joinir_block_converter/** production owner;
  bridge conversion, VM execution, JoinModule model and metadata;
  grammar, MIR, runtime, route, diagnostics.

Non-claims:
  no feature work; no fallback/retry; no replacement credit;
  no disposition change for the four retained fences.

Next:
  JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0.
```

`JOINMODULE-PHI-OBSERVER-RETIRE0-RET0` — T1 detached RET0, closed

```text
Deleted:
  src/mir/join_ir/verify_phi_reserved.rs;
  three cfg(debug_assertions) observe_phi_dst hooks;
  src/mir/builder/phi_observation_tests.rs;
  join_ir/builder module wiring and JoinIR README observation row.

Regenerated:
  mirbuilder-native-owner-candidate-inventory-v0.json with the existing
  tools/rust_lifecycle generator.  Its pre-existing stale baseline was refreshed
  separately in 5bfb44e4c6; this RET0 records only the exact observer removal.

Preserved:
  next_value_id allocation; carrier/invariant PHI order and types;
  JoinIR lowering/verifier; routing, diagnostics, runtime/backend behavior.

Evidence:
  observer/hook/test symbols = 0;
  cargo check --lib = green;
  loop_header_phi_info 3/3 and phi_block_remapper 2/2 = green;
  native-owner inventory --check and current-state pointer guard = green.
  The broader merge filter has four Ring0Context-not-initialized failures;
  the same representative failure reproduces at pre-change HEAD 5bfb44e4c6,
  so it is not caused by this deletion.

Measured:
  source/test/README surface = 384 deleted lines;
  generated inventory delta = 5 additions / 13 deletions;
  new source/test/check files = 0; replacement credit = 0.

R4:
  JOINMODULE-PHI-OBSERVER-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=9, unregistered=6.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS36-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS35-D0` — read-only census, closed

```text
Selected:
  JOINMODULE-PHI-OBSERVER-RETIRE0-RET0, T1 detached RET0.

Exact asset:
  verify_phi_reserved.rs global BTreeSet observer/report and internal tests;
  three cfg(debug_assertions) observe_phi_dst hooks;
  builder phi_observation_tests.rs;
  join_ir/builder module wiring; JoinIR README row;
  generated native-owner inventory row.

Activation:
  hooks are debug-compiled writes only.
  enable/get/analyze/disable consumers are dedicated tests.
  production semantic / routing / diagnostic reads = 0.

Atomic delete:
  complete asset above = 0;
  regenerate mirbuilder-native-owner-candidate-inventory-v0.json with the
  existing tools/rust_lifecycle generator.

Preserve:
  builder.next_value_id allocation; carrier/invariant PHI order and types;
  JoinIR lowering/verifier; runtime/backend behavior; all non-observer tests.

R4:
  R4-UNREGISTERED-PHI-OBSERVER-001 is promoted to
  JOINMODULE-PHI-OBSERVER-SUNSET-001 for exact RET0.
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=8, unregistered=6.

Next:
  JOINMODULE-PHI-OBSERVER-RETIRE0-RET0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS34-D0` /
`RAW-STATIC-MAIN-COMPAT-BATCH-DISPOSITION0-D0` — read-only disposition, closed

```text
Decision:
  RAW-STATIC-MAIN-COMPAT-BATCH-SUNSET-001 = RETAIN-FENCED.

Owner:
  PreparedRawStaticMainBoxCompatibilityV1.

Activation:
  raw expression dispatcher static Main
  -> RawLegacyChildLoweringPortV1::lower_static_main_box.
  selected normal verified App Main reachability = 0.

Why no RET0 / REOWN:
  drive_raw_legacy_* remains live; the batch owns cloned box_name+methods
  rather than an exact Program/source locator; helper-first lowering and root
  diagnostics are coupled to LegacyEnvironment entry materialization.

Release:
  one exact raw located-source + entry-materialization contract must delete in
  one named row:
    dispatcher -> static-Main terminal
    RawLegacy port -> prepared compatibility batch
    prepared helpers -> raw static-method terminal
    prepared root -> legacy-policy Main terminal.

R4:
  retain-fenced=4, active compatibility=0, closed=8, unregistered=7.
  code / behavior / grammar / route delta=0.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS35-D0.
```

`RAW-ROOT-LEGACY-BRANDED-TERMINAL-RESIDUE0-RET0` — T1 detached RET0, closed

```text
Deleted:
  ModuleLoweringPortV1::complete_legacy_child_branded
  ModuleLoweringPortV1::commit_legacy_pending_branded

Preserved:
  complete_legacy_child; commit_legacy_pending;
  commit_legacy_symbol_pending; commit_legacy_symbol_pending_branded;
  resolved/canonical terminals; both live nested-Box legacy issuers.

Measured:
  LegacyChildDraftAdmissionV1 32 -> 30 occurrences, 6 files unchanged.
  module_lowering_invocation_legacy_term.rs 144 -> 106 lines.
  largest touched source/check file = 106 lines.

Evidence:
  exact caller/symbol absence; cargo check --lib; legacy terminal 4/4;
  reentrant nested/failure/reuse 10/10; source admission 1/1; children 7/7;
  existing children guard; all touched source/check files below 800.

R4:
  RAW-ROOT-LEGACY-BRANDED-TERMINAL-SUNSET-001 = closed.
  retain-fenced=3, active compatibility=1, closed=8, unregistered=7.
  replacement credit=0; production/grammar/result/route delta=0.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS34-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS33-D0` — read-only census, closed

```text
LegacyChildDraftAdmissionV1:
  32 occurrences / 6 src/mir files.

Crosswalk:
  production vocabulary                     = 2 occurrences
  production neutral terminals              = 5 occurrences
  production live nested issuers             = 3 occurrences
    (one import + exact static/instance constructors)
  cfg(test) evidence                         = 22 occurrences

Live source:
  nested ordinary static method
  nested instance constructor / method
  -> NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001, RETAIN-FENCED.
  Exact function-relative source transport remains absent; no safe REOWN.

Selected detached residue:
  complete_legacy_child_branded              = definition only
  commit_legacy_pending_branded              = definition only
  disposition                                = RET0
  production / grammar / route delta         = 0
  replacement credit                         = 0

Excluded:
  resolved/canonical caller-zero terminals; eight raw port-dropping facades;
  phi observer; raw static Main; JoinModule families. Each remains a separate
  responsibility and requires a fresh census or named D0.

Next:
  RAW-ROOT-LEGACY-BRANDED-TERMINAL-RESIDUE0-RET0.
```

`RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0` — T2 REOWN, closed

```text
Production:
  explicit raw-root helper schedule / callable Main
  -> one existing RawSourceLocatorV1 + typed demand role
  -> unchanged LegacySymbol / symbol / arity collector projection.

Deleted:
  module_invocation_brand0 direct LegacyChildDraftAdmissionV1 issuer/import;
  legacy-admission version of the sole branded static-child terminal.

Preserved:
  lexical helper order; helpers before callable Main; request -> reserve ->
  child -> ledger complete; prefix/abort; LegacyReplaceWholePair; candidate
  discard and fresh compiler reuse. No second locator, duplicated physical
  fields, catalog widening, grammar, result, fallback, retry, or reselection.

Measured:
  LegacyChildDraftAdmissionV1 35 occurrences / 7 src/mir files
  -> 32 occurrences / 6 files.
  largest touched source/check file = recursive_child_lowering.rs, 791 lines.

Evidence:
  cargo check --lib; source-keyed admission 1/1; raw children 7/7;
  callable Main 3/3; receipt ledger 11/11; raw public ingress 6/6;
  raw physical 2/2; reentrant failure/reuse 1/1; children and public-ingress
  guards; current-state pointer guard.

R4:
  RAW-ROOT-STATIC-CHILD-DRAFT-COMPAT-SUNSET-001 = closed.
  retain-fenced=3, active compatibility=1, closed=7, unregistered=7.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS33-D0.
```

`RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-D0` — T2 design, accepted

```text
Named production caller:
  explicit raw-root helper schedule and callable-Main compatibility
  -> RawRootPhysicalStateV1
  -> InvocationPhysicalStateV1::complete_raw_static_child.

Existing source authority:
  RawSourceLocatorV1 = top-level statement + Box name + method name, with
  source-verified symbol and arity projections. OwnedRawSourceV1 directly
  indexes the Program and co-seals the declaration into
  RawRootStaticChildWorkV1 before ledger or Builder effects.

New owner:
  RawRootStaticChildDraftAdmissionV1 owns the existing locator by value plus
  exactly one demand role:
    StaticHelper { schedule_ordinal }
    CallableMain
  It creates no second locator, source identity, catalog row, or duplicated
  physical symbol/arity fields.

Terminal:
  helper and callable-Main typed consuming constructors issue the role;
  the shared physical terminal captures the unchanged body once, then consumes
  the admission into LegacySymbol(symbol), symbol, and arity exactly once.
  ModuleLoweringPortV1 applies unchanged LegacyReplaceWholePair.

Preserve:
  lexical helper schedule; helpers before callable Main; request -> reserve ->
  child -> ledger complete; prefix/abort evidence; candidate-only publication;
  last-completed whole-pair replacement; fresh compiler reuse.

Atomic delete:
  module_invocation_brand0.rs direct
  LegacyChildDraftAdmissionV1::legacy_symbol(work.symbol(), work.arity())
  and its now-unused import; legacy-admission version of the sole branded
  static-child terminal.

Non-claims:
  Main.main has separately materialized equal locators for root and callable
  demands; repository-wide unique locator issuance is not claimed.
  Raw static-Main and nested Box compatibility are separate fences.

Structure:
  new raw_root_static_child_admission.rs owns only source admission/projection;
  recursive_child_lowering.rs replaces, rather than stacks on, its sole branded
  legacy terminal and remains below 800; no new test/check file; existing
  children guard is extended.

Next:
  RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0, one atomic T2 commit.
```

`RAW-DRAFT-DISCONNECTED-PROOF-RETIRE0-RET0` — T1 detached deletion, closed

```text
Deleted:
  raw_draft_invocation.rs                         396 lines
  raw_draft_invocation_p0.rs                       98 lines
  cut0_i0_root0_raw_source0_lower_s0_guard.py     119 lines
  builder/compiler wiring and checks-index row     15 lines

Production:
  MirCompiler::begin_raw_draft callers before deletion = 2, both cfg(test).
  production caller / behavior / grammar / route delta = 0.
  replacement credit = 0.

Preserved:
  SourceBoundRawPackageV1; raw root source/planning; raw expansion receipt
  ledgers; RawRootPhysicalStateV1; selected normal and explicit raw routes.

R4:
  RAW-DRAFT-DISCONNECTED-PROOF-SUNSET-001 = closed.
  LegacyChildDraftAdmissionV1 = 35 occurrences / 7 src/mir files.
  retain-fenced=3, active compatibility=1, closed=6, unregistered=7.

Evidence:
  exact absence/census; cargo check --lib; raw expansion receipt ledger 11/11;
  raw root physical 2/2; active public-ingress guard; pointer guard.

Next:
  RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS32-D0` — read-only census, closed

```text
Decision:
  select one detached RET0 before the next production REOWN.

Selected asset:
  RawDraftInvocationV1 / RejectedRawDraftInvocationV1.
  MirCompiler::begin_raw_draft has exactly two callers, both cfg(test) in the
  dedicated raw_draft_invocation_p0 fixture. Production caller = 0.

Atomic RET0:
  delete raw_draft_invocation.rs and raw_draft_invocation_p0.rs;
  delete builder registration/re-export and MirCompiler::begin_raw_draft;
  delete its dedicated guard and checks-index row; update stale positive guard
  assertions. Shared raw source projection and receipt ledgers remain.

Measured reduction:
  source/test Rust -494 lines before incidental wiring changes;
  LegacyChildDraftAdmissionV1 37 occurrences / 8 files
  -> 35 occurrences / 7 files.

Not replacement credit:
  no named production caller changes. This is a detached RET0 asset deletion.

Raw static Main:
  RETAIN-FENCED. Its direct RawLegacy owner edge is one, selected-normal
  reachability is zero, but arbitrary-AST RawLegacy callers prevent a safe
  reachability-zero claim. Helper-first ordering, delayed Main diagnostic, and
  LegacyEnvironment semantics also prevent a local REOWN.

Following production candidate:
  the explicit raw-root static-child issuer already receives an exact
  RawSourceLocatorV1 and may admit a bounded source-keyed REOWN after a fresh
  D0. Do not mix it into this detached deletion.

Next:
  RAW-DRAFT-DISCONNECTED-PROOF-RETIRE0-RET0.
```

`NESTED-BOX-SOURCE-OCCURRENCE0-D0` — T2 design, closed

```text
Decision:
  NoSafeSourceTransport; RETAIN-FENCED.

Reusable vocabulary:
  SourceNodeSiteV1 / SourcePathSegmentV1 can describe one function-relative
  structural path. They do not by themselves issue an occurrence.

Missing production seam:
  RawInvocationChildPortV1 retains only ModuleLoweringPortV1. Raw body,
  statement, expression, If/Loop/scope/Lambda, and Box terminals consume bare
  ASTNode values without enclosing source owner, body index, or child role.
  A nested-only wrapper therefore cannot prove one exact source occurrence.

Rejected:
  symbol/arity as source identity; Span identity; name matching; AST pre-scan
  event queue; clone/reparse; root catalog widening; a renamed
  LegacyChildDraftAdmissionV1; constructor-as-method role collapse.

Required future product:
  one function-root-relative located raw transport, issued at the existing
  recursive traversal and preserved through every child portal. Only after a
  fresh RAW-LOCATED-BODY-TRANSPORT0-D0 names a bounded production edge and its
  same-series deletion may nested StaticMethod / InstanceConstructor /
  InstanceMethod occurrences co-seal the unchanged physical symbol, arity,
  LegacyReplaceWholePair, depth-first order, and candidate-only publication.

Execution:
  S0 = 0; I0/R0 = 0. Building a caller-zero location substrate now would repeat
  the proof-only route failure.

R4:
  NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001 becomes RETAIN-FENCED.
  active compatibility=1, retain-fenced=3, closed=5, unregistered=7.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS32-D0.
```

`NESTED-BOX-RAW-BODY-DISPOSITION0-D0` — T2 design, closed

```text
Decision:
  NoSafeI0; exact prerequisite source authority selected.

Live selected-normal edge:
  active callable body
  -> RawInvocationChildPortV1
  -> nested non-Main static / instance Box
  -> two LegacyChildDraftAdmissionV1::legacy_symbol issuers.

Covered roles:
  NestedStaticMethod
  NestedInstanceConstructor
  NestedInstanceMethod.

Missing capability:
  enclosing callable identity + exact recursive source site/path + nested Box
  occurrence. Root callable catalog excludes nested body declarations, while
  physical symbol/arity alone cannot identify the source occurrence.

Preserve:
  nested Main pre-effect rejection; sync rejection; static method sort; instance
  metadata -> constructors -> methods order; receiver physical arity; depth-first
  child-before-parent collection; LegacyReplaceWholePair parity; candidate-only
  publication and failure discard.

Reject:
  type-renaming LegacyChildDraftAdmissionV1; root catalog widening; AST pre-scan,
  clone, or reparse; constructor/method authority collapse; retry/fallback.

R4:
  R4-UNREGISTERED-NESTED-BOX-RAW-BODY-001 is promoted to
  NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001.
  active compatibility=2, retain-fenced=2, closed=5, unregistered=7.

Next:
  NESTED-BOX-SOURCE-OCCURRENCE0-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS31-D0` — read-only census, closed

```text
Decision:
  NoSafeSlice.

Raw static Main:
  RawLegacyChildLoweringPortV1
  -> PreparedRawStaticMainBoxCompatibilityV1
  -> sorted helpers first
  -> Missing / NotFunction / LegacyEnvironment Main.

Selected normal:
  RawInvocationChildPortV1
  -> raw_invocation_main_box fail-fast.
  Prepared compatibility reachability = 0.

Blocker:
  heterogeneous non-test RawLegacy facades still exist, but a complete
  static-Main production reachability map is absent. RET0 is unproven; direct
  normal-Main reuse would change source authority and helper/root ordering.

R4 registry:
  active compatibility=1, retain-fenced=2, closed=5, unregistered=8.
  LegacyChildDraftAdmissionV1 remains 37 occurrences / 8 source files and
  still requires the mandatory per-site disposition crosswalk before C0.

Next:
  NESTED-BOX-RAW-BODY-DISPOSITION0-D0, because the LegacyChild census found
  two higher-priority selected-normal live issuers in nested Box descent.

Parked:
  RAW-STATIC-MAIN-COMPAT-BATCH-DISPOSITION0-D0 remains required before R4 C0.
```

`NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-I0-R0` — T1 atomic
replacement, closed

```text
Production:
  exact 9 selected Script unsupported kinds
  -> DirectSelectedUnsupportedStatement
  -> current span
  -> existing raw expression recursion guard
  -> shared unsupported raw-AST diagnostic.

Atomic delete:
  exact 9
  -> StatementControlCompatibility / DeclarationIngressCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Parity:
  all 9 normal errors equal legacy errors; declaration-fact preparation order,
  recursion-depth precedence, candidate discard, and fresh compiler reuse are
  unchanged. Successful MIR effect, port demand, retry, fallback, and grammar
  delta are zero.

R4:
  selected Script compatibility residual = 0.
  NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003 = closed.
  active compatibility=1, retain-fenced=2, closed=5, unregistered=8.

Evidence:
  focused non-Box disposition, direct-owner, and runtime-work tests;
  shared public-ingress guard; cargo check --lib; all touched source/check
  files below 800 lines.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS31-D0.
```

`NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-D0` — T1 design,
closed

```text
Decision:
  one exact selected-Script unsupported-statement diagnostic terminal.

Named caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected surface:
  LoopRange, Break, Continue, ImportStatement, BuildGate, EnumDeclaration,
  BrandDeclaration, TypeAliasDeclaration, GlobalVar.

Current outcome:
  each kind passes the raw expression recursion guard and reaches the same
  final Unsupported AST node diagnostic; successful MIR effect = 0.

Selected owner:
  DirectSelectedUnsupportedStatement
  -> align current statement span
  -> existing with_legacy_expression_recursion_guard_v1
  -> one shared unsupported raw-AST diagnostic factory.

Atomic delete:
  exact 9 selected Script kinds
  -> StatementControlCompatibility / DeclarationIngressCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Preserve:
  Program declaration-fact preparation before runtime; recursion-depth error
  precedence; exact Debug-format diagnostic; candidate discard and compiler
  reuse. Raw/reference and nested loop/body routes are non-claims.

Forbid:
  emit_void; declaration installation in the terminal; LoopRange/exit semantic
  activation; AST rewrite; port demand; retry/fallback; wildcard source set.
```

`MIRBUILDER-LIVE-EDGE-CENSUS30-D0` — read-only census, closed

```text
Finding:
  no remaining kind has a successful direct runtime owner, but all exact nine
  selected Script roots share one effect-free final raw rejection.

Decision:
  behavior-neutral diagnostic REOWN is the next live T1. It closes the
  selected Script RawCompatibility execution surface without claiming nested
  LoopRange/Break/Continue semantics or accepting declaration ingress.

R4:
  NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003 can close in the same I0/R0
  when selected Script RawCompatibility execution reaches zero.

Next:
  NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-I0-R0.
```

`NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-I0-R0` — T1 atomic replacement,
closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected path:
  FastMemRegion
  -> DirectFastMemRegion
  -> lower_direct_fastmem_region_v1
  -> existing build_fastmem_region_with_port_v1.

Atomic delete:
  selected Script FastMemRegion
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Preserved:
  exact source span; contract/body/span transport; same RawInvocation port;
  register -> push -> source-order body -> pop; candidate-local region metadata;
  typed body error and outer-region restoration.

Failure:
  typed child failure pops the inner region, rejects and discards the candidate,
  and the same compiler accepts a fresh FastMem request. Panic unwind cleanup
  and candidate-internal metadata rollback remain non-claims.

Evidence:
  disposition tests; full normal/legacy MIR and verification parity; FastMem
  metadata/MemOp tests; typed cleanup test; direct-owner 6/6; runtime mapping;
  shared guard; cargo check --lib; diff check.

Residual:
  10 -> 9 exactly.

Structure:
  runtime work = 799; direct owner = 649; disposition = 237;
  FastMem region tests = 179; shared guard = 799.
  New source/test/check files = 0.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS30-D0.
```

`NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-D0` — T1 design, closed

```text
Decision:
  one direct selected Script FastMemRegion admission.

Named caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected existing owner:
  build_fastmem_region_with_port_v1.

Atomic delete:
  selected Script FastMemRegion
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Contract:
  exact FastMemRegion source; statement span aligned once; contract, body,
  source span, and the same RawInvocation port passed once to the existing
  register -> push -> body -> pop lifecycle owner.

Failure:
  register fails before push; typed body failure still pops the inner region
  and candidate isolation prevents metadata or Builder publication. Panic
  unwind cleanup and rollback inside the discarded candidate are non-claims.

Evidence:
  full normal/legacy MIR and metadata parity; same-port body order; typed
  body failure cleanup; candidate discard and fresh compiler reuse.

Exclude:
  fastmem lifecycle rewrite; metadata duplication; fresh child port; nested
  body reclassification; retry/fallback; new source/failure/compat owner.
```

`MIRBUILDER-LIVE-EDGE-CENSUS29-D0` — read-only census, closed

```text
Selected:
  FastMemRegion is the sole safe live T1 edge. Its existing port-aware owner
  already owns register -> push -> same-port body -> pop and metadata.

NoSafeSlice:
  LoopRange / Break / Continue require loop/exit/CFG authority.
  ImportStatement / BuildGate / EnumDeclaration / BrandDeclaration /
  TypeAliasDeclaration / GlobalVar have no equivalent direct runtime owner.

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 remains a separate 37-occurrence / 8-file
  census metric; it is not a fence count.

Next:
  NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-I0-R0.
```

`NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-I0-R0` — T1 atomic replacement,
closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected path:
  If
  -> DirectIfStatement
  -> lower_direct_if_statement_v1
  -> drive_raw_if_statement_with_port_v1
  -> existing IfForm
  -> complete_if_statement_v1.

Atomic delete:
  selected Script If
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Preserved:
  exact statement span; same RawInvocation port; condition then optional-else
  demand order; unknown-span Program branch shells; CFG/PHI/JoinIR behavior;
  success-only Void completion; branch termination and suffix stop.

Failure:
  condition/then/else failures reject the candidate without another route;
  live Builder remains unpublished and the same compiler accepts a fresh If.

Evidence:
  direct-owner 5/5; disposition 2/2; If-descent 7/7; runtime-work 6/6;
  If-parity focused tests; shared guard; cargo check --lib; diff check.

Residual:
  11 -> 10 exactly.

Structure:
  runtime work = 794; direct owner = 585; disposition = 225;
  shared guard = 799. New source/test/check files = 0.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS29-D0.
```

`NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-D0` — T1 design, closed

```text
Decision:
  Candidate A — one direct selected Script statement-If admission.

Named caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected existing owner:
  drive_raw_if_statement_with_port_v1
  -> existing IfForm
  -> complete_if_statement_v1.

Atomic delete:
  selected Script If
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Contract:
  exact If source; statement span aligned once; condition then branch and
  optional else branch demanded through the same RawInvocation port; existing
  unknown-span Program branch shells and Void completion preserved.

Evidence:
  no-else / else / nested-body full MIR parity; condition, then, and else
  failure ordering; branch termination and suffix stop; candidate discard and
  compiler reuse.

Exclude:
  FastMemRegion region lifecycle; LoopRange/Break/Continue loop-exit authority;
  Import/BuildGate/declaration ingress; new CFG or completion semantics.

Structure:
  normal_script_runtime_work.rs and the shared guard begin at 799 lines.
  The atomic row must include only meaning-neutral local compaction sufficient
  to keep both files below 800; no new source/check file.

Next:
  NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-I0-R0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS28-D0` — read-only census, closed

```text
Exact residual:
  StatementControl = If / LoopRange / Break / Continue / FastMemRegion.
  DeclarationIngress = Import / BuildGate / Enum / Brand / TypeAlias / Global.

Bounded owners:
  If -> existing raw statement-If descent and completion.
  FastMemRegion -> existing region owner, but it requires an independent
  register/push/body/pop and metadata parity row.

No direct equivalent:
  LoopRange / Break / Continue currently terminate in raw unsupported or
  loop-frame-specific authority.
  All six ingress kinds currently terminate in raw unsupported; Enum/Brand
  declaration facts do not make runtime completion Void. Direct no-op would
  change behavior.

Selection:
  If alone, T1. No multi-kind bulk cutover.

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 = 37 occurrences / 8 src/mir files, separately.
```

`NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-I0-R0` — T1 atomic
replacement, closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Atomic delete:
  Assignment / CompoundAssignment / Loop / Nowait / TaskScope / ContextScope /
  TryCatch / Throw / Local / ScopeBox / Outbox / Program / UsingStatement
  -> RawCompatibility
  -> drive_legacy_statement_v1
  -> build_statement_with_port_v1 fallthrough
  = 0.

Selected path:
  DirectPortAwareExpression
  -> drive_legacy_expression_v1 with the same RawInvocation port
  -> the unchanged raw-expression statement-surface terminal.

Parity:
  all 13 roots compare exact normal/legacy success or diagnostic; successful
  rows compare full MirPrinter output and verification_result. Existing Return
  suffix termination and failure/reuse evidence remain green.

Residual:
  24 -> 11 exactly.
  StatementControl = If / LoopRange / Break / Continue / FastMemRegion.
  DeclarationIngress = ImportStatement / BuildGate / EnumDeclaration /
  BrandDeclaration / TypeAliasDeclaration / GlobalVar.

Non-delta:
  grammar/result/verification/publication/raw-reference/fallback/retry = 0.

Evidence:
  shared cutover guard; direct-owner 4/4; disposition 2/2; runtime-work 4/4;
  statement-surface/task-scope focused tests; cargo check --lib; diff check.

Structure:
  direct owner = 467 lines; disposition = 223; shared guard = 799.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS28-D0.
```

`NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-D0` — T1 design, closed

```text
Decision:
  Candidate A — delete one shared statement-to-expression adapter for the
  complete StatementSurfaceFallthrough0 set.

Exact set:
  Assignment / CompoundAssignment / Loop / Nowait / TaskScope / ContextScope /
  TryCatch / Throw / Local / ScopeBox / Outbox / Program / UsingStatement.

Structural proof:
  RawInvocationChildPortV1::lower_statement
  -> build_statement_with_port_v1
  -> none of {If, StaticConstTable, FastMemRegion}
  -> drive_legacy_expression_v1 with the same port
  -> raw expression statement_surface exact owner.

New path:
  DirectPortAwareExpression
  -> drive_legacy_expression_v1 with the same RawInvocation port
  -> the same raw expression statement_surface exact owner.

This is one adapter responsibility:
  inner Assignment/place, Loop/CFG, async, scope/body, exception, binding,
  ContextScope diagnostic, Program body, and Using Void owners are not grouped,
  copied, or changed. They remain the terminal authorities in both paths.

Atomic delete:
  the 13 selected roots -> RawCompatibility -> drive_legacy_statement_v1 = 0.
  Residual 24 -> 11.

Exact residual:
  StatementControl = If / LoopRange / Break / Continue / FastMemRegion.
  DeclarationIngress = Import / BuildGate / Enum / Brand / TypeAlias / Global.

Forbid:
  If/FastMem special-arm bypass; a blanket AST match; a second dispatcher or
  port; terminal-specific semantic edits; source allowlists; fallback/retry;
  raw/reference widening; new source/check file; any file reaching 800.

Evidence:
  exhaustive classifier set; old dispatcher three-special-arm guard; parity
  matrix across all 13 terminal families; ContextScope exact diagnostic;
  child/terminal failure and fresh reuse; body/suffix/termination/source-order;
  same-port nested call/Box/Loop behavior.

Next:
  NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-I0-R0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS27-D0` — read-only census, closed

```text
Selected Script residual:
  StatementControlCompatibility = 16
  DeclarationIngressCompatibility = 8
  total = 24.

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 = 37 occurrences / 8 src/mir files, separately.

Selection:
  the exact 13-kind StatementSurfaceFallthrough0 structural set. Single-kind
  Using or Nowait rows are rejected as unnecessarily fine-grained because the
  outer adapter relation is identical and terminal owners remain independent.
```

`NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-I0-R0` — T1 atomic
replacement, closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

New selected path:
  StaticConstTable
  -> DirectStaticConstRuntimeCompletion
  -> normal_script_direct_statement_owner
  -> exact source check
  -> statement span
  -> emit_void.

Atomic delete:
  selected Script StaticConstTable
  -> DeclarationIngressCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Authority:
  PreparedNormalProgramStaticTableMetadataV1 remains the sole metadata owner;
  its prepare/commit still precedes work-plan/runtime exactly once. The direct
  runtime helper has no metadata read, reconstruction, validation, or commit.

Evidence:
  disposition and runtime partitions are exhaustive; valid table compilation
  preserves full module/verification outcome and table span; metadata pair
  ordering and failed-prepare atomicity tests remain green; shared guard fixes
  pre-runtime ordering and forbids metadata access in the direct helper.

Result:
  residual 25 -> 24; DeclarationIngress 9 -> 8; StatementControl remains 16;
  new source/metadata owner, port, route, grammar, publication, fallback,
  retry = 0.

Structure:
  direct owner 342 lines; disposition 223; runtime work 799; shared guard 799.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS27-D0.
```

`NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-D0` — T1 design, closed

```text
Decision:
  Candidate A — direct selected-Script runtime completion for StaticConstTable.

Existing authority:
  PreparedNormalProgramDeclarationFactsV1 and
  PreparedNormalProgramStaticTableMetadataV1 complete source-order metadata
  preparation and atomic commit exactly once before Program work-plan/runtime.

Old runtime path:
  StaticConstTable
  -> DeclarationIngressCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  -> statement span
  -> emit_void.

New runtime path:
  DirectStaticConstRuntimeCompletion
  -> existing normal_script_direct_statement_owner sibling
  -> exact StaticConstTable check
  -> statement span
  -> existing emit_void.

Atomic delete:
  selected Script StaticConstTable -> drive_legacy_statement_v1 = 0.
  Residual 25 -> 24; DeclarationIngress 9 -> 8; StatementControl stays 16.

Forbid:
  metadata read/rebuild/revalidation/recommit in runtime; DirectPortAwareExpression
  misuse; child port; AST clone; Enum/Brand/Using or another ingress kind;
  result/grammar/publication change; retry/fallback; new source/check file.

Evidence:
  exact disposition partition; metadata prepare/commit precedes runtime; table
  span owns one Void completion; multiple tables preserve metadata/runtime
  source order and scalar tail; invalid metadata fails before runtime and fresh
  request reuses compiler; shared guard and all files below 800.

Next:
  NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-I0-R0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS26-D0` — read-only census, closed

```text
Selected Script residual:
  StatementControlCompatibility = 16
  DeclarationIngressCompatibility = 9
  total = 25

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 = 37 occurrences / 8 src/mir files, separately.

Decision:
  StaticConstTable is the only next zero-child metadata/runtime-completion
  responsibility. Remaining control and ingress kinds retain distinct CFG,
  binding, scope, exception, import, or declaration authorities.
```

`NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-I0-R0` — T1 atomic replacement, closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Change:
  ASTNode::Return is classified as DirectPortAwareExpression and reaches the
  existing raw expression statement-surface Return owner through the same
  RawInvocation port.

Atomic delete:
  selected Script Return
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Preserved:
  root span; return; Void completion; arbitrary value-child descent; cleanup;
  Match-return; defer; emitted Return; block termination and suffix stop;
  diagnostics; candidate discard; fresh compiler reuse.

Evidence:
  direct root parity covers void and FunctionCall-bearing value Return;
  a Return followed by an undefined-variable Print proves suffix suppression;
  failing value lookup followed by fresh success proves reuse; existing raw
  Return descent suites, shared guard, cargo check, and pointer guard are green.

Result:
  residual 26 -> 25; StatementControl 17 -> 16; DeclarationIngress remains 9;
  new owner/product/route/grammar/result/fallback/retry = 0.

Structure:
  direct owner 323 lines; disposition 210; runtime work 789; shared guard 799.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS26-D0.
```

`NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-D0` — T1 design, closed

```text
Decision:
  Candidate A — route every ASTNode::Return through the existing
  DirectPortAwareExpression terminal.

Old selected path:
  Return
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  -> build_statement_with_port_v1
  -> raw expression statement-surface Return owner.

New selected path:
  Return
  -> DirectPortAwareExpression
  -> drive_legacy_expression_v1 with the same RawInvocation port
  -> the same raw expression statement-surface Return owner.

Preserved owners:
  return; uses build_void_return_statement;
  return value uses drive_value_return_statement_v1, including cleanup
  preflight, Match-return probe, one arbitrary value-child descent, defer, and
  emit_return_from_value. Block termination and suffix stopping remain in the
  unchanged block driver.

Atomic delete:
  selected Script Return -> drive_legacy_statement_v1 = 0.
  Residual 26 -> 25; StatementControl 17 -> 16; DeclarationIngress stays 9.

Forbid:
  a Return operand allowlist; a second port; custom Return semantics; new
  owner/product/route/failure; fallback/retry; StaticConstTable or another
  statement responsibility in this row.

Evidence:
  void/value/arbitrary-child full MIR and verification parity; exact diagnostic
  parity; Return span; Match-return and termination/suffix behavior; late
  failure then fresh compiler reuse; shared guard and all files below 800.

Next:
  NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-I0-R0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS25-D0` — read-only census, closed

```text
Selected Script residual:
  StatementControlCompatibility = 17 exact kinds
  DeclarationIngressCompatibility = 9 exact kinds
  total = 26

Retired category:
  CallObjectHeaderCompatibility live source occurrence = 0.

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 remains 37 occurrences / 8 src/mir files and is
  an independent observation metric, not a fence count.

Safe independent candidates:
  Return and StaticConstTable. Return is selected because it reaches an
  existing exact owner through the already selected expression terminal;
  StaticConstTable retains a separate metadata-before-runtime completion
  contract.
```

`NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-I0-R0` — T1 atomic
replacement, closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement

Selected responsibility:
  QMarkPropagate / MatchExpr / EnumMatchExpr / ArrayLiteral / MapLiteral /
  RecordLiteral / RecordUpdate / Lambda / BlockExpr / Arrow /
  GroupedAssignmentExpr / MethodCall / FieldAccess / Index / New / This /
  FromCall / ThisField / MeField / FunctionCall / Call.

New path:
  DirectPortAwareExpression
  -> existing normal_script_direct_statement_owner
  -> drive_legacy_expression_v1 with the same RawInvocation port.

Atomic delete:
  CallObjectHeaderCompatibility = 0;
  the 21 roots -> RawCompatibility -> drive_legacy_statement_v1 = 0.

Parity:
  representative call/object/allocation/control/nested-function roots compare
  full MirPrinter + verification on success and exact diagnostics on failure;
  existing root-span, nested FunctionCall, late-failure, and compiler-reuse
  evidence remains green.

Result:
  selected Script compatibility residual 47 -> 26 exact kinds;
  compatibility terminal count remains one; new owner/route/grammar/result/
  publication/fallback/retry = 0.

Structure:
  direct owner 250 lines; disposition 198; runtime work 789; shared guard 799.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS25-D0.
```

`JOINMODULE-REFERENCE-ASSET-DISPOSITION0-D0` — T2 disposition, closed

```text
normal/default JoinModule execution = 0

REOWN (separate CorePlan/MIR rows):
  CFG/boundary carriers (`JumpArgsLayout`, `JoinInlineBoundary`, carrier and
  loop-scope facts); finalization type helpers; shared operator/error policy.

RETIRE (after each named closure):
  normalized-shadow emission/observation, direct JoinIR runner, and cfg(test)
  VM-bridge handlers. They are not final planners or acceptance truth.

RETAIN-FENCED:
  JoinIR model/lowering/JSON/format only while required by the explicit
  `NYASH_JOINIR_VM_BRIDGE` VM route or LLVM experiment gates. Neither is a
  normal/default route; their sunset is decided before R4.

DERIVEDSHADOW:
  all 48 manifests are `mainline_selected=0`. Retire the direct stale
  `condition_fn_injection` bundle now; reown/refresh bounded-finalize,
  function-region-stack, and aggregate evidence that named its deleted edge;
  retain the remaining 45 caller-zero reference families. Raw root condition
  drafts are a separate live owner and are excluded.
```

## Current design decision

`JOINMODULE-PHI-RETURN-STRATEGY-REOWN0-D0` — T2, closed

```text
Decision: Candidate A — Builder-finalization return-type strategy rehome.

The sole live consumer is finalize_module.  The existing strategy moves as one
Builder sibling; JoinIR's TypeHintPolicy and GenericTypeResolver are deleted
with their exports, not re-exported.  This corrects the stale P3-D-only card:
the actual policy is one ordered finalization owner.

Observed order (must not move):
  Direct value type -> primary name hint -> P3-D known definition
  -> P4 PhiTypeResolver -> P3-C uniform-PHI fallback.

No route, grammar, result, publication, fallback/retry, Ownership/View, or
feature delta.  VM/LLVM, normalized-shadow, Loop/CorePlan, JumpArgsLayout,
JoinInlineBoundary, and all remaining R3 fences stay separate.
```

## Closed design decision

`JOINMODULE-NORMALIZED-SHADOW-RETIRE0-D0` — RETAIN-FENCED

```text
Default normal compilation has normalized-shadow execution = 0, but explicit
JoinIR dev/debug reaches two direct execution sites and a body observer:

  1. cf_loop_joinir_impl -> try_normalized_shadow
  2. drive_legacy_block_v1 -> NormalizedShadowSuffixRouterBox
  3. strict/dev StepTree observer (diagnostic only)

Fence: JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0
  - selected normal authority = 0
  - compatibility expansion = 0
  - new fallback/retry approval = 0
  - JoinInlineBoundary and JumpArgsLayout ownership move = 0

Sunset: remove both direct execution edges only after one verified
Recipe/CorePlan owner covers their loop shapes, strict/dev parity is green,
and the observer contract is independently disposed. The explicit VM/Stage1/
StageB reference consumers are handled only by the next reference-sunset D0.
```

## Census13 disposition

`MIRBUILDER-LIVE-EDGE-CENSUS13-D0` — closed, NoSafeLiveI0

```text
Selected normal/default is one candidate-session -> collector -> finalization
route. It has no RawLegacyChild port, raw driver, build_module edge, or safe
competing live authority left to switch atomically.

Separate D0 boundaries, not executable I0s:
  raw/static-Main callable compatibility (env policy can create Main.main/N)
  header-sensitive Global Call result policy
  selected-invocation Loop/CorePlan
  If/JoinIR control

RETAIN-FENCED:
  explicit VM bridge/LowerOnly, normalized-shadow dev route, LLVM experiment,
  frontend metadata, JumpArgsLayout/JoinInlineBoundary carriers.

R3 selection:
  one cfg(test)-only return-collector asset may retire. This is the first
  detached retirement after the preceding live I0/R0; it earns no replacement
  credit. After it closes, Census14 is mandatory before another selection.
```

## Latest closeout

`JOINMODULE-RETURN-COLLECTOR-TEST-ASSET-RET0` — T1 detached R3 retirement

```text
Change:
  `join_ir/lowering/return_collector.rs`, its cfg(test) module declaration, and
  five stale current control-flow inventory rows = 0.

Contract:
  The asset had no external Rust consumer beyond that cfg(test) declaration;
  return semantics, normal/default, VM bridge, LLVM, routes, and fences stayed
  unchanged.

Done:
  Source/current-inventory references = 0; lib/vm-reference and reusable
  lane/pointer guards = green.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS14-D0`.
```

## Census14 disposition

`MIRBUILDER-LIVE-EDGE-CENSUS14-D0` — closed

```text
Selected normal/default remains one Program-only candidate-session -> collector
-> finalization route; no raw legacy port, build_module edge, fallback, or safe
competing live owner is reachable. Therefore NoSafeLiveI0.

R3 has no second detached asset: inline_boundary_builder has Boundary-carrier
test consumers; the other inspected JoinIR surfaces have live/fenced consumers.
Therefore NoSafeDetachedR3.

The only selected next boundary is raw/static Main compatibility. Its module
ingress snapshot can materialize Main.main/N and alter runner entry selection;
it is a policy D0, not an atomic cleanup.
```

## Current design stop

`RAW-STATIC-MAIN-CALLABLE-COMPATIBILITY0-D0` — T2, closed as RETAIN-FENCED

```text
`prepare_module` snapshots `NYASH_BUILD_STATIC_MAIN_ENTRY` once into the sole
`CallableMainCompatibilityPolicyV1`; body lowering never rereads it. Required
materializes `Main.main/N` through the existing port, while Omitted does not.

This is not caller-zero or a decls-only cleanup: selected normal reaches it,
`Main.main/0` changes normal runner entry selection, and `N > 0` retains
explicit-entry semantics. The raw ledger is a separate reference witness, not
the selected normal receipt.

Do not rehome yet. Moving only the conditional would preserve every authority
and make a wrapper, not an in-place replacement.

Sunset requires one explicit entry-materialization request/result contract
consumed by normal, raw/reference, and runner entry selection; only then may a
later row retire the snapshot adapter, compilation-context policy field, direct
lower-side read, and raw ledger/physical disposition together.
```

## Current design stop

`RAW-ENTRY-MATERIALIZATION-CONTRACT0-D0` — T2 policy boundary

```text
Decision: Candidate C — source-owned materialization facts; route-specific
normal/raw receipts; runner-specific selection stays with its existing adapter.

Shared vocabulary:
  `CallableMainMaterializationPolicyV1` plus an exact issued target
  (symbol/arity only). It owns no AST, source identity, brand, collector, or
  runner choice.

Route receipts:
  normal: ingress snapshot -> Program expansion source receipt -> collector
  completion; raw/reference: existing explicit selection -> raw source receipt
  -> raw physical/ledger completion. Do not combine their brands, drains, or
  runner routes.

Runner boundary:
  receipt proves what functions materialized; it does not choose execution.
  Preserve each current selector, including `NYASH_ENTRY`, MIR/PyVM/mock
  `Main.main/0` preference, native LLVM `Main.main/1`, and raw exact `main/0`.

Compatibility:
  normal Script + Required remains Omitted; raw Script + Required remains its
  existing source rejection. `Main.main/0` is a preference candidate; `N > 0`
  remains explicit invocation, not default entry.

Hard stop: no global runner selector, entry-name inference, AST/config clone,
env reread, second route/collector, public result/JSON change, retry/fallback,
Ownership/View, or feature activation.
```

## Latest closeout

`ENTRY-MATERIALIZATION-RECEIPT0-S0` — T2 prerequisite, closed

```text
Change:
  `CallableMainMaterializationPolicyV1`, symbol/arity target vocabulary, and
  separate normal/raw source receipts are source-only products.

Contract:
  No receipt stores AST/config/brand or selects a runner entry. Builder,
  collector, publication, raw ledger, and runners remain unchanged.

Done:
  normal Script+Required -> Omitted; raw Script+Required has no receipt and
  retains the existing binding rejection; App keeps exact `Main.main/N` facts.

Next:
  `ENTRY-MATERIALIZATION-NORMAL-CONSUMPTION0-I0-R0`.
```

## Latest closeout

`ENTRY-MATERIALIZATION-NORMAL-CONSUMPTION0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Named caller:
  NormalDefaultPublishedPipelineV1::compile

Change:
  snapshot the materialization policy once at normal ingress; thread the sealed
  normal receipt through the existing one-session lifecycle; delete the selected
  normal lower-side environment snapshot/materialization decision.

Keep:
  raw/reference source receipts and physical ledger, all runner selectors,
  result/publication policy, and the existing candidate reuse contract.

Evidence:
  Required/Omitted x Script/App x Main.main/0/nonzero; exact symbol/arity;
  helper -> callable -> root order; failure leaves the live Builder reusable.

Kept:
  global compatibility-field deletion, raw/reference consumption, runner-policy
  changes, a second route/collector, AST/config duplication, reread/retry,
  Ownership/View, or feature work.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS15-D0` — read-only, closed

```text
Inventory:
  selected normal, raw/reference, and runner materialization consumers.

Result:
  selected normal has no safe immediate I0/R0; raw/reference receipt has
  production consumer=0; runner selectors remain independent fences.

Selected D0:
  `NORMAL-RUNTIME-INPUT-SNAPSHOT0-D0`.
```

## Current design decision

`NORMAL-RUNTIME-INPUT-SNAPSHOT0-D0` — T2, closed

```text
Decision:
  Candidate N — infallible normal-only ingress receipt.

Normal preserves its current permissive contract: only untrimmed case-insensitive
1/true/on enables the entry safepoint; absent, empty, malformed, wrong-typed,
or empty script-argument JSON means no pushed arguments and no diagnostic.
NYASH takes precedence over HAKO even when its value is malformed.  Raw's strict
snapshot remains separate because it rejects malformed input and carries distinct
provenance.
```

## Latest closeout

`NORMAL-RUNTIME-INPUT-SNAPSHOT0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  NormalDefaultPublishedPipelineV1 captures NormalRuntimeInputSnapshotV1 once;
  the existing candidate lifecycle consumes it for entry safepoint and Main
  wrapper arguments.  Selected lower-side reads = 0.

Result:
  NYASH/HAKO precedence, permissive malformed values, App/Script behavior,
  request-versus-compile timing, failure/reuse, raw static-Main compatibility,
  normal/vm-reference checks, and the reusable ingress guard are green.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS16-D0`.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS16-D0` — closed

```text
Result:
  Selected normal has no safe immediate I0/R0.  Its Program root still sends
  Box methods, instance constructors, and top-level functions through
  LegacyChildDraftAdmissionV1; raw receipts, runtime inputs, runners, and
  explicit reference lanes remain fenced.  The
  JoinModule generic Case-A, VM bridge, LowerOnly, and LLVM surfaces also have
  live consumers, so Census16 records them RETAIN-FENCED rather than inventing
  an R3 retirement.

Next:
  `NORMAL-CALLABLE-DRAFT-IDENTITY-AND-ADMISSION0-D0`.
```

## Latest design decision

`NORMAL-CALLABLE-DRAFT-IDENTITY-AND-ADMISSION0-D0` — T2, closed

```text
Inventory:
  The live child port carries catalog-addressable static/instance Box methods,
  uncatalogued instance constructors, and uncatalogued top-level functions.

Decision:
  Candidate C is accepted: first replace catalog-addressable Box methods only.
  Their existing CanonicalSameModuleCallableKeyV1 becomes a source witness and
  seals one physical symbol/arity relation; instance physical arity includes
  exactly one receiver.  The existing one body snapshot, parent restoration,
  LegacySymbol key, and LegacyReplaceWholePair collector policy remain exact.

Reject:
  ResolvedChildDraftAdmissionV1 is not a substitute: it requires an
  invocation-local FunctionOwnerIdV1, a canonical no-body session, and reject-
  duplicate collector policy.  Do not fabricate that owner or change drain
  policy in this cell.

Fence:
  Constructors and top-level functions stay on the existing normal
  compatibility edge.  Their sunset is
  `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001`: each must obtain an
  exact source identity before its branch can move; neither is completion debt
  hidden behind the cataloged-method product.
```

## Latest closeout

`NORMAL-CATALOGED-BOX-METHOD-DRAFT-ADMISSION0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  Main helpers, non-Main static methods, and ordinary instance methods carry
  their existing CanonicalSameModuleCallableKeyV1 through the root port into
  NormalCatalogedBoxMethodDraftAdmissionV1.  The receipt derives the physical
  symbol and arity, including exactly one instance receiver.

Delete:
  Those selected child paths construct LegacyChildDraftAdmissionV1 = 0.  The
  receipt maps once to the unchanged LegacySymbol + LegacyReplaceWholePair
  collector boundary; collector-key replacement remains a later named cell.

Evidence:
  source/physical receipt tests, general-module normal-vs-legacy MIR parity
  including static and instance methods, candidate failure/reuse, raw legacy
  terminal tests, lib/vm-reference checks, and current guards are green.

Residual:
  constructors, top-level functions, optional callable Main, and Script-runtime
  Box descent remain on explicit normal compatibility edges pending independent
  source/port ownership.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS17-D0` — read-only, closed

```text
Result:
  `NormalEntryMaterializationSourceReceiptV1::App` already carries the exact
  `Main.main/N` source target and the installed callable catalog supplies its
  exact static row.  Required callable Main is therefore the one safe selected
  normal I0/R0.  Collector/drain stays RETAIN-FENCED: it still consumes only
  LegacySymbol + LegacyReplaceWholePair and has no old selected-normal edge to
  delete.

Fence:
  Constructors and top-level functions lack source callable owners.  Script
  runtime Box descent is shared raw-port work without normal admission facts.
  Keep them separate; do not fold them into callable Main.  Record
  `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001` for constructors and
  top-level functions, and `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-COMPAT-
  SUNSET-001` for Script runtime Box descent.

Next:
  `NORMAL-CALLABLE-MAIN-MATERIALIZATION-ADMISSION0-I0-R0`.
```

## Latest closeout

`NORMAL-CALLABLE-MAIN-MATERIALIZATION-ADMISSION0-I0-R0` — T1 atomic selected-normal cutover, closed

```text
Change:
  Required App callable `Main.main/N` now proves its receipt target against the
  installed static catalog row, seals NormalCatalogedBoxMethodDraftAdmissionV1,
  and uses the cataloged static port.  Its selected materialization ->
  LegacyChildDraftAdmissionV1 edge = 0; raw policy materialization is unchanged.

Done:
  exact target/catalog fixture, missing-row fail-fast, Required/Omitted normal
  integration, raw static-Main compatibility, candidate/reuse, lib/vm-reference,
  and reusable lane guards are green; `decls.rs` is 449 lines.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS18-D0`.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS18-D0` — read-only, closed: NoSafeLiveI0

```text
Result:
  Script Program runtime re-enters raw Box descent, while constructors and
  top-level functions still have no callable source identity.  Thus no normal
  edge can move without a new source/port decision.  Collector/drain and all
  explicit VM/LLVM/raw fences remain retained.

R3:
  Legacy AST->JoinModule frontend and the cfg(test) legacy handler lane are
  disposition D0 candidates only.  Neither retires until its owned test and
  reference contract is independently resolved.  R4 still decides delete vs
  explicit fenced-reference disposition for the complete old-IR scope.

Next:
  `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-ADMISSION0-D0`.
```

## Latest design decision

`NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-ADMISSION0-D0` — T2, closed: Candidate A

```text
Change:
  Select one Program work-plan-owned, source-order Script runtime receipt and
  a narrow selected-normal Box-callable adapter.  It classifies each runtime
  Box statement exactly once before Builder effects; generic raw ports and the
  raw expression dispatcher remain non-normal authorities.

Contract:
  Preserve Program immediate/runtime order, one body descent, source identity,
  collector mapping, runner selection, and candidate failure/reuse.  Ordinary
  non-Main static and instance methods use their installed exact catalog rows;
  constructors, top-level functions, static Main, nested/raw-reference Box
  descent stay with their separately registered compatibility residuals.

Stop:
  Return if runtime source order cannot coexist with the existing block driver,
  an instance method could enter the collector twice, a generic raw port gains
  normal authority, or a constructor/top-level identity, collector policy,
  second session, fallback, or retry is required.
```

## Latest closeout

`NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-ADMISSION0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  Selected-normal Script runtime uses one source-order admission receipt.
  Ordinary non-Main static methods enter the installed cataloged static port;
  ordinary instance methods retain their runtime declaration prefix but do not
  re-admit already cataloged methods.  Raw/reference uses a neutral statement
  carrier and never constructs the normal admission receipt.

Deleted:
  selected Script runtime direct raw admission for catalog-addressable ordinary
  non-Main static/instance methods = 0.

Done:
  mixed static/instance Script normal-vs-legacy MIR parity, no duplicate
  callable functions, late method failure/fresh reuse, neutral raw carrier,
  lib/vm-reference builds, focused lifecycle/port suites, and shared guards
  are green.  `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-COMPAT-SUNSET-001` is closed.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS19-D0`.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS19-D0` — read-only, closed: NoSafeLiveI0

```text
Result:
  The remaining selected-normal LegacyChild admissions are top-level
  FunctionDeclaration and instance constructors.  The existing callable
  catalog owns only static/instance Box methods, so neither edge can move as a
  T1/I0 replacement.  Raw static-Main remains explicit raw compatibility.

Registry correction:
  Script I0 retired only plain direct Box ordinary-method admission.  Non-plain
  Script Boxes still select raw compatibility and are registered as
  `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-COMPAT-SUNSET-002`; no surface is hidden
  by the closed plain-Box sunset.

Next:
  `NORMAL-TOPLEVEL-FUNCTION-CALLABLE-IDENTITY0-D0`.
```

## Latest design decision

`NORMAL-TOPLEVEL-FUNCTION-CALLABLE-IDENTITY0-D0` — T2, closed: Candidate A

```text
Decision:
  one selected-normal Program-work-plan receipt per top-level
  `FunctionDeclaration`, carrying a source-order occurrence key
  `{ statement_index, declared_name, declared_arity }` and a separately sealed
  physical `{ symbol = name/arity, arity }` admission.

Collector:
  source occurrence identity is distinct from physical collector identity.
  Preserve `LegacySymbol + LegacyReplaceWholePair`, including legacy
  source-order last-wins when two occurrences project to one `name/arity`.
  Body capture, header lookup, parent restoration, result policy, and candidate
  rejection remain existing owners.

Reject:
  widening `VerifiedSameModuleCallableDeclarationCatalogV1` or its Box-method
  namespace; synthetic Box owners; caller-zero normal_source_plan reuse;
  CanonicalRejectDuplicate; a detached S0; raw/reference receipt issuance;
  source reread/reparse/second root scan; fallback or retry.

I0 contract:
  selected normal only replaces
  `PreparedProgramRootTopLevelFunctionWorkV1::lower_with_port_v1`
  -> raw static method -> `LegacyChildDraftAdmissionV1` with one dedicated
  selected top-level capture port.  Constructors, Script non-plain Boxes,
  static Main, and raw/reference retain their registered routes.
```

## Current execution

`NORMAL-TOPLEVEL-FUNCTION-CALLABLE-IDENTITY0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  selected Program top-level functions now issue one source-order occurrence
  receipt and separate legacy physical admission in the existing work plan;
  the selected capture port consumes it while raw/reference retains the raw
  work item.

Delete:
  selected normal top-level FunctionDeclaration -> raw static method
  `LegacyChildDraftAdmissionV1` = 0.

Evidence:
  normal-vs-legacy general-module MIR/function-set parity, including duplicate
  `name/arity` last-wins; source-order/physical receipt tests; late body
  failure then fresh reuse; raw/reference receipt = 0; shared guard; and all
  touched source/check files below 800 lines.

Registry:
  `NORMAL-TOPLEVEL-FUNCTION-CALLABLE-COMPAT-SUNSET-003` is closed.  The older
  combined `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001` is narrowed
  to constructors only, so no active row silently retains top-level scope.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS20-D0`; do not preselect another replacement.
```

## Latest census

`MIRBUILDER-LIVE-EDGE-CENSUS23-D0` — read-only, closed: expression facade selected for D0

```text
Remaining selected compatibility:
  54 exact kinds
  = PortAwareExpression 7
  + StatementControl 17
  + DeclarationIngress 9
  + CallObjectHeader 21

Observed seven-kind edge:
  Literal / Variable / Me / Unary / Binary / Await / Check
  -> NormalScriptRuntimeStatementAdmissionV1::RawCompatibility
  -> drive_legacy_statement_v1
  -> RawInvocationChildPortV1::lower_statement
  -> build_statement_with_port_v1
  -> drive_legacy_expression_v1

Finding:
  build_statement_with_port_v1 has no kind-specific policy for these roots. It
  writes the root span once, then the raw expression dispatcher writes the same
  span and lowers through the same RawInvocation port. Descendant MethodCall,
  New, Field, Call, or control-sensitive shapes therefore do not need a new
  allowlist: they retain the exact selected port they already receive.

Selected design stop:
  NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-D0

Required D0:
  decide one direct drive_legacy_expression_v1 handoff for all seven roots;
  preserve block order/suffix/termination and every existing expression owner;
  delete only the seven-kind statement-facade edge in the later atomic I0/R0.

Not selected:
  Nowait has Future/type/binding/slot publication ordering; Local/Assignment,
  control/exit, declaration/ingress, and call/object/header remain separate.

R4 census:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 remains 37 occurrences / 8 source files.
```

`MIRBUILDER-LIVE-EDGE-CENSUS22-D0` — read-only, closed: non-Box Script residual registered

```text
Selected production edge:
  selected normal Program -> Script runtime work
  -> NormalScriptRuntimeBlockPortV1::lower_statement
  -> RawCompatibility -> drive_legacy_statement_v1

Decision:
  Direct BoxDeclaration is retired by the preceding I0, but every direct
  non-Box Script statement still reaches this broad compatibility edge. Record
  `NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003` in the sole R4 registry
  before selecting its disposition. It is a live selected residual, not an
  unregistered R4 family and not a `LegacyChildDraftAdmissionV1` occurrence.

Next:
  `NORMAL-SCRIPT-NONBOX-STATEMENT-DISPOSITION0-D0`. It must inventory the
  exact non-Box kind families, identify port-neutral/statement/control/call
  boundaries, and select at most one source-only partition with a named old-edge
  delete. Do not open an I0 from the catch-all branch.

R4 census:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  `LegacyChildDraftAdmissionV1` remains 37 occurrences / 8 source files.
```

## Current design decision

`NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-D0` — T1, closed

```text
Decision:
  Candidate A — move the complete 21-kind CallObjectHeader family to the
  existing DirectPortAwareExpression terminal as one production responsibility.

Selected kinds:
  QMarkPropagate / MatchExpr / EnumMatchExpr / ArrayLiteral / MapLiteral /
  RecordLiteral / RecordUpdate / Lambda / BlockExpr / Arrow /
  GroupedAssignmentExpr / MethodCall / FieldAccess / Index / New / This /
  FromCall / ThisField / MeField / FunctionCall / Call.

Parity basis:
  build_statement_with_port_v1 does not intercept any selected kind. It writes
  the root span and immediately calls drive_legacy_expression_v1; the raw
  expression dispatcher then writes the identical span and uses the same
  RawInvocation port. Header loans, collector visibility, allocation/type/birth
  effects, QMark/Match control, Lambda/BlockExpr lifecycle, arbitrary children,
  diagnostics, and failure remain owned by the existing expression terminals.

Atomic delete:
  all 21 selected Script roots
  -> CallObjectHeaderCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Structure:
  reuse normal_script_direct_statement_owner exactly; add no product, owner,
  route terminal, test/check file, or per-row guard. Delete the retired
  CallObjectHeaderCompatibility category. Residual count becomes 26.

Evidence:
  exhaustive source partition; representative full MIR/verification/diagnostic
  parity across header/collector, allocation, control, nested lifecycle, and
  currently unsupported members; distinct root/child spans; late failure then
  fresh compiler reuse; selected statement-facade edge zero.

Not selected:
  Return is a valid later direct statement responsibility with completion and
  cleanup pairing. StaticConstTable is a valid later metadata-runtime Void
  completion. Neither enters this expression-facade row.

Forbid:
  child allowlists; new header/collector/allocation/control policy; selecting
  only successful members; raw/reference widening; retry/fallback/reselection;
  a second port; AST reparse/clone beyond current terminals; or any source/check
  file reaching 800 lines.

Next:
  NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-I0-R0.
```

`NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-D0` — T1, closed

```text
Decision:
  Candidate A — retire the statement facade for all seven direct expression
  roots as one production responsibility.

Selected roots:
  Literal / Variable / Me / UnaryOp / BinaryOp / AwaitExpression / CheckExpr.
  Descendants are unrestricted and keep the exact existing RawInvocation port.

Old path:
  PortAwareExpressionCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  -> build_statement_with_port_v1
  -> drive_legacy_expression_v1

New path:
  DirectPortAwareExpression
  -> selected direct-statement sibling
  -> drive_legacy_expression_v1 with the same port

Parity basis:
  build_statement_with_port_v1 has no policy for these roots. Its only extra
  operation is writing the root span immediately before the expression
  dispatcher writes the identical span. Header, collector, Box, Loop, Call,
  child failure, and nested expression routes remain on the same port.

Structure:
  normal_script_runtime_work.rs is already 798 lines. Create one small
  normal_script_direct_statement_owner sibling, move the existing DirectPrint
  terminal into it, and add the expression terminal there. The source-only
  disposition file must not gain Builder/lowering authority.

Atomic delete:
  the seven roots -> RawCompatibility -> drive_legacy_statement_v1 = 0.
  Residual kind count becomes 47; compatibility terminal count remains one.

Evidence:
  exhaustive/disjoint source partition; full MIR/verification parity across
  all seven roots and nested call/object/control-sensitive descendants; exact
  root/child span parity; undefined-variable failure then fresh reuse; no new
  RawLegacy port, build_expression facade, retry, or fallback.

Forbid:
  operand allowlists; new expression semantics; Nowait or another statement
  kind in this row; block-driver bypass; raw/reference widening; AST reparse;
  new failure/source identity; or any source/check file reaching 800 lines.

Next:
  NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-I0-R0.
```

`NORMAL-SCRIPT-NONBOX-STATEMENT-DISPOSITION0-D0` — T1 partition, closed

```text
Observed edge:
  selected normal Script
  -> NormalScriptRuntimeBlockPortV1::RawCompatibility
  -> drive_legacy_statement_v1
  -> RawInvocationChildPortV1::lower_statement

Correction:
  This is not a RawLegacy-port fallback. The selected RawInvocation child port
  already crosses the facade unchanged. The debt is the broad statement
  dispatcher and its catch-all admission.

Total direct non-Box inventory:
  55 AST kinds exactly; direct BoxDeclaration is already owned by the preceding
  I0, and top-level FunctionDeclaration is consumed by immediate work.

Selected first slice:
  Print, with its complete current expression surface. Existing
  PreparedRawPrintV1 source observation, TypeOp/general route, child descent,
  diagnostics, and output emission remain the sole semantics.

Residual partition:
  port-aware expression family                    = 7
  statement/control/state family excluding Print = 17
  declaration/ingress family                     = 9
  call/object/header-sensitive family             = 21
  total residual                                  = 54

Why no operand allowlist:
  Both old and new Print routes use the same selected invocation port. A
  Literal-only or port-neutral-only Print slice would narrow the replacement
  for testing convenience without protecting a real authority boundary.

Next:
  NORMAL-SCRIPT-PRINT-DIRECT-OWNER0-I0-R0
  -> classify the 55-kind partition once
  -> direct Print to PreparedRawPrintV1 and its existing lower terminal
  -> delete Print -> RawCompatibility -> drive_legacy_statement_v1
  -> keep all 54 residual kinds at one compatibility terminal

Forbid:
  grammar/result/publication changes; Print TypeOp re-observation; a second
  child port; raw/reference widening; selected failure -> compatibility retry;
  block-driver/suffix bypass; AST clone/reparse; new failure/source identity;
  or selecting another residual kind in the same I0.
```

## Current closeout

`NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-I0-R0` — T1 atomic cutover, closed

```text
Change:
  Move the existing DirectPrint terminal into one small selected
  direct-statement sibling and add a direct expression handoff for all seven
  previously classified expression roots.

Direct owner:
  Literal / Variable / Me / Unary / Binary / Await / Check
  -> drive_legacy_expression_v1
  -> the exact same RawInvocation child port and existing expression owners

Delete:
  seven direct expression roots
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0

Parity:
  Full normal/legacy MIR, verification, diagnostic and distinct-span outcomes
  are exact across all roots. A nested FunctionCall remains on the same port.
  Undefined-variable failure discards the candidate and a fresh request reuses
  the compiler. No operand allowlist, grammar change, retry, or fallback exists.

Structure:
  normal_script_direct_statement_owner.rs = 191 lines,
  normal_script_runtime_work.rs = 790,
  shared guard = 799; every source/check file remains below 800.

Registry:
  NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003 remains active and narrows
  from 54 to 47 exact kinds. The expression family leaves the residual; the
  statement/control, declaration/ingress, and call/object/header families remain
  at one compatibility terminal. No fence or compatibility terminal was added.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS24-D0; select nothing before the fresh census.
```

`NORMAL-SCRIPT-PRINT-DIRECT-OWNER0-I0-R0` — T1 atomic cutover, closed

```text
Change:
  One exhaustive source-only disposition owner partitions all 57 AST kinds:
  direct Box and top-level Function remain owned elsewhere, Print is selected,
  and the other 54 direct non-Box kinds remain in four compatibility families.

Direct owner:
  Print -> PreparedRawPrintV1
        -> lower_prepared_raw_print_with_port_v1
        -> the same RawInvocation child port

Delete:
  selected Script Print
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0

Parity:
  General and TypeOp Print routes keep current source observation, expression
  descent, diagnostics, MIR, verification, block order, and failure/reuse.
  No operand allowlist, grammar change, new port, retry, or fallback exists.

Structure:
  the new source file owns only the total disposition; production/test/check
  files are 798 / 201 / 799 lines at closeout and all remain below 800.

Registry:
  NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003 remains active but is
  narrowed from 55 to 54 exact kinds. No new fence or compatibility terminal
  was created.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS23-D0; do not preselect another AST responsibility.
```

`NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-DISPOSITION0-I0-R0` — T1, closed

```text
Change:
  Replace the selected Script runtime's direct BoxDeclaration
  RawCompatibility -> drive_legacy_statement_v1 branch with one total direct-Box
  partition.  The I0/R0 delete is only the direct-Box use of that broad raw
  branch; non-Box Script statements retain their current raw compatibility.

Contract:
  non-sync/non-Main static Box -> existing selected non-Main static lifecycle;
  non-sync instance Box -> existing full selected instance lifecycle, retaining
  its second Script demand through the already-issued constructor source batch
  and cataloged method admissions; sync Box -> the current fail-fast diagnostic
  at the same runtime statement point; static Main -> its separately fenced
  invocation-port compatibility terminal. No new Box semantics, identity, collector
  key/policy, source read/clone, result/publication policy, or fallback/retry.

Evidence:
  every direct Script Box is selected exactly once by the new source-only
  partition; no direct Box reaches drive_legacy_statement_v1. Normal/legacy
  parity covers generic instance callable output, Script's repeated instance
  demand, sync diagnostic/order, and failure-then-fresh-reuse.

Registry:
  `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-COMPAT-SUNSET-002` is closed. The raw
  statement compatibility now owns non-Box Script statements only; raw/reference
  and nested Box descent remain outside this row.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS22-D0`; do not preselect another I0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS21-D0` — read-only, closed: non-plain Script Box D0 selected

```text
Selected production edge:
  selected normal Program -> Script runtime work
  -> NormalScriptRuntimeBlockPortV1::lower_statement
  -> RawCompatibility -> drive_legacy_statement_v1

Decision:
  `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-DISPOSITION0-D0`.
  `RawCompatibility` is a catch-all for non-Box Script statements too, so no
  direct I0 is safe.  D0 must decide one total, source-only partition for only
  direct non-plain BoxDeclaration shapes, with one exact new owner or a
  parity-equivalent pre-descent rejection per shape.  It must leave non-Box
  statements, raw/reference, Script order, and fallback/retry unchanged.

Not selected:
  nested Box raw body descent remains the separately unregistered R4 family:
  it crosses Main/static/instance/constructor/header/collector lifecycle and
  therefore needs NESTED-BOX-RAW-BODY-DISPOSITION0-D0 first.

R4 census:
  registry before I0 was retain-fenced=2, active compatibility=2, unregistered=8,
  closed=3. LegacyChildDraftAdmissionV1 remains a separate 37-occurrence / 8
  source-file observation, not an owner-disposition count.  The sole registry
  now names exact active owner anchors and a named-or-forced-R4 release path.
```

`MIRBUILDER-LIVE-EDGE-CENSUS20-D0` — read-only, closed: constructor D0 selected

```text
NoSafeLiveI0:
  No remaining selected-normal LegacyChild edge can be deleted without a new
  source authority.  Raw static-Main remains explicit raw compatibility; Script
  non-plain Box remains its separately registered broad disposition.

Candidate:
  `NORMAL-INSTANCE-CONSTRUCTOR-CALLABLE-IDENTITY0-D0` (T2).
  The one selected-normal residual is the instance-constructor path:
    selected Program immediate instance Box (including Script non-plain)
    + selected Script plain-instance prefix
    -> constructor batch -> raw instance child terminal -> LegacyChild.
  Existing ordinary instance methods are cataloged already and remain excluded.

Registry correction:
  `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001` covers selected
  Program immediate constructors for every instance Box and the second
  selected Script plain-instance-prefix demand.  Script non-plain Boxes also
  have that immediate demand; only their later full raw runtime lifecycle is
  owned by the separately fenced non-plain Script surface.
  Selected function-body nested `BoxDeclaration` can independently reach the
  raw recursive child terminal, so it is registered as an unregistered R4
  family.  It is not folded into the Program/root constructor row.

Constructor D0 must decide:
  source identity from exact Box occurrence plus parser-owned
  `init|pack|birth/arity` key; Script's immediate-plus-runtime constructor
  demand law; physical receiver arity; and unchanged LegacySymbol +
  LegacyReplaceWholePair parity.  It must not widen the Box-method catalog,
  issue receipts for raw/reference, change collector policy, or add retry.
```

## Current closeout

`NORMAL-INSTANCE-CONSTRUCTOR-CALLABLE-IDENTITY0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  Program work-plan issues `{statement index, Box name, parser init|pack|birth/arity}`
  once per parser-normalized constructor row. Every selected Program instance
  Box consumes one source-keyed admission; selected Script plain-prefix gets
  a second physical admission from the same transported source occurrence.

Delete:
  selected constructor direct `LegacyChildDraftAdmissionV1` construction = 0.
  Raw/reference and non-plain Script raw-runtime edges do not consume a
  selected receipt and remain outside this closed row.

Evidence:
  parser-key order/non-Function skip, Script two-demand source transport,
  non-plain receipt exclusion, App and Script normal/legacy MIR parity,
  late-failure fresh reuse, shared guard, pointer guard, and lib check = green.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS21-D0`; do not preselect another I0.
```

## Latest closeout

```text
JOINMODULE-PHI-RETURN-STRATEGY-REOWN0-I0-R0
  Builder finalization owns Direct -> primary hint -> P3-D -> P4 -> P3-C
  phi_type_inference + JoinIR TypeHintPolicy/GenericTypeResolver + exports = 0
  focused strategy + normal parity + lib/vm-reference + lifecycle/lane/pointer = green
  next = fresh census13 D0
```

## Latest closeout

```text
JOINMODULE-OWNERSHIP-ANALYSIS-RETIRE0-RET0
  join_ir/ownership tree, module export, stale private-BindingId inventory/docs = 0
  semantic rehome / normal-default / VM-LLVM bridge / feature delta               = 0
  scoped build + seam/lane/pointer guards                                          = green
  next                                                                             = fresh census D0
```

## Latest design decision

`MIRBUILDER-LIVE-EDGE-CENSUS11-D0` — closed

```text
Normal root, child descent, collector, finalization, and publication have no
safe remaining live competing edge. The 13-file join_ir/ownership analysis
asset is caller-zero outside its module export, so RET0 is selected; all other
JoinIR/bridge/EdgeArgs surfaces remain independently fenced or rehome work.
```

## Prior design decision

`NORMAL-COLLECTOR-DRAIN-LIFECYCLE0-D0` — closed

```text
Candidate C-prime: reuse normal drain semantics in a normal-owned lifecycle,
and bind it to the existing candidate-session brand. Raw and canonical drains
remain incompatible family/receipt owners, not adapters. The brand is session
correspondence only; it does not reclassify normal work as a raw route.
```

## Latest closeout

```text
NORMAL-COLLECTOR-DRAIN-LIFECYCLE0-I0-R0
  selected normal collector: existing session brand -> one normal receipt -> ordered commit
  old normal_legacy_drain module and selected caller                                  = 0
  general Program function-set/MIR/metadata parity; collision/reuse; lib/vm-ref/gates = green
  next                                                                                = census11 D0 (closed)
```

## Latest closeout

```text
JOINMODULE-VERIFY-REFERENCE-RET0
  join_ir::verify module / export / progress-select test closure / stale inventory = 0
  verify_phi_reserved / VM bridge / LowerOnly / normalized shadow / LLVM             = unchanged
  focused if-select + lib/vm-reference checks + lane guards                           = green
  next                                                                                 = fresh R3 census D0
```

## Prior selection

```text
JOINMODULE-REFERENCE-LIVE-EDGE-CENSUS2-D0
  caller-zero legacy join_ir/ownership analysis                     = retirement candidate
  AST frontend                                                       = larger test/fixture closure
  VM bridge / normalized shadow / LLVM / phi observer / carriers    = fenced or separate
  consecutive detached RET0                                          = 3; fourth RET0 prohibited
  next                                                               = live-edge census10 D0
```

## Prior selection

```text
MIRBUILDER-LIVE-EDGE-CENSUS10-D0
  Program/root final collector drain                                = only live residual
  existing raw/canonical drains                                    = incompatible ownership
  expression/statement port                                        = already selected; NoSafeLiveI0
  next                                                              = normal collector-drain lifecycle D0
```

## Prior selection

```text
JOINMODULE-REFERENCE-LIVE-EDGE-CENSUS1-D0
  selected caller-zero verifier closure; all other R3 surfaces stayed fenced,
  retained, or separate pending a fresh post-retirement census.
```

## Latest closeout

```text
JOINMODULE-JSONIR-V0-REFERENCE-RET0
  serializer / snapshots / export / env helpers / duplicate manifest evidence = 0
  Stage-B bridge tests                                                        = retained
  focused Stage-B tests + lib/vm-reference checks + lane guards              = green
  normal/default, bridge, strict, LLVM delta                                 = 0
  next                                                                        = fresh R3 census D0
```

## Latest closeout

```text
GENERIC-CASE-A-APPEND-DEFS-RET0
  generic append lowerer / selector vocabulary / ValueId range = 0
  current two-input helper / non-append generic Case-A routes    = retained
  selector and ValueId tests + lib/vm-reference checks           = green
  bridge, normal/default, strict, LLVM delta                     = 0
  next                                                           = R3 reference census D0
```

## Latest closeout

```text
JOINMODULE-LOWERONLY-STALE-DESCRIPTOR-RET0
  stale bridge target / Loop exclusion / Case-A name facade = 0
  generic ArrayAccumulation lowerer / range / shape route   = retained
  target, Case-A, Loop predicate tests + lib checks          = green
  normal/default, bridge behavior, strict, LLVM delta        = 0
  next                                                       = generic asset D0
```

## Latest closeout

```text
JOINMODULE-NORMALIZED-SHADOW-RETIRE0-D0
  default normal normalized-shadow execution                         = 0
  explicit dev/debug direct execution + observer                     = retained-fenced
  normal authority / grammar / result / publication delta            = 0
  new fallback/retry approval                                         = 0
  next                                                               = explicit-reference D0
```

## Latest closeout

```text
JOINMODULE-DIRECT-RUNNER-RETIRE0-RET0
  direct JoinIR runner / test-only callers / module export            = 0
  HMI caller inventory                                                   = 10 -> 8
  normal/default / VM bridge / LLVM experiment behavior                 = unchanged
  failure-outcome inventories / default + vm-reference builds / guards  = green
  next                                                                   = VM bridge fence D0
```

## Latest closeout

```text
JOINMODULE-VM-BRIDGE-FENCE0-D0 / DOC0
  route                           = explicit vm-reference --backend vm only
  default mir / vm-fallback       = 0 reachability
  activation                       = NYASH_JOINIR_VM_BRIDGE=1 only
  Exec / LowerOnly / nonstrict VM continuation = retained-fenced
  stale gate and stdout contract   = corrected without behavior change
  sunset                           = VM-BRIDGE-COMPAT-SUNSET-001
  next                             = strict policy D0
```

`VM-BRIDGE-COMPAT-SUNSET-001` owns only
`join_ir_vm_bridge_dispatch` from the explicit VM keep route. It retires when
that dispatcher caller reaches zero or a separately selected one-execution
owner replaces the entire explicit lane; it does not authorize normal/default
fallback, VM bridge growth, or LLVM changes.

## Latest closeout

```text
JOINMODULE-VM-BRIDGE-STRICT-POLICY0-D0 / STRICT-ALIAS0-I0-R0
  strict authority                 = HAKO_JOINIR_STRICT || NYASH_JOINIR_STRICT
  changed surface                  = explicit VM bridge Exec failure only
  global JoinIR strict helper      = unchanged
  LowerOnly / dev-trace success    = unchanged and retained-fenced
  dual-alias policy tests          = green
  next                             = LowerOnly target alignment D0
```

## Previous closeout

```text
JOINMODULE-METHOD-RETURN-HINT-REOWN0-I0-R0
  selected P3-D normal finalization observation                    = private owner
  obsolete JoinIR helper import/call/module/file                   = 0
  resolver order / type-annotation policy / grammar / publication  = unchanged
  focused policy tests / normal parity / candidate reuse / guards  = green
  fallback / retry                                                 = 0
  next                                                             = normalized-shadow D0
```

## Previous closeout

```text
DERIVED-CONDITIONFN-SHADOW-RETIRE0-RET0
  direct condition_fn generated bundle / generator / dedicated guards = 0
  bounded-finalize and function-region-stack evidence                = refreshed
  aggregate and strict-converter evidence                             = refreshed
  raw root condition draft / JoinModule / normal-default routes      = unchanged
  focused lifecycle, artifact, parity, reuse, cargo check            = green
  next                                                                = JoinModule carrier-boundary D0
```

## Census9 closeout

```text
MIRBUILDER-LIVE-EDGE-CENSUS9
  Program/root/lifecycle                  = NoSafeLiveI0
  finalization/call                       = NoSafeLiveI0
  raw/reference compatibility             = NoSafeLiveI0
  no-header Call                          = separate caller-zero D0 only
  R2                                      = closed
  next                                    = JoinModule/reference-asset disposition
```

## Latest closeout

```text
FINALIZE0-CONDITIONFN-RET0-I0-R0 / ba8c111974
  finalizer missing-symbol injection             = 0
  Call materializer name-special const-1 path    = 0
  minimal lifecycle / normal parity / reuse      = green
  RawRequiredConditionDraftV1                    = unchanged
  next                                           = R2 live-edge census
```

## Census8 closeout

```text
MIRBUILDER-LIVE-EDGE-CENSUS8
  publication/pipeline                   = NoSafeLiveI0 (sole commit terminal)
  Program root / raw compatibility        = NoSafeLiveI0
  finalization metadata projections       = already replaced
  selected bounded design                 = FINALIZE0-CONDITIONFN-RET0-D0
  JoinModule                              = remains R3-only; not reactivated
```

## Latest closeout

```text
MODULE-FINALIZATION-FUNCTION-METADATA0-I0-R0

prepared function-metadata owner                      = exactly one
selected inline type/origin-caller projection          = 0
type hints -> owner commit -> return/PHI inference     = preserved
unit / normal general parity / candidate reuse / guard = green
fallback / retry / grammar / result / publication delta = 0
new source/test/check file                             = 1 / 0 / 0
largest touched source/check file                      < 800
next                                                   = fresh live-edge census
```

## Previous closeout

```text
NORMAL-PROGRAM-DEFERRED-STATIC-CONTEXT0-I0-R0

selected direct context open/clear                                 = 0
private scoped context owner                                       = exactly one
prior None/Some restored on success, error, and unwind             = green
method order, primary error, callable capture                      = preserved
non-Main static candidate failure -> fresh corrected reuse         = green
shared guard / focused tests / cargo check                         = green
fallback / retry / grammar / collector / finalization delta        = 0
new source/test/check file                                         = 0 / 0 / 0
largest touched source/check file                                  < 800
next                                                               = fresh live-edge census
```

## Previous closeout

```text
MIRBUILDER-LIVE-EDGE-CENSUS5

safe live T1 replacement                                           = none
selected normal live boundary                                      = deferred static context
selected next stop                                                 = T2 D0
detached RET0 selected                                             = 0 (three-row horizon closed)
raw/static-Main, no-header Call, Loop/CorePlan                     = separate D0 only
JoinModule normal/default execution                                = 0; R3 disposition required
JoinModule current inventory                                       = 34,212 LOC
next                                                               = deferred-static context D0
```

## Previous closeout

```text
NORMAL-PROGRAM-STATIC-TABLE-PLAN0-I0-R0

`PreparedNormalProgramStaticTableMetadataV1`                     = exactly one
selected direct source collect/plan/two metadata writes           = 0
facts -> paired static-table metadata -> work-plan/body           = preserved
source order, diagnostics, candidate discard/reuse                = preserved
static-table unit / existing static-const tests / guard           = green
fallback / retry / grammar / result / finish / publication delta  = 0
new source/test/check file                                        = 1 / 0 / 0
largest touched source/check file                                 < 800
next                                                              = fresh live-edge census
```

## Previous closeout

```text
RAW-INDIRECT-CALL-LEGACY-FACADE-RETIRE0-RET0

`build_indirect_call_expression` facade                           = 0
ambient production `RawLegacyChildLoweringPortV1`                  = 0
raw dispatcher -> with-port indirect-Call owner                   = exactly one
raw-port Call regression / shared guard / cargo check             = green
fallback / retry / grammar / result / Call policy delta           = 0
new source/test/check file                                        = 0 / 0 / 0
largest touched source/check file                                 < 800
next                                                              = fresh live-edge census
```

## Previous closeout

```text
RAW-CHECK-LEGACY-FACADE-RETIRE0-RET0

`build_check_expression` facade                              = 0
ambient production `RawLegacyChildLoweringPortV1`             = 0
raw dispatcher -> with-port Check owner                       = exactly one
Check unit / raw-port integration / shared guard / cargo check= green
fallback / retry / grammar / result / control delta           = 0
new source/test/check file                                    = 0 / 0 / 0
largest touched source/check file                             < 800
next                                                          = fresh live-edge census
```

## Latest closeout

```text
RAW-QMARK-LEGACY-FACADE-RETIRE0-RET0

`build_qmark_propagate_expression` facade                = 0
ambient `RawLegacyChildLoweringPortV1` construction       = 0
raw dispatcher -> with-port QMark owner                  = exactly one
raw-port QMark regression / shared guard / cargo check   = green
fallback / retry / grammar / result / control delta      = 0
new source/test/check file                               = 0 / 0 / 0
largest touched source/check file                        < 800
format check                                             = unrelated pre-existing diffs
next                                                     = fresh live-edge census
```

## Latest closeout

```text
NORMAL-PROGRAM-COLLECTOR-DRAIN0-I0-R0

selected direct `collector.into_draft_functions()`                 = 0
selected direct `try_add_functions_atomic(drafts)`                 = 0
prepared final-row normal legacy drain -> atomic commit            = exactly one
legacy symbol/replacement admission and RootLower mapping          = preserved
normal general parity / collision / reuse / imports                = green
fallback / retry / grammar / result / publication delta            = 0
new source/test/check file                                         = 1 / 0 / 0
largest touched source/check file                                  < 800
next                                                               = fresh live-edge census
```

## Latest closeout

```text
CALL-GLOBAL-PRESENCE-LEGACY-FACADE-RETIRE0-RET0

Call global-presence facade/direct module observation = 0
authority-aware resolver live emitter                 = exactly one
LegacyCompatibility no-header authority               = retained
header authority / lane guards                         = green
grammar / result / dialect / fallback delta           = 0
new source/test/check file                            = 0 / 0 / 0
next                                                  = fresh live-edge census
```

## Previous closeout

```text
RAW-LEGACY-EXPRESSION-FACADE-RETIRE0-I0-R0

raw expression facade/module/input-view boundary       = 0
sole port-aware raw matcher                             = retained
raw static-Box / Lambda direct-matcher evidence        = green
raw dispatcher unit suite / shared lane guard          = green
grammar / result / route / fallback delta              = 0
new source/test/check file                             = 0 / 0 / 0
next                                                   = fresh live-edge census
```

## Previous closeout

```text
PROGRAM-ROOT-WORK-PARTITION0-I0-R0

source-only source-ordered work plan                  = exactly one
selected mixed Program statement coordinator           = 0
facts -> static-table -> immediate -> deferred -> terminal = preserved
runtime non-Function/Box retention                     = preserved
Script/App terminal and collector failure authority    = unchanged
normal general parity / candidate reuse / imports      = green
fallback / retry / grammar / result delta              = 0
new source/test/check file                             = 1 / 0 / 0
largest touched source/check file                      < 800
next                                                   = fresh live-edge census
```

## Previous closeout

```text
NORMAL-DEFAULT-PROGRAM-DECLARATION-FACTS0-I0-R0

source-only source-ordered facts product             = exactly one
selected `declaration_indexer` file/module/caller    = 0
catalog -> facts -> static-table -> body              = preserved
Brand / Enum / record defaults / Box-field-weak       = preserved
static-scalar updates retain source-order removal     = preserved
normal general parity / candidate reuse / imports     = green
fallback / retry / grammar / result delta             = 0
new source/test/check file                            = 1 / 0 / 0
largest touched source/check file                     < 800
next                                                  = fresh live-edge census
```

## Previous closeout

```text
RAW-THROW-DEBUG-TRACE-COMPAT-RETIRE0-I0-R0

statement_surface Throw -> prepare/lower                = exactly once
NYASH_BUILDER_DISABLE_THROW definition/read/docs         = 0
debug completion/enum/field/fixture/guard residue        = 0
physical Throw completion                                = sole
Throw unit / normal parity / failure-reuse / imports      = green
fallback / retry / grammar delta                          = 0
new source/test/check file                              = 0
largest touched source/check file                       < 800
```

## Forward task order

This is a dependency order, not a pre-authorized construction queue.  A row
opens only when its predecessor's evidence is green and the required fresh
census or D0 has selected it.

```text
1. ENTRY-MATERIALIZATION-RECEIPT0-S0                    (closed)
   Source-only normal/raw request, target, and receipt vocabulary.  No Builder,
   collector, ledger, runner, or old-edge effect.

2. ENTRY-MATERIALIZATION-NORMAL-CONSUMPTION0-I0-R0      (closed)
   Named caller: NormalDefaultPublishedPipelineV1.
   Consume the normal source receipt through the existing one-session lifecycle
   and delete the selected lower-side environment snapshot/materialization edge.
   Raw/reference and every runner selector retain their current authority.

3. MIRBUILDER-LIVE-EDGE-CENSUS15-D0                     (closed)
   Re-inventory selected normal, raw/reference, and runner materialization
   consumers.  It may select exactly one bounded D0, one live I0/R0, or
   NoSafeLiveI0; it may not assume a raw handoff or runner cutover in advance.

4. NORMAL-RUNTIME-INPUT-SNAPSHOT0-D0                    (closed)
   Candidate N selects one infallible normal-only ingress receipt.  It preserves
   normal's permissive malformed-value behavior; raw/reference remains separate.

5. NORMAL-RUNTIME-INPUT-SNAPSHOT0-I0-R0                 (closed)
   Named caller: NormalDefaultPublishedPipelineV1::compile.  One atomic
   normal-only receipt cutover deletes the two selected lower-side ambient reads.

6. MIRBUILDER-LIVE-EDGE-CENSUS16-D0                     (closed)
   Re-inventory the remaining selected-normal, compatibility, and fenced
   reference edges before selecting another live replacement or retirement.

7. NORMAL-CALLABLE-DRAFT-IDENTITY-AND-ADMISSION0-D0     (closed)
   Selected catalog-addressable Box-method replacement; uncatalogued children
   are explicit compatibility, not omitted coverage.

8. NORMAL-CATALOGED-BOX-METHOD-DRAFT-ADMISSION0-I0-R0   (closed)
   One atomic selected-normal static/instance Box-method source-witness cutover.

9. MIRBUILDER-LIVE-EDGE-CENSUS17-D0                     (active)
   Fresh selection after cataloged Box-method admission; no successor is
   presumed before live consumer evidence.

10. Entry-materialization residuals                     (census-selected only)
   A raw/reference receipt handoff and each runner-adapter receipt are separate
   responsibility decisions.  They must preserve their route-specific policies:
   no global selector, no `NYASH_ENTRY` reinterpretation, and no provenance
   collapse.  Their shared completion goal is the removal of the old snapshot /
   compilation-context / lower-side materialization authority, not a new route.

11. R3 reference-asset disposition                      (interleaved only by census)
   Each cycle is fresh consumer census -> one RET0, REOWN, or RETAIN-FENCED
   decision -> fresh census.  These rows earn no replacement credit.  The VM
   bridge, normalized shadow, LLVM experiment, and any live carrier remain
   named fences until their own evidence changes.

12. R4 final conformance
   Decide every live edge, compatibility sunset, and retained reference asset.
   The 34K-line JoinModule scope is decided here as either deletion or an
   explicit fenced reference asset; LOC is not a completion metric.  Complete
   requires normal/default reachability=0, acceptance truth=0, and final
   planner=0 for every retained reference family.

13. R5 features, strictly after R4 Complete
   Refresh Ownership readiness -> implement Ownership -> View D0 and I0 -> one
   later unimplemented feature semantic slice at a time.
```

## Task-selection rules

```text
Live I0/R0:
  named non-test production caller + new owner + same-series old-edge deletion
  + parity/failure/reuse evidence.  fallback/retry = 0.

Prerequisite S0:
  allowed only as the immediate, explicitly named predecessor of its I0/R0.
  A second proof-only row cannot be stacked onto it.

Compatibility owner:
  one bounded residual branch inside the selected pipeline, never a second
  route.  Creation or retention records sunset ID, exact non-growing surface,
  retirement owner, retire condition, and target row/evidence.  Any expansion
  returns to D0.

RET0:
  removes only a registered caller-zero asset, earns no replacement credit, and
  cannot revive JoinModule as a normal/default planner.

Frozen until R5:
  whole-function acceptance variants, Ownership, View, and feature work.
```

The observed shelves—header-sensitive Call, selected-invocation Loop/CorePlan,
raw/static-Main, function-state/control residuals, and raw AST/Recipe
composition—are census input, not an implementation queue.  JoinModule remains
outside R2 replacement commits but inside R4 completion: do not delete it by
name or silently inherit its final disposition.

## Previous closeout

```text
RAW-LAMBDA-LEXICAL-CAPTURE-LIFECYCLE0-I0-R0

raw dispatcher Lambda edge                  = lifecycle once
build_lambda_expression / exprs_lambda.rs   = 0
capture order                                = lexical first demand
missing capture / direct Me                  = pre-effect failure
nested Lambda / Function / Box               = pre-effect failure
closure metadata                             = reserve -> emit -> commit
raw Lambda / normal reuse / general parity   = green
fallback / retry / compatibility             = 0
new source/test/check file                   = 2 / 0 / 0
largest touched source/check file            < 800
```

## Previous closeout

```text
RAW-STATIC-MAIN-COMPAT-FACADE-RETIRE0-I0-R0

implementation commit                        = b23b654aa7
RawLegacyChildLoweringPort direct prepared handoff = exactly 1
build_static_main_box facades                = 3 -> 0
fresh RawLegacyChildLoweringPort construction = 1 -> 0
helper/root order and error mapping          = preserved
focused raw/verified Main ordering           = green
normal parity / failure / reuse              = green
release build / pointer / lane guards        = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
sunset manifest                              = active / facade edges 0
```

## Previous closeout

```text
VERIFIED-MAIN-ROOT-BODY-LOWERING-HANDOFF0-I0-R0

implementation commit                       = c0929f8171
selected root().source() lowering handoff    = 0
selected lower-side FunctionDeclaration rematch = 0
selected late missing/not-function errors    = 0
typed verified root payload handoff          = exactly 1
raw Main / callable-main / root behavior delta = 0
Main expansion/order/failure tests           = green
general module parity / failure / reuse      = green
release build / pointer / lane guards        = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## Previous closeout

```text
VERIFIED-MAIN-STATIC-CHILD-LOWERING-HANDOFF0-I0-R0

implementation commit                       = 67085bec4f
selected helper lower-side AST rematch       = 0
late static-child-source rejection           = 0
typed verified child payload handoff         = exactly 1
raw Main / root / callable-main identity delta = 0
expansion / helper order / failure tests     = green
general module parity / failure / reuse      = green
release build / current pointer / guards     = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## Latest closeout

```text
RAW-PORT-AWARE-COMPOUND-EXPR-OWNED-INPUT0-I0-R0

implementation commit                       = 1e73b9180f
immediate compound-expression deep clones   = 8 -> 0
Call / QMark / Await / Record owner APIs     = unchanged
child order / diagnostics / RootLower delta = 0
focused Call/QMark/Await/Record tests        = green
general module parity / failure / reuse      = green
release build / current pointer / lane guard = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## Latest closeout

```text
RAW-INSTANCE-METHOD-PARAM-NORMALIZATION-ONCE0-I0-R0

implementation commit                       = 71714556db
instance params normalization calls         = exactly 1
instance param-decls normalization calls    = exactly 1
duplicate capture normalization pair        = 0
normalized-input-only capture terminal      = exactly 1
static capture / route / grammar delta       = 0
focused normalization / constructor tests   = green
depth-three capture / first-failure tests    = green
release build / current pointer / lane guard = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

R1 closeout:

```text
selected normal construction sites = exactly 4
NormalCompileRequestV1 constructors = exactly 4
selected-normal Legacy reachability = 0
candidate / finish / publication    = exactly 1
compatibility build_module edge     = exactly 1
fallback / retry / reselection      = 0
new test/check file                 = 0
```

## Evidence

```text
selected normal constructors
  -> NormalCompileRequestV1
  -> NormalDefaultPublishedPipelineV1
  -> ModuleBuilderInvocationSessionV1
  -> ExistingGeneralModuleCompatibilityV1
  -> MirBuilder::build_module(ASTNode)

selected normal constructors:
  execute_mir_mode
  execute_mir_json_minimal
  LLVM source compiler
  Wasm source compiler

explicit compatibility:
  VM keep/fallback, Stage1, REPL, Program JSON v0, selfhost macro-preexpand
explicit reference:
  VM-Hako and the three VM-reference lanes
definition-only:
  execute_mir_interpreter_mode
```

The shared source-hint wrappers are provenance-blind and remain compatibility
surfaces. NarrowV1 lacks normal imports and general module/callable coverage;
only its source-neutral lifecycle kernels are reusable.

R2b closeout:

```text
selected lifecycle caller            = 1
ExistingGeneralModuleCompatibilityV1 = 0
selected-normal build_module edge    = 0
root-level AST clone                 = 1
typed lifecycle failure evidence     = 3/3
normal parity / failure / reuse      = 4/4
explicit compatibility build_module = 2, unchanged
new source file                      = 1, 292 lines
new test/check file                  = 0
all source/check files               < 800
optional quick gate                  = pre-existing EBNF naming-charter failure
clean efe2c467c2 reproduces the same failure
```

R2d closeout:

```text
shared root classifier                 = exhaustive 57/57
Program owner                          = unchanged
selected recursive-safe kinds          = 5
registered residual kinds              = 51
broad self.build_expression fallback   = 0
selected invocation-port descent       = 1
root-specific raw compatibility edge   = 1
selected failure retry                 = 0
focused tests                          = 7/7
new source file                        = 1, 338 lines
new test/check file                    = 0
largest touched source/check file      = 796
```

R2f closeout:

```text
Await recursive closure                 = selected safe / residual unsafe
selected invocation-port descent        = 1
existing Await completion owner          = 1
selected failure retry                   = 0
selected recursive-safe kinds            = 6
registered residual kinds                = 50
focused tests                            = 8/8
new source/test/check/task file           = 0
largest touched source/check file         = 574
```

R2h closeout:

```text
Check recursive closure                  = selected safe / residual unsafe
same-port eager child order              = sealed
existing Check completion owner          = unchanged
selected failure retry                   = 0
selected recursive-safe kinds            = 7
registered residual kinds                = 49
focused Rust tests                        = 13/13
existing Check surface guard              = green
new source/test/check/task file           = 0
largest touched source/check file         = 582
```

R2j closeout:

```text
safe Print compatibility edge           = 0
selected expression kinds               = 7
selected statement-root kinds           = 1
registered residual kinds               = 48
selected / compatibility terminals      = 1 / 1
direct instruction/span parity           = green, unified on/off
focused tests                            = 6/6
fallback / retry / reselection           = 0
production Rust delta                    = +37
new source/test/check/task file           = 0
largest touched source/check file         = 593
largest relevant source/check file        = 774, unchanged
```

R2l closeout:

```text
safe Nowait compatibility edge             = 0
selected expression kinds                 = 7
selected statement-root kinds             = 2
registered residual kinds                 = 47
selected / compatibility terminals        = 1 / 1
MIR/span/Future/binding/slot parity         = green
focused Rust tests                          = 7/7
fallback / retry / reselection             = 0
Rust net delta, including focused tests     = +153
new source/test/check/task file             = 0
largest touched source/check file           = 706
largest relevant source/check file          = 774, unchanged
```

R2n closeout:

```text
safe Array compatibility edge              = 0
selected expression kinds                 = 8
selected statement-root kinds             = 2
registered residual kinds                 = 46
selected / compatibility terminals        = 1 / 1
empty/homogeneous/mixed/nested parity       = green
focused Rust tests                          = 8/8
fallback / retry / reselection             = 0
Rust net delta, including focused tests     = +206
new source/test/check/task file             = 0
largest touched source/check file           = 745
largest relevant source/check file          = 774, unchanged
```

R2p closeout:

```text
safe Map compatibility edge                = 0
selected expression kinds                 = 9
selected statement-root kinds             = 2
registered residual kinds                 = 45
selected / compatibility terminals        = 1 / 1
duplicate/nested/unified off-on parity      = green
focused Rust tests                          = 9/9
fallback / retry / reselection             = 0
Rust net delta, including focused tests     = +241
new source/test/check/task file             = 0
largest touched source/check file           = 769
largest relevant source/check file          = 774, unchanged
```

## Program-v0 closeout

```text
RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-R0

production / test files              = 317 / 447 lines
existing focused tests               = 4/4 green
selected expression / statement      = 9 / 2 unchanged
registered residual kinds            = 45 unchanged
selected / compatibility terminals   = 1 / 1 unchanged
new test-only Rust file               = 1
production behavior / grammar delta  = 0
shared guard / artifact inventory     = green
fallback / retry / reselection        = 0
```

## Latest closeout

```text
RAW-NONPROGRAM-GROUPED-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0

safe Grouped Assignment compatibility edge = 0
selected expression / statement kinds      = 10 / 2
registered residual kinds                   = 44
selected / compatibility terminals          = 1 / 1
root focused tests                           = 6/6
grouped parity / failure / reuse             = 3/3
existing assignment/raw-port evidence        = green
normal-vs-Legacy non-Program parity          = green
shared guard / artifact inventory            = green
fallback / retry / reselection               = 0
new source/test/check file                    = 0
largest touched source/check file             = 665
```

## Latest closeout

```text
RAW-NONPROGRAM-INDEX-COMPOSITIONAL-DESCENT0-I0-R0

safe Index compatibility edge             = 0
selected expression / statement kinds     = 11 / 2
registered residual kinds                  = 43
selected / compatibility terminals         = 1 / 1
static / generic descent laws               = 0/1 and 1/1 green
StaticDataLoad success-only type evidence   = green
root focused tests                          = 7/7
Array/Map Index full-effect parity          = green
normal-vs-Legacy parity/failure/reuse        = green
shared guard / artifact inventory           = green
fallback / retry / reselection              = 0
new source/test/check file                   = 0
largest touched source/check file            = 675
```

## Latest closeout

```text
RAW-NONPROGRAM-EMPTY-BLOCK-EXPR-COMPOSITIONAL-DESCENT0-I0-R0

safe empty-prelude BlockExpr edge       = 0
selected expression / statement kinds  = 12 / 2
registered residual kinds               = 42
selected / compatibility terminals      = 1 / 1
statement / tail demands                = 0 / 1
nested selected / unsafe-tail partition = green
raw-port MIR/type parity                 = green
normal-vs-Legacy parity/failure/reuse    = green
shared guard / artifact inventory        = green
fallback / retry / reselection           = 0
new source/test/check file               = 0
largest touched source/check file        = 729
```

## Latest closeout

```text
RAW-NONPROGRAM-ANNOTATION-FREE-LOCAL-ROOT-DESCENT0-I0-R0

safe annotation-free Local edge          = 0
selected expression / statement kinds    = 12 / 3
registered residual kinds                 = 41
selected / compatibility terminals        = 1 / 1
typed-array / record special-hook reach    = 0 / 0
root partition tests                       = 8/8 green
existing Local descent/raw/parity suites   = 8/8, 7/7, 6/6 green
standalone lexical-scope diagnostic parity = green
candidate discard / compiler reuse         = green
shared guard / artifact inventory          = green
fallback / retry / reselection             = 0
new source/test/check file                 = 0
largest touched source/check file          = 782
```

## Latest closeout

```text
RAW-NONPROGRAM-ROOT-PARITY-TEST-SEAM1-R0

production Rust delta                = 0
moved fixture body parity            = 6/6 exact
root partition/parity tests          = 8/8 green
normal integration/failure tests     = 8/8 green
selected expr/stmt/residual          = 12 / 3 / 41
selected/compatibility terminals     = 1 / 1
parent/child/shared guard lines       = 482 / 305 / 718
shared guard / artifact inventory    = green
fallback / retry / reselection       = 0
new test-only Rust file              = 1
new check/guard/task file            = 0
all source/check files               < 800
```

## Latest closeout

```text
RAW-NONPROGRAM-BLOCK-EXPR-COMPOSITIONAL-PRELUDE0-I0-R0

safe non-empty BlockExpr compatibility edge = 0
selected prelude responsibilities             = Expr / Print / Nowait / Local
unsafe prelude or tail                         = whole compatibility
existing raw BlockExpr semantic owner          = unchanged
standalone Local lexical-scope failure parity  = green
root partition/parity tests                    = 10/10 green
normal integration/failure tests               = 8/8 green
selected expr/stmt/residual                     = 12 / 3 / 41
selected/compatibility terminals                = 1 / 1
production/parent/parity/integration/guard LOC  = 386/512/374/623/735
shared guard / artifact inventory               = green
fallback / retry / reselection                  = 0
new source/test/check/task file                  = 0
all source/check files                           < 800
```

## Latest closeout

```text
RAW-NONPROGRAM-TASK-SCOPE-COMPOSITIONAL-DESCENT0-I0-R0

safe TaskScope compatibility edge          = 0
empty/non-empty/nested safe partition      = green
safe TaskScope in BlockExpr prelude        = green
existing early-exit / push-body-pop owner  = unchanged
child failure pop-order parity             = green
root partition/parity tests                = 12/12 green
normal integration/failure tests           = 8/8 green
selected expr/stmt/residual                 = 12 / 4 / 40
selected/compatibility terminals           = 1 / 1
production/parent/parity/integration/guard = 413/577/440/646/751 lines
shared guard / artifact inventory          = green
fallback / retry / reselection             = 0
new source/test/check/task file             = 0
all source/check files                      < 800
```

## Closed execution

`NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-I0-R0` — T2, parent
`NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-D0`.

```text
result:
  four selected normal constructors -> one opaque Program admission
  -> one session-owned Program root/catalog kernel
  selected bare-AST/non-Program/generic-root admission = 0
  fallback / retry / reselection = 0

evidence:
  root/catalog lifecycle 2/2; constructor admission 1/1; normal candidate 8/8
  generic Program parity 1/1; raw non-Program partition 12/12
  shared guards, pointer guard, diff check, release build = green

structure:
  module_lifecycle.rs 796 -> 575
  program_root_lowering.rs = 286
  shared guard = 798
  new source/test/check files = 1/0/0
  every touched source/check file < 800
```

## Closed conformance census

`MIRBUILDER-EIGHT-PACK-FINAL-CONFORMANCE0-C0` — T0 conformance census,
replacement credit 0.

```text
verdict:
  selected-normal production chain = Complete
  repository-wide final pipeline   = Residual
  replacement credit               = 0

selected-normal:
  four typed constructors
  -> one Program admission before token/session
  -> one candidate/session
  -> one root/catalog lifecycle
  -> one collector-backed callable batch
  -> one finish/readiness/external commit

ledger reconciliation:
  14 landed production rows backfilled
  SOURCE-NEUTRAL-CALL-RECEIPT = ReuseNeutral closed
  PRELOOP-STAGEB-SPECIAL-ACTIVATION = Delete closed
  test-only seam rows receive replacement credit = 0
```

Eight-pack verdict:

| Pack | Verdict | Exact residual |
| --- | --- | --- |
| `REPLACEMENT-LEDGER0` | Residual | detached Stage-B asset is deleted; active compatibility sunsets remain |
| `DESCENT-SPINE0` | Complete | fixed selected old-edge inventory is physically zero |
| `FUNCTION-STATE0` | Residual | `function_state` / PHI / `variable_map` authority remains distributed |
| `CALL-OBJECT0` | Residual | MethodCall / Call / New / Field / Index and other header-sensitive compatibility surfaces remain |
| `CONTROL0` | Residual | If / Loop / TryCatch / Throw / QMark / Match and related control authority remains |
| `FUNCTION-LIFECYCLE0` | Residual | selected-normal is complete; raw legacy direct function publication remains |
| `MODULE-LIFECYCLE0` | Residual | selected-normal is complete; two production arbitrary-AST `build_module` surfaces remain |
| `COMPILER-RESIDUE0` | Residual | MirCompiler/runtime arbitrary-AST compatibility remains; Stage-B activation is zero |

Repository-wide final-pipeline completion additionally requires a **legacy
JoinModule disposition decision**.  The decision is not a blind `JoinModule`
file-count target: it must classify remaining carrier/boundary use, state
whether each surface is retired or intentionally retained, and prove that no
retained surface is a final planner, acceptance truth, or normal/default
pipeline route. It must keep the CorePlan carrier/boundary ledger separate
from JoinModule execution, observation, JSON/format, runner, and explicit-env
bridge ledgers; each family needs a retire or named non-JoinModule replacement
decision before repository-wide completion is claimed.

## Closed execution

`PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-RET0` — T1 detached-asset
retirement, one atomic commit.

```text
Decision:
  Delete the complete caller-zero Stage-B whole-source selector -> activation
  -> carrier -> function-session -> physical-publication closure.

Atomic delete roots:
  compiler:
    legacy_source_selection
    legacy_static_import_snapshot
    legacy_whole_source_request
    legacy_module_activation/**
  mir:
    preloop_stageb_candidate_shell
    preloop_stageb_carrier/**
  builder:
    preloop_stageb_context_install
    preloop_stageb_function_activation
    calls/preloop_stageb_instance_function_session/**
    calls/preloop_located_argument_*
    calls/preloop_located_outer_completion
    calls/preloop_nested_result_*
    calls/preloop_outer_carrier_*
    all dedicated cfg(test) support for those physical owners
  wiring:
    module declarations/reexports
    Stage-B-only module-lifecycle/readiness/install helpers
    Stage-B-only compilation-context helpers
  proof:
    two Stage-B child guards and their aggregate positive assertions

Keep:
  source_call_target/**
  source_instance_result_contract/**
  callable result/catalog and generic method-call receipts
  unified emitter, recursive child ports, generic function session
  ModuleDraftCollectorV1

Measured law:
  production caller                         = 0 -> 0
  detached production-capable asset family  = 1 -> 0
  replacement cell / credit                 = 0
  replacement owner / fallback              = 0

Ledger:
  PRELOOP-STAGEB-SPECIAL-ACTIVATION
  Delete pending -> Delete closed

Evidence:
  exact special-root repository-zero census
  retained source-neutral receipt tests = green
  normal candidate/session focused tests = 8/8 green
  retained method-call focused tests = green
  cargo check --tests = green
  cargo test --lib attempted; pre-existing baseline failures remain
  (one exact edgecfg failure reproduced at clean pre-RET0 HEAD)
  existing aggregate/replacement/pointer guards
  git diff --check
  all source/check files < 800

Hard stop:
  any non-test caller outside this closure appears
  selected-normal or explicit build_module behavior needs an edit
  a retained source-neutral receipt/catalog semantic must change
  a tombstone, alias, forwarding facade, fallback, or new guard is needed
  JoinIR/runtime Stage-B would be touched
```

## Latest closeout

`PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-I0-R0`

```text
production caller moved                    = 1
typed Program admission/lifecycle          = 1
ProgramV0Compatibility origin/constructor  = 0
Program-v0 raw-binding tombstone            = 0
loader compile_legacy edge                  = 0
direct ProgramV0-to-MIR JSON bridge delta   = 0
source hint / Builder imports               = exact / empty
module, metadata, verification, diagnostics = parity green
failure / compiler reuse                    = green
fallback / retry / reselection              = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## REPL closeout

`REPL-TYPED-PROGRAM-INGRESS0-I0-R0`

```text
production caller moved                  = 1
typed Program constructor/caller         = 1 / 1
REPL compile_legacy edge                 = 0
ReplCompatibility Rust symbols           = 0
source hint / Builder imports             = <repl> / empty
repl/quiet/plugin/ContinueLive config     = parity green
MIR/verification/failure/reuse            = parity green
vm-reference build and REPL execution     = green
VM/session/rewrite/auto-display delta     = 0
fallback / retry / reselection            = 0
direct production build_module edges      = 2, unchanged
new source/test/check/task file            = 0
largest touched source/check file          = 799
```

## Historical TryCatch transaction closeout
```text
Decision: Candidate A
Row: RAW-TRYCATCH-FUNCTION-STATE-TRANSACTION0-I0-R0
Ceremony: T2, one atomic implementation/retirement commit
Pack: FUNCTION-STATE0 + CONTROL0

Caller:
  statement_surface ASTNode::TryCatch

New owners:
  PreparedRawTryCatchV1
    -> DisabledCompatibility(try body only)
    -> Enabled(owned try / catches / finally)
  ActiveRawTryCatchFunctionStateV1
    -> exact seven-field success-only transaction

Exact state:
  return_defer_active / slot / target / emitted
  in_cleanup_block / cleanup_allow_return / cleanup_allow_throw

Contract:
  disable/enabled route sampled pre-effect exactly once
  same child port used exactly once
  first catch only, current try/catch/finally order unchanged
  catch body clone = 0
  success restores exact seven fields
  every typed failure restores 0 and preserves current dirty state
  primary String error unchanged
  CFG / ID / type / binding rollback = 0
  fallback / retry / reselection = 0
  grammar / MIR success / result / publication delta = 0
```

Success-only restoration is intentional. Restoring on failure would be a
separate behavior change; the outer candidate session already owns live-Builder
isolation.

### Atomic delete and sunset

```text
delete:
  statement_surface -> cf_try_catch_with_port_v1(raw fields)
  old terminal definition/export
  lower-side disable-route read
  seven saved_* locals and manual restore assignments
  catch body clone
  caller-zero cf_try_catch / MirBuilder facade / fresh Legacy port facade

sunset:
  id: RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-SUNSET-001
  owner: PreparedDisabledRawTryCatchV1
  surface: NYASH_BUILDER_DISABLE_TRYCATCH=1 -> try body only
  retire_when: env definition/read/documented consumers and fixture are zero
  growth: forbidden
```

### Evidence and hard stops

Use module-local tests plus existing integration/shared guards. New test,
check, task, and per-row guard files are zero.

```text
evidence:
  enabled success restores seeded seven fields and preserves MIR
  try/catch failure leaves current inner defer state and stops later bodies
  finally failure leaves current cleanup state and stops exit
  nested success restores outer state, then caller state
  disabled route lowers try only with no transaction/block/catch/finally
  first catch executes once; later catches execute zero
  failed candidate leaves live Builder unchanged; fresh request succeeds

hard stop:
  error-path restore or Drop/RAII restore
  broad FunctionOwnedStateTransactionV1 or non-seven-field capture
  MIR/CFG/ID/type/binding rollback
  cleanup-policy sampling before finally entry
  changed debug timing, catch semantics, or primary error
  clone/reparse, second port, fallback, retry, or old wrapper
  Return/Throw/QMark/If/Loop or Ownership/View/feature work
  compatibility growth/missing sunset
  any touched source/check file >= 800
```

Closeout evidence:

```text
prepared production caller                    = exactly 1
old cf_try_catch terminals/facades             = 0
manual seven-field snapshot/restore authority  = 0
catch-body clone                               = 0
success exact restore                          = green
try/catch/finally failure-state parity         = green
disabled prepare route                         = green
candidate isolation / fresh reuse              = green
fallback / retry / reselection                 = 0
release build                                  = green
quick gate                                     = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
largest touched source/check file              = 774
```

## Latest static Box closeout

```text
Decision: non-Main static Box state authority
Row: RAW-NONMAIN-STATIC-BOX-COMPILATION-STATE-TRANSACTION0-I0-R0
Ceremony: T2, one atomic implementation/retirement commit
Pack: FUNCTION-STATE0

Named caller:
  raw_expression_dispatch
  -> BoxDeclaration
  -> is_static && name != Main && !root_is_app_mode

New owner:
  ActiveRawStaticBoxCompilationStateV1

Exact state:
  variable_map
  TypeContext snapshot
  current_slot_registry
  BoxCompilationContext option
```

The transaction begins after `register_user_box`, captures/installs the four
states in their current order, and restores them only after every sorted
method succeeds. A method failure consumes a typed rejection without restoring,
preserving the current dirty candidate state and primary String error. The
outer invocation session remains the sole unpublished-candidate discard owner.

### Atomic delete

```text
dispatcher saved_var_map / saved_type_ctx       = 0
dispatcher saved_slot_registry / saved_comp_ctx = 0
dispatcher direct BoxCompilationContext install = 0
dispatcher four manual success restores         = 0
transaction begin / complete / reject            = exactly 1
```

The existing registration point, sorted method iteration, FunctionDeclaration
filter, same child port, per-method lowering, draft behavior, and
restore-before-Void order remain unchanged.

### Evidence and hard stops

Use module-local transaction/route tests plus existing candidate reuse and
static-Box parity evidence. New test/check/task files and per-row guards are
zero.

```text
evidence:
  seeded four-state success restores exactly before Void
  method N failure stops N+1 and leaves the current inner state
  zero methods begin/restore once and emit Void
  nested success restores outer transaction then caller
  Main / App-mode / instance / Program-root static routes do not participate
  late failure publishes no candidate and a fresh compiler request succeeds

hard stop:
  restore-on-error or Drop/RAII restore
  whole FunctionLoweringState / whole MirBuilder capture
  reuse of per-function FunctionOwnedStateTransactionV1
  register_user_box movement or rollback
  method order/filter/port/draft/publication change
  Main, App-mode, instance Box, or Program-root static integration
  Match clone cleanup in the same commit
  fallback, retry, grammar, View/Ownership, or feature work
  new per-row guard/test file or any source/check file >= 800
```

Match owned-input clone retirement remains a fresh-census candidate; it is not
bundled with this state-authority replacement.

Closeout evidence:

```text
named production branch                      = exactly 1
four-state begin / complete / reject          = exactly 1
dispatcher saved_* / direct context authority = 0
success exact restore before Void             = green
method-N failure / later-method stop          = green
failure non-restore / primary String parity   = green
static + instance invocation collection       = green
general-module MIR/result parity              = green
outer candidate discard / compiler reuse      = green
fallback / retry / reselection                = 0
release build                                 = green
quick gate                                    = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
shared spine guard                            = binary-only green
  full mode has pre-existing selected_static_sites drift
largest touched source/check file             = 781
```

## Latest deferred static Box closeout

```text
PROGRAM-ROOT-DEFERRED-STATIC-BOX-LIFECYCLE0-I0-R0
Pack: FUNCTION-LIFECYCLE0
Ceremony: T1
```

```text
direct Program-root context and method-lifecycle authority = 0
new consuming owner production calls                       = exactly 1
sorted demand / success clear / method-N dirty failure      = green
general Program parity / candidate reuse                    = green
fallback / retry / reselection                             = 0
new source/test/check file                                  = 0
largest touched source/check file                           = 784
release build                                               = green
quick gate                                                  = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

The owner preserves the existing success-only lifecycle: method failure leaves
the dirty candidate context, skips later methods, and retains the primary
String. Outer candidate discard remains the sole isolation owner.

## Latest static method-batch closeout

```text
NONMAIN-STATIC-BOX-METHOD-BATCH-SSOT0-I0-R0
Pack: FUNCTION-LIFECYCLE0
Ceremony: T0

prepared batch production issuers                   = exactly 2
caller-local non-Main static method dispatch copies = 0
sorted/non-Function/main-name/symbol/arity contract  = green
Program success-clear / method-N dirty failure       = green
raw four-state success / failure                     = green
general Program parity / candidate reuse             = green
fallback / retry / reselection                       = 0
new source file                                      = 1, 89 lines
new test/check file                                  = 0
largest touched source/check file                    = 776
release build                                        = green
quick gate                                           = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

## Current design stop

`MIRBUILDER-POST-CALL-NAME-CLASSIFICATION-LIVE-EDGE-CENSUS0-D0`.

```text
Read only:
  recount the production graph after call-name policy cutover
  select at most one behavior-neutral responsibility with a named caller
  require same-commit deletion of its competing old edge

Do not:
  infer the next row from deferred Main-helper or record-helper candidates
  add caller-zero routes, compatibility growth, View, Ownership, or features
```

## Latest call-name policy closeout

```text
CALL-NAME-CLASSIFICATION-SSOT0-I0-R0 / T1 / CALL-OBJECT0

decision owner / production consumers        = 1 / 3
Raw admission / Callee class facts           = independent, one total match
old predicate definitions / call edges       = 0 / 0
resolution priority and cross-surface matrix = exact
stable guard transports / Hako parity        = green
fallback / retry / compatibility growth      = 0
route/MIR/result/View/Ownership delta         = 0
new source / test / check files               = 1 / 0 / 0
new policy / largest touched source-check     = 98 / 460 lines
largest relevant source-check                 = 799, unchanged
release build                                 = green
quick gate                                    = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

## Latest call policy closeout

```text
CALL-BOX-KIND-POLICY-SSOT0-I0-R0
Pack: CALL-OBJECT0
Ceremony: T1

decision owner                              = 1
production consumers                       = 6
resolver-extended / general contexts        = 2 / 4
old classifier definitions and calls        = 0
analyzer compatibility growth               = forbidden
policy/resolver/call-route/parity/reuse      = green
fallback / retry / reselection               = 0
route/MIR/result/View/Ownership delta         = 0
new source file                              = 1, 114 lines
new test/check file                          = 0
largest source/check file                    = 799
release build                                = green
quick gate                                   = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

## Latest closeout

`INSTANCE-BOX-DECLARATION-LIFECYCLE-SSOT0-I0-R0` / T1 /
`FUNCTION-LIFECYCLE0` is closed.

```text
authority:
  one PreparedInstanceBoxDeclarationLifecycleV1
  one effectful common prefix
  distinct consuming root/raw method terminals

production issuers:
  Program-root instance Box = 1
  raw instance Box          = 1

deleted from both callers:
  register_user_box_declared_fields
  build_box_declaration
  constructor-batch issue/lower
  instance-method-batch issue/lower

preserved:
  field -> metadata -> every constructor -> every method
  metadata/constructor/method first-error dirty prefix
  root exact catalog key and missing-row diagnostic
  raw lookup-free method demand and trailing Void placement
  compatibility owner/sunset = 0
  fallback/retry/reselection = 0
  grammar/result/publication/View/Ownership delta = 0

evidence:
  lifecycle capture tests 14/14
  depth-three constructor/method capture, general Program parity, reuse = green
  route inventory/root/binary guards and release build = green
  quick gate = unrelated pre-existing EBNF compatibility-alias failure

structure:
  new source file = 1, 98 lines
  new test/check file = 0
  largest source/check file = 799
```

`INSTANCE-BOX-METHOD-BATCH-SSOT0-I0-R0` / T1 /
`FUNCTION-LIFECYCLE0` is closed.

```text
authority:
  PreparedInstanceBoxMethodBatchV1 prepares each lexically sorted
  non-static instance FunctionDeclaration once

production issuers:
  Program-root instance Box = 1
  raw instance Box          = 1

durable terminals:
  root -> exact catalog key -> lower_root_instance_method
  raw  -> no catalog lookup -> lower_instance_box_method

deleted:
  both caller-local sorted_method_entries loops
  duplicated filtering, symbol construction, payload cloning, dispatch

preserved:
  build_box_declaration -> constructor batch -> method batch
  lexical order, static/non-Function skip, first-error prefix
  root missing-key diagnostic and raw lookup-free behavior
  grammar/result/publication/View/Ownership delta = 0
  fallback/retry/reselection = 0

evidence:
  exact canonical namespace/symbol handoff, skip matrix, prefix failure
  nested constructor/method order, general Program parity, compiler reuse
  route inventory/root/binary guards and release build = green
  quick gate = unrelated pre-existing EBNF compatibility-alias failure

structure:
  new source file = 1
  new test/check file = 0
  largest source/check file = 799
```

`NORMAL-DEFAULT-VERIFIED-MAIN-LOWERING-HANDOFF0-I0-R0` / T1 /
`MODULE-LIFECYCLE0` is closed.

```text
authority:
  selected Program App terminal consumes VerifiedMainExpansionV1 directly
  for exact Main root source, sorted static children, and callable-main symbol

deleted from selected Program:
  raw Main method-map accumulator and methods.clone()
  second App/Main re-selection and impossible Script fallback
  build_static_main_box_with_port_v1 compatibility-facade edge
  helper re-sort/filter/symbol re-projection

preserved:
  RootExpansion validation precedence
  helper order, first failure, stop-before-later-helper/Main body
  callable-main Omitted/Required policy and Main body exactly once
  explicit raw Main compatibility via one shared body/state kernel
  Main args/state restoration, general MIR/result/publication behavior
  fallback / retry / reselection = 0

evidence:
  verified helper order and helper-N failure stop        = green
  Required selected/compat helper+Main order parity      = green
  general Program MIR/result parity                      = green
  late failure candidate isolation/compiler reuse        = green
  shared root guard / binary-only lane guard              = green
  release build                                           = green
  quick gate                                              = unrelated pre-existing
    docs/reference/language/EBNF.md naming-token failure
  new source/test/check file                              = 0
  largest touched source/check file                       = 799
```

## Previous closeout

`NORMAL-DEFAULT-ROOT-EXPANSION-ROUTE-HANDOFF0-I0-R0` / T1 /
`MODULE-LIFECYCLE0` is closed.

```text
authority:
  VerifiedRawRootExpansionV1 is issued once before prepare_module and borrowed
  through the selected normal Program kernel; is_app_mode is consumed once.

deleted:
  declaration_indexer::has_main_static definition and sole caller
  root_is_app_mode.unwrap_or_else ambient fallback
  second source AST Script/App classification

preserved:
  invalid/duplicate Main RootExpansion precedence
  CatalogSeal/CatalogInstall/RootLower/Finalize ordering
  root_is_app_mode publication for the existing raw observer
  Main/helper/body, catalog/index/static-data, collector/publication behavior
  explicit compatibility, fallback/retry/reselection = 0

evidence:
  Script/App disposition handoff fixture             = green
  invalid Main precedence                            = green
  general Program MIR/result parity                  = green
  late failure candidate isolation/compiler reuse    = green
  shared root guard / binary-only lane guard          = green
  release build                                      = green
  quick gate                                         = unrelated pre-existing
    docs/reference/language/EBNF.md naming-token failure
  new source/test/check file                         = 0
  largest touched source/check file                  = 792
```

## Latest closeout

`INSTANCE-BOX-CONSTRUCTOR-BATCH-SSOT0-I0-R0` / T0 /
`FUNCTION-LIFECYCLE0` is closed.

```text
new owner:
  PreparedInstanceBoxConstructorBatchV1

named production issuers:
  Program-root instance Box = 1
  raw instance Box          = 1

deleted:
  both caller-local constructor sort/projection/symbol/clone/dispatch loops
  sorted_constructor_entries helper and its caller-zero test

preserved:
  field registration and build_box_declaration ordering
  ordinary instance-method routes
  Main/static behavior
  stop-on-constructor-N failure and partial candidate state
  grammar/result/publication behavior
  fallback/retry/reselection = 0

evidence:
  lexical order/non-Function skip/first-failure stop = green
  nested constructor and depth-three capture paths   = green
  general Program parity and compiler reuse          = green
  focused shared guard and binary-only lane guard    = green
  full legacy module-draft/headerport guard           = pre-existing stale
    port_aware_function_draft.rs path failure before selected assertions
  release build                                      = green
  quick gate                                         = unrelated pre-existing
    docs/reference/language/EBNF.md naming-token failure
  new source file                                    = 1, 91 lines
  new test/check file                                = 0
  largest touched source/check file                  = 799
```

Lambda capture collector SSOT is a pre-designed candidate only. Main helper
batching and Match hygiene also remain unselected. Feature additions remain
parked until a fresh live-edge census selects one bounded replacement.
Breaking series selected by `MIRBUILDER-PUBLIC-ROOT-API0-D0`:
```text
1 MIRBUILDER-ROOT-TEST-EVIDENCE0-R0 closed (direct callers 15 -> 5)
2 HOST-PROVIDER-CFGTEST-AST-JSON-COMPAT0-RET0 closed (5 -> 4)
3 MIRBUILDER-RAW-OWNER-TEST-EVIDENCE0-R0 closed (4 -> 1)
4 MIRBUILDER-MINIMAL-LIFECYCLE-SMOKE0-R0 closed (1 -> 0)
5 MIRBUILDER-PUBLIC-ROOT-API0-RET0 closed (definition/wrappers -> 0)
```
External consumers are unknown; migration is `MirCompiler::compile*` for Program.

Compatibility sunset:
```text
sunset_id: RAW-NONPROGRAM-ROOT-COMPAT-SUNSET-001
state: closed
owner / residual surface / root-specific raw edge / execution callers: 0
retired by: RAW-NONPROGRAM-ROOT-COMPAT-RET0-R0

sunset_id: STAGE1-DIRECT-POST-MACRO-NONPROGRAM-COMPAT-SUNSET-001
state: closed
owner and NonProgram Legacy edge: 0
retired by: STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0
```

Closed sunset:

```text
NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-SUNSET-001
  owner ExistingGeneralModuleCompatibilityV1 = 0
  selected-normal build_module surface       = 0
  global build_module definition/callers     = non-claim
```

Compatibility sunsets:

```text
CALL-BOX-KIND-ANALYZER-COMPAT-SUNSET-001
  state: active
  owner: CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler
  surface: BreakFinderBox / PhiInjectorBox / LoopSSA
  growth: forbidden
  retire_when: analyzer production routes are zero, or one-profile
    classification parity is proven and all callers migrate atomically

RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-SUNSET-001
  state: closed
  owner: deleted
  definition/read/docs/fixture/route shell = 0
  retired by: RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-RETIRE0-I0-R0

RAW-THROW-DEBUG-TRACE-COMPAT-SUNSET-001
  state: closed
  owner: deleted
  definition/read/docs/fixture/route shell = 0
  retired by: RAW-THROW-DEBUG-TRACE-COMPAT-RETIRE0-I0-R0

MIRCOMPILER-ARBITRARY-AST-COMPAT-SUNSET-001
  state: closed
  production build_module edge: 0
  retired by: MIRCOMPILER-PUBLIC-PROGRAM-ADMISSION0-I0-R0

RUNTIME-MIRBUILDER-AST-JSON-COMPAT-SUNSET-001
  state: closed
  measured build_module edge: src/runtime/mirbuilder_emit.rs = 0
  env.mirbuilder.emit contract: Program(JSON v0) only
  AST JSON rejection: before Builder, no retry
```

## Queue

```text
R0  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-D0 closed
R1  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-I0-R0 closed
R2a NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0 closed
R2b NORMAL-DEFAULT-ROOT-CATALOG-LIFECYCLE0-I0-R0 closed
R2c NORMAL-DEFAULT-NONPROGRAM-ROOT-DESCENT0-D0 closed
R2d RAW-NONPROGRAM-PORT-NEUTRAL-EXPR-DESCENT0-I0-R0 closed
R2e RAW-NONPROGRAM-NEXT-COMPOSITIONAL-EXPR0-D0 closed
R2f RAW-NONPROGRAM-AWAIT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2g RAW-NONPROGRAM-NEXT-COMPOSITIONAL-EXPR1-D0 closed
R2h RAW-NONPROGRAM-CHECK-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2i RAW-NONPROGRAM-NEXT-RESPONSIBILITY0-D0 closed
R2j RAW-NONPROGRAM-PRINT-ROOT-DESCENT0-I0-R0 closed
R2k RAW-NONPROGRAM-NEXT-RESPONSIBILITY1-D0 closed
R2l RAW-NONPROGRAM-NOWAIT-ROOT-DESCENT0-I0-R0 closed
R2m RAW-NONPROGRAM-NEXT-RESPONSIBILITY2-D0 closed
R2n RAW-NONPROGRAM-ARRAY-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2o RAW-NONPROGRAM-NEXT-RESPONSIBILITY3-D0 closed
R2p RAW-NONPROGRAM-MAP-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2q RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-D0 closed
R2r RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-R0 closed
R2s RAW-NONPROGRAM-NEXT-RESPONSIBILITY4-D0 closed
R2t RAW-NONPROGRAM-GROUPED-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2u RAW-NONPROGRAM-NEXT-RESPONSIBILITY5-D0 closed
R2v RAW-NONPROGRAM-INDEX-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2w RAW-NONPROGRAM-NEXT-RESPONSIBILITY6-D0 closed
R2x RAW-NONPROGRAM-EMPTY-BLOCK-EXPR-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2y RAW-NONPROGRAM-NEXT-RESPONSIBILITY7-D0 closed
R2z RAW-NONPROGRAM-ANNOTATION-FREE-LOCAL-ROOT-DESCENT0-I0-R0 closed
R2aa RAW-NONPROGRAM-NEXT-RESPONSIBILITY8-D0 closed
R2ab RAW-NONPROGRAM-ROOT-PARITY-TEST-SEAM1-R0 closed
R2ac RAW-NONPROGRAM-NEXT-RESPONSIBILITY9-D0 closed
R2ad RAW-NONPROGRAM-BLOCK-EXPR-COMPOSITIONAL-PRELUDE0-I0-R0 closed
R2ae RAW-NONPROGRAM-NEXT-RESPONSIBILITY10-D0 closed
R2af RAW-NONPROGRAM-TASK-SCOPE-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2ag RAW-NONPROGRAM-NEXT-RESPONSIBILITY11-D0 closed
R2ah NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-D0 closed
R2ai NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-I0-R0 closed
R3  MIRBUILDER-EIGHT-PACK-FINAL-CONFORMANCE0-C0 closed: Residual
R4  PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-D0 closed
R5  PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-RET0 closed
R6  PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-D0 closed
R7  PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R8  REPL-TYPED-PROGRAM-INGRESS0-D0 closed
R9  REPL-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R10 POST-MACRO-PROGRAM-ADMISSION0-D0 closed
R11 STAGE1-DIRECT-POST-MACRO-PROGRAM-INGRESS0-I0-R0 closed
R12 MIR-INTERPRETER-POST-MACRO-PROGRAM-INGRESS0-D0 closed: NoProductionCaller
R13 MIR-INTERPRETER-DETACHED-ASSET-RETIRE0-RET0 closed
R14 BENCH-DETACHED-ASSET-RETIRE0-RET0 closed
R15 INTERPRETER-LEGACY-FEATURE-CLOSURE0-D0 closed: Retire
R16 INTERPRETER-LEGACY-FEATURE-RETIRE0-RET0 closed
R17 MIR-CONTROL-FLOW-DETACHED-HELPERS0-RET0 closed
R18 RAW-NONPROGRAM-VARIABLE-ASSIGNMENT-COMPOSITIONAL-DESCENT0-D0 closed
R19 RAW-NONPROGRAM-VARIABLE-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R20 RAW-NONPROGRAM-VARIABLE-COMPOUND-ASSIGNMENT-COMPOSITIONAL-DESCENT0-D0 closed
R21 RAW-NONPROGRAM-VARIABLE-COMPOUND-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R22 RAW-NONPROGRAM-SAFE-RETURN-ROOT-DESCENT0-D0 closed: Accept
R23 RAW-NONPROGRAM-SAFE-RETURN-ROOT-DESCENT0-I0-R0 closed
R24 RAW-NONPROGRAM-PLAIN-SCOPEBOX-COMPOSITIONAL-DESCENT0-D0 closed: Accept
R25 RAW-NONPROGRAM-PLAIN-SCOPEBOX-COMPOSITIONAL-DESCENT0-I0-R0 closed
R26 RAW-NONPROGRAM-SAFE-THROW-ROOT-DESCENT0-D0 closed: NoProductionConstructor
R27 RAW-NONPROGRAM-ROOT-INGRESS-POLICY0-D0 closed: IndependentSunsets
R28 RUNTIME-MIRBUILDER-AST-JSON-COMPAT-RETIRE0-I0-R0 closed
R29 POST-MACRO-ROOT-CONTRACT0-D0 closed: WholeFileProgram
R30 STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0 closed
R31 SELFHOST-MACRO-PREEXPAND-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R32 VM-HAKO-POST-MACRO-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R33 VM fallback closed; R34 VM keep closed; R35 helper DEL0 closed
R36 public contract D0 closed; R37 public Program admission closed
R38 public root D0 closed; R39 root test evidence closed
R40 host cfgtest AST JSON closed
R41 RAW-TRYCATCH-FUNCTION-STATE-TRANSACTION0-I0-R0 closed
R42 MIRBUILDER-POST-TRYCATCH-LIVE-EDGE-CENSUS0-D0 closed: static Box selected
R43 RAW-NONMAIN-STATIC-BOX-COMPILATION-STATE-TRANSACTION0-I0-R0 closed
R44 MIRBUILDER-POST-STATIC-BOX-LIVE-EDGE-CENSUS0-D0 closed: deferred Program static Box selected
R45 PROGRAM-ROOT-DEFERRED-STATIC-BOX-LIFECYCLE0-I0-R0 closed
R46 MIRBUILDER-POST-DEFERRED-STATIC-BOX-LIVE-EDGE-CENSUS0-D0 closed: method-batch SSOT selected
R47 NONMAIN-STATIC-BOX-METHOD-BATCH-SSOT0-I0-R0 closed
R48 MIRBUILDER-POST-STATIC-METHOD-BATCH-LIVE-EDGE-CENSUS0-D0 closed: constructor batch selected
R49 INSTANCE-BOX-CONSTRUCTOR-BATCH-SSOT0-I0-R0 closed
R50 MIRBUILDER-POST-CONSTRUCTOR-BATCH-LIVE-EDGE-CENSUS0-D0 closed: root expansion handoff selected
R51 NORMAL-DEFAULT-ROOT-EXPANSION-ROUTE-HANDOFF0-I0-R0 closed
R52 MIRBUILDER-POST-ROOT-EXPANSION-HANDOFF-LIVE-EDGE-CENSUS0-D0 closed: verified Main handoff selected
R53 NORMAL-DEFAULT-VERIFIED-MAIN-LOWERING-HANDOFF0-I0-R0 closed
R54 MIRBUILDER-POST-VERIFIED-MAIN-HANDOFF-LIVE-EDGE-CENSUS0-D0 closed
R55 INSTANCE-BOX-METHOD-BATCH-SSOT0-I0-R0 closed
R56 MIRBUILDER-POST-INSTANCE-METHOD-BATCH-LIVE-EDGE-CENSUS0-D0 closed
R57 INSTANCE-BOX-DECLARATION-LIFECYCLE-SSOT0-I0-R0 closed
R58 MIRBUILDER-POST-INSTANCE-BOX-LIFECYCLE-LIVE-EDGE-CENSUS0-D0 closed
R59 CALL-BOX-KIND-POLICY-SSOT0-I0-R0 closed
R60 MIRBUILDER-POST-CALL-BOX-KIND-POLICY-LIVE-EDGE-CENSUS0-D0 closed
R61 CALL-NAME-CLASSIFICATION-SSOT0-I0-R0 closed
R62 MIRBUILDER-POST-CALL-NAME-CLASSIFICATION-LIVE-EDGE-CENSUS0-D0 closed: Match owned-input selected
R63 RAW-MATCH-OWNED-INPUT-SINGLE-USE0-I0-R0 closed
R64 MIRBUILDER-POST-MATCH-OWNED-INPUT-LIVE-EDGE-CENSUS0-D0 closed: record-helper body selected
R65 RECORD-HELPER-BODY-INVOCATION0-I0-R0 closed
R66 MIRBUILDER-POST-RECORD-HELPER-BODY-LIVE-EDGE-CENSUS0-D0 closed: instance normalization selected
R81 RAW-NONMAIN-STATIC-BOX-LIFECYCLE-HANDOFF0-I0-R0 closed: raw dispatcher lifecycle deleted
R82 post-raw-static-Box census closed: no bounded edge; Lambda design selected
R83 RAW-LAMBDA-CAPTURE-OBSERVATION0-D0 closed: NoSafeI0
R84 RAW-LAMBDA-LEXICAL-BOUNDARY-MATRIX0-D0 closed
R85 RAW-LAMBDA-LEXICAL-CAPTURE-LIFECYCLE0-I0-R0 closed: old authority deleted
R86 post-Lambda census closed: its NoSafeSlice verdict was corrected by the
    later multi-owner task census
R87 RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-RETIRE0-I0-R0 closed: disable route and
    sunset retired; fresh live-edge census is current
R88 RAW-THROW-DEBUG-TRACE-COMPAT-RETIRE0-I0-R0 closed: debug-trace route and
    sunset retired; fresh live-edge census is current
R89 MIRBUILDER-LIVE-EDGE-CENSUS0 closed: no safe immediate I0/R0; Program
    declaration facts selected for T2 D0, while JoinModule remains final-C0
    family-disposition work
R90 NORMAL-DEFAULT-PROGRAM-DECLARATION-FACTS0-D0 closed: total source-ordered
    facts product accepted; atomic indexer replacement is next
R91 NORMAL-DEFAULT-PROGRAM-DECLARATION-FACTS0-I0-R0 closed: source-only facts
    product replaces the selected raw indexer edge; fresh live-edge census is current
R92 MIRBUILDER-LIVE-EDGE-CENSUS0 closed: raw non-Program is NoSafeI0, immediate
    compatibility retirement has no safe I0, and Program-root work partition is the
    sole selected T2 design stop; JoinModule remains final-C0 disposition work
R93 PROGRAM-ROOT-WORK-PARTITION0-D0 closed: one total source-only partition of the
    mixed Program-root coordinator is accepted; atomic I0/R0 is next
R94 PROGRAM-ROOT-WORK-PARTITION0-I0-R0 closed: one source-only work plan replaces the
    mixed coordinator while preserving source order, runtime retention, and terminals;
    fresh live-edge census is current
R95 RAW-LEGACY-EXPRESSION-FACADE-RETIRE0-I0-R0 closed: caller-zero raw expression
    facade and expression input view are deleted; fresh live-edge census is current
R96 CALL-GLOBAL-PRESENCE-LEGACY-FACADE-RETIRE0-RET0 closed: caller-zero direct
    module-presence facade is deleted; authority-aware resolver is sole entry
R97 MIRBUILDER-LIVE-EDGE-CENSUS0 closed: no safe live T0 replacement remains;
    normal collector drain selected as the sole T2 design stop

after every bounded retirement:
  fresh-census then select one named production edge or detached Delete asset

after final-pipeline Complete only:
F0  refresh missing-feature / Ownership / View readiness inventory
F1  resume the existing Ownership taskboard from its read-only readiness gate
F2  Unique Box / ScopedAlias -> callable ABI -> Anchored View
F3  select one later unimplemented feature from the language status index
```

The old M2c-to-M8 complete-program queue is superseded. Passive assets are
reconsidered only when the selected live edge names an exact consumer.
Source-level Ownership/View and other new language semantics do not enter the
MirBuilder replacement train. Analysis-only views used to observe existing
control flow are not source-language View activation.

R23 removed the root-only safe Return compatibility edge without body widening.
## Closed tail

```text
MODULE-SOURCE0-S0 / e6baf9b4
  exact Main0 + plain instance Boxes + callable catalog co-seal

INSTANCE-INTEGER-RETURN0-S0 / 34ea62cfea
  every instance method -> exact integer-literal Return plan

MAIN0-BRIDGE0-S0 / 7aed7848e6
  retained instance owner + existing Main0 semantic receipts

INSTANCE-CUMULATIVE0-S0 / 7e3144da62
  one source-owning cumulative set; exact ordered key coverage

INSTANCE-I64-PARAMETER-RETURN0-S0 / bdd0812c26
  total two-family classifier; exact Receiver + Parameter(0) + Return use
  evidence 76/76; production +464, test +62, check +36; max file 791

INSTANCE-INTEGER-LOCAL-RETURN0-S0 / adbb737f8a
  third cumulative variant; exact Receiver + Local(0) + Integer initializer
  + terminal Local read; evidence 74/74; production +391, test +62, check +8;
  one new source file, no new test/check file, max source/check 799

NORMAL-SOURCE-PLAN0-PROOF-COMPACTION / 8859caecba
  behavior/grammar delta 0; tests 701 lines, callable guard 755 lines
```

Detailed landed diffs and older cell measurements belong to git history and
the linked task map. They are not copied into this rolling card.

## Fixed packs

```text
REPLACEMENT-LEDGER0  production owner / detached asset accountability
DESCENT-SPINE0       body / statement / expression / argument descent
FUNCTION-STATE0      function facts / PHI / finalization state
CALL-OBJECT0         calls / new / fields / index / collections / lambda
CONTROL0             If / Loop / Match / QMark / cleanup / async
FUNCTION-LIFECYCLE0  draft / collector / function finalize
MODULE-LIFECYCLE0    declaration / catalog / module transaction
COMPILER-RESIDUE0    compiler ingress / old selectors / proof routes
```

新しい発見はこの8 packのいずれかへ入れる。新packは増やさない。

## Parked

```text
source-level Ownership/View and unimplemented language features until the
repository-wide final pipeline is Complete
.hako selfhost MirBuilder/parser migration
unselected cleanliness work
new language semantics
default Raw/Canonical cutover before M7
```

新しいper-row shell guardは作らない。通常gateと詳しいassertionはactive
source/testおよび既存shared guardが所有する。
