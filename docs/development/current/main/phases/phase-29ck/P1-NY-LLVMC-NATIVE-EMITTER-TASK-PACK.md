---
Status: Task Pack
Decision: accepted
Date: 2026-03-14
Scope: `ny-llvmc native` bootstrap seam と thin backend boundary cutover を `BE0-min1..min6` の単位に分け、1 blocker = 1 commit で進められる形にする。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29ck/P0-BACKEND-ZERO-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/phases/phase-29ck/P3-THIN-BACKEND-CUTOVER-LOCK.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - crates/nyash-llvm-compiler/README.md
  - crates/nyash-llvm-compiler/src/main.rs
---

# P1: Native Bootstrap Seam / Boundary Cutover Task Pack

Note:
- file name is historical.
- final architecture is fixed by `de-rust-backend-zero-boundary-lock-ssot.md`.
- this pack treats `native_driver.rs` as bootstrap seam, not final owner.

## Purpose

- bootstrap seam を「大きな夢」ではなく、小さな受理形で進める。
- final target を `.hako -> thin backend boundary` に固定したまま進める。
- `.hako` builder line や optimization handoff を early に混ぜない。
- 1 min = 1 acceptance shape = 1 owner-local patch の運用を固定する。

## Fixed Order

| id | goal | owned surface | acceptance shape | do not mix |
| --- | --- | --- | --- | --- |
| `BE0-min1` | `ny-llvmc` public CLI contract freeze | docs + `crates/nyash-llvm-compiler/src/main.rs` | `.hako` / runner 側が依存してよい CLI surface が 1 枚で固定される | native codegen 実装 |
| `BE0-min2` | native driver seam を `ny-llvmc` 内へ作る | `crates/nyash-llvm-compiler/src/**` | default llvmlite route を変えずに native route を opt-in で選べる | runner route widening |
| `BE0-min3` | minimal native object canary | `crates/nyash-llvm-compiler/src/**` | 既存 small MIR fixture で `.o` が Python 非依存で出る | exe parity / app parity |
| `BE0-min4` | native executable parity | `crates/nyash-llvm-compiler/src/main.rs` + 必要時のみ `src/runner/modes/common_util/exec.rs` | 同じ canary が既存 static-first link line で EXE 化できる | optimization tuning |
| `BE0-min5` | runner / hakorune opt-in parity | `src/runner/modes/llvm/**` の薄い route glue | existing `.hako` fixture が native route で走る | llvmlite delete |
| `BE0-min6` | thin backend boundary cutover lock | `lang/src/shared/backend/**`, `lang/c-abi/**`, caller docs | `.hako` caller が thin boundary を daily route で使える前提が lock される | optimization handoff |

## Min-by-min Notes

### `BE0-min1` CLI contract freeze

- lock するもの:
  - `--in`
  - `--out`
  - `--emit {obj|exe}`
  - `--dummy`
  - `--nyrt`
  - `--libs`
- contract SSOT:
  - `crates/nyash-llvm-compiler/README.md`
- lock しないもの:
  - native implementation の内部 module shape
  - pass pipeline の中身
- landed (2026-03-14):
  - `crates/nyash-llvm-compiler/README.md`
  - `crates/nyash-llvm-compiler/src/main.rs` の `EmitKind` enum + clap parse tests
  - `--harness` は implementation-detail として `--help` に明示された

### `BE0-min2` native driver seam

- native route の selector は default route を壊さない
- 一時 selector を env にするなら、隠しトグル禁止ルールに従い docs を同時更新する
- 可能なら repo-wide env 追加ではなく `ny-llvmc` 局所の contract で済ませる
- landed (2026-03-14):
  - implementation-detail selector is now `--driver {harness|native}`
  - default route is still `harness`
  - `native` is currently fail-fast seam only; no silent fallback

### `BE0-min3` minimal native object canary

- first canary は既存 small MIR fixture でよい
  - seed: `apps/tests/mir_shape_guard/collapsed_min.mir.json`
