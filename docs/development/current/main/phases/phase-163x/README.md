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
  - LLVM `field_get` / `field_set` now take a typed IntegerBox path for known user-box `field_decls`
  - LLVM `field_get` now also takes a typed BoolBox path for known user-box `field_decls`
  - LLVM `field_set` now takes a typed BoolBox path only when the source stays on the bool-safe boundary (`BoolBox` handle or bool immediate)
  - compare/bool expressions now lower in value context on the `.hako` builder path, so the BoolBox micro loop shape is accepted structurally instead of via a `.hako` workaround
  - thin-entry inventory is now landed as a no-behavior-change MIR metadata lane:
    - known user-box field/method routes and enum/sum local routes now emit `thin_entry_candidates`
    - verbose MIR and MIR JSON now surface the same inventory
    - `Program(JSON v0)` bridge now refreshes the inventory after callsite canonicalization
  - thin-entry selection pilot is now landed as a no-behavior-change manifest metadata lane:
    - `thin_entry_selections` now bind manifest rows on top of `thin_entry_candidates`
    - primitive user-box field routes now choose between `inline_scalar` thin entries and explicit `public_default` rows
    - known user-box methods and enum/sum local routes now surface manifest-selected thin internal entries while current carriers remain public/compat where the backend has not switched yet
    - verbose MIR, MIR JSON, and `Program(JSON v0)` now surface the same selection results
    - product LLVM/Python user-box `field_get` / `field_set` now consult the selector first:
      - `user_box_field_{get,set}.inline_scalar` rows can keep the typed primitive helper path even when backend-side `field_decls` rediscovery is absent
      - `user_box_field_{get,set}.public_default` rows still keep the generic fallback path for the selected subject
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
  - current keeper pair is `kilo_micro_userbox_point_add` + `kilo_micro_userbox_flag_toggle`
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
  - canonical sum MIR lowering is now landed on the same compiler-first lane:
    - MIR now has `SumMake` / `SumTag` / `SumProject`
    - JSON v0 bridge lowers `EnumCtor` / `EnumMatch` into the dedicated sum lane instead of object-field encoding
    - MIR JSON emit/parse now preserves the same sum ops for handoff/debug
  - VM / LLVM / fallback runtime support is now landed for the same MVP sum lane:
    - VM interpreter snapshots `enum_decls` and executes `SumMake` / `SumTag` / `SumProject` through synthetic `__NySum_<Enum>` fallback `InstanceBox` values
    - LLVM/Py builder registers the same synthetic runtime boxes before `ny_main` and lowers sum ops through `nyash.instance.*_field_h`
    - concrete `Integer` / `Bool` / `Float` payload hints use typed helper lanes
    - LLVM now also recovers erased/generic payloads back to typed `Integer` / `Bool` / `Float` when `sum_make` can observe an actual payload fact locally
    - unknown/genuinely dynamic payloads still stay on boxed-handle fallback
    - malformed tag projections fail fast on both backends instead of silently projecting
    - product `ny-llvmc` ownership remains separate from this compat/harness slice
  - narrow record variants are now landed on the same source / JSON v0 route:
    - declaration surface accepts `Ident { name: String }`
    - qualified construction accepts `Token::Ident { name: expr }`
    - known-enum shorthand match accepts `Ident { name } => ...`
    - record payloads lower through synthetic hidden payload boxes `__NyEnumPayload_<Enum>_<Variant>` while enum values themselves stay on the existing sum lane
    - constructors / patterns must mention the declared field set exactly; multi-payload variants stay deferred
  - post-primitive follow-on queue:
    1. keep `lifecycle-typed-value-language-ssot.md` as the parent reading for boxless interior / boxed boundary work
    2. keep the aggregate/objectization audit pair as the current evidence base:
      - `docs/development/current/main/investigations/phase163x-aggregate-truth-audit-2026-04-09.md`
      - `docs/development/current/main/investigations/phase163x-early-objectization-audit-2026-04-09.md`
    3. recommended next cut = `sum placement/effect pilot`
      - first proving slice: `sum outer-box sinking`
      - the inspection chain (`thin_entry_selections` -> `sum_placement_facts` -> `sum_placement_selections` -> `sum_placement_layouts`) is now landed
      - LLVM now uses the landed selection/layout metadata to keep selected local non-escaping sums boxless through `sum_make` / `sum_tag` / `sum_project`
      - LLVM now materializes runtime `__NySum_*` compat boxes only at `return` / `call` / `boxcall` escape barriers for that selected local route
      - next active substep: validate the proving slice with focused tests/docs before starting the separate `ny-llvmc` parity wave
      - keep canonical `Sum*` unchanged and leave VM / JSON v0 compat fallback intact in this slice
      - after the slice is proven, fold it into the later generic placement/effect pass instead of growing a permanent sum-only branch family
    4. after that, run a separate `ny-llvmc` parity wave
      - proving slice is now landed:
        - product LLVM/Python lowering seeds `thin_entry_selections` into the resolver alongside the already-landed sum placement metadata
        - metadata-bearing product smoke is green on `phase163x_boundary_sum_metadata_keep_min.sh` via boundary compat replay -> harness keep lane
      - native-driver metadata awareness remains canary-only backlog, not the current blocker
    5. `tuple multi-payload` compat transport is now landed
      - parser/AST now accept tuple payload declarations while preserving tuple payload truth above canonical MIR
      - Stage1 lowers tuple ctors/matches through `__NyEnumPayload_<Enum>_<Variant>` hidden payload boxes with `_0`, `_1`, ... field slots
      - canonical `EnumCtor` / `EnumMatch` / `SumMake` / `SumProject` stay single-slot in the same wave
    6. `void/null` cleanup is now landed
      - tokenizer/parser accept both `null` and `void` literal surface, including literal-match arms
      - direct compat null checks treat `NullBox` and `VoidBox` as the same no-value family
      - reference EBNF now matches the executable surface for both literals
    7. pre-optimization cleanup/doc sync is now landed
      - LLVM/Python local-sum escape barriers now share one helper instead of repeating materialization wrappers in `call` / `boxcall` / `ret`
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
