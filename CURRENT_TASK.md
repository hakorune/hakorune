# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-12
Scope: repo root から current lane / current front / restart read order に最短で戻るための薄い pointer。

## Purpose

- root から active lane / next lane に最短で戻る
- landed history / rejected perf evidence は phase docs と investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-216x/README.md`
4. `docs/development/current/main/phases/phase-163x/README.md`
5. `git status -sb`
6. `tools/checks/dev_gate.sh quick`

## Restart Handoff

- expected worktree:
  - clean
- active lane:
  - `phase-163x primitive and user-box fast path`
- sibling guardrail:
  - `phase-137x` string corridor / exact-keeper guardrail
- immediate next:
  - `generic placement / effect`
- immediate follow-on:
  - `semantic simplification bundle`
- current stop-lines:
  - do not mix lane B with lane C (`Debug` / terminator-adjacent operand/control liveness cleanup)
  - do not mix lane B with `generic placement / effect`
  - do not mix parked `phase-96x` backlog into the active optimization lane
- parked corridor:
  - `phase-96x vm_hako LLVM acceptance cutover`
  - only remaining backlog is monitor-policy decision for the frozen `vm-hako-core` pack

## Design Owners

- implementation lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- roadmap SSOT:
  - `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- generic placement / effect docs-facts owner:
  - `docs/development/current/main/phases/phase-207x/README.md`
  - `docs/development/current/main/phases/phase-208x/README.md`
  - `docs/development/current/main/phases/phase-209x/README.md`
  - `docs/development/current/main/phases/phase-210x/README.md`
  - `docs/development/current/main/phases/phase-211x/README.md`
  - `docs/development/current/main/phases/phase-211x/211x-90-generic-placement-effect-owner-seam-ssot.md`
  - `docs/development/current/main/phases/phase-212x/README.md`
  - `docs/development/current/main/phases/phase-212x/212x-90-placement-effect-agg-local-fold-ssot.md`
  - `docs/development/current/main/phases/phase-213x/README.md`
  - `docs/development/current/main/phases/phase-213x/213x-90-sum-outer-box-sinking-consumer-seed-ssot.md`
  - `docs/development/current/main/phases/phase-214x/README.md`
  - `docs/development/current/main/phases/phase-214x/214x-90-user-box-local-body-consumer-seed-ssot.md`
  - `docs/development/current/main/phases/phase-215x/README.md`
  - `docs/development/current/main/phases/phase-215x/215x-90-thin-entry-consumer-seed-ssot.md`
  - `docs/development/current/main/phases/phase-216x/README.md`
  - `docs/development/current/main/phases/phase-216x/216x-90-sum-seed-metadata-helper-consumer-fold-ssot.md`
- thin-entry actual consumer switch owner:
  - `docs/development/current/main/phases/phase-210x/README.md`
- DCE lane split owner:
  - `docs/development/current/main/phases/phase-190x/190x-90-remaining-dce-boundary-inventory-ssot.md`
- generic memory lane-B contract owner:
  - `docs/development/current/main/design/generic-memory-dce-observer-owner-contract-ssot.md`
- observer/control lane-C contract owner:
  - `docs/development/current/main/design/observer-control-dce-owner-contract-ssot.md`
- primitive / user-box fast-path owner:
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
- lifecycle / value parent:
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
- string guardrail owner:
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
- post-primitive enum / generic owner:
  - `docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md`

## Recent Landed Summary

- `phase-165x` / `phase-166x`
  - semantic refresh, generic relation owners, and compat recovery quarantine are landed
- `phase-167x` / `phase-168x`
  - user-box method determinism and exact-route refresh are landed
- `phase-169x` through `phase-180x`
  - sibling string guardrail, `StringKernelPlan`, publication slices, and seam cleanup are landed
  - only the final emitted-MIR return-carrier cleanup stays parked on `phase-137x`
- `phase-176x` / `phase-177x` / `phase-181x` / `phase-182x` / `phase-183x` / `phase-184x` / `phase-185x` / `phase-186x` / `phase-187x` / `phase-188x` / `phase-189x` / `phase-190x` / `phase-191x` / `phase-192x` / `phase-196x`
  - semantic simplification bundle is landed through DCE lane A2
- `phase-178x` / `phase-193x` / `phase-194x`
  - BoxShape-only structure cuts are landed
- `phase-195x` / `phase-197x`
  - roadmap regroup and pointer hygiene are landed
- `phase-198x` / `phase-199x`
  - root restart surfaces are compressed and lane-B docs/facts are fixed
- `phase-209x`
  - `agg_local scalarization` owner seam is landed; folded routes are exported as MIR metadata only
- `phase-211x`
  - `generic placement / effect` owner seam is landed; folded string / sum / thin-entry routes are exported as MIR metadata only
- `phase-212x`
  - `placement_effect_routes` now also folds placement-relevant `agg_local` routes while storage-only typed-slot routes remain agg-local-only
- `phase-213x`
  - current sum lowering now seeds the local aggregate route from `placement_effect_routes` first, with sum-specific metadata kept as fallback
- `phase-214x`
  - current user-box local aggregate seeding now reads folded `placement_effect_routes` first, with thin-entry subject lookup kept as fallback
- `phase-215x`
  - current thin-entry consumer seeding now reads folded `placement_effect_routes` first, with `thin_entry_selections` kept as fallback
- `phase-216x`
  - current sum local seed metadata helper now reads folded `placement_effect_routes` first, with legacy thin-entry/sum/agg-local metadata kept as fallback
- `phase-200x`
  - dead `Load` pruning is now landed for definitely private carrier roots with copy-only alias propagation
  - first cut keeps `Store`, `Debug`, terminator, and generic placement/effect widening out of scope
- `phase-201x`
  - overwritten `Store` pruning is now landed for definitely private carrier roots on the same block with copy-only alias propagation
  - first cut keeps cross-block store reasoning, forwarding, `Debug`, and terminator-adjacent operand/control liveness cleanup out of scope
- `phase-202x`
  - observer/control ownership is now inventoried as a docs-only cut
- `phase-203x`
  - `Debug` is now explicitly locked as a permanent observer anchor in mainline DCE
- `phase-204x`
  - lane `C2a` is now landed: mainline DCE explicitly keeps `Return.value`, `Branch.cond`, and reachable edge args live as control-anchor operands
- `phase-205x`
  - legacy instruction-list control-anchor seeding is removed; mainline DCE now keeps control-anchor operands only through `block.terminator` and reachable edge args
- `phase-206x`
  - DCE / SimplifyCFG handoff boundary is now explicit in docs and code
  - immediate next is the next layer step, starting with `generic placement / effect`

## Current Checks

1. `git status -sb`
2. `tools/checks/dev_gate.sh quick`
3. `git diff --check`

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-163x/README.md`
4. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
5. `docs/development/current/main/phases/phase-137x/README.md`

## Current Front

- read [phase-163x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-163x/README.md) for the active implementation lane
- read [phase-137x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-137x/README.md) for the sibling string guardrail
- read [phase137x-substring-rejected-optimizations-2026-04-08.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md) before retrying any substring-local perf cut
