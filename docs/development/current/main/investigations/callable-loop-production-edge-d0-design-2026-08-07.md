# Callable Loop Production Edge D0

Status: design stop after the caller-zero DraftSeal canary.

Decision: do not promote the cfg(test)-only callable canary directly into
production. First fix one named production caller boundary and its old-edge
disposition. This row is docs-only; it does not open selector, Generic G0,
legacy scheduler, retry, fallback, or production activation.

## Sole source authority

```text
resolved callable source
  -> VerifiedCallableFunctionLoweringInputV1
  -> PreparedCallableLoopPhysicalizationV1
  -> profile-close evidence
  -> finish_profile_close
  -> finish_for_draft_seal
  -> DraftSeal prepare/commit
```

The existing callable chain is the only physical finish authority. The
cfg(test) canary is evidence, not a production input. The common physicalizer
remains unaware of callable Tail, ABI, Completion, Return, DraftSeal, and
production caller names.

## Non-authority and fail-fast boundary

Reject before any production switch when any of these is missing or foreign:

```text
exact callable owner/session/profile/ABI/Completion
one fresh unpublished function session
one profile-close receipt
one DraftSeal result
named production caller
```

Forbidden:

```text
AST or name re-walk
route-label or legacy-scheduler selection
Generic G0 adapter substitution
call-time discovery
retry/fallback
collector/module publication in this D0
```

## Required D0 evidence

1. Census one exact production caller candidate and its current old edge.
2. Record the source input, output receipt, owner/session boundary, and
   failure/discard behavior for that caller.
3. Define a thin named-caller adapter contract with no implementation.
4. Name the one old edge that a later I0/R0 will replace.
5. List parity and fresh-session fixtures required before opening the switch.

The selector remains closed. No current named caller means this row remains
`NoSafeSlice`; do not invent a by-name or compatibility route.

## Later implementation order

```text
D0: production caller census + exact old-edge disposition
D0: thin named-caller adapter contract
I0/R0: one caller switch and same-slice old-edge removal
M10b: production activation only after Generic G0/all required route gates
M11/M12: legacy scheduler/fallback deletion after activation evidence
```

## Documentation requirement

This design row updates the callable physical-demand/session SSOT and current
mirrors only. Any later caller switch must update the exact reference
documentation, diagnostics, migration note, and current pointers in the same
commit as the implementation.
