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
non-callable-tail compatibility remains explicit. A `Ready` authority cannot
be discarded into the compatibility AST lane.

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
