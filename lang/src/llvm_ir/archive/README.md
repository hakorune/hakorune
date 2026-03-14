# llvm_ir archive

目的
- backend-zero の official route から外れた historical `.hako` 資産を保管する。
- 削除ではなく archive に寄せて、履歴参照と compat 調査だけ残す。

Archive policy
- `legacy_script_builder/`
  - old `LLVMModuleBox` / `LLVMFunctionBox` / `LLVMBuilderBox` / `LLVMTypesBox`
  - old `LLVMV0BuilderBox`
  - old `LLVMAotFacadeBox`
- `examples/`
  - old AotBox example

Non-goals
- archived route を新しい daily caller に戻さない。
- archive 配下へ新機能を追加しない。

Official route
- `.hako` caller:
  - `lang/src/shared/backend/llvm_backend_box.hako`
- thin C helper:
  - `lang/c-abi/shims/hako_aot.c`
