# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-12
Scope: repo root から current lane / current front / restart read order に最短で戻るための薄い pointer。

## Purpose

- root から active lane/front を最短で読む
- landed history / rejected perf evidence は phase docs と investigations を正本にする
- `CURRENT_TASK.md` は pointer に徹し、ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-163x/README.md`
4. `git status -sb`
5. `tools/checks/dev_gate.sh quick`

## Restart Handoff

- current expected worktree on reopen:
  - clean
  - do not mix the parked `phase-96x` monitor-policy backlog with the current `phase-163x` optimization lane
- runtime-wide pattern anchor:
  - `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
- current implementation lane:
  - `phase-163x primitive and user-box fast path`
- current primitive/user-box design anchor:
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
- lifecycle/value architecture anchor:
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
- post-primitive enum/generic design anchor:
  - `docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md`
- sibling string guardrail anchor:
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
- current implementation phase:
  - `docs/development/current/main/phases/phase-163x/README.md`
  - optimization snapshot: see `docs/development/current/main/10-Now.md` for the row-by-row done / partial / backlog split; `phase137x` stays active as the sibling guardrail lane, and `phase165x` / `phase166x` are landed structural cuts inside `phase163x-optimization-resume`
- active escape barrier sub-corridor:
  - `docs/development/current/main/phases/phase-165x/README.md`
  - phase-165x is landed; operand-role escape classification now stays in MIR authority and remains separate from both `used_values()` and backend/runtime helper logic
- landed semantic-structure follow-on:
  - `docs/development/current/main/phases/phase-166x/README.md`
  - phase-166x is landed; semantic refresh ownership, generic `value_origin` / `phi_relation`, compat semantic recovery quarantine, and the `boundary/lifecycle extraction` defer decision are now fixed
- landed direct-route determinism repair:
  - `docs/development/current/main/phases/phase-167x/README.md`
  - phase-167x is landed; instance methods now seal through the shared finalize owner and seed receiver `Box(...)` metadata, so direct `Counter.step_chain` known-receiver shape no longer drifts on missing method-local facts
  - release direct emit repeat is green again (`Counter.step_chain` known-receiver shape `6/6`)
  - pure-first AOT build/asm still stops separately on backend seed matching and is not part of `phase-167x`
- landed exact-route follow-on:
  - `docs/development/current/main/phases/phase-168x/README.md`
  - `phase-168x` is landed; the stale pure-first/backend exact contract for `Counter.step_chain` now matches the current narrow forwarding body again
  - direct contract smoke, boundary owner-lane smoke, exact asm, and exact perf are green again; `ny_main` is back to `mov $0x2b, %eax ; ret`
- landed string phi-length follow-on:
  - `docs/development/current/main/phases/phase-169x/README.md`
  - `phase-169x` is landed; merged header `%21` on `kilo_micro_substring_concat` keeps the `stop_at_merge` plan-window contract and now also carries a narrow `stable_length_scalar` witness
  - the live post-sink loop body now collapses the complementary `substring_len_hii + const + substring_len_hii` path into `source_len + const`, with `interesting_n = 14`
  - direct/post-sink smoke, phi-merge contract smoke, daily owner smoke, exact asm, exact perf, and `tools/checks/dev_gate.sh quick` are green
- portability-ci validation:
  - workflow `portability-ci` on `public-main` completed success for commit `6b91896c0`
  - Windows check and macOS build (release) both passed in run `24211665863`
- sibling string guardrail phase:
  - `docs/development/current/main/phases/phase-137x/README.md`
- parked vm retirement corridor:
  - `docs/development/current/main/phases/phase-96x/README.md`
  - `phase-96x` is no longer an active owner lane; it is parked after cutting `vm_hako` down to the frozen `vm-hako-core` 4-row monitor pack
  - the only remaining backlog item is `96xE5` monitor-policy decision for that frozen pack
- repo-wide formatting maintenance corridor:
  - `docs/development/current/main/phases/phase-164x/README.md`
  - phase-164x is landed; repo-wide `cargo fmt --check` drift is cleaned up and remains separate from the `phase-163x` optimization lane
- landed inventory scaffold:
  - `src/mir/storage_class.rs`
  - `StorageClass` facts are now refreshed after corridor facts and surfaced in verbose MIR / JSON dumps
- landed typed field scaffold:
  - `.hako` parser / AST / Stage1 Program JSON / MIR metadata / MIR JSON now preserve typed `field_decls`
  - canonical MIR now has first-class `FieldGet` / `FieldSet`
  - MIR interpreter and LLVM/PyVM compatibility paths accept the canonical field ops without changing current field semantics
- landed declared-field storage bridge:
  - `FieldGet` now seeds `value_types` from declared field type on the `.hako` builder path
  - type propagation and storage-class refresh also recognize `FieldGet.declared_type` as a fallback seed
  - current typed field path is still behavior-preserving for generic field semantics
- landed typed primitive pilot:
  - LLVM lowering now treats `IntegerBox` / `BoolBox` handle facts as primitive numeric sources in `binop` / `compare`
  - numeric paths call `nyash.integer.get_h` / `nyash.bool.get_h` before integer arithmetic or integer compare
  - current pilot is still narrow: primitive handle unbox only
- landed typed user-box integer pilot:
  - local perf gate `kilo_micro_userbox_point_add` now exists in `benchmarks/` + kilo micro ladder
  - LLVM `field_get` / `field_set` now select a typed IntegerBox internal path for known user-box `field_decls`
  - weak fields and non-user-box paths stay on the generic fallback
- landed typed user-box bool pilot:
  - LLVM `field_get` now selects a typed BoolBox internal path for known user-box `field_decls`
  - LLVM `field_set` now selects a typed BoolBox internal path only when the source is bool-safe (`BoolBox` handle or bool immediate)
  - ambiguous/non-boolish set sources stay on the generic fallback
- landed BoolBox perf proof:
  - compare/bool expressions now lower in value context on the `.hako` builder path, so loop bodies like `acc + (f.enabled == true)` and `f.enabled = i < flip_at` are accepted structurally
  - `kilo_micro_userbox_flag_toggle` now exists in `benchmarks/` + kilo micro ladder as the dedicated BoolBox local perf proof
  - pure-first boundary seed now matches the narrow Flag/BoolBox toggle micro
