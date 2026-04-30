---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: route archived phase29bq mirbuilder Program(JSON v0) emit pins through the neutral shell helper.
Related:
  - docs/development/current/main/phases/phase-29ci/P18-PROGRAM-JSON-V0-SHELL-EMIT-SSOT.md
  - tools/lib/program_json_v0_compat.sh
  - tools/smokes/v2/profiles/archive/joinir/
---

# P20 Archive Program JSON v0 Emit Helper Sync

## Goal

Archived phase29bq mirbuilder pins still spelled the raw public compat flag
directly. They are not current acceptance lanes, but the direct spelling makes
repo-wide inventory noisier than necessary.

P20 keeps the archived scripts runnable and moves them through the neutral
helper:

```text
tools/lib/program_json_v0_compat.sh
```

No current behavior or acceptance contract changes.

## Acceptance

```bash
rg -n -g '!tools/historical/**' -g '!target/**' -- '--emit-program-json-v0' src tools
bash -n tools/smokes/v2/profiles/archive/joinir/phase29bq_hako_mirbuilder_phase5_min_vm.sh \
  tools/smokes/v2/profiles/archive/joinir/phase29bq_hako_mirbuilder_phase7_min_vm.sh \
  tools/smokes/v2/profiles/archive/joinir/phase29bq_hako_mirbuilder_phase9_min_vm.sh
```
