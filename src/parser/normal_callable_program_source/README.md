# Normal callable Program source

This parser boundary retains the complete callable anchor set after the total
parser postpass and verifies one whole-file transform before normal MIR
compilation.

Authority:

- the parser chooses `SourceBacked` or a typed compatibility cohort;
- a source-backed transform consumes the initial product exactly once;
- final callable slots and declarations must remain an exact set; and
- the verified final product owns the transformed Program and retained opaque
  anchors atomically.

The final product lends one complete callable syntax batch from those anchors
and slots. Direct-method parameter rows are joined as an exact optional
projection on each batch slot; they retain borrowed declared-type spelling but
never define total callable cardinality or classify ABI/Home semantics.
Top-level, selected-gate, and generated callables remain batch members without
receiving a fabricated `Ordinary` parameter source.

## Parser normal-program source authority I0

`ParserNormalProgramSourceAuthorityV1` is the parser-owned, non-`Clone`
source owner for the source-backed normal Program handoff. Its sole issuer is
the parser product constructor, where the completed postpass, callable source
disposition, parser invocation witness, and the bounded composite disposition
are co-sealed once. The authority stores AST-free top-level body rows and the
nested parser composite token; it never stores AST references, spans, pointers,
names as keys, MIR sites, target selection, candidate/C disposition, Recipe, or
Builder state.

The required move chain is:

```text
ParsedProgramWithCallableParameterSourceV1
  -> PreparedNormalCallableProgramSourceV1
  -> VerifiedFinalCallableProgramSourceV1
  -> VerifiedResolvedCallableSemanticBatchV1
  -> VerifiedNormalCallableSemanticPackageV1
```

The source owner exposes one HRTB callback loan. Each loan item pairs one
parser-issued body row with its AST statement in one cursor; callers cannot
return the borrowed AST or split parallel arrays for ordinal re-pairing. The
semantic package only delegates this scoped loan. The neutral Script window
issuer will consume this loan once and will become the only production issuer
of `VerifiedScriptRootDemandWindowV1`; the existing Builder window remains
transport-only until that cutover.

Transform validation preserves the parser token by move. It checks the initial
body coverage/kind and the bounded composite role tree, returning typed
`ProgramSource`/`Composite` rejection. Existing callable declaration checks
remain responsible for callable-set/body changes, and the established
macro-generated test-tail compatibility remains explicit. A `Ready` authority
cannot be discarded into the compatibility AST lane.

Selected-normal materialization may attach one `NormalParserSourceLineageV1`
projection: source identity, digest, grammar profile, UTF-8 length, and the
one-read/one-parse receipt. It is transported from the parser handoff rather
than reconstructed from the transformed AST. The canonical normal-file front
door retains the same non-Clone handoff through source-plan classification.

Non-authority:

- names, arity, spans, ordinals, and AST addresses never recreate identity;
- a failed exact transform never retries through compatibility;
- this module does not resolve callables or own Builder, Recipe, Home, ABI, or
  physical lowering; and
- macro transform policy remains owned by `src/macro/`.

The first cohort accepts exact callable-preserving transforms. Added, removed,
reordered, or changed callable declarations reject. Broader transform receipts
must be added as explicit source transactions rather than AST reconstruction.

## Normal source-root disposition transport I0

The parser-only `ParserNormalRootSourceDispositionV1` is issued exactly once
by the private parser facade inside
`ParsedProgramWithCallableParameterSourceV1::new`. It co-seals the existing
opaque App seal or positive pure-Script cohort witness. It does not own or
clone `CanonicalScriptSourceRowsV1`; those rows remain the separate A-handoff
product.

The unified root product is transported as one required, non-`Clone` field
through the existing source-backed owners:

```text
ParsedProgramWithCallableParameterSourceV1
  -> PreparedNormalCallableProgramSourceV1
  -> VerifiedFinalCallableProgramSourceV1
```

The retained parser-source lane carries the same unified disposition. A
reference Script-A frontdoor explicitly moves it to `DiscardedBeforeA` before
moving the independent Script rows; `AppReady` on that route is a typed reject.
`Compatibility` remains a separate AST-only lane and never receives a
synthetic Main/App or Script row. The transform moves the existing root
disposition without re-running parser observation or classifying the root
again. No root selection, `NormalCompileRequestV1`, Builder, Recipe, Join,
MIR, publication, or fallback consumer is connected by this transport cell.

## Final root preservation A-I0

