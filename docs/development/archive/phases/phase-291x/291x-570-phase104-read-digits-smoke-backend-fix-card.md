---
Status: Landed
Date: 2026-04-28
Scope: fix archived phase104 read-digits smoke backend timeout
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_vm.sh
  - apps/tests/phase104_read_digits_loop_true_min.hako
---

# 291x-570: Phase104 Read-Digits Smoke Backend Fix

## Goal

Fix the archived phase104 read-digits smoke that timed out when it forced the
legacy `--backend vm` route.

The fixture itself lowers and runs correctly through the current mainline MIR
route. The blocker was the smoke script's stale backend selection, not the
read-digits policy or JoinIR lowering.

## Evidence

```bash
env HAKO_JOINIR_STRICT=1 NYASH_DISABLE_PLUGINS=0 \
  ./target/release/hakorune --backend mir \
  apps/tests/phase104_read_digits_loop_true_min.hako
```

returns:

```text
2
1
```

The same fixture with explicit `--backend vm` times out in the archived smoke.
`--backend vm` is now the raw legacy compat/proof ingress; this archived smoke
should pin the read-digits behavior on the current mainline MIR route.

## Cleaner Boundary

```text
phase104 read-digits smoke
  verifies read-digits loop(true) behavior through mainline MIR

legacy --backend vm
  remains a separate compat/proof/debug route, not this behavior gate
```

## Boundaries

- Smoke script only.
- Do not change the fixture source.
- Do not change read-digits lowering or policy code.
- Keep the historical script filename stable.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_vm.sh` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Updated the archived phase104 smoke to execute the fixture via `--backend mir`.
- Added script comments noting that the filename is historical and the backend
  is intentionally mainline MIR.

## Verification

```bash
bash tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_vm.sh
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
git diff --check
```
