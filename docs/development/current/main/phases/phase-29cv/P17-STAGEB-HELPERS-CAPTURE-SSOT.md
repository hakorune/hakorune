# P17: Stage-B helpers capture SSOT

Scope: route `tools/smokes/v2/lib/stageb_helpers.sh` through the shared
Stage-B Program(JSON v0) capture helper.

## Why

P14 moved balanced Stage-B stdout extraction into
`tools/selfhost/lib/stageb_program_json_capture.sh`, but the smoke helper still
used an inline `awk` line matcher:

```bash
awk '(/"version":0/ && /"kind":"Program"/) ...'
```

That only works for one-line payloads and keeps a second extraction policy in
the JoinIR/MirBuilder fixture lane.

## Decision

Source `tools/selfhost/lib/stageb_program_json_capture.sh` from
`tools/smokes/v2/lib/stageb_helpers.sh` and use
`stageb_program_json_extract_from_stdin` for Stage-B stdout capture.

This is a BoxShape cleanup only. It does not change the fixture producers or
the Program(JSON v0) contract they pin.

## Acceptance

```bash
bash -n tools/smokes/v2/lib/stageb_helpers.sh
bash tools/smokes/v2/profiles/integration/stageb/stageb_print_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase1_min_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
