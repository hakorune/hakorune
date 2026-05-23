---
Status: Landed
Date: 2026-05-24
Scope: close out the MIMAP-560A presentation-only extension pilot after the later evidence flags landed exact `usize`.
Blocker: MIMAP-560A-PRESENTATION-ONLY-EXTENSION-PILOT-CLOSEOUT
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/development/current/main/phases/phase-294x/294x-159-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT-LATER-HOOK-PROVIDER-GLOBAL-ALLOCATOR-EVIDENCE-FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-61-MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-RUNNER.md
---

# 294x-160 Hako Alloc Usize C Mimalloc Result Presentation-Only Extension Pilot Closeout

## Decision

Close the MIMAP-560A presentation-only extension pilot now that the later
hook/provider/global-allocator evidence flags are exact `usize`.

The pilot has now covered:

- owner-local presentation counters;
- comparison count / byte payloads;
- report metadata and evidence-status fields;
- performance, memory, repeated-benchmark, and process-replacement evidence;
- later hook/provider/global-allocator evidence flags.

This is a closeout, not a new allocator feature row. Provider activation,
host allocator replacement, hook installation, and `#[global_allocator]`
remain parked. The next useful row is the hako-side pure-first EXE
memory-use evidence runner already defined by
`MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-001`.

## Stable Output

The closeout record is intentionally small:

```text
pilot_closeout
usize_evidence
comparison_payload
reason_vocabulary
status_flags
follow_on
summary
```

The closeout keeps the earlier comparison payloads and signed delta/reason
fields unchanged.

## Stop Line

This closeout does not open:

- provider activation;
- host allocator replacement;
- hook installation;
- `#[global_allocator]`;
- worker/TLS behavior;
- remote-free stress;
- atomic bitmap execution;
- native allocator replacement claims.

## Follow-On

```text
MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-001:
  add a hako pure-first EXE memory-use evidence runner that builds a selected
  comparison `.hako` app to an exact-MIR EXE, runs that EXE, records peak RSS /
  exit status / output-summary evidence, and keeps provider activation, host
  replacement, hooks, TLS, atomics, and allocator replacement parked.
```

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
