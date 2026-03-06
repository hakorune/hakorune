# Plan/Frag SSOT Registry (SSOT)

Status: Draft (Phase 29ai P2)  
Scope: JoinIR plan/frag 導線（仕様不変）

目的: “真実の所在（SSOT）” を表で固定し、層が勝手に再解析/再推論して SSOT が崩れるのを防ぐ。

## SSOT Table

| Layer | SSOT (Truth) | Forbidden (Must Not) | Verification (Fail-Fast) |
|---|---|---|---|
| Facts | CFG/Terminator/境界情報から抽出した “観測” と “導出” を分離した Facts | planner が CFG を再走査する前提の不足した Facts を作る / emit が CFG を覗いて “穴埋め” | Facts 収集時: 契約違反は `Freeze(contract)`（strict/dev は即Fail） |
| Normalize | Facts の表現ゆれ除去（純変換） | 追加の解析（CFG/AST を見に行く） / 値の意味を変える変形 | normalize 後の不変条件を `verify_*` で検証（strict/dev） |
| Planner | Canonical Facts → Recipe（候補集合→一意化） | pattern 名で入口分岐を公開APIに漏らす / emit の都合で再解析 | 0候補=Ok(None), 1候補=Ok(Some), 2+=Freeze(ambiguous) |
| Plan | Recipe / VerifiedRecipe（current runtime の意味境界） | historical planner-payload wording を current contract に戻す / CFG 再解析が必要な “情報欠落” Plan / 二重Plan語彙 | verifier + emit 前に構造不変条件を検証（strict/dev） |
| Emit | VerifiedRecipe / CorePlan → Frag（生成のみ） | Facts/CFG に戻って再推論 / silent fallback | emit は入力不足を Freeze(bug/contract) で落とす（strict/dev） |
| Frag | 生成結果（EdgeCFG/JoinIR lowering の出力） | Frag が “真実” として再利用されること（派生物） | 既存の frag verifier / contract_checks を入口で実行 |

## Notes

- “Forbidden” は将来の if 地獄 / hidden fallback を防ぐための境界ガード。
- Verification は既存の `contract_checks` と整合する形で増やす（既定挙動は変えない）。

## References

- Entry: `docs/development/current/main/phases/phase-29ai/README.md`
- Plan/Frag overview: `docs/development/current/main/design/edgecfg-fragments.md`
- scan_with_init/split_scan route contracts (legacy labels `Pattern6/7` are traceability-only): `docs/development/current/main/design/pattern6-7-contracts.md`
- CorePlan Skeleton/Feature model: `docs/development/current/main/design/coreplan-skeleton-feature-model.md`
- Shadow-adopt tag coverage SSOT: `docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md`
- CorePlan FlowBox interface SSOT: `docs/development/current/main/design/coreplan-flowbox-interface-ssot.md`
- Return-in-loop minimal SSOT: `docs/development/current/main/design/return-in-loop-minimal-ssot.md`
- Post-PHI final form SSOT: `docs/development/current/main/design/post-phi-final-form-ssot.md`
- Effect classification SSOT: `docs/development/current/main/design/effect-classification-ssot.md`
- ExitKind/Cleanup/Effect contract SSOT: `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md`
- CoreLoop ExitMap composition SSOT: `docs/development/current/main/design/coreloop-exitmap-composition-ssot.md`
