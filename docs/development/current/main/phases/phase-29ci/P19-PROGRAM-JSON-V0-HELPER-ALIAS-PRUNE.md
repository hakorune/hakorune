---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: prune the unused selfhost-specific alias from the neutral Program(JSON v0) shell helper after P18.
Related:
  - docs/development/current/main/phases/phase-29ci/P18-PROGRAM-JSON-V0-SHELL-EMIT-SSOT.md
  - tools/lib/program_json_v0_compat.sh
---

# P19 Program JSON v0 Helper Alias Prune

## Goal

P18 moved the raw shell spelling of `--emit-program-json-v0` to one neutral
helper. The temporary selfhost-named alias is now unused.

Remove only:

```text
selfhost_emit_program_json_v0_to_file()
```

Keep:

```text
program_json_v0_compat_emit_to_file()
```

## Acceptance

```bash
rg -n 'selfhost_emit_program_json_v0_to_file' tools src
bash -n tools/lib/program_json_v0_compat.sh tools/selfhost/lib/stage1_contract.sh tools/selfhost/lib/selfhost_build_stageb.sh tools/smokes/v2/lib/stageb_helpers.sh
```
