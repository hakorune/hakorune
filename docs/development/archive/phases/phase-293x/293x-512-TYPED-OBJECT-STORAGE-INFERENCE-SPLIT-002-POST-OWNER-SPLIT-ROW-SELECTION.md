# 293x-512 TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-002 Post-Owner-Split Row Selection

Status: landed
Date: 2026-05-17

## Decision

`TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-001` closed the typed-object storage
inference owner-layout split.

Select exactly one next cleanup row:

```text
CURRENT-DOCS-PHASE-SLIM-001:
  slim the phase-293x current docs/taskboard pointers so the next agent can
  find the live row without reading duplicated historical tables
```

## Worker Inventory

Worker棚卸で、次の残り cleanup 候補を確認した:

| Candidate | Risk | Why not now |
| --- | --- | --- |
| `CURRENT-DOCS-PHASE-SLIM-001` | low | current docs/taskboard duplicate live-row wording and are safe to slim before more code rows |
| `FUNCTION-TYPES-PLAN-MODEL-SPLIT-001` | medium | `src/mir/function/types.rs` is large, but it touches central MIR model types |
| `GLOBAL-CALL-RETURN-PROFILE-SPLIT-001` | medium | string/global route analysis is large, but depends on current route rows |
| `USERBOX-KNOWN-RECEIVER-SEED-SPLIT-001` | medium | seed route cleanup should stay separate from active allocator behavior |
| `PLACEMENT-EFFECT-SPLIT-001` | medium | placement/effect is semantic-route adjacent; do after docs/current slim |

## Why This Row

The code owner split is now landed. Before selecting another behavior-adjacent
compiler file, slim the current phase docs that agents read first:

```text
docs/development/current/main/phases/phase-293x/README.md
docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
docs/development/current/main/05-Restart-Quick-Resume.md
docs/development/current/main/10-Now.md
CURRENT_TASK.md
```

The goal is not to erase history. The goal is to keep live pointers short and
make the phase taskboard point to landed cards instead of restating the same
current status in multiple places.

## Selected Row

```text
row:
  CURRENT-DOCS-PHASE-SLIM-001
owner:
  docs/development/current/main/phases/phase-293x/
  docs/development/current/main/
scope:
  deduplicate live current-row wording, convert repeated phase/taskboard
  current-status paragraphs into pointers, and keep the restart path readable
stop_line:
  no task deletion
  no landed-card archive/delete
  no current-state semantic change beyond selecting this row
  no code behavior change
evidence:
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
  git diff --check
```

## Stop Lines

- Do not archive or delete active phase cards.
- Do not rewrite historical landed card contents.
- Do not change mimalloc/provider activation policy.
- Do not mix this docs-slim row with code owner cleanup.

## Closeout

This row closes when `CURRENT-DOCS-PHASE-SLIM-001` has a selected current card
with owner, scope, stop lines, and evidence.
