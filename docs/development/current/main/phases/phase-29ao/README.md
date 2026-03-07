---
Status: Closeout
Scope: CorePlan composition（Feature合成→Normalizerへ、仕様不変で段階導入）
Related:
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/design/coreloop-composer-v0-v1-boundary-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao: CorePlan composition from Skeleton/Feature (Step-C/D)

Goal: numbered route label の “complete pattern” に寄り過ぎない形で、**Skeleton + Feature** から `CorePlan` を合成していく（仕様不変で段階導入）。

Gate（SSOT）:
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

Closeout SSOT:
- Done criteria: `docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md`
- v0/v1 boundary: `docs/development/current/main/design/coreloop-composer-v0-v1-boundary-ssot.md`

Reading note:
- 下の instruction filename に残る `PATTERN*` は historical instruction token だよ。
- smoke script は semantic wrapper を current entry とし、old `phase29ao_*` / `phase29ab_*` / `phase263_pattern2_*` stem は compat wrapper / legacy fixture pin token として読む。

## P0: Composer scaffold（未接続・仕様不変）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P0-COREPLAN-COMPOSER-SCAFFOLD-INSTRUCTIONS.md`
- ねらい: `CanonicalLoopFacts`（projection済み）→ `CorePlan` 合成の入口を 1 箇所に作り、以後の実装を “合成だけ” に寄せる

## P1: Composer API決定 + bridge（未接続・仕様不変）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P1-COREPLAN-COMPOSER-API-BRIDGE-INSTRUCTIONS.md`
- ねらい:
  - `CorePlan` が `BasicBlockId/ValueId/Frag` を要求するため、Facts→合成の段階で **どこが allocation を持つか**を SSOT として固定する
  - まずは “bridge” として、composer が `CanonicalLoopFacts` から `DomainPlan`（既存語彙）を構築して `PlanNormalizer` を呼べる形まで整える（未接続のまま）

## P2: Composer→Normalizer bridge（未接続・仕様不変）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P2-COREPLAN-COMPOSER-VIA-NORMALIZER-INSTRUCTIONS.md`
- ねらい: `CanonicalLoopFacts → DomainPlan → PlanNormalizer → CorePlan` の橋渡しを未接続で固定

## P3: CoreLoop skeleton を CorePlan で直接生成（未接続・仕様不変）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P3-CORELOOP-SKELETON-COMPOSE-INSTRUCTIONS.md`
- ねらい: `CanonicalLoopFacts` から `CorePlan::Loop`（skeleton）を direct 生成（loop_simple_while subset のみ。legacy Pattern1 label は traceability-only）

## P4: ExitMap presence を Frag.exits に投影（未接続・仕様不変）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P4-EXITMAP-PRESENCE-WIRE-INSTRUCTIONS.md`
- ねらい: `exit_kinds_present` の presence を `Frag.exits` に投影（未配線のまま語彙だけ固定）

## P5: Cleanup presence を ExitKind 語彙へ投影（未接続・仕様不変）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P5-CLEANUP-PRESENCE-WIRE-INSTRUCTIONS.md`
- ねらい: `cleanup_kinds_present` を ExitKind 語彙として `Frag.exits` に投影（未配線のまま語彙だけ固定）

## P6: ValueJoin presence の安全ゲート（未接続・仕様不変）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P6-VALUEJOIN-PRESENCE-GATE-INSTRUCTIONS.md`
- ねらい: `value_join_needed` が立つケースは direct skeleton を採用しない（fallback維持）

## P7: ValueJoin wire（EdgeArgs layout の語彙固定 + 局所 verify）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P7-VALUEJOIN-EDGEARGS-LAYOUT-VERIFY-INSTRUCTIONS.md`
- ねらい: `ExprResultPlusCarriers` の語彙と最小検証を PlanVerifier に追加（未接続）

## P8: compose が EdgeArgs を保持することの検証（仕様不変）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P8-VALUEJOIN-EDGEARGS-COMPOSE-PRESERVE-INSTRUCTIONS.md`
- ねらい: compose::seq/if_/cleanup が EdgeArgs(layout+values) を保持することをテストで固定

