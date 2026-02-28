---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: 29cc-221 の Non-target（keep-for-now）群を source-zero final wave 向けに棚卸しし、実装順を decision-complete に固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - crates/nyash_kernel/src/plugin/array.rs
  - crates/nyash_kernel/src/plugin/string.rs
  - crates/nyash_kernel/src/plugin/map.rs
  - crates/nyash_kernel/src/plugin/console.rs
  - crates/nyash_kernel/src/plugin/intarray.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/plugin/handle_helpers.rs
---

# 29cc-244 Final-Wave Non-target Discovery Lock

## Purpose

`29cc-221` の Non-target 群を「どれから切るか」「どこが高リスクか」で迷走しないよう、呼び出し経路・ABI契約・移植難度を 1 枚に固定する。

## Decision Summary

- すべての実装順は `1 boundary = 1 commit` を維持する。
- final wave の先頭は `handle_helpers.rs` と `module_string_dispatch.rs` を優先する。
- `array/map/string/console/intarray` は上記基盤の後に切る。
- no-delete-first を維持し、compat default-off は変更しない。

## Integrated Inventory (decision-complete)

| Module | Role | Main callers | ABI touchpoints / contract | Compat dependency | Complexity | Recommended order |
|---|---|---|---|---|---|---|
| `handle_helpers.rs` | handle cache / typed box bridge | `array.rs`, `map.rs`, `instance.rs`, `runtime_data.rs` | `with_array_box/with_map_box/with_instance_box` が `None` を返す fail-safe 契約。drop epoch + TLS cache を保持。 | cache miss / stale handle は silent `None` | High | 1 |
| `module_string_dispatch.rs` | Stage1 module string dispatch | `invoke/by_name.rs` | `try_dispatch` + encode/decode string handle。契約違反は `[freeze:contract]` 文字列ハンドルで返す。 | `HAKO_STAGE1_MODULE_DISPATCH_TRACE` | High | 2 |
| `array.rs` | array handle API exports | tests, lowerers, ABI clients | `nyash.array.*` / alias exports。legacy `set_h` は常時 `0`（互換）で、hh/hi ルートは成否を返す。 | `NYASH_CLI_VERBOSE` と legacy set contract | High | 3 |
| `map.rs` | map handle API exports | tests, JIT/LLVM lowering | `nyash.map.*` exports。`set_h` は互換 `0` 返却、`has/get` は失敗時 `0`。 | `NYASH_LLVM_MAP_DEBUG` と legacy set contract | Medium-High | 4 |
| `string.rs` | string C ABI + handle helpers | tests, lowering | `concat/substring/length/to_i8p_h`。null/invalid UTF-8 は null or sentinel return。 | legacy `concat_si/is` shim 維持 | Medium | 5 |
| `intarray.rs` | numeric int-array API | hako runtime intarray core | `new_h/len_h/get_hi/set_hii`。setterは `0/1` 契約。 | `NYASH_CLI_VERBOSE` | Medium | 6 |
| `console.rs` | logging / readline exports | C shim, runtime logging paths | `nyash.console.*`。invalid handle は fail-safe（数値出力 or empty）。 | legacy `print` alias | Low | 7 |

## Commit Slices (fixed)

### Commit 1: Handle cache foundation boundary
- Scope: `crates/nyash_kernel/src/plugin/handle_helpers.rs` only
- Goal: cache/epoch/typed helper 契約を source-zero final wave 向けに固定
- Acceptance:
  - `handle_helpers.rs` の既存 test (`cache_invalidation_on_drop_epoch`, `array_or_map_route_detection`) green
  - array/map/instance/runtime_data の呼び出し点に契約変更が波及しない

### Commit 2: Stage1 dispatch boundary
- Scope: `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` only
- Goal: Stage1 module dispatch の handle/JSON 契約を固定
- Acceptance:
  - `invoke/by_name.rs` 経路で `try_dispatch` が同一契約を維持
  - freeze contract (`[freeze:contract]`) を崩さない

### Commit 3: Array API boundary
- Scope: `crates/nyash_kernel/src/plugin/array.rs` only
- Goal: `nyash.array.*` 系の mainline/compat 契約を固定
- Acceptance:
  - underscore export / dotted alias / runtime_data alias すべて解決可能
  - legacy `set_h -> 0` 契約を維持
  - array get/set/push/length の失敗時 fail-safe を維持

### Commit 4: Map API boundary
- Scope: `crates/nyash_kernel/src/plugin/map.rs` only
- Goal: `nyash.map.*` の helper境界と invalid handle fail-safe 契約を固定
- Acceptance:
  - `set_h/set_hh` の completion code `0` 維持
  - `size/get/has` の invalid handle `0` 維持

### Commit 5: String C-buffer boundary
- Scope: `crates/nyash_kernel/src/plugin/string.rs` only
- Goal: C-string 生成経路の重複縮退と `to_i8p_h` fallback 契約固定
- Acceptance:
  - C buffer helper が単一路線（挙動不変）
  - `to_i8p_h` invalid/missing handle で numeric string を返す

### Commit 6: IntArray boundary
- Scope: `crates/nyash_kernel/src/plugin/intarray.rs` only
- Goal: downcast境界を集約し、`len/get/set` の bounds/invalid 契約を固定
- Acceptance:
  - `set_hii`: success=`0` / fail=`1`
  - `len_h/get_hi`: invalid/bounds で `0`

### Commit 7: Console boundary
- Scope: `crates/nyash_kernel/src/plugin/console.rs` only
- Goal: handle文字列化/出力 prefix 重複の縮退と null-safe export 契約固定
- Acceptance:
  - `log/warn/error/trace` の null pointer 入力は `0`
  - handle系 API は fail-safe return `0` を維持

## Reopen Criteria (failure-driven)

- 以下のどれかが発生したときのみ final wave を reopen する:
  1. runtime/plugin route guard fail
  2. ABI contract drift（戻り値/失敗時挙動の差）
  3. Stage1 dispatch freeze contract drift

## Assumptions

- この lock は「順序固定」のための discovery lock であり、source 削除は扱わない。
- mac/windows の再ビルド保守のため、Rust source は当面残置する。

## Execution Status (2026-02-28)

- [x] Commit 1 done: `handle_helpers.rs` boundary fixed
  - commit: `e8e9e2d79`
  - contracts pinned: invalid handle short-circuit / array index fail-safe
- [x] Commit 2 done: `module_string_dispatch.rs` boundary fixed
  - commit: `ea54764df`
  - contracts pinned: unknown route `None` / freeze decode contracts
- [x] Commit 3 done: `array.rs` boundary fixed
  - commit: `a53c9a53d`
  - contracts pinned: legacy `set_h -> 0` + `hi/hii` fail-safe aliases
- [x] Commit 4 done: `map.rs` boundary fixed
  - commit: `5a575c503`
  - contracts pinned: helper dedupe + invalid handle fail-safe (`size/get/has -> 0`, `set -> 0`)
- [x] Commit 5 done: `string.rs` boundary fixed
  - commit: `ecd44c43d`
  - contracts pinned: C-buffer path dedupe + `to_i8p_h` fallback numeric string contract
- [x] Commit 6 done: `intarray.rs` boundary fixed
  - commit: `ca0d82dd0`
  - contracts pinned: downcast boundary SSOT + bounds/invalid return codes
- [x] Commit 7 done: `console.rs` boundary fixed
  - commit: `5f191ff25`
  - contracts pinned: handle text helper SSOT + null-safe export return
