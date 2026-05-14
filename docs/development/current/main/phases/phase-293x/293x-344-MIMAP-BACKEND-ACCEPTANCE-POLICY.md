# 293x-344 MIMAP Backend Acceptance Policy

Status: landed.
Decision: accepted.

## Goal

Fix the VM/LLVM backend acceptance split before MIMAP-011 object-heavy facade
work continues.

## Landing shape

- VM remains a semantic reference executor and small scalar proof backend.
- LLVM/EXE is primary for MIMAP-011+ page queue, heap facade, lifecycle, and
  object-return allocator routes.
- MIMAP VM guards must use timeout.
- Known VM limitations are documented with a retirement condition.

## Files

- `docs/development/current/main/design/mimalloc-backend-acceptance-policy-ssot.md`
- `docs/development/current/main/design/vm-known-limitations-ssot.md`
- `tools/checks/lib/guard_common.sh`
- `tools/checks/k2_wide_mimalloc_backend_acceptance_policy_guard.sh`

## Guard

```bash
bash tools/checks/k2_wide_mimalloc_backend_acceptance_policy_guard.sh
```

Next selected row remains: `MIMAP-011 allocator facade lifecycle route pilot`.