- primitive-family audit snapshot:
  - parser surface already accepts `Float` / `Bool` / `Null` literals and typed `field_decls`; docs must stay synced to that current surface
  - current recent MIR keeper is solid for `IntegerBox` / bool-safe `BoolBox`, and the `Float` route now has both the surface-close and the first narrow fast path:
    - Stage1 Program JSON v0 now lowers float literals, including unary-minus float literals
    - recent value lowering now accepts float literals and keeps float arithmetic results typed as `MirType::Float` on the same keeper path
    - LLVM primitive-handle lowering now recognizes `FloatBox` handles as the float family
    - LLVM `field_get` now takes a typed `FloatBox` path for known user-box `field_decls`
    - LLVM `field_set` now takes a typed `FloatBox` path only when the source stays on the float-safe boundary (`FloatBox` handle or actual `f64`)
    - `MirType::Float` / typed `FloatBox` field facts now classify as `InlineF64` in MIR storage-class inventory
  - `ArrayBox` narrow typed-slot pilot is now landed:
    - runtime authority stays in `ArrayBox`; `NyashValue::Array` is not the keeper lane
    - internal i64-specialized array routes now birth/preserve a narrow `InlineI64` storage lane
    - existing generic any-store/append routes now also birth/preserve narrow `InlineBool` and `InlineF64` lanes for `BoolBox` / `FloatBox` payloads without widening the public ABI symbol surface
    - `slot_load_hi` stays on the existing encoded-any contract; float slots materialize as `FloatBox` handles on read instead of forcing a new row
    - boxed/string/mixed routes explicitly promote back to boxed storage before mutation
    - focused ArrayBox/kernel tests and `phase21_5_perf_kilo_micro_machine_lane_contract_vm` are green on this slice
  - `Null` / `Void` fast paths remain backlog and are outside the current keeper wave
  - enum/sum + generic backlog is now inventoried in a dedicated SSOT:
    - `enum` stays first-class and separate from `box`
    - user-facing `template` is rejected; generic surface stays on `<T>`
    - MVP target is `Type::Variant(...)` + shorthand patterns only for known-enum matches
  - enum parser / AST / Stage1 surface is now landed:
    - `enum Name<T> { ... }` parses as first-class surface
    - Stage1 Program JSON now inventories `enum_decls`
    - `Type::Variant(...)` now lowers to Stage1 `EnumCtor`
  - known-enum shorthand match is now landed on the parser / AST / Stage1 lane:
    - `match x { Some(v) => ... None => ... }` now resolves against known enum inventory
    - known-enum matches now fail fast on missing variant arms
    - current shorthand lane stays narrow:
      - unit + single-payload variants only
      - no guarded enum shorthand arms yet
      - `_` does not satisfy known-enum exhaustiveness
    - canonical sum MIR lowering is now landed on the JSON v0 bridge:
      - `EnumCtor` now lowers to `SumMake`
      - `EnumMatch` now lowers to `SumTag` + compare/branch + `SumProject` + PHI
      - MIR JSON emit/parse now preserves the sum lane too
    - VM / LLVM / fallback runtime parity is now landed on the MVP sum lane:
      - VM interpreter executes `SumMake` / `SumTag` / `SumProject` via synthetic `__NySum_<Enum>` fallback boxes
      - LLVM/Py builder registers the same synthetic enum runtime boxes at entry and lowers sum ops through `nyash.instance.*_field_h`
      - malformed tag projections now fail fast (`[vm/sum:*]` on VM, `unreachable` on LLVM)
      - LLVM now recovers erased/generic payloads back to typed `Integer` / `Bool` / `Float` when `sum_make` can observe an actual payload fact locally
      - unknown/genuinely dynamic payloads still stay on boxed-handle fallback
      - product `ny-llvmc` ownership remains separate from this compat/harness slice
    - narrow record variants are now landed on the same source / JSON v0 route:
      - declaration surface accepts `Ident { name: String }`
      - qualified construction accepts `Token::Ident { name: expr }`
      - known-enum shorthand match accepts `Ident { name } => ...`
      - implementation uses synthetic hidden payload boxes `__NyEnumPayload_<Enum>_<Variant>` while the sum runtime box stays `__NySum_<Enum>`
      - constructors / patterns must mention the declared field set exactly; multi-payload variants remain deferred
- fixed typed field authority:
  - `field_decls` is the typed authority
  - names-only `fields` stays as a compatibility mirror for old payloads and old runtime consumers
- vm fallback separation anchor:
  - `docs/development/current/main/design/vm-fallback-lane-separation-ssot.md`
