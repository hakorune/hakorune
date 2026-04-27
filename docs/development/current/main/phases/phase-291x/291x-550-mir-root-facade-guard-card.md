---
Status: Landed
Date: 2026-04-27
Scope: Add a no-regrowth guard for the MIR root facade export surface
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/tools/check-scripts-index.md
  - src/mir/mod.rs
  - tools/checks/dev_gate.sh
  - tools/checks/mir_root_facade_allowlist.txt
  - tools/checks/mir_root_facade_guard.sh
---

# 291x-550: MIR Root Facade Guard

## Goal

Close the MIR-root export cleanup series with a fail-fast no-regrowth guard.

`src/mir/mod.rs` is allowed to be a small facade for core MIR surfaces and
refresh orchestration entry points. It must not silently become a semantic
metadata catalog again.

## Inventory

Added guard surfaces:

- `tools/checks/mir_root_facade_guard.sh`
- `tools/checks/mir_root_facade_allowlist.txt`

Updated gate/docs surfaces:

- `tools/checks/dev_gate.sh`
- `docs/tools/check-scripts-index.md`
- `docs/development/current/main/design/mir-root-facade-contract-ssot.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Cleaner Boundary

```text
src/mir/mod.rs
  exposes allowlisted facade symbols only

tools/checks/mir_root_facade_allowlist.txt
  records the current approved facade symbol set

tools/checks/mir_root_facade_guard.sh
  rejects root-export drift and wildcard pub use

owner modules
  keep semantic metadata vocabulary
```

## Boundaries

- BoxShape-only.
- Do not change MIR behavior.
- Do not change any root export during this guard card.
- Do not bless semantic metadata vocabulary as root-facade API.
- Only update the allowlist with a phase card and contract rationale.

## Acceptance

- `bash tools/checks/mir_root_facade_guard.sh` passes.
- `tools/checks/dev_gate.sh` quick profile runs the guard.
- Check script index documents the guard.
- MIR root facade contract points to the guard and allowlist.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Added an exact allowlist guard for `pub use` exports from `src/mir/mod.rs`.
- Wired the guard into the quick dev gate.
- Documented the guard in the MIR root facade SSOT and checks index.

## Verification

```bash
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
