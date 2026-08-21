---
Status: Design stop — exact membership, canonical lookup authority, and a production caller are missing
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-A-SOURCE-CAPABILITY-D0
Parent: docs/development/current/main/investigations/script-direct-static-a-issuer-boundary-d0-2026-08-21.md
ProductionCaller: none; the current vm-reference caller is reference-only
ReplacementCell: source authority -> exact pre-capability membership -> private one-shot candidate capability -> named A consumer
Classification: premise-reset design stop; no A/C/physical product or implementation row is selected
NextCard: none; remain on this card until one real positive source/catalog basis and one production caller are selected
---

# SCRIPT-DIRECT-STATIC-A-SOURCE-CAPABILITY-D0

## Six-line brief

Decision: Conditionally accept the source-plan HRTB loan and private one-shot capability pattern, but reject implementation while the capability would have to discover its own semantic window or escape as `Ready`. Exact direct-static membership must be issued first; only its candidate arm may enter capability/A.

Source authority + canonical issuer: one move-bound owner co-seals `SourceEnvelopeReady`, `SealedNormalScriptSourceV1`, and their opaque parser-invocation relation. A neutral window issuer, resolver result, and owned target/result authority feed one total `CanonicalScriptDirectStaticMembershipIssuerV1`; its candidate arm alone may call private `CanonicalScriptASourceCapabilityIssuerV1::issue_into_named_a(...)`.

Non-authority: parser rows, source name/digest/ordinal/pointer, `ScriptSyntaxViewV1`, caller-built `VerifiedScriptRootDemandWindowV1`, Builder semantic/window products, pointer-branded Script target inventory, empty/default catalogs, old Recipe, physical/publication owners, and local green tests cannot issue canonical membership or A meaning.

Fail-fast boundary: replace the current edge between `SourceEnvelopeReady` and `prepare_script_recipe()` with one total pre-effect selector. Candidate enters capability once; a proved residual may enter only the registered old-Recipe compatibility owner before capability; deferred, incomplete, invalid, or capability/A failure never reaches Recipe, raw, retry, or fallback.

Smallest next slice: stay on this D0 and name one admitted positive source/catalog pair, the owned canonical target/result supplier, every membership arm, one selected production caller, and the exact residual/sunset set. Do not create a child D0, code, fixture, fallback, production switch, or semantic receipt until those premises close.

Non-claims: no source-shape expansion, A complete-zero or public C noncandidate product, Recipe/Join, physical Call, publication/Return, residual retirement, backend parity, ABI, or performance claim.

## Premise reset

The proposed transport/capability split is useful, but it does not close the
missing semantic unit. Three current facts prevent an implementation slice.

1. `CanonicalScriptSourceRowsV1` is a total parser-syntax inventory. It records
   top-level ordinal and broad syntax kind, not the retained runtime/semantic
   window. `VerifiedScriptRootDemandWindowV1::seal(...)` only checks rows given
   by its caller. Neither is the canonical window issuer.
2. The current normal Script source plan rejects non-Main `Box`, `Using`, `Import`,
   `Brand`, `Enum`, `TypeAlias`, `Global`, and `StaticConst` top-level shapes.
   The existing direct-static positive fixture supplies its target through a
   same-source `static box Helpers`. Therefore no current evidence proves a
   positive direct-static member inside the admitted canonical Script cohort.
3. The only non-test caller of canonical core dispatch is fenced by the
   `vm-reference` feature. It is a reference caller, not a selected/default
   production caller, so it cannot satisfy an I0/CUT0 replacement cell.

The counterexample is decisive:

```text
current admitted Script cohort
  + same-source `static box Helpers` positive fixture
  = source-plan rejection, not a canonical A candidate
```

Creating a capability first would hide this mismatch behind a new receipt.
The exact source/runtime membership and target supplier must be observable
before the capability can exist.

## Accepted authority chain

The accepted direction is one linear chain with a partition before A:

```text
SourceEnvelopeReady
+ SealedNormalScriptSourceV1
+ opaque parser-invocation relation
  -> move-bound CanonicalScriptASourceAuthorityV1
  -> consuming private source loan
  -> CanonicalScriptMembershipWindowIssuerV1
  -> resolver Complete | Deferred(cause, site)
  -> CanonicalScriptDirectStaticMembershipIssuerV1
       Candidate
         -> private CanonicalScriptASourceCapabilityV1
         -> immediate CanonicalScriptAObservationIssuerV1::consume(...)
         -> A.DirectStaticSourceReady
       Residual(reason)
         -> registered pre-capability Recipe compatibility owner
       Deferred / Incomplete / IntegrityInvalid
         -> stop before Recipe/effects
```

