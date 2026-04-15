# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-15
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
- folderization map owner:
  - `src/mir/builder/control_flow/FOLDERIZATION_MAP.md`
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
- folderization backlog snapshot:
  - owner-local closeout remaining:
    - close out `loop_collect_using_entries_v0::facts`
    - confirm the current owner-local inventory reaches `none confirmed`
  - end-state folderization epics after the owner-local queue is empty:
    - pin destination buckets for current `plan/` directories under `facts / recipes / verify / lower / ssa / cleanup`
    - move shared descriptive infra first (`facts`, `canon`, `extractors`, `route_shape_recognizers`)
    - move recipe/CorePlan infra next (`recipes`, `recipe_tree`, `parts`, `steps`, `features`, `skeletons`)
    - move lowering/orchestration infra (`lowerer`, `emit`, `planner`, `single_planner`, `composer`)
    - move verification/diagnostic infra (`verifier`, `diagnostics`, `observability`)
    - move cleanup/policy infra (`normalizer`, `normalize`, `policies`, `common`)
    - relocate owner-local route families only after their internals no longer mix responsibilities
    - remove the `plan/` name last, after imports/docs/registry point at the end-state owners
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
   - target owner flow:
     - `facts -> route -> recipe -> join sig -> phi materializer -> verifier -> cleanup`
    - inventory queue / recent closeouts:
      - `LoopCondReturnInBody`
      - `LoopTrueBreakContinue`
      - `LoopCondContinueOnly`
      - `LoopCondBreakContinue`
      - `LoopCondContinueWithReturn`
      - `GenericLoopV1`
      - `nested_loop_depth1`
      - `nested_loop_plan`
      - `generic_loop_body::nested_loop_plan`
      - `loop_scan_phi_vars_v0`
      - `loop_scan_methods_block_v0`
      - `loop_scan_v0`
      - `loop_scan_methods_v0`
      - `loop_bundle_resolver_v0`
      - `loop_collect_using_entries_v0`
      - `loop_break`
        - current handoff snapshot:
          - detailed landed seam history lives in `29bq-90-selfhost-checklist.md`
          - `loop_scan_v0` closeout is landed
          - final helper-family inventory was `body_local_policy`
          - `gather_facts_step_box` is landed
          - `apply_policy_step_box` is landed
          - `normalize_body_step_box` is landed
          - `body_local_derived_step_box` is landed
          - `carrier_updates_step_box` is landed
          - `post_loop_early_return_step_box` is landed
          - `emit_joinir_step_box` is landed
          - `merge_step_box` is landed
          - `loop_break_steps` closeout is landed
          - `promote_decision` is landed
          - `promote_prepare_helpers` is landed
          - `promote_finalize_helpers` is landed
          - `promote_runner` is landed
          - `loop_break::api` closeout is landed
          - `body_local_policy_helpers` is landed
          - `body_local_policy_inputs` is landed
          - `body_local_policy_types` is landed
          - `body_local_policy_runner` is landed
          - `body_local_facts_helpers` is landed
          - `body_local_facts` is landed
          - `body_local_trim_matcher` is landed
          - `body_local_digit_matcher` is landed
          - `body_local_common` is landed
          - `body_local_facts_shape_matchers` closeout is landed
          - `body_local_policy` closeout is landed
          - likely first seam:
            - `none confirmed`
        - likely follow-on seams:
          - `none confirmed`
        - next:
          - inventory `nested_loop_depth1`
      - `nested_loop_depth1`
        - current handoff snapshot:
          - detailed landed seam history lives in `29bq-90-selfhost-checklist.md`
          - `facts` is separated
          - `facts_helpers` is landed
          - `facts_types` is landed
          - route-local acceptance / fallback dispatch is separated
          - preheader freshness rewrite is separated
          - stmt-only fastpath ownership is separated
          - likely first seam:
            - `none confirmed`
          - closeout:
            - landed
        - likely follow-on seams:
          - `nested_loop_plan`
        - next:
          - inventory `nested_loop_plan`
      - `nested_loop_plan`
        - current handoff snapshot:
          - detailed landed seam history lives in `29bq-90-selfhost-checklist.md`
          - shared recipe-first fallback bridge is separated
          - `loop_cond_continue_with_return` bridge is separated
          - `loop_cond_break_continue` bridge is separated
          - shared recipe fallback orchestration is separated
          - recipe fallback selection policy is separated
          - likely first seam:
            - `none confirmed`
          - closeout:
            - landed
        - likely follow-on seams:
          - `generic_loop_body::nested_loop_plan`
        - next:
          - close out `generic_loop_body::nested_loop_plan`, then inventory `loop_scan_phi_vars_v0`
      - `generic_loop_body::nested_loop_plan`
        - current handoff snapshot:
          - detailed landed seam history lives in `29bq-90-selfhost-checklist.md`
          - local recipe-fallback ordering is separated
          - `strict_nested_loop_guard` / `freeze_no_plan` are separated
          - depth1 fastpath handoff is separated
          - likely first seam:
            - `none confirmed`
          - closeout:
            - landed
        - likely follow-on seams:
          - `loop_scan_phi_vars_v0`
        - next:
          - split `loop_scan_phi_vars_v0::facts`
      - `loop_scan_phi_vars_v0`
        - current handoff snapshot:
          - detailed landed seam history lives in `29bq-90-selfhost-checklist.md`
          - nested-loop depth1 fastpath handoff is separated
          - nested-loop recipe stmt-only / fastpath handoff is separated
          - found-if branch stmt partition / nested dispatch is separated
          - nested-loop segment arm is separated
          - linear segment verification / lowering is separated
          - `facts_helpers` is landed
          - `facts_types` is landed
          - `facts_shape_routes` is landed
          - likely first seam:
            - `none confirmed`
          - closeout:
            - landed
        - likely follow-on seams:
          - `loop_scan_methods_block_v0`
        - next:
          - split `loop_scan_methods_block_v0::facts`
      - `loop_scan_methods_block_v0`
        - current handoff snapshot:
          - detailed landed seam history lives in `29bq-90-selfhost-checklist.md`
          - nested-loop recipe-first fallback handoff is separated
          - linear block recipe arm split is separated
          - nested-loop stmt-only fastpath ownership is separated
          - segment-level nested dispatch is separated
          - `facts_helpers` is landed
          - `facts_types` is landed
          - `facts_shape_routes` is landed
          - `facts_recipe_builder` is landed
          - likely first seam:
            - `none confirmed`
          - closeout:
            - landed
        - likely follow-on seams:
          - `loop_bundle_resolver_v0`
        - next:
          - split `loop_bundle_resolver_v0::facts`
      - `loop_bundle_resolver_v0`
        - current handoff snapshot:
          - detailed landed seam history lives in `29bq-90-selfhost-checklist.md`
          - `pipeline` / `recipe` are already separate
          - `facts_helpers` is landed
          - `facts_types` is landed
          - likely first seam:
            - `none confirmed`
          - closeout:
            - landed
        - likely follow-on seams:
          - `loop_collect_using_entries_v0`
        - next:
          - split `loop_collect_using_entries_v0::facts`
      - `loop_collect_using_entries_v0`
        - current handoff snapshot:
          - detailed landed seam history lives in `29bq-90-selfhost-checklist.md`
          - `pipeline` / `recipe` are already separate
          - `facts_helpers` is landed
          - `facts_types` is landed
          - `facts_shape_routes` is landed
          - `facts_recipe_builder` is landed
          - likely first seam:
            - `none confirmed`
          - closeout:
            - landed
        - likely follow-on seams:
          - `plan/` destination mapping
        - next:
          - land top-level `control_flow/cleanup/` owner surface
4. `phase-29bq legacy lowerer removal`
    - landed and closed
5. `phase-29bq loop owner seam cleanup`
    - next:
      - land top-level `control_flow/cleanup/` owner surface

## Legacy Compatibility Block

- compiler lane: `phase-29bq / none`（active: monitor-only）
  - done: `JIR-PORT-08`
  - next: `none`
