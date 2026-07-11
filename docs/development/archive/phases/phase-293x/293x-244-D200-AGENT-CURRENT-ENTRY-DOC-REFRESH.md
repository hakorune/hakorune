# 293x-244 D200 Agent Current Entry Doc Refresh

Status: Complete

## Purpose

D200 refreshes the AI/developer instruction entry contract after the manifest
runner cleanup work.

`AGENTS.md` is ignored locally by this repository, so durable policy must not
live only in that root file. This row adds a tracked SSOT for how agents should
read `AGENTS.md` without treating historical Phase-15 / PyVM / Cranelift notes
as current direction.

## Decision

Add:

```text
docs/development/current/main/design/agent-current-entry-contract-ssot.md
```

and wire it into:

```text
docs/development/current/main/DOCS_LAYOUT.md
docs/development/current/main/design/current-docs-update-policy-ssot.md
docs/development/current/main/design/README.md
```

Local `AGENTS.md` is refreshed to point at `CURRENT_STATE.toml` first, name the
tracked contract, and mark old fixed-phase guidance as historical. Because
`AGENTS.md` is gitignored, the tracked SSOT is the durable project record.

## Stop Lines

- Do not force-add ignored root instruction files.
- Do not make `AGENTS.md` a landed-card ledger.
- Do not copy current blocker history into AGENTS or current mirrors.
- Do not wire new guards into `dev_gate.sh`.
- Do not change compiler/runtime behavior.

## Acceptance

- A tracked agent current-entry SSOT exists.
- `DOCS_LAYOUT.md` names `AGENTS.md` as a root instruction entry, not a current
  lane ledger.
- `current-docs-update-policy-ssot.md` includes `AGENTS.md` in the mirror
  thinning contract.
- `design/README.md` links the tracked agent current-entry SSOT.
- Local `AGENTS.md` points agents to `CURRENT_STATE.toml` before old fixed
  phase guidance.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
