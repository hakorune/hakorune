# shared/backend

目的
- backend-zero の official `.hako` caller boundary をここに置く。
- `.hako` 側は backend orchestration だけを持ち、raw LLVM API は持たない。

Current owner
  - `llvm_backend_box.hako`
  - thin backend boundary の caller facade
  - final target は `LlvmBackendBox -> hako_aot -> backend helper`
  - first implementation uses `MirV1MetaInjectBox` + `CodegenBridgeBox -> HostFacadeBox.call("loader","codegen.*", ...)`
  - public first-cut contract:
    - `compile_obj(json_path)` -> object path or `null` with `[llvmbackend/*]`
    - `link_exe(obj_path, out_path, libs)` -> `1` or `null` with `[llvmbackend/*]`
      - non-empty `libs` is currently forwarded as a single extra-ldflags string
      - empty `libs` still falls back to `HAKO_AOT_LDFLAGS` under the C boundary

Non-goals
- legacy `llvm_ir/AotFacade` route をここへ混ぜない。
- libLLVM の広い surface を `.hako` へ見せない。

Pointers
- C helper:
  - `lang/c-abi/shims/hako_aot.c`
- boundary SSOT:
  - `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`