This card supersedes the implementation assumption that every Script must
first produce `A.CompleteNoDirectStaticRows` and then use
`C.NonCandidate -> old Recipe`. That edge is post-capability fallback and is
forbidden. A call-free or genuinely empty Script may be a source-backed
`Membership.Residual` only after the sole selector proves complete coverage;
it is never inferred from `Vec::new()`, `None`, or a missing catalog.

The residual branch is not a second ingress. It is allowed only under the
production compatibility-owner sunset law:

- the production selector is exactly one;
- candidate and residual are total and pairwise-disjoint;
- selection happens before capability, Builder effects, Recipe, or physical
  work;
- the residual has an exact source surface, live owner, registry row, sunset
  row, and `retire_when` condition; and
- candidate/deferred/incomplete/invalid/A errors have zero edges to residual,
  old Recipe, raw, retry, or compatibility.

`TargetOutsideCatalog` is not silently changed into a residual reason. The
current behavior treats it as an error; changing that behavior requires a
separate reference Decision. Only absence from a complete catalog may be
classified according to an already accepted membership rule.

## Source-loan contract

The source owner must be consumed, not replayably borrowed. The design shape
is deliberately narrower than a public generic callback:

```rust
impl CanonicalScriptASourceAuthorityV1 {
    fn consume_with_canonical_a_source(
        self,
        use_loan: impl for<'src> FnOnce(
            CanonicalScriptASourceLoanV1<'src>,
        ) -> CanonicalScriptMembershipDraftV1,
    ) -> Result<CanonicalScriptMembershipOutcomeV1,
               CanonicalScriptSourceLoanErrorV1>;
}
```

The method and all loan fields remain module-private. The callback returns one
fixed AST-free draft type, not arbitrary `R`. `self` prevents a second source
scan through the same owner. HRTB prevents a safe borrowed AST reference from
escaping, but HRTB alone does not prevent cloning, pointer extraction, or
side effects; private constructors, an item cursor, fixed output fields, and
structural guards must close those channels.

```rust
struct CanonicalScriptBorrowedItemV1<'src> {
    site: SourceStmtSiteV1,
    node: &'src ASTNode,
}

struct CanonicalScriptItemCursorV1<'src> {
    // private; lends site + node as one item
}
```

The parser invocation witness is primary identity. Source path and digest are
integrity evidence. Ordinal may validate complete order inside this cursor,
but it is never a pairing key. AST pointers and address-derived `usize` values
are forbidden in the authority, membership, capability, and A products.

## Owner map

| Owner | Owns | Must not own |
| --- | --- | --- |
| parser/source envelope | parser witness, source identity/digest/profile/read-parse transport | semantic window, target membership, A |
| sealed normal Script source | owned AST and retained source-plan lineage | target selection, Recipe meaning after cutover |
| membership-window issuer | exact admitted runtime items and complete source-site coverage | Builder work-plan choice, target lookup |
| resolver kernel | forest construction and `Complete` or typed `Deferred(cause, site)` | source admission, membership, capability issuance |
| static lookup authority | owned AST-free complete target/result catalogs and their relation | source-site pairing, candidate/noncandidate issuance |
| direct-static membership issuer | one total/disjoint candidate/residual/error partition | A facts, Recipe keys, physical IDs |
| private capability issuer | atomic co-seal of an already selected candidate and immediate move to A | source classification, public `Ready`, storage/retry |
| A observation issuer | one AST-free candidate observation consumed once | AST rescan, noncandidate fallback, physical effects |
| registered residual owner | exact pre-capability old-Recipe compatibility surface | candidate retry, second ingress, surface growth |
| physical/continuation consumers | consume future A/C products through existing kernels | second target resolver, argument matcher, Return writer |

The current Builder-owned `VerifiedScriptDirectStaticCallTargetInventoryV1`
is not the static lookup authority. It address-brands AST/window/declaration/
import products and also issues target/noncandidate rows. Reusing it would
combine identity, membership, and A classification under a selected Builder
owner.

## Finite state and edge table