`ParserNormalRootPreservationIssuerV1::seal_after_transform` is the sole
parser issuer for the final root-preservation token. It consumes the already
issued `ParserNormalRootSourceDispositionV1`, the same
`ParserNormalProgramSourceAuthorityV1`, and the parser-owned transform session;
it does not reclassify App/Script from the transformed AST.

`ParserNormalRootPreservationV1::Ready` is a non-`Clone` opaque token carrying
one closed `App | Script` relation and parser invocation witness. Before it is
issued, the parser checks that the parser-owned body-row count, initial Program
statement count, and final Program statement count are identical, then compares
every statement at its exact position. Addition, removal, reorder, or
replacement is a typed rejection. `Outside`, unavailable, incomplete, invalid,
terminal, and `DiscardedBeforeA` states remain typed terminal transport rather
than a guessed role.

For `App`, the same final-source issuer also moves the existing private
`ParserMainAppEntrySealV1` into one private relation with the exactly one
opaque-identity-matching callable row and that row's already-paired final slot.
The row must belong to the same parser invocation, remain a direct static-Box
method at the App seal's private source relation, and retain its exact BoxMethod
placement. The App admission's sole-constructor contract supplies
`CallableMainIsRoot` and `NoStaticChildren`; the final issuer does not rescan
`Main`/`main`, recreate identity from ordinals, or call the raw Builder root
classifier. The relation exposes no parser site, anchor, name, or ordinal.

The production final source can only be issued through
`ParserNormalCallableTransformSessionV1::finish_exact`; this entry accepts no
raw AST callback and moves the unchanged parser Program into the final product
without a deep clone. A `#[cfg(test)]` transform hook exists only for the
parser preservation rejection matrix. The old free raw-AST production entry
is intentionally absent. The token moves through
`VerifiedFinalCallableProgramSourceV1` and the existing prepared root wrapper.
Root consumption, lowering, raw classifier retirement, Builder effects,
fallback, and production switching remain a later bounded slice.

The macro/test-harness owner classifies its own work before this exact finish:

```text
Unchanged
  -> finish_exact

GeneratedTail(AST)
  -> Compatibility(TestHarnessGeneratedTail)
  -> no parser final source or root token
```

If composite preservation is already `Ready`, `GeneratedTail` is a typed
`CompatibilityLoss` rejection instead of an authority drop. A macro-engine
mutation not owned by a named transform is `UnclassifiedSourceMutation`; it
does not enter either the exact or compatibility lane. The parser never reads
the test-harness environment or infers generated-tail provenance from AST
shape.

## Root consumer loan S0

The parser-owned HRTB root-consumer loan is available at the final source
owner. `ParserNormalRootConsumerLoanV1` has exactly two ready views:

```text
App
  -> root body/result/uses/attrs
  -> one Program-order cursor
       RootMain
       Sibling { syntax kind, borrowed statement }

Script
  -> one paired { syntax kind, borrowed statement } cursor
```

The App cursor replaces the already-admitted Main statement at its exact
Program position; it never exposes the raw Main declaration, parser locator,
name, or ordinal. The Script cursor never separates source rows from AST
statements. Both views are issued from the existing non-Clone
`ParserNormalRootPreservationV1` relation and complete Program authority under
one parser witness. Their higher-ranked callback cannot return borrowed
syntax. Outside, Script terminal, unavailable, incomplete, invalid, and
discarded states reject with typed causes before the callback.

`NORMAL-ROOT-CONSUMER-LOAN-S0` deliberately has no production caller. Its only
permitted successor is the move-consuming lifecycle facade in
`NORMAL-ROOT-CONSUMER-I0`; Builder effects, compatibility behavior, remaining
raw observers, Recipe, and physical lowering remain closed in S0.

## Composite preservation transport

The parser's bounded composite disposition is a required field of
`PreparedNormalCallableProgramSourceV1` and
`VerifiedFinalCallableProgramSourceV1`. The move chain is:

```text
ParsedProgramWithCallableParameterSourceV1
  -> PreparedNormalCallableProgramSourceV1
  -> exact transform guard
  -> VerifiedFinalCallableProgramSourceV1
  -> PreparedNormalDefaultProgramRootV1
  -> NormalCompileRequestV1
```

`ParserCompositeSourcePreservationV1` is non-`Clone`, AST-free, and parser
private. The transform guard compares the parser-issued role tree against the
unchanged/transformed root AST and rejects provider, result, call, receiver,
argument, terminal, or compatibility drift with a typed
`FinalCallableProgramSourceRejectV1::Composite` error. A `Ready` token never
enters the compatibility AST lane and is not exposed through a parallel
request field. This cell transports source preservation only; resolver,
target/result lookup, A/C, Recipe, and physical lowering remain later cells.
