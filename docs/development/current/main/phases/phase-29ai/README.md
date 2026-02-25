# Phase 29ai: Plan/Frag single-planner (Facts SSOT)

Goal: pattern 名による分岐を外部APIから消し、Facts（事実）→ Plan → Frag の導線を 1 本に収束させる（仕様不変）。

## P0: LoopFacts SSOT + Single Planner skeleton

- 指示書: `docs/development/current/main/phases/phase-29ai/P0-LOOPFACTS-SSOT-SINGLE-PLANNER-INSTRUCTIONS.md`
- 追加される骨格（未接続・仕様不変）: `src/mir/builder/control_flow/plan/{facts,normalize,planner,emit}/`

## P1: Planner candidate-set + Freeze SSOT

- 指示書: `docs/development/current/main/phases/phase-29ai/P1-PLANNER-CANDIDATES-FREEZE-SSOT-INSTRUCTIONS.md`
- ねらい: `Ok(None)` / `Err(Freeze)` の境界と “候補集合→一意化” をSSOT化（未接続のまま、仕様不変）

## P2: SSOT Registry + Freeze Taxonomy（docs-only）

- 指示書: `docs/development/current/main/phases/phase-29ai/P2-SSOT-REGISTRY-FREEZE-TAXONOMY-INSTRUCTIONS.md`
- ねらい: “真実の所在” と Freeze 分類を1枚に固定して、後続実装の迷子を防ぐ（仕様不変）
- SSOT Registry: `docs/development/current/main/design/planfrag-ssot-registry.md`
- Freeze taxonomy: `docs/development/current/main/design/planfrag-freeze-taxonomy.md`

## P3: Typed Freeze + CandidateSet implementation（code）

- 指示書: `docs/development/current/main/phases/phase-29ai/P3-TYPED-FREEZE-CANDIDATESET-IMPLEMENTATION-INSTRUCTIONS.md`
- ねらい: Planner の契約を型/候補集合で固定（Facts 未実装の間は未到達、仕様不変）

## P4: First LoopFacts（Pattern6 scan-with-init）

- 指示書: `docs/development/current/main/phases/phase-29ai/P4-FIRST-LOOPFACTS-PATTERN6-SCAN_WITH_INIT-INSTRUCTIONS.md`
- ねらい: Facts→Planner を 1 ケースだけ前進（未接続のまま、仕様不変）

## P5: Single-planner bridge（router → 1 entrypoint）

- 指示書: `docs/development/current/main/phases/phase-29ai/P5-SINGLE-PLANNER-BRIDGE-ROUTER-INSTRUCTIONS.md`
- ねらい: JoinIR の pattern ルーティングを外部APIから剥がし、入口を 1 本に収束（仕様不変）

## P6: Move Pattern6/7 extractors to Plan layer

- 指示書: `docs/development/current/main/phases/phase-29ai/P6-MOVE-PATTERN6-7-EXTRACTORS-TO-PLAN-LAYER-INSTRUCTIONS.md`
- ねらい: 抽出（pattern固有知識）のSSOTを plan 側へ寄せ、依存方向を一方向に固定（仕様不変）

## P7: Planner returns DomainPlan（二重Planの解消）

- 指示書: `docs/development/current/main/phases/phase-29ai/P7-PLANNER-RETURNS-DOMAINPLAN-INSTRUCTIONS.md`
- ねらい: 29ai planner の候補集合/Freeze を `DomainPlan` 上で行い、Plan語彙を1本化（仕様不変）

## P8: Wire planner into single_planner（Pattern6 subset）

- 指示書: `docs/development/current/main/phases/phase-29ai/P8-WIRE-PLANNER-INTO-SINGLE_PLANNER-PATTERN6-INSTRUCTIONS.md`
- ねらい: Facts→Planner を実行経路へ1歩だけ接続し、Pattern6最小ケースから吸収を開始（仕様不変）