- acceptance は「Python を呼ばずに `.o` が出る」こと
- ここではまだ runner や `.hako` source fixture を混ぜない
- landed (2026-03-14):
  - `crates/nyash-llvm-compiler/src/native_driver.rs` が `const(i64)` + `ret` の最小 native object emission を owner
  - `./target/release/ny-llvmc --in apps/tests/mir_shape_guard/collapsed_min.mir.json --driver native --out /tmp/nyllvmc_collapsed_native.o` が green

### `BE0-min4` native executable parity

- object parity が先
- EXE parity は既存 link line の再利用だけを見る
- `.o` を出し直す backend-specific optimization はこの min で始めない
- landed (2026-03-14):
  - `./target/release/ny-llvmc --in apps/tests/mir_shape_guard/collapsed_min.mir.json --driver native --emit exe --nyrt target/release --out /tmp/nyllvmc_collapsed_native.exe` が green
  - generated EXE は `rc=0`

### `BE0-min5` runner / hakorune parity

- first app parity seed:
  - `apps/tests/hello_simple_llvm.hako`
- acceptance は `phase29x_llvm_cabi_link_min.sh` 相当の意味契約を native route でも再現できること
- current llvmlite default route はこの min が閉じるまで keep
- landed (2026-03-14):
  - `crates/nyash-llvm-compiler/src/native_driver.rs` now supports `mir_call(print/1)` for the app seed
  - `tools/build_llvm.sh` honors `NYASH_LLVM_COMPILER=crate` + `NYASH_LLVM_BACKEND=native`
  - `tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh` is green
  - `src/runner/modes/common_util/exec.rs` now forwards `NYASH_LLVM_BACKEND=native` to `ny-llvmc --driver native`
  - direct runner parity is green for `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/hello_simple_llvm.hako`

### `BE0-min6` thin backend boundary cutover lock

- target owners:
  - `lang/src/shared/backend/llvm_backend_box.hako`
  - `lang/c-abi/shims/hako_aot.c`
  - caller wiring docs
- contract:
  - `.hako` caller は thin boundary を daily route の owner として使う
  - `native_driver.rs` は final owner ではなく canary-only / retire target になる
  - no third canonical ABI is introduced
- archive rule:
  - legacy `llvm_ir` script-builder / AotFacade route は archive keep へ退避する
- exact owner/env/archive contract:
  - `P3-THIN-BACKEND-CUTOVER-LOCK.md`
- landed (2026-03-14):
  - `LlvmBackendBox` first implementation is live under `lang/src/shared/backend/llvm_backend_box.hako`
  - owner route is pinned to `LlvmBackendBox.compile_obj(json_path)` / `LlvmBackendBox.link_exe(obj_path, out_path, libs)` lowering to canonical `env.codegen.compile_json_path/link_object`
  - public first-cut contract is `compile_obj(json_path)` / `link_exe(obj_path, out_path, libs)`
  - acceptance smokes are:
    - `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh`

## Post-`BE0-min6` Follow-up

- official C owner dedupe (`hako_aot.h` / `hako_aot.c`)
- runtime proof blocker inventory (`hostbridge` runtime support / `vm-hako` subset-check)
- backend-specific optimization owner は thin boundary cutover 完了までは `llvmlite`
- cutover 完了後に新規 backend-specific optimization owner を thin boundary の内側へ移す
- llvmlite は compat/canary keep へ降格する

## Hard Boundaries

1. `.hako` backend caller waveは `BE0-min4` より前に開かない
2. `Cranelift` は keep のまま固定し、task pack に混ぜない
3. new hidden env / silent fallback は作らない
4. native route fail は fail-fast で出し、llvmlite への自動退避で隠さない
5. `native_driver.rs` を final owner として docs に固定しない

## Recommended First Slice

- start at `BE0-min1`
- 1 patch の owner は `crates/nyash-llvm-compiler/src/main.rs` と phase docs に閉じる
- first code goal は「実装着手」ではなく「CLI contract を固定し、native route 用の seam を作る前提を固める」こと
