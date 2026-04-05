# Phase 110x: selfhost execution vocabulary SSOT

目的: selfhost 実行面の用語を `stage / route / backend override / lane / kernel` に分離して、`exe` / `vm` / `kernel` の多義性を止める。

Current front:
- inventory lock: public surfaces と internal owners を 5 語に割り当てる
- exact focus:
  - `tools/selfhost/run.sh --runtime --runtime-route mainline`
  - `hakorune --backend vm|vm-hako|llvm`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
  - `src/runner/core_executor.rs`
  - `crates/nyash_kernel/`
  - `lang/src/vm/`

SSOT target:
- `stage`
  - artifact 生成段階 / historical phase 名
- `route`
  - end-to-end 実行経路
- `backend override`
  - CLI 明示 override
- `lane`
  - backend family の内部 concrete path
- `kernel`
  - product/native runtime core に限定

Success condition:
- long-lived SSOT を 1 枚に固定する
- current/docs から `exe` / `vm` / `kernel` の多義説明を減らす
- `phase-111x` の rename lane に渡せる vocabulary を決める

Primary artifact:
- `docs/development/architecture/selfhost_execution_ssot.md`