| State | Sole owner | Next edge | Old Recipe |
| --- | --- | --- | --- |
| `Transport.SourceEnvelopeReady` | envelope owner | bind source authority once | temporary unconditional edge exists only before cutover |
| `SourceAuthority.Bound` | private facade | consume source loan once | no independent discard/retry API |
| `Membership.Candidate` | membership issuer | private capability issuer only | forbidden |
| `Membership.Residual(reason)` | same total selector | registered residual owner | allowed before capability only |
| `Membership.ObservationDeferred(cause, site)` | resolver outcome preserved by selector | typed stop | forbidden by default; never failure fallback |
| `Membership.Incomplete(error)` | coverage validator | typed stop | forbidden |
| `Membership.IntegrityInvalid(error)` | co-seal verifier | typed stop | forbidden |
| `Capability.Ready` | private capability issuer | immediate named A consumer in the same facade | forbidden; cannot escape/store |
| `Capability.Consumed` | named A consumer | one A result | forbidden; no replay |
| `A.DirectStaticSourceReady` | A issuer | future C/direct consumer | forbidden |
| `Residual.Consumed` | registered compatibility owner | existing Recipe completion | candidate entry/retry forbidden |
| `NoSafeSlice` | design process | remain on this D0 | never encode as runtime state |

The resolver already has detailed `ShadowResolveErrorV0` causes and source
sites. The current `ResolveScriptForestOutcomeV1::Deferred` drops that detail.
The future neutral boundary preserves the existing cause; it does not invent a
new semantic authority or turn deferred into complete-zero.

## Contingent task queue

These are dependency rows, not selected execution cards. They remain inside
this rolling D0 until its acceptance conditions choose one bounded next slice.

### 0. Current D0 — close the premises

```text
Change:
  name one admitted natural positive source/catalog pair
  name the owned canonical target/result supplier
  enumerate every membership arm and transferred subtree
  select one real production caller/backend role
  name exact residual surface, owner, sunset, and atomic old-edge delete set

Done:
  source -> exact membership -> AST-free A -> fail-fast/residual is finite
  candidate and residual are total/pairwise-disjoint
  every candidate-required authority has one issuer and one live consumer

Stop:
  no positive intersection, no owned catalog, or no production caller
  => park the lane; do not create another D0
```

### 1. `SCRIPT-DIRECT-STATIC-A-SOURCE-OBSERVATION-I0-R0`

Classification: contingent BoxShape only. It is legal only if the existing
source-only Builder mapping is proven identical and keeps a live existing
consumer.

```text
Change:
  re-own one neutral Script membership-window issuer
  consume the source through the fixed private loan
  preserve resolver Deferred cause/site exhaustively

Retire in the same bounded series:
  the selected Builder-private duplicate window-issuance edge
  the selected unit-Deferred information-loss edge

Non-claims:
  no new source shape, target catalog, capability, A, or production switch
```

If the mapping differs, this is not BoxShape and the task returns to the
current D0.

### 2. Choose exactly one lookup prerequisite

`SCRIPT-DIRECT-STATIC-A-STATIC-LOOKUP-I0-R0` is a contingent BoxShape only if
an existing owned, AST-free, complete target/result relation can be re-owned
for a live consumer without changing accepted source shapes. Its R0 retires
the pointer-branded Script inventory as authority on the selected edge.

`SCRIPT-DIRECT-STATIC-A-SOURCE-ADMISSION-I0` is instead a BoxCount if the only
real positive requires admitting `static Box` or another currently rejected
source shape. It must own one exact shape plus positive/negative/reference
evidence. It must not be combined with lookup re-ownership or capability
cutover.

If neither row has a live production responsibility and selected old edge,
neither row may be implemented as a disconnected proof.

### 3. `SCRIPT-DIRECT-STATIC-A-CANDIDATE-CUTOVER-I0-R0`

This is selectable only after tasks 0-2 provide a real production caller and
already-live branch owners.

```text
one production selector
  Candidate
    -> private capability
    -> immediate named A consumer
    -> existing direct-static physical/publication owner
  Residual
    -> registered old-Recipe compatibility owner
  Deferred / Incomplete / IntegrityInvalid
    -> fail before effects

same-commit R0:
  delete SourceEnvelopeReady -> unconditional prepare_script_recipe()
  delete selected pointer/Builder authority edge
  register exact residual surface and sunset

guards:
  production caller >= 1
  selected unconditional old edge = 0
  capability issuer = 1; named A consumer = 1; orphan Ready storage = 0
  candidate/error -> old Recipe/raw/retry/fallback = 0
```

