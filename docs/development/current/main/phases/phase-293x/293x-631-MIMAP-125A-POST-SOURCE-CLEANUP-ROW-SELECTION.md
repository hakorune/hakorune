# 293x-631 MIMAP-125A Post-Source-Cleanup Row Selection

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-125A` is a planning-only row after the focused source-cleanup slices:

```text
RUNTIME-UNWRAP-001
WASM-LOG-001
```

The next step should return to the mimalloc / hako_alloc implementation lane
unless the next allocator row exposes a concrete compiler or language
acceptance blocker.

## Scope

- Review the current segment allocation modeled lane.
- Select exactly one next mimalloc / hako_alloc or Hakorune compiler row.
- Keep provider activation and host allocator replacement closed.

## Stop Lines

- No allocator behavior in this row.
- No compiler route behavior.
- No source syntax.
- No cleanup bundle.
- No provider activation.
- No host allocator replacement.
- No backend matchers.
- No silent fallback.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `125A.1` | Read the taskboard and granularity SSOT. | next candidates are concrete. | no implementation |
| `125A.2` | Pick exactly one next row. | selected card exists with owner/proof/guard/stop lines. | no bundle |
| `125A.3` | Update current pointers. | pointer guard and diff check pass. | no behavior |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
