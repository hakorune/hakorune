---
Status: Design accepted — Candidate A selected; parser-only I0 opened
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-ADMISSION-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-static-mixed-postpass-policy-d0-2026-08-23.md
ProductionCaller: 0
ProductionEdit: none; entry-authority/census design only
CeremonyTier: T2 — program entry authority boundary
---

# NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-ADMISSION-D0

## Current capsule

- **Current decision:** `Main.main`/App admission is a separate program-level
  source decision. It must not be inferred by the static parent issuer or by
  the old Builder raw-root expansion.
- **Current implementation status:** static parent parser-only I0 and the
  static/mixed postpass policy are accepted. No entry consumer or production
  switch is open.
- **Next ordered task:** implement the parser-only I0 at the completed
  callable-source product boundary, then verify the disposition without a
  downstream consumer.
- **Production stop line:** no `root_is_app_mode` mutation, Main child/body
  lowering, NormalCompileRequest, Builder effect, Recipe/Join, MIR, runner
  entry selection, fallback, or compatibility retirement.
- **Retirement finish line:** one entry authority is named, the old
  `VerifiedRawRootExpansionV1` is either retired or explicitly kept as a
  noncanonical compatibility owner, and no name-only or second source scan
  remains on the selected path.

## Six-line brief

```text
Decision:
  make Main.main/App admission a parser-owned program-level disposition,
  separate from static-parent source coverage and separate from raw Builder
  expansion; keep the entry slice at one exact Main/main/0 cohort.
Source authority + canonical issuer:
  one same-invocation static-parent seal plus exact program-level uniqueness
  and direct callable relation; the sole issuer is the private
  ParserMainAppEntryAuthorityIssuerV1::issue_once at
  ParsedProgramWithCallableParameterSourceV1::new.
Non-authority:
  Main or main names alone, AST re-scan, raw VerifiedRawRootExpansionV1,
  root_is_app_mode, normal source-plan classification, Main-child selection,
  runner select_entry_function, Builder, MIR, runtime registry, and fallback.
Fail-fast boundary:
  after parser entry admission and before root_is_app_mode, registration,
  draft/collector, Main child/body lowering, NormalCompileRequest, or MIR.
Smallest next slice:
  one top-level static Main, exactly one direct static main/0, no fields/init/
  helper/constructor/generated members, no mixed program, typed disposition.
Non-claims:
  Script admission, multiple Main boxes, mixed programs, helper children,
  result semantics, resolver, Recipe/Join, physical lowering, publication,
  fallback, runner/backend behavior, and production switch.
```

## Why this is a separate authority

The static-parent source seal proves a bounded source shape and exact parser
relations. It does not mean that the method is the program entry. Conversely,
the current `VerifiedRawRootExpansionV1::from_program` scans the owned AST for
static Box declarations named `Main`, then expands the root and compatibility
children. It is an existing Builder-side source selector, not a parser-owned
entry receipt.

The current authority map is therefore:

```text
parser static parent source
  -> ParserStaticBoxParentSourceDispositionV1

program-level entry admission (this D0)
  -> proposed ParserMainAppEntryDispositionV1

semantic Main completion/result/ABI
  -> existing normal-source-plan owners, later

root/App physical lowering
  -> existing Builder/collector owners, later
```

The entry issuer may use the exact `Main`/`main` spelling as a closed
language-level admission rule, but the spelling is never an identity key. It
must be bound to the same parser invocation, exact Box path, exact member
site, and exact callable source row. A second AST walk or a `(name, ordinal)`
join is not an implementation of this design.

## Recommended disposition

The design-only target vocabulary is:

```rust
enum ParserMainAppEntryDispositionV1 {
    AppMainReady(ParserMainAppEntrySealV1),
    Outside(ParserMainAppEntryOutsideReasonV1),
    SourceAuthorityUnavailable(ParserMainAppEntryUnavailableV1),
    Incomplete(ParserMainAppEntryIncompleteV1),
    IntegrityInvalid(ParserMainAppEntryIntegrityIssueV1),
}
```

