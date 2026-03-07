# Phase 286: JoinIR Line Absorption（JoinIR→CorePlan/Frag 収束）

Status: ✅ COMPLETE (P0, P1, P2, P2.1, P2.2, P2.3, P2.4.1, P2.6, P2.7, P2.8, P3.1, P3.2, P3 COMPLETE)

## Goal

移行期間に残っている「2本の lowering」を、構造で 1 本に収束させる。

- Plan line（Pattern6/7）: `CorePlan → Frag(compose) → emit_frag()` が SSOT
- JoinIR line（Pattern1–5,9）: `JoinIR → bridge → merge` が SSOT

Phase 286 では JoinIR line を “第2の lowerer” として放置せず、**Plan/Frag SSOT へ吸収**する道筋を固定する。

Reading note:
- 下に出てくる `src/mir/builder/control_flow/joinir/patterns/*` は、特記がない限り execution-time の historical physical path token だよ。
- current live surfaces は `src/mir/builder/control_flow/joinir/route_entry/`, `src/mir/builder/control_flow/plan/`, `src/mir/builder/control_flow/plan/extractors/`, `src/mir/loop_route_detection/` を優先して読むよ。

## Why（なぜ今）

- `return` のような「大きな出口語彙」は、責務が分散すると実装場所が揺れて事故りやすい
- 移行期間の弱点は「同じASTでも経路により意味論が割れる可能性がある」こと
- pattern を溶かしていく思想の最後の壁が “JoinIR line の残存” になりやすい

## SSOT（Phase 286 で守る憲法）

- **SSOT=extract**（Phase 282）: 検出は extract の成功でのみ決める。`pattern_kind` は O(1) safety valve のみ。
- **CFG/terminator SSOT**（Phase 280/281）: `Frag + compose::* + emit_frag()` が唯一の terminator 生成点。
- **Fail-Fast**: close-but-unsupported を `Ok(None)` で黙殺しない（silent reroute 禁止）。

## Responsibility Map（どこを触るか）

- JoinIR line の共通入口（現状）:
  - `src/mir/builder/control_flow/plan/conversion_pipeline.rs`
    - historical path token: `src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs`
  - `src/mir/join_ir_vm_bridge/bridge.rs`
  - `src/mir/builder/control_flow/joinir/merge/mod.rs`
- Plan/Frag SSOT（収束先）:
  - `src/mir/builder/control_flow/plan/*`
  - `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`
  - `src/mir/builder/control_flow/edgecfg/api/emit.rs`

## Scope（提案）

## Next（短期の道筋）

- **将来設計の相談（別フェーズでSSOT化してから）**: `docs/development/current/main/investigations/phase-286-plan-normalization-consult.md`
- **次のおすすめ（design-first）**: Phase 284（Return as ExitKind SSOT）

## Coverage Map（Plan line 移行状況）

| Pattern | Shape | Status | Notes |
|---:|---|---|---|
| 1 | SimpleWhile | ✅ Plan line | PoC + integration fixture固定済み |
| 2 | Break | ✅ Plan line | PoCサブセットは Plan 完走、PoC外は `Ok(None)` で legacy fallback |
| 3 | If-Phi | ✅ Plan line | Fail-Fast 統一（extract `Some` 後の legacy fallback なし） |
| 4 | Continue | ✅ Plan line | PoC + integration fixture固定済み |
| 5 | InfiniteEarlyExit | ✅ Plan line | `loop(true)` literal に限定した PoC サブセット（Return/Break） |
| 6 | ScanWithInit | ✅ Plan line | Phase 273 で SSOT |
| 7 | SplitScan | ✅ Plan line | Phase 273 で SSOT |
| 8 | BoolPredicateScan | ✅ Plan line | static box は設計上スキップ（ReceiverNormalizeBox が担当） |
| 9 | AccumConstLoop | ✅ Plan line | Pattern1 より優先（より具体的） |

### P0（docs-only）✅ COMPLETE (2025-12-25)