This cutover does not claim final old-Recipe retirement while a registered
residual remains.

### 4. `SCRIPT-DIRECT-STATIC-A-RESIDUAL-RETIREMENT-I0-R0`

For each registered residual arm, provide its source-backed Facts/Recipe and
named completion owner, then remove that residual old-Recipe caller. The final
row closes only when canonical `prepare_script_recipe()` callers, old stage/
error/restore edges, fallback, and retry are all zero. A residual cannot grow
after registration.

## Acceptance evidence for this D0

Close this D0 and select one contingent implementation row only when all are
observable:

1. the semantic unit and exact runtime membership map every source-plan
   classifier arm, transferred/opaque subtree, and source-window type demand;
2. one admitted positive direct-static source reaches a complete owned
   target/result supplier without same-source rejected declarations;
3. one selected/default production caller is named; `vm-reference` alone is
   insufficient;
4. the consuming fixed-output loan, private cursor, opaque identity relation,
   and AST/pointer exclusion are fixed;
5. resolver `Deferred` retains its existing cause/site and remains distinct
   from missing, invalid, residual, and complete-empty;
6. candidate/residual/error is total and pairwise-disjoint, with
   `TargetOutsideCatalog` behavior unchanged unless separately accepted;
7. capability `Ready` is module-private and immediately moved to exactly one
   A consumer; and
8. the unconditional old edge delete set and any residual registry/sunset are
   exact before an I0 begins.

## NoSafeSlice conditions

Remain in `design_stop` while any condition holds:

- no positive candidate exists inside the admitted canonical Script cohort;
- canonical target/result inputs exist only as pointer-branded Builder
  products or caller-built empty/default catalogs;
- no selected/default production caller exists;
- parser rows or a caller-provided window are treated as semantic membership;
- source admission BoxCount and lookup/window BoxShape are mixed;
- the loan uses replayable `&self`, returns arbitrary `R`, exposes AST fields,
  or relies on HRTB alone to forbid clones/pointers/side effects;
- resolver deferred cause/site is lost or converted to zero/residual by
  default;
- actual empty and missing coverage share one representation;
- target absence, missing authority, and integrity contradiction are merged;
- capability issues candidate/noncandidate meaning, escapes `Ready`, can be
  stored, or has more than one consumer;
- `A.CompleteNoDirectStaticRows` or `C.NonCandidate` reaches old Recipe after
  capability starts;
- residual selection is not pre-capability, total, disjoint, registered, and
  sunset-bound; or
- the candidate path can retry old Recipe, raw, compatibility, ordinary
  static lowering, or a second resolver.

No code, fixture, fallback, production switch, or new semantic
`Verified*`/`Prepared*` receipt is authorized while this list is nonempty.

## Review receipt

The external proposal is accepted only for its phase separation, opaque
parser identity, consuming source loan, and private immediate capability-to-A
handoff. Two independent read-only worker audits found the missing premise:
the current parser rows are syntax, the resolver is a borrowed kernel, the
Builder window/target products are separate authorities, the current positive
fixture is outside the admitted Script cohort, and the only canonical caller
is reference-only. They also found that resolver detail already exists before
the current unit `Deferred` outcome and should be preserved rather than
reissued.

The resulting Decision is therefore conditional, not an implementation
approval. The current row remains `SCRIPT-DIRECT-STATIC-A-SOURCE-CAPABILITY-D0`
with `next_execution_card = none`.

## References

- `docs/development/current/main/investigations/script-direct-static-a-issuer-boundary-d0-2026-08-21.md`
- `docs/development/current/main/investigations/script-direct-static-a-consumer-bind-d0-2026-08-21.md`
- `docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md`
- `src/parser/callable_parameter_source/script_source_rows_model.rs`
- `src/parser/callable_parameter_source/script_source_rows.rs`
- `src/mir/compiler/canonical_core_dispatch.rs`
- `src/mir/compiler/normal_source_plan/classifier.rs`
- `src/mir/compiler/normal_source_plan/product.rs`
- `src/mir/resolved_semantics/owner_resolver.rs`
- `src/mir/resolved_semantics/shadow/product.rs`
- `src/mir/resolved_semantics/shadow/script_root_window.rs`
- `src/mir/source_call_target/script_direct_static.rs`
- `src/mir/source_call_target/script_direct_static_tests.rs`
