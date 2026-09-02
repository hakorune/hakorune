---
Status: SSOT
Decision: accepted-for-final-convergence-tasking; implementation remains parked behind the active product cutover
Date: 2026-09-02
Scope: repo の物理構造を docs/設計の責務分離に追いつかせるための BoxShape cleanup 順序を固定する。即時の `src/mir` crate split や broad rename は扱わない。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/archive/phase-29cr/README.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/design/current-docs-archive-policy-ssot.md
  - docs/development/current/main/design/code-retirement-history-policy-ssot.md
---

# Repo Physical Structure Cleanup (SSOT)

## Current Capsule

- **Current decision:** physical cleanup must reduce tracked files/lines when
  it claims repository reduction; moves and renames are tracked separately.
- **Current implementation status:** the receipt-only
  `REPO-LIFECYCLE-BASELINE-REFRESH-R0` task and the first one-file
  `DOCS-HISTORY-RETIRE-R0` reduction are landed; guard retirement remains
  parked. Hako physical ingress remains
  `ParkedSealed__HakoIngressMissing`.
- **Next ordered task:** keep broad guard/docs retirement parked until another
  finite reference-zero family has an equal-or-stronger successor; no bulk
  archive or phase purge is implied.
- **Production stop line:** no cleanup row changes language/compiler behavior,
  current authority, a selected backend, or an unresolved test/guard contract.
- **Retirement finish line:** every selected batch has caller/reference zero,
  equal-or-stronger retained evidence, no copied full-body archive, and
  strictly lower tracked file and line counts.

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

### Absolute reduction refresh (2026-09-01)

The reduction baseline measured at `9ed98f1088` is tracked content, not every
physical file in the checkout:

```text
tracked docs                          = 13,262 files / 1,692,367 lines
tracked Markdown                      = 11,456 files / 1,325,375 lines
current phase Markdown                =  5,428 files /   451,340 lines
current investigations               =  1,028 files /   294,162 lines
tools/checks all tracked paths        =  3,804 files /   486,483 lines
tools/checks Python+shell scripts     =  3,583 files /   407,012 lines
legacy-tests pre-retirement snapshot  =     34 files /     3,653 lines
tracked generated Hako artifacts      =    108 files /    13,461 lines
Rust MIR interpreter                  =     93 files /    18,449 lines
```

The physical docs count is 17,530 files / 2,016,957 lines, but its untracked
`docs/private` component is outside this SSOT. Moving tracked history from one
repo directory to another changes navigation, not the absolute footprint.

### `REPO-ABSOLUTE-REDUCTION-R0` parked task train

This train is nonblocking cleanup and does not preempt the selected MirBuilder
production vertical. It may run only when `CURRENT_STATE.toml` explicitly
selects a clean-worktree cleanup window. The source/guard/docs reductions are
separate bounded commits; they never share a semantic implementation commit.

```text
REPO-LIFECYCLE-BASELINE-REFRESH-R0
  -> GUARD-FAMILY-RETIREMENT-R0
  -> DOCS-HISTORY-RETIRE-R0
  -> GENERATED-ARTIFACT-RETENTION-D0
```

1. `REPO-LIFECYCLE-BASELINE-REFRESH-R0` landed after adjudicating the strict
   lifecycle manifest. The generated and committed receipt now agrees at
   `13,262` docs, `3,807` checks, `4,687` src paths, and `7,427` tools paths;
   registry/allowlist violations were `0`.

### `REPO-LIFECYCLE-BASELINE-REFRESH-R0` (landed)

This was a one-time BoxShape/receipt task, not a repository reduction batch
and not a MirBuilder semantic implementation. It ran only after
`CURRENT_STATE.toml` selected the row in a clean worktree.

