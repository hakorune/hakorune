# Phase 158x: observe tls backend

- Status: Landed
- 目的: `perf-observe` build の exact counter backend を TLS-first に切り替え、hot path の shared atomic を外す。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
  - `crates/nyash_kernel/src/observe/**`

## Goal

- exact counter backend を `AtomicU64` 常設から TLS-first に寄せる
- default release の compile-out は維持する
- `perf-observe` build では `NYASH_PERF_COUNTERS=1` の narrow probe をそのまま使える

## Current Shape

- default build
  - observer code は compile-out
- `--features perf-observe`
  - contract identity:
    - `store.array.str`
    - `const_suffix`
  - backend:
    - TLS exact counters
    - exited thread merge
    - current-thread flush on summary
  - sink:
    - stderr summary

## Next

1. `phase-159x observe trace split`
2. `phase-137x main kilo reopen selection`

## Exit

- `backend_atomic.rs` が current backend でなくなる
- `backend_tls.rs` が current backend として source-backed に読める
- `perf-observe` build の exact counter summary が維持される
- current docs が TLS backend を current truth として読める

## Proof

- `cargo check -p nyash_kernel` PASS
- `cargo check -p nyash_kernel --features perf-observe` PASS
- `cargo build --release -p nyash_kernel --features perf-observe` PASS
- `cargo test -p nyash_kernel --features perf-observe observe::backend_tls::tests::tls_store_array_str_counters_flush_current_thread -- --nocapture` PASS
- `cargo test -p nyash_kernel --features perf-observe observe::backend_tls::tests::tls_const_suffix_counters_flush_current_thread -- --nocapture` PASS
