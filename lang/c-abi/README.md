# C ABI Kernel — Minimal Shim for Phase 20.9

Responsibility
- Provide a portable, minimal C ABI surface used by the LLVM line.
- Read‑only GC externs first (`hako_gc_stats`, `hako_gc_roots_snapshot`), plus memory/console/time/local-env helpers.
- backend-zero では `.hako` caller から object/exe emission を受ける thin transport boundary の置き場でもある。
- C ABI 自体は LLVM driver/provider の selector ではない。実際の route は
  `ny-llvmc --driver`、CAPI recipe/replay、または明示 provider が選ぶ。
- generic `hako_aot` は互換 ingress として残り、daily Boundary と同一視しない。

Inputs/Outputs
- In: Extern calls from Hakorune code compiled to LLVM; the selected
  `ny-llvmc`/CAPI route may be Boundary or an explicitly named compat lane.
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
  - `hako_diag_mem_shared_impl.inc` — TLS diagnostics / libc memory の shared source truth
  - `hako_aot_shared_impl.inc` — AOT compile/link の shared source truth
  - public path-owner names are `mir_json_path` / `obj_path` / `exe_path` under `hako_aot.h`

Caller-zero pinned-Text lowering fixture
- `shims/hako_llvmc_ffi_pinned_text_residence_carrier.inc` consumes only the
  Rust-issued `hako.pinned_text_residence_carrier@1` projection together with
  its existing backend-frame contract and writes textual LLVM containing the
  direct Residence Enter normal/trap branch and success-only Finish before
  explicit returns.
- `hako_llvmc_emit_pinned_text_residence_carrier_fixture` is a test/inspection
  helper, not a production compiler entry. It does not open the TargetMachine,
  publish an object, infer lane meaning, or select a fallback route. Missing,
  foreign, stale, duplicate, or trap Finish carrier data rejects before the
  output file is opened.

Replay admission
- `hako_aot_compile_json` is the generic AOT entry and rejects inherited
  harness replay before FFI lookup, child spawn, or object creation.
- `hako_aot_compile_json_compat_harness` is the versioned, explicit
  compatibility/oracle keep entry. It is not a production fallback and must
  remain separately censused for the staged llvmlite G1/G2/G3 retirement.

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