## P9: ValueJoin minimal wire（BlockParams 足場 + strict/dev Fail-Fast）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P9-VALUEJOIN-MINIMAL-WIRE-INSTRUCTIONS.md`
- ねらい: EdgeCFG の block params 足場と strict/dev verify を追加し、join 受け口の整合を Fail-Fast で固定

## P10: ValueJoin minimal wiring（block_params → MIR PHI）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P10-VALUEJOIN-BLOCKPARAMS-PHI-INSERTION-INSTRUCTIONS.md`
- ねらい: `Frag.block_params` を `emit_frag()` で PHI に落とす唯一の接続点を追加（未接続のまま）

## P11: Normalizer generates block_params (If2 demo) ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P11-VALUEJOIN-NORMALIZER-GENERATES-BLOCKPARAMS-INSTRUCTIONS.md`
- ねらい: Normalizer が `Frag.block_params` を生成する最小ケースを追加し、PHI挿入まで unit test で固定

## P12: ValueJoin の最初の実使用（split_scan route の step join を block_params 化）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P12-VALUEJOIN-FIRST-REAL-USAGE-PATTERN7-SPLITSCAN-INSTRUCTIONS.md`
- ねらい: step join の 2 PHI を `Frag.block_params + EdgeArgs` で表現し、emit_frag() の挿入経路を 1 件固定

## P13: ValueJoin expr_result の実使用（if_phi_join route の merge join を block_params 化）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P13-VALUEJOIN-REAL-USAGE-PATTERN3-IFPHI-MERGE-INSTRUCTIONS.md`
- ねらい: merge join の 1 PHI を `Frag.block_params + EdgeArgs` で表現し、expr_result 的な join 値の経路を 1 件固定

## P14: ValueJoin exit の実使用（loop_break route の after join を block_params 化）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P14-VALUEJOIN-REAL-USAGE-PATTERN2-BREAK-EXITJOIN-INSTRUCTIONS.md`
- ねらい: after join の 1 PHI を `Frag.block_params + EdgeArgs` で表現し、exit join の経路を 1 件固定

## P15: JoinIR 回帰パックに if_phi_join（VM）を追加 ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P15-REGRESSION-PACK-INCLUDE-PATTERN3-INSTRUCTIONS.md`
- ねらい: P13 の実経路（if_phi_join）が回帰ゲート（phase29ae pack）で必ず実行されるようにする

## P16: ValueJoin exit の実使用（loop_true_early_exit route の after join を block_params 化）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P16-VALUEJOIN-REAL-USAGE-PATTERN5-EXITJOIN-INSTRUCTIONS.md`
- ねらい: after join の 1 PHI を `Frag.block_params + EdgeArgs` で表現し、exit join の経路を 1 件固定

## P17: loop_simple_while を Facts→CorePlan へ寄せる（strict/dev のみ shadow adopt）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P17-COMPOSER-PATTERN1-STRICT-SHADOW-INSTRUCTIONS.md`
- ねらい: strict/dev のみ Facts→CorePlan(skeleton) を採用し、既定経路は維持

## P18: single_planner が planner outcome（facts+plan）を返す（P17の二重planner呼び出し撤去）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P18-SINGLE_PLANNER-OUTCOME-PLUMBING-INSTRUCTIONS.md`
- ねらい: planner outcome を single_planner から受け取り、router の二重実行を撤去

## P19: 回帰ゲートに loop_simple_while strict/dev shadow adopt を含める ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P19-REGRESSION-PACK-ADD-PATTERN1-STRICT-SHADOW-INSTRUCTIONS.md`
- 変更:
  - `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_strict_shadow_vm.sh` を current wrapper として追加（compat wrapper: `phase29ao_pattern1_strict_shadow_vm.sh`）
  - `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に組み込み
  - `docs/development/current/main/phases/phase-29ae/README.md` の回帰セットに追記
- ねらい: P17/P18 の strict/dev shadow adopt が回帰ゲートで必ず踏まれる状態を SSOT 化

## P20: CoreLoop ExitMap composition（docs-first）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P20-CORELOOP-EXITMAP-COMPOSITION-SSOT-INSTRUCTIONS.md`
- ねらい: Loop skeleton に対する ExitMap/Cleanup/ValueJoin の合成規約を SSOT として固定