**完了内容**:
- **SSOT ドキュメント作成**: `docs/development/current/main/design/joinir-plan-frag-ssot.md` を作成
- **8章構成で固定**:
  1. Scope / Non-goals - 対象範囲の明確化
  2. 用語（Terms） - JoinIR line, Plan, Frag, Boundary, ExitKind, Freeze point, SSOT の定義
  3. 責務（Responsibilities） - Planが決めること・決めないこと / Fragが保持すること・保持しないこと
  4. 禁止事項（Prohibitions） - Planでの実行・名前解決・最適化・ルール実装の禁止 等
  5. 凍結点（Freeze Points） - PlanFreeze / BoundaryFreeze / ValueIdAllocate / MergeComplete
  6. 不変条件（Invariants / Fail-Fast） - Plan段階(V1-V7) / Boundary段階(B1-B2, C1-C2) / Merge段階(M1-M4)
  7. 2本コンパイラ根治の合流点 - 共通パス・分岐点・差分許容場所/非許容場所
  8. デバッグ導線 - NYASH_CLI_VERBOSE, HAKO_JOINIR_DEBUG, NYASH_TRACE_VARMAP 等

**重要な設計決定**:
- JoinIR line を AST → MIR 全体ではなく、「(Pattern detection/Plan) → (Frag+Boundary) → (MIR merge) の限定されたパイプライン」として定義
- 禁止事項の「例外なし」表現を削除し、「診断専用の扱い（debugタグ付き・既定OFF）」という運用ルールに変更
- ValueId 100-999 固定範囲を「host ValueId と衝突しない領域」という原則に変更（具体数値は実装詳細として注記）

**成果物**:
- `docs/development/current/main/design/joinir-plan-frag-ssot.md` (新規)
- コード変更なし（docs-only）

### P1 (contract_checks 導入 + 実バグ修正) ✅ COMPLETE (2025-12-25)

**完了内容**:
- **contract_checks.rs に検証関数追加**: `verify_boundary_contract_at_creation()`
  - B1検証: join_inputs が Param 領域にあること
  - C2検証: condition_bindings が Param 領域にあること
- **merge/mod.rs に検証呼び出し追加**: merge開始時にFail-Fast検証
- **実バグ3件修正**: loop_break / loop_continue_only / loop_true_early_exit（historical labels: Pattern2/4/5）で `alloc_local()` を誤って使っていた箇所を `alloc_param()` に修正

**成果物**:
- `src/mir/builder/control_flow/joinir/merge/contract_checks.rs` (変更)
- `src/mir/builder/control_flow/joinir/merge/mod.rs` (変更)
- `src/mir/join_ir/lowering/loop_with_break_minimal.rs` (変更)
- `src/mir/join_ir/lowering/loop_with_continue_minimal.rs` (変更)
- historical path token: `src/mir/builder/control_flow/joinir/patterns/pattern5_infinite_early_exit.rs` (変更)

**発見された問題**:
- 各 pattern の lowering で関数パラメータに `alloc_local()` を使っていた（本来は `alloc_param()`）
- これにより join_inputs に Local ValueId (1000+) が混入し、検証エラーになっていた

**改善の示唆（Post-P1 Polish 実施済み）**:
- API名の曖昧さが誤用を招いていたため、`alloc_join_param()` / `alloc_join_local()` の導入が検討されている
- エラーメッセージの「原因特定」強化として context パラメータの追加が検討されている

**Post-P1 Polish 追加** (2025-12-25):
- **新API追加**: `JoinValueSpace::alloc_join_param()` / `alloc_join_local()` （薄いラッパー）
- **エラーメッセージ改善**: `verify_boundary_contract_at_creation()` に `context: &str` パラメータ追加
- **docs反映**: SSOTドキュメントに脚注形式で数値記載、新API使用の明記

### P2（PoC）✅ COMPLETE (2025-12-26)

**完了内容**:
- **Pattern4 (Loop with Continue) を Plan/Frag SSOT に移行**
  - DomainPlan::Pattern4Continue 追加
  - PlanNormalizer::normalize_pattern4_continue() 実装（phi_bindings によるAST抽出ベース）
  - Router integration（Plan line routing → legacy fallback）

