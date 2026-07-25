# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-26
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read its `latest_card_path`.
3. Read its `latest_workstream_card` when present.
4. Check `active_lane` and `current_blocker_token`; do not infer them here.
5. Run:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Run heavier gates only when the active card requires them. Current scope,
acceptance, parked items, and non-claims belong in the active card and the
workstream SSOT, not this pointer.

When `current_blocker_token` contains `DESIGN-STOP`, do not invent a new
executable owner from historical mirrors. When it names an implementation
row, follow only the `latest_card_path` contract.

## Handoff

Continue only the exact `current_blocker_token` and `latest_card_path` named by
`CURRENT_STATE.toml`.

`LANGUAGE-DOCS-POSTFIX-CATCH-D1-CLOSEOUT` is closed. It recorded the accepted
tryless surface without changing registry, parser, AST, MIR, runtime, backend,
or JSON behavior:

```text
source try / throw       = rejected in both language profiles
postfix catch            = canonical protected-region target
catchable target Outcome = RecoverableFailure
Fault                    = terminal and non-catchable
cleanup                  = separate scope-exit owner
box.fini()               = separate lifecycle owner
```

`RecoverableFailure` producer and ABI remain parked for
`LANGUAGE-RECOVERABLE-FAILURE-D0`. The active row is now
`NORMAL-FILE-VM0-G0`: promote the Forge guard into the production route guard.
The connected explicit, default-off `normal-file-vm-reference` CLI has already
passed real-binary status/diagnostic and feature-gated parity. Prove one
central selector, one Raw caller, one normal caller, default-route isolation,
shared authorities, no fallback, and the below-800-line boundary. The existing
Raw reference route, normal/default route, `compile_with_source`, and every
legacy caller remain untouched. After this row, run the core-complete proof.
