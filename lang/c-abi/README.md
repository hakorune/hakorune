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

Intrinsic allocation transport
- The existing published frame supports kind8 `IntrinsicArrayNew`, with an exact
  site and required dst, plus an explicit intrinsic body tag. It reuses the
  selected Array allocation ABI. Named construction retains its legacy encoding.
- Consumer acceptance does not activate literal source producers; raw/typed/Core
  source cutover is separately gated by the construction design and source tests.

Physical definition plan
- `shims/hako_llvmc_ffi_physical_definition_plan.inc` owns planned leaf and
  same-module memberships and their entry-metadata reader. Both memberships
  may hold one symbol; actual body eligibility and emitted-state remain separate.
  Existing declaration, membership and ordered definition consumers read this
  owner. Capacity, duplicate count and partial-error behavior are unchanged.
  The reader now retains its result and borrowed overflow detail without
  invoking the compile diagnostic owner. The sole generic prescan consumer
  reports that detail before the existing generic plan error, at the same late
  stage. First-error precedence and early pattern success are unchanged.
  The owner unit test deliberately provides no diagnostic callback.
- Focused tests: compile/run `tests/physical_definition_plan_test.c` with yyjson;
  run `python3 lang/c-abi/tests/physical_definition_emission_test.py` after the
  selected C build. Its optional library path compares the same22 physical
  role/metadata/capacity cases with the parent. Named allocation's60 cases and
  the existing selected Dynamic provenance producer cover the adjacent emitters.
  Synthetic physical cases do not establish source-family admission.

Pure-first document ownership
- `compile_json_compat_pure` owns one parsed document and frees it after the
  borrowed core returns. Validator, pinned census, route readers and generic
  emitters borrow that document; they do not free it. The standalone file
  validator keeps its public ABI and uses the same private validator.
- Validation order and late definition-plan reading are preserved, including
  the earlier indexof pattern return. Non-document cleanup is unchanged.
  Downstream legacy exact-seed/replay file readers and other ABI entrypoints
  are outside this parse-once boundary; selected typed rows reject those routes.
- Build the ownership instrument with ASan, then run the seven physical cases:

```bash
cc -fsanitize=address -fno-omit-frame-pointer -g \
  -Iplugins/nyash-json-plugin/c/yyjson \
  lang/c-abi/tests/pure_document_lifetime_driver.c \
  lang/c-abi/shims/hako_aot.c lang/c-abi/shims/hako_json_v1.c \
  plugins/nyash-json-plugin/c/yyjson/yyjson.c -ldl \
  -o /tmp/hako-document-lifetime
python3 lang/c-abi/tests/pure_document_lifetime_test.py /tmp/hako-document-lifetime
```

- An optional second argument to the Python runner is an existing source-issued
  selected Dynamic JSON body, adding its eighth case. Counters require one read,
  one free per successful parse and no live document at return; ASan checks
  invalid access/double free. LeakSanitizer is disabled because unrelated LLVM
  and global allocations are outside this test. These cases prove document
  lifetime, not source admission, Map cutover or concurrent compilation.

Allocation configuration ownership
- File ingress captures `HAKO_TYPED_OBJECT_STORE`, `HAKO_ARRAY_SLOT_STORE` and
  `HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER` once through the private common owner.
  The borrowed core receives runtime flags and an independent helper boolean.
  Nested Named/field/method emitters and runtime requirements use this value.
  Runtime bits remain1/2; single-thread exact helper selection is not a new bit.
- `generic_method_lowering.inc` keeps dispatch; direct Array emission and fused
  String slot stores are included at their original lexical positions from
  `generic_direct_array_emit.inc` and `generic_array_string_slot_store.inc`.
  This is physical size separation, not new semantic authority.
- Build `tests/allocation_config_capture_driver.c` with the same C dependencies
  as the document driver above (ASan optional), then run:
  `python3 lang/c-abi/tests/allocation_config_capture_test.py DRIVER`.
  Its16 physical cases cover12 input classes plus unset/unknown/nonexact values.
  After capture the driver changes ambient settings; both walkers' allocation,
  typed field get/set, Array get and generated runtime requirements must still
  follow the capture. No atomic environment snapshot or concurrency claim.

Named allocation emission
- `shims/hako_llvmc_ffi_named_allocation_select.inc` selects the existing physical
  consumer once for the generic and same-module emitters. Walker and array-store
  choice are explicit inputs; typed-plan reads remain lazy after builtin/alias
  precedence. Materialization, register facts and diagnostics stay in emitters.
- This extraction changes no language or public ABI contract. Prescan is still
  a separate observer; it does not provide allocation admission for the Map frame.
- Focused reproduction after `bash tools/build_hako_llvmc_ffi.sh`:
  `cc -Wall -Wextra lang/c-abi/tests/named_allocation_select_test.c -o /tmp/hako-named-select`
  then `/tmp/hako-named-select` and
  `python3 lang/c-abi/tests/named_allocation_emission_test.py`.
  The latter accepts an optional parent library path for the same 60 physical
  cases. It checks emitted symbols, alias execution and rejection before object;
  its machine-code/relocation digests exclude temporary object path headers.
  Whole-program layout/prepass rejection is dependency evidence, not proof that
  an unreachable emitter branch ran. No source-family or Map cutover claim.

Map literal runtime boundary

