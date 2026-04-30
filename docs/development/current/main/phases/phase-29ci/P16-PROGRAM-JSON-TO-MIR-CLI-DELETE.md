---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: raw `--program-json-to-mir` CLI surface and runner pipe implementationを削除する。
Related:
  - docs/development/current/main/phases/phase-29ci/P15-PHASE29CG-RAW-BRIDGE-RETIRE.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - src/runner/pipe_io.rs
  - src/cli/args.rs
  - src/host_providers/mir_builder.rs
---

# P16 Program JSON To MIR CLI Delete

## Goal

P15 で current tools/src caller が 0 になった raw
`--program-json-to-mir` CLI surface を削除する。

## Decision

- CLI arg/config/group field `program_json_to_mir` を削除する。
- `runner::pipe_io` から Program(JSON)->MIR file conversion branch を削除し、
  JSON artifact intake/executionだけに戻す。
- deprecation warning for the removed CLI surface を削除する。
- `user_box_decls` preservation proof は `src/host_providers/mir_builder.rs` の
  `test_program_json_to_mir_json_with_user_box_decls_*` を正本にする。
- Program(JSON)->MIR capability 自体は `env.mirbuilder.emit` / host provider
  path に残す。削除したのは raw CLI surface だけ。

## Result

`--program-json-to-mir` は current CLI surface ではない。

`--emit-program-json-v0` はまだ残る。stage0 direct compat /
Stage-B Program(JSON) producer / phase29bq mirbuilder fixture producer が live
なので、別 slice で扱う。

## Acceptance

```bash
rg -n 'program_json_to_mir|program-json-to-mir|warn_program_json_to_mir' src/cli src/runner src/runtime/deprecations.rs -g '*.rs'
cargo check -q
cargo test -q test_program_json_to_mir_json_with_user_box_decls
bash tools/checks/current_state_pointer_guard.sh
```
