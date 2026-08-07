---
Status: SSOT
Decision: provisional
Date: 2026-03-22
Scope: repo の物理構造を docs/設計の責務分離に追いつかせるための BoxShape cleanup 順序を固定する。即時の `src/mir` crate split や broad rename は扱わない。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/archive/phase-29cr/README.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# Repo Physical Structure Cleanup (SSOT)

## Goal

- 設計文書の美しさを、repo の物理構造でも読める形へ寄せる。
- root / `CURRENT_TASK.md` / `src/mir` の認知負荷を下げる。
- cleanup を `BoxShape` として進め、受理形追加や broad rename と混ぜない。

## Pressure Snapshot

Local snapshot on 2026-03-22:

- `src/**/*.rs`: `1789` files / `342813` lines
- `lang/**/*.hako`: `451` files / `54853` lines
- `src/mir/**/*.rs`: `1031` files / `210851` lines
- `src/mir/builder` subdirectories: `92`

Current reading:

- 設計哲学は先に整っている
- 物理構造がまだ追いついていない
- まず必要なのは crate split ではなく、入口と衛生の整理

Audit refresh on 2026-08-07 (`42ec69ab84`, local `rg` census):

```text
src/mir/**/*.rs                         = 2508
src/mir/builder/**/*.rs                 = 1205
lang/src/compiler/**/*.hako             = 476
src/mir Rust files containing cfg(test) = 745
Rust parser / Hako parser files         = 81 / 40
design Markdown recursive / direct      = 859 / 847
src/mir/mod.rs module declarations      = 198
```

These counts prove repository-topology pressure, not semantic ownership.
`cfg(test)` presence does not make a file legacy, and seed/pilot names do not
authorize deletion. The root-module namespace gap and its post-cutover rows are
owned by `mir-root-facade-contract-ssot.md`.

## Reading Rule

This wave is **BoxShape cleanup**.

Do:

- root hygiene
- entry-point thinning
- archive policy
- module/folder responsibility cleanup
- README / SSOT strengthening

Do not mix:

- new language acceptance
- runtime semantics change
- immediate `src/mir` crate split
- broad `nyash -> hako` rename

## Fixed Order

### P0. Root hygiene

Goal:

- repo root を “作業残骸置き場” ではなく “再起動入口” に戻す

Safe first buckets:

- `.gitignore` candidates:
  - `*.err`
  - `*.backup*`
- keep-root allowlist:
  - `basic_test.hako`
  - `test.hako`
- landed archive move targets:
  - docs archive:
    - `CURRENT_TASK_ARCHIVE_2026-01-23.md`
    - `HAKORUNE_RUST_CLEANUP_CAMPAIGN.md`
    - `NUMERIC_CORE_PHI_FIX_SUMMARY.md`
    - -> `docs/archive/cleanup/root-hygiene/`
  - tools archive:
    - `test_joinir_debug.rs`
    - `test_numeric_core_phi.sh`
    - `test_simple_windows.c`
    - `test_using.nyash`
    - `test_len_any`
    - `nyash.toml.backup2`
    - `build.err`
    - `check.err`
    - `llvm.err`
    - `vm.err`
    - `boxbase_identity_consultation_bundle.zip`
    - -> `tools/archive/root-hygiene/`

Rule:

- root の非 allowlist 新規追加は禁止
- 一時物は `tools/archive/root-hygiene/` or scratch
- 履歴物は archive

### P1. `CURRENT_TASK.md` slim

Goal:

- root pointer を cheap restart file に戻す

Keep in root:

- current blocker
- current priority
- exact next files
- reopen conditions
- recent accepted decisions only

Move out:

- long historical residue
- parked lane lore
- completed detail logs

`CURRENT_STATE.toml` is part of the same current-entry boundary. The current
file is 830 lines with 266 top-level assignments even though its header calls
for compact live pointers. Do not hand-delete keys: consumers may still depend
on historical names.

Parked order:

```text
CURRENT-STATE-CONSUMER-CENSUS0-P0
  -> classify every key and every code/guard/docs reader

CURRENT-STATE-LIVE-SCHEMA0-D0
  -> fix the minimal live schema and the non-authoritative history destination

CURRENT-STATE-LIVE-CUTOVER0-I0-R0
  -> migrate consumers first, then atomically remove historical keys
```

The candidate floor for the final live file is current row/owner/design/task,
next blocker, latest card, and explicitly parked-family pointers. The exact
key set is owned only by `CURRENT-STATE-LIVE-SCHEMA0-D0` after consumer census.
There is no second writable current-state authority. Pointer guards and all
consumers must be green before the old keys are removed.

Design-root cleanup stays on its existing H3/V1 lane. Current registry truth is
`mode = warning`, `unregistered_baseline = 77`. The sharded V1 migration is
behavior-neutral and must preserve that baseline; it does not earn a strict
claim. After V1 cutover, H3 classification drains the unregistered set to zero,
and only then may lifecycle enforcement select strict mode. Do not create a
second registry or a competing cleanup task family here.