**成果物**:
- `apps/tests/phase286_pattern4_frag_poc.hako` (最小fixture: single continue)
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern4_frag_poc.sh` (integration smoke)
- `src/mir/builder/control_flow/plan/mod.rs` (Pattern4ContinuePlan struct追加)
- `src/mir/builder/control_flow/joinir/patterns/extractors/pattern4.rs` (extract_pattern4_plan追加)
- `src/mir/builder/control_flow/joinir/patterns/router.rs` (Plan routing追加)
- `src/mir/builder/control_flow/plan/normalizer.rs` (normalize_pattern4_continue + phi_bindings)

**重要な設計決定**:
- **phi_bindings**: lower_*_ast関数でPHI dstを優先参照（variable_mapの初期値ではなく）
- **2-step branching + header PHI merge**: NO Select instruction（CoreEffectPlanにない）
- **carrier passthrough**: Add 0 不要、carrier_current をそのままPHI入力に渡す

**検証結果**:
- Integration test: phase286_pattern4_frag_poc.sh PASS (output: 6)
- Regression test: quick smoke 154 PASS, 0 FAILED

**Pattern2 調査結果（別タスク化）**:
- break経路の値再接続が複雑（after_bbにPHI必要）
- 詳細: [pattern2-deferred.md](./pattern2-deferred.md)

### P2.1（PoC）✅ COMPLETE (2025-12-26)

**完了内容**:
- **Pattern1 (SimpleWhile) を Plan/Frag SSOT に移行**
  - DomainPlan::Pattern1SimpleWhile 追加
  - PlanNormalizer::normalize_pattern1_simple_while() 実装（phi_bindings によるPHI dst優先参照）
  - Router integration（Plan line routing → legacy fallback）

**検証結果**:
- Integration test: `phase286_pattern1_frag_poc` PASS (return: 3)
- Regression test: quick smoke 154 PASS, 0 FAILED

### P2.2 (hygiene: extractor重複排除 + router小整理) ✅ COMPLETE (2025-12-26)

**完了内容**:
- **extractor helper化**: `extract_loop_increment_plan` を `common_helpers.rs` に統一
  - Pattern1/Pattern4 が呼ぶだけに変更（重複排除 ~25行）
- **router helper化**: `lower_via_plan()` を追加し Pattern6/7/4/1 で共用
  - 3行パターン（normalize→verify→lower）を1関数に集約（ボイラープレート削減 ~40行）

**成果物**:
- current semantic surfaces:
  - `src/mir/builder/control_flow/plan/extractors/common_helpers.rs`
  - `src/mir/builder/control_flow/joinir/route_entry/router.rs`
- historical path tokens:
  - `src/mir/builder/control_flow/joinir/patterns/extractors/common_helpers.rs`
  - `src/mir/builder/control_flow/joinir/patterns/extractors/pattern1.rs`
  - `src/mir/builder/control_flow/joinir/patterns/extractors/pattern4.rs`
  - `src/mir/builder/control_flow/joinir/patterns/router.rs`

**検証結果**:
- Regression test: quick smoke 154 PASS
- Pattern1 PoC: PASS, Pattern4 PoC: PASS

### P2.3 (Pattern9 AccumConstLoop Plan化 PoC) ✅ COMPLETE (2025-12-26)

**完了内容**:
- **Pattern9 (AccumConstLoop) を Plan/Frag SSOT に移行**
  - DomainPlan::Pattern9AccumConstLoop 追加
  - PlanNormalizer::normalize_pattern9_accum_const_loop() 実装（PHI 2本: loop_var, acc_var）
  - Router integration（Plan line routing → legacy fallback）
  - Pattern9 は Pattern1 より優先（より具体的なパターン）

**設計決定**:
- **PoC は const/var 両方 OK**: `sum = sum + 1`（定数）または `sum = sum + i`（変数）
- **本体の順序固定**: 1行目=累積更新, 2行目=ループ変数更新
- **CFG 構造**: Pattern1 と同じ骨格、PHI 2本（i_current, sum_current）

**成果物**:
- `apps/tests/phase286_pattern9_frag_poc.hako` (最小fixture: const accumulation)
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern9_frag_poc.sh` (integration smoke)
- `src/mir/builder/control_flow/plan/mod.rs` (Pattern9AccumConstLoopPlan + DomainPlan variant)
- `src/mir/builder/control_flow/joinir/patterns/extractors/pattern9.rs` (extract_pattern9_plan() 新規)
- `src/mir/builder/control_flow/joinir/patterns/extractors/mod.rs` (pattern9 モジュール追加)
- `src/mir/builder/control_flow/plan/normalizer.rs` (normalize_pattern9_accum_const_loop())
- `src/mir/builder/control_flow/joinir/patterns/router.rs` (Pattern9 Plan line routing)