- active lane/front:
  - lane: `phase-163x primitive and user-box fast path`
  - lifecycle/value parent anchor:
    - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
  - design reading lock for this lane:
    - authority order stays `.hako owner / policy -> MIR canonical contract -> Rust executor / accelerator -> LLVM generic optimization / codegen`
    - `Birth / Placement` is outcome vocabulary / seam reading, not a fifth authority layer
    - keep one canonical `Call`; thin/public physical entry split stays below canonical MIR via pass + manifest
    - current runtime/public value truth is still `imm_i64 / imm_bool / handle_owned / handle_borrowed_string / boxed_local`
    - parent-design `imm / borrow / agg_local / handle` is architecture/end-state vocabulary; `owned_buf` remains backend-private / future-child vocabulary
    - thin-entry inventory and the first manifest-driven selection pilot are landed as inspection metadata; direct thin-entry lowering is still not the current default lowering truth
    - `null` / `void` are already language-surface aliases of runtime `Void`, but fast-path work for them is outside the current keeper wave
  - local gates:
    - `kilo_micro_userbox_point_add`
    - `kilo_micro_userbox_flag_toggle`
    - `kilo_micro_userbox_counter_step`
    - `kilo_micro_userbox_point_sum`
    - latest `2026-04-09` WSL `3 runs + asm` reread:
      - pre-cleanup baseline:
        - `kilo_micro_userbox_point_add`: `c_instr=12,120,416 / c_cycles=2,187,984 / c_ms=3` vs `ny_aot_instr=22,457,049 / ny_aot_cycles=4,461,297 / ny_aot_ms=4`
        - `kilo_micro_userbox_flag_toggle`: `c_instr=18,120,465 / c_cycles=4,203,879 / c_ms=4` vs `ny_aot_instr=20,457,266 / ny_aot_cycles=3,972,375 / ny_aot_ms=4`
      - landed micro-seed cleanup on the pure proof route:
        - `lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed.inc` now keeps only the benchmark-anchor accumulator volatile and leaves synthetic field slots `%x/%y/%enabled` promotable for `mem2reg`
      - post-cleanup reread:
        - `kilo_micro_userbox_point_add`: `c_instr=12,120,460 / c_cycles=2,197,103 / c_ms=3` vs `ny_aot_instr=12,457,600 / ny_aot_cycles=3,033,086 / ny_aot_ms=4`
        - `kilo_micro_userbox_flag_toggle`: `c_instr=18,120,421 / c_cycles=4,198,832 / c_ms=4` vs `ny_aot_instr=16,456,964 / ny_aot_cycles=3,545,279 / ny_aot_ms=4`
      - post-selector-normalization + actual-consumer keeper reread:
        - thin-entry inventory now normalizes boxed primitive field hints like `MirType::Box("IntegerBox")` / `MirType::Box("BoolBox")` back to inline scalar value classes for user-box field routes
        - `kilo_micro_userbox_point_add`: `c_instr=12,120,460 / c_cycles=2,210,315 / c_ms=4` vs `ny_aot_instr=12,456,831 / ny_aot_cycles=3,045,907 / ny_aot_ms=3`
        - `kilo_micro_userbox_flag_toggle`: `c_instr=18,120,465 / c_cycles=4,212,866 / c_ms=4` vs `ny_aot_instr=16,456,727 / ny_aot_cycles=3,348,861 / ny_aot_ms=3`
      - post-point-add loop-shape cut on the actual keeper seed:
        - `lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed.inc` now carries point-add through the loop-visible `sum` lane instead of separate `x/y/i` allocas, keeping only the volatile accumulator as the benchmark anchor
        - latest exact reread: `kilo_micro_userbox_point_add` = `c_instr=12,120,458 / c_cycles=2,187,858 / c_ms=3` vs `ny_aot_instr=8,456,727 / ny_aot_cycles=2,756,274 / ny_aot_ms=3`
        - latest exact reread: `kilo_micro_userbox_flag_toggle` = `c_instr=18,120,421 / c_cycles=4,188,196 / c_ms=4` vs `ny_aot_instr=16,457,454 / ny_aot_cycles=3,369,293 / ny_aot_ms=4`
      - current `ny_main` asm for both benches is still call-free in the hot loop:
        - point-add now matches the C-style bottom-tested `sum += 3` induction loop and keeps only the volatile accumulator store in the body
        - flag-toggle now keeps `enabled` in a register and only spills the volatile accumulator
      - current reading: box/helper cost is gone on this keeper pair, and point-add hot-loop shape is now close to the C loop; the remaining cycle delta is increasingly dominated by fixed process/runtime overhead outside `ny_main` on the actual AOT route
      - fixed-cost audit result:
        - `_dl_fini` is a harness/process-exit artifact from repeated `posix_spawn`/child execution in `tools/perf/bench_micro_aot_asm.sh`, not a codegen win
        - `trim_matches` is startup parsing overhead from runner/config quote stripping (`src/runner/mod.rs`, `src/runtime/plugin_config.rs`, `src/runner/modes/common_util/resolve/*`), not perf-report overhead
      - three-lane measurement split is now landed:
        - `tools/perf/bench_micro_c_vs_aot_lanes.sh` reports `total CLI` / `startup baseline (ret0)` / `kernel-only` for both C and `ny-llvmc`
        - latest `1/3/10` point-add reread: `ny_total_ms=3 / ny_startup_ms=3 / ny_kernel_ms=0.700`, with kernel cycles `c=2,025,422` vs `ny=2,046,604`
        - latest `1/3/10` flag-toggle reread: `ny_total_ms=4 / ny_startup_ms=3 / ny_kernel_ms=0.800`, with kernel cycles `c=4,053,730` vs `ny=2,837,417`
       - current reading: the keeper pair is now effectively a startup-vs-kernel split decision; codegen work should read `kernel-only`, while total CLI regressions should be treated as startup/runtime budget
       - minimal startup route is now landed:
         - `--emit-mir-json-minimal` skips using/prelude resolution and plugin init while keeping the small `.hako` parser normalizations used by the perf fixtures
         - use it for front-end startup checks; the existing three-lane AOT split stays the kernel/startup companion
       - first product LLVM/Python proving slice is now landed for that reusable family:
         - selected primitive user boxes stay boxless through `newbox` / `field_get` / `field_set` when the birth block initializes every declared primitive field before first read
         - the selected local route is inferred from `field_decls` + `thin_entry_selections.inline_scalar` and materializes only at `call` / `boxcall` / `ret`
        - this keeps canonical MIR unchanged
  - post-primitive follow-on queue:
    1. keep `lifecycle-typed-value-language-ssot.md` as the architecture parent for boxless interior / boxed boundary work
    2. keep the landed audit pair as the decision base:
       - `docs/development/current/main/investigations/phase163x-aggregate-truth-audit-2026-04-09.md`
       - `docs/development/current/main/investigations/phase163x-early-objectization-audit-2026-04-09.md`
    3. next fixed cut:
        - `sum placement/effect pilot`
        - the inspection chain is now landed for the `sum outer-box sinking` slice:
          - `thin_entry_selections`
          - `sum_placement_facts`
          - `sum_placement_selections`
          - `sum_placement_layouts`
        - LLVM now uses the landed selection/layout metadata to keep selected local non-escaping sums boxless through `sum_make` / `sum_tag` / `sum_project`
        - LLVM now materializes runtime `__NySum_*` compat boxes only at `return` / `call` / `boxcall` escape barriers for that selected local route
        - focused `ny-llvmc` proving slice is now landed:
          - `apps/tests/mir_shape_guard/sum_option_project_local_i64_min.prebuilt.mir.json` now stays green on the boundary `pure-first` owner lane without compat replay
          - `tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_sum_metadata_keep_min.sh` now pins the same no-replay contract
        - next active substep: start the separate `ny-llvmc` parity wave
        - keep canonical `SumMake` / `SumTag` / `SumProject` unchanged
        - keep VM / JSON v0 compat fallback unchanged in this slice
        - keep the landed slice scoped, then fold the shape into a later generic placement/effect pass instead of growing a permanent sum-only framework
    4. after that:
        - `ny-llvmc` parity wave
        - proving slice is now landed:
          - product LLVM/Python lowering seeds `thin_entry_selections` into the resolver alongside the already-landed sum placement metadata
          - product LLVM/Python user-box `field_get` / `field_set` now consult `thin_entry_selections` first for known selector subjects
          - selected `user_box_field_{get,set}.inline_scalar` rows can keep the typed primitive helpers without re-discovering `field_decls` on the backend side when the declared box family is already pinned
          - selected `user_box_field_{get,set}.public_default` rows still keep the generic fallback even if the compat mirror looks scalar-shaped
          - selected `user_box_method.known_receiver` rows now also act as an actual consumer on the LLVM/Python route: `mir_call.method_call` first tries a thin-known-receiver direct box-method call beneath canonical `Call`, while the previous direct known-box route stays as compatibility fallback
          - selected `user_box_method.known_receiver` rows now also have a broader native-driver/shim boundary consumer slice: `hako_llvmc_ffi_user_box_micro_seed.inc` accepts metadata-bearing `Counter.step`, `Counter.step_chain`, and `Point.sum` fixtures only when the selector row and the matching scalar field selections are present
          - canonical callsite rewrite is now landed too:
            - `callsite_canonicalize` rewrites known user-box receiver calls from `RuntimeDataBox`/union or `Global <Box>.<method>/<arity>` into canonical `Call(Method{box_name=<Box>, certainty=Known, box_kind=UserDefined})`
            - `phase163x_direct_emit_user_box_counter_step_contract.sh` pins the current direct-route `Counter.step` contract with two `user_box_method.known_receiver` rows and canonical callsites in `bench_kilo_micro_userbox_counter_step.hako`
          - product LLVM/Python now also keeps selected primitive user-box bodies boxless through `newbox` / `field_get` / `field_set` and materializes only at `call` / `boxcall` / `ret`
          - metadata-bearing sum smoke is green on `phase163x_boundary_sum_metadata_keep_min.sh` via boundary `pure-first` owner lane without compat replay
        - narrow actual-consumer parity is now also landed for the current keeper pair:
          - `thin_entry_candidates` now classify boxed primitive `declared_type` hints (`IntegerBox` / `BoolBox` / `FloatBox`) as inline scalar value classes instead of leaving them on the generic handle lane
          - `lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed.inc` now requires the same `user_box_field_{get,set}.inline_scalar` selector rows before the Point/Flag keeper seeds fire
          - focused `3 runs + asm` still shows call-free `ny_main` loops on `kilo_micro_userbox_point_add` and `kilo_micro_userbox_flag_toggle`
          - first measured local-method keeper is now landed:
            - `benchmarks/bench_kilo_micro_userbox_counter_step.hako` + `benchmarks/c/bench_kilo_micro_userbox_counter_step.c`
            - `hako_llvmc_ffi_user_box_micro_seed.inc` now has a narrow `Counter.step` pure-first micro seed behind the same `user_box_method.known_receiver` + `Counter.value` scalar selections
            - latest exact reread: `kilo_micro_userbox_counter_step` = `c_instr=127,242 / c_cycles=208,224 / c_ms=3` vs `ny_aot_instr=465,881 / ny_aot_cycles=794,663 / ny_aot_ms=3`
            - current `ny_main` object snippet is now `mov $0x52041ab, %eax ; ret`, so the remaining gap reads as startup/process cost rather than loop/codegen churn
          - second measured local-method keeper is now landed:
            - `benchmarks/bench_kilo_micro_userbox_point_sum.hako` + `benchmarks/c/bench_kilo_micro_userbox_point_sum.c`
            - `hako_llvmc_ffi_user_box_micro_seed.inc` now has a narrow `Point.sum` pure-first micro seed behind the same `user_box_method.known_receiver` + `Point.{x,y}` scalar field selections
            - `phase163x_direct_emit_user_box_point_sum_contract.sh` pins the current direct-route `Point.sum` contract
            - latest exact reread: `kilo_micro_userbox_point_sum` = `c_instr=127,235 / c_cycles=216,542 / c_ms=3` vs `ny_aot_instr=465,837 / ny_aot_cycles=1,127,654 / ny_aot_ms=3`
            - current `ny_main` object snippet is now `mov $0x5b8d83, %eax ; ret`
          - recursive one-hop delegate keeper is now landed:
            - `benchmarks/bench_kilo_micro_userbox_counter_step_chain.hako` + `benchmarks/c/bench_kilo_micro_userbox_counter_step_chain.c`
            - `hako_llvmc_ffi_user_box_micro_seed.inc` now has a narrow `Counter.step_chain` pure-first micro seed behind the same `Counter.value` scalar write and the recursive `Counter.step_chain` / `Counter.step` known-receiver rows
            - `apps/tests/mir_shape_guard/user_box_counter_step_chain_local_i64_min.prebuilt.mir.json` now proves the direct local-i64 `Counter.step_chain` shape without depending on the benchmark loop body
            - `phase163x_direct_emit_user_box_counter_step_chain_contract.sh` pins the current direct-route `Counter.step_chain` contract
            - `phase163x_boundary_user_box_method_known_receiver_min.sh` now keeps `Counter.step_chain` green on boundary pure-first without compat replay
            - latest exact reread: `kilo_micro_userbox_counter_step_chain` = `c_instr=127,245 / c_cycles=230,857 / c_cache_miss=3,693 / c_ms=3` vs `ny_aot_instr=466,852 / ny_aot_cycles=836,012 / ny_aot_cache_miss=8,495 / ny_aot_ms=4`
            - current `ny_main` object snippet is now `mov $0x2b, %eax ; ret`
          - next thin-entry actual-consumer follow-on after this slice:
            - first broader boundary parity widening is now landed too:
              - `apps/tests/mir_shape_guard/user_box_point_sum_local_i64_min.prebuilt.mir.json` now proves the direct local-i64 `Point.sum` known-receiver shape without depending on the benchmark loop body
              - `phase163x_boundary_user_box_method_known_receiver_min.sh` now keeps both `Counter.step` and `Point.sum` green on boundary `pure-first` without compat replay
            - single-copy receiver alias widening is now landed too:
              - `apps/tests/mir_shape_guard/user_box_counter_step_copy_local_i64_min.prebuilt.mir.json`
              - `apps/tests/mir_shape_guard/user_box_point_sum_copy_local_i64_min.prebuilt.mir.json`
              - `phase163x_boundary_user_box_method_known_receiver_min.sh` now also keeps the same known-receiver contract green when the receiver flows through one local `copy`
          - continue widening broader local-method parity from measured contracts instead of a single benchmark shape
          - keep `ArrayBox` typed-slot read-side observer evidence and any new typed-load ABI row separate from that local-method widening
    5. `tuple multi-payload` compat transport is now landed:
        - parser/AST accept `Variant(T, U, ...)` and shorthand `Variant(a, b)` arms
        - Stage1 lowers tuple ctors/matches through `__NyEnumPayload_<Enum>_<Variant>` with `_0`, `_1`, ... synthetic field slots
        - canonical `EnumCtor` / `EnumMatch` / `SumMake` / `SumProject` stay single-slot
    6. `void/null` cleanup is now landed:
        - tokenizer/parser accept both `null` and `void` literal surface, including literal-match arms
        - box helper aliasing now treats `null` and `void` as the same no-value family for direct compat checks
        - reference EBNF now matches the executable surface for both literals
    7. pre-optimization cleanup/doc sync is now landed:
        - LLVM/Python local-sum escape barriers now share one helper instead of repeating materialization logic in `call` / `boxcall` / `ret`
        - runtime nullish checks now converge on the `NullBox::check_null()` helper in the safe compat/tolerance paths touched by this lane
        - MIR reference docs now split cleanly into instruction SSOT + metadata SSOT, and stale "all-in-one" references are reduced to thin pointers
    8. next ready task: resume optimization lane
        - `phase163x-optimization-resume`
        - current parity-wave keeper now landed:
          - metadata-bearing Point local-i64, Flag local-bool, and PointF local-f64 user-box JSON fixtures now stay green on boundary `pure-first` owner lane without compat replay
          - `tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_metadata_keep_min.sh`
          - `apps/tests/mir_shape_guard/user_box_point_local_i64_min.prebuilt.mir.json`
          - `apps/tests/mir_shape_guard/user_box_point_copy_local_i64_min.prebuilt.mir.json`
          - `apps/tests/mir_shape_guard/user_box_flag_local_bool_min.prebuilt.mir.json`
          - `apps/tests/mir_shape_guard/user_box_flag_copy_local_bool_min.prebuilt.mir.json`
          - `apps/tests/mir_shape_guard/user_box_pointf_local_f64_min.prebuilt.mir.json`
          - `apps/tests/mir_shape_guard/user_box_pointf_copy_local_f64_min.prebuilt.mir.json`
        - landed Variant* proof for `phase163x-sum-thin-entry-cutover`:
          - `sum_result_ok_project_copy_local_i64_min.prebuilt.mir.json` now proves the same cutover when `variant_project` reads through a single local `copy` alias
          - `sum_result_ok_project_copy_local_f64_min.prebuilt.mir.json` now proves the same cutover when `variant_project` reads through a single local `copy` alias on the Float payload lane
          - `sum_result_ok_project_copy_local_handle_min.prebuilt.mir.json` now proves the same cutover when `variant_project` reads through a single local `copy` alias on the handle payload lane
          - `sum_result_ok_project_local_f64_min.prebuilt.mir.json` now proves the same cutover for `variant_project` on a Float payload lane
          - `sum_result_ok_project_local_handle_min.prebuilt.mir.json` now proves the same cutover for `variant_project` on a handle payload lane
          - `sum_result_ok_tag_only_local_min.prebuilt.mir.json` now proves the same cutover for a payload-less `variant_tag` keep-lane proof
          - `sum_result_ok_tag_local_f64_min.prebuilt.mir.json` now proves the same cutover for `variant_tag` on a Float payload lane
          - `sum_result_ok_tag_local_handle_min.prebuilt.mir.json` now proves the same cutover for `variant_tag` on a handle payload lane
        - `phase163x-sum-thin-entry-cutover` is complete; the current `ny-llvmc` parity-wave keeper slice now covers Point / Flag / PointF direct+single-copy local keep routes
        - sibling string retained-view exact-micro consumer expansion is now landed:
          - boundary `pure-first` recognizes the current `kilo_micro_substring_views_only` retained-view exit-len shape and collapses it before `substring_hii` / `len_h` replay
          - latest exact reread on `kilo_micro_substring_views_only`: `instr=465,637 / cycles=704,757 / cache-miss=8,280 / AOT 3 ms`
          - latest microasm dump: `ny_main` is now `mov $0x20, %eax ; ret`
        - sibling string mixed accept guardrail is now landed too:
          - `string_corridor_sink` now gets a second sweep after DCE so complementary `substring_len_hii` pairs can fuse once dead borrowed-string temps are removed
          - `kilo_micro_substring_only` now compiles without `substring_len_hii` / `substring_hii` in emitted MIR
          - latest exact reread on `kilo_micro_substring_only`: `instr=1,669,909 / cycles=1,061,204 / cache-miss=8,516 / AOT 3 ms`
          - latest microasm dump: `ny_main` now keeps only the preloop source-length read and the hot loop is scalar `add %rax,%rcx`
        - sibling string retained-slice length consumer expansion is now landed too:
          - `string_corridor_sink` now rewrites `length()` / `len()` on retained slice values into `substring_len_hii` even when the slice producer lives in a dominating block and is only reached through local copy aliases
          - `kilo_micro_len_substring_views` now compiles without loop `RuntimeDataBox.length` / `substring_len_hii` consumers
          - latest exact reread on `kilo_micro_len_substring_views`: `instr=1,672,259 / cycles=1,022,005 / cache-miss=10,525 / AOT 3 ms`
          - latest split-pack reread now keeps all three string split fronts in the same 3 ms band:
            - `kilo_micro_substring_only = instr=1,669,659 / cycles=1,077,794 / cache-miss=8,810`
            - `kilo_micro_substring_views_only = instr=466,001 / cycles=841,958 / cache-miss=9,391`
            - `kilo_micro_len_substring_views = instr=1,672,096 / cycles=1,009,964 / cache-miss=8,902`
        - next substep after the current parity-wave keeper:
          - move from the landed sibling exact micros into the broader string corridor placement/effect rewrite before widening any broader user-box local-body parity backlog
          - keeper repair landed: the exact `pure-first` `kilo_micro_substring_concat` seed now re-accepts the post-sink body shape (`substring_len_hii` pair + `substring_concat3_hhhii`), so the generic concat-observer rewrite no longer ejects the exact lane into the slow fallback route
          - proof-bearing plan metadata widening is now landed: `StringCorridorCandidate` carries `plan` metadata for borrowed-slice and concat-triplet proofs, and MIR JSON exposes the same plan surface to downstream consumers
          - first `publication_sink` inventory slice is now landed too: direct `substring_concat3_hhhii` helper results stay on the same corridor-candidate lane, so emitted MIR JSON on `kilo_micro_substring_concat` now carries a concat-triplet-backed `publication_sink` plan on the helper result itself (`%36`)
          - first actual `publication_sink` transform is now landed on the same non-phi slice: when a direct `substring_concat3_hhhii` helper result stays on the corridor lane, `string_corridor_sink` now rewrites helper-result `length()` to `end - start` and composes helper-result `substring()` back into `substring_concat3_hhhii` from the same plan metadata
          - first non-`phi` `materialization_sink` slice is now landed too: when a direct `substring_concat3_hhhii` helper result flows only into a single local `ArrayBox.set` through copy aliases, `string_corridor_sink` now sinks that birth to the store boundary
          - first post-store observer slice is now landed too: when the same direct `substring_concat3_hhhii` helper result feeds one local `ArrayBox.set` boundary plus one trailing `length()` observer, `string_corridor_sink` now keeps `array.set` as the first `Store` boundary, rewrites the observer to `end - start`, and removes the copy-only observer/store chains
          - first plan-selected `direct_kernel_entry` slice is now landed too: boundary `pure-first` reads `string_corridor_candidates[*].plan.start/end` for direct helper-result receivers and lowers `length()` as the same window arithmetic (`end - start`) instead of rediscovering the route from legacy remembered substring calls
          - targeted boundary proof is now pinned on `apps/tests/mir_shape_guard/string_direct_kernel_plan_len_window_min_v1.mir.json` plus `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_direct_kernel_plan_len_min.sh`; the compile log must hit `substring_len_direct_kernel_plan_window` and lowered IR must not fall back to `nyash.string.len_h` / `nyash.string.substring_len_hii`
          - fresh broader-corridor reread keeps `kilo_micro_substring_concat` (`instr=5,565,773 / cycles=6,143,112 / cache-miss=9,610 / AOT 5 ms`) as the current exact front, while exploratory `kilo_meso_substring_concat_array_set` stayed essentially flat (`instr=384,347,679 / cycles=185,582,276 / AOT 42 ms`), so this cut is a canonical-MIR/kernel asset landing rather than a meso perf keeper by itself
          - fixed reading: do not add a new string-only MIR dialect; with plan metadata landed, the current string work order is now: helper-result `publication_sink` inventory -> helper-result actual `publication_sink` -> `materialization_sink` -> plan-selected `direct_kernel_entry` -> shrink the remaining dynamic/exact bridge paths that still do not read the plan directly
          - migration-safe reading: keep this lane in canonical MIR facts/candidates/sink and kernel/backend substrate only; do not reopen Rust-builder-local shape logic, so the work survives `.hako` builder authority cutover
          - exact `pure-first` seed logic in `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc` is bridge-only and should shrink only after the generic plan-selected route proves out
        - keep `phi_merge` and `call` / `boxcall` / `return` barrier relaxation out of this cut; those require a separate metadata-contract phase first, and the post-store observer lane still needs `array.set + trailing length()` facts before widening beyond the new store-only sink
        - fixed return order from the current bridge shrink:
          1. keep shrinking the remaining exact-seed structural checks only where the live post-sink metadata contract already proves the route
          2. lift loop-carried base/root interpretation out of string-specific placement ownership into a generic MIR phi query / relation seam, then store the string-side continuity as relation metadata before candidate refresh
          3. landed first narrow `plan window across phi_merge` cut on the single-input backedge phi `%22`; merged `%21` is now explicitly `stop_at_merge` and any broader widening stays in a separate metadata-contract phase
        - verified non-Variant optimization order after the current parity wave:
          1. broader string corridor genericization on the existing metadata path
             - current `string_corridor_placement.rs` is inspection-only and does not mutate MIR
             - do not introduce a second string MIR dialect; keep canonical MIR as the only truth
             - landed: `string_corridor_candidates` now carry proof-bearing plan metadata for borrowed-slice and concat-triplet routes
             - landed: direct `substring_concat3_hhhii` helper results now stay on that same metadata lane with a concat-triplet-backed `publication_sink` plan
             - landed: helper-result `length()` / `substring()` consumers now read that same `publication_sink` plan in `string_corridor_sink` without crossing `phi_merge`
             - landed: boundary `pure-first` now reads `direct_kernel_entry` plan windows on helper-result receivers and lowers `length()` as window arithmetic
             - then land the next real string transforms in this order:
               - shrink the remaining `direct_kernel_entry` bridge paths (`find_string_substring_call` and matching exact-seed branches) behind the now-landed plan-window consumer
             - separate follow-on, not this cut:
               - any `phi_merge` relaxation for the loop-carried `text = out.substring(...)` route
             - retire exact seed paths in `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc` only after the generic route proves out
          2. actual-consumer switch for selected user-box thin entries that are still metadata-only today
             - `thin_entry_selection` already inventories `user_box_method.known_receiver`
             - keep this as backend-private lowering work under canonical `Call`, not `.hako` syntax work and not a public MIR dialect fork
          3. `ArrayBox` typed-slot expansion beyond the landed `InlineI64` pilot
             - landed next narrow slices: `InlineBool` and `InlineF64` birth/preserve on existing `slot_store_hih` / `slot_append_hh` any routes
             - current stop-line: do not add new typed load ABI rows without measured observer-side evidence first
             - do not widen this before the string and user-box consumer cuts above have evidence
        - restart handoff:
          - cleanup queue is empty
          - continue `phase163x-optimization-resume` next
          - `phase137x-substring-retained-view-consumer` stays in progress as the sibling string lane
    9. deferred deep deletions (backlog only; do not mix into the current perf proof cut)
        - `phase163x-deep-delete-sum-compat-carriers`
          - retire the remaining `__NyVariant_*` / tuple-enum compat carriers after the current string guardrail keeper lands
          - landed slice: llvm_py entry no longer synthesizes enum-facing `__NyVariant_*` user box declarations; runtime fallback materialization stays on demand in lowering/escape barriers
        - `phase163x-deep-delete-instance-legacy-field-store`
          - remove `InstanceBox` dual legacy field-storage paths only after the current optimization proof and follow-on parity remain green
          - landed:
            - VM variant runtime fallback no longer uses `InstanceBox::set_field_dynamic_legacy`; payloads now ride the interpreter `obj_fields` compatibility store
            - `InstanceBox` no longer gates box-valued fields behind `NYASH_LEGACY_FIELDS_ENABLE`; dedicated `box_fields` are always present for identity-carrying handles
            - legacy helper/toggle cleanup landed: `set_field_dynamic_legacy`, `get_field_legacy`, `set_field_legacy`, and the `NYASH_LEGACY_FIELDS_ENABLE` env toggle are gone
            - `InstanceBox.size` / debug field listing now read the unified field-name union (`fields_ng` + `box_fields`)
            - dead unified/weak InstanceBox facades are gone; `host_box_ops` now calls the canonical `get_field_ng` / `set_field_ng` field path directly
            - sum fallback bridge is now isolated in `sum_bridge`; `__NyVariant_*`, `__variant_tag`, and `__variant_payload` helpers no longer leak across handlers
            - interpreter object-field access is now wrapped by `object_field_store`; `get_object_field` / `set_object_field` / `object_field_root_count` are the only live entry points
            - array/string source cleanup landed: `StringHandleSourceKind` is gone, `with_array_store_str_source` now returns only `ArrayStoreStrSource`, and `array_string_slot` derives source-kind from the enum directly
    10. verified backlog-only optimization inventory
        - semantic/generic backlog:
          - keep `where` / enum methods / full monomorphization in backlog
          - do not promote full monomorphization into the current perf lane; generic surface/type-resolution infrastructure exists, but whole-program specialization is not the next hot-path cut
        - generic optimizer backlog:
          - basic MIR DCE already exists in `src/mir/passes/dce.rs` for unused pure instructions
          - stronger cross-block / partial DCE is later-only and is not the current keeper blocker
          - generic `escape_elide_barriers_vm` remains explicitly VM-only footing in `src/mir/passes/escape.rs`
          - keep any generic LLVM-side escape pass separate from the already-landed narrow objectization-at-boundary route for selected local enums/user boxes
        - not yet fixed as current SSOT tasks:
          - `MapBox` typed value slots
          - float niche tuning (`fast-math` / `FMA` / SIMD-style follow-ons)
          - closure/lambda optimization
  - sibling string guardrail accept gate:
    - `kilo_micro_substring_only`
  - sibling string guardrail split exact fronts:
    - `kilo_micro_substring_views_only`
    - `kilo_micro_len_substring_views`
  - sibling string broader-corridor reopen front: `kilo_micro_substring_concat`
  - pure Rust reference compare lane for string guardrail:
    - `benchmarks/rust/bench_kilo_micro_substring_views_only.rs`
    - `tools/perf/bench_rust_vs_hako_stat.sh kilo_micro_substring_views_only 1 3`
    - latest pure Rust reference: `instr=5,667,104 / cycles=1,572,750 / cache-miss=5,254 / ms=3`
    - latest C-like Rust reference: `instr=12,566,914 / cycles=3,404,383 / cache-miss=5,256 / ms=3`
  - rule: WSL は `3 runs + perf` でしか delta を採らない
  - current string guardrail baseline:
    - `kilo_micro_substring_only: instr=1,669,659 / cycles=1,077,794 / cache-miss=8,810 / AOT 3 ms`
    - split exact reread:
      - `kilo_micro_substring_views_only: instr=466,001 / cycles=841,958 / cache-miss=9,391 / AOT 3 ms`
      - `kilo_micro_len_substring_views: instr=1,672,096 / cycles=1,009,964 / cache-miss=8,902 / AOT 3 ms`
    - broader-corridor reopen front:
      - `kilo_micro_substring_concat: instr=5,565,773 / cycles=6,143,112 / cache-miss=9,610 / AOT 5 ms`
      - `kilo_micro_array_string_store: c_ms=9 / ny_aot_ms=9`; this family is not the current blocker
    - reading: the sibling exact micros are closed at boundary `pure-first`, and `phase-169x` now proves the merged-header length collapse on `substring_concat`, so the next string keeper target stays the broader corridor genericization family
    - fixed genericization order: landed proof-bearing plan metadata on `string_corridor_candidates` -> landed helper-result `publication_sink` inventory slice -> landed helper-result actual `publication_sink` -> next `materialization_sink` -> then plan-selected `direct_kernel_entry`
    - bridge rule: keep `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc` as temporary exact-seed surface only until the generic route can consume the same proof
