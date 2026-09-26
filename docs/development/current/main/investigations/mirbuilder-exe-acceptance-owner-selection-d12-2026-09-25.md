# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D12

Status: design_stop__2026-09-25
Date: 2026-09-25
Parent: workstream row H (unified resume, gate 1)
Mode: design_stop — census and Decision only.

## Blocking observation

```text
[freeze:contract] loop winner selection declined: zero selected
family candidates
```

`json_stream_aggregator` advanced past
`callable-semantic-lowering/placement-local-missing` (nested
`{ }` local now resolves its `locals` row) and now stops at the
next honest terminal: `issue_loop_node_winner_recipe_v1` returns
`Declined` for a walked loop in
`joinir/route_entry/router.rs:198-205`.

## Known context

- The router establishes exact loop membership
  (`resolver loop inventory` match on condition+body structure)
  before winner selection — the freeze fires AFTER membership
  succeeded, i.e. the walked loop IS in the inventory but no
  selected family candidate admits it.
- Loop family admission is decided by `issue_loop_node_winner_
  recipe_v1` — the one authority for family winner selection on
  this lane. `Declined` = zero selected candidates; distinct
  from `Unresolved`/`Rejected` (spine terminal).
- The app's loop lives in `apps/json-stream-aggregator/main.hako`
  (and/or imported `StringHelpers` helpers) — shape unknown
  until identified.

## Census questions

1. Which exact loop statement declines (function + site + loop
   condition/body shape)?
2. Which family candidates were probed, and which named reject
   reason did each produce? (family reject vocabulary, not a
   guess)
3. Is the shape inside an existing family's bounded vocabulary
   (a missing admission edge) or genuinely unclaimed shape
   (NoSafeSlice → design)?
4. Existing owner for the shape — is there a designed family
   admission path the walked loop fails to reach, or is a new
   bounded family edge needed?

## Bounded surface

Read-only census across `issue_loop_node_winner_recipe_v1`
family candidates, the per-family reject vocabulary, and the
resolver loop inventory for the failing function. Does not
touch the nested-program item-site producer (landed), the env
route (landed), or static-result ingress.

## Census results

(to be filled)

## Decision

(to be filled)

## Exit

- [ ] Failing loop identified; family reject reasons enumerated.
- [ ] One bounded S-card emitted, or NoSafeSlice recorded.
