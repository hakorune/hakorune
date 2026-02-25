# CURRENT_TASK (archive)

Status: SSOT  
Scope: Repo root の旧リンク互換。現行の入口は `docs/development/current/main/10-Now.md`。

- Now: `docs/development/current/main/10-Now.md`
- Backlog: `docs/development/current/main/30-Backlog.md`

---

## Handoff (current)

### 状況（SSOT）

**JoinIR 回帰 SSOT**
`./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が唯一の integration gate。phase143_* は対象外（legacy pack で隔離）。phase286_pattern9_* は legacy pack (SKIP) で運用。

**CorePlan migration 道筋 SSOT**
`docs/development/current/main/design/coreplan-migration-roadmap-ssot.md` が移行タスクの Done 判定の入口。

**Next implementation (Phase 29ao P37)**
- 目的: TBD
- 指示書: TBD
- After P37: TBD

**2025-12-30: Phase 29ao P36 COMPLETE (release adopt Pattern1 CorePlan skeleton pilot)**
Pattern1 subset を release 既定でも Facts→CorePlan(skeleton) に寄せる Stage-2 パイロットを開始した（仕様不変）。

**2025-12-30: Phase 29ao P35 COMPLETE (shadow-adopt tag coverage SSOT + pattern1 negative gate)**
shadow adopt タグの必須/禁止を SSOT 化し、Pattern1 subset reject の negative gate を回帰で固定した（仕様不変）。

**2025-12-30: Phase 29ao P34 COMPLETE (Pattern2 negative shadow adopt tag gates)**
Pattern2 の freeze/notapplicable ケースで shadow adopt タグが出ないことを回帰で固定した（仕様不変）。

**2025-12-30: Phase 29ao P33 COMPLETE (Pattern2 LoopBodyLocal planner-derive + tag gate)**
Pattern2 LoopBodyLocal を planner 由来 Pattern2Break に引き上げ、strict/dev の shadow adopt タグを回帰で固定した（仕様不変）。

**2025-12-30: Phase 29ao P32 COMPLETE (Pattern2 real-world strict/dev shadow adopt)**
Pattern2 real-world を planner subset に引き上げ、strict/dev で Facts→CorePlan shadow adopt を踏ませた（仕様不変）。

**2025-12-30: Phase 29ao P31 COMPLETE (shadow adopt routing SSOT)**
shadow adopt の判定/Fail-Fast/タグを composer 側に集約し、router を薄くした（仕様不変）。

**2025-12-30: Phase 29ao P30 COMPLETE (Shadow adopt composer SSOT)**
Facts→CorePlan の入口を `plan/composer` に集約し、Normalizer の責務を DomainPlan→CorePlan に縮退した（挙動不変）。

**2025-12-30: Phase 29ao P25 COMPLETE (Pattern5 strict/dev adopt from facts)**
Pattern5（Infinite Early-Exit）を strict/dev で Facts→CorePlan に寄せ、DomainPlan 経路との差分を Fail-Fast で検知できるようにした。

**2025-12-30: Phase 29ao P27 COMPLETE (Pattern6 subset strict/dev adopt from facts)**
Pattern6(ScanWithInit) の subset を strict/dev で Facts→CorePlan に寄せ、planner subset のズレを Fail-Fast で検知できるようにした。

**2025-12-30: Phase 29ao P28 COMPLETE (Shadow adopt observability tags + gate smokes)**
strict/dev shadow adopt が実際に踏まれていることを安定タグと回帰スモークで検証できるようにした（仕様不変）。

**2025-12-30: Phase 29ao P29 COMPLETE (Shadow adopt tag coverage)**
regression gate に含まれる全パターンで shadow adopt のタグを必須化した（仕様不変）。

**2025-12-30: Phase 29ao P26 COMPLETE (Pattern2 subset strict/dev adopt from facts)**
Pattern2(Break) の subset を strict/dev で Facts→CorePlan に寄せ、planner subset のズレを Fail-Fast で検知できるようにした。

**2025-12-30: Phase 29ao P24 COMPLETE (Pattern7 strict/dev adopt from facts)**
Pattern7（SplitScan）を strict/dev で Facts→CorePlan に寄せ、DomainPlan 経路との差分を Fail-Fast で検知できるようにした。

**2025-12-30: Phase 29ao P23 COMPLETE (Pattern3 strict/dev adopt from facts)**
Pattern3（If‑Phi）を strict/dev で Facts→CorePlan に寄せ、DomainPlan 経路との差分を Fail-Fast で検知できるようにした。

**2025-12-30: Phase 29ao P22 COMPLETE (Pattern1 CoreLoop dedup)**
Pattern1 の CoreLoop 構築を SSOT で 1 箇所に統一し、DomainPlan/Facts の二重化を解消した。

**2025-12-30: Phase 29ao P21 COMPLETE (Pattern1 subset step-only gate)**
Pattern1 subset を body=step のみに引き締め、strict/dev shadow adopt の誤マッチを遮断した。

**2025-12-30: Phase 29ao P20 COMPLETE (CoreLoop ExitMap composition SSOT)**
CoreLoop の ExitMap/Cleanup/ValueJoin 合成規約を SSOT 化し、合成境界と Fail-Fast ルールを固定した（docs-only）。

**2025-12-29: Phase 29am P0 COMPLETE (CorePlan If/Exit lowerer/verifier)**
CorePlan の If/Exit を lowerer/verifier で扱えるようにして、CorePlan 移行の土台を作った。

**2025-12-30: Phase 29ao P12 COMPLETE (Pattern7 SplitScan step join via block_params)**
Pattern7 SplitScan の step join を `Frag.block_params + EdgeArgs` で表現し、CorePhiInfo の step PHI を撤去した。

**2025-12-30: Phase 29ao P18 COMPLETE (single_planner outcome plumbing)**
single_planner が planner outcome を返す SSOT に寄せ、router の二重 planner 実行を撤去した。

**2025-12-30: Phase 29ao P17 COMPLETE (Pattern1 strict/dev shadow adopt)**
strict/dev のみ Facts→CorePlan(skeleton) を採用し、既定経路は維持した。

**2025-12-30: Phase 29ao P14 COMPLETE (Pattern2 Break exit join via block_params)**
Pattern2 Break の after join を `Frag.block_params + EdgeArgs` で表現し、CorePhiInfo の after PHI を撤去した。

**2025-12-30: Phase 29ao P16 COMPLETE (Pattern5 Break exit join via block_params)**
Pattern5 Infinite Early-Exit の after join を `Frag.block_params + EdgeArgs` で表現し、CorePhiInfo の exit PHI を撤去した。

**2025-12-29: Phase 29am P1 COMPLETE (CoreLoopPlan.body Seq flatten)**
CoreLoopPlan.body の `Seq([Effect...])` を再帰で flatten して emit できるようにした（Effect-only制約は維持）。

**2025-12-29: Phase 29am P2 COMPLETE (Verifier: Loop.body Effect-only)**
Loop.body に If/Exit/Loop が混入したら PlanVerifier で fail-fast（[V12]）。Seq-of-effects は許可。

**2025-12-29: Phase 29am P3 COMPLETE (Exit alignment: “Exit is last” rule)**
CorePlan 内の Exit の乱用を抑制し、Frag/ExitMap と整合する表現へ寄せた（[V11]）。

**PlanRuleOrder SSOT**
single_planner の順序/名前 SSOT は `src/mir/builder/control_flow/plan/single_planner/rule_order.rs` に固定。PlannerContext で Pattern1 facts の抑制を開始し、残りの guard/filter は段階移行。

**2025-12-29: Phase 29aj P6 COMPLETE (JoinIR regression gate SSOT)**
JoinIR 回帰の integration gate を phase29ae pack に固定し、phase143_* を legacy pack で隔離。

**2025-12-29: Phase 29aj P7 COMPLETE (Pattern8 planner-first)**
Pattern8 BoolPredicateScan の Facts→Planner-first を導入し、single_planner の extractor 依存を縮小。

**2025-12-29: Phase 29ak P0 COMPLETE (PlanRuleOrder + PlannerContext plumbing)**
PlanRuleOrder を SSOT 化し、PlannerContext を配線（未使用）。single_planner の手書きテーブルを撤去。

**2025-12-29: Phase 29ak P1 COMPLETE (Pattern1 facts guard via planner)**
Pattern1 以外のループで pattern1_simplewhile facts 抽出を抑制。single_planner 側の guard は安全策として維持。

**2025-12-29: Phase 29ak P2 COMPLETE (Pattern8 static box filter via planner)**
static box では Pattern8 facts 抽出を抑制。single_planner 側の filter は安全策として維持。

**2025-12-29: Phase 29ak P3 COMPLETE (remove Pattern8 filter in single_planner)**
Pattern8 static box filter を single_planner から撤去し、planner/facts 側 SSOT に一本化。

**2025-12-29: Phase 29ak P4 COMPLETE (remove Pattern1 guard in single_planner)**
Pattern1 guard を single_planner から撤去し、planner/facts 側 SSOT と fallback 抑制に統一。

**2025-12-29: Phase 29ak P5 COMPLETE (planner candidate ctx gate)**
Pattern1/8 の候補抑制を planner 側に集約し、single_planner の Pattern1 抑制を撤去。

**2025-12-29: Phase 29al P0 COMPLETE (Skeleton/Feature model SSOT, docs-only)**
CorePlan を骨格→特徴→合成で説明する SSOT を追加し、Freeze taxonomy に unstructured を追加。

**2025-12-29: Phase 29al P1 COMPLETE (post-phi final form SSOT, docs-only)**
join 値（PHI相当）の最終表現（layout/mapping/pred分類/verify）を 1 枚 SSOT として固定。

**2025-12-29: Phase 29al P2 COMPLETE (effect classification SSOT, docs-only)**
effect 分類（Pure/Mut/Io/Control）と “許される変形” の最小法典を SSOT 化。

**2025-12-29: Phase 29al P3 COMPLETE (exitkind/cleanup/effect contract SSOT, docs-only)**
cleanup を ExitKind 語彙として扱い、Control/Io を跨いだ移動や DCE 消去の事故を防ぐ境界を SSOT 化。

**2025-12-29: Phase 29aj P10 COMPLETE (single_planner unified shape)**
single_planner を全パターンで planner-first → extractor フォールバックの共通形に統一（挙動不変）。

**2025-12-29: Phase 29aj P9 COMPLETE (phase286 pattern9 legacy isolation)**
phase286_pattern9_frag_poc を legacy pack (SKIP) に隔離し、JoinIR 回帰 SSOT を phase29ae pack に固定。

**2025-12-29: Phase 29aj P8 COMPLETE (Pattern9 planner-first)**
Pattern9 AccumConstLoop の Facts→Planner-first を導入し、single_planner の extractor 依存を縮小。

**2025-12-29: Phase 29ai P15 COMPLETE (Pattern2 promotion hint observe)**
strict/dev 時のみ `[plan/pattern2/promotion_hint:{TrimSeg|DigitPos}]` を観測できるようにし、次は promotion hint の Plan/Frag 吸収残作業（P16候補）。

**2025-12-29: Phase 29aj P0 COMPLETE (PlannerOutcome observability SSOT)**
planner outcome（facts+plan）を SSOT 化し、single_planner の観測が planner facts のみに依存するように統一。

**2025-12-29: Phase 29aj P1 COMPLETE (Remove legacy_rules)**
single_planner の legacy_rules を撤去し、Pattern1/3/4/5/8/9 の抽出を plan/extractors に統一。

**2025-12-29: Phase 29aj P2 COMPLETE (Pattern1 planner-first)**
chosen_rule を撤去し、Pattern1 SimpleWhile の Facts→Planner-first を導入（仕様不変）。

**2025-12-29: Phase 29aj P3 COMPLETE (Pattern3 planner-first)**
Pattern3 If-Phi の Facts→Planner-first を導入し、single_planner の extractor 依存を縮小。

**2025-12-29: Phase 29aj P4 COMPLETE (Pattern4 planner-first)**
Pattern4 Continue の Facts→Planner-first を導入し、single_planner の extractor 依存を縮小。

**2025-12-29: Phase 29aj P5 COMPLETE (Pattern5 planner-first)**
Pattern5 Infinite Early Exit の Facts→Planner-first を導入し、single_planner の extractor 依存を縮小。

**2025-12-27: Phase 188.3 / Phase 287 P2 COMPLETE (Pattern6 nested loop: merge/latch fixes)**
Pattern6（1-level nested loop）の JoinIR→bridge→merge 経路で発生していた `undefined ValueId` と `vm step budget exceeded`（無限ループ）を解消。`apps/tests/phase1883_nested_minimal.hako` が RC=9 を返し、quick 154 PASS を維持。

- SSOT（mergeの契約）:
  - latch_incoming を記録してよいのは `TailCallKind::BackEdge` のみ（LoopEntry は上書き禁止）
  - entry-like 判定は “JoinIR MAIN のみ” を対象にする（block-id 推測はしない）: `src/mir/builder/control_flow/joinir/merge/contract_checks/entry_like_policy.rs`
  - latch 二重設定は `debug_assert!` で fail-fast（回帰検知）
- 変更箇所:
  - `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`
  - `src/mir/builder/control_flow/joinir/merge/loop_header_phi_info.rs`
- 検証（DONE）:
  - [x] 再現とPHI確認（Pattern6）
  - [x] latch記録条件を修正（BackEdgeのみ）
  - [x] デバッグ出力を撤去（恒常出力なし）
  - [x] quick/fixtureで検証（quick 154 PASS / fixture RC=9）
  - [x] docsを締めて次の指示書を用意（refactor挟み込み用）

**2025-12-26: Phase 286 P2.2 COMPLETE (Hygiene: extractor重複排除 + router小整理)**
Pattern1/Pattern4 の Plan/Frag PoC 完了後、extractor の `extract_loop_increment_plan` を `common_helpers.rs` に統一、router の 3行パターン（normalize→verify→lower）を `lower_via_plan()` ヘルパーで共通化。~65行削減、quick 154 PASS 維持。

**2025-12-26: Phase 286 P2.3 COMPLETE (Pattern9 AccumConstLoop Plan化 PoC)**
Pattern9 (AccumConstLoop) を Plan/Frag SSOT に移行完了。PHI 2本（loop_var, acc_var）、const/var 両方 OK。Pattern9 は Pattern1 より先にチェック（より具体的なパターン）。quick 154 PASS 維持。

- 現行の入口: `docs/development/current/main/10-Now.md`
- Phase 286 詳細: `docs/development/current/main/phases/phase-286/README.md`
- 次の候補: `docs/development/current/main/30-Backlog.md`
- Design goal: `docs/development/current/main/design/join-explicit-cfg-construction.md`

### 直近の道筋（JoinIR / Normalized）

### 設計方針メモ（SSOT候補）

- ExprLowererBox（式SSOT）
  - 役割: `AST(expr)` → `(prelude: Vec<Inst>, value: ValueId)`（ANF含む）
  - pure/impure/whitelist/strict の契約を集約（入口SSOT）
- ConditionLowererBox（条件→分岐SSOT）
  - 役割: `AST(cond)` → `BranchPlan`（短絡なら分岐語彙で組む）
  - 評価順は ExprLowererBox に委譲（ANFで順序固定）
  - `&&/||` は制御として扱い、式で無理しない
- ControlLowererBox（制御SSOT）
  - 役割: `StepNode/ControlTree` → JoinIR（継続 + env）
  - `if/loop` はここ、条件の中身は ConditionLowererBox に委譲

- Phase 139: if-only `post_k` の return lowering を `ReturnValueLowererBox` に統一（DONE）
  - `docs/development/current/main/phases/phase-139/README.md`
- Phase 140: `NormalizedExprLowererBox` 初版（pure expression のみ）（DONE）
  - SSOT: `docs/development/current/main/design/normalized-expr-lowering.md`
  - `docs/development/current/main/phases/phase-140/README.md`
- Phase 141 P0: impure 拡張点（contract）を SSOT 化（Call/MethodCall はまだ out-of-scope）（DONE）
  - `docs/development/current/main/phases/phase-141/README.md`
- Phase 141 P1: “既知 intrinsic だけ” を許可して段階投入（DONE）
  - `docs/development/current/main/phases/phase-141/README.md`
- Phase 141 P1.5: known intrinsic registry + available_inputs 3-source merge + diagnostics（DONE）
  - `docs/development/current/main/phases/phase-141/README.md`
- Phase 142-loopstmt P0: 正規化単位を statement（loop 1個）へ寄せる（DONE）
  - `docs/development/current/main/phases/phase-142-loopstmt/README.md`
- Phase 142-loopstmt P1: LLVM EXE smoke（同 fixture）を追加（DONE）
  - `docs/development/current/main/phases/phase-142-loopstmt/README.md`
- Phase 141 P2+: Call/MethodCall（effects + typing）を分離して段階投入
  - Historical context: `docs/development/current/main/investigations/joinir-generalization-study.md`
- Phase 143-loopvocab P0/P1: loop 内 if/break/continue の語彙追加（DONE）
  - `docs/development/current/main/phases/phase-143-loopvocab/README.md`
- Phase 143-loopvocab P2: else 対称化（B-C / C-B）（DONE）
  - `docs/development/current/main/phases/phase-143-loopvocab/README.md`
- Phase 145-anf P0/P1/P2: ANF（impure hoist + 再帰的線形化）（DONE）
  - `docs/development/current/main/phases/phase-145-anf/README.md`
- Phase 146（in progress）: Loop/If 条件式へ ANF を横展開（順序固定と診断）
  - `docs/development/current/main/phases/phase-146/README.md`

## Resolved (historical)

### WSL EXDEV / cargo build failure (resolved)

- 2025-12-18: `Invalid cross-device link (os error 18)` により `cargo build` が失敗する事象があったが、`wsl --shutdown` 再起動後に復旧。
- 再発時のワークアラウンド: `tools/build_llvm.sh` は EXDEV を避けるため `TMPDIR` を `target/...` 配下へ寄せる。