- current string broader-corridor reopen candidate:
  - loop-carried `text = out.substring(...)` inside `kilo_micro_substring_concat`
  - latest reread: `instr=5,565,773 / cycles=6,143,112 / cache-miss=9,610 / AOT 5 ms`
  - emitted MIR JSON keeps the direct `substring_concat3_hhhii` helper result on the corridor lane with `publication_sink` / `materialization_sink` / `direct_kernel_entry` candidates, while merged header `%21` now also carries `stable_length_scalar` on top of the existing `stop_at_merge` contract
  - the exact front no longer keeps loop `substring_len_hii`; broader generic merged-window carry remains separate backlog
- target band for the next string guardrail keeper:
  - mixed accept gate: hold `instr <= 1.8M`
  - local split `kilo_micro_substring_views_only`: hold `instr <= 0.6M`
  - control split `kilo_micro_len_substring_views`: hold `instr <= 1.8M`
  - broader-corridor reopen `kilo_micro_substring_concat`: first keeper target `instr < 5.5M`
  - whole strict: hold `<= 709 ms`; ideal band is `690-705 ms`
- ideal `len_h` steady-state asm shape:
  - direct `STRING_DISPATCH_FN` load once; do not carry the `STRING_DISPATCH_STATE` state machine in `nyash.string.len_h`
  - direct `host_handles::DROP_EPOCH` load once
  - primary/secondary handle compare only
  - `JIT_TRACE_LEN_ENABLED_CACHE` load once with cold init off the hot return path
  - trace-off fast hit returns directly without carrying extra cold work inline
