# Check Scripts Legacy Ledger

Status: Compatibility note

The active guard entrypoint is [`check-scripts-index.md`](./check-scripts-index.md).

This file intentionally does not preserve the old full per-row table in the
working tree. The old table made the active docs hard to scan and also carried
historical wording that current naming guards must not reintroduce.

For old row archaeology, use git history for this file before the check-scripts
index slimming change. For current work, add stable public entries to
`check-scripts-index.md` only when the guard is reusable, behavior-changing, or
named by an active card.
