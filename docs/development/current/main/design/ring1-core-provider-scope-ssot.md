# Ring1 Core Provider Scope SSOT

Status: SSOT  
Date: 2026-02-19  
Owner: runtime lane (`phase-29y`)

## Purpose

- `ring1` の責務を「静的・最小・信頼できる provider」に固定する。
- `file/array/map/path/console` の適用範囲を明示し、placeholder 運用の曖昧さをなくす。
- 実装追加時に「どこを触るべきか」を 1 枚で辿れるようにする。
- collection current-truth と `0rust` cutover は `docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md` を参照する。

## Decision Matrix

| Domain | Decision | Current State | Runtime Wiring |
| --- | --- | --- | --- |
| `file` | `accepted` | 実装済み（`ring0_fs_fileio` / `core_ro` / `nofs_fileio`） | `provider_lock::new_filebox_provider_instance` が SSOT |
| `array` | `accepted` | runtime lane は `Ring1ArrayService` で wired、AOT/LLVM lane は still-Rust keep が残る | `provider_lock::{set_arraybox_provider,new_arraybox_provider_instance}` + `PluginHost` 初期化が SSOT |
| `map` | `accepted` | runtime lane は `Ring1MapService` で wired、AOT/LLVM lane は still-Rust keep が残る | `provider_lock::{set_mapbox_provider,new_mapbox_provider_instance}` + `PluginHost` 初期化が SSOT |
| `path` | `accepted` | 実装済み（`Ring1PathService` + `PathBox` runtime consumer） | `provider_lock::{set_pathbox_provider,get_pathbox_provider_instance}` + `PluginHost` + `boxes_path` が SSOT |
| `console` | `accepted` | 実装済み（`Ring1ConsoleService`） | `provider_lock::{set_consolebox_provider,new_consolebox_provider_instance}` + `PluginHost` 初期化が SSOT |

## Scope Contract

1. `ring1` は `ring2`（plugin）へ依存しない。
2. `ring1` は「意味決定」を持たず、薄い provider 実装に限定する。
3. `provisional` ドメインは runtime 配線を追加しない。README のみを正とする。
4. `accepted` へ昇格するときは、以下を同一タスクで固定する。

- 実装: `src/providers/ring1/<domain>/`
- 配線: runtime 側 SSOT（`provider_lock` など）
- 契約: fixture + smoke + guard
- 文書: この SSOT と `CURRENT_TASK.md`

## Collection Current Truth Note

- `array` / `map` が `accepted` であることは、「domain は ring1 に固定された」を意味する。
- それは「mainline concrete implementation がすでに `.hako` owner だけで動いている」を意味しない。
- current truth:
  - runtime/provider lane は `src/providers/ring1/{array,map}/mod.rs` の Rust provider 実装で wired
  - AOT/LLVM lane は `crates/nyash_kernel/src/plugin/{array,map,runtime_data}.rs` と `crates/nyash_kernel/src/exports/birth.rs` の still-Rust keep が残る
  - `.hako` 側 `lang/src/runtime/collections/**` と `lang/src/vm/boxes/abi_adapter_registry.hako` は thin wrapper / adapter owner
- `0rust` target は `.hako ring1` owner への cutover であり、`ring0` へ collection semantics を移すことではない。

## Code Pointers

- `src/providers/ring1/README.md`
- `src/providers/ring1/file/mod.rs`
- `src/providers/ring1/array/mod.rs`
- `src/providers/ring1/map/mod.rs`
- `src/providers/ring1/path/mod.rs`
- `src/providers/ring1/console/mod.rs`
- `src/runtime/provider_lock/mod.rs`
- `src/runtime/provider_lock/console.rs`
- `src/runtime/plugin_host.rs`
- `src/boxes/file/mod.rs`
- `src/boxes/path_box.rs`
- `src/backend/mir_interpreter/handlers/boxes_path.rs`

## Guard

- `tools/checks/ring1_core_scope_guard.sh`
- lane gate integration: `tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`

## Promotion Procedure

- `provisional -> accepted` の昇格手順は次を正本とする:
  - `docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md`
