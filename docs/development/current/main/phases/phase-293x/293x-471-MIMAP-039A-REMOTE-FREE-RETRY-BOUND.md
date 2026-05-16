# 293x-471 MIMAP-039A Remote-Free Retry Bound

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-039A` is the BoxShape cleanup selected by `MIMAP-038B`.

It replaces the raw retry-bound literal in:

```text
HakoAllocRemoteFreePolicy.pushRetry(...)
```

with a named owner method.

## Scope

- Add `HakoAllocRemoteFreePolicy.maxPushRetries()`.
- Use the named value in `pushRetry`.
- Add a focused guard that rejects `retries < 5` in the retry loop.
- Reuse the existing hako_alloc remote-free policy EXE proof.

## Stop Lines

- Do not change the retry count.
- Do not change pointer atomic route behavior.
- Do not add pointer fetch_add rows.
- Do not add page queue / facade behavior.
- Do not add provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- Do not add backend `.inc` matcher shortcuts.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `039A.1` | Add retry-bound SSOT. | Owner method and stop lines are documented. | no behavior change |
| `039A.2` | Replace raw loop bound with named value. | Existing proof output remains unchanged. | no route widening |
| `039A.3` | Add focused guard. | Raw `retries < 5` cannot return. | no backend matcher |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_remote_free_retry_bound_guard.sh
bash tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row closes when the retry bound has a named owner and current moves to the
next planning row.

## Landed Implementation

```text
owner:
  lang/src/hako_alloc/memory/remote_free_policy_box.hako
proof app:
  apps/hako-alloc-remote-free-policy-proof/main.hako
guard:
  tools/checks/k2_wide_hako_alloc_remote_free_retry_bound_guard.sh
```

Closeout:

```text
current blocker moves to MIMAP-039B post-remote-free-retry-bound row selection.
```
