---
Status: Active
Decision: provisional
Date: 2026-02-17
Scope: vm-hako subset-check（Rust）と mir_vm_s0 runtime（.hako）の boxcall 引数契約を1枚で固定する。
Related:
  - src/runner/modes/vm_hako/subset_check.rs
  - src/runner/modes/vm_hako/tests/boxcall_contract.rs
  - lang/src/vm/boxes/mir_vm_s0.hako
  - docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md
---

# VM-Hako Boxcall Contract SSOT

## Purpose

- `boxcall` の method ごとの引数形状を、Rust subset-check と `.hako` runtime で同じ基準に固定する。
- 受理/拒否境界のドリフトを防ぐ。

## Contract Table

| method | args shape | subset-check reject tag | runtime reject tag |
| --- | --- | --- | --- |
| `birth` | `0` | `boxcall(birth:args>0)` | `boxcall-missing-box` など contract tag |
| `push` | `1` | `boxcall(push:args!=1)` | `op=boxcall args>1` / `boxcall-arg0-missing` |
| `open` | `2` or `3` | `boxcall(open:args!=2or3)` | `op=boxcall args<2` / `boxcall-open-arg-missing` / `boxcall-open-handle-missing` |
| `read` | `0` or `1`（receiver mirror） | `boxcall(read:args>1)` / `boxcall(read:arg0:non-reg)` | `op=boxcall args>1` / `boxcall-read-file-missing` |
| `close` | `0` or `1`（receiver mirror） | `boxcall(close:args>1)` / `boxcall(close:arg0:non-reg)` | `op=boxcall args>1` / `boxcall-close-file-missing` |
| `length` | `0` | `boxcall(length:args!=0)` | `op=boxcall args>1` |
| `get` | `0` or `1`（receiver mirror / generic keep） | `boxcall(get:args>1)` / `boxcall(get:arg0:non-reg)` | `op=boxcall args>1` / `boxcall-get-arg0-missing` |
| `has` | `0` or `1`（receiver mirror / generic keep） | `boxcall(has:args>1)` / `boxcall(has:arg0:non-reg)` | `op=boxcall args>1` / `boxcall-has-arg0-missing` / `boxcall-has-key-handle-missing` |
| `set` | `2` | `boxcall(set:args!=2)` / `boxcall(set:args:non-reg)` | `op=boxcall args>2` / `boxcall-set-arg-missing` / `op=boxcall method=set` |
| `size` | `0` | `boxcall(size:args>1)` / `boxcall(size:arg0:non-reg)` | `op=boxcall args>1` |
| `keys` | `0` | `boxcall(keys:args>1)` / `boxcall(keys:arg0:non-reg)` | `op=boxcall args>1` / `op=boxcall0 method=keys` |
| `clear` | `0` | `boxcall(clear:args>1)` / `boxcall(clear:arg0:non-reg)` | `op=boxcall args>1` / `op=boxcall0 method=clear` |
| `indexOf` | `1` or `2` | `boxcall(indexOf:args!=1or2)` | `op=boxcall args>2` / `boxcall-indexOf-*` |
| `substring` | `2` | `boxcall(substring:args!=2)` | `op=boxcall args<2` / `op=boxcall args>2` / `boxcall-substring-*` |
| `compile_obj` | `1` | `boxcall(compile_obj:args>1)` / `boxcall(compile_obj:arg0:non-reg)` | `boxcall-arg0-missing` / `llvmbackend-compile-path-handle-missing` / `[llvmbackend/read/*]` / `[llvmbackend/emit/*]` |
| `link_exe` | `3` | `boxcall(link_exe:args!=3)` / `boxcall(link_exe:args:non-reg)` | `op=boxcall args<3` / `llvmbackend-link-*` / `[llvmbackend/link/*]` |
| (other methods) | `<=1` | `boxcall(args>1)` | `op=boxcall args>1` / method-specific unimplemented |

Current exact blocker note:
- `MapBox.set(key, value)` is now tracked as `RVP-C17` ported.
- `MapBox.size()` is now tracked as `RVP-C18` ported.
- `MapBox.get(key)` is now tracked as `RVP-C19` ported.
- `MapBox.has(key)` is now tracked as `RVP-C20` ported.
- `MapBox.delete(key)` is now tracked as `RVP-C21` ported.
- `MapBox.keys()` is now tracked as `RVP-C22` ported.
- `MapBox.clear()` is now tracked as `RVP-C23` ported.
- `MapBox.get(missing-key)` is now tracked as `RVP-C24` ported.
- The current exact blocker is `RVP-C25` (`MapBox.get(non-string key)` -> stale scalar `0`).
- Do not widen the generic rule silently; close `RVP-C25` with fixture + smoke + contract update in the same commit.

## Update Rule

- `boxcall` 引数契約を変更する場合は、次を同一コミットで更新する。
  - `src/runner/modes/vm_hako/subset_check.rs`
  - `src/runner/modes/vm_hako/tests/boxcall_contract.rs`
  - `lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako`（または該当する boxcall runtime owner）
- capability row（RVP-C11/C12）に影響する場合は `81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md` も同一コミットで同期する。
- `MapBox.set/get/has/size` のような collection-owner verbs を widen する場合も `81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md` を同一コミットで同期する。

## Validation

- unit: `cargo test -q boxcall_contract`
- gate: `bash tools/smokes/v2/profiles/integration/apps/phase29y_vm_hako_caps_gate_vm.sh`
