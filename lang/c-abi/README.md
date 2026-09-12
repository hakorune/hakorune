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

Pure-first invocation ownership
- `compile_json_compat_pure` transfers its parsed document into the private
  `HakoLlvmcInvocation`, alongside captured config, Named outcomes, program view
  and definition plan. The wrapper and direct-core config test initialize it at
  its final address; do not copy it because outcomes borrows its config.
  Core borrows these products, without independent program/plan/config copies.
  Program and plan readers stay at their original positions; mutable function
  cursors remain local. No readiness or public query state is implied.
- One invocation destroy releases outcomes before the document after core return.
  Validator, pinned census, route readers and generic
  emitters borrow that document; they do not free it. The standalone file
  validator keeps its public ABI and uses the same private validator.
- Validation order and late definition-plan reading are preserved, including
  the earlier indexof pattern return. Non-document cleanup is unchanged.
  Downstream legacy exact-seed/replay file readers and other ABI entrypoints
  are outside this parse-once boundary; selected typed rows reject those routes.

Pure-first ingress profiles
- The selected `hako_llvmc_compile_json_pure_first` entry and typed Static V2
  invocation stamp the existing invocation with a strict physical profile.
  Before the shared lowering walk, that profile rejects an instruction whose
  top-level `op` is the legacy `call` form with the named terminal
  `[freeze:contract][pure-first/legacy-op-call]`. The check reads only the
  parsed `functions[*].blocks[*].instructions[*].op` fields; it does not scan
  raw JSON text or metadata/nested values containing `call`.
- The public `hako_llvmc_compile_json` export remains the generic compatibility
  profile. It retains the existing legacy reader and terminal for external
  callers; this is not a selected-product caller-zero or retirement claim.
- The focused physical proof is part of
  `tests/published_rows_preartifact_test.c`. After the selected C build,
  compile/run it with the yyjson implementation. The proof covers a strict
  legacy-call rejection with no object, a structured-callee
  `Global/print` generic compatibility success with `rc == 0` and an object,
  and a nested metadata `call` string that is accepted by the strict profile.
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
  consumer for the generic and same-module emitters. Document root, walker and array-store
  choice are explicit inputs; typed-plan reads remain lazy after builtin/alias
  precedence. Materialization, register facts and diagnostics stay in emitters.
- `typed_object_root_lookup.inc` is the shared explicit-root lookup for this
  selector and existing layout readers. First matching plan wins; there is no
  query-specific lookup or nested root capture. Compile/run
  `tests/typed_object_root_lookup_test.c` with yyjson for duplicate-plan and
  independent-document evidence. The selector test retains mocked lazy-read
  counters; actual emitter comparison remains Named60.
- Conditional selection states what an actual walker would consume. It does
  not prove execution, admission or a bypass. Retained observations are
  separate from required frame-row coverage; optimizer matchers are not copied
  into this lookup layer.
- `named_allocation_outcomes.inc` scans the owned document once and retains both
  walker conditions by function/instruction identity. Both actual emitters use
  indexed immutable lookup; neither reselects or marks frame rows used. All
  target-member instructions remain with the intrinsic consumer, before lookup.
  Invalid/unsupported outcomes and incomplete storage fail only on actual demand.
  The wrapper frees this storage before the document, including early returns.
- Compile/run `tests/named_allocation_outcomes_test.c` with yyjson for identity,
  repeated reads, target exclusion and deferred allocation failure. The ASan
  document driver also checks table destruction before document release. Its
  eight injected OOM cases cover first/growth failure, earlier schema/pattern
  outcomes, and both actual Named dispatchers; an optional Dynamic input brings
  the total to16 Named/document cases. Six empty-V2 call-activity cases bring
  the suite to22. Same-module dispatch is boolean: lookup failure returns0.