The type is not to be implemented from this card yet. Its intended meaning is:

| State | Meaning | Allowed next step |
| --- | --- | --- |
| `AppMainReady` | one exact static `Main.main/0` entry relation is complete | later named consumer only |
| `Outside` | complete source is outside the one-entry cohort | typed terminal; no retry |
| `SourceAuthorityUnavailable` | static parent or program relation is unavailable | typed terminal |
| `Incomplete` | required Main/header/method/arity evidence is missing | typed terminal |
| `IntegrityInvalid` | foreign, duplicate, stale, or contradictory relation | typed terminal |

`AppMainReady` is still source admission, not a semantic result contract or a
physical root-lowering permit. It must not set `root_is_app_mode` directly.

## Bounded cohort

The first admission cohort is deliberately smaller than the existing raw
expansion:

```text
one Program from one parser invocation
one top-level static Box whose exact source identity is retained
Box name = Main as an admission rule
one direct static method whose exact source identity is retained
method name = main as an admission rule
arity = 0
no fields, init/static-init, constructor, delegate, generated member, or helper
no interface/record/build-gate/import/inheritance/mixed program
```

All other shapes are typed `Outside` or a typed missing/integrity state. They
are not silently converted to Script, an ordinary source seal, or a legacy
entry route. The existing raw expansion may continue to serve its current
compatibility/test surface until an explicit retirement decision; that does
not make it the canonical source issuer for this cohort.

## Current authority census

The read-only audit found these current owners and consumers:

```text
VerifiedRawRootExpansionV1::from_program
  = existing Builder-side raw root/App selector

root_is_app_mode
  = Builder root lifecycle state, written after preflight

Main-child selection / lowering input
  = normal_callable_semantic_package install owner

runner select_entry_function
  = runtime/runner entry selection, not source admission

ParserStaticBoxParentSourceAuthorityIssuerV1
  = parser source issuer, one call site, no MIR consumer
```

The census must distinguish three things that are currently easy to conflate:

```text
parser source entry admission
semantic Main completion/result plan
physical root/runner entry selection
```

The new entry disposition may only own the first. It must not absorb the
second or third just because all three mention `Main.main`.

## Candidate comparison

### Candidate A — parser-owned entry issuer (recommended)

Consume the existing static parent source relation plus a same-invocation
program-level exact entry relation at one parser postpass boundary. Issue one
typed source disposition, transport it as a sibling, and leave semantic and
physical consumers closed.

### Candidate B — wrap `VerifiedRawRootExpansionV1`

Rejected as canonical source authority. It keeps the old AST/name scan as the
source truth and creates a second route beside the parser seal. It can remain
an explicitly noncanonical compatibility owner during migration, but it cannot
issue the new source disposition.

### Candidate C — let Builder infer App from the parser seal

Rejected. It moves program entry meaning into a physical/root lifecycle owner,
requires a Builder-side source lookup or AST rescan, and makes
`root_is_app_mode` an accidental semantic authority.

## Fail-fast and forbidden edges

The intended boundary is:

```text
parser static parent seal
  -> exact program entry inventory
  -> one Main/App entry issuer
  -> typed disposition transport
  -> stop before Builder/root/runner effects
```

Forbidden edges for this D0:

```text
entry admission failure -> Script fallback
entry admission failure -> VerifiedRawRootExpansionV1 retry
Main name only -> App
main method ordinal only -> entry
static parent Ready -> root_is_app_mode write
entry disposition -> Main child lowering
entry disposition -> Recipe/Join/MIR
runner select_entry_function -> parser source authority
```

## Ordered task queue

### D0.1 — Exact relation inventory

Determine whether the current parser static parent seal exposes enough exact
header/member/callable relation to prove `Main.main/0` without a second AST
scan. If not, record the missing parser-owned relation as `NoSafeSlice`; do not
fill it from names, ordinals, or Builder products.

### D0.2 — Program-wide uniqueness