**検証結果**:
- Integration test: `phase286_pattern9_frag_poc` PASS (return: 3)
- Regression test: quick smoke 154 PASS, 0 FAILED

### P3 (error context enrichment) ✅ COMPLETE (2025-12-25)

**完了内容**:
- **P2**: host_fn をエラーコンテキストに追加（関数名での特定を容易に）
- **P3**: join-side 情報（continuation数・boundaryサマリ）をエラーコンテキストに追加
  - `[conts=X exits=Y conds=Z]` 形式のサマリを追加
  - 固定キー名で解析容易に

**成果物**:
- `src/mir/builder/control_flow/joinir/merge/mod.rs` (変更)
- 最終エラーフォーマット: `[merge_joinir_mir_blocks host=X join=Y [conts=A exits=B conds=C]]`

### 286C-2 (instruction_rewriter.rs 箱化) ✅ COMPLETE (2025-12-25)

**完了内容**:
- **instruction_rewriter.rs の箱化・意味論不変**: 1400行ファイルに責務リストコメントを追加し、4つの箱モジュールを抽出
  - **InstructionFilterBox**: Skip判定ロジック（純粋関数）
    - `should_skip_copy_overwriting_phi()` - CopyがPHI dstを上書きするか判定
    - `should_skip_function_name_const()` - Const String（関数名）のスキップ判定
    - `should_skip_boundary_input_const()` - Boundary input Constのスキップ判定
  - **ReturnConverterBox**: Return→Jump変換ヘルパー
    - `should_keep_return()` - 非スキップ可能継続のReturn保持判定
    - `remap_return_value()` - Return値のremapヘルパー
  - **TailCallDetectorBox**: テイルコール検出ヘルパー
    - `is_recursive_call()` - 再帰呼び出し判定
    - `is_loop_entry_call()` - ループエントリ呼び出し判定
    - `should_skip_param_binding()` - パラメータ束縛スキップ判定
    - `call_type_description()` - 呼び出しタイプの説明文字列取得
  - **ParameterBindingBox**: パラメータ束縛ヘルパー
    - `should_skip_phi_param()` - PHI dstパラメータのスキップ判定
    - `carrier_param_count()` - キャリアパラメータ数取得
    - `has_more_carrier_args()` - キャリア引数残確認
    - `carrier_arg_index()` - キャリア引数インデックス計算

**成果物**:
- `src/mir/builder/control_flow/joinir/merge/rewriter/instruction_filter_box.rs` (新規)
- `src/mir/builder/control_flow/joinir/merge/rewriter/return_converter_box.rs` (新規)
- `src/mir/builder/control_flow/joinir/merge/rewriter/tail_call_detector_box.rs` (新規)
- `src/mir/builder/control_flow/joinir/merge/rewriter/parameter_binding_box.rs` (新規)
- `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` (変更: 責務リストコメント追加 + 箱使用)
- `src/mir/builder/control_flow/joinir/merge/rewriter/mod.rs` (変更: モジュール追加)

**注意点**:
- 意味論は完全不変（既存のinlineロジックを箱関数呼び出しに置換）
- ファイル行数は1454行に増加（コメント・import追加により）
- 核ロジックは main loop に密結合しているため、完全な分離にはさらなるリファクタリングが必要
- スモークテスト: 既存FAILなし（1件のemit失敗は本変更と無関係）

### P2.4 (Pattern8 BoolPredicateScan Plan化 PoC) ✅ COMPLETE (2025-12-26)

**背景**:
- Pattern8 (BoolPredicateScan) は Phase 269 P1.2 で `static box` コンテキストを明示的にスキップする設計決定あり
- 既存 fixture (`phase269_p0_pattern8_frag_min.hako`) は static box のため Pattern8 がマッチせず Pattern1 にフォールバック
- PoC のためには Pattern8 が実際にマッチする**非 static box の fixture** が必要

**実装方針**:
- **非 static box fixture**: `box StringUtils` に変更し、`Main.main()` から `new StringUtils()` でインスタンス生成
- **Plan line 抽出**: `extract_pattern8_plan()` で parts 抽出（既存 pattern8 の構造を参考）
- **Normalizer**: `normalize_pattern8_bool_predicate_scan()` で Scan 系の骨格を最小で再利用
- **Router integration**: PLAN_EXTRACTORS テーブルに Pattern8 追加、`Ok(None)` なら legacy Pattern8 へフォールバック

