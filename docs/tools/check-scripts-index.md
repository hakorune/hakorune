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
| `tools/checks/dev_gate.sh` | 日常ゲートの統合実行（quick/hotpath/portability/milestone）。 |
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
| `tools/checks/k2_wide_mimalloc_size_class_table_exe_guard.sh` | M21 の static u16 size-class table + raw-page pure-first EXE 合成 proof を固定し、runtime Array/Map table materialization、app-specific `.inc` matcher、新しい source syntax / allocator policy の混入を防ぐ。 |
| `tools/checks/k2_wide_mimalloc_two_class_page_exe_guard.sh` | M22 の static u16 size-class table + two-class raw-page pure-first EXE proof を固定し、small/medium の reject/release/reuse が既存 route surface だけで動くことを検証する。 |
| `tools/checks/k2_wide_mimalloc_dynamic_bin_exe_guard.sh` | M23 の runtime-indexed static u16 size-class table + raw-page pure-first EXE proof を固定し、非定数 `static_data_load` index が app-specific `.inc` matcher なしで動くことを検証する。 |
| `tools/checks/k2_wide_mimalloc_size_to_bin_inline_exe_guard.sh` | M24 の `Profile(allocator.fast)` size_to_bin inline + runtime-indexed static u16 table + raw-page pure-first EXE proof を固定し、backend/.inc が profile 名を読まないことを検証する。 |
| `tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh` | M25 の `OsVmCoreBox.reserve_bytes_i64/commit_bytes_i64/decommit_bytes_i64` + pure-first EXE proof を固定し、OSVM route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh` | M26 の `TlsCoreBox.cache_slot_get_i64/cache_slot_set_i64` + pure-first EXE proof を固定し、TLS cache-slot route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh` | M27 の `AtomicCoreBox.cas_i64` + pure-first EXE proof を固定し、atomic CAS route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh` | M28 の `AtomicCoreBox.load_i64` + pure-first EXE proof を固定し、atomic load route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh` | M29 の `AtomicCoreBox.store_i64` + pure-first EXE proof を固定し、atomic store route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh` | M30 の `AtomicCoreBox.fetch_add_i64` + pure-first EXE proof を固定し、atomic fetch-add route が MIR-owned extern route facts から emit されることを検証する。 |
| `tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh` | M31 の fixed-slot i64 remote-free sketch + pure-first EXE proof を固定し、既存 atomic route facts の合成で LIFO push が動くことを検証する。 |

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