Freeze the rules for zero, one, and multiple static `Main` boxes, and for zero,
one, and multiple direct `main` rows. Duplicate and foreign parser brands must
be `IntegrityInvalid`, not a first-row selection.

### D0.3 — Semantic/physical handoff boundary

Record that `AppMainReady` is transport-only. It does not replace normal Main
completion/result/ABI plans and does not write `root_is_app_mode`.

### D0.4 — Old authority census

List every production caller of `VerifiedRawRootExpansionV1`, every
`root_is_app_mode` write, Main-child lowering consumer, and runner entry
selector. Classify each as canonical candidate, compatibility owner, test-only,
or retirement candidate. No caller is retired by this D0.

### D0.5 — Decision gate

Accept the entry design only if Candidate A can prove the exact relation with
one issuer and no AST/name/ordinal repair. Otherwise remain `NoSafeSlice` and
design the missing parser relation first.

## Read-only current census

The current main branch confirms that the entry authority is not yet unified:

```text
VerifiedRawRootExpansionV1::from_program production references
  = 10 non-test references across raw projection, callable catalog,
    normal root lifecycle, entry materialization, and declarations

root_is_app_mode writes
  = builder root lifecycle only; initialized in builder_init and written by
    program_root_lowering after expansion preflight

Main-child semantic consumer
  = normal_callable_semantic_package::with_main_static_child_lowering_input
    plus its selected_mapping role check

runner entry selectors
  = 2 runtime consumers of select_entry_function; neither is source authority

parser static-parent source production consumer outside parser/tests
  = 0

new parser Main/App issuer
  = 0
```

This census is evidence for the separation, not permission to connect the
paths. In particular, replacing the ten raw-root references is a later
retirement series, not part of this D0.

## NoSafeSlice conditions

Do not implement Main/App admission if any of these is true:

```text
the static parent seal cannot prove Main/main relation without AST re-scan
program-wide uniqueness requires name/ordinal/pointer pairing
the proposed issuer must infer result/ABI/body semantics
VerifiedRawRootExpansionV1 and the new issuer would both be canonical
Main/App admission requires Builder effects before the typed disposition
mixed/ordinary compatibility policy must change in this slice
entry failure needs Script/legacy fallback or re-selection
Main child selection or runner entry selection must be moved at the same time
the exact relation cannot stay below the 760-line split trigger
```

## D0.1 relation decision

The read-only source audit closes the exact-relation question for the bounded
I0. The existing parser products already carry the required facts without an
AST re-scan:

```text
ParserStaticBoxSourceSealV1
  -> exact Box declaration syntax/name
  -> same-invocation Box path/brand
  -> total member coverage
  -> one exact direct method site for the bounded cohort

ParserCallableParameterSourceCatalogV1
  -> same-invocation method source site
  -> static declaration kind
  -> diagnostic method syntax name
  -> exact parameter rows and arity
```

The only source-side addition allowed by I0 is a parser-private accessor for
the already co-sealed direct method site. It is not a new source scan, identity
key, or second issuer. `ParserMainAppEntryAuthorityIssuerV1::issue_once` can
then compare the `Main`/`main`/zero-parameter admission rules against those
same-brand/path facts at the completed callable-source product boundary.

Therefore Candidate A is accepted for a parser-only I0. The old
`VerifiedRawRootExpansionV1` remains a noncanonical compatibility owner until
a later retirement card; it is not wrapped or invoked by the new issuer.

## Acceptance packet for this design stop

```text
Decision brief = 1
source authority and sole proposed issuer named = 1
candidate comparison = 3
program-wide uniqueness cases = 0/1/multiple specified
old raw-root callers inventoried = 10 non-test references recorded
Builder/runner effects before admission = 0
AST/name/ordinal repair = 0
fallback/retry/reselection = 0
production caller = 0
```

## Current Decision

Candidate A is accepted only for the bounded parser-only I0 described below.
No semantic, Builder, runner, or production entry switch is accepted by this
D0. The next card owns implementation and evidence.
