---
Status: Design stop — canonical source capability owner is missing; no implementation
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-A-SOURCE-CAPABILITY-D0
Parent: docs/development/current/main/investigations/script-direct-static-a-issuer-boundary-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: sealed Script source + envelope transport -> one AST-free A capability
Classification: design stop; no A observation, C disposition, or physical product
NextCard: none until this capability boundary is accepted
---

# SCRIPT-DIRECT-STATIC-A-SOURCE-CAPABILITY-D0

## Six-line brief

Decision: Existing `resolved_semantics` provides resolver kernels, but no
canonical source-capability owner can feed them without importing Builder
authority. Define that owner before implementing A observation.

Source authority + canonical issuer: `SealedNormalScriptSourceV1` owns the
retained source plan and `SourceEnvelopeReady` owns parser/source identity and
transport integrity. A future
`CanonicalScriptASourceCapabilityIssuerV1` must consume both once and issue one
AST-free, lifetime-safe capability for the later A issuer.

Non-authority: `ScriptSyntaxViewV1` by itself, `VerifiedScriptRootDemandWindowV1`
sealing, `owner_resolver.rs` kernel calls, Builder
`normal_script_root_demand_window.rs`/`normal_script_semantic_source.rs`,
parser rows alone, `RawScriptBodyRecipeV1`, AST/name/ordinal pairing,
`ValueId`/`MirType`, and empty/default windows cannot issue this capability.

Fail-fast boundary: after envelope validation and before the current
`prepare_script_recipe()` edge. Missing source capability, resolver deferral,
missing coverage, or foreign/duplicate/stale rows must stop before A meaning,
Recipe, physical entry, child effects, or publication.

Smallest next slice: freeze the capability fields, one source-owner callback,
one resolver-kernel invocation boundary, exhaustive capability states, and the
old-Recipe pre-A-only guard. Do not add code or a `Verified*`/`Prepared*`
receipt in this D0.

Non-claims: no A direct-static/noncandidate observation, C disposition, B
transport, Recipe/Join, physical Call, publication/Return, compatibility/raw
retirement, production switch, ABI/backend, performance, or source-shape
expansion.

## Why the existing resolver is not yet a capability owner

The only Script resolver entry is
`src/mir/resolved_semantics/owner_resolver.rs:132-168`:

```text
resolve_script_forest_with_declaration_views(
    ScriptSyntaxViewV1<'_>,
    &VerifiedScriptRootDemandWindowV1,
    RecordSchemaDemandV1,
    EnumVariantDemandV1,
    EnumMatchDemandV1,
    VerifiedBrandProgramDeclarationCatalogV1,
)
```

It is a useful kernel, but it accepts an AST-borrowing `ScriptSyntaxViewV1`, a
window with semantic entries, and declaration-demand providers. It does not
own their source admission, identity, completeness, or cross-product seal.
`ScriptSyntaxViewV1::from_program` (`resolved_semantics/script_view.rs:18-27`)
is therefore not an issuer. `VerifiedScriptRootDemandWindowV1::seal`
(`resolved_semantics/shadow/script_root_window.rs:176-199`) only validates a
caller-provided list; it cannot manufacture the semantic entries.

The existing Builder path is a separate authority:

```text
normal_script_root_demand_window.rs
  -> normal_script_semantic_source.rs
  -> owner_resolver::resolve_script_forest_with_declaration_views
```

Those products may remain evidence for the selected-normal lane, but copying
or re-pairing them into canonical A would create a second source authority.
`resolve_forest` for a generic Function root is not a Script-root substitute,
and an empty declaration/window map is not a valid neutral capability.

## Source and callback boundary

The transport owner and the retained-source owner must not be flattened.

```text
CanonicalScriptSourcePlanEnvelopeV1
  -> into_a_transport_parts(self)       # proposed move-only transport seam

SealedNormalScriptSourceV1
  -> with_a_source_view(|view| {         # proposed HRTB/lifetime-bound seam
       CanonicalScriptASourceCapabilityIssuerV1::issue(view, transport)
     })
  -> CanonicalScriptASourceCapabilityV1 # AST-free output only
```

`into_a_transport_parts(self)` transfers the already checked parser rows,
source identity/digest, profile, read/parse receipt, and envelope seal as one
unit. It issues no semantic A meaning. The `normal_source_plan` callback may
lend a `ScriptSyntaxViewV1` and retained source sites to the resolver kernel,
but the callback must return only owned/verified AST-free rows; no AST pointer
may escape into dispatch state, a capability field, or a later physical owner.

The single capability issuer is responsible for co-sealing:

- source identity, parser witness, digest/profile/read-parse receipt;
- retained Script window identity and complete source-site coverage;
- a canonical resolver-kernel input and its explicit `Complete`/
  `Deferred` outcome;
- declaration, Brand, and import demand views with source identity;
- canonical static-target/result capability inputs;
- ordered child/argument-site and terminal (`FinalSequence`/
  `RootReturn`) capability inputs needed by the later A issuer.

The capability does not itself classify direct-static calls, emit a
noncandidate row, choose C, or issue a physical identity. It is the one
source-backed input boundary that makes those later decisions possible without
AST/name re-resolution.