## P9: Planner support + wiring（Pattern7 split-scan subset）

- 指示書: `docs/development/current/main/phases/phase-29ai/P9-PLANNER-PATTERN7-SPLITSCAN-WIRE-INSTRUCTIONS.md`
- ねらい: Pattern7（split-scan）の最小ケースを Facts→Planner で `Ok(Some(DomainPlan::SplitScan))` まで到達させ、single_planner で planner-first を開始（仕様不変を維持しつつ段階吸収）
- 完了: Facts split-scan subset / planner candidate / single_planner Pattern7 planner-first を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P10: Move Pattern2 extractor to plan layer（SSOT）

- 指示書: `docs/development/current/main/phases/phase-29ai/P10-MOVE-PATTERN2-EXTRACTOR-TO-PLAN-LAYER-INSTRUCTIONS.md`
- ねらい: Pattern2 の抽出（pattern固有知識）を plan 側へ寄せて依存方向を一方向に固定（仕様不変）
- 完了: plan/extractors へ移設、JoinIR 側は wrapper 化、legacy_rules を plan 側へ統一
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P11: Planner support + wiring（Pattern2 break subset）

- 指示書: `docs/development/current/main/phases/phase-29ai/P11-PLANNER-PATTERN2-BREAK-SUBSET-WIRE-INSTRUCTIONS.md`
- ねらい: Pattern2（break）の PoC subset を Facts→Planner に吸収し、single_planner で planner-first を開始（仕様不変で段階吸収）
- 完了: Pattern2BreakFacts/Planner 候補/Pattern2 planner-first を接続、subset fixture + smoke 追加
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile integration --filter "phase29ai_pattern2_break_plan_subset_ok_min"`

## P12: Facts SSOT（Pattern2 LoopBodyLocal promotion）

- 指示書: `docs/development/current/main/phases/phase-29ai/P12-FACTS-PATTERN2-LOOPBODYLOCAL-PROMOTION-INSTRUCTIONS.md`
- ねらい: Pattern2 の “LoopBodyLocal promotion” を Facts として仕様化し、planner/emitter が二重に解析しない形へ寄せる（既定挙動は不変）
- 完了: Pattern2LoopBodyLocal facts 抽出 + LoopFacts 接続（guard解除） + unit tests 追加
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P13: Memoize Facts in single_planner（SSOT）

- 指示書: `docs/development/current/main/phases/phase-29ai/P13-PLANNER-MEMOIZE-FACTS-IN_SINGLE_PLANNER-INSTRUCTIONS.md`
- ねらい: planner 呼び出しを 1 回に収束し、Pattern6/7/2 の planner-first が二重に Facts を走らせないようにする（仕様不変）
- 完了: single_planner 内で planner 結果を memoize し、Pattern6/7/2 は型一致時のみ採用
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P14: Planner support（Pattern2 LoopBodyLocal promotion subset）

- 指示書: `docs/development/current/main/phases/phase-29ai/P14-PLANNER-PATTERN2-LOOPBODYLOCAL-PROMOTION-SUBSET-INSTRUCTIONS.md`
- ねらい: LoopBodyLocal promotion の “要求” を Pattern2BreakPlan に付加し、挙動不変のまま Plan にメタ情報を載せる
- 完了: promotion hint を plan vocab に追加し、planner が facts から hint を付与（legacy は None）
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P15: Observe Pattern2 promotion hint (strict/dev)

- 指示書: `docs/development/current/main/phases/phase-29ai/P15-OBSERVE-PATTERN2-PROMOTION_HINT-INSTRUCTIONS.md`
- ねらい: strict/dev のときだけ promotion hint を安定タグで観測できるようにする（挙動不変）
- 完了: planner outcome から LoopBodyLocal facts を参照して `[plan/pattern2/promotion_hint:{TrimSeg|DigitPos}]` を出力し、2 本をタグ検証に昇格
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
