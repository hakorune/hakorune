# Phase 163x: primitive and user-box fast path

- Status: Active
- 目的: primitive semantic builtin family と user-box field access を current implementation lane として進め、`.hako` surface を変えずに compiler/MIR 主導の typed fast path を広げる。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
  - `src/mir/storage_class.rs`
  - `src/mir/instruction.rs`
  - `src/llvm_py/instructions/binop.py`
  - `src/llvm_py/instructions/compare.py`

## Decision Now

- this is the current implementation lane
- `phase-137x` stays active as string guardrail / borrowed-corridor validation lane
- `field_decls` is the typed authority
- names-only `fields` stays as compatibility mirror only
- `sink` stays landed in the string lane; do not delete it here
- do not add new `.hako` syntax or widen `@rune`
- do not start with flattening

## Restart Handoff

- design owner:
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
- lifecycle/value parent design owner:
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
- post-primitive backlog design owner:
  - `docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md`
- sibling guardrail lane:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
- landed upstream facts:
  - typed `field_decls` now survive `.hako parser -> AST -> Stage1 Program JSON -> MIR metadata -> MIR JSON`
  - canonical MIR now has `FieldGet` / `FieldSet`
  - `FieldGet.declared_type` now seeds `value_types` and `StorageClass`
  - LLVM lowering now has the first typed primitive pilot for `IntegerBox` / `BoolBox` via `nyash.integer.get_h` / `nyash.bool.get_h`
  - local perf gate `kilo_micro_userbox_point_add` now exists in `benchmarks/` + the kilo micro ladder
  - local perf gate `kilo_micro_userbox_flag_toggle` now also exists in `benchmarks/` + the kilo micro ladder as the dedicated BoolBox proof
  - local perf gate `kilo_micro_userbox_counter_step` now also exists in `benchmarks/` + the kilo micro ladder as the measured known-receiver local-method proof
  - local perf gate `kilo_micro_userbox_point_sum` now also exists in `benchmarks/` + the kilo micro ladder as the second measured known-receiver local-method proof
  - LLVM `field_get` / `field_set` now take a typed IntegerBox path for known user-box `field_decls`
  - LLVM `field_get` now also takes a typed BoolBox path for known user-box `field_decls`
  - LLVM `field_set` now takes a typed BoolBox path only when the source stays on the bool-safe boundary (`BoolBox` handle or bool immediate)
  - compare/bool expressions now lower in value context on the `.hako` builder path, so the BoolBox micro loop shape is accepted structurally instead of via a `.hako` workaround
  - thin-entry inventory is now landed as a no-behavior-change MIR metadata lane:
    - known user-box field/method routes and enum local routes now emit `thin_entry_candidates`
    - verbose MIR and MIR JSON now surface the same inventory
    - `Program(JSON v0)` bridge now refreshes the inventory after callsite canonicalization
  - thin-entry selection pilot is now landed as a no-behavior-change manifest metadata lane:
    - `thin_entry_selections` now bind manifest rows on top of `thin_entry_candidates`
    - primitive user-box field routes now choose between `inline_scalar` thin entries and explicit `public_default` rows
    - known user-box methods and enum local routes now surface manifest-selected thin internal entries while current carriers remain public/compat where the backend has not switched yet
    - verbose MIR, MIR JSON, and `Program(JSON v0)` now surface the same selection results
    - product LLVM/Python user-box `field_get` / `field_set` now consult the selector first:
      - `user_box_field_{get,set}.inline_scalar` rows can keep the typed primitive helper path even when backend-side `field_decls` rediscovery is absent
      - `user_box_field_{get,set}.public_default` rows still keep the generic fallback path for the selected subject
    - first local user-box body proving slice is now landed on the product LLVM/Python consumer:
      - selected primitive user boxes now stay boxless through `newbox` / `field_get` / `field_set` when the birth block initializes every declared primitive field before first read
      - the selected local route is inferred from `field_decls` + `thin_entry_selections.inline_scalar`, without widening canonical MIR
      - `call` / `boxcall` / `ret` now materialize a compat runtime box only at the escape boundary for that selected local route
    - narrow actual-consumer parity is now landed for the current keeper pair:
      - thin-entry inventory now normalizes boxed primitive `declared_type` hints (`IntegerBox` / `BoolBox` / `FloatBox`) back to inline scalar classes for user-box field routes
      - `lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed.inc` now requires the same `user_box_field_{get,set}.inline_scalar` selector rows before the Point/Flag keeper seeds fire
      - latest WSL `3 runs + asm` reread on `ny-llvmc(boundary pure-first)` stays call-free:
        - point-add keeper seed now carries only the loop-visible `sum` lane plus the volatile accumulator anchor, matching the C-style bottom-tested induction loop
        - `kilo_micro_userbox_point_add`: `ny_aot_instr=8,456,727 / ny_aot_cycles=2,756,274 / ny_aot_ms=3`
        - `kilo_micro_userbox_flag_toggle`: `ny_aot_instr=16,457,454 / ny_aot_cycles=3,369,293 / ny_aot_ms=4`
      - three-lane measurement split is now landed:
        - `tools/perf/bench_micro_c_vs_aot_lanes.sh` reports `total CLI` / `startup baseline (ret0)` / `kernel-only`
        - point-add latest `1/3/10` reread: `ny_total_ms=3 / ny_startup_ms=3 / ny_kernel_ms=0.700`, with kernel cycles `c=2,025,422` vs `ny=2,046,604`
        - flag-toggle latest `1/3/10` reread: `ny_total_ms=4 / ny_startup_ms=3 / ny_kernel_ms=0.800`, with kernel cycles `c=4,053,730` vs `ny=2,837,417`
        - current read: keep codegen decisions on `kernel-only`; treat remaining total CLI delta as startup/runtime budget
      - minimal startup route is now landed:
        - `--emit-mir-json-minimal` skips using/prelude resolution and plugin init while keeping the small `.hako` parser normalizations used by the perf fixtures
        - use it for front-end startup checks; the existing three-lane AOT split stays the kernel/startup companion
    - portability-ci on `public-main` succeeded for commit `6b91896c0` (run `24211665863`), covering Windows check and macOS build (release)
  - sum placement/effect pilot inspection chain is now landed for the enum/sum local route:
    - `sum_placement_facts` record local-vs-objectization evidence on top of `thin_entry_selections`
    - `sum_placement_selections` distinguish selected local aggregate routes from compat/runtime fallback routes
    - `sum_placement_layouts` choose the LLVM-side payload lane (`tag_only` / `tag_i64_payload` / `tag_f64_payload` / `tag_handle_payload`) for selected local aggregate routes
    - verbose MIR, MIR JSON, and `Program(JSON v0)` now surface the same facts/selections/layouts chain
