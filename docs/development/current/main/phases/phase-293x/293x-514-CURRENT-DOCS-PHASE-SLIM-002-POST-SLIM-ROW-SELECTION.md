# 293x-514 CURRENT-DOCS-PHASE-SLIM-002 Post-Slim Row Selection

Status: landed
Date: 2026-05-17

## Decision

`CURRENT-DOCS-PHASE-SLIM-001` closed the current docs / phase taskboard slim
row.

Select exactly one next row:

```text
MIMAP-NEXT-BEHAVIOR-SELECTION-001:
  choose the next allocator/compiler row after the cleanup sidecar sequence,
  without mixing allocator behavior with language/compiler acceptance work
```

## Worker Inventory

Worker棚卸の結論:

- Current mimalloc/language-feature granularity is acceptable.
- The lane already separates allocator behavior, compiler acceptance sidecars,
  BoxShape cleanup, and provider/global allocator activation.
- Do not broaden user-facing concurrency/language features speculatively.
- After docs slim, select one next row before implementation resumes.

## Candidate Set

| Candidate | Type | Risk | Use when |
| --- | --- | --- | --- |
| `MIMAP-NEXT-BEHAVIOR-SELECTION-001` | BoxShape | low | choose the next allocator/compiler row after the long cleanup chain |
| `FUNCTION-TYPES-PLAN-MODEL-SPLIT-001` | BoxShape | medium | no allocator behavior is ready and central MIR model cleanup is preferred |
| `PLACEMENT-EFFECT-SPLIT-001` | BoxShape | medium | placement/effect truth duplication blocks route clarity |
| `MIR-ROW-D` | BoxCount | medium-high | the next allocator row needs dense queue field-read proof |
| `NEXT-MIMAP-BEHAVIOR-ROW` | BoxShape/BoxCount | medium | a specific allocator behavior can be chosen with owner/proof/guard |

## Selected Row

```text
row:
  MIMAP-NEXT-BEHAVIOR-SELECTION-001
owner:
  docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
scope:
  inspect the landed allocator/compiler sidecar chain and select exactly one
  next row: allocator behavior, compiler acceptance sidecar, or BoxShape cleanup
stop_line:
  no source code changes
  no allocator/provider activation
  no host allocator replacement, hooks, or #[global_allocator]
  no broad language feature implementation
evidence:
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
  git diff --check
```

## Closeout

This row closes when `MIMAP-NEXT-BEHAVIOR-SELECTION-001` has a selected current
card with owner, scope, stop lines, and evidence.
