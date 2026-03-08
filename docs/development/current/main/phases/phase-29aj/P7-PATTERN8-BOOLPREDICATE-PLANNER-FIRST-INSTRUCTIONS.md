# Phase 29aj P7: bool_predicate_scan planner-first via Facts（historical label 8, subset）

Date: 2025-12-29  
Status: Ready for execution  
Scope: bool_predicate_scan facts → planner candidate → single_planner planner-first（仕様不変）
Goal: bool_predicate_scan route を Facts→Planner に乗せ、extractor 依存を 1 本減らす

## Objective

- bool_predicate_scan（historical label 8）を LoopFacts に追加し、planner が該当 route candidate を返せるようにする
- single_planner は bool_predicate_scan の型一致時のみ planner-first 採用
- 既定挙動・観測・エラー文字列は不変

## Non-goals

- bool_predicate_scan サブセット拡張
- ルール順序 SSOT の CandidateSet 移管
- 新 env var / 新ログ追加

## Implementation Steps

### Step 1: Facts SSOT 追加（bool_predicate_scan / historical label 8）

Files:
- `src/mir/builder/control_flow/plan/facts/bool_predicate_scan_facts.rs`

Facts:
- `BoolPredicateScanFacts { loop_var, haystack, predicate_receiver, predicate_method, condition, step_lit }`

Extraction rules (Ok(None) fallback only):
- condition は `i < s.length()`（超保守で length のみ）
- step は `i = i + 1`（StepShape を利用）
- body 内に `if not receiver.method(s.substring(i, i + 1)) { return false }` がある
- hardcode 禁止: receiver / method 名は AST から取得

Unit tests:
- 最小成功ケース
- step 違い / else 付きは Ok(None)

### Step 2: LoopFacts に接続

Files:
- `src/mir/builder/control_flow/plan/facts/mod.rs`
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Changes:
- LoopFacts に `bool_predicate_scan` を追加
- `try_build_loop_facts()` に抽出を追加
- all-none 判定に `bool_predicate_scan` を含める

### Step 3: Planner candidate 追加

File:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Changes:
- facts が Some のとき bool_predicate_scan route candidate を候補に追加
- historical rule token は inventory lane で追跡
- unit test 追加

### Step 4: single_planner を historical label 8 planner-first に

File:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Changes:
- RuleKind に historical label 8 を追加
- planner_opt が bool_predicate_scan route のとき採用
- それ以外は extractor へフォールバック

### Step 5: docs / CURRENT_TASK 更新

Files:
- `docs/development/current/main/phases/phase-29aj/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Acceptance Criteria

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/run.sh --profile integration --filter "bool_predicate_scan"` (任意)

## Commit

- `git add -A && git commit -m "phase29aj(p7): planner-first bool predicate scan subset"`

## Next (P8 candidate)

- TBD