## P21: loop_simple_while subset body is step-only（shadow adopt safety）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P21-PATTERN1-SUBSET-BODY-IS-STEP-ONLY-INSTRUCTIONS.md`
- ねらい: loop_simple_while subset を body=step のみに引き締め、strict/dev shadow adopt の誤マッチを遮断

## P22: Dedup loop_simple_while CoreLoop construction（SSOT統一）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P22-DEDUP-PATTERN1-CORELOOP-CONSTRUCTION-INSTRUCTIONS.md`
- ねらい: DomainPlan/Facts 経路の CoreLoop 構築を 1 箇所へ統一し divergence を防ぐ

## P23: strict/dev if_phi_join adopt from facts ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P23-STRICT-ADOPT-PATTERN3-IFPHI-FROM-FACTS-INSTRUCTIONS.md`
- ねらい: if_phi_join を strict/dev で Facts→CorePlan に寄せ、DomainPlan とのズレを早期検知

## P24: strict/dev split_scan adopt from facts ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P24-STRICT-ADOPT-PATTERN7-SPLITSCAN-FROM-FACTS-INSTRUCTIONS.md`
- ねらい: split_scan を strict/dev で Facts→CorePlan に寄せ、fallback/近似マッチによるズレを早期検知（既定挙動は不変）

## P25: strict/dev loop_true_early_exit adopt from facts ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P25-STRICT-ADOPT-PATTERN5-INFINITE-EARLY-EXIT-FROM-FACTS-INSTRUCTIONS.md`
- ねらい: loop_true_early_exit を strict/dev で Facts→CorePlan に寄せ、DomainPlan 経路との差分を早期検知（既定挙動は不変）

## P26: strict/dev loop_break subset adopt from facts ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P26-STRICT-ADOPT-PATTERN2-BREAK-SUBSET-FROM-FACTS-INSTRUCTIONS.md`
- ねらい: loop_break のうち Facts が表現できる subset を strict/dev で Facts→CorePlan に寄せ、段階的に CorePlan 合成へ収束（既定挙動は不変）

## P27: strict/dev scan_with_init subset adopt from facts ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P27-STRICT-ADOPT-PATTERN6-SCANWITHINIT-SUBSET-FROM-FACTS-INSTRUCTIONS.md`
- ねらい: scan_with_init のうち planner subset（Facts由来）だけを strict/dev で Facts→CorePlan に寄せ、reverse/matchscan 等の variant は fallback 維持（既定挙動は不変）

## P28: Shadow adopt observability（strict/dev tags + gate smokes）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P28-SHADOW-ADOPT-OBSERVABILITY-TAGS-AND-GATE-SMOKES-INSTRUCTIONS.md`
- ねらい: strict/dev の shadow adopt が “実際に踏まれている” ことを安定タグと回帰スモークで SSOT 化（仕様不変）

## P29: Shadow adopt tag coverage（all gate patterns）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P29-SHADOW-ADOPT-TAGS-COVERAGE-ALL-GATE-PATTERNS-INSTRUCTIONS.md`
- ねらい: regression gate に含まれる全パターンで “shadow adopt を踏んだ” をタグ必須として固定（仕様不変）

## P30: Shadow adopt composer SSOT（Facts→CorePlan入口を集約）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P30-MOVE-SHADOW-ADOPT-COMPOSER-SSOT-INSTRUCTIONS.md`
- ねらい: Facts→CorePlan の入口を `plan/composer` に集約し、Normalizer の責務を DomainPlan→CorePlan に縮退（挙動不変）

## P31: shadow adopt routing SSOT（router を薄くする）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P31-REFactor-SHADOW-ADOPT-ROUTER-TO-COMPOSER-SSOT-INSTRUCTIONS.md`

## P32: loop_break real-world strict/dev shadow adopt（phase263 をタグ必須で固定）✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P32-STRICT-ADOPT-PATTERN2-REALWORLD-FROM-FACTS-INSTRUCTIONS.md`
- ねらい: `phase263_pattern2_*` legacy fixture pin token 群が strict/dev で Facts→CorePlan shadow adopt を踏むことを “タグ必須” で固定し、CorePlan 完全移行の回帰穴を塞ぐ（仕様不変）

