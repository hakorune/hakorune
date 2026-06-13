# CONC-CHANNEL-003 Await-Visible Route Bridge

Status: landed-code
Scope: future `Channel<T>` route vocabulary and fail-fast bridge.

## Decision

`CONC-CHANNEL-003` fixes the canonical channel route shapes without opening
Program JSON, MIR, LLVM, or hidden ordinary blocking calls.

```text
await ch.send(value)
await ch.recv()
await ch.close()
ch.try_send(value)
ch.try_recv()
```

## Landed Code

```text
src/runtime/channel_route.rs
HakoChannelRoute
ChannelRouteDescriptor
channel_route_descriptors()
channel_route_report_fields()
channel_route_activation_report_fields()
channel_route_source_shape_report_fields()
```

## Report Fields

```text
channel_route_await_send_descriptor_present=1
channel_route_await_recv_descriptor_present=1
channel_route_await_close_descriptor_present=1
channel_route_try_send_descriptor_present=1
channel_route_try_recv_descriptor_present=1
channel_route_hidden_blocking_ordinary_call_enabled=0
channel_route_mir_lowering_enabled=0
channel_route_program_json_enabled=0
channel_route_llvm_enabled=0
channel_route_legacy_p2p_channelbox_reused=0
```

## Stop Line

```text
no receiver type inference
no Program JSON / MIR / LLVM lowering
no source-level ordinary blocking call
no legacy P2P ChannelBox reuse
```

## Verification

```bash
cargo test -q --lib runtime::channel_route
cargo test -q --lib runtime::channel_queue
bash tools/checks/concurrency_channel_api_guard.sh
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
CONC-CONTEXT-002:
  context snapshot on nowait child creation inside explicit co/task_scope.
```
