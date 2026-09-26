# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D11

Status: design_stop__2026-09-25
Date: 2026-09-25
Parent: workstream row H (unified resume, gate 1)
Mode: design_stop — census and Decision only.

## Blocking observation

```text
[freeze:contract][callable-semantic-lowering/placement-local-missing]
```

`json_stream_aggregator` advanced past
`static-result-ingress/no-exact-static-target` (env.get now emits
ExternCall) and now stops at the next honest terminal inside the
callable-semantic lowering lane.

## Known context

- `local_placement` (map_local.rs) looks up
  `self.locals[statement][ordinal]`; `locals` is populated from
  `owner.declaration_sites()` → `SourceBindingSiteV1::Local` rows
  in the resolved-semantics declaration inventory.
- `placement-local-missing` = a `local` statement was lowered on
  the callable lane whose statement site is absent from the
  declaration inventory.
- Prime suspect: `local dbg = env.get("HAKO_STAGEB_DEBUG")` inside
  a **bare block** `{ ... }` in
  `lang/src/shared/common/string_helpers.hako` (`starts_with`,
  `starts_with_kw`). Bare-block locals may not produce
  `Local` declaration sites, or their site identity may differ
  from the lowered statement site.
- Second suspect: any local statement whose site key differs
  between the resolved walk's registration and the callable
  lane's `current_callable_site_v1` locator.

## Census questions

1. Which exact statement hits the freeze (function + site path)?
2. Does the resolved-semantics walk register `local` declarations
   inside bare `{ }` blocks? If yes, under which site key?
3. Is the gap a walk coverage issue (locals inside bare blocks
   never registered) or a site-identity mismatch (registered
   under a different key)?
4. Existing owner for nested-scope locals — is there a designed
   scope-resolution authority, or is bare-block local a newly
   observed shape needing a bounded slice?

## Bounded surface

Read-only census across `resolved_semantics` declaration-site
issuance, `CallableSemanticLoweringState` locals population, and
the callable lane's site locator. Does not touch the env route
(landed), the ordinary-new lane, or static-result ingress.

## Exit

- [ ] Failing statement identified + whether the declaration
      walk covers bare-block locals named (or NoSafeSlice with
      reopen trigger).
- [ ] One bounded S-card emitted; pointers synced.