- current whole-kilo health:
  - `tools/checks/dev_gate.sh quick` is green
  - `kilo_kernel_small_hk` strict latest accepted reread: `ny_aot_ms=709`
  - parity: `vm_result=1140576`, `aot_result=1140576`
- do not reopen:
  - `OwnedText` backing for this lane
  - live-source direct-read widening on `as_str_fast()`
  - global `dispatch` / `trace` false-state fast probes outside `string_len_export_impl()`
  - lifting substring runtime cache mechanics (`cache lookup` / `source liveness check` / `handle reissue`) into `.hako` or `MIR`
  - widening `@rune` beyond declaration-local metadata for this lane
  - generic scalar/cache/route frameworks before a second keeper lane proves the same invariant
- current landed substring truth:
  - `str.substring.route` observe read shows `view_arc_cache_handle_hit=599,998 / total=600,000`
  - `view_arc_cache_reissue_hit=0`, `view_arc_cache_miss=2`, `fast_cache_hit=0`, `dispatch_hit=0`, `slow_plan=2`
  - current keeper removes redundant `view_enabled` state from `SubstringViewArcCache`; this cache only runs under the `view_enabled` route, so the flag compare/store was dead hot-path work
  - `nyash.string.substring_len_hii` is now the current mixed sink candidate; it uses `with_text_read_session_ready(...)` to avoid the hot `REG` ready probe and currently rereads at `47,270,021 instr / 28,264,307 cycles / 9,191 cache-miss / 8 ms`
  - split exact fronts now put `substring_hii` retained-view path at `34.37M instr`
  - `2026-04-09` perf reread on `kilo_micro_substring_views_only`:
    - exact: `instr=34,363,814 / cycles=6,537,017 / cache-miss=10,232 / AOT 4 ms`
    - top: `nyash.string.substring_hii 87.04%`, `ny_main 6.00%`
    - annotate reading:
      1. first hot cluster is `SUBSTRING_ROUTE_POLICY_CACHE` load/decode
      2. second hot cluster is `substring` provider state read + `SUBSTRING_VIEW_ARC_CACHE` TLS entry/state check
      3. only then `SubstringViewArcCache` steady-state compare path
      4. slow plan / materialize is not the dominant cost on this front
  - latest baseline asm reread says the next visible tax is still before the view-arc cache compare block:
    1. `SUBSTRING_ROUTE_POLICY_CACHE` decode
    2. `substring_view_enabled` / fallback provider state reads
    3. only then `SubstringViewArcCache` steady-state compare path
  - current keeper is on `len_h`: `string_len_fast_cache_lookup()` now hoists one `handles::drop_epoch()` read and reuses it across primary/secondary slot checks
  - current keeper also keeps the `len_h` fast-hit return thin: `string_len_export_impl()` now tail-calls a tiny helper so trace-off steady state returns `cached` without carrying `trace_len_fast_hit(...)` inline
  - current keeper removes the `STRING_DISPATCH_STATE` state machine from emitted `nyash.string.len_h`; the hot entry now probes `STRING_DISPATCH_FN` directly once
  - current keeper also splits trace state into `jit_trace_len_state_raw()` and cold `jit_trace_len_state_init()`, so the hot cache-hit path sees one trace-state load and returns directly when trace is off
  - current keeper also lands the `drop_epoch()` global mirror: emitted `nyash.string.len_h` now reads `host_handles::DROP_EPOCH` directly and no longer carries the `host_handles::REG` ready probe / `OnceCell` path
  - latest split exact reread moves first priority back to `substring_hii`; `len_h` now reads as the secondary control split
  - pure Rust reference is the current lower bound for this front; current AOT is about `6.06x instr / 4.10x cycles` over it
  - C-like Rust reference is the current contract-aligned comparison point; current AOT is about `2.73x instr / 1.91x cycles` over it
  - upstream corridor pilot is now structurally landed:
    - single-use `substring(...).length()` chains can sink to `nyash.string.substring_len_hii`
    - kernel export + MIR interpreter fallback are in place
    - current status is structural plus perf-positive candidate: compile/test are green, and the mixed accept gate now rereads at `instr=47,270,021 / cycles=28,264,307 / cache-miss=9,191 / AOT 8 ms`
  - upstream carrier is now also landed in MIR JSON:
    - `functions[].metadata` emits `string_corridor_facts` and `string_corridor_candidates`
    - boundary `pure-first` can now consume the same corridor vocabulary that verbose MIR already exposed
  - boundary `pure-first` consumer is now landed for `substring(...).length()`:
    - direct route trace on `kilo_micro_len_substring_views` now shows `string_len_corridor -> substring_len_direct_kernel_plan_window`
    - post-consumer reread on `kilo_micro_len_substring_views`: `instr=47,263,778 / cycles=28,345,762 / cache-miss=10,603 / AOT 9 ms`
    - post-consumer reread on `kilo_micro_substring_views_only`: `instr=34,364,317 / cycles=6,565,794 / cache-miss=9,276 / AOT 5 ms`
    - latest bridge shrink drops the `substring_len_hii` declaration need on the same plan-window lane; direct-kernel len now consumes metadata rather than substring-call re-inference
    - current `--emit-mir-json` / `--emit-mir-json-minimal` probe on `bench_kilo_micro_substring_concat.hako` reads `interesting_n = 14`, and the active `phase29x_backend_owner_daily_substring_concat_loop_min` smoke now points at the refreshed `apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1_post_sink.mir.json`
    - the phase29x daily-owner route is unblocked again: plain `backend=mir` now executes compiled MIR instead of stopping after compile, and the `.hako ll emitter` runtime decl manifest accepts `nyash.string.substring_len_hii` / `nyash.string.substring_concat3_hhhii`, so the daily smoke emits the expected `[hako-ll/daily]` evidence on the post-sink fixture
    - use `tools/smokes/v2/lib/emit_mir_route.sh --route direct ...` as the trustworthy current-shape probe on this front; `tools/hakorune_emit_mir.sh` is helper-local and can persist a non-strict JSON payload from selfhost stdout capture
    - current live post-sink benchmark body is pinned separately by `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_post_sink_shape.sh`; that smoke now requires the collapsed `source_len + const_len` loop body with no loop `substring_len_hii`, while helper-result `%36` still keeps `publication_sink` / `direct_kernel_entry` plans on the live MIR
    - the same post-sink probe now also pins the seed preheader/exit semantics (`StringBox.length()` on entry, then exit `length() + ... + ret`), so those truths are observable outside the exact seed even while the seed still owns the current guard
    - `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_phi_merge_contract.sh` now also pins the landed metadata-contract follow-on: live direct MIR still carries `%21 = phi([4,0], [22,20])` and `%22 = phi([36,19])`, `%22` keeps `preserve_plan_window`, `%21` keeps `stop_at_merge`, and merged header `%21` now also exposes `stable_length_scalar` with the entry-length witness
    - the same phi smoke now also pins the header/latch loop semantics (`phi/phi/phi`, positive loop bound, compare `<`, branch, and the latch `const 1` increment), so the remaining exact-seed work moved to a semantic-boundary decision rather than more raw body-shape cleanup
    - structure lock: loop-carried corridor continuity now consumes the generic MIR seam in `src/mir/phi_query.rs`; `src/mir/string_corridor_relation.rs` is now the string-side relation layer, and `string_corridor_placement` only maps stored `facts -> relations -> candidates` continuity to optimization candidates
    - decision now fixed: stop shrinking the exact seed at the semantic-guard boundary for this phase
      - keep preheader/exit `length` truth plus header/latch loop truth in the seed as the current miscompile-prevention owner
      - treat any future retirement of those semantic guards as a separate contract phase, not as more bridge cleanup in this wave
    - first direct-set insert-mid smoke is now pinned too: `phase137x_boundary_string_insert_mid_direct_set_min.sh` uses the synthetic direct-set probe to observe `string_insert_mid_window`, keep `nyash.string.insert_hsi` in the lowered IR, and require the plan-backed `plan_window_match` route on the synthetic fixture
    - current reading: the consumer slice is a structural enabler, but the next visible keeper still has to come from retained-view `substring_hii` shapes rather than another runtime-local retry
  - concat/objectization reading is now fixed before the next cut:
    - exact `kilo_micro_substring_concat` is parity-locked again after the pure-first seed repair for the post-sink `substring_len_hii` / `substring_concat3_hhhii` body shape, so it still does not prove the generic concat consumer lane by itself
    - the generic concat observer front is `kilo_micro_concat_hh_len`
    - landed first generic observer pilot:
      - defer concat pair/triple when the consumer stays in compiler-visible string observers
      - lower `len()` from concat chain state without forcing immediate handle birth when the chain stays compile-time-known
    - landed second compiler-visible concat consumer slice:
      - pair/triple concat chains now lower `substring(...)` through `nyash.string.substring_concat_hhii` / `nyash.string.substring_concat3_hhhii`
      - direct pure-first route proof on the dynamic split fixture now hits `string_substring_route -> substring_concat3_hhhii`
    - `2026-04-09` observe direct probe on `kilo_micro_concat_hh_len` now shows:
      - `birth.placement`: `return_handle=0 / borrow_view=0 / freeze_owned=0 / fresh_handle=0 / materialize_owned=0 / store_from_source=0`
      - `birth.backend`: `freeze_text_plan_total=0 / string_box_new_total=0 / handle_issue_total=0 / materialize_owned_total=0 / gc_alloc_called=0`
      - `str.concat2.route=0`, `str.len.route=0`
    - `2026-04-09` exact reread on `kilo_micro_concat_hh_len`: `instr=7,657,032 / cycles=2,284,266 / cache-miss=8,479 / AOT 4 ms`
    - remaining concat publication barriers stay deferred:
      - `return` / `store` / host-boundary concat consumers
      - keep that work separate from the landed `concat -> len` and `concat -> substring` cuts
