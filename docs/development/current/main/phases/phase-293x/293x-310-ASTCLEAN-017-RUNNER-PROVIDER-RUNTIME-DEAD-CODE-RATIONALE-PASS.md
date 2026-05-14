# 293x-310 ASTCLEAN-017 runner/provider/runtime dead_code rationale pass

Status: complete

## Decision

Decision: accepted.

Runner/provider/runtime shelves are allowed only when they are explicit optional
routes or diagnostic surfaces. Live APIs should not carry stale
`#[allow(dead_code)]`, and unused wrapper helpers should be removed rather than
kept as unowned compatibility residue.

## Scope

- Remove stale `dead_code` allowances from live runner/provider APIs.
- Delete unused runner/provider helper wrappers where no caller exists.
- Delete unused plugin special-method helper functions from plugin loader v2.
- Keep optional stage0 VM capture and plugin diagnostic surfaces only with
  `ASTCLEAN-017` rationale comments.

## Non-goals

- No language surface change.
- No provider-selection behavior change.
- No plugin ABI behavior change.
- No `LOCALTYPE-001` implementation in this cleanup row.

## Guard

- `tools/checks/k2_wide_astclean_runner_provider_runtime_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_runner_provider_runtime_guard.sh` passed locally.