- fixed authority:
  - `field_decls` = source of truth for typed field declarations
  - `fields` = names-only compatibility mirror for old payloads and old runtime consumers
- primitive-family audit snapshot:
  - parser/current surface already accepts float/bool/null literals and typed field declarations; docs must stay aligned to that
  - current keeper set is `kilo_micro_userbox_point_add` + `kilo_micro_userbox_flag_toggle` + `kilo_micro_userbox_counter_step` + `kilo_micro_userbox_point_sum`
  - `Float` surface-close is now landed on the current compiler route:
    - Stage1 Program JSON v0 now lowers float literals, including unary-minus float literals
    - recent value-lowering now accepts float literals and preserves `MirType::Float` on float arithmetic results
  - `FloatBox` fast-path pilot is now landed on the LLVM/current keeper slice:
    - primitive-handle lowering now treats `FloatBox` handles as float-family numeric values
    - LLVM `field_get` now uses a typed `FloatBox` helper for known user-box `field_decls`
    - LLVM `field_set` now uses a typed `FloatBox` helper only when the source is float-safe (`FloatBox` handle or actual `f64`)
  - `Float` storage-class inventory is now promoted too:
    - `MirType::Float` and typed `FloatBox` field facts now classify as `InlineF64`
    - this is inventory-only for this wave; runtime behavior stays unchanged
  - `ArrayBox` narrow typed-slot pilot is now landed:
    - runtime authority stays in `ArrayBox`; `NyashValue::Array` is not the keeper lane
    - internal i64-specialized routes now birth/preserve a narrow `InlineI64` storage lane
    - existing generic any-store/append routes now also birth/preserve narrow `InlineBool` / `InlineF64` lanes for `BoolBox` / `FloatBox` payloads without widening the public ABI rows
    - `slot_load_hi` stays on the existing encoded-any contract, so float slots read back as `FloatBox` handles instead of forcing a new typed load row
    - boxed/string/mixed routes explicitly promote back to boxed storage before mutation
    - current keeper proof is focused ArrayBox/kernel tests + `phase21_5_perf_kilo_micro_machine_lane_contract_vm`
  - enum/sum MIR types and user-defined generics now have a backlog SSOT:
    - `enum` stays separate from `box`
    - user-facing `template` is rejected; generic surface stays on `<T>`
    - constructor target is `Type::Variant(...)`, while shorthand patterns stay limited to known-enum matches
  - enum parser / AST / Stage1 MVP is now landed:
    - `enum Name<T> { ... }` parses on the Rust surface
    - unit variants + single-payload tuple variants are inventoried in AST / Stage1 Program JSON
    - `Type::Variant(...)` now lowers to Stage1 `EnumCtor`
  - known-enum shorthand match / exhaustiveness is now landed on the same narrow lane:
    - `Some(v)` / `None` shorthand now resolves against known enum inventory
    - known-enum matches must name every variant explicitly
    - `_` does not satisfy known-enum exhaustiveness
    - guarded enum shorthand arms remain out of MVP
  - canonical enum MIR lowering is now landed on the same compiler-first lane:
    - MIR now has `VariantMake` / `VariantTag` / `VariantProject`
    - JSON v0 bridge lowers `EnumCtor` / `EnumMatch` into the dedicated variant op lane instead of object-field encoding
    - MIR JSON emit/parse now preserves the same enum ops for handoff/debug
  - VM / LLVM / fallback runtime support is now landed for the same MVP variant lane:
    - VM interpreter snapshots `enum_decls` and executes `VariantMake` / `VariantTag` / `VariantProject` through synthetic `__NyVariant_<Enum>` fallback `InstanceBox` values
    - LLVM/Py builder registers the same synthetic runtime boxes before `ny_main` and lowers enum ops through `nyash.instance.*_field_h`
    - concrete `Integer` / `Bool` / `Float` payload hints use typed helper lanes
    - LLVM now also recovers erased/generic payloads back to typed `Integer` / `Bool` / `Float` when `variant_make` can observe an actual payload fact locally
    - unknown/genuinely dynamic payloads still stay on boxed-handle fallback
    - malformed tag projections fail fast on both backends instead of silently projecting
    - product `ny-llvmc` ownership remains separate from this compat/harness slice
  - cleanup splits now landed on the runtime seam:
    - `sum_bridge` owns `__NyVariant_*`, `__variant_tag`, and `__variant_payload` bridge helpers
    - `object_field_store` owns interpreter object-field get/set/root-count access instead of raw `obj_fields`
  - narrow record variants are now landed on the same source / JSON v0 route:
    - declaration surface accepts `Ident { name: String }`
    - qualified construction accepts `Token::Ident { name: expr }`
    - known-enum shorthand match accepts `Ident { name } => ...`
    - record payloads lower through synthetic hidden payload boxes `__NyVariantPayload_<Enum>_<Variant>` while variant values themselves stay on the existing variant op lane
    - constructors / patterns must mention the declared field set exactly; multi-payload variants stay deferred
  - post-primitive follow-on queue:
    1. keep `lifecycle-typed-value-language-ssot.md` as the parent reading for boxless interior / boxed boundary work
    2. keep the aggregate/objectization audit pair as the current evidence base:
      - `docs/development/current/main/investigations/phase163x-aggregate-truth-audit-2026-04-09.md`
      - `docs/development/current/main/investigations/phase163x-early-objectization-audit-2026-04-09.md`
    3. recommended next cut = `sum placement/effect pilot`
      - first proving slice: `sum outer-box sinking`
      - the inspection chain (`thin_entry_selections` -> `sum_placement_facts` -> `sum_placement_selections` -> `sum_placement_layouts`) is now landed
      - LLVM now uses the landed selection/layout metadata to keep selected local non-escaping enums boxless through `variant_make` / `variant_tag` / `variant_project`
      - LLVM now materializes runtime `__NyVariant_*` compat boxes only at `return` / `call` / `boxcall` escape barriers for that selected local route
      - focused `ny-llvmc` proving slice is now landed:
        - `apps/tests/mir_shape_guard/sum_option_project_local_i64_min.prebuilt.mir.json` now stays green on the boundary `pure-first` owner lane without compat replay
        - `apps/tests/mir_shape_guard/sum_result_ok_local_i64_min.prebuilt.mir.json` now proves the same cutover without depending on `Option::Some` naming
        - `apps/tests/mir_shape_guard/sum_result_ok_project_copy_local_i64_min.prebuilt.mir.json` now proves the same cutover when `variant_project` reads through a single local `copy` alias
        - `apps/tests/mir_shape_guard/sum_result_ok_project_copy_local_f64_min.prebuilt.mir.json` now proves the same cutover when `variant_project` reads through a single local `copy` alias on the Float payload lane
        - `apps/tests/mir_shape_guard/sum_result_ok_project_copy_local_handle_min.prebuilt.mir.json` now proves the same cutover when `variant_project` reads through a single local `copy` alias on the handle payload lane
        - `apps/tests/mir_shape_guard/sum_result_ok_project_local_f64_min.prebuilt.mir.json` now proves the same cutover for `variant_project` on a Float payload lane
        - `apps/tests/mir_shape_guard/sum_result_ok_project_local_handle_min.prebuilt.mir.json` now proves the same cutover for `variant_project` on a handle payload lane
        - `apps/tests/mir_shape_guard/sum_result_ok_tag_only_local_min.prebuilt.mir.json` now proves the same cutover for a payload-less `variant_tag` keep-lane proof
        - `apps/tests/mir_shape_guard/sum_result_ok_tag_local_f64_min.prebuilt.mir.json` now proves the same cutover for `variant_tag` on a Float payload lane
        - `apps/tests/mir_shape_guard/sum_result_ok_tag_local_handle_min.prebuilt.mir.json` now proves the same cutover for `variant_tag` on a handle payload lane
        - `apps/tests/mir_shape_guard/sum_result_ok_tag_local_i64_min.prebuilt.mir.json` now proves the same cutover for `variant_tag` on a non-`Option` enum
        - `apps/tests/mir_shape_guard/sum_result_ok_tag_copy_local_i64_min.prebuilt.mir.json` now proves the same cutover when `variant_tag` reads through a single local `copy` alias
        - `tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_sum_metadata_keep_min.sh` now pins the same no-replay contract across the metadata-bearing direct/copy `variant_project`, payload-less `variant_tag`, Float-lane `variant_tag`, handle-lane `variant_tag`, direct `variant_tag`, and copied-`variant_tag` fixtures
      - next active substep is `ny-llvmc` parity wave; the Variant* inventory for this cut is now exhausted
      - separate-phase backlog, not part of `sum-thin-entry-cutover`:
        - `PhiMerge` / cross-block alias routes stay blocked by the current `sum_placement` `phi_merge` barrier and require a contract change before optimization
        - `call` / `boxcall` / `return` de-objectization stays blocked by the current escape-barrier contract and must not be mixed into this boundary pure-first cut
      - keep canonical `Variant*` unchanged and leave VM / JSON v0 compat fallback intact in this slice
      - keep the landed slice scoped, then fold it into the later generic placement/effect pass instead of growing a permanent enum-only branch family
    4. after that, run a separate `ny-llvmc` parity wave
      - proving slice is now landed:
        - product LLVM/Python lowering seeds `thin_entry_selections` into the resolver alongside the already-landed sum placement metadata
        - product LLVM/Python lowering now also keeps selected primitive user-box bodies boxless through `newbox` / `field_get` / `field_set` and materializes only at `call` / `boxcall` / `ret`
        - metadata-bearing Point local-i64, Flag local-bool, and PointF local-f64 user-box fixtures are now green on `tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_metadata_keep_min.sh` via boundary `pure-first` owner lane without compat replay, including the Point, Flag, and PointF single-copy alias routes
        - metadata-bearing sum smoke is green on `phase163x_boundary_sum_metadata_keep_min.sh` via boundary `pure-first` owner lane without compat replay
        - thin-entry inventory now classifies boxed primitive field hints as `inline_scalar`, and the current Point/Flag native-driver keeper seeds require those selector rows before firing
      - the current `ny-llvmc` parity-wave keeper slice now covers Point / Flag / PointF direct+single-copy local keep routes
      - generic native-driver / `ny-llvmc` parity for the broader user-box local-body route remains later actual-consumer backlog, not the current blocker
      - sibling string retained-view exact-micro consumer expansion is now landed:
        - boundary `pure-first` recognizes the current `kilo_micro_substring_views_only` exit-len shape and collapses it before `substring_hii` / `len_h` replay
        - latest exact reread: `instr=465,637 / cycles=704,757 / cache-miss=8,280 / AOT 3 ms`
        - latest microasm: `ny_main = mov $0x20, %eax ; ret`
      - sibling string mixed accept guardrail is now landed too:
        - `string_corridor_sink` now reruns once after DCE so complementary `substring_len_hii` pairs can fuse after dead borrowed-string temps are removed
        - `kilo_micro_substring_only` now emits no `substring_len_hii` / `substring_hii`
        - latest exact reread: `instr=1,669,909 / cycles=1,061,204 / cache-miss=8,516 / AOT 3 ms`
        - latest microasm: `ny_main` now keeps only the preloop source-length read and the loop body is scalar `add %rax,%rcx`
      - sibling string single-use retained-slice length consumer expansion is now landed too:
        - `string_corridor_sink` now rewrites retained-slice `length()` / `len()` consumers into `substring_len_hii` even when the slice producer lives in a dominating block and is reached only through local copy aliases
        - `kilo_micro_len_substring_views` now compiles without loop `RuntimeDataBox.length` / `substring_len_hii` consumers
        - latest exact reread: `instr=1,672,259 / cycles=1,022,005 / cache-miss=10,525 / AOT 3 ms`
        - latest split-pack reread now keeps `kilo_micro_substring_only`, `kilo_micro_substring_views_only`, and `kilo_micro_len_substring_views` in the same 3 ms band
      - immediate next task after the parity keeper:
        - move from the landed sibling exact micros into the broader string corridor placement/effect rewrite
        - `phase-169x` landed the merged-header `stable_length_scalar` witness on the same exact front, so the live post-sink loop body no longer keeps `substring_len_hii` on `kilo_micro_substring_concat`
        - `phase-170x` landed the next bridge shrink on the same lane too: boundary `pure-first` helper-result `substring()` now reads concat-triplet piece carriers from `direct_kernel_entry.plan.proof` instead of remembered concat-chain state
        - `phase-171x` is landed as the pure-first exact seed bottom-tested loop-shape cut
        - `phase-172x` is now landed on that sibling lane: it consumes the landed `%21 stable_length_scalar -> %5` witness through the header string-lane phi and switches the current benchmark to the existing length-only exact seed route
        - fresh reread now keeps `kilo_micro_substring_concat` (`instr=1,666,187 / cycles=1,049,205 / cache-miss=8,799 / AOT 4 ms`) as the best exact front for broader corridor placement/effect work
        - `phase-173x` is now landed on that sibling lane: same-block direct-helper `return` publication sink now rides the existing `publication_sink` plan metadata under a focused unit guard
        - `phase-174x` is now landed on that sibling lane too: same-block canonical `Store { value, .. }` / `FieldSet { value, .. }` write boundaries now ride the same `publication_sink` plan metadata under a focused unit guard
        - `phase-175x` is now landed on that sibling lane too: same-block `RuntimeDataBox.set(...)` now rides the same `publication_sink` plan metadata as the first host-boundary publication slice under a focused unit guard
        - remaining sibling string backlog is now only the final emitted-MIR return-carrier cleanup
      - verified non-Variant optimization order after this parity wave:
        1. broader string corridor placement/effect rewrite
           - `src/mir/string_corridor_placement.rs` is still inspection-only in this wave
           - use the already-landed candidate vocabulary as the contract for the next real transform family:
             - `borrowed_corridor_fusion`
             - `publication_sink`
             - `materialization_sink`
             - `direct_kernel_entry`
        2. actual-consumer switch for selected user-box thin entries that are still metadata-only today
           - `thin_entry_selection` already inventories `user_box_method.known_receiver`
           - keep this beneath canonical `Call`; do not widen surface syntax or add a public MIR dialect fork
           - landed first LLVM/Python consumer slice:
             - `mir_call.method_call` now checks `user_box_method.known_receiver` selector rows before the legacy direct known-box fallback
             - when the selector says `thin_internal_entry`, lowering takes a dedicated thin-known-receiver direct route while keeping the old direct known-box call as compatibility fallback
           - landed canonical callsite rewrite too:
             - `callsite_canonicalize` rewrites known user-box receiver calls from `RuntimeDataBox`/union and `Global <Box>.<method>/<arity>` into canonical known `Call(Method{box_name=<Box>, certainty=Known, box_kind=UserDefined})`
             - `phase163x_direct_emit_user_box_counter_step_contract.sh` now pins the current direct-route `Counter.step` contract on `bench_kilo_micro_userbox_counter_step.hako`
           - landed first native-driver/shim boundary pure-first consumer slice too:
             - `phase163x_boundary_user_box_method_known_receiver_min.sh` now pins metadata-bearing `Counter.step`, `Counter.step_chain`, and `Point.sum` fixtures on the owner lane without compat replay
             - the current shim consumer stays local-i64 + known-receiver narrow and consumes `user_box_method.known_receiver` together with the matching scalar field selections, including one local receiver-copy alias and the one-hop recursive delegate
           - first measured local-method keeper is now landed:
             - `bench_kilo_micro_userbox_counter_step.hako` + `benchmarks/c/bench_kilo_micro_userbox_counter_step.c`
             - the narrow `Counter.step` pure-first micro seed now collapses the exact bench to `ny_main = mov $0x52041ab, %eax ; ret`
             - latest exact reread: `kilo_micro_userbox_counter_step` = `c_instr=127,242 / c_cycles=208,224 / c_ms=3` vs `ny_aot_instr=465,881 / ny_aot_cycles=794,663 / ny_aot_ms=3`
           - second measured local-method keeper is now landed:
             - `bench_kilo_micro_userbox_point_sum.hako` + `benchmarks/c/bench_kilo_micro_userbox_point_sum.c`
             - `phase163x_direct_emit_user_box_point_sum_contract.sh` now pins the current direct-route `Point.sum` contract on the known-receiver lane
             - the narrow `Point.sum` pure-first micro seed now collapses the exact bench to `ny_main = mov $0x5b8d83, %eax ; ret`
             - latest exact reread: `kilo_micro_userbox_point_sum` = `c_instr=127,235 / c_cycles=216,542 / c_ms=3` vs `ny_aot_instr=465,837 / ny_aot_cycles=1,127,654 / ny_aot_ms=3`
           - recursive one-hop delegate keeper is now landed:
             - `benchmarks/bench_kilo_micro_userbox_counter_step_chain.hako` + `benchmarks/c/bench_kilo_micro_userbox_counter_step_chain.c`
             - `phase163x_direct_emit_user_box_counter_step_chain_contract.sh` now pins the current direct-route `Counter.step_chain` contract on the known-receiver lane
             - the narrow `Counter.step_chain` pure-first micro seed remains the intended exact route, but current exact build/asm has a separate backend-seed stop-line and must not be treated as proven by the direct-route repair alone
           - direct-route determinism repair is now landed too:
             - `phase-167x` routes instance methods through the shared `finalize_function()` owner, seeds receiver `Box(...)` metadata at the parameter boundary, and keeps deterministic lexical member traversal as supporting structure
             - `Counter.step_chain` direct lowering now stays on canonical known-receiver `Method` shape in repeated release direct probes (`6/6`)
             - the separate pure-first/backend exact stop-line is now closed by `phase-168x`, which refreshed the stale seed/smoke forwarding expectation to the current narrow body
           - exact-route refresh follow-on is now active:
             - `phase-168x` refreshed the stale seed/smoke expectation that `Counter.step_chain/0` still forwarded through two receiver copies
             - exact build/asm is green again and the current `ny_main` snippet stays `mov $0x2b, %eax ; ret`
           - first broader boundary parity widening is now landed:
             - `apps/tests/mir_shape_guard/user_box_point_sum_local_i64_min.prebuilt.mir.json` now proves the direct local-i64 `Point.sum` known-receiver shape without relying on the benchmark loop body
             - `phase163x_boundary_user_box_method_known_receiver_min.sh` now keeps both known-receiver fixtures green on boundary `pure-first` without compat replay
           - single-copy receiver alias widening is now landed too:
             - `apps/tests/mir_shape_guard/user_box_counter_step_copy_local_i64_min.prebuilt.mir.json`
             - `apps/tests/mir_shape_guard/user_box_point_sum_copy_local_i64_min.prebuilt.mir.json`
             - `phase163x_boundary_user_box_method_known_receiver_min.sh` now keeps the same known-receiver contract green when the receiver flows through one local `copy`
           - keep further local-method widening separate from `ArrayBox` read-side observer evidence
        4. `ArrayBox` typed-slot expansion beyond the landed `InlineI64` pilot
           - landed next narrow slices: `InlineBool` / `InlineF64` birth/preserve on existing `slot_store_hih` / `slot_append_hh` any routes
           - current stop-line: keep read-side on encoded-any `slot_load_hi`; do not add a new typed load row without measured observer evidence (`kilo_micro_array_getset` still does not justify it)
        5. backlog-only after the above:
           - stronger cross-block / partial DCE beyond current pure-instruction DCE
           - generic LLVM-side escape pass beyond the already-landed narrow local objectization-at-boundary route
           - current escape narrow slice is now alias-aware too: barrier elision follows `Copy` chains and one-input carry `phi` aliases for non-escaping local boxes, while multi-input `phi_merge` and broader generic escape analysis remain backlog
           - `phase-165x` landed the operand-role escape barrier vocabulary cut so escape widening no longer reuses the coarse `used_values()` surface
           - `phase-166x` is now landed as the structural follow-on:
             - first unify MIR semantic refresh ownership
             - generic `value_origin` ownership is now landed in `src/mir/value_origin.rs`
             - generic `phi_relation` ownership is now also landed in `src/mir/phi_query.rs`
             - helper/runtime-name semantic recovery is now quarantined in `src/mir/string_corridor_compat.rs`
             - the `boundary_fact` / lifecycle extraction decision is now explicit too: keep string-local lifecycle vocabulary separate from barrier-cause vocabularies until another real lifecycle consumer appears
           - `where` / enum methods / full monomorphization
        6. do not promote these into the current phase task order without a measured hotspot + SSOT first:
           - `MapBox` typed value slots
           - float niche tuning (`fast-math` / `FMA` / SIMD-style follow-ons)
           - closure/lambda optimization
    5. `tuple multi-payload` compat transport is now landed
      - parser/AST now accept tuple payload declarations while preserving tuple payload truth above canonical MIR
      - Stage1 lowers tuple ctors/matches through `__NyVariantPayload_<Enum>_<Variant>` hidden payload boxes with `_0`, `_1`, ... field slots
      - canonical `EnumCtor` / `EnumMatch` / `VariantMake` / `VariantProject` stay single-slot in the same wave
    6. `void/null` cleanup is now landed
      - tokenizer/parser accept both `null` and `void` literal surface, including literal-match arms
      - direct compat null checks treat `NullBox` and `VoidBox` as the same no-value family
      - reference EBNF now matches the executable surface for both literals
    7. pre-optimization cleanup/doc sync is now landed
      - LLVM/Python local-enum escape barriers now share one helper instead of repeating materialization wrappers in `call` / `boxcall` / `ret`
      - safe runtime nullish checks touched in this lane now converge on `NullBox::check_null()`
      - MIR reference docs now split into instruction SSOT + metadata SSOT, while stale all-in-one references are reduced to thin pointers
    8. next ready task: `phase163x-optimization-resume`
    9. keep `where` / enum methods / full monomorphization in backlog

## Fixed Task Order

1. keep `field_decls` as authority and stop treating names-only `fields` as design truth
2. add the user-box local micro before wider typed lowering
3. pilot typed user-box field access on `declared_type = IntegerBox` first; only then allow `BoolBox` on a bool-safe internal slice
4. keep plugin / reflection / ABI / weak-field paths on generic fallback
5. do not reopen flattening until typed user-box access has a keeper

## Guardrails

- `tools/perf/build_perf_release.sh` stays mandatory before perf/asm probes
- string split pack remains guardrail:
  - `kilo_micro_substring_only`
  - `kilo_micro_substring_views_only`
  - `kilo_micro_len_substring_views`
- any typed user-box slice must not silently redefine string lane ownership or restart order
