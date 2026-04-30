---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: move the remaining shell-owned `--emit-program-json-v0` syntax into one neutral helper without deleting Program(JSON v0) keeper lanes.
Related:
  - docs/development/current/main/phases/phase-29ci/P17-EMIT-PROGRAM-JSON-V0-HELPER-CENTRALIZE.md
  - docs/development/current/main/phases/phase-29ci/P7-RAW-COMPAT-CALLER-INVENTORY.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - tools/lib/program_json_v0_compat.sh
---

# P18 Program JSON v0 Shell Emit SSOT

## Goal

P17 left two helper-owned shell emit sites:

- selfhost helper owner
- smoke fixture helper owner

P18 keeps both evidence lanes, but moves the raw public compat flag spelling to
one neutral shell helper:

```text
tools/lib/program_json_v0_compat.sh
```

This is still a cleanup slice, not a route migration:

- no new Program(JSON v0) caller family
- no deletion of `--emit-program-json-v0`
- no change to `.hako mirbuilder` Program(JSON) fixture semantics

## Ownership After P18

| Layer | Helper | Responsibility |
| --- | --- | --- |
| neutral shell compat owner | `tools/lib/program_json_v0_compat.sh` | the only current shell emit spelling of `--emit-program-json-v0`; archive pins source this helper after P20 |
| selfhost | `tools/selfhost/lib/stage1_contract.sh`, `tools/selfhost/lib/selfhost_build_stageb.sh` | source the neutral helper; no raw flag spelling |
| smoke fixture producer | `tools/smokes/v2/lib/stageb_helpers.sh` | source the neutral helper; no raw flag spelling |

## Remaining Keeper Lanes

The remaining work is replacing evidence lanes, not cleaning duplicate shell
syntax:

1. Stage-B/selfhost build still needs Program(JSON v0) as its primary payload.
2. Stage1 `emit-program` contract still needs Program(JSON v0) as explicit
   compat evidence.
3. `.hako mirbuilder` smokes intentionally consume Program(JSON v0) fixtures.

## Acceptance

```bash
rg -n -g '!tools/historical/**' -g '!target/**' -- '--emit-program-json-v0' src tools
bash -n tools/lib/program_json_v0_compat.sh tools/selfhost/lib/stage1_contract.sh tools/selfhost/lib/selfhost_build_stageb.sh tools/smokes/v2/lib/stageb_helpers.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```
