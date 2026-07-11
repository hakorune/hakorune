# 3235 - GUARD-CACHE-EMIT-EXE-AND-DIRTY-MEMO-001

Status: landed

## Scope

Reduce deep guard-chain latency without changing guard semantics.

ProgramJSON lifecycle guards repeatedly compile the same generated `.hako`
probe through `tools/bin/hako --emit-exe`. `--emit-mir-json` already had an
input-hash cache, but executable emission did not. In addition, prerequisite
guard memoization skipped dirty worktrees with untracked files, which is common
while developing new card/fixture/script rows.

This card adds:

```text
tools/bin/hako --emit-exe cache
guard_cached_run dirty-worktree memo support for untracked file contents
```

## Acceptance

```text
--emit-exe cache uses binary fingerprint, input hash, normalized args, and HAKO/NYASH env hash.
--emit-exe cache normalizes the output path like --emit-mir-json.
cache-hit executable outputs remain executable.
HAKO_EMIT_EXE_CACHE=0 disables the executable cache.
HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 includes tracked, staged, and untracked content in the dirty digest.
changing untracked content invalidates dirty guard result memoization.
```

## Evidence

```bash
bash tools/checks/hako_emit_exe_cache_wrapper_guard.sh
bash tools/checks/guard_result_cache_dirty_untracked_memo_guard.sh
```

Expected result:

```text
hako-emit-exe-cache-wrapper-guard: cache_status=miss_then_hit
guard-result-cache-dirty-untracked-memo: dirty_untracked_guard_cache_memo=1
```

## Non-Claims

```text
MIR JSON cache semantics are unchanged.
guard dependency flattening is not added.
runtime parity semantics are unchanged.
ProgramJSON migration semantics are unchanged.
Source Selfhost remains unclaimed.
```
