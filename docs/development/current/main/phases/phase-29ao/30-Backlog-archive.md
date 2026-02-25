# Self Current Task — Backlog (archive)

Status: Active  
Scope: 「次にやる候補」を短く列挙するメモ。現状は `docs/development/current/main/10-Now.md` を入口にする。  
Related:
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/DOCS_LAYOUT.md`

## 直近（JoinIR/selfhost）

- **Phase 29an（✅ COMPLETE）: Skeleton/Feature Facts（SSOT）**
  - 入口: `docs/development/current/main/phases/phase-29an/README.md`
  - 状況: P0–P14 ✅ 完了 / P15 closeout ✅
  - Gate: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

- **Phase 29ao（active）: CorePlan composition from Skeleton/Feature**
  - 入口: `docs/development/current/main/phases/phase-29ao/README.md`
  - 状況: P0–P36 ✅ 完了 / Next: P37（TBD）
  - Next 指示書: TBD

- **Phase 29af（✅ COMPLETE）: Boundary hygiene / regression entrypoint / carrier layout SSOT**
  - 入口: `docs/development/current/main/phases/phase-29af/README.md`

- **Phase 29ag（✅ COMPLETE）: JoinIR merge SSOT unification**
  - 入口: `docs/development/current/main/phases/phase-29ag/README.md`

- **Phase 29ah（✅ COMPLETE）: JoinIR regression pack expansion（real-world coverage）**
  - 入口: `docs/development/current/main/phases/phase-29ah/README.md`

- **Phase 29aj（candidate）: PlannerOutcome observability SSOT**
  - 入口: `docs/development/current/main/phases/phase-29aj/README.md`
  - 状況: P0/P1/P2/P3/P4/P5/P6/P7/P8/P9/P10 ✅ 完了
  - Next: Phase 29aj P11（TBD）
  - 運用: integration filter で phase143_* は回さない（JoinIR 回帰は phase29ae pack のみ）
  - 運用: phase286_pattern9_* は legacy pack (SKIP) を使う

- **Phase 29ak（candidate）: PlanRuleOrder SSOT + PlannerContext plumbing**
  - 入口: `docs/development/current/main/phases/phase-29ak/README.md`
  - 状況: P0/P1/P2/P3/P4/P5 ✅ 完了
  - Next: None（phase-29al へ）

- **Phase 29al（candidate）: CorePlan composition hardening (docs-first)**
  - 入口: `docs/development/current/main/phases/phase-29al/README.md`
  - 状況: P0/P1/P2/P3 ✅ 完了（docs-only）
  - Next: P4（unwind を含む ExitKind 拡張: design only）
  - 道筋 SSOT: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

- **Phase 29am（✅ COMPLETE）: CorePlan Step-A implementation (lowerer/verifier)**
  - 入口: `docs/development/current/main/phases/phase-29am/README.md`
  - 状況: P0/P1/P2/P3 ✅ 完了
  - Next: Phase 29an
  - Gate: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

- **Phase 29ai（candidate）: Plan/Frag single-planner（Facts SSOT）**
  - 入口: `docs/development/current/main/phases/phase-29ai/README.md`
  - Next: Phase 29ai P16（TBD: promotion hint を JoinIR 側の orchestrator へ配線、挙動不変）

- **Phase 29ae P1（✅ COMPLETE）: JoinIR Regression Pack (SSOT固定)**
  - 入口: `docs/development/current/main/phases/phase-29ae/README.md`

- **Phase 29ad（✅ COMPLETE）: Naming SSOT for Pattern6/7 fixtures**
  - 入口: `docs/development/current/main/phases/phase-29ad/README.md`

- **Phase 29ac（✅ COMPLETE）: De-freeze Pattern6/7**
  - 入口: `docs/development/current/main/phases/phase-29ac/README.md`

- **Phase 29ab（✅ COMPLETE）: JoinIR completion triage**
  - 入口: `docs/development/current/main/phases/phase-29ab/README.md`

- **Phase 288（✅ P0–P3 + 288.1 complete）: REPL mode**
  - 入口: `docs/development/current/main/phases/phase-288/README.md`
  - SSOT: `docs/reference/language/repl.md`
  - 次: Phase 288.2+（任意）: REPL UX improvements（history / multiline / load 等）

- **Phase 287（✅ COMPLETE）: Developer Hygiene（意味論不変）**
  - 状況: P0–P9 ✅ 完了（big files / facade / SSOT / docs guard / closeout）
  - 成果: merge/mod.rs (1,555→1,053行), ast_feature_extractor (1,148→135行), plan.rs (741→120行)
  - 入口: `docs/development/current/main/phases/phase-287/README.md`

- **Phase 284（✅ COMPLETE）: Return as ExitKind SSOT（patternに散らさない）**
  - 目的: `return` を “pattern最適化の個別実装” にせず、`ExitKind` と `compose::*` / `emit_frag()` に収束させる
  - ねらい: 移行期間中の「二重ルール」「検出の穴」を減らし、将来の pattern 増殖を防ぐ
  - 入口: `docs/development/current/main/phases/phase-284/README.md`
  - P0（docs-only）: `docs/development/current/main/phases/phase-284/P0-INSTRUCTIONS.md`
  - SSOT:
    - Composition: `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`
    - Terminator emission: `emit_frag()`（`src/mir/builder/control_flow/edgecfg/api/emit.rs`）
    - Frag docs: `docs/development/current/main/design/edgecfg-fragments.md`
  - 進め方:
    - P0（docs-only）で “return の意味” と “Ok(None)/Err” の境界を固定
    - P1+ で Rust/LLVM の実装を SSOT に収束（pattern側に例外実装を増やさない）

- **Phase 285（✅ COMPLETE）: Box lifecycle / weakref / finalization / GC SSOT**
  - 目的: Box の生存期間（強参照/弱参照/解放/最終化）を SSOT として固定し、「実装が仕様」になっている箇所を潰す
  - ねらい:
    - VM 側の weakref/finalization を仕様化（テストで固定）
    - LLVM harness 側の未対応/差分を “仕様として明文化” し、将来の実装計画を切る
  - 入口: `docs/development/current/main/phases/phase-285/README.md`
  - 状況（完了）:
    - P0/P1/P2 ✅ 完了（weak 成功パターンは smoke 固定）
    - P2.1 ✅ 完了: hidden root を根治し、weak-fail smoke を PASS に復帰
    - P2.2 ✅ 完了: KeepAlive の二重責務を命令分離で解消（`KeepAlive`/`ReleaseStrong`）
    - P3.1/P3.2 ✅ 完了: LLVM 検出 + quick SSOT（`--features llvm` でも quick の意味が変わらない）
    - P4 ✅ 完了: `phase285_weak_basic_llvm` を PASS に復帰（quick 154/154）
  - 参考（現状の入口候補）:
    - weakref 表現: `src/value.rs`（`NyashValue::WeakBox`）
    - finalization: `src/finalization.rs`
  - 追加（syntax cleanup, small & focused）:
    - `weak` の表面構文を `weak <expr>` に収束（`weak(<expr>)` を持ち込まない）
    - `let weak w;` / `let weak w = e` の糖衣を検討（概念を増やさず `let w = weak e` にデシュガー）
    - fixture/smoke は `apps/tests/*.hako` を SSOT にして VM/LLVM で共通化（必要なら LLVM 側は SKIP で理由を固定）

- **Phase 29y（✅ P0 COMPLETE, docs-first SSOT finalized）: MIR lifecycle vocab freeze（RC/weak/ABI）**
  - 状況: P0 ✅ 完了（docs-first SSOT finalized: ABI/RC insertion/Observability）
  - 成果: 3つのSSOT文書（10/20/30）Ready、pilot実装固定、Next steps明文化
  - 入口: `docs/development/current/main/phases/phase-29y/README.md`
  - 次: Phase 29z（RC insertion minimal）または Phase 29x（De-Rust runtime）候補

- **Phase 29z（P2 CLOSEOUT準備中, implementation-minimal）: RC insertion minimal**
  - 進捗: P1まで完了（上書き＋Store null＋Return終端cleanup で ReleaseStrong 挿入、単一block・安全ガード付き）
  - ガード: Cargo feature `rc-insertion-minimal`（既定OFF、env var 新設なし）
  - 検証: quick 154/154 PASS 維持 + `rc_insertion_selfcheck`（opt-in）
  - 残課題（P2で方針化→次フェーズへ）
    - null伝搬の精度向上（copy以外の伝搬パターン追加を段階的に）
    - Branch/Jump 終端での cleanup をどう安全に扱うかの設計
    - PHI/loop/early-exit の安全な cleanup 設計（誤release防止）
  - 入口: `docs/development/current/main/phases/phase-29z/README.md`
  - 指示書: `docs/development/current/main/phases/phase-29z/P0-RC_INSERTION_MINIMAL-INSTRUCTIONS.md`

- **Phase 29aa（✅ COMPLETE）: RC insertion safety expansion（CFG-aware）**
  - 進捗: P2 ✅ 完了（Jump/Branch 終端で cleanup を入れない契約の SSOT 化）
  - 進捗: P3 ✅ 完了（Jump→Return single-predecessor state 伝播）
  - 進捗: P4 ✅ 完了（Jump-chain propagation to Return）
  - 進捗: P5 ✅ 完了（Multi-predecessor Return join: state完全一致のみ）
  - 進捗: P6 ✅ 完了（Multi-predecessor Return join: intersection）
  - 進捗: P7 ✅ 完了（Deterministic ReleaseStrong ordering）
  - 進捗: P8 ✅ 完了（Null propagation across CFG; Copy-only）
  - 入口: `docs/development/current/main/phases/phase-29aa/README.md`

- **Phase 29x（planned, post self-host）: De-Rust runtime for LLVM execution**
  - 目的: LLVM 実行経路のランタイム依存を段階的に Rust から切り離す（脱Rust）。
  - 前提: self-host ラインが安定し、VM/LLVM conformance（Phase 285）が十分に固まっていること。
  - 方針:
    - 仕様SSOT（strong/weak/fini/cleanup/void）は維持し、実装だけを差し替え可能にする。
    - まず ABI 境界（例: `nyrt_*`）を “将来置換する契約” として固定し、独立ランタイムに差し替える。
  - 受け入れ条件（最小）:
    - 既存の `apps/tests/*.hako` fixture を再利用し、VM/LLVM parity のスモークが維持される。
    - weak の語彙（`weak <expr>` / `weak_to_strong()`）が同じ意味で動作する（cycleは当面リーク仕様でも可）。

- **Phase 29y（planned, post self-host / docs-first）: MIR lifecycle vocab freeze（RC/weak/ABI）**
  - 目的: 参照カウント（strong/weak）を “どこで” 実装するか（MIR語彙 vs runtime ABI）を設計SSOT化し、脱Rust実装の土台にする
  - 前提: self-host 後／Phase 285（weak conformance + hidden root 根治）が落ち着いていること
  - 入口（phase）: `docs/development/current/main/phases/phase-29y/README.md`
  - 入口（相談パケット）: `docs/development/current/main/investigations/phase-29y-mir-lifecycle-vocab-consult.md`
  - 進捗（pilot, 仕様不変）:
    - docs SSOT（ABI/RC insertion/observability）を phase-29y 配下に集約
    - Phase 29y.1: “最小導線” の実装だけ先行（意味論は変更しない）
      - NyRT handle ABI shim（`crates/nyash_kernel/src/ffi/lifecycle.rs`）
      - RC insertion pass 入口（no-op）
      - leak report に root categories（handlesのみ）を追加（Phase 1 limitation 明記）

- **Phase 286（✅ complete）: JoinIR Line Absorption（JoinIR→CorePlan/Frag 収束）**
  - 目的: 移行期間に残っている「2本の lowering（Plan line / JoinIR line）」を、構造で 1 本に収束させる
  - ねらい: `return/break/continue` のような “大きな出口語彙” の実装場所が揺れない状態にする
  - 入口: `docs/development/current/main/phases/phase-286/README.md`
  - P0（docs-only）: `docs/development/current/main/phases/phase-286/P0-INSTRUCTIONS.md`
  - 状況: Pattern1–9 の主要ループ形が Plan/Frag SSOT に吸収され、quick gate は green を維持。
  - 将来の設計相談（Phase 286 の範囲外で、別フェーズでSSOT化してから）: `docs/development/current/main/investigations/phase-286-plan-normalization-consult.md`
  - SSOT:
    - Plan/Frag: `compose::*` + `emit_frag()`（Phase 280/281）
    - JoinIR line 共通入口: `src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs`

- （✅ done）**Phase 282**: Router shrinkage + detection SSOT + extractor refactor
  - 完了: `docs/development/current/main/phases/phase-282/README.md`

（✅ done）**Phase 279 P0**: Type propagation pipeline SSOT 統一（lifecycle / JoinIR / LLVM の二重化解消）
  - 完了: `docs/development/current/main/phases/phase-279/README.md`

- **Phase 272（✅ complete）: Pattern6/7 を Frag+emit_frag へ吸収（段階適用）**
  - 目的: scan系 loop の CFG 構築を `Frag/ExitKind` 合成へ寄せ、pattern列挙の増殖を止める
  - 完了: P0.1（Pattern6）✅ + P0.2（Pattern7）✅
  - 入口: fixture/smoke を SSOT として固定（Pattern6→Pattern7 の順で段階適用）
  - 詳細: `phases/phase-272/README.md`

- （✅ done）**Phase 273**: Plan line SSOT（Pattern6/7）
  - 完了: `docs/development/current/main/phases/phase-273/README.md`

- （✅ done）**Phase 280**: Frag composition SSOT positioning
  - 完了: `docs/development/current/main/phases/phase-280/README.md`

- （✅ done）**Phase 281**: compose adoption（Pattern6/7, P0–P3）
  - 完了: `docs/development/current/main/phases/phase-281/README.md`

- （✅ done）**Phase 277/278**: PHI strict + env var 収束
  - 完了: `docs/development/current/main/phases/phase-277/README.md`
  - 完了: `docs/development/current/main/phases/phase-278/README.md`


- **Phase 274（active, design-first）: Type SSOT Alignment（local + dynamic runtime）**
  - 入口SSOT: `docs/reference/language/types.md`
  - P1（✅完了）: Rust VM が `TypeOp(Check/Cast)` を実行可能（`is/as` が動く）
  - P2（✅完了）: LLVM ライン（llvmlite harness）の `TypeOp` を SSOT に合わせる
  - P3（decision）: truthiness / equality / compare / `+` の coercion を SSOT として固定（必要なら “仕様変更フェーズ” を追加）
  - 詳細: `phases/phase-274/README.md`

- **Phase 270（✅ 完了）: JoinIR-only minimal loop SSOT**
  - `apps/tests/phase270_p0_loop_min_const.hako` + VM smoke で “最小 const loop” を固定（exit=3）
  - Pattern1 は test-only stub のため不適合 → Pattern9（AccumConstLoop）を橋渡しとして追加
  - 詳細: `phases/phase-270/README.md`

- **Phase 271（✅ 完了, docs-only）: Bridge pattern 撤去条件SSOT**
  - 対象: bridge pattern（例: `Pattern9_AccumConstLoop`）
  - 目的: 「汎用化しない」「Frag 合成へ吸収して削除する」を SSOT 化
  - 成果物:
    - `docs/development/current/main/design/edgecfg-fragments.md` の `Bridge patterns（撤去条件SSOT）` に “bridge contract” テンプレを追加
    - `Pattern9_AccumConstLoop` の撤去条件（fixture/smoke/手順）を同セクションに明文化
  - SSOT: `docs/development/current/main/design/edgecfg-fragments.md`

- **Phase 269 P1（✅ 完了）: Pattern8 を EdgeCFG で実装（SSA を閉じる）**
  - 方針: emission 入口で Frag 構築（break/continue 無しなので `compose::loop_()` は使わず手配線）
  - 完了: header に `i` の PHI を追加して SSA を閉じた（`i_current = phi [i_init, preheader], [i_next, step]`）
  - early-exit の `return false` は Return wire、`return true` は loop 後 AST に任せる
  - Pattern8 の返り値は当面 `void`（loop-statement 扱い）
  - 補足（DONE）: static box の `this/me` は MethodCall 共通入口で static call に正規化済み（Pattern8 は現状 static box 文脈を対象外）
  - 詳細: `phases/phase-269/README.md`

- **Phase 270+（planned）: Pattern6/7 への Frag 適用**
  - Pattern6: Match scan（index_of 系）
  - Pattern7: Split scan（split 系）
  - 統一的なループ処理パターン確立
  - Pattern 分岐削減

- **収束方針（SSOT案）: Expr/Condition/Control の 3 箱分割**
  - ExprLowererBox（式SSOT）: `AST(expr)` → `(prelude, value)`（ANF含む）。pure/impure/whitelist/strict を集約（入口SSOT）。
  - ConditionLowererBox（条件→分岐SSOT）: `AST(cond)` → `BranchPlan`。評価順は ExprLowererBox に委譲し、`&&/||` は制御語彙で扱う。
  - ControlLowererBox（制御SSOT）: `StepNode/ControlTree` → JoinIR（継続 + env）。`if/loop` を担当し、条件は ConditionLowererBox に委譲。

- **Phase 141 P2+（planned）: Call/MethodCall（effects + typing を分離して段階投入）**
  - ねらい: pure/impure 境界を壊さずに、impure lowering を段階投入する。
  - 前提（DONE）:
    - Phase 141 P1.5: known intrinsic allowlist + available_inputs 3-source merge + diagnostics
  - 受け入れ条件:
    - out-of-scope は `Ok(None)` でフォールバック（既定挙動不変）
    - effects の順序付けは SSOT で固定してから解禁（by-name 増殖禁止）

- **Phase 144-anf（planned）: impure 式導入の順序固定（ANF）**
  - ねらい: `x + f(y)` 等の “pure + impure 混在” で評価順が仕様になる前に、ANF で順序固定を SSOT 化する
  - 入口: `docs/development/current/main/phases/phase-144-anf/INSTRUCTIONS.md`
  - 受け入れ条件:
    - impure を lowering できない場合は `Ok(None)` でフォールバック（既定挙動不変）
    - dev/strict では「順序固定の欠落」を Fail-Fast（診断に順序ログを含める）

- **Phase 143-loopvocab R0（planned）: Contract SSOT 抽出（refactor P0 → modular components）**
  - 目的: loop_true_if_break_continue.rs を「検出/契約/変換」に分割し、P1/P2 での if分岐増殖を防ぐ
  - 実装:
    - 新ファイル: `src/mir/control_tree/normalized_shadow/common/loop_if_exit_contract.rs`
      - `enum LoopIfExitThen { Break, Continue }`
      - `struct LoopIfExitShape { has_else: bool, then: LoopIfExitThen, else_: Option<LoopIfExitThen>, cond_scope: ExprLoweringScope }`
      - `enum OutOfScopeReason { NotLoopTrue, BodyNotSingleIf, ThenNotExit, ElseNotSupported, CondOutOfScope(...) }`
    - Refactor: loop_true_if_break_continue.rs は「shape抽出 → lower」だけに縮退（SSOT は contract側）
    - Tests: unit test を dedicated module へ分離（test maintainability）
  - 受け入れ条件:
    - cargo check ✅（no errors）
    - P1/P2 での if分岐を防ぐ（contract で決定性を保証）
    - out-of-scope は `Ok(None)` で一貫（既定挙動不変）

- **Phase 143-loopvocab P1（planned）: continue 語彙追加**
  - 対象: `loop(true) { if(cond_pure) continue }` を same lowering に通す
  - 実装:
    - LoopIfExitShape で `LoopIfExitThen::Continue` を許可
    - JoinModule: if true → loop_step (continue semantics)
    - Fixtures: `phase143_loop_true_if_continue_min.hako`
    - Smoke: VM + LLVM EXE
  - Out-of-scope は `Ok(None)` のまま

（DONE）Phase 143-loopvocab P2: else 対称化（B-C / C-B）
  - 記録: `docs/development/current/main/10-Now.md`

- **Phase 143-loopvocab P3+（planned）: impure conditions 対応**
  - 目的: `if(cond_impure) break/continue` を ANF/順序固定の上で段階投入する
  - 方針: Phase 145-anf の契約（hoist + left-to-right）を条件式にも適用

- **Phase 263+（planned）: Pattern2 LoopBodyLocal promotion（seg）**
  - 目的: Stage‑B compile（bundle_resolver系）で露出している Pattern2 `LoopBodyLocal(seg)` を受理し、quick の first FAIL を進める
  - 受け入れ条件:
    - 最小再現 fixture + smoke で固定（先に失敗を SSOT 化）
    - Pattern2 が不成立のときは “部分続行” せず `Ok(None)` で fallback（既定挙動不変）

- **（DONE）Phase 263 P0.2: Pattern2 PromoteDecision API hardening**
  - 入口SSOT: `src/mir/builder/control_flow/joinir/patterns/pattern2/api/`
  - `PromoteDecision::{Promoted, NotApplicable, Freeze}` と `try_promote(...)` に参照点を収束（Option揺れを撤去）

- **Phase 264（✅ 入口作成完了）: EdgeCFG Fragment 入口作成（design-first）**
  - **ステータス**: ✅ 入口作成完了（適用は次フェーズ）
  - **実装内容**:
    - `edgecfg/api/` フォルダに SSOT 入口作成
    - `ExitKind`, `EdgeStub`, `Frag` の型定義
    - `seq`, `if_`, `loop_`, `cleanup` のシグネチャ固定（pub(crate)）
    - 最小ユニットテスト 3個
    - ドキュメント連動（edgecfg-fragments.md）
  - **制約遵守**:
    - 既存 pattern6/7/8 未改変
    - merge/EdgeCFG 未改変
    - cargo test -p nyash-rust --lib --no-run 成功確認
  - **次フェーズへの橋渡し**:
    - Phase 265 で Pattern8 適用時に `compose::loop_` を実装
    - 再利用確認後、pattern番号分岐を段階的に削減

- **Phase 265 P0（✅ 完了）: compose/verify 最小実装**
  - **目的**: 入口SSOTの形を固める（迷子防止）
  - **実装**:
    - compose::loop_() 最小実装（exit集合分類のみ、配線なし）
    - verify_frag_invariants() 最小実装（デバッグガード付き）
    - compose::loop_() ユニットテスト 2個追加
  - **制約**:
    - Pattern8 未改変（P0では触らない、偽Frag回避）
    - 配線ロジックは P1 以降
  - **次**: Phase 265 P1 で配線ロジック + Pattern8適用

- **Phase 265 P1（✅ 完了）: compose 配線ロジック実装**
  - **目的**: Frag/ExitKind の配線能力を BasicBlockId 層で証明
  - **実装**:
    - EdgeStub.target 追加（Option<BasicBlockId>）
    - compose::loop_() 配線ロジック（Continue → header, Break → after）
    - verify_frag_invariants() 配線契約検証
    - test-only PoC（5個のテスト: 既存2個更新 + 新規3個追加）
  - **配線契約**:
    - Continue(loop_id) の EdgeStub.target = Some(header)
    - Break(loop_id) の EdgeStub.target = Some(after)
    - Normal/Return/Unwind の EdgeStub.target = None（上位へ伝搬）
  - **制約**:
    - MIR 命令生成なし（Frag 層のみ）
    - NormalizedShadow 未適用（Phase 267 に繰り越し）

- **（✅ 完了）Phase 265 P2: seq/if_ 実装（wires/exits 分離）**
  - **目的**: 「解決済み配線（wires）」と「未解決 exit（exits）」を分離し、Frag 合成の基本パターンを完成
  - **完了内容**:
    - Frag に `wires: Vec<EdgeStub>` 追加
    - wires/exits 分離設計確立（exits = target None, wires = target Some）
    - loop_() を wires 対応に更新
    - seq(a, b) 実装（a.Normal → wires）
    - if_(header, cond, t, e, join_frag) 実装（t/e.Normal → wires）
    - verify 強化（wires/exits 分離契約、警告のみ）
    - 全テスト PASS（13個: frag 3 + compose 9 + verify 1）
  - **設計判断**:
    - wires/exits 分離で再配線バグ防止
    - if_ は join_frag: Frag で「join 以降」を明確化
    - verify は警告のみ（Err 化は Phase 266 の strict 版で段階導入）
  - **次**: Phase 266 で MIR 命令生成 PoC（emit_wires）、Phase 267 で NormalizedShadow/JoinIR へ実適用

- **（✅ 完了）Phase 266: wires → MIR terminator 生成（最小 PoC）**
  - **目的**: wires を MIR terminator に変換する最小 PoC を実装し、Phase 267 での本格適用に備える
  - **完了内容**:
    - emit.rs 作成（emit_wires 実装 + unit test 4個）
    - verify_frag_invariants_strict() 追加（段階導入を壊さない）
    - Jump/Return 対応（Branch は Phase 267）
    - mod.rs 更新（emit module エクスポート）
    - 全テスト PASS（1392 passed: 既存 1388 + 新規 4個）
  - **核心原則**:
    - from ごとにグループ化して1本だけ許可（1 block = 1 terminator 制約）
    - Return は target=None を許可（意味を持たない）
    - verify_frag_invariants() は変更なし（警告のまま、strict 版を別名で用意）
    - Phase 260 terminator 語彙ルールを厳守
  - **設計判断**:
    - from グループ化で terminator 上書きバグ防止
    - Return は target 不要（呼び出し元に戻る）
    - verify strict 版で段階導入を壊さない
  - **次**: Phase 267 で JoinIR Pattern への適用

- **（✅ P0 完了）Phase 267: BranchStub + emit_frag（Branch の第一級化）**
  - **目的**: Frag に Branch を第一級で追加し、wires（Jump/Return）と同様に MIR terminator へ落とす入口を作る
  - **完了内容（P0）**:
    - `BranchStub` 追加 + `Frag.branches` 追加
    - `compose::if_` が header→then/else の BranchStub を生成
    - `emit_frag(function, frag)` を追加（`emit_wires` + `set_branch_with_edge_args`）
    - 1 block = 1 terminator（wire/branch の衝突）を Fail-Fast
    - unit tests + `cargo test -p nyash-rust --lib` PASS
  - **P1（延期）**:
    - "層を跨がない実適用"は候補が抽象化層へ委譲済みのため、Phase 268 で体系的に適用する方針
  - **詳細**: `docs/development/current/main/phases/phase-267/README.md`

- **（✅ 完了）Phase 268: if_form.rs への Frag 適用 + entry edge-args SSOT化**
  - **目的**: EdgeCFG Fragment を層を跨がずに実戦投入し、compose::if_() の edge-args を SSOT 化
  - **完了内容（P0）**:
    - emission/branch.rs に emit_conditional_edgecfg() 追加（薄いラッパー）
    - if_form.rs を Frag+emit_frag 経由に変更（emit_conditional + emit_jump を削除）
    - emission 層経由で層が綺麗に保たれる
  - **完了内容（P1）**:
    - compose::if_() シグネチャ変更（then_entry_args, else_entry_args 追加）
    - emission/branch.rs から空 EdgeArgs を渡す
    - EdgeCFG テスト更新（compose.rs 2箇所、emit.rs 1箇所）
    - TODO コメント削除完了（Phase 267 P2+ TODO 解消）
  - **テスト結果**:
    - cargo build --release: 成功
    - cargo test --lib --release: 1444/1444 PASS
    - quick smoke: 45/46 PASS（既存状態維持）
  - **核心原則**:
    - emission 層経由で Frag 構築を MirBuilder 層から分離
    - SSOT 原則: compose::if_() は edge-args を内部生成しない
  - **次**: Phase 269 で Pattern6/7/8 への Frag 適用 + fixture/smoke test
  - **詳細**: `docs/development/current/main/phases/phase-268/README.md`

- **real-app loop regression の横展開（VM + LLVM EXE）**
  - ねらい: 実コード由来ループを 1 本ずつ最小抽出して fixture/smoke で固定する（段階投入）。
  - 現状: Phase 107（find_balanced_array/object / json_cur 由来）まで固定済み。
  - 次候補: JsonLoader/JsonCur から 1 本ずつ（fixture + integration smoke）で増やす。

- **P5b “完全E2E”**（escape skip の実ループを end-to-end で固定）
  - 現状: Phase 94 で VM E2E まで固定済み。次は selfhost 実コード（`apps/selfhost-vm/json_loader.hako`）へ横展開して回帰を減らす。
  - 入口: `docs/development/current/main/phases/phase-94/README.md`

- **制御の再帰合成（docs-only → dev-only段階投入）**
  - ねらい: `loop/if` ネストの "構造" を SSOT（ControlTree/StepTree）で表せるようにする
  - 注意: canonicalizer は観測/構造SSOTまで（ValueId/PHI配線は Normalized 側へ）
  - 現状: Phase 119–128（if-only Normalized: reads/inputs/unknown-read/partial-assign keep/merge）まで完了
  - ✅ 完了: Phase 129-C（post-if を post_k continuation で表現）
  - 入口: `docs/development/current/main/design/control-tree.md`

## 中期（ループ在庫の残り）

- **P5（guard-bounded）**: 大型ループを “小粒度” に割ってから取り込む（分割 or 新契約）
- **P6（nested loops）**: capability guard で Fail-Fast 維持しつつ、解禁時の契約を先に固定

## 中期（制御の表現力）

北極星: `docs/development/current/main/design/join-explicit-cfg-construction.md`  
設計メモ: `docs/development/current/main/design/exception-cleanup-async.md`

- **catch/cleanup（Invoke）**
  - 追加語彙を `Invoke(ok_edge, err_edge)` に絞って例外 edge を明示する（例外値は edge-args で運ぶ）。
  - 実装タイミング: Phase 260（edge-args terminator 収束）の P1〜P2 以降が推奨。
- **cleanup/defer（cleanup normalizer）**
  - Return/Throw/Break/Continue を cleanup に寄せる “脱出 edge 正規化” を箱化する（finally の後継としての cleanup）。
  - 実装タイミング: catch/cleanup の次（例外 edge も含めて正規化するため）。
- **async/await（state machine lowering）**
  - CFG語彙に混ぜず、AsyncLowerBox で state machine 化してから MIR に落とす。
  - 実装タイミング: finally/defer の後（cancel/drop と cleanup の接続を先に固める）。

## ドキュメント運用

- 重複が出たら「設計 SSOT（design）」に集約し、Phaseログ（phases）は “何をやったか/検証したか” に限定する
- 調査ログ（investigations）は結論を SSOT に反映してから Historical 化する
