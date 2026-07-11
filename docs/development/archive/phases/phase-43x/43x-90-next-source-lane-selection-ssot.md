---
Status: Landed
Date: 2026-04-03
Owner: Codex
Scope: phase-42x closeout 後に続く next source lane を選び、rust-vm を proof/compat keep のまま次の主線へ handoff する。
---

# 43x-90 Next Source Lane Selection SSOT

## Goal

- select the next source lane after vm caller starvation / direct-core owner migration
- keep proof-only VM gates frozen and non-growing
- keep `rust-vm` from regrowing into a feature-tax mainline

## Candidate Lanes

| Candidate | Read as | Notes |
| --- | --- | --- |
| `direct/core follow-up` | continue starving remaining vm-facing callers and push new work into direct/core owners | highest leverage if more mainline work still leaks back into vm-gated surfaces |
| `vm residual cleanup` | shrink the remaining proof/compat keep surfaces | lower leverage than direct/core follow-up, but still keeps rust-vm narrow |
| `archive sweep 2` | continue moving drained shims and legacy wrappers out of the live surface | useful if doc or shim pressure is still noisy |
| `kilo` | far-future optimization lane | not the next lane |

## Selection Criteria

- reduces feature tax on `rust-vm`
- keeps new capability work on direct/core owners
- avoids reopening optimization as the next lane
- has a clear current owner and a short path to proof

## Selection Result

- selected successor lane: `phase-44x stage0 direct/core follow-up`
- reason:
  - live owner pressure still sits in `tools/selfhost/lib/selfhost_build_stageb.sh`
  - runtime/direct helper pressure still sits in `tools/selfhost/lib/selfhost_run_routes.sh`
  - stage0 child capture still hardcodes `--backend vm` in `src/runner/modes/common_util/selfhost/stage0_capture.rs`
- rejected as next lane:
  - `vm residual cleanup`: useful later, but lower leverage while live callers still feed vm-gated routes
  - `archive sweep 2`: hygiene-only relative to current feature tax
  - `kilo`: far-future optimization lane

## External Review Reading

- current repo shape already supports direct/core ingress through:
  - `src/runner/mod.rs --mir-json-file`
  - `src/runner/core_executor.rs`
  - `tools/selfhost/stage1_mainline_smoke.sh`
- current repo shape still leaks live ownership through:
  - `tools/selfhost/lib/selfhost_build_stageb.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `src/runner/modes/common_util/selfhost/stage0_capture.rs`
- exact reading:
  - do not delete `vm.rs` first
  - do starve live `--backend vm` callers first
  - keep `core.hako` compat narrow and non-growing
  - keep proof-only VM gates explicit until direct/core routes replace them as defaults

## Current Front

| Item | State |
| --- | --- |
| Now | `phase-44x stage0 direct/core follow-up` |
| Blocker | `none` |
| Next | `44xA1 stage-b direct/core target lock` |

## Big Tasks

1. shortlist candidate successor lanes
2. compare leverage and maintenance cost
3. choose `direct/core follow-up`
4. hand off to `phase-44x`

## Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `43xA1` | landed | candidate lane shortlist |
| `43xA2` | landed | successor lane decision |
| `43xD1` | landed | proof / closeout |
