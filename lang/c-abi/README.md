# C ABI Kernel — Minimal Shim for Phase 20.9

Responsibility
- Provide a portable, minimal C ABI surface used by the LLVM line.
- Read‑only GC externs first (`hako_gc_stats`, `hako_gc_roots_snapshot`), plus memory/console/time/local-env helpers.
- backend-zero では `.hako` caller から object/exe emission を受ける thin backend boundary の有力置き場でもある。
- legacy `llvm_ir/AotFacade` caller は archive 化し、daily route は `LlvmBackendBox -> hako_aot` へ寄せる。

Inputs/Outputs
- In: Extern calls from Hakorune code compiled to LLVM (llvmlite harness / ny-llvmc).
- Out: Simple values (i64) or newly allocated `char*` (caller frees with `hako_mem_free`).

Contracts
- Ownership: `char*` returns are callee-owned; free via `hako_mem_free()`.
- Alignment: pointers from `hako_mem_alloc/realloc` satisfy `max_align_t`.
- Thread-safety: memory API and read-only helpers are thread-safe.
- Diagnostics: use short, stable messages (NOT_FOUND/UNSUPPORTED/VALIDATION) via TLS `hako_last_error` when applicable.
  - Missing env key: `hako_env_local_get` returns NULL and sets `NOT_FOUND`.
  - LLVM lowering emits a short warn (stderr) on missing; return handle remains `0`.

Layout
- `include/` — public headers
  - `hako_hostbridge.h` — broader C ABI surface
  - `hako_aot.h` — canonical AOT compile/link header
- `shims/` — libc-backed reference implementation for canaries (`hako_kernel.c`)
  - `hako_aot.c` — AOT compile/link helper boundary の first cutover target
  - `hako_aot_shared_impl.inc` — AOT compile/link の shared source truth

Guards
- No Rust modules or cargo manifests under `lang/`.
- No parsing or codegen here; this is a plain ABI surface.
- Do not turn this into a third canonical ABI. Runtime/plugin canonical ABI remains Core C ABI / TypeBox ABI v2.

Build (example)
```
cc -I../../include -shared -fPIC -o libhako_kernel_shim.so shims/hako_kernel.c
```

Link (LLVM canary)
- Use rpath + `-L` to locate `libhako_kernel_shim.so` at runtime.
- Example flags: `-L$ROOT/target/release -Wl,-rpath,$ROOT/target/release -lhako_kernel_shim`

APIs (Phase 20.9)
- Memory: `hako_mem_alloc/realloc/free`
- GC (read‑only): `hako_gc_stats`, `hako_gc_roots_snapshot`
- Console: `hako_console_log/warn/error` (void side‑effect; returns 0)
- Time: `hako_time_now_ms`
- Local env: `hako_env_local_get` (caller frees via `hako_mem_free`)

Notes
- Future control hooks (`hako_gc_collect/start/stop`) are defined but gated; do not silently succeed.
 - Platform CRT note: Only `hako_mem_free()` may be used to free memory obtained from any `hako_*` API to avoid CRT boundary issues (Windows msvcrt/ucrt, macOS libc).
