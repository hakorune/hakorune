# tools/compat

Phase 29x X39 で導入した「互換（Rust）レーン専用」の入口。

原則:

1. daily/milestone の既定導線は LLVM-only lane を使う
2. Rust lane はこのディレクトリ配下のコマンドでのみ実行する
3. 互換レーン実行には明示 opt-in（`PHASE29X_ALLOW_RUST_LANE=1`）が必要

Entry:

- `tools/compat/phase29x_rust_lane_gate.sh`
