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
   - target owner flow:
     - `facts -> route -> recipe -> join sig -> phi materializer -> verifier -> cleanup`
    - closeout-ready / closed families:
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
      - current handoff snapshot:
        - detailed landed seam history lives in `29bq-90-selfhost-checklist.md`
        - current helper-family inventory is `loop_break`
        - likely first seam:
          - facts namespace entry
        - likely follow-on seams:
          - facts family consolidation inventory
4. `phase-29bq legacy lowerer removal`
   - landed and closed
5. `phase-29bq loop owner seam cleanup`
    - next:
      - add `loop_break::facts` namespace entry and inventory the remaining facts-family consolidation seam

## Legacy Compatibility Block

- compiler lane: `phase-29bq / none`（active: monitor-only）
  - done: `JIR-PORT-08`
  - next: `none`
