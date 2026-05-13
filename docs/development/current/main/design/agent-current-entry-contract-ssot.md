---
Status: SSOT
Date: 2026-05-13
Scope: `AGENTS.md` の current-first 読み順と historical section の扱い。
Related:
  - AGENTS.md
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/tools/check-scripts-index.md
---

# Agent Current Entry Contract

## Purpose

`AGENTS.md` is local AI/developer instruction material. It is intentionally
ignored by git in this repository, so durable policy must also live in tracked
docs.

This SSOT fixes how agents should read that local file without reviving old
phase-specific guidance.

## Decision

Read current-state documents first:

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `CURRENT_TASK.md`
3. `docs/development/current/main/05-Restart-Quick-Resume.md`
4. `docs/development/current/main/10-Now.md`
5. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
6. `docs/development/current/main/DOCS_LAYOUT.md`

Then read `AGENTS.md` for personality, always-on engineering rules, and
stop-the-line policy.

If a fixed phase name, old backend preference, or historical runtime line in
`AGENTS.md` conflicts with `CURRENT_STATE.toml`, the current-state SSOT wins.

## Historical Sections

Sections about these topics in `AGENTS.md` are historical unless the active
card explicitly reopens them:

- Phase-15 / PyVM development flow
- Cranelift/JIT branch purpose
- old feature-addition pause until VM bootstrap
- old fixed selfhost gate examples
- old PyVM dev helper environment setup

They may remain in the local file for traceability, but new work must not take
them as current direction.

## Current Guard/Proof Entry

Current guard/proof entrypoints are listed in:

```text
docs/tools/check-scripts-index.md
```

Manifest runner pilots keep stable shell entrypoints:

```text
tools/checks/run_row_guard.sh
tools/checks/run_proof_app.sh
```

Their shared implementation is:

```text
tools/checks/lib/manifest_runner.py
```

These pilots are local-run/index-listed unless a later card explicitly promotes
them into `dev_gate.sh` or allocator-wide.

## Update Policy

Do not update `AGENTS.md`, `CURRENT_TASK.md`, `10-Now.md`, restart mirrors,
phase README, or taskboards for every landed card.

Update `AGENTS.md` only when root AI/developer instruction policy changes.
When that happens, update this tracked SSOT and the current docs layout/update
policy docs in the same slice.

## Non-Goals

- no physical archive/move of local `AGENTS.md`
- no attempt to make ignored root instruction files versioned
- no per-card landed history in `AGENTS.md`
- no new guard wiring