```text
Decision:
  Reconcile the generated lifecycle receipt with the current tracked tree once;
  do not delete, archive, rename, or reinterpret any artifact in this row.
Source authority + canonical issuer:
  repository_artifact_lifecycle_inventory.py over the current git tree;
  the generated JSON is the sole receipt writer.
Non-authority:
  the stale receipt, LOC/grep counts, unknown_retain labels, --write itself,
  warning-only guards, and any presumed historical age.
Fail-fast boundary:
  classify every changed count/registry entry and verify allowlist/strict
  consistency; any unclassified drift or registry violation stops the row.
Smallest next slice:
  compare a temporary generated receipt, record the finite drift, write the
  existing manifest once, then run the existing docs-slim/pointer/diff guards.
Non-claims:
  no archive/delete, no test or guard retirement, no compiler/backend/MIR
  change, no baseline health claim, and no new guard or receipt family.
Census boundary: committed lifecycle JSON -> generator output over the current
  git tree; includes counts, registry, and allowlist drift, excludes artifact
  deletion, archive copies, unknown-retain adjudication, and semantic lanes.
```

Finite preflight recorded by the worker audit:

```text
counts: checks 3662 -> 3807; docs 13023 -> 13262;
        current 8803 -> 9039; src 4490 -> 4687; tools 7283 -> 7427
registry/allowlist violations: 0
permitted files: lifecycle manifest, cleanup SSOT, lifecycle workstream,
                 CURRENT_STATE pointer
```

Acceptance was one receipt update whose strict check passed, with the same
generated schema and no deletion/archive or semantic diff. The pointer has
returned to the Hako `ParkedSealed` boundary; the next guard/docs retirement
batch still requires a separate explicit selection and finite
caller/reference-zero evidence.

### `GUARD-FAMILY-RETIREMENT-R0` (ParkedSealed)

The read-only guard census found no new deletion-safe family. Keep this train
parked; do not convert `unknown_retain` into delete permission or remove an
existing navigation tombstone.

```text
Decision:
  NoSafeSlice; no guard deletion row is opened.
Source authority + canonical issuer:
  guard_surface_inventory.py, guard_rows.toml, docs/check index, and proof_apps.
Non-authority:
  unknown_retain, age, file name, pilot profile, grep absence, or tombstone text.
Fail-fast boundary:
  finite owner + caller/reference-zero + equal-or-stronger successor +
  positive/negative evidence + observable reopen trigger must all exist.
Smallest next slice:
  run one fresh bounded recensus only after a named successor and old family
  appear; otherwise remain parked.
Non-claims:
  no guard/index/tombstone deletion, no new guard or receipt, and no compiler,
  MIR, test, or backend change.
Census boundary: tools/checks tracked guard surface -> docs index/registry/
  proof_apps/quick profile; includes active and historical guard ownership,
  excludes unknown-retain adjudication and semantic lanes.
```

Current machine census:

```text
rows=3807; stable_public_entry=94; family_manifest_case=115;
focused_behavior_test=212; unknown_retain=3386;
historical_archive=0; delete_after_equivalent=0;
index_untracked=0; manifest_untracked=0
```

The two closest rows are already retired (`13ca2339a3`) with the successor
`hako_bounded_body_loop_feature_summary_v0_guard.sh`; their tombstones remain
the evidence owner. No new delete set exists at this boundary.
Closed tombstone: `LEGACY-TESTS-RETIRE-R0` landed at `bcc9a6ba65`. The disabled
feature, four cfg barrels, 34 roots, and nine exclusive support files were
retired from the tree; Git history owns the detailed delete set. It is not a
future cleanup-train step.

### `DOCS-HISTORY-RETIRE-R0` (landed)

This was the first absolute-reduction candidate after the receipt refresh. It
uses `RetireFromTree`, not a tracked archive copy: the candidate is a
superseded 75-line investigation with no current pointer, lifecycle, filename,
or title references, and its surviving ProgramRoot owner plus duplicate
validator are both below the 760-line split threshold.

