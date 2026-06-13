# CONC-CHANNEL-002 Reference Close Semantics

Status: landed-code
Scope: future `Channel<T>` queue reference runtime.

## Decision

`CONC-CHANNEL-002` implements only the close-side reference semantics for the
future canonical `Channel<T>` queue.

It does not reuse the legacy P2P `ChannelBox`, and it does not expose a hidden
ordinary blocking source call.

```text
runtime owner:
  src/runtime/channel_queue.rs

canonical source surface:
  await ch.close()

legacy object:
  src/core/channel_box.rs ChannelBox
  not reused as Channel<T>
```

## Landed Code

```text
ChannelQueue<T>
ChannelQueue::send(value)
ChannelQueue::close()
ChannelQueue::try_recv()
ChannelQueue::recv_blocking_reference()
channel_queue_reference_report_fields()
```

`recv_blocking_reference()` exists to prove that `close()` wakes a waiting
reference receiver. It is a runtime proof helper, not a source-level blocking
API.

## Contract

```text
close marks the queue closed
close wakes current blocking reference receivers
send after close is rejected and returns the value
recv drains buffered items after close
recv returns closed after the buffer is empty
double close is rejected
```

## Report Fields

```text
channel_queue_reference_runtime_enabled=1
channel_queue_legacy_p2p_channelbox_reused=0
channel_queue_close_wakes_waiters_reference=1
channel_queue_send_after_close_rejected=1
channel_queue_drain_after_close_enabled=1
channel_queue_double_close_rejected=1
channel_queue_true_parallel_scheduler_required=0
channel_queue_source_blocking_call_enabled=0
```

## Stop Line

```text
no source-level blocking send/recv
no worker-pool route activation
no Program JSON / MIR / LLVM widening
no reinterpretation of legacy P2P ChannelBox
```

## Verification

```bash
cargo test -q --lib runtime::channel_queue
cargo test -q --lib runtime::sync_box
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
CONC-CHANNEL-003:
  await-visible send/recv route shape or fail-fast bridge
  no hidden blocking ordinary call
```
