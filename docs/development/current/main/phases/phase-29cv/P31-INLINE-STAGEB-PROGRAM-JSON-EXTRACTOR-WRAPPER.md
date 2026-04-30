---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: inline the single-use Stage-B Program(JSON) extraction wrapper in the selfhost EXE helper.
Related:
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - tools/selfhost_exe_stageb.sh
  - tools/selfhost/lib/stageb_program_json_capture.sh
---

# P31 Inline Stage-B Program JSON Extractor Wrapper

## Goal

Keep the explicit Program(JSON)->MIR bridge probe bucket thin without moving
ownership.

`tools/selfhost_exe_stageb.sh` sourced
`tools/selfhost/lib/stageb_program_json_capture.sh`, but still kept a local
`extract_program_json()` wrapper that only forwarded to
`stageb_program_json_extract_from_stdin()` and had a single callsite.

## Decision

- inline the capture SSOT helper at the pipe callsite
- delete the local wrapper
- keep Stage-B capture semantics unchanged

## Non-goals

- do not change `stageb_program_json_extract_from_stdin()`
- do not change `program_json_mir_bridge_emit()`
- do not change Stage-B delegate route semantics
- do not weaken bridge probe evidence

## Acceptance

```bash
bash -n tools/selfhost_exe_stageb.sh
tmp_log=/tmp/selfhost_exe_stageb.p31.log
tmp_exe=/tmp/selfhost_exe_stageb.p31.out
rm -f "$tmp_log" "$tmp_exe"
bash tools/selfhost_exe_stageb.sh apps/tests/hello_simple_llvm.hako -o "$tmp_exe" >"$tmp_log" 2>&1 || true
grep -qF "[emit-route] stageb-delegate" "$tmp_log"
grep -qF "[emit] MIR JSON:" "$tmp_log"
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
