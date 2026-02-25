# Phase 29ak: PlanRuleOrder SSOT + PlannerContext plumbing

Goal: single_planner の「順序・名前・ガード」の SSOT を 1 箇所へ寄せ、planner 側へ ctx を通す土台を作る（仕様不変）。

## P0: PlanRuleOrder SSOT + PlannerContext plumbing

- 指示書: `docs/development/current/main/phases/phase-29ak/P0-RULE-ORDER-SSOT-PLANNER-CONTEXT-PLUMBING-INSTRUCTIONS.md`
- ねらい: rule_order.rs を順序/名前 SSOT に固定し、PlannerContext を配線（未使用）
- 完了: PlanRuleOrder を追加し、single_planner の手書きテーブルを撤去
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P1: Pattern1 guard を planner 側へ移動（facts 抽出抑制）

- 指示書: `docs/development/current/main/phases/phase-29ak/P1-PLANNER-PATTERN1-GUARD-INSTRUCTIONS.md`
- ねらい: pattern_kind が Pattern1 以外のとき pattern1 facts 抽出を行わない
- 完了: PlannerContext を参照して loop_facts 入口で Pattern1 を抑制（single_planner 側の guard は維持）
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P2: Pattern8 static box filter を planner 側へ移動

- 指示書: `docs/development/current/main/phases/phase-29ak/P2-PLANNER-PATTERN8-STATIC-BOX-FILTER-INSTRUCTIONS.md`
- ねらい: static box では Pattern8 facts 抽出を抑制（single_planner 側の filter は維持）
- 完了: PlannerContext.in_static_box を参照して loop_facts 入口で Pattern8 を抑制
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P3: single_planner から Pattern8 static box filter を撤去

- 指示書: `docs/development/current/main/phases/phase-29ak/P3-REMOVE-SINGLE_PLANNER-PATTERN8-STATICBOX-FILTER-INSTRUCTIONS.md`
- ねらい: Pattern8 static box filter を planner/facts 側 SSOT に一本化
- 完了: single_planner の Pattern8 特例フィルタを削除（debugログは SSOT ではない）
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P4: single_planner から Pattern1 guard を撤去

- 指示書: `docs/development/current/main/phases/phase-29ak/P4-REMOVE-SINGLE_PLANNER-PATTERN1-GUARD-INSTRUCTIONS.md`
- ねらい: Pattern1 guard を planner/facts 側 SSOT に一本化
- 完了: single_planner の guard を削除し、fallback 側で同契約を維持
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P5: planner 側に ctx gate を集約（candidate 抑制）

- 指示書: `docs/development/current/main/phases/phase-29ak/P5-PLANNER-CANDIDATE-CTX-GATE-SSOT-INSTRUCTIONS.md`
- ねらい: Pattern1/8 の候補抑制を planner の candidate 生成で SSOT 化
- 完了: build_plan_from_facts_ctx で ctx gate を集中管理し、single_planner の Pattern1 fallback 抑制を撤去
- 補足: Pattern1 extractor は nested loop を `Ok(None)` に倒して、phase1883（Pattern6NestedLoopMinimal）が plan 側に吸われないこと
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
