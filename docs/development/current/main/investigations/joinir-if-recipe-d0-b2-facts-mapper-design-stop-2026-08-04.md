# JOINIR-IF-RECIPE-D0-B2-FACTS-MAPPER-DESIGN-STOP

Status: D0-B2-A/B/C landed; D0-B2-D invariant-defense design stop active
Date: 2026-08-04
Decision target: same-pass facts -> fixed-shell `IfRecipeArtifactV1`

This card follows the green D0-B1 schema/verifier row. It is the only design
boundary before a facts mapper, JoinSig, or production If consumer may be
implemented.

## Worker-verified facts

`AnalyzerV1::analyze` walks one `LocatedBodyV1` and seals
`VerifiedTrivialIfRecipeFactsV1` only after:

```text
claim_if_control -> condition -> then/else -> continuation
-> if-control coverage -> fact coverage -> owner transport -> facts seal
```

Assignments, reads, literals, and binaries retain the same-pass source sites.
The facts product is owner-branded and non-`Clone`; its assignment fields are
private. It does not contain an authoritative pre-If entry value, however.

The existing `IfJoinRowV1.entry_value` cannot be filled by copying the
condition's `ReadBinding`, choosing an arbitrary expression, or inventing a
recipe value. Those are silent provenance guesses. The selected golden shape
needs a logical pre-If entry witness for the branch merge binding.

## Selected boundary (D0-B2-A decision)

Preserve the landed B1 row and close the gap with one non-`Clone` logical entry
witness capability. Its minimum contract is:

```text
IfFactDraftV1:
  entry_binding: BindingRefV1
  entry_representation: TrivialRepresentationV1
  declaration/source-order proof for the entry binding
```

Decision: emit this witness during the existing same-pass analyzer from the
pre-branch `ValueEnvironmentV1`. This is one private facts-field extension, not
a portable schema change and not a second source traversal. The witness carries
no AST site or physical ID. The mapper later validates its declaration/order
against the owner product's definition ledger, creates one recipe-local `Input`
value, and uses it as `IfJoinRowV1.entry_value`.

This preserves the semantic distinction:

```text
same-pass facts owner  -> logical entry witness
mapper                 -> recipe-local keys/operations
IfRecipeVerifier       -> portable structural checks
IfJoinSig (later)      -> predecessor/value-edge proof
Canonical SSA/PHI      -> existing physical owner, later cutover
```

Rejected alternatives:

- infer entry from a condition read or branch expression: unsound provenance;
- change `IfJoinRowV1` to carry a `BindingRefV1`: portable owner/identity leak;
- silently synthesize an entry value: forbidden. If the capability cannot
  prove the pre-If value, the mapper returns `EntryValueWitnessMissing` as a
  pre-effect typed rejection; this is a contract failure when facts were
  already sealed as admitted.

## Mapper boundary

The mapper must live beside the facts owner (`resolved_value_profile`) or use
a typed one-shot accessor. It may consume only:

```text
VerifiedTrivialCanonicalOwnerV1
VerifiedTrivialIfRecipeFactsV1
VerifiedResolvedFunctionV1::function_origin()
```

It must not accept raw `BindingRefV1`/assignment maps as a portable API, infer
function origin from `FunctionOwnerIdV1`, or re-open `input.source()` after
facts are sealed. `IfRecipeArtifactV1` remains source-bound by explicit ordinal
owner/path claims and semantic normalization remains owner-brand independent.

Local key construction is deterministic and dependency-first:

```text
entry input -> condition closure -> then value closure -> else value closure
-> continuation read
```

Only the required expression roots and their binary operands are emitted. The
mapper preserves actual then/else assignment path indices; it must not assume
index zero. Unsupported cross-region, nested-control, short-circuit, Call,
Record, Match, BlockExpr, foreign-site, missing-expression, or representation
cases return a typed map rejection. The result is `Result`, never `Option` or
retry.

## Ordered task slice

1. `D0-B2-A` — add and seal the private same-pass entry witness from the
   pre-branch environment, with typed accessors and `EntryValueWitnessMissing`.
   Landed in `a907874551`.
2. `D0-B2-B` — implement the facts-to-recipe mapper in the facts owner. Convert
   source sites to the fixed source-claim grammar without AST rescanning.
   Landed in `1bd50829c5`; the mapper immediately invokes the structural
   verifier and has no production caller.
3. `D0-B2-C` — call the existing structural verifier and add deterministic
   semantic/source-bound normalization tests plus the reachable negative
   matrix. Landed in `f2afec934d`, `0c2ee5e9dd`, and `1fd0e5ab70`; both branch
   source-claim paths, arbitrary accepted item indices, and reachable mapper
   and preflight rejection rows are asserted. Do not add JoinSig or PHI.
4. `D0-B2-D` — classify remaining defensive mapper variants as sealed-facts
   invariants versus reachable input rejection. Do not synthesize malformed
   non-`Clone` facts solely to hit defensive arms. Then open D0-B3 for
   non-`Clone` `IfJoinSig` and typed physical-input sealing.

The classification is now explicit:

```text
reachable ordinary input:
  MissingFacts, OwnerMismatch, EntryClassMismatch

sealed-facts / mapper firewall:
  MissingEntryWitness, EntryBindingMismatch, MissingAssignment,
  EntryDefinition*, UnsupportedRepresentation, MissingExpression,
  CrossRegionDependency, ExpressionCycle, Binding*Mismatch,
  SourcePathMismatch, ContinuationMismatch, Recipe(...)
```

Nested/control/effect shapes either make the facts product absent or stop in
the analyzer's earlier typed boundary. They are not silently promoted to a
mapper-specific failure. A future producer may promote a defensive variant
only with a new same-pass fixture and an SSOT update.

## Acceptance gates

- one analyzer traversal and zero source traversal after facts sealing;
- entry witness is explicit, logical, and sourced from the pre-branch
  environment;
- no `BindingRefV1`, `FunctionOwnerIdV1`, AST, `MirBuilder`, `ValueId`, or
  `BasicBlockId` appears in the portable artifact;
- same source origin with different owner brands has identical semantic
  normalization;
- actual source claim role/order/path indices are preserved (including a
  non-zero branch item) and foreign or duplicate claims reject;
- golden explicit-else maps and re-verifies deterministically;
- reachable negatives cover implicit-else, branch-cardinality mismatch,
  missing continuation, unsupported representation, foreign owner, and entry
  class drift; nested/control/effect shapes are recorded as preflight
  `MissingFacts` or earlier typed analyzer stops;
- defensive-only variants (missing entry witness, cross-region operands,
  expression cycle, missing expression, source-path mismatch, and verifier
  rejection) are not claimed as ordinary-input coverage;
- no production Recipe caller, JoinSig, PHI writer, route retry, or Builder
  mutation is introduced;
- all touched Rust/test files remain below 800 lines.

## Non-claims

This row does not make `IfRecipeArtifactV1` a production authority and does
not claim repository-wide PHI/SSA adoption. The canonical resolved owner is
already named (`CanonicalSsaFunctionSessionV2` + `CanonicalCfgSessionV1` +
`PhiTxn`), but old If/JoinIR writers remain until a later caller-zero row.
