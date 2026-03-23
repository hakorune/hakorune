---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `hako.tls` の current live strategy と final end-state を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md
  - docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
  - docs/development/current/main/design/gc-tls-atomic-capability-ssot.md
  - docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - lang/src/runtime/substrate/tls/README.md
  - crates/nyash_kernel/src/plugin/handle_cache.rs
  - crates/nyash_kernel/src/exports/string_span_cache.rs
  - lang/c-abi/shims/hako_diag_mem_shared_impl.inc
---

# Thread And TLS Capability (SSOT)

## Goal

- current migration slice の TLS と、final language-level TLS end-state を分けて読む。
- helper-shaped truth を current live row として使いながら、final abstraction を raw slot API にしない。

## Fixed Reading

### Current live strategy

- current live TLS rows are helper-shaped only.
- current first truthful row is:
  - `TlsCoreBox.last_error_text_h()`
- current helper truth includes:
  - diagnostics TLS (`hako_last_error`)
  - helper-local Rust TLS caches (`HANDLE_CACHE`, string span cache)

### Final end-state

Final TLS should be language-level on the `.hako` side:

- `thread_local` declaration form, or
- `TlsCell<T>` style library abstraction

Lowering rule:

- executable/mainline route
  - lower to LLVM TLS globals when available
- plugin/DSO/unsupported route
  - lower to thin native fallback ABI

## Rejected For This Wave

- raw numeric TLS slot APIs as the public `.hako` abstraction
- exposing helper-local caches as the final TLS abstraction
- pretending diagnostics TLS is the final end-state

## Current Rule

- use helper-shaped TLS rows to prove truthful capability presence first
- keep final `thread_local` / `TlsCell<T>` as docs-locked end-state until a later widening wave
- do not widen `hako.tls` into allocator policy or generic slot orchestration in this slice
- do not define `ThreadLocal` as public Rune v0 surface; declaration form / library abstraction remains the preferred end-state
