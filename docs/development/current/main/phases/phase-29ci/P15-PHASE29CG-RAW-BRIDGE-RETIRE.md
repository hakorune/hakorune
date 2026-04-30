---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: phase29cg stage2 bootstrap proof から raw Program(JSON)->MIR CLI caller を削る。
Related:
  - docs/development/current/main/phases/phase-29ci/P14-SELFHOST-EXE-BRIDGE-NONRAW.md
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
  - tools/selfhost/lib/program_json_mir_bridge.sh
---

# P15 Phase29cg Raw Bridge Retire

## Goal

P14 後に最後まで残っていた shell-side raw Program(JSON)->MIR CLI caller を
削る。

## Decision

- `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` は stage1-cli から
  Program(JSON v0) を出す proof 形は維持する。
- Program(JSON)->MIR conversion だけを
  `tools/selfhost/lib/program_json_mir_bridge.sh` の non-raw bridge に寄せる。
- stage1 binary が無い環境では従来どおり missing-stage1 で fail-fast する。

## Result

Current shell/source grep:

```bash
rg -l -g '!tools/historical/**' -- '--program-json-to-mir' tools src
```

returns no current tools/src callers.

The raw CLI implementation is still present. It is now blocked only by
implementation-side cleanup:
- `src/runner/pipe_io.rs`
- CLI arg/config fields
- preservation proof for `user_box_decls`

P16 follow-up deletes that raw CLI implementation/config surface. The
`user_box_decls` proof remains in `src/host_providers/mir_builder.rs`.

## Acceptance

```bash
bash -n tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
rg -l -g '!tools/historical/**' -- '--program-json-to-mir' tools src
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
```

The last command currently fails at the pre-existing missing-stage1 guard when
`target/selfhost/hakorune.stage1_cli` is absent; it must not reach a raw CLI
conversion path.
