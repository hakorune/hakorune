---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P198, generic string body ArrayPush write-any payload flow
Related:
  - docs/development/current/main/phases/phase-29cv/P192-HOSTBRIDGE-EXTERN-INVOKE-ROUTE-FACT.md
  - docs/development/current/main/phases/phase-29cv/P197-GENERIC-STRING-SCALAR-VOID-GUARD.md
  - lang/src/mir/builder/internal/delegate_provider_box.hako
  - src/mir/global_call_route_plan/generic_string_surface.rs
  - src/mir/generic_method_route_plan.rs
---

# P198: Generic String ArrayPush Write-Any Payload

## Problem

P197 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=BuilderDelegateProviderBox.try_emit/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`BuilderDelegateProviderBox.try_emit/1` is a string-or-null delegate wrapper:

```hako
local args = new ArrayBox()
args.push(program_json)
return hostbridge.extern_invoke("env.mirbuilder", "emit", args)
```

The hostbridge extern route already exists and returns `string_handle_or_null`.
The rejected instruction is the internal `ArrayBox.push`.

The route fact for ArrayPush is already:

```text
generic_method.push
core_op=ArrayPush
route_kind=array_append_any
value_demand=write_any
```

The generic string classifier was narrower than this fact: it only accepted
string payloads. That blocks a valid argument-vector construction path.

## Decision

Keep this in the existing generic string/string-or-void body. Do not add a
`BuilderDelegateProviderBox` by-name shape.

Align the classifier with the existing `ArrayPush write_any` route fact:

- receiver must still be proven `ArrayBox`
- canonical one-arg method MIR is accepted: `push(value)`
- two-arg method MIR is accepted only when the first arg is the receiver array:
  `push(receiver, value)`
- payload may be unknown because `array_append_any` accepts any runtime
  i64/handle payload
- payload does not become string evidence by itself

The shape must not:

- accept arbitrary method calls with unknown args
- infer unknown payloads as strings
- treat ArrayBox construction as a string return surface
- add delegate-provider name matching

## Acceptance

```bash
cargo test -q array_push_write_any --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p198_array_push_write_any_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.

## Implementation

- Kept `ArrayPush` inside the existing generic string/string-or-void classifier.
- Aligned classifier acceptance with the existing `array_append_any/write_any`
  route fact.
- Accepted both one-arg and receiver-duplicated two-arg method MIR surfaces.
- Kept unknown payloads opaque: they are appendable values, not string
  evidence.
- Updated the generic method route fact and module generic string C emitter so
  two-arg ArrayPush reads the second arg as the payload.

## Probe Result

P198 removes the previous blocker:

```text
target_shape_blocker_symbol=BuilderDelegateProviderBox.try_emit/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The stage1 `--emit-exe` probe now fails later at:

```text
target_shape_blocker_symbol=CompatMirEmitBox.emit_array_push_sequence/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```
