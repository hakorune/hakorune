# P26: ny-llvmc entry-args birth pure-first pin

Scope: unblock the next `selfhost_build.sh --exe` cleanup step by teaching the
current `ny-llvmc(boundary pure-first)` lane the narrow direct-MIR shape emitted
for `static box Main { method main(args) { return <int> } }`.

## Why

P24/P25 left `selfhost_build.sh --exe` on the Program(JSON v0) -> MIR(JSON) ->
EXE route because the direct source -> MIR(JSON) -> `ny-llvmc` probe failed on
the quick `main(args)` fixture with:

```text
unsupported pure shape for current backend recipe
```

The smaller `main()` const-return shape already compiles through pure-first.
The failing delta is the synthetic entry-args initialization:

```json
{"op":"newbox","dst":1,"type":"ArrayBox"}
{"op":"copy","dst":2,"src":1}
{"op":"mir_call","dst":null,"mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","name":"birth","receiver":2},"args":[]}}
{"op":"const","dst":3,"value":{"type":"i64","value":7}}
{"op":"ret","value":3}
```

## Decision

- Accept only the method-call birth shape for `ArrayBox.birth`/`MapBox.birth`
  when the receiver is already known to come from a matching `newbox` birth and
  the call has no return destination.
- Treat that method call as the constructor-finalization no-op for this MIR
  dialect. The actual runtime handle is already produced by `newbox`.
- Do not enable harness replay and do not broaden generic method fallback.
- Keep `selfhost_build.sh --run` on its existing Program(JSON v0) path until the
  separate `--mir-json-file` execution loader gap is fixed.

## Files

- `lang/c-abi/shims/hako_llvmc_ffi_mir_call_surface_policy.inc`
- `lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc`
- `lang/c-abi/shims/hako_llvmc_ffi_mir_call_dispatch.inc`
- `tools/smokes/v2/profiles/integration/apps/phase29ci_source_mir_nyllvmc_main_args_birth_min.sh`
- `tools/selfhost/lib/selfhost_build_route.sh`
- `tools/selfhost/lib/selfhost_build_exe.sh`

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/phase29ci_source_mir_nyllvmc_main_args_birth_min.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
bash tools/checks/current_state_pointer_guard.sh
```

The new smoke must prove:

- direct source -> MIR(JSON) through the direct emit helper
- default `ny-llvmc` route selects `boundary pure-first` with `compat_replay=none`
- no `[llvm-route/replay] lane=harness`
- linked executable exits with the source return value

After that pin is green, the normal `selfhost_build.sh --exe` route can bypass
Stage-B Program(JSON v0). Diagnostic routes that request `--run`, `--keep-tmp`,
or `NYASH_SELFHOST_KEEP_RAW=1` stay on the Stage-B artifact path for now.
