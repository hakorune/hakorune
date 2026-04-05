# Phase 132x SSOT: vm default backend decision

- Status: SSOT
- Date: 2026-04-05
- Scope: remove `vm` from the default backend after the explicit legacy vm migration.

## Decision Frame

- The default `vm` backend is the last remaining legacy public gate.
- Caller bucketization is complete enough to change the default now.
- Remove the default first; do not wait for full vm source retirement.
- Keep explicit vm / vm-hako proof-debug callers alive while mainline work resumes.
- Update callers, docs, and help text together.

## Caller Buckets

- move to mainline / route-first candidates
  - `tools/using_e2e_smoke.sh`
- keep now
  - `tools/selfhost_json_guard_smoke.sh`
  - `tools/selfhost_parser_json_smoke.sh`
  - `tools/using_unresolved_smoke.sh`
  - `tools/using_resolve_smoke.sh`
  - `tools/using_strict_path_fail_smoke.sh`
  - `tools/selfhost_read_tmp_dev_smoke.sh`
  - `tools/ny_selfhost_inline.sh`
  - explicit proof/debug/compat callers
  - vm-hako reference/conformance callers
  - route observability and direct bridge probes that still intentionally observe vm-family behavior
- delete/archive candidate
  - none in the active tree at this point; archive-only evidence already lives under `tools/archive/**`

## Safe Order

1. change `args.rs` default and help text
2. align public docs and error wording
3. update route-first candidates where semantics do not depend on vm-family behavior
4. run proof and close out the lane

## Success Criteria

- the default backend is no longer `vm`
- public docs/help match the chosen behavior
- no caller is left depending on an accidental default
- the last route-first candidate is moved or intentionally frozen

## Not In Scope

- vm-hako interpreter recut
- product/native route work
- unrelated cleanup lanes
