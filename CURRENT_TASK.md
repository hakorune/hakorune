# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-14
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane / next lane に最短で戻る
- landed history は phase docs を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/15-Workstream-Map.md`
4. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
5. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
6. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
7. `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
7. `git status -sb`
8. `tools/checks/dev_gate.sh quick`

## Restart Handoff

- expected worktree:
  - clean
- active lane:
  - `phase-29bq selfhost mirbuilder failure-driven`
- sibling guardrail:
  - `phase-137x` string corridor / exact-keeper guardrail
- immediate next:
  - `compiler expressivity first`
- immediate follow-on:
  - `phase-29bq loop owner seam cleanup`
- current blocker:
  - `none`
- current stop-lines:
  - do not mix lane B with lane C (`Debug` / terminator-adjacent operand/control liveness cleanup)
  - do not mix lane B with `generic placement / effect`
  - do not mix parked `phase-96x` backlog into the active optimization lane
- parked corridor:
  - `phase-96x vm_hako LLVM acceptance cutover`
  - only remaining backlog is monitor-policy decision for the frozen `vm-hako-core` pack

## Design Owners

- implementation lane:
  - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
- next layer landing:
  - `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- roadmap SSOT:
  - `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
- string guardrail owner:
  - `docs/development/current/main/phases/phase-137x/README.md`
- generic memory lane-B contract owner:
  - `docs/development/current/main/design/generic-memory-dce-observer-owner-contract-ssot.md`
- observer/control lane-C contract owner:
  - `docs/development/current/main/design/observer-control-dce-owner-contract-ssot.md`
- concurrency manual owner:
  - `docs/reference/concurrency/semantics.md`
