# Docs Tooling

This directory holds small helpers that reduce current-doc synchronization
work. Helpers here should be narrow and explicit: they may update compact
current-state pointers and create row skeletons, but they must not rewrite
historical phase ledgers or move docs.

## Phase Row Writer

Use `tools/docs/phase_row.py` for new current-row boilerplate.

Default mode is dry-run:

```bash
python3 tools/docs/phase_row.py create \
  --row 295x-200 \
  --row-number 200 \
  --slug EXAMPLE-ROW \
  --title "Example Row" \
  --scope "example scope" \
  --blocker EXAMPLE-BLOCKER-295X-001 \
  --summary "selected the example follow-on" \
  --previous-card docs/development/current/main/phases/phase-295x/295x-199-example.md \
  --queue-boundary "Select the example follow-on." \
  --land-row 199
```

Add `--write` only after checking the dry-run output.

The helper owns only repetitive row mechanics:

- create the phase card skeleton;
- update `CURRENT_STATE.toml` latest-card fields;
- update the short taskboard current blocker / queue when requested;
- append a check-script index row when `--guard` and `--guard-description` are
  provided.

It does not replace row-specific engineering judgment, implementation, or
evidence checks.