- rejected perf history:
  - exact evidence is centralized in
    `docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md`
  - current rejected local cuts:
    1. broad `NyashBox` substring-source contract widening
    2. `substring_view_arc_cache_lookup` / `entry_hit` hot-path fusion
    3. birth-side second `with_handle(...)` removal via planner-local source metadata carry
    4. reissue-side slot carry / `refresh_handle` rematch removal
    5. concrete `Arc<StringViewBox>` cache carrier narrowing
    6. `len_h` cache-first reorder
    7. `drop_epoch_if_ready()` fast accessor probe
    8. global `dispatch` / `trace` false-state fast probes
    9. `len_h` dispatch-hit cold split
    10. `trace_len_state()` helper / trace cache single-load probe
    11. `len_h` two-slot pre-match + single epoch-guard probe
    12. local `dispatch_known_absent_fast` + cold dispatch probe combo
    13. `drop_epoch_after_cache_hit()` ready-after-hit probe
    14. `len_h` dispatch single-probe + raw trace-state split
    15. `len_h` 1-probe hash-slot cache shape
    16. registry-pointer epoch read on len cache hits
    17. `len_h` `ReadOnlyScalarLane` separation-only slice
    18. `len_h` combined `ReadOnlyScalarLane` + entry snapshot slice
    19. `len_h`-specific 4-box slice (`façade + control snapshot + pure cache probe + cold path`)
    20. `SubstringViewArcCache` global compare reorder (`start/end` before `source_handle`)
    21. `SubstringViewArcCache` `same_source_pair` specialization
    22. `substring_hii` common-case body duplication via `route_raw == 0b111`
    23. `substring` provider `raw read + cold init` adoption (`substring_view_enabled` / fallback policy / route policy)
    24. `substring_route_policy()` cold init split while keeping the active caller shape unchanged
    25. `substring_hii` route/provider snapshot + eager `DROP_EPOCH` snapshot
    26. `SubstringViewArcCache::entry_hit` reissue/clear cold split