```text
Decision:
  Delete exactly one reference-closed investigation body; Git history keeps
  the detail and no archive/stub is added.
Source authority + canonical issuer:
  current ProgramRootWorkPlan owner, landed ffcae72725 validator, and the
  existing lifecycle inventory/reference scan.
Non-authority:
  age, line count alone, filename, worker silence, archive copies, broad counts.
Fail-fast boundary:
  current pointer, tracked status, status/superseded_by evidence, inbound refs,
  successor line budgets, and strict receipt must all remain unchanged.
Smallest next slice:
  remove `docs/development/current/main/investigations/mir-call-d1b-program-root-toplevel-work-split-r0-2026-08-26.toml`,
  regenerate the existing receipt, run the existing guards, and return to Hako.
Non-claims:
  no phase/investigation bulk purge, archive migration, semantic/compiler/MIR,
  guard/test retirement, or whole-repository health claim.
Census boundary: one tracked current investigation -> inbound refs and landed
  successor; excludes all other docs, phases, archives, and source lanes.
```

Acceptance is `-1` tracked docs file and `-75` tracked lines for this selected
batch, strict lifecycle/docs-slim/pointer/active-surface/diff checks green, and
the pointer returned to the Hako `ParkedSealed` boundary. If any additional
candidate or reference appears, keep this row parked and do not open another
cleanup card.

Implementation evidence: the candidate was deleted with no archive copy; the
selected batch measured one tracked file/75 lines before and zero afterward.
The generated lifecycle receipt and existing guards were refreshed in the same
closeout, and no compiler, MIR, test, or guard semantics changed.

2. `GUARD-FAMILY-RETIREMENT-R0` reuses
   `guard_surface_inventory.py`. Its current 3,804-row inventory classifies
   94 stable entries, 114 manifest cases, 212 focused behavior tests, and
   3,384 `unknown_retain` rows. Unknown is not delete permission. Each batch
   must select one finite family, install or identify equal-or-stronger
   central coverage, reach caller-zero, delete the superseded scripts and
   index rows, and strictly reduce both script count and script lines. It may
   not add a new guard to prove guard deletion.
3. `DOCS-HISTORY-RETIRE-R0` extends existing R5/R6 lifecycle work with the
   `RetireFromTree` / `Tombstone` dispositions from the archive policy. The
   first batch takes one reference-closed phase or investigation cluster and
   strictly reduces tracked docs files and lines. A move into another tracked
   archive is navigation progress only and does not close this row.
4. `GENERATED-ARTIFACT-RETENTION-D0` classifies the 108 generated files by
   generator, canonical input, reproducibility, production caller, and release
   need. Only reproducible caller-zero copies may be deleted; missing generator
   ownership is `UnknownRetain`, not a reason to regenerate or remove them.

The landed MirBuilder test-inventory retirement and later family-owned
consolidations own root test-count reduction. VM removal remains the existing
post-Call MS3 lane; this train does not create a second VM retirement owner.

Every implementation batch has one measurable finish line:

```text
selected tracked files  before > after
selected tracked lines  before > after
replacement coverage / current authority remains green
unclassified deletion candidates = 0
copied full-body archive replacement = 0
```

A census-only, docs-only, rename-only, or net-zero move batch is progress zero.

## Family Repository Surface Budget

Repository reduction is part of each production family's closeout, not a
future broad-cleanup promise. This is a local repayment check: it removes only
assets exclusively owned by the family that just switched. It does not insert
a general cleanup lane ahead of the next product row.

One before/after receipt records:

```text
tracked files / tracked lines
production files/lines; family tests/proofs; guard owner/cases
active card/manifest lines; temporary assets; retired family assets
```

```text
delete/shrink tracked content          -> reduction credit
rename/move/archive copy               -> zero credit
new durable contract test/reference    -> retained, not temporary debt
caller-zero duplicate/superseded asset -> retire before next family
```

Close only after classifying every positive asset. Durable reference/tests or
one reusable lane guard may remain with one owner. Temporary proofs, row
guards, adapters, compatibility fixtures, and disconnected receipts are
deleted in-series or carry one finite repayment row and `retire_when`;
unresolved temporary delta blocks the next family.