Archive policy:

- archive when the slice is done
- archive when reopen condition is absent
- archive when SSOT/phase README pointers are enough to resume

### P2. `src/` top-level cleanup

Goal:

- flat top-level Rust scatter を減らす

Primary candidates:

- box-ish roots:
  - `box_trait.rs`
  - `box_arithmetic.rs`
  - `box_operators.rs`
  - `method_box.rs`
  - `type_box.rs`
- core-ish roots:
  - `value.rs`
  - `environment.rs`
  - `instance_v2.rs`

Rule:

- facade/re-export first
- physical move second

Landed first slice:

- `box_arithmetic.rs` -> inline facade
- `box_operators.rs` -> `src/boxes/operators/`
- `runner_plugin_init.rs` -> `src/runner/plugin_init.rs`

P2 landed:

- `box_trait.rs` -> `src/boxes/box_trait.rs`
- `operator_traits.rs` -> `src/boxes/operator_traits.rs`
- `channel_box.rs` -> `src/core/channel_box.rs`
- `environment.rs` -> `src/core/environment.rs`
- `exception_box.rs` -> `src/core/exception_box.rs`
- `finalization.rs` -> `src/core/finalization.rs`
- `instance_v2.rs` -> `src/core/instance_v2.rs`
- `method_box.rs` -> `src/core/method_box.rs`
- `type_box.rs` -> `src/core/type_box.rs`
- `value.rs` -> `src/core/value.rs`
- `ast.rs` -> `src/ast/mod.rs`
- `benchmarks.rs` -> `src/benchmarks/mod.rs`
- `wasm_test.rs` -> `src/wasm_test/mod.rs`

P3 first slice landed:

- `src/mir/README.md`
- `src/mir/builder/README.md`
- `src/mir/join_ir/README.md`
- `src/mir/loop_canonicalizer/README.md`
- `src/mir/passes/README.md`
- `src/mir/control_tree/README.md`
- `src/mir/control_tree/step_tree/README.md`
- `src/mir/control_tree/normalized_shadow/README.md`

P4 first slice landed:

- `src/mir/builder/control_flow/plan/normalizer/helpers_pure_value.rs`
- `src/mir/builder/control_flow/plan/normalizer/helpers_layout.rs`
- `src/mir/builder/control_flow/plan/normalizer/helpers_value.rs`
- `src/mir/passes/rc_insertion.rs` facade
- `src/mir/passes/rc_insertion_helpers.rs` implementation split
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_common.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_break_if.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_realworld.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_local.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_condition.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_loop.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_trim_whitespace_helpers.rs`

Next safe slice:

- P5 crate split prep: `src/mir/README.md` / `src/mir/builder/README.md` / `src/mir/passes/README.md`

P5 first packaging slice landed:

- `hakorune_mir_core` package with `types.rs` / `value_id.rs`
- `hakorune_mir_core` package with `value_kind.rs`
- `src/mir/types.rs` / `src/mir/value_id.rs` became thin re-export wrappers
- `src/mir/value_kind.rs` became a thin re-export wrapper

P5 substrate ID slice landed:

- `hakorune_mir_core` package now also owns `basic_block_id.rs` / `binding_id.rs`
- `hakorune_mir_core` package now also owns `value_kind.rs`
- `hakorune_mir_builder` package now also owns `core_context.rs` / `context.rs`
- `hakorune_mir_builder` package now also owns `binding_context.rs`
- `hakorune_mir_builder` package now also owns `type_context.rs`
- `hakorune_mir_builder` package now also owns `variable_context.rs`
- `hakorune_mir_joinir` package now also owns `join_ir/ownership/types.rs`
- `src/mir/basic_block.rs` re-exports the substrate IDs
- builder / edgecfg / optimizer / tests now use public `crate::mir::{BasicBlockId, EdgeArgs}`
- backend/mir_interpreter now uses public `crate::mir::BasicBlock` / `BasicBlockId`
- remaining README cleanup landed for `contracts/`, `control_tree/`,
  `join_ir_vm_bridge/`, `join_ir_vm_bridge_dispatch/`, and `policies/`

### P3. `src/mir` navigation-first cleanup

Goal:

- `src/mir` を crate split 前に読めるようにする

First non-destructive unit:

- strengthen `src/mir/builder/README.md`
- fix `builder/control_flow/plan/` reading order
- make the top-level map explicit:
  - `core`
  - `builder`
  - `join_ir`
  - `passes`
  - `policies`
  - `verifier`

Rule:

- entry modules and README first
- physical split later

Current activation rule:

- finish the active Loop Recipe production selection and old
  scheduler/retry/PHI-edge retirement first;
- then execute the bounded root-module rows in
  `mir-root-facade-contract-ssot.md`;
- do not copy the proposed conceptual categories into a bulk folder move.

### P4. `src/mir` physical clustering

Goal:

- giant files and local sprawl を減らす

Do:

- split oversized files
- separate helpers / tests / patterns from mixed owner files
- reduce direct deep-path reading

### P5. `src/mir` crate split preparation

Goal:

- only after P0-P4, prepare crate boundaries

Prep doc:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Future targets:

- `hakorune-mir-core`
- `hakorune-mir-builder`
- `hakorune-mir-joinir`
- `hakorune-mir-passes`

Rule:

- do not split before the public/internal API seam is documented

### P6. Naming cleanup

Goal:

- keep the historical cleanup lane parked; the actual naming cleanup batch is now phase-29cs

Rule:

- naming cleanup is late polish, not the first cleanup wave

Current naming slice:

- `phase-29cs` owns the actual kernel/plugin naming cleanup lane
- MIR substrate packages are now named `hakorune-mir-core` and `hakorune-mir-defs`
- future MIR crate candidates use the `hakorune-mir-*` naming family

## Integrated convergence order (2026-08-07)

This is a routing order, not a second task ledger. Exact row contracts remain
in their named owner SSOTs.

```text
0. pipeline-order authority unification
   -> docs/registry correction landed; old note remains supporting only

