---
Status: SSOT
Date: 2026-05-14
Scope: Loop-surface cleanup lane before resuming PACKED-001.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/loop-range-parser-capsule-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-289-LOOPCLEAN-001-LOOP-CLEANUP-PHASE.md
---

# Loop Cleanup Before PackedArray SSOT

## Decision

Before resuming `PACKED-001 PackedArray eligibility gate`, clean up the loop
surface and internal naming drift.

Canonical repetition remains:

```hako
loop cond {
    ...
}

loop i in start..end {
    ...
}

loop {
    ...
}
```

Non-canonical / legacy accepted surfaces:

```hako
while cond {
    ...
}

for i in start..end {
    ...
}
```

These legacy surfaces must not regain canonical documentation status.

## Problem

Historical context: this cleanup lane started while code still carried three
loop-family AST shapes:

```text
Loop       condition + body
While      condition + body
ForRange   var + start + end + body
```

`While` is semantically duplicate with `Loop`. Stage1 Program JSON already
lowers both to `"type": "Loop"`.

`ForRange` was later renamed to `LoopRange`. LoopRange is not equivalent to
plain `Loop`: it carries index binding, range bounds, entry capture,
continue-step semantics, and verifier facts. Cleanup rows must not merge it into
plain `Loop`.

## Cleanup Rules

- BoxShape cleanup only until a row explicitly changes behavior.
- Do not implement source-level range desugar such as `local i; loop; i += 1`.
- Do not merge `LoopRange` into plain `Loop` in cleanup rows.
- Do not make `while` or `for` canonical again.
- Keep compatibility decoding for old AST JSON shapes where needed.

## Rows

| Row | Scope | Type |
| --- | --- | --- |
| `LOOPCLEAN-001 loop cleanup phase` | docs-only phase cut and task split | docs |
| `LOOPCLEAN-002 while parser normalization` | Complete as `293x-290`; new parsed `while` returns `ASTNode::Loop`; JSON `While` compat remains decode-only for now | BoxShape parser cleanup |
| `LOOPCLEAN-003 while variant quarantine` | Complete as `293x-291`; keep `ASTNode::While` as legacy-only input and guard Program(JSON) Loop compatibility | BoxShape cleanup |
| `LOOPCLEAN-004 range parser helper commonization` | Complete as `293x-292`; legacy `for i in a..b` and canonical `loop i in a..b` share range-header parsing | BoxShape parser cleanup |
| `LOOPCLEAN-005 LoopRange AST rename` | Complete as `293x-405`; stale internal `ForRange` AST name was renamed to `LoopRange`, old JSON decode stays compat-only | BoxShape cleanup |
| `LOOPCLEAN-006 while parser facade merge` | Complete as `293x-406`; Stage-3 `while` compatibility routes through `parse_loop()` and no separate `parse_while_stage3()` facade remains | BoxShape parser cleanup |
| `LOOP-003 Stage1 LoopRange lowering` | entry-bound capture, readonly index, continue-safe step, verifier facts | Stage1 semantics |

## Recommended Order

1. `LOOPCLEAN-002`
2. `LOOPCLEAN-003`
3. `LOOPCLEAN-004`
4. resume `PACKED-001` unless loop cleanup exposes a blocker
5. leave `LOOPCLEAN-005` and `LOOP-003` for docs-first / Stage1 route work

## Stop Line

This SSOT does not implement code changes. It only makes loop cleanup the
current BoxShape lane before PackedArray work resumes.
