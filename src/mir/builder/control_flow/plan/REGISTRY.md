# Plan Registry (SSOT helper)

See also: `src/mir/builder/control_flow/plan/ARCHITECTURE.md`
See also: `src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md`

目的: JoinIR/CorePlan の “箱（plan rule）” が増えても迷子にならないように、入口・責務・SSOT を 1 枚に固定する。

前提:
- release default は不変
- strict/dev + `HAKO_JOINIR_PLANNER_REQUIRED=1` のときだけ受理範囲を増やす（silent fallback なし）
- no AST rewrite（見かけ等価の式変形・コード移動は禁止。analysis-only view は可）

## Daily entry (fast)

- Fast gate (Phase 29bq): `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- Dev gate (checkpoint): `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`

## Boxes (high-level)

| Box / rule | Scope | What it unlocks | SSOT |
|---|---|---|---|
| `generic_loop_v0` | release + strict/dev | basic loop(cond) skeletons | `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md` |
| `generic_loop_v1` | strict/dev + planner_required | loop body as CorePlan tree (If/Exit), minimal carriers | `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md` |
| `loop_true_break_continue` | strict/dev + planner_required (fallback-only) | `loop(true)` with `break/continue/return(value)` in ExitIf, plus unconditional tail `return(value)` (return+prelude+else は禁止) | `src/mir/builder/control_flow/plan/loop_true_break_continue/README.md` |
| `loop_cond_break_continue` | strict/dev + planner_required | loop(cond) with multi `ExitIf(break/continue)` and conditional update/join; cluster3/4/5 are selected via `facts/nested_loop_profile.rs` | `docs/development/current/main/design/loop-cond-break-continue-ssot.md` |
| `loop_scan_v0` | strict/dev + planner_required | loop(cond) scan: `ch=substring(i,i+1)` + comma-continue + close-break + tail `i=i+1` (one-shape) | `src/mir/builder/control_flow/plan/loop_scan_v0/` |
| `loop_scan_methods_v0` | strict/dev + planner_required | FuncScannerBox._scan_methods outer loop(cond) with nested loops/ifs (one-shape) | `src/mir/builder/control_flow/plan/loop_scan_methods_v0/` |
| `loop_scan_methods_block_v0` | strict/dev + planner_required | scan_methods loop(cond) where scan window inner loop is wrapped by a block stmt (`{ ... }`) (one-shape) | `src/mir/builder/control_flow/plan/loop_scan_methods_block_v0/` |
| `loop_scan_phi_vars_v0` | strict/dev + planner_required | loop(i<n) outer loop with nested break-search-loop + found-if + collect-loop (one-shape, selfhost _collect_phi_vars) | `src/mir/builder/control_flow/plan/loop_scan_phi_vars_v0/` |
| `loop_collect_using_entries_v0` | strict/dev + planner_required | loop(pos<n) with var-to-var step (`pos = next_pos`) and if/else chain (one-shape, Stage1UsingResolverBox._collect_using_entries) | `src/mir/builder/control_flow/plan/loop_collect_using_entries_v0/` |
| `loop_bundle_resolver_v0` | strict/dev + planner_required | loop(i<n) with loop-local step var `i = next_i` and nested return (one-shape, BundleResolver.resolve/4; generic_loop_v* can't match var-to-var step) | `src/mir/builder/control_flow/plan/loop_bundle_resolver_v0/` |
| `loop_cond_continue_only` | strict/dev + planner_required | loop(cond) continue-only (nested if/continue) | `src/mir/builder/control_flow/plan/loop_cond_unified/variants/continue_only/` |
| `loop_cond_continue_with_return` | strict/dev + planner_required | loop(cond) continue-only with nested return | `src/mir/builder/control_flow/plan/loop_cond_unified/variants/continue_with_return/` |
| `loop_cond_return_in_body` | strict/dev + planner_required | loop(cond) with nested return and no break/continue (fixture-derived) | `src/mir/builder/control_flow/plan/loop_cond_unified/variants/return_in_body/` |
| `scopebox_seq_flatten` | analysis-only (ScopeBox opt-in) | treat nested `ScopeBox` Block chains as Seq(n-ary) for planner facts | `src/mir/builder/control_flow/plan/facts/stmt_view.rs` |

## loop_*_v0 audit snapshot (CLEAN-PLAN-V0-AUDIT-1, 2026-02-08)

目的: `loop_*_v0` の “実配線（facts→router→compose）” を inventory 化し、削除/縮退の起点を固定する。

### Active (routed)

| Box | Route status | Classification | Evidence / next |
|---|---|---|---|
| `loop_scan_v0` | routed (`registry/mod.rs` + handlers + composer) | keep | Phase C15 recipe-first active |
| `loop_scan_methods_v0` | routed (`registry/mod.rs` + handlers + composer) | keep | Phase C15 recipe-first active |
| `loop_scan_methods_block_v0` | routed (`registry/mod.rs` + handlers + composer) | keep | disjoint predicate with `loop_scan_methods_v0` |
| `loop_scan_phi_vars_v0` | routed (`registry/mod.rs` + handlers + composer) | keep | selfhost blocker path pinned |
| `loop_collect_using_entries_v0` | routed (`registry/mod.rs` + handlers + composer) | keep | Phase C16 recipe-first active |
| `loop_bundle_resolver_v0` | routed (`registry/mod.rs` + handlers + composer) | keep | Phase C16 recipe-first active |

### Retired

| Box | Status | Evidence |
|---|---|---|
| `loop_flag_exit_v0` | removed (no facts field / no module) | `CLEAN-PLAN-V0-REMOVE-1` で物理撤去（`plan/mod.rs` + `facts/loop_types.rs` + `loop_flag_exit_v0/*`） |

Audit rule (active運用):
- routed が無い box は “活性箱” 扱いにしない。削除・縮退は別コミットで fixture/gate を固定して行う。

### Retired History (read-only)

- `loop_flag_exit_v0` は `CLEAN-PLAN-V0-REMOVE-1` で物理撤去済み。
- removal boundary（`CLEAN-PLAN-V0-SHRINK-2`）の詳細ファイル群は、`CURRENT_TASK.md` の同名セッション記録を参照する。

## Entry SSOT (router→planner→composer→lower)

Single entry pipeline for loop routing (no bypass):

`route_loop` → `single_planner::try_build_outcome` → recipe/composer adopt path → plan lowerer

Router: active module surface `crate::mir::builder::control_flow::joinir::route_entry::router`
(physical path: `src/mir/builder/control_flow/joinir/route_entry/router.rs`)  
Planner: `src/mir/builder/control_flow/plan/single_planner/`  
Composer (adopt ordering SSOT): `src/mir/builder/control_flow/plan/composer/`  
Lower: `src/mir/builder/control_flow/plan/lowerer/`

## Notes (avoid box explosion)

- Canon (analysis-only) lives in `src/mir/builder/control_flow/plan/canon/` (phase-29bq migration; legacy layouts remain in box folders).
- Skeletons (blocks/frags) live in `src/mir/builder/control_flow/plan/skeletons/`.
- Features (delta apply) live in `src/mir/builder/control_flow/plan/features/`.
- `generic_loop_v0/v1`: skeleton=`plan/skeletons/generic_loop.rs`, pipeline=`plan/features/generic_loop_pipeline.rs` (body=`features/generic_loop_body.rs` → cond=`features/generic_loop_step.rs::apply_generic_loop_condition` (pre-body map) → step=`features/generic_loop_step.rs::apply_generic_loop_step` (post-body map) → carriers finalize[v1]).
- `loop_true_break_continue`: pipeline=`plan/features/loop_true_break_continue_pipeline.rs` (skeleton=`plan/skeletons/loop_true.rs`, exit=`features/exit_if_map.rs` + `features/exit_branch.rs` (`ContinueWithPhiArgs`), nested=`features/nested_loop_depth1.rs` with depth1 loop(true)/loop(cond) support).
- `loop_cond_break_continue`: pipeline=`plan/features/loop_cond_break_continue_pipeline.rs` (exit-map=`features/exit_if_map.rs`, join=`features/conditional_update_join.rs`, carrier=`features/carrier_merge.rs`).
- `loop_cond_continue_only`: pipeline=`plan/features/loop_cond_continue_only_pipeline.rs` (continue-if recipe + carrier join, no ExitIfMap dependency).
- `loop_cond_continue_with_return`: pipeline=`plan/features/loop_cond_continue_with_return_pipeline.rs` (continue-if + hetero-return-if + CoreIfJoin merge).
- `loop_cond_return_in_body`: pipeline=`plan/features/loop_cond_return_in_body_pipeline.rs` (return-in-body + CoreIfJoin merge).
- `coreloop_skeleton`: template=`plan/features/coreloop_skeleton/` (SSOT template for Standard5 loop structure with carrier PHI management; pipelines reuse via `build_coreloop_frame` / `build_header_step_phis` / `build_continue_with_phi_args`).
- `scan_with_init`: skeleton=`plan/skeletons/scan_with_init.rs`, pipeline=`plan/features/scan_with_init_pipeline.rs` (ops=`features/scan_with_init_ops.rs`).
- `split_scan`: skeleton=`plan/skeletons/split_scan.rs`, pipeline=`plan/features/split_scan_pipeline.rs` (ops=`features/split_scan_ops.rs`, emit/match=`features/split_emit.rs`).
- `loop_true_early_exit`: skeleton=`plan/skeletons/loop_true.rs`, recipe=`plan/recipe_tree/loop_true_early_exit_{builder,composer}.rs` (semantic route=`loop_true_early_exit`; no dedicated feature pipeline file remains).
- scan/split pipeline order (SSOT): skeleton → ops → (split_emit for split) → if_join (join args/phis) → edgecfg_stubs (branches) → carrier_merge (final_values).
- loop phis are attached via `features/loop_carriers.rs::with_loop_carriers` (ops must not set `phis` directly).
- carrier collection is centralized in `features/carriers.rs` (pipelines pass carrier lists to carrier_merge/conditional_update_join).
- legacy scan/split logic moved: normalizer logic → `features/scan_with_init_ops.rs` / `features/split_scan_ops.rs` (pipeline entry only).
- JoinFeature (if-join PHI insertion) is `src/mir/builder/control_flow/plan/features/if_join.rs`.
- “continue 経路の合流” は plan rule を増やして個別対応せず、CorePlan の primitive（例: `ContinueWithPhiArgs`）へ寄せる。
- `continue` の飛び先が “常に step_bb” だと箱ごとに例外が増えるため、ContinueTarget slot（`continue_target_bb`）を土台として先に入れる（SSOT: `docs/development/current/main/design/coreloop-continue-target-slot-ssot.md`）。
- 候補競合は “ルール順” より “責務スロット” で解決する（例: fallback-only / requires-carrier-merge / forbids-value-join）。
- `loop_cond_break_continue` は handled-guard (`if handled==0 { break }`) と continue-if-with-fallthrough-else を許可する土台として拡張中。
- `loop_cond_break_continue` の正規化は `features/exit_if_map.rs` / `features/conditional_update_join.rs` / `features/carrier_merge.rs` に分解済み。
- `loop_cond_break_continue` の Program block（ASTNode::Program）は container として展開し、展開後 stmt を同じルールで判定する。
- `loop_cond_break_continue` の general-if は nested loop を許可する（branch に top-level exit が無いこと）。
- `loop_cond_break_continue` の else-only-break は base 受理（`IfMode::ElseOnlyExit` / `IfContractKind::ExitAllowed`、planner_required に限定しない）。
- `loop_cond_break_continue` は accept_kind で Facts->Lower 契約を固定し、Lower は全 variant を match する（局所契約であり他箱へは横展開しない）。
- `loop_cond_break_continue` の `ExitIfTree`（Phase 29bq BoxCount）: nested if-in-loop で全枝が exit で終わる再帰構造を受理。Recipe: `ExitKind`/`ExitLeaf`/`ExitIfTree`, Pipeline: `item_lowering.rs` + `else_patterns.rs::lower_exit_if_tree`。
- loop_cond 系 Facts は `src/mir/builder/control_flow/plan/loop_cond_unified/variants/` に集約（Level 1 cleanup完了：旧 wrapper directories 削除済み、recipes も unified module に移動）。
- loop_cond の共通 helper SSOT は `src/mir/builder/control_flow/plan/loop_cond_unified/REGISTRY.md` に集約。
- cluster3/4/5 は **profile 駆動**（`CLUSTER_PROFILES`）で loop_cond_break_continue に統合済み。
  - planner_tag（TSV rule 文字列）は CLUSTER_PROFILES から引き継ぎ、挙動不変。
  - 追加点は `facts/nested_loop_profile.rs` の const テーブル1箇所。
- `nested_loop_depth1_methodcall` は method call を含む depth=1 の nested loop を単独受理する（loop_cond_break_continue の nested 入口）。
- `nested_loop_depth1_break_continue_pure` は break/continue を含む depth=1 の nested loop を単独受理する（call無し、末尾continueを許可）。
- `nested_loop_depth1_no_break_or_continue` は break/continue を含まない depth=1 の nested loop を単独受理する（call stmt を含む場合のみ）。
- `nested_loop_depth1_no_break_or_continue_pure` は break/continue と call を含まない depth=1 の nested loop を単独受理する。
- Phase 29bv targets (SSOT): scan/split normalizer cleanup (`normalizer/pattern_scan_with_init.rs`, `normalizer/pattern_split_scan.rs`; 入口: `docs/development/current/main/phases/phase-29bv/README.md`)。

## Decomposition candidates (planned cleanup)

目的: “pattern名の増殖”ではなく、`Skeleton + FeatureSet` 合成に寄せてコンパイラをシュッとさせる。
SSOT: `docs/development/current/main/design/coreplan-skeleton-feature-model.md`

- `generic_loop_v0/v1`: loop_var/step/exit/carrier/step_mode を Feature に分離（Facts→Canon→Normalize の責務を細くする）
- `scan_with_init` / `split_scan`: アルゴリズム意図（scan/split）と LoopSkeleton を分離
- `loop_true_early_exit`: `loop(true)` skeleton + exit_if + carrier update に分離（Phase 29bw: pipeline 化済み）
- `if_phi_join`: IfSkeleton + JoinFeature（CoreIfJoin）の pred 収集を1箇所に集約
- `loop_true_break_continue`: LoopTrueSkeleton + ExitIfMapFeature + NestedLoopFeature(depth<=1) に寄せ、fallback-only 条件は planner 側の slot へ
- `loop_true_break_continue` の旧正規化ロジックは削除済み（feature 経由の入口のみ）。
- `loop_cond_break_continue`: LoopCondSkeleton + (ExitIfMap/ConditionalUpdate/CarrierMerge/GuardBreak) を Feature 化して肥大化を防ぐ
- `ExitBranch`（planned）: If/BranchN/Loop 内の “exit 付きブランチ” を共通 feature として抽出し、exit_if/match/loop の重複を減らす（“例外パターンの堆積” を防ぐ）
  - SSOT: `docs/development/current/main/design/exitbranch-ssot.md`
- helper boundary SSOT（join/exit/phi/carrier の“一箇所化”ルール）:
  - `docs/development/current/main/design/feature-helper-boundary-ssot.md`

## Remaining legacy normalizers / vocabulary hotspots (planned lego-ization)

目的: selfhost canary の “1箱ずつ拡張” が例外パターンの堆積（負債）にならないよう、未レゴ化（未pipeline化）の残りを SSOT として可視化する。

原則:
- “拡張”は strict/dev + planner_required に閉じる（release 既定は不変）
- legacy normalizer に分岐を足さない（必要なら pipeline/skeleton/feature へ昇格）
- 同種の拡張が 2 回目に入ったら、まず “レゴ化（pipeline化）” を検討する（例外の増殖を止める）

| Legacy normalizer | Current state | Planned direction | Promotion trigger |
|---|---|---|---|
| `normalizer/simple_while_coreloop_builder.rs` | legacy helper lane | `generic_loop_v0/v1` へ寄せるか、`LoopSkeleton + ExitMap/ValueJoin` 合成へ分解 | simple-while helper に route-specific 分岐を足したくなった時点 |
| `normalizer/loop_break.rs` | test-only loop_break harness | `ExitMap` feature へ（“別pattern” を増やさない） | break/return 形の拡張が必要になった時点 |
| `normalizer/pattern_*`（string/helpers: skip_ws/split_lines/escape_map/is_integer/int_to_str/starts_with 等） | legacy（小粒） | 直近は現状維持（小粒）。将来は `generic_loop_v1`/FlowBox へ吸収候補 | 2 箇所以上で同型の手書き合流が増えた時点 |

Note:
- `bool_predicate_scan` / `accum_const_loop` は facts + recipe_tree entry へ収束済みで、dedicated normalizer file は現行 tree に存在しない。

Hard patterns (special-rule required):
- irreducible/multi-entry loop, unwind/finally, coroutine/yield 等は “例外で通す” ではなく Freeze taxonomy + SSOT で扱う（入口: `docs/development/current/main/design/coreplan-skeleton-feature-model.md` の Section 3）。

## Lego-ization Plan (design-only)

- Rule: 1 normalizer = 1 pipeline + 1 skeleton + 1 feature (最小分割)
- Order:
  1) 最も entry が多いものから分割
  2) 例外経路のないものを優先
  3) 既に parts 依存が薄いものを優先
- Invariants:
  - 挙動不変
  - planner_required でのみ有効化（必要なら）
  - 受理拡張なし

## How to add a new box (template)

1. Fixture を 1 本作る（`apps/tests/phase29bq_*_min.hako`）
2. gate を 1 本追加し、`phase29bq_fast_gate_cases.tsv` に入れる
3. SSOT（design or phase README）に「受理条件 / 禁止事項 / 期待 stdout/RC」を 5 行で固定
4. `phase29bq_fast_gate_vm.sh` → `phase29bp_planner_required_dev_gate_v4_vm.sh` で緑維持を確認

Checklist:
- [ ] fixture 1本
- [ ] gate 1本（fast gate listに追加）
- [ ] SSOT 5行
- [ ] fast/dev gate green