## P33: loop_break body-local planner-derive + tag gate ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P33-PLANNER-DERIVE-PATTERN2-LOOPBODYLOCAL-SMOKES-INSTRUCTIONS.md`
- ねらい: `phase29ab_pattern2_loopbodylocal_{min,seg_min}` legacy fixture pin token 群を planner 由来 loop_break plan に引き上げ、shadow adopt タグを strict/dev 回帰で必須化（仕様不変）

## P34: loop_break negative shadow adopt tag gates ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P34-PATTERN2-NEGATIVE-SHADOW-ADOPT-TAG-GATES-INSTRUCTIONS.md`
- ねらい: `phase29ab_pattern2_seg_{freeze,notapplicable}` legacy fixture token 群で shadow adopt タグが出ないことを回帰で固定（仕様不変）

## P35: Shadow-adopt tag coverage SSOT + loop_simple_while negative gate ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P35-SHADOW-ADOPT-TAG-COVERAGE-SSOT-AND-PATTERN1-NEGATIVE-GATE-INSTRUCTIONS.md`
- ねらい: タグ必須/禁止を SSOT 化し、loop_simple_while subset reject の negative gate を回帰で固定（仕様不変）

## P36: Stage-2 pilot — release adopt loop_simple_while CorePlan skeleton (subset) ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P36-RELEASE-ADOPT-PATTERN1-COREPLAN-SKELETON-PILOT-INSTRUCTIONS.md`
- ねらい: loop_simple_while subset を release 既定でも Facts→CorePlan(skeleton) で採用する Stage-2 パイロット（仕様不変）

## P37: Stage-2 expand — release adopt scan_with_init subset ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P37-RELEASE-ADOPT-PATTERN6-SCANWITHINIT-SUBSET-INSTRUCTIONS.md`
- ねらい: scan_with_init planner subset を release 既定で Facts→CorePlan に採用し、非strict経路の回帰を追加（仕様不変）

## P38: Stage-2 expand — release adopt split_scan subset ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P38-RELEASE-ADOPT-PATTERN7-SPLITSCAN-SUBSET-INSTRUCTIONS.md`
- ねらい: split_scan planner subset を release 既定で Facts→CorePlan に採用し、非strict経路の回帰を追加（仕様不変）

## P39: Stage-2 expand — release adopt loop_break subset ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P39-RELEASE-ADOPT-PATTERN2-BREAK-SUBSET-INSTRUCTIONS.md`
- ねらい: loop_break planner subset を release 既定で Facts→CorePlan に採用し、非strict経路の回帰を追加（仕様不変）

## P40: Stage-2 expand — release adopt if_phi_join subset ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P40-RELEASE-ADOPT-PATTERN3-IFPHI-SUBSET-INSTRUCTIONS.md`
- ねらい: if_phi_join planner subset を release 既定で Facts→CorePlan に採用し、非strict経路の回帰を追加（仕様不変）

## P41: Stage-2 expand — release adopt loop_true_early_exit subset ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P41-RELEASE-ADOPT-PATTERN5-INFINITE-EARLY-EXIT-SUBSET-INSTRUCTIONS.md`
- ねらい: loop_true_early_exit planner subset を release 既定で Facts→CorePlan に採用し、非strict経路の回帰を追加（仕様不変）

## P42: Stage-3 design — CoreLoopComposer v0 (Skeleton+Feature) ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P42-STAGE3-CORELOOPCOMPOSER-V0-DESIGN-INSTRUCTIONS.md`
- ねらい: Skeleton+Feature 合成による CoreLoopComposer v0 の境界を SSOT で固定（docs-first, 仕様不変）

## P43: CoreLoopComposer v0 scaffold (unconnected) ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P43-CORELOOPCOMPOSER-V0-SCAFFOLD-INSTRUCTIONS.md`
- ねらい: CoreLoopComposer v0 の足場を追加し、合成入口の SSOT を先に固定（未接続, 仕様不変）

## P44: CoreLoopComposer v0 — loop_simple_while minimal composition ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P44-CORELOOPCOMPOSER-V0-PATTERN1-MINIMAL-COMPOSITION-INSTRUCTIONS.md`
- ねらい: loop_simple_while skeleton の最小合成を v0 で開始し、Facts→CorePlan の責務を composer 側へ寄せる（仕様不変）