- Private `hako_llvmc_ffi_named_query.inc` projects unique function/block/ordinal
  coordinates from the same invocation. Program observation is cached once;
  unavailable program/storage and unaddressable/unobserved sites remain distinct.
  Query neither activates typed rows nor reruns allocation selection. The retained
  compile entry uses shared V2 rows begin/finish/end. The static host owns the
  complete open/query/frame/compile/close lifetime; public V1 is retired.
- `tests/named_query_driver.c` is a temporary live subprocess bridge to the actual
  Rust `MapBodyIndex` planner. Build like the document-lifetime driver, adding
  `-finstrument-functions -fsanitize=address -fno-omit-frame-pointer -g`.
  Run `tests/named_query_test.py DRIVER [DYNAMIC_JSON]` for coordinate/error cases.
  Set `HAKO_NAMED_QUERY_TEST_DRIVER=DRIVER` and run the ignored Rust library test
  `map_literal_actual_c_query` with the quick profile and `RUST_MIN_STACK=16777216`. The driver retains its
  document across Rust planning; non-Map input compiles after its file is removed,
  while Map/planner rejection cancels. Retire this driver/protocol when public
  V2 host tests cover the connection. These are dependency witnesses, not source
  admission or Map object/executable evidence.
- Published-call activity is explicit in the existing row owner. V1 requires
  nonempty rows; private V2 allows NULL/0 without enabling legacy Global/Extern,
  exact-seed routes or failure retry. `published_rows_preartifact_test.c` covers
  empty/invalid pairs and failed nested activation preserving the ledger.
  These are call-activity tests, not a complete V2 frame/Map consumer. The storage
  remains global and does not provide concurrent compilation safety.
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

The selected static V2 value projection separates result kind from finite
physical operation selection (integer/Bool/String comparison, String concat,
integer binary and integer/Bool Not). Body opcode and operands remain the sole
instruction graph. Retained V2 compilation binds an invocation-owned
index and publishes staged output only after both ledgers finish. Both walkers
consume ExactBits, selected integer/Bool/String operations, host-backed allocation,
canonical Static/Free Call results and boxed Tag/Integer/Bool/String projections.
Integer comparison bypasses dynamic kind dispatch; Bool payload normalization
preserves existing i1 producers. String concat uses proved payloads directly.
Operation input closure checks demanded rows against the retained body. Direct
array/typed-store tokens cannot escape as host handles; boxed Make/general handle
escape and selected boxed Unit comparison remain explicit unsupported. The static host and raw/Core literal issuers now use V2. Callable generic/checked
consumer selection and natural mixed-formal acceptance remain open. Both typed Static/Free
callers share `emit_published_i64_call` for argument output and result consumption;
one invocation-owned expanded index now supplies that owner and the existing
signature owner. Demanded Formal parameters receive kind/payload and only when
needed an original i64 lane. Exact incoming calls participate in the same demand
and domain closure, including recursion and mixed caller kinds. Expanded targets
use internal linkage and bypass old numeric-leaf emission; original definition
plan membership is retained. First-match old-ABI ingress, internal/entry symbol
collisions and root Formal expansion reject before artifact publication.

Copy/NamedAlias forward both Map lanes through the shared formatter. Map-only
transfers omit the old producer; original-required transfers retain it and consume
the row after success. NamedAlias requires the retained walker outcome, never a
StringBox name exception. Missing/cyclic source rows and original-demand gaps
reject. CopyOwned remains explicit unsupported.

Phi/Select emit kind and payload lanes in the existing PHI/condition owners.
Seeded loops are supported; unresolved inputs and incompatible whole operand
domains reject (String remains distinct from other Handle). PHI overflow rejects
before truncation. Shared mapped/nonmapped i64 PHIs consume stable original
references issued by successful flags1 producers, including original-only Copy
chains. Original values/types stay unchanged; producer-side add/zext handles
boxed aliases and future Select widths without recovering originals from Map
payloads. Compatible i1 PHIs retain their existing path; pending aliases/widths
reject for mapped and nonmapped destinations. Each walker normalizes emitted
selected i1 originals after the complete PHI group; shared i64 inputs use those
fixed references, including backedges. Original i1 values remain available.
Select shares one normalized condition with both Map lanes. No normalization
enters a PHI group.