**成果物** (予定):
- `apps/tests/phase286_pattern8_plan_poc.hako` (新規: 非 static box fixture)
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern8_plan_poc_vm.sh` (新規: integration smoke)
- `src/mir/builder/control_flow/plan/mod.rs` (変更: Pattern8BoolPredicateScanPlan + DomainPlan variant)
- `src/mir/builder/control_flow/joinir/patterns/extractors/pattern8.rs` (新規: extract_pattern8_plan)
- `src/mir/builder/control_flow/joinir/patterns/extractors/mod.rs` (変更: pattern8 モジュール追加)
- `src/mir/builder/control_flow/plan/normalizer.rs` (変更: normalize_pattern8_bool_predicate_scan)
- `src/mir/builder/control_flow/joinir/patterns/router.rs` (変更: PLAN_EXTRACTORS に Pattern8 追加)

**検証結果**:
- Integration test: `phase286_pattern8_plan_poc_vm` PASS (exit 7)
- Regression test: quick smoke 0 failed
- Debug log: `route=plan strategy=extract pattern=Pattern8_BoolPredicateScan` 確認

**P2.4.1（Plan 完走 / Fail-Fast 統一）** ✅ COMPLETE (2025-12-26)
- Pattern8 の normalizer を実装し、Plan line で完走（legacy fallback 禁止）
- router の “文字列判定 e.contains(...)” などの暫定フォールバックを撤去し、extract 成功後は Fail-Fast に統一

**補足（設計相談）**:
- “Pattern を Plan に落とした後、Plan 側に残る pattern 臭さをどう減らすか” の相談パケット:  
  `docs/development/current/main/investigations/phase-286-plan-normalization-consult.md`

### P2.6 (Pattern3 Plan化 PoC + Pattern1 退行修正) ✅ COMPLETE (2025-12-26)

**背景**:
- **退行バグ発見**: `apps/tests/phase118_pattern3_if_sum_min.hako` が FAIL (期待 12、実際 10)
- **原因**: Pattern1 Plan が Pattern3 fixture を誤ってマッチ
  - Pattern1 extractor の `has_control_flow_statement()` が if/else をチェックしていない
  - pattern_kind ガードもなく、Pattern1 Plan が Pattern3 にマッチしていた

**実装内容**:

**Step 0: Pattern1 退行修正（最優先）** ✅ COMPLETE
- **0.1 Router guard**: `router.rs` の `try_plan_extractors()` で Pattern1 Plan は `ctx.pattern_kind == Pattern1SimpleWhile` のみマッチ
- **0.2 has_if_statement 追加**: `common_helpers.rs` に再帰的 if 検出ヘルパー追加
- **0.3 Pattern1 extractor 強化**: `has_if_statement()` による if-else 拒否を追加（防御in深さ）
- **検証**: phase118 PASS (出力 12)、legacy Pattern3 ルートが正しく動作

**Step 1: Pattern3 Plan line 実装** ✅ COMPLETE
- **DomainPlan 追加**: `Pattern3IfPhiPlan { loop_var, carrier_var, condition, if_condition, then_update, else_update, loop_increment }`
- **Extractor**: `extract_pattern3_plan()` - 既存 `extract_loop_with_if_phi_parts()` を活用
- **Normalizer**: CFG構造 `preheader → header(PHI: i, sum) → body → then/else → merge(PHI: sum) → step → header → after`
- **Router**: Plan 完走のため、Pattern3 の “stub fallback” を撤去し Fail-Fast に統一（extract が Some の後は Err を伝播）

**成果物** (予定):
- `src/mir/builder/control_flow/joinir/patterns/router.rs` (変更: Pattern1 guard ✅)
- `src/mir/builder/control_flow/joinir/patterns/extractors/common_helpers.rs` (変更: has_if_statement ✅)
- `src/mir/builder/control_flow/joinir/patterns/extractors/pattern1.rs` (変更: if 拒否 ✅)
- `src/mir/builder/control_flow/plan/mod.rs` (変更: Pattern3IfPhiPlan + DomainPlan variant)
- `src/mir/builder/control_flow/joinir/patterns/extractors/pattern3.rs` (変更: extract_pattern3_plan)
- `src/mir/builder/control_flow/plan/normalizer.rs` (変更: normalize_pattern3_if_phi)

**成功基準**:
- Regression fix: `phase118_pattern3_if_sum_vm` PASS (出力 12) ✅
- Pattern3 Plan line: `route=plan pattern=Pattern3_IfPhi` 確認
- Full regression: `tools/smokes/v2/run.sh --profile quick` 0 failed

**P2.6.1 指示書**:
- `docs/development/current/main/phases/phase-286/P2.6.1-INSTRUCTIONS.md`

**注意（学び / 契約）**:
- `CoreLoopPlan` の "body ブロック" の effect は `block_effects` ではなく `loop_plan.body`（CorePlan列）に積む必要がある（lowerer が `loop_plan.body` を emit するため）。

### P2.7 (Plan line guardrails - V10不変条件) ✅ COMPLETE (2025-12-26)

**背景**:
- P2.6.1 で発見した「body_bb の effects 配置問題」を契約化し、再発防止

**実装内容**:
- **V10 不変条件追加**: `verifier.rs` に body_bb の block_effects が空であることを検証
  - 違反時のエラー: `[V10] Loop at depth N has non-empty block_effects for body_bb ...`
- **SSOT ドキュメント更新**: `joinir-plan-frag-ssot.md` に V10 を追加
- **テスト追加**: `test_v10_body_bb_effects_in_block_effects_fails`

**成果物**:
- `src/mir/builder/control_flow/plan/verifier.rs` (変更: V10検証追加)
- `docs/development/current/main/design/joinir-plan-frag-ssot.md` (変更: V10記載)

**検証結果**:
- Build: cargo build --release PASS
- Regression: quick smoke 0 failed

### P3.1 (Pattern2 Plan化 - Loop with Break) ✅ COMPLETE (2025-12-26)

**背景**:
- Pattern2 (Loop with Break) は P2 で別タスク化された（break経路の値再接続が複雑）
- 詳細: [pattern2-deferred.md](./pattern2-deferred.md)

**本質的課題: after_bb PHI**:
- break経路では carrier 更新が実行されない場合がある
- after_bb に PHI 必要（header経路 vs break経路の値選択）
```
carrier_out = PHI(header: carrier_current, break_then: carrier_break)
```

**CFG構造**（6ブロック）:
```
preheader → header(PHI: i_current, carrier_current)
              ↓
           body(break_cond check)
              ↓
         ┌────┴────┐
    break_then    step
    (optional      ↓
     update)    header (back-edge)
         ↓
       after_bb(PHI: carrier_out)
         ↑
       header (natural exit when !cond_loop)
