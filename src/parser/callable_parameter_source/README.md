# Callable parameter source

This directory is the parser-side physical owner for callable parameter source
syntax. It exists so the transfer-source catalog can grow without adding a
second authority to the near-limit `source_seal.rs` or mixing parameter
semantics into the general resolver handoff.

Current I0 boundary:

- `parse_product.rs` co-issues the neutral `ParamDecl` projection and one-shot
  parameter source rows from the same parse;
- `session.rs` binds those rows to the parser invocation and exact direct
  Box-method source site, rejecting foreign/duplicate/partial coverage;
- `catalog.rs` owns the complete non-Clone static/instance sibling catalog;
- `product.rs` pairs that catalog with the completed total parser postpass and
  moves the pair atomically into the retained source owner;
- `retained.rs` keeps the completed postpass and complete catalog non-Clone
  and non-splittable while lending repeatable callback-scoped declaration
  syntax to the future sole resolved semantic batch;
- `syntax_loan.rs` consumes that pair into one callback-scoped exact
  declaration loan using only sealed statement/inventory placement; the loan
  cannot escape and never searches by name;
- `model.rs` owns the closed `Ordinary | Take` vocabulary, explicit
  `Absent | Explicit` type syntax, declaration kind, and exact ordinals;
- only `Ordinary` has an issuer. `Take` remains vocabulary-only.

The catalog is deliberately not part of `ParserBoxSourceSealV1`: static Boxes
remain on the total postpass compatibility arm while sharing the same parser
brand/Box path/source-member cursor. Inventory ordinal, method name, Box name,
arity, raw strings, and the cloneable AST are not source identity. Selected
build gates, interfaces, constructors, generated methods, top-level functions,
and Lambdas remain outside this I0.

Each declaration retains two deliberately different coordinates:

```text
source member ordinal
  = parser source identity; fields and other members participate

method inventory ordinal
  = descriptive placement receipt; method inventory lookup only
```

Session duplicate detection compares the complete parser-owned
`SourceBoxMethodSiteV1`, including its selected-gate branch path.  The lossy
pair `(root statement ordinal, source member ordinal)` is diagnostic-only and
must not collapse distinct branch declarations before postpass selection.
This does not admit selected gates into the legacy parameter catalog: its
finalizer still rejects that cohort until the complete callable source owner
lands.

The placement receipt comes from the same explicit-method commit. It must
never replace the source site as identity or authorize name-based repair. Its
only downstream purpose is to let a parser-owned loan locate the already
committed declaration without reconstructing source order.

The retained source keeps the completed parser postpass and parameter catalog
together. Its scoped loan validates Box kind, explicit/direct provenance,
method identity, parameter order/name/type coverage, and static mode before
lending syntax. The same retained owner may lend the exact rows again, but no
AST reference can escape and there is no AST/catalog `into_parts` surface.

The older consuming callback remains only for the already-landed standalone
parameter-demand path. It retires when the demand becomes a borrowed projection
of `VerifiedResolvedCallableSemanticBatchV1`; new semantic owners must use the
retained source instead of consuming and independently resolving the catalog.

This module issues no Home demand, receiver/result ABI, resolver BindingRef,
Recipe key, MIR value, fallback, or production activation. The next owner
retains this whole source, resolves its declarations exactly once, and owns the
only forest/projection batch used by normal-source and parameter-demand views.