Replacement and retirement series default to non-positive total tracked-file
and tracked-line deltas. A genuinely new T2 language/ABI capability may be
positive only when its durable assets are named and no parallel alias,
authority, route, or proof family remains. LOC reduction never authorizes
deleting a required language contract, reference, test, or guard.

```text
caller switch + old edge deletion
  -> family-owned asset retirement
  -> tracked files/lines remeasure
  -> no unclassified or temporary positive delta
  -> next family may be selected
```

Closed execution prose defaults to Git history. `Tombstone` and tracked
archive retention remain exceptions owned by
`current-docs-archive-policy-ssot.md`; neither counts as absolute reduction
unless tracked files and lines actually decrease without a copied full body.

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
- `src/mir/builder/control_flow/plan/normalizer/helpers_value/mod.rs` facade
- `src/mir/builder/control_flow/plan/normalizer/helpers_value/lower.rs`
- `src/mir/builder/control_flow/plan/normalizer/helpers_value/variant.rs`
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
   -> authority-role census -> MIR-CONTEXT-OWNER-CENSUS0-D0
   -> bounded MIR-CONTEXT-OWNER-SPLIT0-S0 when a seam is safe
   -> crate-boundary recheck

4. native .hako authority migration
   -> MIR-AUTHORITY-ROLE-MANIFEST0-D0
   -> Rust/.hako/compat role manifest -> existing selfhost
      inventory/parity/contract-cutover order
   -> parser handoff after MirBuilder

5. physical debt retirement
   -> MIR-LEGACY-JOINMODULE-DISPOSITION0-D0
   -> Rust semantic residue, compat/bootstrap keeps, and old facade edges
      retire only from exact caller/reference evidence

6. terminal convergence audit
   -> REPO-FINAL-CONVERGENCE-AUDIT0-G0
   -> R4 final conformance may close only after this audit is green