```

**実装ステップ**:
1. ✅ Step 0: docs-first - README.md P3.1節追加、pattern2-deferred.md更新
2. ✅ Step 1: Fixture B作成 + smoke script追加
3. ✅ Step 2: DomainPlan::Pattern2Break + extract_pattern2_plan() 追加
4. ✅ Step 3: normalize_pattern2_break() 実装（after_bb PHI）
5. ✅ Step 4: router に Pattern2 追加
6. ✅ Step 5: 検証（Fixture B PASS 出力11、quick 154/154 PASS）

**PoC サブセット厳守**:
以下は必ず `Ok(None)` で legacy へ fallback（Fail-Fast 回帰防止）:
- loop_increment が取れない（構造が複雑）
- break_cond が単一 if でない（ネスト、複数条件）
- break_then が複数文で carrier 更新が特定できない
- carrier が複数（PoC は single carrier のみ）
- body 側の carrier 更新が特定できない

**成果物** (予定):
- `apps/tests/phase286_pattern2_break_no_update_min.hako` (新規: break without update fixture)
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern2_break_no_update_vm.sh` (新規)
- `src/mir/builder/control_flow/plan/mod.rs` (変更: Pattern2BreakPlan + DomainPlan variant)
- `src/mir/builder/control_flow/joinir/patterns/extractors/pattern2.rs` (変更: extract_pattern2_plan)
- `src/mir/builder/control_flow/plan/normalizer.rs` (変更: normalize_pattern2_break)
- `src/mir/builder/control_flow/joinir/patterns/router.rs` (変更: Pattern2 Plan line routing)

**検証結果**:
- ✅ Fixture A (break with update): legacy fallback（PoC サブセット外、出力 42）
- ✅ Fixture B (break without update): Plan line PASS (出力 11)
- ✅ Regression: quick smoke 154 PASS, 0 FAILED

