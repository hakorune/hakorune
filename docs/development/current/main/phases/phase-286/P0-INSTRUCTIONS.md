# Phase 286 P0（docs-only）: JoinIR Line Absorption SSOT

目的: 「JoinIR line が第2の lowerer として残っている」状態を、設計で終わらせる（収束点の SSOT を固定する）。

## このP0でやること（コード変更なし）

1) 収束点（1本化ポイント）を SSOT として決める  
2) 禁止事項（散布・二重SSOT）を文章で固定する  
3) Phase 284（Return）/ Phase 285（Box lifecycle）と整合する責務マップを作る  

## README に必ず入れる事項（チェックリスト）

- [ ] 「Plan line / JoinIR line」の現状と、なぜ二重化が危険か（迷子の原因）  
- [ ] 収束後の SSOT フロー（`Extractor → PlanFreeze → Lowerer → Frag/emit_frag`）  
- [ ] JoinIR の将来位置づけ（DomainPlan生成補助 / もしくは撤去までの段階）  
- [ ] 禁止事項（pattern側へ return/break/continue を散布しない、Ok(None)黙殺禁止）  
- [ ] 次フェーズ（P1 investigation / P2 PoC）の観測点と最小成功条件  

## SSOTリンク

- Router（SSOT=extract / safety valve）: `docs/development/current/main/phases/phase-282/README.md`
- Frag/compose/emit SSOT: `docs/development/current/main/design/edgecfg-fragments.md`
- JoinIR line 共通入口: `src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs`
- Plan line SSOT: `docs/development/current/main/phases/phase-273/README.md`

