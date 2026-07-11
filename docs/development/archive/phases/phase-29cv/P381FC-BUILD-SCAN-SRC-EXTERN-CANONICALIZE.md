# P381FC Build Scan-Source Extern Canonicalize

Date: 2026-05-06
Scope: source-owner cleanup for the remaining BuildBox scan-source handoff caller.

## Context

`BuildBox.emit_program_json_v0(source, null)` already canonicalized to the
Stage1 build-surrogate extern:

```text
nyash.stage1.emit_program_json_v0_h
```

But one direct bundle-aware caller still bypassed that route:

```text
BuildBundleFacadeBox.emit_program_json_v0(...)
  -> BuildBox._emit_program_json_from_scan_src(scan_src)
```

That left Stage0 free to select the private BuildBox parser handoff helpers even
though the public build-surrogate contract already existed.

## Change

Extended MIR callsite canonicalization so:

```text
BuildBox._emit_program_json_from_scan_src/1
```

also rewrites to:

```text
nyash.stage1.emit_program_json_v0_h/1
```

The call still passes the prepared merged `scan_src` handle through unchanged.
No new route family was added; this only makes the bundle-aware direct caller
consume the same existing Stage1 extern contract as the public source-only call.

## Verification

Commands:

```bash
cargo test -q stage1_buildbox_emit_program_json_from_scan_src_rewrites_to_extern_route
bash tools/build_hako_llvmc_ffi.sh
cargo test -q stage1_emit_program_json_extern_route
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The remaining direct scan-source build-surrogate caller now avoids same-module
selection of the BuildBox parser body. The next parser handoff blocker is the
true remaining owner path, not this leftover direct caller.