- next active cut:
  - current implementation lane:
    1. keep `primitive-family-and-user-box-fast-path-ssot.md` as the design owner
    2. keep `field_decls` as authority and treat names-only `fields` as compatibility mirror only
    3. add `kilo_micro_userbox_point_add` before wider typed lowering
    4. pilot typed user-box field access only for internal `IntegerBox` / `BoolBox` fields first
    5. keep plugin / reflection / ABI / weak-field paths on generic fallback
    6. keep flattening, tagged pointer, and `@rune` widening out of this wave
  - sibling string guardrail:
    1. keep `kilo_micro_substring_only` as accept gate
    2. use `kilo_micro_substring_views_only` for local `substring_hii` cuts
    3. keep `len_h` runtime mechanics stable unless split fronts move again
    4. latest keeper eliminated the remaining `len_h` control-plane hot loads; do not reopen `len_h` local cuts until `substring` is re-read
    5. do not reopen broad provider-adoption or common-case body duplication cuts already rejected in `phase-137x`
    6. treat concat transient work as a separate observer front:
       - exact seed lane: `kilo_micro_substring_concat`
       - generic consumer lane: `kilo_micro_concat_hh_len`
       - landed compiler-visible cuts: `concat -> len`, then `concat -> substring`
       - remaining publication barriers: `return` / `store` / host-boundary
