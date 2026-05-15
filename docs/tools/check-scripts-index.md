# Check Scripts Index (SSOT)

Status: Active  
Scope: `tools/checks/*.sh` の入口を一本化して、用途別に迷わず実行できるようにする。

## Quick Entry

```bash
cd /home/tomoaki/git/hakorune-selfhost
tools/checks/dev_gate.sh quick
```

## Core Gates

| Script | Purpose |
| --- | --- |
| `tools/checks/dev_gate.sh` | 日常ゲートの統合実行（quick/hotpath/allocator-wide/portability/milestone）。quick は daily slim、allocator-wide は full allocator/mimalloc/provider proof。 |
| `tools/checks/k2_wide_allocator_gate.sh` | dev_gate allocator-wide から呼ぶ allocator/mimalloc/provider proof group。個別 guard の docs/dev_gate 導線は維持しつつ、実行本体を1入口へ集約する。 |
| `tools/checks/run_row_guard.sh` | `tools/checks/guard_rows.toml` を読む非破壊 row guard pilot runner。安定入口は shell に残し、共通 TOML/argv-array 実行は `tools/checks/lib/manifest_runner.py` に集約する。`dev_gate.sh` / allocator-wide にはまだ組み込まない。 |
| `tools/checks/run_proof_app.sh` | `tools/checks/proof_apps.toml` を読む非破壊 proof app runner。app-local `test.sh` 入口を保ったまま、共通 TOML/argv-array 実行を `tools/checks/lib/manifest_runner.py` に委譲する。 |
| `tools/checks/manifest_runner_pilot_guard.sh` | D199 manifest runner cleanup guard。`run_row_guard.sh` / `run_proof_app.sh` が薄い wrapper に留まり、共有 runner が `tomllib` と `subprocess.run(argv)` を持ち、pilot が `dev_gate.sh` / allocator-wide に入らないことを固定する。 |
| `tools/checks/allocator_provider_inactive_sentinel_guard.sh` | quick 用の軽量 provider inactive sentinel。selection/proof consumption/rollback/gate/hook/replacement/`.inc` matcher の危険信号だけを共有 forbidden-pattern guard で固定する。 |
| `tools/checks/lib/cargo_test_filter_group.sh` | quick first-row guards 用の共有 helper。main crate lib test target に限定して関連 cargo test filter を contract-family 単位に束ね、route/file lock は各 guard 側に残す。 |
| `tools/checks/current_state_pointer_guard.sh` | `CURRENT_STATE.toml` をSSOTとして current pointer の必須path / latest-card整合 / stale phase 名を fail-fast で検出する。current mirrors に latest-card履歴の再掲は要求しない。 |
| `tools/checks/inc_codegen_thin_shim_guard.sh` | `.inc` codegen の raw MIR analysis debt no-growth baseline。削減は許可し、新規/増加を fail-fast で止める。明示された view-owner 領域だけは `tools/checks/inc_codegen_thin_shim_view_allowlist.tsv` で別枠固定する。 |
| `tools/checks/generic_method_set_policy_mirror_guard.sh` | `CollectionMethodPolicyBox.set_route(...)` と C shim の generic-method `Set` route/demand mirror を固定し、`ArrayStoreString` の source/identity/publication demand drift を fail-fast で検出する。 |
| `tools/checks/core_method_contract_manifest_guard.sh` | `CoreMethodContractBox` から生成する `core_method_contract_manifest.json` の drift を fail-fast で検出する。 |
| `tools/checks/core_method_contract_inc_no_growth_guard.sh` | CoreMethodContract 移行中の generic-method policy / mir-call route policy `.inc` method/box 名比較が manifest と撤去条件なしに増えないことを fail-fast で検出する。 |
| `tools/checks/mir_root_facade_guard.sh` | `src/mir/mod.rs` の root facade export を allowlist で固定し、core/facade/refresh 以外の再肥大を fail-fast で検出する。 |
| `tools/checks/mir_root_import_hygiene_guard.sh` | MIR root wildcard import、semantic metadata vocabulary の root 経由参照、crate-internal detection bridge の再導入を禁止し、owner-module import 境界を fail-fast で固定する。 |
| `tools/checks/mir_builder_calltarget_owner_guard.sh` | `CallTarget` の owner path を `builder/calls` に固定し、`builder_calls` compatibility shell / re-export / caller regrowth を fail-fast で検出する。 |
| `tools/checks/map_lookup_fusion_reader_boundary_guard.sh` | `map_lookup_fusion_routes` を読む `.inc` を共有 reader seam に限定し、get/has policy が enum/table consumer に留まることを fail-fast で検出する。 |
| `tools/checks/route_detector_legacy_surface_guard.sh` | JoinIR route detector の `legacy/` storage / legacy module / 旧 compatibility path / `LoopPatternKind` alias が再導入されないことを fail-fast で検出する。 |
| `tools/checks/array_string_push_get_metadata_fixture_guard.sh` | array-string boundary fixtures の `RuntimeDataBox.push/get(ArrayBox)` が MIR-owned CoreMethod metadata を持ち、pure-first route state に消費されることを fail-fast で検出する。 |
| `tools/checks/stage1_emit_program_json_runtime_helper_guard.sh` | public `BuildBox.emit_program_json_v0(source, null)` が Stage1 Program(JSON v0) runtime helper route として pure-first で消費されることを fail-fast で検出する。 |
| `tools/checks/stage0_shape_inventory_guard.sh` | `GlobalCallTargetShape` variants が Stage0 LLVM line shape inventory SSOT に全て棚卸しされていることを fail-fast で検出する。 |
| `tools/checks/program_json_dev_surface_guard.sh` | archived Program(JSON) diagnostics probes / empty dev capsule directory が active `tools/dev` surface に戻らないことを fail-fast で検出する。 |
| `tools/checks/program_json_v0_compat_caller_guard.sh` | raw `Program(JSON v0)` compat emit helper の active shell caller を `stage1_contract.sh` / `stageb_helpers.sh` に限定する。 |
| `tools/checks/program_json_mir_bridge_caller_guard.sh` | `Program(JSON)->MIR` bridge helper の active shell caller を `selfhost_exe_stageb.sh` / phase29cg proof に限定する。 |
| `tools/checks/stageb_program_json_capture_caller_guard.sh` | Stage-B Program(JSON) stdout capture helper の active shell caller を MIR emit / Stage-B helper surfaces に限定する。 |
| `tools/checks/stage1_program_json_compat_caller_guard.sh` | Stage1 Program(JSON) compat execution helper の active shell caller を phase29ch explicit probe に限定する。 |
| `tools/checks/phase29ch_route_probe_surface_guard.sh` | archived phase29ch route diagnostics probes が active `tools/dev` surface に戻らず、live compat keeper だけが残ることを fail-fast で検出する。 |
| `tools/checks/phase29ck_preperf_probe_surface_guard.sh` | archived phase29ck pre-perf diagnostics probes が active `tools/dev` surface に戻らず、live compat/dialect keepers が残ることを fail-fast で検出する。 |
| `tools/checks/phase29ck_small_entry_probe_surface_guard.sh` | archived phase29ck small-entry perf diagnostics probes が active `tools/dev` surface に戻らず、runtime-proof smoke anchor が残ることを fail-fast で検出する。 |
| `tools/checks/phase29ci_verify_probe_surface_guard.sh` | archived phase29ci W17 verify proof が active `tools/dev` surface に戻らないことを fail-fast で検出する。 |
| `tools/checks/phase216217_normalization_canary_surface_guard.sh` | archived phase216/217 normalization bring-up canaries が active `tools/dev` surface に戻らず、current phase2160 dehang proof が残ることを fail-fast で検出する。 |
| `tools/checks/legacy_dev_utility_surface_guard.sh` | archived one-shot / old-phase / duplicate dev-env utilities が active `tools/dev` surface に戻らないことを fail-fast で検出する。 |
| `tools/checks/lang_include_surface_guard.sh` | `lang/src` の `include "..."` 再導入と旧 `tools/dev` guard path の復活を fail-fast で検出する。 |
| `tools/checks/tools_dev_surface_inventory_guard.sh` | `tools/dev` の active file set が `tools/dev/README.md` の棚卸しから drift しないことを fail-fast で検出する。 |
| `tools/checks/hakorune_emit_mir_direct_caller_guard.sh` | `tools/hakorune_emit_mir.sh` の direct shell caller を thin preset/selfhost runtime owners に限定し、smoke/check/perf/dev からは `emit_mir_route.sh` 経由に固定する。 |
| `tools/checks/mir_builder_layer_dependency_guard.sh` | MIR builder layer の `origin -> observe -> rewrite` dependency direction を fail-fast で検出する。 |
| `tools/checks/loop_pattern_context_zero_guard.sh` | Rust source に `LoopPatternContext` が再導入されないことを fail-fast で検出する。 |
| `tools/checks/phase29ca_direct_verify_dominance_block_canary.sh` | phase29ca direct route の dominance/Phi blocker と loop progression 回帰を release binary で監視する。 |
| `tools/checks/cargo_check_safe.sh` | EXDEV 環境向けの cargo check wrapper。`exdev_rename_copy_fallback.c` を LD_PRELOAD して rename EXDEV を copy+unlink に変換する。 |
| `tools/checks/module_registry_hygiene_guard.sh` | `hako.toml` / `nyash.toml` の module registry 境界検証。 |
| `tools/checks/phase29cl_by_name_mainline_guard.sh` | `nyash.plugin.invoke_by_name_i64` の owner 集合を allowlist で固定し、新しい mainline caller を fail-fast で防ぐ。 |
| `tools/checks/ring1_core_scope_guard.sh` | ring1 provider の受理ドメイン境界検証。 |
| `tools/checks/k2_wide_rawbuf_first_row_guard.sh` | `RawBufCoreBox` の最小 allocation facade が `MemCoreBox` 直上に留まり、allocator policy/state や layout/MaybeInit/TLS/atomic/OSVM に広がらないことを固定する。 |
| `tools/checks/k2_wide_static_const_table_decl_guard.sh` | M11b-decl の `static const NAME: u16[]` 受理形が Rust parser / `.hako` parser / Program JSON / MIR `static_data_plans` / ll_emit reader に流れることを固定する。 |
| `tools/checks/k2_wide_static_const_table_load_guard.sh` | M11b-load の `NAME[index]` 受理形が MIR `StaticDataLoad` / MIR JSON `static_data_load` / VM metadata read / ll_emit direct load に流れることを固定する。 |
| `tools/checks/k2_wide_static_const_table_eval_guard.sh` | M11b-eval の `u16[]` initializer integer const expressions が Rust parser / `.hako` parser / Program JSON / MIR metadata に evaluated values として流れることを固定する。 |
| `tools/checks/k2_wide_inline_plan_preserve_guard.sh` | M11c-preserve の `@rune Hint(inline/noinline/hot/cold)` が MIR `metadata.inline_plans` に保存され、`.inc` が inline 判断を持たないことを固定する。 |
| `tools/checks/k2_wide_inline_plan_soft_leaf_guard.sh` | M11c-soft-leaf の narrow same-module MIR leaf inline が optimizer pass に閉じ、`.inc` が inline 判断を持たないことを固定する。 |
| `tools/checks/k2_wide_inline_required_vocab_guard.sh` | M11c-required-vocab の `@rune Lowering(inline_required)` が Rust parser / `.hako` parser / MIR `request=required` metadata に流れ、`.inc` が required inline 判断を持たないことを固定する。 |
| `tools/checks/k2_wide_rune_contract_repeat_guard.sh` | M11c-contract-repeat の distinct `Contract(...)` repeatable parser policy を Rust parser / `.hako` parser に閉じ、同一 Contract 重複を fail-fast に保つことを固定する。 |
| `tools/checks/k2_wide_inline_required_verify_guard.sh` | M11c-required-verify の `Lowering(inline_required)` verifier acceptance が `Contract(no_alloc/no_safepoint)` と narrow leaf-inline shape に閉じ、`.inc` が required inline を消費しないことを固定する。 |
| `tools/checks/k2_wide_effect_capability_plan_guard.sh` | M11d の `EffectPlan` / `CapabilityPlan` metadata boundary が MIR/verifier に閉じ、rune-derived plan refresh が `refresh_function_rune_plans` に集約され、Capability parser surface と `.inc` consumption が増えないことを固定する。 |
| `tools/checks/k2_wide_mimalloc_raw_page_proof_guard.sh` | M12 raw-page proof が `RawBufCoreBox` + `RawArrayCoreBox` + `Contract(no_alloc/no_safepoint)` の MIR verifier/metadata fixture に閉じ、Profile/Capability/unsafe/backend special-case を増やさないことを固定する。 |
| `tools/checks/k2_wide_profile_registry_docs_guard.sh` | M12b/M12c Profile registry docs が `docs/reference/mir/rune-profile-registry.md` に閉じ、Capability parser surface と `.inc` profile-name consumption が増えないことを固定する。 |
| `tools/checks/k2_wide_profile_expansion_to_facts_guard.sh` | M12c Profile expansion が reserved registry names だけを parser surface とし、MIR InlinePlan / EffectPlan / CapabilityPlan facts へ展開し、backend/.inc が profile names を読まないことを固定する。 |
| `tools/checks/k2_wide_allocator_fast_path_exe_guard.sh` | M13 scalar allocator-fast proof が `Profile(allocator.fast)` を verified required InlinePlan として MIR optimizer で消費し、pure-first EXE へ profile-name-free の scalar MIR として渡ることを固定する。 |
| `tools/checks/k2_wide_return_proof_vocab_guard.sh` | M10c-pre の return proof vocabulary が docs/TOML/Rust で同期し、handle return class が LLVM pointer attrs を持たないことを固定する。 |
| `tools/checks/k2_wide_runtime_decl_return_proof_row_guard.sh` | M10c-proof-row の runtime-decl return proof row schema が fixture/Rust validator/docs で同期し、active runtime-decl と `.inc` が strong attrs を出さないことを固定する。 |
| `tools/checks/k2_wide_native_ptr_decl_type_guard.sh` | M10c-native-ptr-declare-type の `native_ptr_* -> ptr` 型名対応を `.hako` ll_emit reader に閉じ、型名 reader が `.inc` に漏れないことを固定する。 |
| `tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh` | M10c-hako-mem alloc/realloc/free rows の `hako_mem_alloc` / `hako_mem_realloc -> native_ptr_nullable` と `hako_mem_free -> void` runtime-decl row / generated defaults 同期を固定し、native pointer arg emission が `ptr` で void call が `call void` であること、他 native pointer row / `ret_proofs` / strong attrs の混入を防ぐ。 |
| `tools/checks/k2_wide_hako_mem_alloc_runtime_decl_guard.sh` | 293x-052 互換入口。現在は `k2_wide_hako_mem_runtime_decl_guard.sh` に委譲する。 |
| `tools/checks/k2_wide_mimalloc_raw_page_exe_guard.sh` | M20 の mimalloc raw-page pure-first EXE parity guard を固定し、M14-M19 raw memory / RawBuf / RawArray route surface の合成が allocator policy なしで実行できることを検証する。 |
| `tools/checks/k2_wide_mimalloc_size_class_table_exe_guard.sh` | M21 の static u16 size-class table + raw-page pure-first EXE 合成 proof を固定し、runtime Array/Map table materialization、app-specific `.inc` matcher、新しい source syntax / allocator policy の混入を防ぐ。 |
| `tools/checks/k2_wide_mimalloc_two_class_page_exe_guard.sh` | M22 の static u16 size-class table + two-class raw-page pure-first EXE proof を固定し、small/medium の reject/release/reuse が既存 route surface だけで動くことを検証する。 |
| `tools/checks/k2_wide_mimalloc_dynamic_bin_exe_guard.sh` | M23 の runtime-indexed static u16 size-class table + raw-page pure-first EXE proof を固定し、非定数 `static_data_load` index が app-specific `.inc` matcher なしで動くことを検証する。 |
| `tools/checks/k2_wide_mimalloc_size_to_bin_inline_exe_guard.sh` | M24 の `Profile(allocator.fast)` size_to_bin inline + runtime-indexed static u16 table + raw-page pure-first EXE proof を固定し、backend/.inc が profile 名を読まないことを検証する。 |
| `tools/checks/k2_wide_mimalloc_size_class_policy_guard.sh` | M163 の `.hako` `SizeClassBox` policy owner を固定し、mimalloc-shaped size-to-bin/bin-size と `LayoutBox` 互換 facade が allocator state や `.inc` matcher なしで動くことを検証する。 |
| `tools/checks/k2_wide_mimalloc_layout_migration_guard.sh` | M164 の layout migration closeout を固定し、`SizeClassBox` が size-class truth、`LayoutBox` が small/medium 互換 facade、`page_heap_box` が facade consumer に留まることを検証する。 |
| `tools/checks/k2_wide_mimalloc_page_model_guard.sh` | M165 の `page_box.hako` page-local model owner を固定し、`free` / `local_free` / `used` / `capacity` / `reserved` の不変条件が heap/queue/OSVM/TLS/atomic/remote-free なしで動くことを検証する。 |
| `tools/checks/k2_wide_mimalloc_page_queue_guard.sh` | M166 の `page_queue_box.hako` page queue/direct-page cache owner を固定し、queue が `freeCount()` でページを選ぶだけで `.acquire()` によるblock popを行わないことを検証する。 |
| `tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh` | M167 の `alloc_fast_path_heap_box.hako` fast-path orchestration を固定し、page queue selection + page-local free-list pop + deterministic modeled fallback が OSVM/TLS/atomic/remote-free/page-map/provider/hook なしで動くことを検証する。 |
| `tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh` | M168 の `osvm_backed_fast_path_heap_box.hako` OSVM-backed fast-path adapter を固定し、fresh modeled page creation が既存 `HakoAllocPageSourcePolicy` reserve/commit/decommit rows を通って pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh` | M169 の `page_box.hako` page-local local-free collection / empty-page retire state を固定し、remote-free atomics / abandoned reclaim / OSVM release / page-map / provider / hook なしで動くことを検証する。 |
| `tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh` | M170 の `remote_free_page_integration_box.hako` が既存 pointer load/store/CAS remote-free policy と `HakoAllocPageModel.releaseLocal(...)` を合成し、page-map / pointer fetch_add / provider / hook / replacement なしで pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_mimalloc_page_map_guard.sh` | M171 の `page_map_box.hako` pointer-to-page ownership model を固定し、arbitrary free / realloc / pointer arithmetic / remote-free atomics / provider / hook / replacement なしで pointer→page/block lookup が VM 実行できることを検証する。 |
| `tools/checks/k2_wide_mimalloc_page_map_release_guard.sh` | M172 の `page_map_release_box.hako` が `HakoAllocPageMap.lookup(...)` / `HakoAllocPageModel.releaseLocal(...)` / `HakoAllocPageMap.unregister(...)` を合成し、registration / realloc / byte copy / provider / hook / replacement なしで VM 実行と MIR route 契約を検証する。 |
| `tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh` | M173 の `page_map_release_invariant_box.hako` が `HakoAllocPageMapReleaseSeam.releasePtr(...)` を観測して handle lifetime / release-unregister timing / reject visibility を固定し、realloc / byte copy / provider / hook / replacement なしで VM 実行と MIR route 契約を検証する。 |
| `tools/checks/k2_wide_mimalloc_realloc_same_class_guard.sh` | M174 の `page_map_realloc_same_class_box.hako` が live page-map identity と current page block size を使って same-class/no-move realloc を固定し、release / unregister / alloc-copy-release fallback / byte copy なしで VM 実行と MIR route 契約を検証する。 |
| `tools/checks/k2_wide_mimalloc_realloc_alloc_copy_release_guard.sh` | M175 の `page_map_realloc_alloc_copy_release_box.hako` が replacement ptr allocation + copy-count modeling + old-ptr release orderingを固定し、same-class branch / byte copy / aligned / huge / provider / hook なしで VM 実行と MIR route 契約を検証する。 |
| `tools/checks/k2_wide_mimalloc_realloc_failure_contract_guard.sh` | M176 の `page_map_realloc_failure_contract_box.hako` が zero / oversized / unknown / stale / released / alloc-fail の failure matrix を M174/M175 delegation 上で固定し、release/register/unregister 実装や aligned/huge へ広がらないことを VM 実行と MIR route 契約で検証する。 |
| `tools/checks/k2_wide_mimalloc_alignment_policy_guard.sh` | M177 の `alignment_policy_box.hako` が alignment normalization / power-of-two reject / padded-size policy を pure policy row として固定し、aligned allocation execution / huge routing / provider / hook へ広がらないことを VM proof で検証する。 |
| `tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh` | M178 の `page_map_aligned_small_path_box.hako` が normal page-map-backed small allocations に alignment metadata を付ける small-path execution を固定し、huge routing / native alignment claim / provider / hook へ広がらないことを VM proof で検証する。 |
| `tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh` | M179 の `huge_threshold_router_box.hako` が padded request を last regular size-class threshold で分類し、huge unsupported を fail-fast しつつ small request を M178 owner にだけ委譲することを VM proof で検証する。 |
| `tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh` | M180 の `huge_page_model_box.hako` が one-allocation huge page metadata を page-map 登録と分離して保持し、huge release / OS release / small-page free-list へ広がらないことを VM proof で検証する。 |
| `tools/checks/k2_wide_mimalloc_huge_release_seam_guard.sh` | M181 の `huge_release_seam_box.hako` が huge model live state と page-map unregister を合成し、small-page `releaseLocal` / OS release / provider hook へ広がらないことを VM proof で検証する。 |
| `tools/checks/k2_wide_mimalloc_secure_list_diagnostics_guard.sh` | M183 の `secure_free_list_diagnostics_box.hako` が free/local_free の out-of-range / duplicate / live-block / count-mismatch を diagnostics-only で検出し、encode/decode や hardening へ広がらないことを VM proof で検証する。 |
| `tools/checks/k2_wide_mimalloc_secure_list_policy_guard.sh` | M184 の `secure_free_list_policy_box.hako` が caller-provided cookie による encoded-next encode/decode と capacity validation に閉じ、entropy source / page mutation / hardening claim へ広がらないことを VM proof で検証する。 |
| `tools/checks/k2_wide_mimalloc_numeric_field_inventory_delta_guard.sh` | M185 の `NUMERIC_FIELDS.md` post-M184 inventory が source の production stored numeric field count と一致し、sentinel / secure-list / next-migration boundaries を固定していることを検証する。 |
| `tools/checks/k2_wide_mimalloc_size_class_usize_policy_guard.sh` | M187 の `SizeClassBox` usize input facades が既存 signed-sentinel result owner に委譲し、stored field migration や backend/.inc special-case を増やさないことを VM proof で検証する。 |
| `tools/checks/k2_wide_mimalloc_request_path_usize_guard.sh` | M188 の alignment/page/acquire/aligned-small/huge-router request path usize input facades が stored field migration や result sentinel migration に広がらないことを VM proof で検証する。 |
| `tools/checks/k2_wide_mimalloc_object_return_api_guard.sh` | M189 の `HakoAllocHeap.allocate/realloc` object-return API が scalar observer substitution なしで VM と pure-first EXE の同一 proof line を保つことを検証する。 |
| `tools/checks/k2_wide_mimalloc_result_contract_guard.sh` | M190 の `HakoAllocHandleResult` / `allocateResult` / `reallocResult` が明示 reason code を返し、VM と pure-first EXE の同一 proof line を保つことを検証する。 |
| `tools/checks/k2_wide_logical_condition_surface_guard.sh` | C197 の parenthesized multiline `&&` / `||` condition surface を固定し、通常 short-circuit semantics と future `check` proof-list surface の分離を検証する。 |
| `tools/checks/k2_wide_check_block_surface_guard.sh` | C198 の `check "name" { "label": expr }` proof-list surface を固定し、全 item eager 評価と scalar pass/fail result を VM proof で検証する。 |
| `tools/checks/k2_wide_compound_assignment_surface_guard.sh` | C199 の `+=` / `-=` / `*=` / `/=` surface を固定し、local / field / index targets が canonical assignment sugar として VM proof で動くことを検証する。 |
| `tools/checks/k2_wide_guard_else_surface_guard.sh` | C200 の `guard expr else { ... }` surface を固定し、既存 `If(UnaryOp::Not(...))` sugar として VM proof で動くことを検証する。 |
| `tools/checks/k2_wide_user_box_field_index_fast_path_guard.sh` | C201 の ordinary user-box field-index fast path metadata を固定し、MIR JSON が legal typed fields に `layout_id` / `field_index` / `storage` を出すことを検証する。 |
| `tools/checks/k2_wide_record_surface_guard.sh` | C202 の `record` declaration surface を固定し、typed fields only / identity-free aggregate contract / ordinary box 非混入を検証する。 |
| `tools/checks/k2_wide_contract_syntax_metadata_guard.sh` | CONTRACT-002 の `requires` / `ensures` / `invariant` Stage0 metadata capsule を固定し、AST/JSON transport と runtime lowering 不在を検証する。 |
| `tools/checks/k2_wide_transition_metadata_capsule_guard.sh` | TRANS-001 の `transition Enum.A -> Enum.B by method` Stage0 metadata capsule を固定し、contextual syntax / AST JSON / Program JSON v0 transport と checker 不在を検証する。 |
| `tools/checks/k2_wide_uses_metadata_capsule_guard.sh` | USES-001 の method-level `uses` Stage0 metadata capsule を固定し、contextual syntax / AST JSON / Program JSON v0 transport と capability checker 不在を検証する。 |
| `tools/checks/k2_wide_generic_type_annotation_metadata_guard.sh` | GEN-001 の generic type annotation Stage0 metadata capsule を固定し、box/record/function/type alias の TYPE_REF transport と Stage1 semantics 不在を検証する。 |
| `tools/checks/k2_wide_generic_arity_checker_guard.sh` | GEN-002 の Stage1 generic arity checker を固定し、既知 generic type reference の型引数数 mismatch を fail-fast で検出する。 |
| `tools/checks/k2_wide_language_surface_admission_guard.sh` | language minimal surface の入場審査を固定し、小さい surface / 強い semantics / fold-first rule が docs 入口から辿れることを検証する。 |
| `tools/checks/k2_wide_array_result_option_surface_guard.sh` | ARRAY-RESULT-SSOT の docs-only canonical surface を固定し、`Array<T>` / `PackedArray<T>` / `Result<T,E>` / `Option<T>` / `Type::Variant` と後続タスク分解を検証する。 |
| `tools/checks/k2_wide_enum_variant_canonical_surface_guard.sh` | ENUMVAR-001 の canonical `Type::Variant` surface を固定し、transition metadata の `Enum::Value` transport と legacy dot normalization を検証する。 |
| `tools/checks/k2_wide_localtype_metadata_capsule_guard.sh` | LOCALTYPE-001 の `local name: Type` metadata capsule を固定し、AST/AST JSON/Program JSON v0 transport と単一束縛制約を検証する。 |
| `tools/checks/k2_wide_array_typed_context_literal_guard.sh` | ARRAY-001 の typed-context `Array<T>` literal lowering を固定し、untyped `[]` と `PackedArray<T> = []` の no-fallback fail-fast を検証する。 |
| `tools/checks/k2_wide_array_typed_method_contract_guard.sh` | ARRAY-002A の typed `Array<T>` method contract を固定し、`push/get/set/length` の名前と arity diagnostics を検証する。 |
| `tools/checks/k2_wide_array_typed_element_checks_guard.sh` | ARRAY-002B の typed local `Array<T>` direct element checks を固定し、literal / `push` / `set` の known mismatch diagnostics を検証する。 |
| `tools/checks/k2_wide_array_inference_failfast_guard.sh` | ARRAY-002C の unsupported `Array<T>` inference fail-fast を固定し、unresolved `Array<T>` element と mixed direct literals の診断を検証する。 |
| `tools/checks/k2_wide_arraybox_json_v0_backend_guard.sh` | ARRAY-002D の ordinary `Array<T>` JSON v0 / ArrayBox route と `PackedArray<T>` no-fallback contract を検証する。 |
| `tools/checks/k2_wide_result_option_prelude_diagnostics_guard.sh` | RESULT-001 の `Option<T>` / `Result<T,E>` built-in enum prelude と dot-variant fail-fast diagnostics を検証する。 |
| `tools/checks/k2_wide_result_option_missing_arm_diagnostics_guard.sh` | RESULT-002A の prelude enum missing-arm diagnostics を固定し、`Option::None` / `Result::Err` と `_` 不成立メッセージを検証する。 |
| `tools/checks/k2_wide_result_option_payload_diagnostics_guard.sh` | RESULT-002B の prelude enum payload diagnostics を固定し、`Some` / `None` / `Ok` / `Err` の arity diagnostics と nullish contract 維持を検証する。 |
| `tools/checks/k2_wide_known_enum_underscore_exhaustiveness_guard.sh` | RESULT-002C の known-enum `_` exhaustiveness rule を固定し、fallback arm が missing variant を満たさないことを検証する。 |
| `tools/checks/k2_wide_result_option_expected_type_diagnostics_guard.sh` | RESULT-002D の prelude generic enum expected-type diagnostics を固定し、型文脈なしの `Option::None` / `Result::Err(...)` を fail-fast する。 |
| `tools/checks/k2_wide_guard_let_pattern_sugar_guard.sh` | GUARDLET-001 の narrow `guard let Type::Variant(binding)` sugar を固定し、既存 Local / If / EnumMatchExpr への rewrite を検証する。 |
| `tools/checks/k2_wide_loopclean_while_parser_normalization_guard.sh` | LOOPCLEAN-002 の while parser normalization を固定し、新規 Stage-3 `while` が canonical `ASTNode::Loop` として出ることを検証する。 |
| `tools/checks/k2_wide_loopclean_while_variant_quarantine_guard.sh` | LOOPCLEAN-003 の while variant quarantine を固定し、新規 source は `Loop`、legacy `ASTNode::While` は Program(JSON) `Loop` 互換 lowering に閉じることを検証する。 |
| `tools/checks/k2_wide_loopclean_range_parser_helper_guard.sh` | LOOPCLEAN-004 の range parser helper commonization を固定し、canonical `loop i in` と legacy `for i in` が共有 header parser から同じ `ForRange` shape を出すことを検証する。 |
| `tools/checks/k2_wide_packed_array_eligibility_guard.sh` | PACKED-001 の Stage1 `PackedArray<T>` eligibility gate を固定し、packed residence が証明できない source を fail-fast しつつ runtime/backend auto-use に広がらないことを検証する。 |
| `tools/checks/k2_wide_source_packed_array_autouse_pilot_guard.sh` | PACKED-002 の source `PackedArray<Record>` auto-use pilot metadata を固定し、backend lowering / public materialization / boxed fallback が閉じていることを検証する。 |
| `tools/checks/k2_wide_astclean_legacy_enum_guard.sh` | ASTCLEAN-001 の legacy AST enum removal を固定し、旧 split enum / `ASTNodeType` が戻らず `is_expression()` の直接判定だけが残ることを検証する。 |
| `tools/checks/k2_wide_record_decl_metadata_transport_guard.sh` | C203a の `record_decls` metadata transport lane を固定し、Program JSON v0 / JSON bridge / MIR metadata / MIR JSON が record を ordinary user-box lane に混ぜないことを検証する。 |
| `tools/checks/k2_wide_record_layout_plan_guard.sh` | C203b の `record_layout_plans` metadata lane を固定し、concrete record fields の slot/storage layout が typed-object/user-box layout lane と混ざらないことを検証する。 |
| `tools/checks/k2_wide_record_local_scalar_metadata_guard.sh` | C203c の `record_local_layout` folded agg-local / placement metadata lane を固定し、record route が user-box seed route や backend matcher に漏れないことを検証する。 |
| `tools/checks/k2_wide_array_record_storage_descriptor_guard.sh` | C204a の `array_record_storage_plans` metadata descriptor lane を固定し、ArrayBox runtime/backend/hako_alloc へまだ漏れないことを検証する。 |
| `tools/checks/k2_wide_arraybox_inline_record_storage_guard.sh` | C204b の `ArrayStorage::InlineRecord` runtime-private vocabulary を固定し、visible materialization / hako_alloc / backend lowering へまだ漏れないことを検証する。 |
| `tools/checks/k2_wide_arraybox_inline_record_probe_guard.sh` | C206b/C206c の `ArrayInlineRecordProbe` を test-only explicit probe owner として固定し、成功構築と ragged-column 拒否が compiler auto-use / public ArrayBox API / hako_alloc migration に広がらないことを検証する。 |
| `tools/checks/k2_wide_arraybox_inline_record_plan_probe_guard.sh` | C206d の `ArrayInlineRecordPlanProbe` を test-only plan-to-runtime probe adapter として固定し、integer-lane plan だけを明示 probe に接続して compiler auto-use / hako_alloc migration に広がらないことを検証する。 |
| `tools/checks/k2_wide_guard_refresh_policy_guard.sh` | D196 の guard refresh policy を固定し、C206+ cleanup/probe guard が local-run/index-listed のまま quick/dev gate や allocator-wide gate に増殖しないことを検証する。 |
| `tools/checks/k2_wide_metadata_store_indexed_read_guard.sh` | C206e の allocator metadata store indexed read cleanup を固定し、aligned-small / huge-page metadata store の pointer API が index-based read seam に委譲することを検証する。 |
| `tools/checks/k2_wide_arraybox_inline_record_autouse_eligibility_guard.sh` | C207 の packed ArrayBox compiler auto-use eligibility gate を固定し、metadata-only eligibility が runtime auto-use / hako_alloc migration / backend lowering へ広がらないことを検証する。 |
| `tools/checks/k2_wide_arraybox_inline_record_materialization_boundary_guard.sh` | C208 の inline-record materialization / escape boundary を固定し、non-escaping direct field reads だけを将来候補として残しつつ visible record materialization / runtime auto-use / hako_alloc migration を閉じる。 |
| `tools/checks/k2_wide_arraybox_inline_record_autouse_pilot_guard.sh` | C209 の non-escaping packed ArrayBox auto-use pilot を固定し、private i64 column read seam だけを開きつつ public materialization / hako_alloc migration / backend lowering を閉じる。 |
| `tools/checks/k2_wide_aligned_small_metadata_packed_store_pilot_guard.sh` | C210 の aligned-small metadata packed-store pilot を固定し、`HakoAllocAlignedSmallMeta` が private i64-column seam を使えることを検証しつつ `.hako` source / huge metadata / public materialization / backend lowering へ広がらないことを検証する。 |
| `tools/checks/k2_wide_huge_page_metadata_packed_store_pilot_guard.sh` | C211 の huge-page metadata packed-store pilot を固定し、`HakoAllocHugePageMeta` が private i64-column seam を使えることと live/sentinel 契約を検証しつつ `.hako` source / small-page state / public materialization / backend lowering へ広がらないことを検証する。 |
| `tools/checks/k2_wide_packed_record_backend_failfast_guard.sh` | C212 の packed record backend fail-fast gate を固定し、backend call site が shared MIR capability gate を使い、future required packed record routes が unsupported backend で silent fallback しないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_metadata_verifier_invariants_guard.sh` | C194 の hako_alloc metadata verifier invariants を固定し、C210/C211 metadata rows の source pilot / column order / materialization closure / huge released sentinel が MIR verifier で fail-fast することを検証する。 |
| `tools/checks/k2_wide_hako_alloc_stats_surface_guard.sh` | M191 の hako_alloc stats surface を固定し、`HakoAllocProductionFacade.statsSnapshot()` が既存 facade/page observers から read-only snapshot を返し、allocator behavior/options/provider/backend vocabulary へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_purge_policy_inventory_guard.sh` | M192 の purge/decommit policy inventory を固定し、空 retired page の候補判定だけを開き、page-source/OSVM release 実行や `.inc` matcher に広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_purge_dry_run_guard.sh` | M193 の purge/decommit dry-run observer を固定し、既存 OSVM-backed heap page/backing 観測を M192 policy に委譲しつつ page-source/OSVM release 実行や `.inc` matcher に広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_purge_execution_failfast_guard.sh` | M194 の purge/decommit execution fail-fast entry を固定し、実行入口は作るが missing/ineligible/eligible decision のすべてを blocked report に止め、page-source/OSVM release 実行や `.inc` matcher に広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_bounded_decommit_policy_guard.sh` | M195 の bounded decommit execution policy を固定し、eligible/in-bound decision だけが caller-provided `decommitPage` executor を1回呼び、unreserve/OS release や `.inc` matcher に広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_page_source_decommit_adapter_guard.sh` | M196 の page-source decommit adapter を固定し、M195 bounded policy が `HakoAllocPageSourcePolicy.decommitPage` にだけ接続され、reserve/commit/unreserve/OS release や `.inc` matcher に広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_purge_heap_decommit_guard.sh` | M197 の purge decommit heap integration を固定し、dry-run observation + bounded policy + page-source adapter の合成だけを開き、heap mutation/unreserve/OS release や `.inc` matcher に広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_purge_decommit_state_marker_guard.sh` | M198 の hako_alloc purge decommit state marker を固定し、成功decommit reportのpage id記録と重複mark拒否、unreserve/OS release 不在を検証する。 |
| `tools/checks/k2_wide_hako_alloc_purge_state_aware_duplicate_guard.sh` | M199 の hako_alloc purge state-aware duplicate guard を固定し、M198 marker による source 実行前の重複decommit防止と unreserve/OS release 不在を検証する。 |
| `tools/checks/k2_wide_hako_alloc_decommitted_page_reuse_precondition_guard.sh` | M200 の hako_alloc decommitted page reuse precondition を固定し、decommit済みpageがrecommit前に再利用不可であることと page-source 実行不在を検証する。 |
| `tools/checks/k2_wide_hako_alloc_recommit_failfast_guard.sh` | M201 の hako_alloc recommit fail-fast entry を固定し、recommit attempt が blocked/report-only に留まり actual recommit / page-source 実行へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_bounded_recommit_policy_guard.sh` | M202 の hako_alloc bounded recommit policy を固定し、caller-provided commit source のみを bounded に呼び、page-source adapter / marker clear / heap mutation へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_page_source_recommit_adapter_guard.sh` | M203 の hako_alloc page-source recommit adapter を固定し、bounded recommit policy から `commitPage` だけを委譲し、marker transition / heap mutation / unreserve / OS release へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_recommit_marker_transition_guard.sh` | M204 の hako_alloc recommit marker transition を固定し、recommit成功後に marker state が generation-counted に戻る一方で heap/page mutation や物理削除へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_recommit_heap_integration_guard.sh` | M205 の hako_alloc recommit heap integration を固定し、M200-M204 を合成して page-local reactivation だけを開き、page sourcing / unreserve / OS release へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_reuse_proof_closeout_guard.sh` | M206 の hako_alloc reuse proof closeout を固定し、decommit duplicate guard と recommit heap integration の2世代再利用ループが pure-first EXE で通ることを検証する。 |
| `tools/checks/k2_wide_hako_alloc_page_lifecycle_invariant_guard.sh` | M207 の hako_alloc page lifecycle invariant freeze を固定し、active/retired/decommitted/recommitted-active の状態表を read-only observer と pure-first EXE proof で検証する。 |
| `tools/checks/k2_wide_hako_alloc_page_lifecycle_verifier_invariants_guard.sh` | C194b の hako_alloc page lifecycle verifier invariants を固定し、M207 で凍結した lifecycle report/function surface が MIR verifier で fail-fast しつつ既存 proof が崩れないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_heap_reuse_priority_policy_guard.sh` | M208 の hako_alloc heap reuse priority policy を固定し、active > recommitted-active > retired-reactivate > fresh の優先順と decommitted page の ineligible stop lineが VM / pure-first EXE で一致することを検証する。 |
| `tools/checks/k2_wide_hako_alloc_lifecycle_stats_observer_surface_guard.sh` | M209 の hako_alloc lifecycle stats observer surface を固定し、M207/M208 の read-only counters を snapshot 化しつつ behavior/options/provider/backend vocabulary へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_decommit_recommit_reuse_exe_hardening_guard.sh` | M210 の hako_alloc decommit/recommit/reuse EXE hardening を固定し、M195-M209 lifecycle/reuse/stats composition が pure-first EXE で安定しつつ provider/hook/OS release/backend shortcut へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_purge_candidate_policy_inventory_guard.sh` | M211 の hako_alloc purge candidate policy inventory を固定し、M207 lifecycle report を read-only に分類しつつ scheduler/decommit/recommit/page-source/provider/backend vocabulary へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh` | M212 の hako_alloc bounded purge/decommit scheduler small path を固定し、bounded scan から M207/M211/M199 seam だけを通して1候補だけ実行しつつ直接M197/M195/page-source/provider/backend vocabulary へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh` | M213 の hako_alloc abandoned/reclaim inventory を固定し、abandoned/reclaim vocabulary を read-only に分類しつつ thread scheduling / atomics / reclaim execution / OS release / provider/backend vocabulary へ広がらないことを検証する。 |
| `tools/checks/k2_wide_allocator_metadata_record_declarations_guard.sh` | C205a の hako_alloc allocator metadata record 宣言を固定し、M178/M180 scalar columns が runtime truth のまま残ることを検証する。 |
| `tools/checks/k2_wide_allocator_record_construction_read_guard.sh` | C205b の builder-local record construction/read scalarization を固定し、record が `NewBox` / typed-object / backend / hako_alloc live migration に漏れないことを検証する。 |
| `tools/checks/k2_wide_aligned_small_metadata_record_store_guard.sh` | C205c/C206a の aligned-small metadata record store を固定し、M178 owner が record-shaped store と単一 `findIndex` lookup seam に委譲しつつ packed ArrayBox / backend / huge migration に広がらないことを検証する。 |
| `tools/checks/k2_wide_huge_page_metadata_record_store_guard.sh` | C205d の huge-page metadata record store を固定し、M180 owner が record-shaped store に委譲しつつ packed ArrayBox / backend / small-page state migration に広がらないことを検証する。 |
| `tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh` | M25 の `OsVmCoreBox.reserve_bytes_i64/commit_bytes_i64/decommit_bytes_i64` + pure-first EXE proof を固定し、OSVM route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh` | M26 の `TlsCoreBox.cache_slot_get_i64/cache_slot_set_i64` + pure-first EXE proof を固定し、TLS cache-slot route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh` | M27 の `AtomicCoreBox.cas_i64` + pure-first EXE proof を固定し、atomic CAS route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh` | M28 の `AtomicCoreBox.load_i64` + pure-first EXE proof を固定し、atomic load route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh` | M29 の `AtomicCoreBox.store_i64` + pure-first EXE proof を固定し、atomic store route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh` | M30 の `AtomicCoreBox.fetch_add_i64` + pure-first EXE proof を固定し、atomic fetch-add route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh` | M31 の fixed-slot i64 remote-free sketch + pure-first EXE proof を固定し、既存 atomic route facts の合成で LIFO push が動くことを検証する。 |
| `tools/checks/k2_wide_atomic_memory_order_args_vocab_guard.sh` | M33 の ordered fixed-slot i64 atomic facade/route vocabulary を docs-only で固定し、active source / `.inc` / NyRT に ordered implementation row が混入しないことを検証する。 |
| `tools/checks/k2_wide_pointer_atomic_vocab_guard.sh` | M34 の native-pointer atomic load/store/CAS facade/route vocabulary を固定し、M35/M39/M40/M41/M42/M43 以降も pointer fetch_add implementation row が混入しないことを検証する。 |
| `tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh` | M35 の direct native-pointer atomic store route + pure-first EXE proof を固定し、`hako_atomic_ptr_store_ordered` が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh` | M36 の TLS cache-slot + direct native-pointer atomic store composition proof を固定し、remote-free mailbox seam が既存 route facts だけで pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh` | M37 の allocator remote-free policy integration proof を固定し、`AllocatorRemoteFreePolicy` が既存 TLS/pointer-store route facts だけで pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_mimalloc_allocator_closeout_guard.sh` | M38 の mimalloc allocator app closeout coverage を固定し、M20-M37 proof apps / guards / docs index / dev_gate allocator-wide の導線が欠けていないことと、app-specific `.inc` matcher がないことを検証する。 |
| `tools/checks/k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh` | M39 の direct native-pointer atomic load route + pure-first EXE proof を固定し、`hako_atomic_ptr_load_ordered` が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh` | M40 の direct native-pointer atomic CAS route + pure-first EXE proof を固定し、`hako_atomic_ptr_cas_ordered` が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh` | M41 の pointer store/load/CAS composition proof を固定し、既存 route facts だけで two-node remote-free list push が pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh` | M42 の allocator remote-free list policy proof を固定し、M41 の two-node push shape が same-module policy box 経由で pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh` | M43 の allocator remote-free retry-loop proof を固定し、same-module policy box 内の bounded CAS retry loop が pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh` | M44 の mimalloc allocator substrate closeout を固定し、M20-M43 proof apps/guards/docs/allocator-wide coverage と app-specific `.inc` matcher 不在を検証する。 |
| `tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh` | M45 の production allocator port entry plan を固定し、M46-M50 の実装順・境界・pointer fetch_add/native attrs inactive を検証する。 |
| `tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh` | M46 の `hako_alloc` production facade boundary を固定し、`HakoAllocProductionFacade` が pure-first EXE で既存 page/free-list policy state へ委譲することを検証する。 |
| `tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh` | M47 の allocator local page policy proof を固定し、`HakoAllocProductionFacade` 経由で small/medium allocate/free/reject/reuse accounting が pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh` | M48 の allocator remote-free policy proof を固定し、`HakoAllocProductionFacade` 経由で bounded CAS retry-loop remote-free policy が pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh` | M49 の allocator OSVM page-source proof を固定し、`HakoAllocProductionFacade` 経由で reserve/commit/decommit が pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh` | M50 の allocator stress production-facade parity を固定し、既存 allocator-stress の accounting shape が `HakoAllocProductionFacade` 経由で pure-first EXE 実行できることを検証する。 |
| `tools/checks/k2_wide_production_allocator_port_closeout_guard.sh` | M51 の production allocator port closeout を固定し、M46-M50 の app/guard/docs/dev_gate allocator-wide coverage と inactive allocator rows を検証する。 |
| `tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh` | M52 の allocator replacement hook boundary を固定し、HookPlan/owner SSOT と process allocator replacement / hook env / `.inc` name matching 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh` | M53 の allocator HookPlan vocabulary を固定し、reserved HookPlan v0 docs/TOML fixture と runtime hook / process allocator replacement / `.inc` name matching 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh` | M54 の allocator hook runtime dry-run boundary を固定し、diagnostic-only runtime seam と runtime hook code / process allocator replacement / `.inc` name matching 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh` | M55 の allocator hook activation proof vocabulary を固定し、reserved activation proof docs/TOML fixture と runtime hook / process allocator replacement / `.inc` name matching 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh` | M56 の allocator hook runtime owner row を固定し、future owner path の命名と runtime hook code / process allocator replacement / `.inc` name matching 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh` | M57 の allocator hook runtime dry-run code を固定し、diagnostic-only runtime validation と process allocator replacement / `.inc` name matching 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh` | M58 の allocator hook dry-run manifest callsite を固定し、reserved TOML text input と file/env discovery / process allocator replacement / `.inc` name matching 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh` | M59 の allocator hook dry-run test surface を固定し、`#[cfg(test)]` reserved-fixture observation と CLI/env/file discovery 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_activation_proof_validator_guard.sh` | M60 の allocator hook activation proof validator を固定し、reserved activation-proof TOML text validation と activation/env/CLI/file discovery 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh` | M61 の allocator hook dry-run CLI surface を固定し、明示 plan/proof file diagnostic と env/implicit discovery/activation 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh` | M62 の allocator hook activation preflight boundary を固定し、activation proof handoff と process allocator replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh` | M63 の allocator hook activation preflight shape を固定し、diagnostic-only runtime facts/report と activation 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh` | M64 の allocator provider boundary vocabulary を固定し、provider ids と provider registry/selection/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh` | M65 の allocator provider manifest vocabulary を固定し、reserved provider TOML fixture と runtime parser/registry/selection 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh` | M66 の allocator provider task breakdown を固定し、M67-M75 task ladder と provider/replacement inactive stop line を検証する。 |
| `tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh` | M67 の allocator provider manifest diagnostic parser を固定し、caller-provided TOML text parser/report と provider registry/selection/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh` | M68 の allocator provider manifest CLI surface を固定し、明示 provider manifest file diagnostic と env/implicit discovery/selection/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh` | M69 の allocator provider readiness preflight shape を固定し、provider manifest readiness と hook activation preflight diagnostics の接続、および selection/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh` | M70 の combined hook/provider dry-run report を固定し、明示 hook plan/proof/provider manifest の合成診断と install/selection/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh` | M71 の allocator provider registry boundary docs を固定し、future registry owner/API shape と active registry/selection/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh` | M72 の hako model provider proof fixture を固定し、reserved `.hako` policy/model provider proof と selection/native activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh` | M73 の debug guarded provider proof fixture を固定し、reserved guarded-provider diagnostic proof と selection/hook activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh` | M74 の native system provider proof boundary を固定し、reserved system allocator ABI proof と `#[global_allocator]`/selection/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh` | M75 の native mimalloc provider proof boundary を固定し、reserved mimalloc provider proof と production activation/selection/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh` | M76 の allocator provider activation entry contract を固定し、future registry/selection ownership・proof consumption・rollback contract と activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh` | M77 の allocator provider registry snapshot diagnostic shape を固定し、reserved provider entries と selection/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh` | M78 の allocator provider selection decision diagnostic shape を固定し、caller-provided request/decision・no selected provider と selection/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh` | M79 の allocator provider proof bundle consumption diagnostic shape を固定し、selected-provider proof inputs と runtime proof consumption/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh` | M80 の allocator provider rollback preflight diagnostic shape を固定し、rollback target facts と rollback preparation/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh` | M81 の allocator provider activation safety gate diagnostic shape を固定し、activation evidence bundle と gate opening/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh` | M82 の allocator provider activation safety diagnostic owner を固定し、runtime owner と過去 guard の future-compatible 化、gate opening/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh` | M83 の allocator provider activation safety diagnostic report を固定し、runtime report と gate-closed output、gate opening/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh` | M84 の allocator provider activation safety CLI surface を固定し、明示 TOML path 診断と env/implicit discovery/gate opening/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh` | M85 の allocator provider activation safety closeout inventory を固定し、M76-M84 の SSOT/fixture/card/guard coverage と activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh` | M86 の allocator provider activation decision surface proposal を固定し、future explicit-input contract と hidden env/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh` | M87 の allocator provider activation decision fixture contract を固定し、reserved TOML と selection/proof/rollback/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_owner_guard.sh` | M88 の allocator provider activation decision diagnostic owner を固定し、future runtime owner と過去 guard の future-compatible 化、selection/proof/rollback/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh` | M89 の allocator provider activation decision diagnostic report を固定し、caller-provided TOML report と selection/proof/rollback/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_decision_cli_surface_guard.sh` | M90 の allocator provider activation decision CLI surface を固定し、明示 TOML path 診断と env/implicit discovery/selection/proof/rollback/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_decision_closeout_guard.sh` | M91 の allocator provider activation decision closeout inventory を固定し、M86-M90 の SSOT/fixture/card/guard coverage と selection/proof/rollback/activation/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh` | M92 の allocator provider activation implementation entry contract を固定し、単一 future owner/entry と selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh` | M93 の allocator provider registry snapshot diagnostic report を固定し、caller-provided TOML report と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh` | M93B の allocator provider diagnostic inactive actions cleanup を固定し、M83/M89/M93 の false output SSOT と past guard latest-card pin 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh` | M94 の allocator provider registry snapshot CLI surface を固定し、明示 TOML path 診断と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh` | M95 の allocator provider activation diagnostic closeout inventory を固定し、M92-M94/M93B の coverage と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh` | M96 の allocator provider selection decision diagnostic report を固定し、caller-provided TOML report と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_selection_decision_cli_surface_guard.sh` | M97 の allocator provider selection decision CLI surface を固定し、明示 TOML path 診断と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_diagnostic_helper_cleanup_guard.sh` | M97B の allocator provider diagnostic helper cleanup を固定し、TOML helper / fact-check 共有化と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_diagnostic_report_guard.sh` | M98 の allocator provider proof bundle consumption diagnostic report を固定し、caller-provided TOML report と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_runtime_diagnostic_module_boundaries_guard.sh` | M98B の allocator provider runtime diagnostic module boundaries を固定し、registry facade の sub-1000 行化、report-owner module 分割、active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_cli_surface_guard.sh` | M99 の allocator provider proof bundle consumption CLI surface を固定し、明示 TOML path 診断と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh` | M100 の allocator provider proof bundle consumption entry contract を固定し、単一 future behavior owner/entry と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_proof_consumption_failfast_entry_guard.sh` | M101 の allocator provider proof consumption fail-fast entry を固定し、runtime attempt report と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_selected_provider_precondition_guard.sh` | M102 の allocator provider selected-provider precondition を固定し、caller-provided selected provider 検査と active registry/selection/proof/rollback/gate/hook/replacement 不在を検証する。 |
| `tools/checks/k2_wide_allocator_provider_proof_validation_guard.sh` | M103 の allocator provider selected-provider proof validation を固定し、proof validation guard が wide allocator gate に個別登録されていないことも検証する。 |
## Env Hygiene

| Script | Purpose |
| --- | --- |
| `tools/checks/env_dead_accessors_report.sh` | `src/config/env/*.rs` の dead accessor 候補と doc-only 候補をCSVで棚卸し。 |
| `tools/checks/route_env_probe.sh` | emit route 直前の Env / route 表示を確認する。 |
| `tools/checks/route_no_fallback_guard.sh` | 日常 route で fallback/helper トグルが混入していないことを fail-fast で検証する。 |

使い方:

```bash
tools/checks/env_dead_accessors_report.sh
```

出力列:
- `status`: `dead` / `doc-only`
- `module`, `function`: 対象 accessor
- `keys`: 関連ENVキー
- `src_hits`, `tools_hits`, `docs_hits`: 参照件数

## Inventory / Maintenance

| Script | Purpose |
| --- | --- |
| `tools/checks/smoke_inventory_report.sh` | 任意の smoke subtree の過密状態を可視化し、suite-aware coverage summary も出す。既定では `integration/apps` を見て、`archive/lib/tmp/fixtures` は live inventory から除外する。 |
| `tools/checks/windows_wsl_cmd_smoke.sh` | Windows(WSL→CMD) 経路の保守監査。 |
| `tools/checks/macos_portability_guard.sh` | macOS portability の継続監査。 |

## Update Policy

- 新しい `tools/checks/*.sh` を追加したら、この文書へ同コミットで追記する。
- script の役割変更時は `Purpose` を先に更新し、実装差分はその後に載せる。
- 日常導線は `dev_gate.sh` を最優先にし、単発スクリプトは理由があるときだけ直接実行する。
| `tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh` | M214 の hako_alloc allocator options/defaults inventory を固定し、read-only option/default facts を観測しつつ mutable options / env toggles / allocation policy changes / provider hooks / replacement / reclaim execution へ広がらないことを検証する。 |
| `tools/checks/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh` | M215 の hako_alloc thread heap owner-token inventory を固定し、owner-token facts を read-only に分類しつつ thread scheduling / atomic claim / remote-free drain / owner mutation / reclaim execution / page-source / OS release へ広がらないことを検証する。 |
| `tools/checks/k2_wide_loop_range_parser_capsule_guard.sh` | LOOP-002 の Stage0 LoopRange parser capsule を固定し、`loop i in start..end` / `loop(i in start..end)` / paren-less `loop cond` を parser + metadata transport に閉じ、Stage1 lowering / readonly index / continue-safe step / bounds facts へ広がらないことを検証する。 |
- `tools/checks/k2_wide_delegate_parser_capsule_guard.sh` - DEL-002 Stage0 delegate parser/metadata capsule guard.
- `tools/checks/k2_wide_delegate_exposes_lowering_guard.sh` - DEL-003 delegate exposes Stage1 forwarding lowering guard.
- `tools/checks/k2_wide_brand_parser_capsule_guard.sh` - BRAND-001 Stage0 brand declaration parser/metadata capsule guard.
- `tools/checks/k2_wide_brand_constructor_unwrap_guard.sh` - BRAND-002 Stage1 brand constructor/unwrap Program JSON v0 policy guard.
- `tools/checks/k2_wide_brand_mismatch_checker_guard.sh` - BRAND-003 Stage1 brand mismatch checker guard.
- `tools/checks/k2_wide_type_alias_parser_capsule_guard.sh` - TYPE-001 Stage0 type alias parser/metadata capsule guard.
- `tools/checks/k2_wide_record_literal_parser_capsule_guard.sh` - REC-001 Stage0 record literal parser/metadata capsule guard.
- `tools/checks/k2_wide_record_construction_read_lowering_guard.sh` - REC-002 Stage1 record literal shape validation and record field read lowering guard.
- `tools/checks/k2_wide_record_with_update_lowering_guard.sh` - REC-003 record with-update parser/metadata and Stage1 replacement lowering guard.

- `tools/checks/k2_wide_astclean_normalize_logical_ops_guard.sh` — guards ASTCLEAN-002 by requiring one parser-owned `normalize_logical_ops` helper and rejecting nested duplicate copies.

- `tools/checks/k2_wide_astclean_parser_depth_noop_guard.sh` — guards ASTCLEAN-003 by rejecting legacy parser depth no-op hook references.

- `tools/checks/k2_wide_astclean_dead_code_inventory_guard.sh` — guards ASTCLEAN-004 by fixing the `#[allow(dead_code)]` source baseline and next-row split.

- `tools/checks/k2_wide_astclean_mir_typeregistry_dead_code_guard.sh` — guards ASTCLEAN-005 by pruning used TypeRegistry `dead_code` allowances and requiring row reasons for retained ones.

- `tools/checks/k2_wide_astclean_numeric_substrate_dead_code_rationale_guard.sh` — guards ASTCLEAN-006 by requiring rationale comments for numeric substrate staged `dead_code` allowances.

- `tools/checks/k2_wide_astclean_mir_loops_duplicate_dead_code_guard.sh` — guards ASTCLEAN-007 by rejecting duplicate adjacent `dead_code` allowances in MIR loop utilities.

- `tools/checks/k2_wide_astclean_test_dev_dead_code_guard.sh` — guards ASTCLEAN-008 by deleting a legacy VM benchmark stub and requiring reasons for retained test/dev `dead_code` allowances.

- `tools/checks/k2_wide_astclean_backend_optimizer_dead_code_guard.sh` — guards ASTCLEAN-009 by pruning backend utility wrappers, deleting stale optimizer diagnostics, and requiring rationales for retained backend utility `dead_code` allowances.

- `tools/checks/k2_wide_astclean_runner_json_bridge_helper_guard.sh` — guards ASTCLEAN-010 by rejecting stale runner JSON bridge wrappers and capping source `dead_code` allowances.

- `tools/checks/k2_wide_astclean_runner_exec_stale_allow_guard.sh` — guards ASTCLEAN-011 by removing stale `dead_code` allowances from live runner exec backend APIs.

- `tools/checks/k2_wide_astclean_host_provider_rationale_guard.sh` — guards ASTCLEAN-012 by requiring Phase 291x-126 rationale comments on hako-ll host-provider staged `dead_code` module allowances.

- `tools/checks/k2_wide_astclean_mir_builder_loops_stale_module_guard.sh` — guards ASTCLEAN-013 by deleting the stale MIR builder loop helper module and capping source `dead_code` allowances.

- `tools/checks/k2_wide_astclean_mir_builder_scope_local_guard.sh` — guards ASTCLEAN-014 by deleting stale MIR builder scope/local helper methods and capping source `dead_code` allowances.

- `tools/checks/k2_wide_astclean_mir_builder_utility_shelf_guard.sh` — guards ASTCLEAN-015 by deleting stale MIR builder utility wrappers and capping source `dead_code` allowances.

- `tools/checks/k2_wide_astclean_call_resolution_duplicate_helper_guard.sh` — guards ASTCLEAN-016 by removing duplicate call-resolution warning helpers and capping source `dead_code` allowances.
- `tools/checks/k2_wide_astclean_runner_provider_runtime_guard.sh` — guards ASTCLEAN-017 by pruning runner/provider/runtime helper shelves and requiring rationale comments for retained optional surfaces.

## Phase 293x mimalloc blueprint guards

- `tools/checks/k2_wide_mimalloc_page_free_list_pilot_guard.sh` - `MIMAP-008` direct page/free-list pilot proof guard.
- `tools/checks/k2_wide_mimalloc_lifecycle_integration_pilot_guard.sh` - `MIMAP-009` page-local decommit/recommit/reuse lifecycle pilot guard.
- `tools/checks/k2_wide_mimalloc_page_queue_lifecycle_selection_guard.sh` - `MIMAP-010` lifecycle-aware page queue selection pilot guard.
- `tools/checks/k2_wide_mimalloc_backend_acceptance_policy_guard.sh` - MIMAP VM/LLVM backend acceptance split, VM known limitation registry, and VM timeout guard policy.
- `tools/checks/k2_wide_mimalloc_facade_lifecycle_route_exe_guard.sh` - `MIMAP-011` LLVM/EXE-primary allocator facade lifecycle selection route guard.
- `tools/checks/vm_lim_001_object_queue_identity_probe.sh` - diagnostic probe for `VM-LIM-001` ArrayBox-held InstanceBox/page object identity across push/get/method receiver.

| `tools/checks/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh` | MIMAP-012 の object-backed lifecycle queue LLVM/EXE-primary proof を固定し、ArrayBox-held page object selection が page lifecycle methods を通りつつ VM を blocker にしないことを検証する。 |
| `tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh` | MIMAP-013 の thin facade object-backed lifecycle queue 合成を固定し、`HakoAllocObjectLifecycleFacade` が page object add、selection 実行、selected identity / queue counter の read-only scalar observer を `HakoAllocObjectLifecyclePageQueue` に委譲して LLVM/EXE primary で実行できることを検証する。 |
| `tools/checks/k2_wide_mimap012_object_loop_row_a_exe_guard.sh` | MIR-ROW-A の `loop + if guard + pages.get(i)` 最小 object-loop 診断。dynamic `ArrayBox.get(i)` 後の user-box receiver facts が LLVM/EXE route まで保たれることを検証する。 |
