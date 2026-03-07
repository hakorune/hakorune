# Phase 29ai P9: Planner support + wiring（Pattern7 split-scan subset）

Date: 2025-12-29
Status: Historical reference (implemented)
Scope: Facts→Planner→historical plan payload を split_scan へ拡張し、single_planner で planner-first を開始する
Goal: 「numbered route label で入口分岐」ではなく「Facts→Plan」の 1 本導線に寄せつつ、既定挙動は不変のまま前進する

Historical note:
- `facts/loop_facts.rs`, `planner/build.rs`, `plan/extractors/pattern7_split_scan.rs`,
  `legacy_rules::pattern7::extract` は P9 実行時の path/token だよ。
- current runtime では `plan/facts/{loop_split_scan.rs,loop_builder.rs}`, `planner/mod.rs`,
  `single_planner::try_build_outcome()` が live surface だよ。

## Objective

split_scan の最小ケースについて、Facts が `Ok(Some(...))` を返し、planner が `Ok(Some(DomainPlan::SplitScan))`
まで到達できるようにする。そのうえで `single_planner` の split_scan rule でも planner-first を開始し、legacy へは `Ok(None)`
時のみフォールバックする。

## Non-goals（この P9 ではやらない）

- 仕様変更（挙動変更・ログ増加・エラーメッセージ変更）
- loop_break などの大物の移植（Phase 29ai P10+ へ）
- Freeze を「実行経路で出す」こと（この P9 は `Ok(None)` 保守で進める）
- plan/extractors の削除（legacy の受け口は残す）

## Target Fixture（SSOT）

以下の既存 fixture/smoke を “SplitScan subset” の SSOT として使う:

- Fixture: `apps/tests/phase29ab_pattern7_splitscan_ok_min.hako`
- Smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern7_splitscan_ok_min_vm.sh`
- Regression pack: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Implementation Steps（Critical Order）

### Step 1: Facts の最小対応（SplitScanFacts）

Current semantic files:
- `src/mir/builder/control_flow/plan/facts/loop_split_scan.rs`
- `src/mir/builder/control_flow/plan/facts/loop_builder.rs`
- `src/mir/builder/control_flow/plan/facts/scan_shapes.rs`（必要なら）

やること:
- `LoopFacts` に `split_scan: Option<SplitScanFacts>` を追加。
- `try_build_loop_facts()` は scan_with_init と同様に “保守的に Some を返す” 方針で、SplitScan 最小形だけ `Some` を返す。
- 最小形の抽出は **AST の変数名が確定できる場合のみ**（推測や固定名は禁止）。

最低限抽出したい変数（`SplitScanPlan` に必要）:
- `s_var`, `sep_var`, `result_var`, `i_var`, `start_var`

注意:
- 抽出が不完全なら `Ok(None)`（Freeze は出さない）。
- execution-time `pattern7_split_scan` extractor token と齟齬が出そうなら、まずはその extractor が要求する
  最小セットに合わせる（この P9 では新たな shape 語彙を増やしすぎない）。

### Step 2: Planner の候補生成（DomainPlan::SplitScan）

ファイル:
- `src/mir/builder/control_flow/plan/planner/mod.rs`

やること:
- `facts.facts.split_scan` が `Some` のときに `DomainPlan::SplitScan(SplitScanPlan { ... })` を CandidateSet に push する。
- 0/1/2+ の境界は CandidateSet の finalize に委譲し、planner 側は “候補を積むだけ” に徹する。

注意:
- P9 時点では `SplitScanFacts` は “最小 OK fixture のみ Some” に抑えて、誤マッチで 2+ にならないようにする。

### Step 3: unit test で Facts/Planner の境界を固定

ファイル:
- `src/mir/builder/control_flow/plan/facts/loop_builder.rs`（`#[cfg(test)]`）
- （必要なら）`src/mir/builder/control_flow/plan/planner/mod.rs`（unit test）

やること:
- Facts: “OK minimal っぽい AST” で `Ok(Some(...))` を返すテスト。
- Facts: “明らかに違う AST” で `Ok(None)` を返すテスト。
- Planner: Facts→Planner で `Ok(Some(DomainPlan::SplitScan))` になるテスト（直で `build_plan_from_facts` を呼ぶ形式で OK）。

### Step 4: single_planner の split_scan も planner-first に

ファイル:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

やること:
- scan_with_init と同様に、split_scan rule の先頭で planner を試す。
- `Ok(Some(plan))` なら採用。
- `Ok(None)` なら execution-time compatibility lane にフォールバック。
- planner の `Ok(None)` では新規ログは出さない（観測差分を抑える）。

### Step 5: Docs / Tracking 更新（SSOT）

ファイル:
- `docs/development/current/main/phases/phase-29ai/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`

やること:
- P9 完了の記録（目的/影響/検証コマンド）。
- Next を P10（例: loop_break extractor → Facts/Planner への段階移植）に向けて 1 行だけ書く。

## Verification Checklist（Acceptance）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

期待:
- 0 errors
- quick 154/154 PASS
- regression pack PASS
- 既定挙動は不変（planner が `Ok(None)` を返す領域は legacy 経路へ落ちる）

## Risk Notes

- **誤マッチ**: Facts は “最小形だけ Some” を徹底し、疑わしい場合は `Ok(None)` に倒す。
- **観測差分**: planner `Ok(None)` でログを出さない。採用時も既存の compatibility log を維持する。
- **Freeze**: P9 は Freeze を実行経路に出さない。Freeze taxonomy の投入は P10+ で段階的に行う。