```

`CURRENT_STATE` slimming and Design Registry V1/H3 classification are separate
docs-topology BoxShape series. They may run only after explicit lane selection
and may not insert work ahead of step 1. Their target is compact live pointers
and eventually strict/zero-unregistered registry state, not a second compiler
architecture.

### Parked final-convergence task cells

These cells are owned by this SSOT and remain parked until the integrated order
reaches them. They are named here so the terminal audit cannot silently assume
that role ownership or the legacy JoinModule disposition is already complete.

#### `MIR-AUTHORITY-ROLE-MANIFEST0-D0`

Create one machine-readable manifest for the final authority-role census over
`src/**`, `lang/**`, and explicitly named compatibility/bootstrap/tooling roots.
Each record classifies `meaning`, `substrate`, `host/backend`, `oracle`, or
explicit `quarantine`, and records exclusions and the no-default-fallback
rule. This is classification only: no source route, resolver, lowering, or
runtime behavior changes. The row is complete only when every in-scope root
has one role, unnamed middle ownership is zero, the role manifest is linked by
the final audit, and README/current-entry/guard-index updates are synchronized.

#### `MIR-LEGACY-JOINMODULE-DISPOSITION0-D0`

Produce the single Legacy JoinModule disposition receipt required by the final
audit. Every legacy JoinModule producer/consumer is classified as `retain`,
`quarantine`, or `retire`, with one owner, exact caller/reference evidence,
default-fallback status, and a removal condition. The row may not delete or
activate a route; it only blocks the final audit from treating this boundary
as implicit. Its receipt, owner README, exact reference delta (or
`reference_delta = 0`), guard index, and current mirror update together.

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

### `REPO-FINAL-CONVERGENCE-AUDIT0-G0`

This is the single terminal audit for the repository-shape cleanup. It is not
a new compiler architecture, a bulk-delete permission, or a second task
ledger. It runs only after Loop production selection, M10b, M11, and M12 are
closed, and it records one machine-readable disposition matrix for the exact
end-state claims below.

Required audit outputs:

1. **Pipeline authority** — `mirbuilder-final-pipeline-ssot.md` is the only
   global order authority; `compiler-pipeline-ssot.md` is historical/supporting
   and has no executable pointer.
2. **MIR root surface** — `src/mir/mod.rs` has durable facade exports only;
   every `pilot`/`seed`/`raw`/`compat`/`legacy` declaration is promoted,
   quarantined, or retired with an owner and removal condition.
3. **Authority roles** — a checked manifest classifies Rust, `.hako`, and
   compatibility/bootstrap paths as meaning, substrate/host/backend/oracle,
   or explicit quarantine. The census scope is `src/**`, `lang/**`, and the
   explicitly named compatibility/bootstrap/tooling roots; the classification
   enum and exclusions are recorded in the manifest. Unnamed middle ownership
   and default fallback are zero. Unknown external callers remain a
   non-claim, never an excuse for a compatibility edge.
4. **Context owners** — each mixed context field has one named catalog,
   environment, options, session, or an explicit blocker/removal condition;
   no JoinIR/pass layer silently regains AST/runtime/config authority.
5. **Live pointers** — `CURRENT_STATE.toml`, current mirrors, task maps, and
   guards agree on one active row; historical ledgers are not executable
   authority. The live schema and Design Registry status are checked through
   their own consumer-census/cutover owners.
6. **Temporary evidence** — every proof/receipt/adapter has a promote,
   quarantine, or retire disposition; closed D4/S-series material is archived
   and cannot be selected by a current pointer. The matrix also records
   `loop_coverage_parity` and `legacy_joinmodule_disposition` explicitly;
   neither may remain an implicit status claim.
7. **Documentation parity** — each landed implementation row has its owning
   README, exact `docs/reference/**` contract (or an explicit
   `reference_delta = 0` record), guard index, and current mirror updated in
   the same commit. Post-cutover reference updates are part of the row, not a
   follow-up.

The matrix is written to
`docs/development/current/main/investigations/repo-final-convergence-audit0-g0-disposition.toml`.
Its sole owner is this terminal audit row, and every record has at least:
`path`, `role`, `owner`, `lifecycle`, `caller_class`, `default_fallback`,
`retire_when`, `evidence`, `loop_coverage_parity`, and
`legacy_joinmodule_disposition`. Unknown fields are rejected; duplicate paths,
unclassified records, and missing evidence are zero. The implementation row
adds `tools/checks/repo_final_convergence_guard.sh` and registers it in
`docs/tools/check-scripts-index.md`; no earlier row may claim this guard.

The current-entry parity set is explicit: `CURRENT_STATE.toml`,
`CURRENT_TASK.md`, `docs/development/current/main/05-Restart-Quick-Resume.md`,
and `docs/development/current/main/10-Now.md`. A mirror is green only when it
points to the same active row and blocker; historical detail remains outside
the executable pointer.

The audit opens only after these dependencies are closed:

```text
LOOP-PRODUCTION-SELECTION-D0 (or its accepted successor)
M8/M9 coverage and parity closeout
Legacy JoinModule disposition receipt
M10b activation
M11 legacy retirement
M12 adapter retirement
CURRENT-STATE-LIVE-CUTOVER0-I0-R0
Design Registry consumer/cutover row
```

If a context seam is still unresolved, `MIR-CONTEXT-OWNER-CENSUS0-D0` may
close with a named blocker and `removal_when`; `MIR-CONTEXT-OWNER-SPLIT0-S0`
is opened only for a behavior-neutral bounded seam. The terminal audit never
silently treats an unresolved field as clean.

Acceptance commands are the existing guards and inventories, not a new
semantic checker:

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/mir_root_import_hygiene_guard.sh
python3 tools/docs/repository_artifact_lifecycle_inventory.py --check --strict
git diff --check
```

Any red item reopens its owning bounded row. The audit must not be made green
by adding a fallback, inflating a baseline, weakening a guard, or deleting an
unclassified file.

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
