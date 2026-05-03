---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: narrow Stage-B Program(JSON) capture helper loading in smoke stageb helpers
Related:
  - tools/smokes/v2/lib/stageb_helpers.sh
  - tools/selfhost/lib/stageb_program_json_capture.sh
---

# P371A: StageB Helpers Lazy Capture Source

## Intent

Keep `tools/smokes/v2/lib/stageb_helpers.sh` from loading the Stage-B
Program(JSON) stdout capture helper at source time.

The capture helper is needed by `stageb_compile_to_json_with_args()`, not by
every caller that sources `stageb_helpers.sh`.

## Boundary

Allowed:

- add a lazy source wrapper for `stageb_program_json_capture.sh`
- call it only before `stageb_program_json_extract_from_stdin`
- preserve all Stage-B helper behavior

Not allowed:

- delete `stageb_program_json_capture.sh`
- delete `stageb_compile_to_json*` fixture helpers
- change Stage-B compile env defaults
- change Program(JSON) fixture semantics

## Acceptance

```bash
bash -c 'source tools/smokes/v2/lib/stageb_helpers.sh; ! declare -F stageb_program_json_extract_from_stdin >/dev/null'
bash -c 'source tools/smokes/v2/lib/stageb_helpers.sh; stageb_source_program_json_capture; declare -F stageb_program_json_extract_from_stdin >/dev/null'
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
