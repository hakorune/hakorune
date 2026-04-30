# P20: delete duplicate provider-min helper

Scope: remove the uncalled `tools/selfhost/gen_v1_from_provider_min.sh`
helper.

## Why

The helper duplicated `tools/selfhost/gen_v1_from_provider.sh` with the same
minimal return-42 Program(JSON v0) payload and the same provider
`env.mirbuilder.emit` route.

`rg --fixed-strings 'tools/selfhost/gen_v1_from_provider_min.sh' .` found no
callers, so keeping both names only widens the Program(JSON v0) helper surface.

## Decision

Delete the duplicate min helper and keep
`tools/selfhost/gen_v1_from_provider.sh` as the active provider v1 fixture
producer.

This does not change any smoke route or accepted shape.

## Acceptance

```bash
! rg --fixed-strings 'tools/selfhost/gen_v1_from_provider_min.sh' tools src docs/development/current/main --glob '!docs/development/current/main/phases/phase-29cv/P20-DELETE-DUP-PROVIDER-MIN-HELPER.md'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
