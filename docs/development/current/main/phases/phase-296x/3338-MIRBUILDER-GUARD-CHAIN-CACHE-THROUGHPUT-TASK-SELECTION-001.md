# 3338 - MIRBUILDER-GUARD-CHAIN-CACHE-THROUGHPUT-TASK-SELECTION-001

## Purpose
Select a guard-chain throughput cleanup before reopening LocalSSA
`finalize_compare`.

The recent hard-authority pilot guards are correct but expensive because deep
guard chains repeatedly call prerequisite guards and AOT/MIR emit paths. This
card records the cleanup task and keeps compiler semantics unchanged.

## Selected Cleanup
```text
MIRBUILDER-GUARD-CHAIN-CACHE-PREFLIGHT-001
```

## Scope
- Reuse the existing `guard_cached_run` result memo contract.
- Reuse existing `tools/bin/hako` MIR JSON / EXE cache wrapper guards.
- Identify the first current-lane guard chain where prerequisite re-execution
  can be safely avoided.
- Keep dirty-worktree caching opt-in unless a stricter contract is added.

## Explicit Non-Claims
- LocalSSA bridge implementation: `0`
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- Source Selfhost: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`

## Return Path
After the cache preflight is green, resume:

```text
MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001
```

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_guard_chain_cache_throughput_task_selection_guard.sh
```