### P3.2 (Pattern5 Plan化 - Infinite Loop with Early Exit) ✅ COMPLETE (2025-12-26)

**完了内容**:
- Pattern5InfiniteEarlyExitPlan 追加（Return版・Break版両対応）
- extract_pattern5_plan() 実装（loop(true) リテラル限定）
- normalize_pattern5_infinite_early_exit() 実装（Return版5blocks、Break版6blocks CFG）
- NormalizationPlanBox で Pattern5 スタイルループを除外（Plan line へルーティング）
- quick smoke 154/154 PASS

**背景**:
- Pattern5 は `loop(true)` の無限ループパターン
- 既存 legacy Pattern5 は `break + continue` 両方必須の複雑な形式
- PoC は **simpler subset**: 早期 return または break 単独

**CFG構造（Return版）**:
```
preheader → header(PHI: i_current) → body(exit_cond)
              ↑                           ↓
              └───── step ←────────  else path
                                          ↓
                               then path: CoreExitPlan::Return
```

**CFG構造（Break版）**:
```
preheader → header(PHI: i, carrier) → body(exit_cond)
              ↑                             ↓
              └───── step ←──────────  else path
                                            ↓
                                 then path → after_bb(PHI: carrier_out)
```

**実装ステップ**:
1. Step 0: docs-first - README P3.2節追加
2. Step 1: integration fixture 2本 (return版、break版)
3. Step 2: DomainPlan::Pattern5InfiniteEarlyExit 追加
4. Step 3: extract_pattern5_plan() 実装
5. Step 4: normalize_pattern5_infinite_early_exit() 実装
6. Step 5: router に Pattern5 追加
7. Step 6: 検証

**PoC サブセット厳守**:
- `loop(true)` リテラルのみ（`loop(1)` や truthy は `Ok(None)`）
- Return版: `if (cond) { return <expr> }` + `i = i + 1`
- Break版: `if (cond) { break }` + `sum = sum + 1` + `i = i + 1`（carrier_update 必須）

**成果物** (予定):
- `apps/tests/phase286_pattern5_return_min.hako` (Fixture A)
- `apps/tests/phase286_pattern5_break_min.hako` (Fixture B)
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern5_return_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern5_break_vm.sh`
- `src/mir/builder/control_flow/plan/mod.rs` (Pattern5InfiniteEarlyExitPlan + Pattern5ExitKind)
- `src/mir/builder/control_flow/joinir/patterns/extractors/pattern5.rs` (extract_pattern5_plan)
- `src/mir/builder/control_flow/plan/normalizer.rs` (normalize_pattern5_infinite_early_exit)
- `src/mir/builder/control_flow/joinir/patterns/router.rs` (Pattern5 Plan line routing)

**成功基準**:
- Fixture A (return): PASS (出力 7)
- Fixture B (break): PASS (出力 3)
- Regression: quick smoke 154 PASS, 0 FAILED

**クリーンアップ完了（P3.2後）**:
- **Legacy Pattern5 削除**: `pattern5_infinite_early_exit.rs` (488行) 完全削除
- **Plan extractor が SSOT**: `extractors/pattern5.rs::extract_pattern5_plan()` が唯一の検出ロジック
- **LOOP_PATTERNS テーブルからエントリ削除**: router.rs の legacy エントリ撤去

**Fail-Fast 方針（extract_pattern5_plan）**:
| 状況 | 返り値 | 動作 |
|------|--------|------|
| PoC サブセット合致 | `Ok(Some(plan))` | Plan line 完走 |
| PoC サブセット外（構造ミスマッチ） | `Ok(None)` | 他のパターンへ回す |
| close-but-unsupported（例: return in legacy Pattern5） | `Err(msg)` | Fail-Fast（silent fallback 禁止） |

**設計決定**:
- PoC サブセット外（複雑な loop(true)）は `Ok(None)` で他へ回す
- 既存 legacy Pattern5 の `break + continue` 必須形式は対象外（extractor でマッチしない）
- 検出できたが未対応の場合は `Err` で明示的に失敗（Fail-Fast 原則維持）

## Acceptance（P0）

- 2本の lowering が "設計として" どこで 1 本に収束するかが明文化されている
- Phase 284（Return）/ Phase 285（GC）と矛盾しない
