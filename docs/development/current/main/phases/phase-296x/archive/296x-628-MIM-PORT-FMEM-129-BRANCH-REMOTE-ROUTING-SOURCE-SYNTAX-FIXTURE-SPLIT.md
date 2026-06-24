---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-129.
Related:
  - docs/development/current/main/phases/phase-296x/296x-618-MIM-PORT-FMEM-119-REMAINING-SOURCE-SYNTAX-SMOKE-RETIREMENT-TASK-ORDER.md
  - tools/hako_check/manifests/fastmem_route_cfg_smoke.toml
  - tools/hako_check/fastmem_terminal_ladder_smoke.sh
  - tools/hako_check/fastmem_source_manifest_runner.py
---

# 296x-628 MIM-PORT-FMEM-129 Branch / Remote Routing Source-Syntax Fixture Split

## Purpose

Split the remaining branch / remote routing source-syntax fixtures into a
dedicated manifest-backed route smoke so the terminal ladder lane can keep the
shared route / ladder evidence without carrying those source-owned fixtures.

This landed slice covers the 628 branch-routing rows:

- branch return scope route body preflight
- remote-owner branch routing lowering
- fastmem branch cfg lowering

## Implementation

```text
manifest promotions:
  BRANCH_RETURN_SCOPE
  REMOTE_OWNER_BRANCH_ROUTING_LOWERING
  FASTMEM_BRANCH_CFG_LOWERING

route smoke:
  tools/hako_check/manifests/fastmem_route_cfg_smoke.toml owns the branch /
  remote routing source-syntax fixtures

terminal ladder smoke:
  keeps the shared route / ladder evidence and seeds the manifest-backed route
  fixtures through the shared manifest runner
```

The 628 split removes the last shell-owned branch / remote routing fixtures
from the source-syntax smoke path and leaves the shared ladder assertions in
the dedicated terminal ladder smoke.

## Closed

```text
shell-owned branch / remote routing source-syntax assertions
branch / remote routing fixture drift
new branch semantics
product/hook/global/winner claims
```

## Verification

```bash
python3 tools/hako_check/fastmem_source_manifest_runner.py \
  --manifest tools/hako_check/manifests/fastmem_route_cfg_smoke.toml
bash tools/hako_check/fastmem_terminal_ladder_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
The branch / remote routing source-syntax fixtures now live in the dedicated
route smoke manifest, and the shared terminal ladder smoke remains the source
of truth for the route / ladder evidence.
```

## Closeout

```text
next: phase-296x next lane selection pending
```
