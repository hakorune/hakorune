---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: `phase-35x stage-a compat route thinning` の concrete queue と evidence command をまとめる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-35x/README.md
  - docs/development/current/main/phases/phase-35x/35x-90-stage-a-compat-route-thinning-ssot.md
---

# 35x-91 Task Board

## Current Queue

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `35xA payload owner split` | active | captured payload-family branching を selfhost orchestration から外す |
| 2 | `35xB orchestration-only lock` | queued | Stage-A route sequencing を thin owner として固定する |
| 3 | `35xC proof/closeout` | queued | Stage-A direct/compat split を evidence 化して handoff する |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `35xA1` | landed | captured payload resolver rehome |
| 2 | `35xA2` | active | `selfhost.rs` orchestration-only lock |
| 3 | `35xB1` | queued | Stage-A compat keep/no-widen lock |
| 4 | `35xC1` | queued | proof/closeout |

## Evidence Commands

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
git diff --check
rg -n 'resolve_captured_payload_to_mir|resolve_program_payload_to_mir|try_run_selfhost_pipeline|enforce_stage_a_' \
  src/runner/selfhost.rs \
  src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs \
  src/runner/modes/common_util/selfhost/stage_a_policy.rs \
  docs/development/current/main/phases/phase-35x
cargo test --manifest-path Cargo.toml resolve_captured_payload_to_mir_ -- --nocapture
cargo check --bin hakorune
```

## Current Result

- current front:
  - `35xA2 selfhost orchestration-only lock`
- current residue reading:
  - `stage_a_compat_bridge.rs::resolve_captured_payload_to_mir(...)` owns captured payload-family resolution
  - `resolve_program_payload_to_mir(...)` remains the Program(JSON v0) compat fallback owner
  - `selfhost.rs::try_run_selfhost_pipeline(...)` now delegates captured payload branching and focuses on spawn/route sequencing/terminal accept
