---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: selfhost EXE / Stage-B EXE helper の Program(JSON)->MIR conversion を raw CLI から non-raw env.mirbuilder.emit bridge へ移す。
Related:
  - docs/development/current/main/phases/phase-29ci/P13-SHARED-SMOKE-HELPER-FALLBACK-RETIRE.md
  - tools/selfhost/lib/program_json_mir_bridge.sh
  - tools/selfhost/lib/selfhost_build_exe.sh
  - tools/selfhost_exe_stageb.sh
---

# P14 Selfhost EXE Bridge Non-Raw

## Goal

P13 後に残った selfhost EXE / Stage-B EXE helper の raw
Program(JSON)->MIR CLI caller を、共通の non-raw bridge に寄せる。

## Decision

- `tools/selfhost/lib/program_json_mir_bridge.sh` を追加する。
- bridge は MIR v1 wrapper から `env.mirbuilder.emit` を呼び、stdout marker
  から MIR(JSON) payload を抽出する。
- `tools/selfhost/lib/selfhost_build_exe.sh` はこの bridge を使って
  Program(JSON)->MIR->EXE lane を維持する。
- `tools/selfhost_exe_stageb.sh` の `stageb-delegate` route も同じ bridge を
  使う。
- raw CLI implementation はまだ削らない。残る shell caller は
  `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` だけで、compiled
  stage1 binary proof が必要。

## Result

Current `--program-json-to-mir` shell caller:

1. `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`

selfhost EXE helpers no longer call the raw Program(JSON)->MIR CLI surface.

## Acceptance

```bash
bash -n tools/selfhost/lib/program_json_mir_bridge.sh
bash -n tools/selfhost/lib/selfhost_build_exe.sh
bash -n tools/selfhost_exe_stageb.sh
rg -l -g '!tools/historical/**' -- '--program-json-to-mir' tools src
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
