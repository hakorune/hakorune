---
Status: Active
Decision: provisional
Date: 2026-04-02
Scope: engineering/bootstrap keep を live top-level から分離し、`tools/engineering/**` へ rehome しながら product/reference/experimental の front を薄くする。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-30x/README.md
  - docs/development/current/main/phases/phase-31x/31x-90-engineering-lane-isolation-ssot.md
  - docs/development/current/main/phases/phase-31x/31x-91-task-board.md
---

# Phase 31x: Engineering Lane Isolation

## Goal

- engineering-only tool surfaces を `tools/engineering/**` に寄せる。
- `tools/` top-level から product/main と無関係な engineering residue を減らす。
- old top-level path は必要な間だけ thin shim にし、drain 後に delete/archive する。
- selfhost/bootstrap/orchestrator の no-touch-first surfaces は先に壊さない。

## Fixed Reading

- `phase-30x backend surface simplification` は landed precursor として読む。
- `phase-31x` は docs-first ではなく source/smoke の actual rehome を含む isolation phase。
- delete を急がず、`rehome -> shim -> drain -> delete` の順で進める。
- raw backend default や deep dispatch/selfhost glue はこの phase の主題にしない。

## Non-Goals

- `src/cli/args.rs` の raw default flip
- `src/runner/dispatch.rs` の central route rewrite
- `tools/selfhost/run.sh` / `tools/selfhost/selfhost_build.sh` の早期 rewrite
- engineering keep の一括削除

## Exact Next

1. `31x-90-engineering-lane-isolation-ssot.md`
2. `31x-91-task-board.md`
3. `docs/development/current/main/phases/phase-30x/README.md`

## Canonical Child Docs

- role / disposition / no-touch-first rules:
  - `31x-90-engineering-lane-isolation-ssot.md`
- concrete task order / evidence commands:
  - `31x-91-task-board.md`

## Acceptance Summary

- low-blast engineering tools move under `tools/engineering/**`
- old top-level entrypoints become explicit compatibility shims only
- current docs point to engineering homes instead of the top-level copies
- selfhost-only smoke wrappers can move under `tools/selfhost/**` while keeping top-level shims
- selfhost/bootstrap/orchestrator surfaces stay explicit no-touch-first keeps
- delete/archive happens only after shim drain is explicit
