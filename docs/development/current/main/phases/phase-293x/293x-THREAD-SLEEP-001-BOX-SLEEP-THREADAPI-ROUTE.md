# THREAD-SLEEP-001: Box Sleep ThreadApi Route

Status: landed
Date: 2026-06-13
Scope: Box runtime substrate cleanup

## Decision

Main runtime Box sleep/polling waits use Ring0 `ThreadApi::sleep`.

This is a behavior-preserving owner cleanup. `StdThread::sleep` is still the
default implementation, but user-visible Box methods no longer call
`std::thread::sleep` directly.

## Implemented

```text
src/boxes/time_box.rs:
  TimeBox.sleep -> Ring0 ThreadApi::sleep

src/boxes/sound_box.rs:
  playback/timing waits -> Ring0 ThreadApi::sleep

src/boxes/socket_box.rs:
  accept-loop polling wait -> Ring0 ThreadApi::sleep
```

## Non-Goals

```text
plugin cdylib thread/sleep migration
runner/selfhost polling sleep migration
macro child process polling sleep migration
test-only wait cleanup
source-level thread syntax
nowait worker-pool activation
mod.rs report re-export cleanup
ThreadRegistryRole descriptor cleanup
```

## Classified Remaining Sites

```text
src/runtime/channel_queue.rs:
  test-only close-waiter spawn/sleep

plugins/nyash-net-plugin/**:
  independent plugin cdylib internals

src/macro/macro_box_ny/child.rs:
  macro child process polling wait

src/runner/modes/**:
  runner/selfhost process polling waits

src/boxes/p2p_box/tests.rs:
  test-only wait
```

## Report Contract

```text
box_substrate_direct_std_thread_sleep_count=0
time_box_threadapi_sleep_route=1
sound_box_threadapi_sleep_route=1
socket_box_threadapi_sleep_route=1
channel_queue_test_only_spawn_classified=1
plugin_direct_thread_sites_classified=1
source_syntax_exposure=0
nowait_os_thread_spawn=0
```

## Verification

```bash
rg -n "std::thread::sleep" \
  src/boxes/time_box.rs \
  src/boxes/sound_box.rs \
  src/boxes/socket_box.rs

cargo fmt --check
cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
```