String constants keep byte length in their existing record/global/owned storage;
V2 materializes length-aware handles at the instruction and traps on zero.
Map write checks i32 status0 and traps otherwise. String/Map and boxed continuations use the existing
checked status tail/PHI owner, including nondemanded boxed sites and backedges. V1 retains its prior String projection.

For the private physical execution proof, build `tests/static_v2_execution_driver.c`
with the same whole-C/yyjson sources as the document driver and ASan. Run
`python3 lang/c-abi/tests/static_v2_execution_test.py DRIVER KERNEL_ARCHIVE`.
The suite links `static_v2_runtime_probe.c` against the current kernel for actual
readback and injects returned statuses to verify traps. Its JSON frame reader is
only a fixture, not a new production transport or source acceptance proof.
- The [v1 runtime contract](../../docs/reference/abi/nyrt_c_abi_v0.md#selected-map-literal-store-v1)
  fixes explicit value kinds, OK/InvalidContract and length-aware String input.
  Kernel exports and the bounded private value consumers are tested; the complete
  compiler/host/source cutover remains pending.
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
- Indexed V4 admission and emission share one invocation-owned function/block/
  value/layout index. The earlier fixed Pair function/layout counts are removed;
  supplied Birth targets and exact object layouts select physical calls. Map,
  prepared key and detached outcome use descriptor-sized opaque regions with
  exact-origin consumption checks, including Fault joins and complete cleanup.
  Live allocation never issues source Home or NoBirth authority. Parser SSA and
  dominance checks remain separate; this is not a claim that all scans are gone.
  The Map physical test command and source-cutover limits are in
  [the shim README](shims/README.md). Indexed Unit roots remain unsupported.
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
- Receiver identity R0 reproduction: after rebuilding the C shim, compile
  `lang/c-abi/tests/published_lifecycle_v4_receiver_identity_test.c` against
  `target/release/libhako_llvmc_ffi.so`. The fixture first accepts the same-typed
  receiver with valid normal/fault cleanup, then mutates only `receiver_object`
  and requires the named `published-lifecycle-v4/receiver-object-mismatch`
  rejection with no artifact. This proves V4 test discrimination; it does not
  add a second identity verifier or an OBJ/EXE claim.
  The Map physical proof reuses its valid normal/fault graph, mutates only the
  layout `fields` member, and asserts the parser-owned named
  `published-lifecycle-physical-parser/abi-layout` rejection before artifact
  publication.
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

The first bounded sub-slice of Task 3 is now selected as
`MIR-CALL-LINK-DIRECT-SEAM-I0`: the two FFI link wrappers no longer need to
mutate `HAKO_AOT_USE_FFI`; they call the existing single AOT link body via one
hidden invocation-mode seam. This closes only link-route state. Compile
recipe/replay/opt-level ownership remains a separate I1 design dependency,
and public v1/v2 AOT compatibility remains retained.

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

Static V2 host verification uses `tests/static_v2_session_test.py LIBRARY`,
which runs the explicit Static V2 contract lifecycle with fake tool propagation
and skips only the legacy object portion when LLVM18 is unavailable. It also
uses the migrated Named/row/document/query proofs above, and the ignored Rust
`static_map_source_v2_direct_and_linked_objects` test. Build the quick kernel
archive for that test; existing Array source regression uses the release archive.
Both must include `nyash.box.from_i8_string_const_len_v1`. Old archives are not a
reason to downgrade String emission. The Script test owns a32MiB test thread.
The private Rust/query bridge remains for its cancellation and counter boundary;
its Map-only fixture is not an executable root-result contract.
