# Dynamic full-body Recipe boundary

This directory owns the bounded source-to-Recipe path for the unchanged
resolver-backed Dynamic Loop cohort.

```text
complete resolver source inventory
  -> deterministic complete V2 Recipe candidate
  -> atomic source/Recipe/Dynamic-envelope co-seal
```

## Owners

- `mapping.rs` owns the deterministic logical V2 Recipe mapping.
- `claims.rs` owns the private complete role-to-Recipe claim table.
- `coseal/coverage.rs` consumes and validates all six binding roles and all
  twenty-eight source roles.
- `coseal/calls.rs` binds I6 and I7 to exact Dynamic envelopes by resolver
  owner plus exact source call site.
- `dynamic_invocation_contract` remains the complete immutable envelope
  catalog owner. This directory borrows it and never copies targets or
  selector semantics.

The semantic source batch owns the exact relation between a catalog callable
and its invocation-local resolver owner. Tests and production integration must
obtain the candidate from that same source authority; equal-looking source
resolved in another session is foreign.

## Acceptance rule

The current fixture has seven Dynamic envelope rows. Exactly two are selected
for this Recipe and the other five remain valid unselected catalog rows.
Seven and two are fixture evidence, not language-wide catalog cardinalities.
Additional valid rows, including rows for the same callable owner, do not
invalidate exact I6/I7 lookup.

If an unchanged valid source row exceeds this boundary, widen the compiler or
stop at a named design question. Never rewrite or narrow the source fixture.

## Non-authority

This directory does not own:

- selector-specific type refinement;
- the iteration-local `ch` Home relation;
- JoinSigV2, continuation, or Dynamic Fault compatibility;
- Callable Tail, Completion consumption, or return ABI;
- Builder, MIR, CFG, PHI, provider selection, runtime invocation, retry, or
  fallback.

The verified co-seal product is therefore not a final semantic program.
