# Phase 145x: compat quarantine shrink

- Status: Landed
- 目的: `host microkernel` glue と `compat quarantine` を source/docs 上で取り違えない状態にする。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `crates/nyash_kernel/src/hako_forward_bridge.rs`
  - `crates/nyash_kernel/src/plugin/future.rs`
  - `crates/nyash_kernel/src/plugin/invoke_core.rs`
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`

## Decision Now

- host-side glue:
  - `hako_forward_bridge.rs`
  - `future.rs`
  - `invoke_core.rs`
- compat quarantine:
  - `module_string_dispatch/**`
- keep the former in `Rust host microkernel`
- keep the latter shrink-only and non-owner

## Exit Criteria

1. host-side glue files say `host service contract` / `runtime glue` explicitly
2. `module_string_dispatch/**` says `compat quarantine` / `shrink-only` explicitly
3. invoke/trace wording no longer makes quarantine read like a main route owner
4. `nyash-kernel-semantic-owner-ssot.md` matches source wording
5. next lane can tighten string boundaries without re-opening quarantine confusion

## Next

1. start `phase-146x string semantic boundary tighten`
2. return to `phase-137x main kilo reopen selection`