- first files to reopen for the next string guardrail slice:
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
  - `docs/development/current/main/design/vm-fallback-lane-separation-ssot.md`
  - `docs/development/current/main/phases/phase-162x/README.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `src/mir/string_corridor.rs`
  - `src/mir/string_corridor_placement.rs`
  - `src/mir/passes/string_corridor_sink.rs`
  - `src/config/env/vm_backend_flags.rs`
  - `src/runner/route_orchestrator.rs`
  - `src/runner/keep/vm_fallback.rs`
  - `src/mir/**`
  - `src/llvm_py/instructions/**`
  - `src/backend/mir_interpreter/**`
- safe restart order:
  1. `git status -sb`
  2. `tools/checks/dev_gate.sh quick`
  3. `tools/perf/build_perf_release.sh` (includes `ny-llvmc` now)
  4. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
  5. `docs/development/current/main/phases/phase-163x/README.md`
  6. `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
  7. `docs/development/current/main/phases/phase-137x/README.md`
  8. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  9. after any `nyash_kernel` / `hakorune` / `ny-llvmc` runtime source edit, rerun `bash tools/perf/build_perf_release.sh` before exact micro / asm probes
  10. `tools/perf/run_kilo_string_split_pack.sh 1 3`
  11. `tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_views_only 'nyash.string.substring_hii' 200`
  12. `docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md`
- documentation rule for failed perf cuts:
  1. keep a short current summary in the phase README
  2. keep exact rejected-cut evidence in one rolling doc per front/family/date
  3. do not create test-by-test folders unless that artifact family itself becomes an independent lane

## Implementation Order

1. `docs` first: keep the primitive/user-box lane and the string guardrail lane separate.
2. `field_decls` authority: keep typed field declarations as the source of truth and names-only `fields` as compatibility mirror.
3. `storage_class` inventory: keep primitive/user-box facts fresh in MIR dumps and JSON.
4. `typed primitive fast path`: keep the narrow primitive pilot green.
5. `typed user box field access`: pilot one user-box lane after the local micro gate is added.
6. `flattening` later: only after typed field access has proof.
7. `sink` stays as a string-lane pilot: do not delete the corridor sink path yet; keep it until a newer direct lowering path replaces it with evidence.

## Order At A Glance

1. `phase-147x semantic optimization contract selection` (landed)
2. `phase-148x borrowed text and sink contract freeze` (landed)
3. `phase-149x concat const-suffix vertical slice` (landed)
4. `phase-150x array string-store vertical slice` (landed)
5. `phase-151x canonical lowering visibility lock` (landed)
6. `phase-155x perf canonical visibility tighten` (landed)
7. `phase-156x perf counter instrumentation` (landed)
8. `phase-157x observe feature split` (landed)
9. `phase-158x observe tls backend` (landed)
10. `phase-159x observe trace split` (landed)
11. `phase-160x capability-family inventory` (landed)
12. `phase-161x hot-path capability seam freeze` (landed)
13. `phase-137x main kilo reopen selection` (active sibling string guardrail)
14. `phase-163x primitive and user-box fast path` (active implementation lane)

## Current Front

- read [phase-163x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-163x/README.md) for current implementation lane context
- read [phase-137x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-137x/README.md) for string guardrail context
- read [phase137x-substring-rejected-optimizations-2026-04-08.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md) before retrying any substring-local perf cut
