# 3339 - MIRBUILDER-GUARD-CHAIN-CACHE-PREFLIGHT-001

## Purpose
Run a narrow guard-chain cache preflight before returning to LocalSSA
`finalize_compare`.

This card does not change compiler semantics or cache defaults. It only proves
that the existing cache helpers are available and that the LocalSSA design-stop
lane can resume after the cache-selection detour.

## Checks
- `guard_cached_run` memoizes repeated prerequisite guard execution.
- `tools/bin/hako` MIR JSON / EXE cache wrappers remain green.
- Dirty-worktree guard-result caching remains opt-in.
- LocalSSA `finalize_compare` design-stop may resume after this preflight.

## Explicit Non-Claims
- Cache implementation change: `0`
- Dirty cache default change: `0`
- LocalSSA bridge execution: `0`
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Return Path
```text
MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001
```

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_guard_chain_cache_preflight_guard.sh
```