## Exhaustive capability state table

The capability state is separate from A/C source disposition. Every state has
one owner and no wildcard/`None`/empty-default merge is allowed.

| State | Sole issuer / authority | Pre-effect behavior | Allowed terminal | Old Recipe/Builder fallback |
| --- | --- | --- | --- | --- |
| `PreCapability.SourceEnvelopeReady` | envelope transport | no capability meaning yet | enter one capability attempt | old Recipe only before A starts |
| `Capability.SourceAuthorityUnavailable` | source-plan seam | stop before resolver/A | typed discard / `NoSafeSlice` | forbidden |
| `Capability.ObservationDeferred` | resolver kernel outcome | retain explicit deferred cause | typed defer/discard | never Complete/zero |
| `Capability.Incomplete` | capability coverage validator | stop before A/Recipe/effects | typed discard / `NoSafeSlice` | no empty window |
| `Capability.IntegrityInvalid` | capability co-seal verifier | stop before effects | typed discard / `NoSafeSlice` | no repair/re-pair |
| `Capability.Ready` | single capability issuer | move capability once to A | later A issuer only | no Builder import |
| `Capability.Consumed` | A issuer handoff | no replay or second scan | A observation path | no old Recipe/retry |
| `Capability.Discarded` | candidate/session owner | no publication/physical effect | rejected candidate | no resurrection |
| `NoSafeSlice` | design boundary | stop before implementation | remain on D0 | never encode as neutral |

`ObservationDeferred` is not `SourceAuthorityUnavailable` and is not a clean
zero result. `Incomplete` is not a candidate decline. `IntegrityInvalid` is not
repairable by matching names or ordinals. These distinctions must survive the
later A -> C -> B state machine.

## Old-Recipe retirement edge

The only current production edge is still:

```text
PreCapability.SourceEnvelopeReady
  -> discard_before_a_consumer()
  -> prepare_script_recipe()
```

It is legal only while capability/A has not started. Once the capability and A
consumer are enabled, the edge must be deleted atomically and its production
caller count must be zero. Capability unavailable, deferred, incomplete,
invalid, ready, or consumed states may never fall through to the old Recipe,
raw, or compatibility routes.

## Acceptance for this D0

Accept only when:

1. the envelope transport owner, retained-source owner, capability issuer,
   resolver kernel, and later A consumer are named separately;
2. the move-only envelope seam and HRTB source-view seam are fixed, with no
   AST-bearing compiler field or pointer escape;
3. all capability inputs share one source identity/witness/digest and the
   resolver `Complete`/`Deferred` result is retained without downgrading;
4. the state table is represented by a focused guard with explicit
   unavailable/deferred/incomplete/invalid/neutral/consumed terminals;
5. Builder window/semantic products are proven non-authoritative for canonical
   A and no by-name/ordinal/pointer re-pairing is permitted;
6. `prepare_script_recipe()` is marked pre-capability-only with an exact
   caller-zero guard for the future cutover; and
7. the future capability and A children keep `canonical_core_dispatch.rs`
   below 760 lines and every touched Rust source below the 800-line hard stop.

## NoSafeSlice conditions

Remain on this D0 if the callback cannot provide a complete source view, if
the resolver kernel requires Builder-owned semantic products, if a window is
inferred from parser row count, if `Deferred` becomes complete/zero, if any
AST pointer escapes, if fields are paired by name/ordinal/digest/pointer, if
the capability has no named A consumer, or if the old Recipe edge remains a
fallback after capability starts. No code, fixture, fallback, or guessed
receipt is authorized while any condition holds.

## Non-claims and parked work

- No capability implementation or new semantic `Verified*`/`Prepared*` type.
- No A direct-static/noncandidate census, C disposition, B transport, or
  physical/publication owner.
- No selected Builder parity claim or reuse of its window/semantic source.
- No compatibility/raw retirement, source-shape expansion, ABI/backend,
  performance, or loop physicalizer cleanup.

## Worker review receipt

Two read-only audits agree that `resolved_semantics` is a kernel, not a
canonical capability owner: its Script entry requires AST view, semantic
window entries, and declaration demands supplied by another authority. The
Builder window/semantic source cannot be imported into canonical A, and the
current dispatch has exactly one non-test old-Recipe caller. This card freezes
the missing capability boundary; it does not authorize implementing the
callback, moving AST storage, or creating A/C products.

## References

- `docs/development/current/main/investigations/script-direct-static-a-issuer-boundary-d0-2026-08-21.md`
- `docs/development/current/main/investigations/script-direct-static-a-consumer-bind-d0-2026-08-21.md`
- `src/mir/compiler/canonical_core_dispatch.rs`
- `src/mir/compiler/canonical_script_source_plan_envelope.rs`
- `src/mir/compiler/canonical_script_source_a_input.rs`
- `src/mir/compiler/normal_source_plan/product.rs`
- `src/mir/resolved_semantics/owner_resolver.rs`
- `src/mir/resolved_semantics/script_view.rs`
- `src/mir/resolved_semantics/shadow/script_root_window.rs`
- `src/mir/builder/normal_script_root_demand_window.rs`
- `src/mir/builder/normal_script_semantic_source.rs`
