# Generic raw structured static-call result publication I1 D0

Status: `design accepted 2026-08-07; implementation remains closed until I1/R0`

Parent evidence:

- `generic-raw-structured-static-call-result-publication-d0-task-2026-08-07.md`
- `generic-raw-structured-static-call-result-publication-i0-r0-implementation-task-2026-08-07.md`
- `docs/reference/mir/generic-loop-stage-matrix.md`

## Decision

Use the existing module candidate session as the sole rollback owner.  Do not
add a route-local snapshot transaction.  Seal the canonical source/site and
one current-owner static terminal before physical effects; after effects,
failure is terminal `Freeze` and the outer candidate is discarded.  The
registry must not advance through `PostEffectRetryDebt` for this activation.

## Required authority split

```text
whole-source exact caller/site proof
  -> activation demand
current-owner terminal physical Call
  -> CompletedUnifiedValueCallEmissionV1
publication consumer
  -> type_ctx result type
local materializer
  -> Copy + metadata propagation only
GenericLoop
  -> verifier only
```

The source product remains a locator and cannot carry `ValueId`, Builder
state, or inferred type.  The physical receipt remains the only destination
authority.  No method-name, owner-name, route-string, final metadata, or
runtime-value selector is allowed.

## Contract

The exact source demand is `(Cataloged StringHelpers.int_to_str/1,
Body(0).Initializer(0), target StringHelpers.to_i64/1)`.  The current-owner
static terminal alone emits the physical Call and returns
`CompletedUnifiedValueCallEmissionV1`; its destination is the only `ValueId`
authority.  Publication consumes that receipt once and writes `Integer` to
`type_ctx`.  The local materializer transports Copy/metadata only, and
GenericLoop verifies only.

`ModuleBuilderInvocationSessionV1`/`CanonicalModuleLoweringSessionV1` owns
rollback of variable/binding state, every `TypeContext` lane, physical blocks
and cursors, cleanup/SSA scratch, and core IDs.  The existing
`with_saved_variable_map_typed` helper is not a rollback owner and must not be
used as one for activation.

## Fail-fast boundary

Admit exactly one canonical source site first.  Success requires a fresh
strict VM probe to pass the previous `MissingTransientType` boundary and to
show the same caller/site/target/destination/type relation in the post-call
local path.  Foreign site/root, missing catalog, another terminal, alternate
route, failed physical Call, duplicate publication, post-effect debt, and
rollback residue are typed reject or terminal-freeze outcomes.

Out of scope:

- broad unannotated-call inference;
- nested required-argument result publication;
- GenericLoop backfill or route-specific type guessing;
- retry/fallback or compatibility-route promotion;
- legacy deletion, backend widening, or generic monomorphization.

## Implementation gate

- one I1/R0 implementation switches the canonical caller and removes its
  post-effect retry edge;
- success and failed-candidate fixtures prove the outer session is discarded;
- focused publication/terminal tests and a fresh strict VM receipt pass;
- current-state, workstream, and Generic reference mirrors are updated in the
  same implementation commit;
- no production caller is counted before the complete receipt and guard pass.

The design stop is closed; no broader Generic inference, nested required
argument, GenericLoop backfill, retry/fallback, legacy deletion, backend
widening, or monomorphization is implied.
