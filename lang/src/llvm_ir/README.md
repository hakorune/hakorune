# llvm_ir

目的
- LLVM line でまだ live な `.hako` 資産を小さく保つ。
- 現在の live owner は `AotPrep` / `normalize` / compat `emit` だけに絞る。
- backend-zero の daily caller route はここに増やさず、`lang/src/shared/backend/` と `lang/c-abi/` に寄せる。

Live surfaces
- `boxes/aot_prep.hako` と `boxes/aot_prep/**`
  - MIR(JSON) の軽量前処理と perf/compat pass
- `boxes/normalize/**`
  - `print` / `ref_*` / legacy `array_*` の normalize helper
- `instructions/**`
  - quick/smoke の self-param helper
- `emit/LLVMEmitBox.hako`
  - compat keep の emit facade
  - legacy phase2034 canary scripts are deleted; this box is not an active smoke gate

Archived surfaces
- legacy script-builder / AotFacade route は `archive/legacy_script_builder/**` に退避した。
- old example は `archive/examples/**` に退避した。
- これらは backend-zero daily route の owner ではない。

Non-goals
- 新しい daily backend caller をここへ追加しない。
- `AotBox` / legacy script-builder route を復活させない。
- raw LLVM API surface を `.hako` 側へ広げない。

Entry pointers
- official thin backend caller:
  - `lang/src/shared/backend/llvm_backend_box.hako`
- C helper side:
  - `lang/c-abi/shims/hako_aot.c`
- boundary SSOT:
  - `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`

Fail-fast
- archived route を daily route に戻す変更はここで受けない。
- compat emit 以外の新しい backend caller が必要なら `llvm_ir` ではなく `shared/backend` 側で設計を起こす。
