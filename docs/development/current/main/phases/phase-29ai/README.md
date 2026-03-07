# Phase 29ai: Plan/Frag single-planner (Facts SSOT)

Goal: numbered route label による分岐を外部APIから消し、Facts（事実）→ Plan → Frag の導線を 1 本に収束させる（仕様不変）。

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

## P4: First LoopFacts（scan_with_init route; legacy Pattern6 label）

- 指示書: `docs/development/current/main/phases/phase-29ai/P4-FIRST-LOOPFACTS-PATTERN6-SCAN_WITH_INIT-INSTRUCTIONS.md`
- ねらい: Facts→Planner を 1 ケースだけ前進（未接続のまま、仕様不変）

## P5: Single-planner bridge（route router → 1 entrypoint）

- 指示書: `docs/development/current/main/phases/phase-29ai/P5-SINGLE-PLANNER-BRIDGE-ROUTER-INSTRUCTIONS.md`
- ねらい: JoinIR の route ルーティングを外部APIから剥がし、入口を 1 本に収束（仕様不変）

## P6: Move scan_with_init / split_scan extractors to Plan layer

- 指示書: `docs/development/current/main/phases/phase-29ai/P6-MOVE-PATTERN6-7-EXTRACTORS-TO-PLAN-LAYER-INSTRUCTIONS.md`
- ねらい: 抽出（pattern固有知識）のSSOTを plan 側へ寄せ、依存方向を一方向に固定（仕様不変）

## P7: Planner returns DomainPlan（二重Planの解消）

- 指示書: `docs/development/current/main/phases/phase-29ai/P7-PLANNER-RETURNS-DOMAINPLAN-INSTRUCTIONS.md`
- ねらい: 29ai planner の候補集合/Freeze を `DomainPlan` 上で行い、Plan語彙を1本化（仕様不変）

## P8: Wire planner into single_planner（scan_with_init subset）

- 指示書: `docs/development/current/main/phases/phase-29ai/P8-WIRE-PLANNER-INTO-SINGLE_PLANNER-PATTERN6-INSTRUCTIONS.md`
- ねらい: Facts→Planner を実行経路へ1歩だけ接続し、scan_with_init 最小ケースから吸収を開始（仕様不変）

## P9: Planner support + wiring（split_scan subset）

- 指示書: `docs/development/current/main/phases/phase-29ai/P9-PLANNER-PATTERN7-SPLITSCAN-WIRE-INSTRUCTIONS.md`
- ねらい: split_scan の最小ケースを Facts→Planner で `Ok(Some(DomainPlan::SplitScan))` まで到達させ、single_planner で planner-first を開始（仕様不変を維持しつつ段階吸収）
- 完了: Facts split-scan subset / planner candidate / single_planner split_scan planner-first を接続
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P10: Move loop_break extraction to plan layer（historical Pattern2 label）

- 指示書: `docs/development/current/main/phases/phase-29ai/P10-MOVE-PATTERN2-EXTRACTOR-TO-PLAN-LAYER-INSTRUCTIONS.md`
- ねらい: loop_break の抽出（legacy Pattern2 label 由来の知識）を plan 側へ寄せて依存方向を一方向に固定（仕様不変）
- 完了: plan/extractors へ移設、JoinIR 側は wrapper 化、legacy_rules を plan 側へ統一
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P11: Planner support + wiring（loop_break subset; historical Pattern2 label）

- 指示書: `docs/development/current/main/phases/phase-29ai/P11-PLANNER-PATTERN2-BREAK-SUBSET-WIRE-INSTRUCTIONS.md`
- ねらい: loop_break PoC subset を Facts→Planner に吸収し、single_planner で planner-first を開始（仕様不変で段階吸収）
- 完了: LoopBreakFacts / Planner 候補 / loop_break planner-first を接続、subset fixture + smoke 追加
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/smokes/v2/run.sh --profile integration --filter "phase29ai_pattern2_break_plan_subset_ok_min"`

## P12: Facts SSOT（loop_break body-local promotion）

- 指示書: `docs/development/current/main/phases/phase-29ai/P12-FACTS-PATTERN2-LOOPBODYLOCAL-PROMOTION-INSTRUCTIONS.md`
- ねらい: loop_break route の body-local promotion を Facts として仕様化し、planner/emitter が二重に解析しない形へ寄せる（既定挙動は不変）
- 完了: LoopBreakBodyLocal facts 抽出 + LoopFacts 接続（guard解除） + unit tests 追加
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P13: Memoize Facts in single_planner（scan_with_init / split_scan / loop_break）

- 指示書: `docs/development/current/main/phases/phase-29ai/P13-PLANNER-MEMOIZE-FACTS-IN_SINGLE_PLANNER-INSTRUCTIONS.md`
- ねらい: planner 呼び出しを 1 回に収束し、scan_with_init / split_scan / loop_break の planner-first が二重に Facts を走らせないようにする（仕様不変）
- 完了: single_planner 内で planner 結果を memoize し、scan_with_init / split_scan / loop_break は型一致時のみ採用
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P14: Planner support（loop_break body-local promotion subset）

- 指示書: `docs/development/current/main/phases/phase-29ai/P14-PLANNER-PATTERN2-LOOPBODYLOCAL-PROMOTION-SUBSET-INSTRUCTIONS.md`
- ねらい: body-local promotion の “要求” を loop_break plan に付加し、挙動不変のまま Plan にメタ情報を載せる
- 完了: promotion hint を plan vocab に追加し、planner が facts から hint を付与（legacy は None）
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P15: Observe loop_break promotion hint (strict/dev)

- 指示書: `docs/development/current/main/phases/phase-29ai/P15-OBSERVE-PATTERN2-PROMOTION_HINT-INSTRUCTIONS.md`
- ねらい: strict/dev のときだけ promotion hint を安定タグで観測できるようにする（挙動不変）
- 完了: planner outcome から LoopBodyLocal facts を参照して `[plan/loop_break/promotion_hint:{TrimSeg|DigitPos}]` を出力し、2 本をタグ検証に昇格
- 検証: `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
