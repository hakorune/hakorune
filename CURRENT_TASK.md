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
`LANGUAGE-RECOVERABLE-FAILURE-D0`. `NORMAL-SOURCE-PLAN0-prime-r1` is accepted.
Follow `current_execution_row` and `latest_card_path` in `CURRENT_STATE.toml`;
profile admission, default routing, and legacy callers remain outside the
active bounded source-plan series until explicitly selected there.

`STATIC-BOX-DERIVE-COMPAT0-S0` is closed: static boxes receive no
receiver-based default `equals` / `toString`, ordinary boxes remain unchanged,
and Main expansion stays strict. The fresh Stage-B rerun reaches
`STAGEB-GENERIC-LOOP-TRANSIENT-TYPE-D0`: its exact nested instance-call result
has no lowering-time transient type. Do not backfill that type in GenericLoop,
from metadata, names, annotations, runtime, or a fallback. `OWN-GRAM-REJECT0`
has its Rust parser half landed; its Hako freeze-tag WIP remains parked until
this new design stop selects an exact result-representation owner.
