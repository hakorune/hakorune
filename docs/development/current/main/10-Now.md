Status: SSOT
Date: 2026-06-15
Scope: current lane / blocker / next pointer only.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Self Current Task - Now (main)

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active task card: read `latest_card_path` in `CURRENT_STATE.toml`
- compiler foundation checkpoint: `docs/development/current/main/phases/phase-293x/293x-1040-COMPILER-FOUNDATION-CHECKPOINT-001.md`
- compiler foundation taskboard: `docs/development/current/main/workstreams/compiler-foundation-current.md`
- BoxCallable registry SSOT: `docs/development/current/main/design/box-callable-registry-ssot.md`
- TypeAbiCatalog planning spine SSOT: `docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md`
- CorePlan migration roadmap SSOT: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- compiler expressivity policy: `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- implementation gaps: none; read `active_lane_status` in `CURRENT_STATE.toml`

## Next

- continue the active phase from `current_blocker_token`, `phase_status`, and
  `latest_card_path` in `CURRENT_STATE.toml`
- current day-to-day tasks live in `latest_workstream_card` from
  `CURRENT_STATE.toml`
- compiler foundation is paused at `COMPILER-FOUNDATION-CHECKPOINT-001`
- exact-front optimization is paused by
  `EXACT-AOT-FASTPATH-PAUSE-CHECKPOINT-001`
- VM product-route app validation is retired by
  `VM-ACTIVE-LANE-RETIRE-001`; EXE/AOT is the primary route for app/selfhost
  validation
- compiler construction now includes build-time reduction planning from
  `BUILD-CRATE-SPLIT-PLAN-001`; the first `mir_core` growth slice moved
  control-flow ID newtypes into `hakorune-mir-core`; the first
  `hakorune-mir-plans` split moved `object_storage_plan` behind a compatibility
  facade; the first cold release build baseline is recorded; the second
  passive split moved `aggregate_storage_plan`; the third split moved
  `map_repr_plan` pure data while leaving refresh logic in the main crate; the
  fourth split moved `local_fastpath_fact` pure aggregation while leaving
  `MirFunction` metadata assignment in the main crate; the fifth split moved
  `TypedObjectFieldStorage` while preserving the existing
  `crate::mir::function` import path; the sixth split moved record-layout /
  ArrayRecord / PackedArray passive metadata rows while leaving producer logic
  in the main crate; the seventh split moved typed-object / direct-state /
  record-state passive rows while leaving declaration inventory and producers
  in the main crate; the eighth split moved loop/range/direct-array/span
  function fact vocabulary while leaving producers and refresh logic in the
  main crate; Stage 1 is now closed. The post-stage1 cold build measured
  real=158.95s, so this was structural rather than a build-time winner. The
  backend preflight rejected wholesale `src/backend` split and selected
  `runner/mir_json_emit` as the next boundary. MIR JSON emitter preflight then
  rejected direct extraction because it still has 372 direct `crate::mir`
  references. The MIR JSON emit boundary now keeps projection in the main crate
  and reserves future crate extraction for serialization only. The export model
  now has passive root and function summary wiring. The export-model seam is
  closed, direct `mir_json_emit` extraction is still blocked by direct MIR
  references, and the DTO boundary now has passive JSON-ready construction. The
  optional AOT passive split is closed. The default-compiled `mir_interpreter`
  surface was audited at 12,944 lines across 66 files; VMValue / VMError are
  live outside the interpreter, so immediate deletion/gating is rejected. The
  next row selected a default-on `vm-reference` feature ladder. The scaffold is
  now in place: VMValue / VMError stay always available, while mir_interpreter
  and backend VM aliases are feature-gated. Default-off is not claimed yet. The
  runner callers are now classified. The central terminal fail-fast seam is in
  place for `vm-reference` disabled builds, while emit-mir-json / emit-exe early
  exits are preserved. Direct VM import families remain in JoinIR, REPL, and
  common VM helpers. The isolated REPL VM direct import is gated, and the
  structure-only JoinIR runner API/executor is now gated behind `vm-reference`.
  `run_joinir_via_vm` is now gated behind `vm-reference` while bridge conversion
  modules remain available. The final keep/vm common helper imports are now
  gated while preserving MIR JSON / EXE emit early exits. The vm-reference gate
  scaffold is closed: no-VM `cli,plugins` checks are green, and remaining full
  no-default failures belong to the plugins-disabled stub surface. The no-VM
  `cli,plugins` cold release profile measured real=151.21s versus the latest
  default baseline real=161.28s, making it a visible build-time candidate.
  Removing `vm-reference` from Cargo defaults is implemented: default/no-vm
  checks are green, explicit `vm-reference` remains green, MIR JSON emit still
  works, and VM execution fail-fasts in the default build. The default-off cold
  release build measured real=149.82s versus the latest default baseline
  real=161.28s. The vm-reference default-off row is closed; Rust VM execution
  remains available through explicit `--features vm-reference`. The next build
  split boundary selected is `hakorune-frontend`. Frontend preflight found zero
  MIR/backend refs, but direct extraction is blocked by AST literal runtime Box
  conversion and parser logging/config refs. The AST passive seam is now split:
  `LiteralValue` stays in `syntax.rs`, runtime Box conversion lives in
  `ast/literal_box_bridge.rs`. Parser config/env and runtime logging access now
  go through parser-local facades. The `hakorune-frontend-ast` passive crate
  scaffold is created, and `Span` is now owned by `hakorune-frontend-ast`
  behind the historical `crate::ast::Span` re-export. The next passive AST
  family selected is `UnaryOperator` / `BinaryOperator` / `BuildPredicate`;
  those types now live in `hakorune-frontend-ast` behind the syntax facade.
  `RuneAttr`, `DeclarationAttrs`, and rune Profile vocabulary now live in
  `hakorune-frontend-ast` behind compatibility re-exports. `LiteralValue`
  split design selected free-function runtime Box conversion because inherent
  methods cannot be preserved after moving the type to a frontend crate.
  `LiteralValue` and `Display` now live in `hakorune-frontend-ast`; runtime
  Box conversion remains in public `ast::literal_box_bridge` functions. The
  AST node preflight rejected direct `ASTNode` extraction and selected simple
  declaration metadata as the next safe passive bundle. That bundle now lives
  in `hakorune-frontend-ast`. Standalone `FieldDecl` split is rejected because
  `default_value` carries `ASTNode`. Recursive graph preflight selected moving
  `ASTNode`, ASTNode-containing metadata, wrapper structs, and inherent ASTNode
  utility methods together. The recursive graph now lives in
  `hakorune-frontend-ast`; `src/ast` is only a compatibility facade plus
  runtime literal Box bridge. The frontend AST split is closed. Parser crate
  preflight found 90 parser/tokenizer Rust files and 15,091 total lines, but
  rejected direct parser extraction because tokenizer config/runtime logging
  seams and parser grammar/syntax/prelude seams remain. Tokenizer env/logging
  access is now isolated behind tokenizer-local facades, leaving zero direct
  tokenizer config/runtime logger refs. Grammar engine seam preflight selected
  a small `hakorune-frontend-grammar` crate because the engine is
  dependency-light but generated table ownership must move with it. The grammar
  engine and generated tables now live in `hakorune-frontend-grammar` behind
  the historical `crate::grammar` facade. Parser/tokenizer grammar consumers
  now read `hakorune_frontend_grammar::engine` directly, leaving zero
  parser/tokenizer `crate::grammar` imports. Syntax/prelude seam preflight
  selected moving `SugarConfig` / `SugarLevel` into
  `hakorune-frontend-grammar`, then passive Result/Option prelude enum
  declarations into `hakorune-frontend-ast`. `SugarConfig` and `SugarLevel`
  now live in `hakorune-frontend-grammar` behind the historical
  `crate::syntax::sugar_config` facade. Parser sugar consumers now read the
  frontend grammar crate directly, leaving zero parser direct syntax sugar
  facade refs. Passive Result/Option prelude enum declaration construction now
  lives in `hakorune-frontend-ast` behind the historical semantics facade.
  Parser initialization now reads that frontend AST owner directly, leaving
  zero parser direct prelude facade refs. Parser/tokenizer crate preflight v2
  found grammar/sugar/prelude seams cleared, but direct extraction is still
  blocked by env/log facades that delegate to main-crate config/runtime. The
  env/log abstraction preflight found parser direct `std::env` reads outside
  the parser env facade; those reads are now centralized behind
  `src/parser/env.rs`. Env/log abstraction preflight v2 selected standalone
  facade simple flags first; those simple flags now live in parser/tokenizer
  local facades. Stage-3 feature parsing and alias warnings now live in
  `src/frontend_env.rs`. Parser/tokenizer runtime logging access now goes
  through `src/frontend_log.rs`. The low-risk CLI verbose config delegate is
  now local. Parser/tokenizer crate preflight v3 found config/runtime scattered
  refs closed, but direct extraction is still blocked by the frontend env/log
  host seam. Host adapter design selected a small `FrontendHostBoundary`
  vocabulary, now added passively in `src/frontend_host.rs`. Wiring preflight
  selected `RuntimeFrontendHost`, and `frontend_env` / `frontend_log` now route
  through it. Parser/tokenizer crate preflight v4 found the host seam blocker
  closed; the remaining root references are layout-compatible. Scaffold design
  selected a passive `hakorune-frontend-parser` crate with root compatibility
  modules, and the passive crate scaffold is now in place. File-move preflight
  selected tokenizer kinds as the first safe passive split, and
  `TokenType`/`Token`/`TokenizeError` now live in `hakorune-frontend-parser`
  behind the historical main-crate facade. Next-move preflight confirmed the
  remaining tokenizer implementation files are inherent impls for
  `NyashTokenizer`. Owner-bundle design selected a host install seam before
  moving tokenizer implementation; that runtime-free host registry and main
  runtime adapter seam is now in place. Owner-bundle move preflight selected a
  main-crate tokenizer wrapper instead of direct `NyashTokenizer` re-export.
  Wrapper design fixed `new()` host installation and `tokenize()` forwarding.
  `NyashTokenizer` and tokenizer impl modules now live in
  `hakorune-frontend-parser` behind the main crate wrapper facade. Post-move
  preflight selected `BuildGateExplainReport` as the next passive parser-side
  split, and that report now lives in `hakorune-frontend-parser` behind the
  main crate facade. Next-boundary preflight selected `BuildMode` and
  `ParserBuildConfig` as the next safe parser-side passive split. Those types
  now live in `hakorune-frontend-parser` behind the main crate parser facade.
  Next-boundary preflight 002 selected `ParseError` as the next passive parser
  boundary. `ParseError` now lives in `hakorune-frontend-parser` behind the
  main crate parser facade. Next-boundary preflight 003 selected `TokenCursor`
  and `NewlineMode` as the next passive parser boundary. Those types now live
  in `hakorune-frontend-parser` behind the main crate cursor facade.
  Next-boundary preflight 004 selected `ExprParserWithCursor` and helper
 modules as the next owner bundle move. That owner bundle now lives in
  `hakorune-frontend-parser` behind the main crate parser facade.
  Next-boundary preflight 005 selected `ParserMetadata` as the next passive
  parser boundary. `ParserMetadata` now lives in `hakorune-frontend-parser`
  behind the main crate parser facade. Next-boundary preflight 006 selected
  delegate exposes lowering as the next owner bundle move. Delegate exposes
  lowering now lives in `hakorune-frontend-parser` behind the main crate parser
  facade. Next-boundary preflight 007 selected the AST-to-AST sugar transform
  as the next passive parser boundary. The sugar transform now lives in
  `hakorune-frontend-parser` behind the main crate parser facade.
  Next-boundary preflight 008 selected no further thin passive parser boundary
  in this series. The parser passive split series is now closed; active parser
  implementation moves require a new design row. The next blocker is
  `BUILD-FRONTEND-PARSER-POST-SPLIT-MEASUREMENT-001`
- current manual entry points now route through current record/box,
  concurrency/thread, and object-storage SSOTs instead of stale historical
  Box-only or thread-spawn readings
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- use the active method anchor from `CURRENT_STATE.toml` instead of stale
  historical lane notes

## Rules

- keep BoxShape and BoxCount separate
- do not grow the restart mirrors with landed history
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-296x/296x-1216-BUILD-FRONTEND-PARSER-SPLIT-SERIES-CLOSEOUT-001.md`
3. `docs/development/current/main/phases/phase-296x/296x-1215-BUILD-FRONTEND-PARSER-NEXT-BOUNDARY-PREFLIGHT-008.md`
4. `docs/development/current/main/phases/phase-296x/296x-1214-BUILD-FRONTEND-PARSER-SUGAR-TRANSFORM-PASSIVE-SPLIT-001.md`
5. `docs/development/current/main/design/build-crate-split-plan-ssot.md`
6. `docs/development/current/main/design/vm-active-lane-retirement-ssot.md`
7. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
