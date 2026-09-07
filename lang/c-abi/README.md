# Compiler C ABI, C LLVM backend, and runtime shims

Responsibility
- Provide a portable, minimal C ABI surface used by the LLVM line.
- Read‑only GC externs first (`hako_gc_stats`, `hako_gc_roots_snapshot`), plus memory/console/time/local-env helpers.
- selected-C compiler transport と、LLVM IR/object を生成する C backend の実装も含む。
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
- `shims/hako_llvmc_*` — compiler transport and physical LLVM backend
- `shims/hako_kernel.c` — libc-backed canary; not the production Rust kernel
- `shims/hako_forward_registry_shared_impl.inc` — callback registry currently
  included by both the Rust kernel's C translation unit and the separate canary
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

Lifecycle invocation ownership
- `hako_lts_open` retains the selected LLVM library, TargetMachine, TargetData,
  triple and data-layout in one private call-local session. `hako_lts_close`
  releases it and clears the owner; failed open leaves no retained resources.
- V3 currently opens, validates and closes before its pending terminal. V4 must
  retain this owner through its real `.ll`/object publication, checking the
  actual module preamble. The existing preamble self-test is not that proof.
- Direct physical JSON requires a root `process_result_site` distinct from all
  checked-operation sites. The selected I64 projection will use Fault reason
  `NYRT_FAULT_REASON_EXIT_CODE_OUT_OF_RANGE_V1` with details `{actual, 0}`.
  Neither this input validation nor session ownership emits a lifecycle object.

Boundary ownership and queued cleanup (2026-09-06)

Source/Facts/Recipe decides meaning; MirBuilder atomically publishes MIR and
definition relations. The selected C LLVM backend chooses physical instructions,
ABI placement and object output. Generated code calls the Rust runtime through
C ABI exports; C ABI does not require an additional C wrapper at runtime.
JSON may transport already-decided operands; it is not a second source resolver.

| Boundary | Owner / responsibility |
| --- | --- |
| Compiler C ABI | `capi_transport.rs` and exported C entry: arguments, borrowing, errors |
| C LLVM backend | `shims/hako_llvmc_*`: physical lowering and LLVM output |
| Runtime C ABI | `nyash_kernel` exports: generated code's calling contract |
| Runtime implementation | Rust value, handle, array, string and OS owners |
| Compatibility | Explicit ingress and libc canary, with their own selection |

Static verification at branch `55f2817a5d`, superseding the review's `93dde7b882`:
the published Rust view still reads both Call and LegacyCallV0. The same-module
C call emitter rejects malformed typed rows, but absent rows continue to legacy
classification. This is missing-row ambiguity, not retry after typed failure.
The lifecycle companion remains a pre-artifact pending terminal; this review
does not establish constructor execution or Pair EXE/linked OBJ exit 30.

Ordered tasks are owned by the current
[workstream](../../docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md#backendruntime-feedback-and-task-order-2026-09-06).
Each selected slice must retire its own replaced production edge:

1. Close constructor source/result handoff, physical consumer and actual typed
   EXE/OBJ acceptance before starting runtime reorganization.
2. Separate canonical selection from explicit compatibility at published view /
   same-module consumer. A selected canonical site without required data stops
   before artifact; remove that site's JSON name-dispatch edge. Acceptance covers
   valid, missing, malformed, duplicate and unconsumed rows plus explicit compat.
3. Replace compile-global row pointers/count/used marks and temporary environment
   mutation with call-owned physical state/options in transport and C consumer.
   Include all compile ingress that shares those globals; prove the entry/caller
   inventory before selecting a delete-set. The inventory explicitly includes the
   two C `setenv`/restore helpers in `hako_llvmc_ffi_route.inc`, alongside Rust
   transport environment mutation. Verify overlapping calls with distinct
   rows/options, failure cleanup and environment preservation. No lock was found
   in the inspected Rust llvm_codegen owner; global serialization and actual races
   are unproven. Do not claim concurrent compilation support before this closes.
4. Unify kernel hook registration in `hako_forward_bridge.rs`. Rust dot-name
   exports currently write Rust atomics and C globals; underscore C registration
   writes only C globals, so Rust dispatch does not observe that registration.
   Preserve both export spellings and C try ABI using one Rust storage owner.
   Include future dispatch, string dispatch and both raw-accessor consumers.
   Delete kernel-only C TU/build recipe/cc dependency and double writes together;
   retain the shared C implementation for the independent canary. Audit the string
   dispatch cache before deletion so no independent responsibility is removed.
   Verify cross-entry registration/dispatch, replacement, NULL unregister,
   null-out/no-call and linked ABI symbols. Define callback lifetime and in-flight
   unregister behavior before implementation; atomic storage alone is insufficient.
5. Later, shrink kernel's dependency on root `nyash-rust` along actual config,
   Box and handle consumers. Measure build dependencies and retained symbols;
   the Cargo dependency alone does not prove compiler code is in the final EXE.

Runtime inventory boundary: tracked hook declarations/definitions/direct symbol
references -> callback invocation, including kernel, canary, headers, tests and
build/guard owners. External consumers and dynamically constructed symbol names
are excluded and unknown. C try exports have no discovered direct repository
callers, which does not authorize public ABI deletion. The existing
`phase29cc_hako_forward_registry_guard.sh`, called by `dev_gate.sh`, still expects
C try calls in the Rust bridge; update that owner invariant in task 4. This is a
static contract mismatch, not an observed test failure. No build, runtime or
concurrency tests were run for this design review. Queue entries are not a closed
whole-repository census or implementation permission for an unclosed mapping.

Replay admission
- `hako_aot_compile_json` is the generic AOT entry and rejects inherited
  harness replay before FFI lookup, child spawn, or object creation.
- `hako_aot_compile_json_compat_harness` is the versioned, explicit
  compatibility/oracle keep entry. It is not a production fallback and must
  remain separately censused for the staged llvmlite G1/G2/G3 retirement.

Guards
- No Rust modules or cargo manifests under `lang/`.
- Backend code may decode transport JSON and emit LLVM. It must not reconstruct
  source targets or receiver meaning from names when published authority is missing.
- Do not turn this into a third canonical ABI. Runtime/plugin canonical ABI remains Core C ABI / TypeBox ABI v2.

Build (canary example; not the selected compiler or production Rust kernel)
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
