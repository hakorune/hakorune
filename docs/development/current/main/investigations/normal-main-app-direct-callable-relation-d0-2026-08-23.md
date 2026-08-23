---
Status: Implementation complete — bounded relation I0 closed
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-DIRECT-CALLABLE-RELATION-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-entry-admission-i0-2026-08-23.md
ProductionCaller: 0
ProductionEdit: parser-only relation I0 complete; Main/App remains closed
CeremonyTier: T2 — parser declaration relation boundary
---

# NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-DIRECT-CALLABLE-RELATION-D0

## Current capsule

- **Current decision:** a direct static declaration must carry one opaque
  parser-issued callable relation into the callable anchor row, parameter
  catalog row, and static-parent member row before Main/App admission can run.
- **Current implementation status:** the parser-only relation I0 is complete;
  Main/App Candidate A remains closed until this evidence is committed and the
  Main/App I0 is explicitly reopened.
- **Next ordered task:** close out this relation evidence, then reopen the
  already-designed Main/App I0 without adding a downstream consumer.
- **Production stop line:** no Main/App disposition, NormalCompileRequest,
  Builder root state, Main child lowering, Recipe/Join, MIR, fallback, or
  raw-root retirement.
- **Retirement finish line:** one parser-issued relation is created at the
  direct declaration commit, every consumer compares that relation, and no
  later product reconstructs it from path/name/ordinal.

## Decision accepted

The read-only worker audit found this to be a conditional safe slice. The
condition is now fixed as part of the contract: `CallableDeclarationAnchorV1`
remains owned and issued only by the existing direct callable source session.
The parameter and static-parent products receive only its cloneable
comparison view, `CallableDeclarationIdentityV1`; they never receive, clone,
or issue the opaque anchor itself.

The relation I0 is authorized with this bounded handoff:

```text
one direct static declaration commit
  -> existing callable row keeps the opaque anchor
  -> parameter row stores comparison identity
  -> static direct-member row stores comparison identity
  -> parser postpass co-seals same_as + same brand/site coverage
```

The I0 must add a distinct relation-mismatch/integrity outcome for same-brand
but different-anchor rows. It must not widen that case into a foreign-parser
error. No Main/App admission or downstream consumer is part of this slice.

## Six-line brief

```text
Decision:
  co-seal one direct declaration relation before Main/App admission; do not
  pair static parent and parameter catalog later from source coordinates alone.
Source authority + canonical issuer:
  the existing direct declaration commit/ParserCallableSourceSessionV1 anchor
  issuance; one relation handoff supplies its opaque comparison identity to
  the callable row, parameter row, and static-parent transaction.
Non-authority:
  SourceBoxMethodSiteV1 alone, Box path/member ordinal, diagnostic name,
  parameter arity, AST pointer, inventory order, digest, Builder, raw root,
  Main child, runner, Recipe/Join, MIR, and fallback.
Fail-fast boundary:
  the direct declaration commit, before the parser postpass products are
  finalized; foreign/duplicate/missing relation terminates the parse/source
  transaction without a downstream effect.
Smallest next slice:
  one direct static Box method relation carried to the three parser products;
  no Main/App disposition, no consumer, no ordinary/mixed policy change.
Non-claims:
  entry selection, Main semantics, result/ABI, resolver, Builder, physical
  lowering, publication, old-route retirement, production switch, performance.
```

## The current gap

Today the same declaration is observed by three parser-side products:

```text
ParserCallableSourceSessionV1
  -> PreparedDirectCallableSourceV1
  -> opaque CallableDeclarationAnchorV1

ParserCallableParameterSourceSessionV1
  -> ParserCallableParameterDeclarationSourceV1
  -> exact SourceBoxMethodSiteV1, name, kind, parameters

OpenParserStaticBoxSourceTransactionV1
  -> PreparedParserStaticBoxMemberSourceRowV1::DirectMethod
  -> exact SourceBoxMethodSiteV1
```

The callable anchor is currently owned only by the direct callable row. The
parameter catalog and static parent seal retain coordinate evidence, but not
the same opaque identity. A later Main/App issuer could compare brand/path/site
and appear exact, yet it would still be re-pairing independent products after
their declaration commits.

That is the blocker. It is narrow and local; it is not a reason to merge the
three owners or to move entry semantics into Builder.

## Recommended relation shape

Reuse the existing parser-issued anchor identity as a comparison-only relation:

```text
CallableDeclarationAnchorV1
  sole owner: PreparedDirectCallableSourceV1
  comparison view: CallableDeclarationIdentityV1
  same_as only; cannot issue or reconstruct an anchor
```

At the one direct declaration commit, the same identity view is passed to:

```text
PreparedDirectCallableSourceV1
ParserCallableParameterDeclarationSourceV1
PreparedParserStaticBoxMemberSourceRowV1::DirectMethod
```

The view may be cloneable because it is comparison-only; the opaque anchor
itself remains non-Clone and has one issuer. The relation is not a Recipe key,
entry key, method name, ordinal, or physical ID.

The static parent issuer must then verify both:

```text
same CallableDeclarationIdentityV1 as the exact direct callable row
same CallableDeclarationIdentityV1 as the parameter catalog declaration
```

The existing brand/path/member checks remain coverage and integrity evidence;
they do not become the primary join key.

## Handoff point

The natural handoff is the existing explicit static method commit:

```text
commit_pending_static_method
  -> issue_committed_static_box_method
  -> receive the parser-issued comparison identity
  -> pass the same identity to parameter-source commit
  -> pass the same identity to static-source commit
```

No AST rescan is needed. No new parser invocation is opened. No second anchor
issuer is allowed. If changing the helper return shape would expose the raw
anchor or create a second owner, use a private relation carrier instead and
keep the public products opaque.

## Candidate comparison

### Candidate A — shared comparison identity from the existing anchor

Recommended. It preserves the existing anchor owner, adds only a comparison
view to the two source products, and lets the later Main/App issuer co-seal
already-related facts without coordinate re-pairing.

### Candidate B — compare exact source sites later

Rejected for this boundary. Source sites are valuable coverage evidence but do
not carry the opaque declaration relation required by the current SSOT. Using
them as the new primary pairing key would weaken the source authority.

### Candidate C — create a fresh anchor in the parameter or static session

Rejected. It creates a second issuer and allows two independently issued
anchors to describe one declaration. The direct declaration commit must remain
the sole issuer.

### Candidate D — let Main/App Builder code pair the rows

Rejected. It reopens AST/name/ordinal or Builder-state inference and places
source authority after the parser handoff.

## Finite relation states

```text
Unissued
  -> one direct declaration issuer
Issued
  -> identity view delivered to all required parser products
CoSealed
  -> parent/direct/parameter relation is same-anchor and same-brand/path
RejectedForeign
  -> foreign parser brand or anchor relation
RejectedDuplicate
  -> duplicate direct row or duplicate relation
RejectedIncomplete
  -> required consumer did not receive the relation
```

Only `CoSealed` may be consumed by the later Main/App entry issuer. These are
parser development states, not runtime Candidate/Declined states.

## Acceptance evidence

The design packet is complete when it proves:

```text
CallableDeclarationAnchorV1 issuer = existing direct declaration issuer only
CallableDeclarationIdentityV1 relation handoff = one declaration commit
parameter declaration stores relation = 1
static direct-member row stores relation = 1
PreparedDirectCallableSourceV1 stores anchor owner = 1
foreign relation reject = typed
duplicate relation reject = typed
missing relation reject = typed
path/name/ordinal-only pairing = 0
Main/App issuer = 0 until this card closes
downstream consumer = 0
source files >= 800 = 0
```

## NoSafeSlice conditions

Return to design stop without implementation if:

```text
the existing anchor cannot be shared as a comparison-only identity
the relation requires a second anchor issuer
parameter/static products can only be paired after postpass
the handoff requires AST reconstruction or name/ordinal lookup
the relation must own semantic entry/result/ABI meaning
ordinary/mixed policy or compatibility fallback must change
Main/App consumer must be connected in the same slice
the source relation exceeds the 760-line split trigger
```

## Next task after acceptance

Only after this D0 is accepted may the Main/App I0 reopen. Its issuer will
consume the co-sealed relation and issue the typed `AppMainReady` disposition;
it will still not set `root_is_app_mode` or invoke any Builder/runner consumer.

## Relation I0 implementation evidence

The bounded parser-only implementation is complete in the working tree:

```text
direct callable source session
  -> one existing CallableDeclarationAnchorV1
  -> CallableDeclarationIdentityV1 comparison view
  -> parameter declaration row
  -> static-parent direct-member row
  -> static-parent identity/coverage co-seal
```

The callable anchor remains non-Clone and has one production issuer. The
parameter catalog and static-parent seal store only the comparison identity.
The static-parent issuer checks identity first and retains brand/path/member
coordinates as coverage evidence. Same-brand but different-anchor rows return
`MethodRelationMismatch`; missing and duplicate rows remain distinct typed
outcomes. The composite source lookup also requires the same comparison
identity before using coordinate coverage.

Observed evidence:

```text
focused relation test                                      = 1 passed
static-parent source suite                                 = 6 passed
cargo check                                                = passed
frontend_static_box_parent_source_i0_guard.sh              = passed
current_state_pointer_guard.sh                             = passed
git diff --check                                           = passed
source-size checks                                         = passed
```

The broader parameter-source suite is `7 passed / 1 failed`. The single red
test is the pre-existing
`unchanged_parser_scan_loop_box_has_four_methods_and_fifteen_rows` expectation
of `None` for `pos`, while the unchanged fixture declares `pos: i64` in both
`HEAD` and the working tree. The relation change only adds identity transport
and does not alter parameter syntax. This is recorded as known baseline debt,
not a relation-I0 failure; the stale expectation is outside this slice and is
not changed here.

The repository-wide `cargo fmt --all -- --check` also reports pre-existing
formatting differences in unrelated unchanged files. It is not used as the
relation-I0 acceptance gate; no formatting sweep is included in this slice.