1. active Loop authority convergence
   -> RECIPE-COSEAL -> common physical path -> M8/M9 -> M10b -> M11/M12

2. MIR root topology control
   -> MIR-TOPOLOGY-REBASE0-P0 -> MIR-ROOT-MODULE-SURFACE0-G0

3. bounded physical cleanup
   -> one root visibility family -> one temporary-surface selection
   -> context owner census/decomposition -> crate-boundary recheck

4. native .hako authority migration
   -> existing selfhost inventory/parity/contract-cutover order
   -> parser handoff after MirBuilder

5. physical debt retirement
   -> Rust semantic residue, compat/bootstrap keeps, and old facade edges
      retire only from exact caller/reference evidence
```

`CURRENT_STATE` slimming and Design Registry V1/H3 classification are separate
docs-topology BoxShape series. They may run only after explicit lane selection
and may not insert work ahead of step 1. Their target is compact live pointers
and eventually strict/zero-unregistered registry state, not a second compiler
architecture.

## Non-Goals

- immediate `src/mir` crate split
- broad `nyash -> hako` rename
- mixing cleanup with active runtime/compiler blocker work
- turning `CURRENT_TASK.md` into a historical archive again

## First Safe Execution Unit

1. root hygiene contract
2. `CURRENT_TASK.md` archive/slim contract
3. `src/` / `src/mir` cleanup pointers

This is intentionally smaller than crate split.

## Acceptance

- a dedicated phase plan exists
- `CURRENT_TASK.md` points to it
- `10-Now.md` mentions the fixed order
- the P0 first batch is landed: root archive relocation + `*.err` ignore policy

## Final convergence acceptance (post-cutover)

The cleanup lane is complete only when the physical repository and the live
design pointers describe the same one-way compiler. These are end-state
conditions, not permission to open work before the active Loop cutover and
its typed function-finish design stop are complete.

- one overall pipeline authority remains:
  `mirbuilder-final-pipeline-ssot.md` owns
  `Resolve -> Observe -> Facts -> Recipe -> Verify -> Lower -> Seal -> Collect
  -> Atomic Publish`; `compiler-pipeline-ssot.md` is historical/supporting
  text only and is not a competing SSOT
- after Loop production cutover, fallback, retry, re-selection, and the
  replaced scheduler/PHI edges for that responsibility are zero; the old
  edge is deleted in the same replacement cell
- `src/mir/mod.rs` exposes durable facades only; pilot/seed/raw/legacy
  surfaces have an explicit promote, quarantine, or retire disposition and
  no unclassified root authority remains
- Rust, `.hako`, and compatibility roles are explicit: `.hako` owns language
  meaning, Rust owns MIR substrate/host/backend/oracle duties, and compat is
  an explicit quarantine with no default fallback; unnamed middle ownership
  is zero
- mixed context ownership is either decomposed into named catalogs/environments
  (for example `SourceCatalog`, `CallableCatalog`, `TypeEnvironment`, and
  `CompilationOptions`) or has a named owner, blocker, and removal condition;
  JoinIR/passes must not silently regain AST/runtime/config authority
- `CURRENT_STATE.toml`, current mirrors, task maps, and guards agree on one
  active row; the live pointer is compact and historical ledgers are not used
  as executable authority
- Design Authority Registry classification reaches strict mode with
  `unregistered_baseline = 0` only after its own consumer census and cutover;
  this cleanup document does not create a second registry or bypass that lane
- every temporary proof/receipt/adapter is either promoted to a durable owner,
  quarantined with an explicit caller boundary, or removed with a recorded
  evidence-based retirement condition
- historical D4/S-series task maps, worker consultation notes, and proof
  ledgers are archived after their owner rows close and their evidence is
  linked from the durable SSOT; current pointers retain no executable
  reference to those historical rows, and an archived row cannot be
  re-selected as the active lane
- each implementation row updates its owning reference/README and the
  current-entry mirrors in the same commit; post-cutover reference updates
  are part of completion, not a follow-up task