## P45: CoreLoopComposer v0 — scan_with_init minimal composition ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P45-CORELOOPCOMPOSER-V0-PATTERN6-SCANWITHINIT-INSTRUCTIONS.md`
- ねらい: scan_with_init planner subset の最小合成を v0 で開始し、composer に合成のSSOTを寄せる（仕様不変）

## P46: CoreLoopComposer v0 — split_scan minimal composition ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P46-CORELOOPCOMPOSER-V0-PATTERN7-SPLITSCAN-INSTRUCTIONS.md`
- ねらい: split_scan planner subset の最小合成を v0 で開始し、composer に合成のSSOTを寄せる（仕様不変）

## P47: CoreLoopComposer v1 — split_scan value-join minimal composition ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P47-CORELOOPCOMPOSER-V1-SPLITSCAN-VALUEJOIN-INSTRUCTIONS.md`
- ねらい: split_scan value-join を v1 で受理し、block_params/EdgeArgs 経由の PHI 一本化を維持（仕様不変）

## P48: CoreLoopComposer v1 — loop_break value-join minimal composition ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P48-CORELOOPCOMPOSER-V1-PATTERN2-BREAK-VALUEJOIN-INSTRUCTIONS.md`
- ねらい: loop_break after-join を v1 で受理し、block_params/EdgeArgs 経由の PHI 一本化を維持（仕様不変）

## P49: CoreLoopComposer v1 — loop_true_early_exit value-join minimal composition ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P49-CORELOOPCOMPOSER-V1-PATTERN5-INFINITE-EARLY-EXIT-VALUEJOIN-INSTRUCTIONS.md`
- ねらい: loop_true_early_exit after-join を v1 で受理し、block_params/EdgeArgs 経由の PHI 一本化を維持（仕様不変）

## P50: CoreLoopComposer v1 — if_phi_join value-join minimal composition ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P50-CORELOOPCOMPOSER-V1-PATTERN3-IFPHI-VALUEJOIN-INSTRUCTIONS.md`
- ねらい: if_phi_join join を v1 で受理し、block_params/EdgeArgs 経由の PHI 一本化を維持（仕様不変）

## P51: CoreLoopComposer v1 — split_scan value-join path unification ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P51-CORELOOPCOMPOSER-V1-PATTERN7-VALUEJOIN-UNIFY-INSTRUCTIONS.md`
- ねらい: split_scan の value-join を v1 に統一し、v0 は no-join 専用に固定（仕様不変）

## P52: CoreLoopComposer v0/v1 — SplitScan v0 reject + adopt branching ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P52-SPLITSCAN-V0-REJECT-AND-ADOPT-GATE-INSTRUCTIONS.md`
- ねらい: SplitScan の v0/v1 分離を両側の reject で固定し、adopt 分岐を value_join_needed だけに単純化（仕様不変）

## P53: CoreLoopComposer v0/v1 — ScanWithInit v0/v1 boundary ✅

- 指示書: `docs/development/current/main/phases/phase-29ao/P53-SCANWITHINIT-V0V1-BOUNDARY-INSTRUCTIONS.md`
- ねらい: ScanWithInit を v0/v1 分離で固定し、value_join_needed の境界をSSOT化（仕様不変）

## P54: CoreLoopComposer v0/v1 closeout — adopt entrypoint SSOT ✅

- SSOT: `docs/development/current/main/design/coreloop-composer-v0-v1-boundary-ssot.md`
- ねらい: scan_with_init / split_scan の v0/v1 分岐を入口で一本化し、境界責務をSSOT化（仕様不変）

## P55: Stage-3 closeout — Done criteria SSOT + legacy roadmap ✅

- SSOT: `docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md`
- ねらい: CorePlan移行のDone判定と legacy削減の順序を docs-first で固定

## Next（planned）

- Next: Phase 29ap (planned)
  - P56: legacy extractor の削減（段階的、回帰ゲート必須）
  - P57: router の pattern 名分岐を削減（planner outcome + composer SSOT）
  - P58: Facts/Feature の拡張（必要に応じて）
