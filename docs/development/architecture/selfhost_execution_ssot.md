# Selfhost Execution SSOT

Status: SSOT  
Date: 2026-04-05

## Purpose

`exe` / `vm` / `kernel` が複数の軸を兼ねていたので、selfhost 実行面の vocabulary を固定する。

この文書では、次の 5 語だけを正本にする。

- `stage`
- `route`
- `backend override`
- `lane`
- `kernel`

## Public Surfaces

### Selfhost facade route

- `tools/selfhost/run.sh --runtime --runtime-route mainline`
- `tools/selfhost/run.sh --runtime --runtime-route compat`
- `tools/selfhost/run.sh --direct --source-file ...`

`run.sh` は shell facade。実 route body は
[tools/selfhost/lib/selfhost_run_routes.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/lib/selfhost_run_routes.sh)
が持つ。

外向き surface の canonical 名は `--runtime-route mainline|compat`。
`--runtime-mode exe|stage-a-compat` と `stage-a` は compatibility alias として残す。

### CLI backend override

- `hakorune --backend llvm`
- `hakorune --backend vm`
- `hakorune --backend vm-hako`
- `hakorune --mir-json-file ...`

CLI の explicit override は
[src/runner/dispatch.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/dispatch.rs)
から入る。

## Vocabulary

### `stage`

artifact 生成段階か historical phase 名に限定する。

例:
- `stage1`
- `stage-a-compat`
- `stage-b-proof`

`stage` は runtime surface の最終 route 名としては使わない。

### `route`

end-to-end 実行経路。

例:
- `runtime/mainline`
- `runtime/compat`
- `direct/proof`
- `direct/mir-json`

`tools/selfhost/run.sh` が選ぶのは route。

### `backend override`

CLI が受ける explicit family selector。

例:
- `llvm`
- `vm`
- `vm-hako`

`hakorune --backend ...` が選ぶのは backend override。

### `lane`

backend family の内部で実際に走る concrete implementation path。

例:
- `rust-vm-keep`
- `vm-hako-reference`
- `vm-compat-fallback`

[src/runner/route_orchestrator.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/route_orchestrator.rs)
が最終的に決めるのは lane。

### `kernel`

runtime core / link-time core に限定して使う。

この repo では
[crates/nyash_kernel](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel)
だけを `kernel` と呼ぶ。

[lang/src/vm](/home/tomoaki/git/hakorune-selfhost/lang/src/vm)
は `VM/reference cluster` と呼び、`kernel` とは呼ばない。

## Route Map

### `runtime/mainline`

Public surface:
- `tools/selfhost/run.sh --runtime --runtime-route mainline`

Body:
1. `compat/run_stage1_cli.sh emit mir-json`
2. temp MIR(JSON) を作る
3. `hakorune --mir-json-file tmp.json`
4. [src/runner/core_executor.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/core_executor.rs)
   が terminal owner

注意:
- historical alias `runtime-mode exe` は native executable route を意味しない
- 実体は `temp MIR handoff -> core executor`

### `runtime/compat`

Public surface:
- `tools/selfhost/run.sh --runtime --runtime-route compat`

Body:
- explicit compat keep
- 実行時には `--backend vm` を使う narrow keep route
- `stage-a` は thin alias only で、canonical route 名ではない

### `direct/proof`

Public surface:
- `tools/selfhost/run.sh --direct --source-file ...`

Body:
- `tools/selfhost/proof/run_stageb_compiler_vm.sh`
- proof-only keep

### `direct/mir-json`

Public surface:
- `hakorune --mir-json-file file.json`

Body:
- already-materialized MIR(JSON)
- [src/runner/core_executor.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/core_executor.rs)
  が terminal owner

## Backend Family Map

### `--backend llvm`

- product route
- LLVM backend 実行

### `--backend vm`

これは concrete implementation 名ではない。`vm` family override であり、
内部では lane selection を行う。

current lane set:
- `rust-vm-keep`
- `vm-hako-reference`
- `vm-compat-fallback`

### `--backend vm-hako`

- explicit reference override
- `vm-hako-reference` lane へ入る

## Kernel Naming

### Product kernel

- [crates/nyash_kernel](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel)
- native executable linking / AOT runtime core

### VM/reference cluster

- [lang/src/vm](/home/tomoaki/git/hakorune-selfhost/lang/src/vm)
- mini VM / reference semantic executor / boxes / engines

`lang/src/vm` は `kernel` ではなく `VM/reference cluster` として扱う。

## Deprecation Direction

### Route naming

外向き rename 方向:
- `runtime-mode exe` -> `runtime-route mainline`
- `stage-a` / `stage-a-compat` -> `runtime-route compat`

互換 alias は当面残してよいが、SSOT の canonical 名は `mainline` / `compat` に寄せる。

### Lane naming

内側の canonical 名:
- `rust-vm-keep`
- `vm-hako-reference`
- `vm-compat-fallback`

`vm` は family 名に留め、family と lane を同じ語にしない。

## Do Not Widen

- `runtime/mainline` は direct/core owner を維持する
- `vm` family keep/reference lane を mainline default に戻さない
- `kernel` は `nyash_kernel` に限定する
- `lang/src/vm` を product kernel と呼ばない