- concurrency runtime-plan owner:
  - `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

## Current Notes

- latest landed phase:
  - `phase-277x`: optimization lane closeout judgment froze the landed optimization roadmap and handed the mainline back to compiler expressivity / selfhost entry
- active focus:
  - `phase-29bq`: failure-driven selfhost mirbuilder lane under compiler-expressivity-first policy
- architecture direction:
  - loop/selfhost cleanup now targets `facts -> route -> recipe -> cfg skeleton -> join sig -> phi materializer -> verifier -> cleanup`
  - keep `facts` descriptive-only and `recipe` normative
  - move PHI/dominance repair out of semantic lowering over time
  - do not absorb all of `plan/` into `recipe`
  - instead, shrink `plan/` into a temporary lowering namespace and later rename by owner
- control-flow end-state directory proposal:
  - `src/mir/builder/control_flow/facts/`
  - `src/mir/builder/control_flow/recipes/`
  - `src/mir/builder/control_flow/verify/`
  - `src/mir/builder/control_flow/lower/`
  - `src/mir/builder/control_flow/ssa/`
  - `src/mir/builder/control_flow/cleanup/`
  - migration rule:
    - keep `src/mir/builder/control_flow/plan/` while owner split is in flight
    - remove the `plan/` name only after route families no longer mix recipe/lower/ssa/cleanup responsibilities
- pointer rule:
  - `CURRENT_TASK.md` is the only live status pointer
  - `05/10/15` stay thin mirrors only
  - landed detail lives in phase docs, not here

## Execution Queue

1. `optimization lane closeout judgment`
   - landed and closed
2. `phase-29bq selfhost mirbuilder failure-driven`
   - broad gate is green; keep exact blocker capture mode as the default operating rule
3. `phase-29bq loop owner seam cleanup`
   - docs-first owner split:
     - `facts`
     - `route`
     - `recipe`
     - `join sig`
     - `phi materializer`
     - `verifier`
     - `cleanup`
   - landed first family seam:
     - `LoopCondReturnInBody` join-sig extraction
   - current inventory:
      - `LoopCondReturnInBody` now has separate owners through `cleanup`
      - `LoopTrueBreakContinue` now has separate owners through `cleanup`
      - `LoopCondContinueOnly` now has separate owners through `verifier`
      - `LoopCondBreakContinue` now has separate owners through `cleanup`
      - `LoopCondContinueWithReturn` now has separate owners through route-local body helpers
      - next family is `nested_loop_depth1`
      - landed first GenericLoopV1 seam:
        - route-local terminality / continue-edge detection now lives under `generic_loop_body/`
      - landed GenericLoopV1 carrier seam:
        - carrier prepare/body/finalize orchestration now lives under `generic_loop_body/`
      - landed GenericLoopV1 handoff seam:
        - condition/step handoff now lives under `generic_loop_handoff`
      - landed GenericLoopV1 cleanup seam:
        - body-local fallthrough continue suppression now lives under `generic_loop_body/cleanup`
      - `GenericLoopV1` is closeout-ready as a route family
      - landed families so far:
        - `LoopCondReturnInBody`
        - `LoopTrueBreakContinue`
        - `LoopCondContinueOnly`
        - `LoopCondBreakContinue`
        - `LoopCondContinueWithReturn`
        - `GenericLoopV1`
      - next family inventory:
        - `nested_loop_depth1`
          - `facts`
          - `route`
          - acceptance / fallback boundary
          - generic-loop handoff influence
      - landed first `nested_loop_depth1` seam:
        - route-local acceptance / fallback dispatch now lives under `nested_loop_depth1_route`
      - landed second `nested_loop_depth1` seam:
        - preheader freshness rewrite now lives under `nested_loop_depth1_preheader`
      - landed third `nested_loop_depth1` seam:
        - stmt-only fastpath ownership now lives under `parts/loop_/nested_depth1`
      - `nested_loop_depth1` is closeout-ready as a route family
      - next family inventory:
        - `nested_loop_plan`
          - shared recipe-first fallback bridge
          - downstream `loop_cond_continue_with_return` / `loop_cond_break_continue` handoff
      - landed first `nested_loop_plan` seam:
        - recipe-first fallback bridge now lives under `nested_loop_plan_bridge`
      - landed second `nested_loop_plan` seam:
        - `loop_cond_continue_with_return` bridge now lives under `nested_loop_plan_continue_with_return`
      - landed third `nested_loop_plan` seam:
        - `loop_cond_break_continue` bridge now lives under `nested_loop_plan_break_continue`
      - landed fourth `nested_loop_plan` seam:
        - shared recipe fallback orchestration now lives under `nested_loop_plan_recipe_fallback`
      - landed fifth `nested_loop_plan` seam:
        - recipe fallback selection policy now lives under `nested_loop_plan_recipe_fallback_policy`
      - `nested_loop_plan` is closed out as a route family
      - next family inventory:
        - `generic_loop_body::nested_loop_plan`
          - depth1 fastpath handoff
          - recipe-fallback route selection
          - generic/nested minimal fallback order
      - prior family closeout:
        - `GenericLoopV1`
          - `facts`
          - `route`
          - `recipe`
          - `cfg skeleton`
          - body lowering
          - body terminality / continue-edge
          - carrier orchestration
          - condition/step handoff
          - `cleanup`
        - `LoopCondContinueWithReturn`
          - `facts`
          - `route`
          - `recipe`
          - `cfg skeleton`
          - `phi materializer`
          - `verifier`
          - `cleanup`
        - `nested_loop_depth1`
          - `facts`
          - `route-local acceptance / fallback dispatch`
          - `preheader freshness rewrite`
          - `stmt-only fastpath ownership`
        - `nested_loop_plan`
          - shared recipe-first fallback bridge
          - `loop_cond_continue_with_return` bridge
          - `loop_cond_break_continue` bridge
          - shared recipe fallback orchestration
          - recipe fallback selection policy
4. `phase-29bq legacy lowerer removal`
   - landed and closed
5. `phase-29bq loop owner seam cleanup`
   - next:
     - inventory the first exact seam under `generic_loop_body::nested_loop_plan`

## Legacy Compatibility Block

- compiler lane: `phase-29bq / none`（active: monitor-only）
  - done: `JIR-PORT-08`
  - next: `none`
