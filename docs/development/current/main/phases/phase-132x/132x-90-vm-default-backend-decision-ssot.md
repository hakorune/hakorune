# Phase 132x SSOT: vm default backend decision

- Status: SSOT
- Date: 2026-04-05
- Scope: decide whether `args.rs` should keep `vm` as the default backend after the explicit legacy vm migration.

## Decision Frame

- The default `vm` backend is the last remaining legacy public gate.
- Do not change it until omitted-backend caller inventory is complete.
- If the default is retained, document it as an explicit legacy keep/debug default.
- If the default is changed, update callers, docs, and help text together.

## Safe Order

1. inventory callers that omit `--backend`
2. decide keep vs change for the default backend
3. update `args.rs`, docs, and smoke callers in one shot
4. run proof and close out the lane

## Success Criteria

- the default backend decision is explicit and recorded
- public docs/help match the chosen behavior
- no caller is left depending on an accidental default

## Not In Scope

- vm-hako interpreter recut
- product/native route work
- unrelated cleanup lanes
