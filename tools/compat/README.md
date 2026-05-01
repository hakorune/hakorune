# tools/compat

Compat-bucket entrypoint directory.

原則:

1. daily/milestone の既定導線は LLVM-only lane を使う
2. Rust lane はこのディレクトリ配下の `phase29x_rust_lane_gate.sh` でのみ実行する
3. legacy codegen / proof payloads live under `tools/archive/legacy-selfhost/compat-codegen/`
4. 互換レーン実行には明示 opt-in（`PHASE29X_ALLOW_RUST_LANE=1`）が必要
5. native LLVM builder は `ny_mir_builder.sh` の
   `NYASH_LLVM_BACKEND=native` からだけ呼ぶ canary capsule として扱う

Entry:

- `tools/compat/phase29x_rust_lane_gate.sh`
- `tools/compat/native_llvm_builder.py`
- `tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_pack.sh`