The unpublished static v2 value projection separates result kind from finite
physical operation selection (integer/Bool/String comparison, String concat,
integer binary and integer/Bool Not). Body opcode and operands remain the sole
instruction graph. This header schema does not activate a consumer: both C
walkers must validate and honor the selection before cutover, including direct
integer Eq/Ne instead of the generic dynamic String-handle comparison helper.
- The [v1 runtime contract](../../docs/reference/abi/nyrt_c_abi_v0.md#selected-map-literal-store-v1)
  fixes explicit value kinds, OK/InvalidContract and length-aware String input.
  Kernel exports are implemented and tested; compiler consumers remain pending.
  Existing static C rows
  cannot be silently reinterpreted; formal-domain and input projection remain
  gated by the Map owner. Legacy Any Map entrypoints keep their current contract.

Lifecycle invocation ownership
- `hako_lts_open` retains the selected LLVM library, TargetMachine, TargetData,
  triple and data-layout in one private call-local session. `hako_lts_close`
  releases it and clears the owner; failed open leaves no retained resources.
- The selected Rust host now uses V4 and retains the target session through
  `.ll` verification, LLVM18 object generation and atomic output publication.
  V2/V3 pending exports, frame ABI and exclusive validators/tests are retired;
  V4 is the sole selected lifecycle object consumer. Physical-v2 validation,
  target/session core and runtime descriptors remain shared owners.
- V4 accepts only `hako.published-lifecycle-physical-program.v2`. The v2 parser
  replaces v1; it checks structure/SSA, then V4 checks type and cohort coverage.
  Parser success alone does not prove executable input. Receiver is explicit,
  Birth formals use kind/payload lanes, and Bool constants keep their own kind.
- NativeArray uses the same parser/V4 consumer with an explicit ABI1 requirement,
  one root and no object profile/layout. Its finite admission validates live
  allocation identities through Copies/CFG, claim-before-append, exact primitive
  lanes, Normal-only out loads and complete release before normal/Fault terminal.
  Fault merges compare live identities; claim capability is unusable there.
  LLVM emission preserves Float bits, narrows proved Bool only at its i32 ABI,
  and disposes Unit roots before exit0. Source/Recipe still owns cleanup order.
  Rust's real OBJ/EXE callers bind the explicit runtime session and admit exact
  retained Script carrier/claim coverage; generic and unselected Stops remain.
- Native consumer reproduction: after rebuilding C and the lifecycle archive,
  run the ignored Rust test `retained_script_inputs_reach_native_c_and_reject_physical_mutations`.
  It supplies real retained Script JSON to `published_native_array_physical_test.py`;
  the V4 driver checks physical mutations while the runtime probe links actual
  production host objects and observes mutation/release/report/dispose calls.
  It covers final-result range Fault after cleanup and test-injected returned
  allocation Fault at the first/second allocation, including skipped successors.
  Injection proves generated Fault control, not recovery from fatal OOM. Physical
  mutations grant no source acceptance; keep Pair and untyped regressions.
- Birth validates all kind/payload pairs, including unused arguments. Integer
  reaches the raw i64 store; valid Bool records FieldTypeMismatch (103) at the
  existing FieldSet site and follows its Fault edge. Invalid kind/payload is
  InvalidContract. Tagged Copy preserves both lanes; HANDLE is never scalar.
- `process_result_site` remains distinct from checked-operation sites; out-of-
  range I64 returns record reason102 after Home cleanup. Runtime frame/descriptor
  revisions remain unchanged. The selected session host has source-backed
  EXE/independent OBJ acceptance; generic session-less OBJ stays rejected.
- Physical field storage uses `HAKO_LLVMC_LIFECYCLE_STORAGE_I64=1` in the
  shared header, parser and V4 admission, mirrored by Rust physical input.
  This tag does not classify source values; unknown storage tags still reject.
- Focused reproduction: build with `bash tools/build_hako_llvmc_ffi.sh`; run the
  existing physical parser preartifact C test and `published_lifecycle_v4_execution_test.py`
  with the three JSON paths captured by the Rust physical_program_json tests
  (`/tmp/hako-issued-physical-v2.json`, and `...-bool-0.json`, `...-bool-1.json`).
  The Python test links the actual lifecycle kernel; temporary LLVM mutation
  probes only test dynamic ABI rejection and grant no new source acceptance.
  `published_mir_object_tests.rs` separately links the same runtime probe against
  actual normal-source host objects. Its DISPOSE observation reports prior Home,
  reclaim and report calls, so totals alone are not mistaken for ordering proof.

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

The Rust view retains Call and LegacyCallV0 readers. Typed malformed rows reject;
published Global absence now rejects at shared peek before legacy classification.
All canonical Global targets (Print/FreeFunction/StaticBoxMethod) already have
Rust-issued rows; same-module prepass/emitter and entry dispatch share that check.
Published Extern now rejects with or without a row, matching Rust host rejection. Generic
Extern remains separate. Rowless runtime Method helpers retain explicit
compatibility; Call-carried Constructor/Math has no newly admitted canonical
issuer. Map literal construction/write remains an open intrinsic cutover in the
[collection owner](../../docs/development/current/main/design/collection-literal-construction-ssot.md#map-literal-selected-construction-design).
Its canonical set lacks the legacy-only plan consumed by C; allocation alone
does not prove populated Map execution. The constructor V4
execution/retirement receipt above supersedes the old lifecycle-pending review.

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
