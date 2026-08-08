# Callable Contracts

Status: accepted language target; typed parser carriage I0 landed, resolver/body-conformance production 0; R6-S3B-C-S1 private parser target-index and C-I0 parser-private batch receipts closed; R6-S3B-D-D0/D-I0 bounded final-seal implementation closed; broad public AST postpass cutover D0 accepted; resolver coverage remains unopened.

Decision: `LANGUAGE-TYPED-CALLABLE-PROFILE-D0` (2026-08-08).

## Canonical source

```hako
box TextLike {
    @rune CallableContract(query)
    length(): i64 {
        // implementation
    }
}
```

`CallableContract` is declaration-local and non-repeatable. The first accepted
value is `query`, and the first implementation cohort is an instance method.
The Rust parser accepts and carries this syntax through its rich,
parser-owned, non-Clone `ParserBoxSourceSealV1`; semantic issuance remains
production zero. The seal is currently bounded to ordinary top-level Rust
Boxes. The Hako parser keeps the same accepted name/value syntax in its
compatibility rune normalizer, but does not issue the Rust source seal. The
Clone-capable `BoxMethodInventoryV1` owns selected placement and descriptive
provenance, not resolver-grade source identity. Exact as-written method sites
are separate from all-row inventory ordinals because generated
property/delegate rows also consume inventory positions. Generated rows
receive only a generated source-member origin and never an explicit method
source site. Legacy JSON and AST-only compatibility rows cannot reconstruct or
promote the seal. The remaining boundary is an explicit consuming
parser→resolver handoff; resolver declaration/Home issuance must still land
before the declared contract issuer opens.

The R4 AST-side atomic reconstruction product and strict recursive JSON v2
codec are landed. The root selects v2 or legacy mode once; malformed nested v2
payloads reject the complete root, and legacy v1 imports remain
`CompatibilityOnly(LegacyJsonV1)`. This descriptive transport still does not
change the production-zero status or authorize a declared callable contract.

The current frontend migration also keeps the ordered
`BoxMethodInventoryV1` carrier intact through the connected static-`Main`
Builder compatibility ports. That R5-S2 edge retains only the historical
name-order projection inside the compatibility leaf; it does not establish
source-order authority, resolver target authority, or a callable-contract
issuer.

The frontend ordered-inventory migration is now closed through R5-S3: the
production Builder no longer round-trips the inventory through compatibility
maps. This does not open the Hako parser carrier, resolver declaration issuer,
or callable-contract semantic issuance; those remain separate design rows.

## `query` meaning

`query` is a stable whole-call behavioral contract:

```text
receiver:
  exact enclosing nominal Box
  exact receiver direct-state reads allowed

required ownership compatibility:
  same-declaration VerifiedHomeAbi receiver demand = Handle

forbidden:
  receiver/global writes
  ambient/global/unrelated heap reads
  Home consume/create/share/end/escape
  allocation
  IO / FFI
  Fault / throw / non-local failure propagation
  suspension
  non-local control transfer

allowed:
  ordinary return
```

The first contract does not authorize transitive reads through arbitrary
objects or calls. A callee/read-footprint composition rule requires a later
Decision; until then it fails before the declared query contract is issued or
before body conformance is sealed.

The rune does not name parameter/result types, arity, MIR representation, or a
backend ABI. Those authorities remain separate:

```text
method signature:
  parameter/result semantic types and arity

callable contract:
  behavioral obligation only

VerifiedHomeAbi:
  receiver/parameter demands and result Home relation

physical ABI verifier:
  semantic value -> target representation
```

Therefore `CallableContract(exact_trivial_i64)` is rejected as a language
design: it repeats `(): i64`, exposes an implementation cohort, and would
create a contract value per type/arity/backend shape.

## Pure versus query

```text
Pure:
  no receiver/heap/global read
  no write, allocation, IO/FFI, Fault/throw, suspension, or non-local control

Query (read-only behavior):
  exact receiver read allowed
  the same write/effect/control prohibitions
```

An ordinary mutable receiver `length()` is a query, not Pure. Only a future
immutable/frozen-receiver proof may strengthen a receiver read to Pure. The
first cohort returns semantic `i64`; this Decision does not define a future
`Result`-returning query contract.

Existing `@rune Contract(pure)` and `@rune Contract(readonly)` remain their
current metadata family; this Decision does not silently promote them into a
whole-call contract. `Profile(...)` also remains a reserved compatibility
bundle and is not a callable semantic issuer.

## Declaration and conformance

The annotation is a declared obligation, not a body proof:

```text
ordered source declaration
  -> VerifiedDeclaredCallableContractV1

method body
  -> semantic body Facts / Recipe
  -> VerifiedCallableContractConformanceV1
```

Declaration contracts may be cataloged before bodies are verified so recursive
and mutually recursive calls can resolve. After body verification, the sealed
declared catalog and one complete same-brand conformance set are atomically
co-sealed into one publishable callable catalog. Missing, duplicate, foreign,
or rejected conformance prevents that product from being issued. Module
publication consumes the publishable catalog and performs no new semantic
decision. The body verifier checks the declared meaning; it never infers or
substitutes a public contract.

## Receiver Home rule

The canonical ownership word is `Handle`:

```text
VerifiedHomeAbi for the bounded query cohort:
  receiver demand = Handle
  parameter demands = []
  result relation = Trivial
```

This does not claim that the receiver object has no Home. It means this call
boundary does not transfer, add, end, or escape a Home token. New
`NoHomeHandle` or `HandleOnly` capability spellings are not introduced.
`query` requires compatibility with this ABI but does not issue, store, or
duplicate the Home axis. `VerifiedHomeAbi` remains the sole call-site
ownership authority and must exist before the declared instance contract can
be sealed.

## Issuer and conflict owner

The parser first normalizes the raw rune spelling into a typed
`CallableContractSyntaxV1::Query` source row with exact method/rune site and
selected-source provenance. Resolver code never reparses the strings
`"CallableContract"` or `"query"`.

The future canonical issuer consumes one exact parser-sealed source-method
capability, nominal Box identity, resolved semantic signature, typed Query
row, and same-declaration `VerifiedHomeAbi`. It returns one declared aggregate
and a sealed declared catalog. The aggregate co-seals these axis owners; it
does not restate Home or infer semantic types from `ExactTrivial*Abi`,
`MirType`, or `FunctionSignature`.

Parser errors own syntax only:

```text
unknown value
invalid placement
duplicate CallableContract rune
duplicate as-written method declaration
```

The semantic issuer owns conflicts with `Profile`, `Ownership`,
`ReturnsOwned`, `CallConv`, or `Contract(pure|readonly)`. This keeps semantic
policy out of duplicate Rust/`.hako` parser branches.

## Disposition

```text
issuer not implemented                          -> NoSafeSlice
exact-family source without CallableContract     -> Declined
contract present but declaration/type unresolved -> Unresolved
contract contradicts source declaration          -> Rejected
exact source-backed declared contract             -> Candidate
Candidate declaration with violating body         -> conformance Rejected
```

`NoSafeSlice` is a development state, not a source disposition. Source
precedence is `Rejected > Unresolved > Declined > Candidate`.

## Activation stop line

This accepted target does not yet authorize:

```text
parser acceptance
resolver declaration or target issuance
body conformance implementation
source-bound call relation
Recipe CallSlot
MIR EffectMask or FunctionSignature projection
Builder/runtime/provider dispatch
fallback or production publication
```

Implementation order is:

```text
ordered Box-method inventory + parser-owned source seal
  -> Rust/.hako source-site and typed Query parity
  -> resolver declaration identity
  -> semantic signature
  -> VerifiedHomeAbi
  -> declared Query behavior and atomic declared catalog
  -> old body-inferred instance-result authority retirement
  -> reusable resolver target
  -> exact source-bound call relation / CallSlot
  -> complete body conformance set
  -> publishable callable catalog
  -> physical ABI projection
  -> production activation
```

Every implementation slice updates this reference and its owning module README
in the same commit.

Current frontend receipt (2026-08-09): R6-S3B-D-I0 extends the bounded
`parse_from_string_with_source_seal` product for direct top-level ordinary Rust
`box` declarations. It consumes the parser-private
`GeneratedDelegateSourceRelationV1` rows after the build-gate/delegate
postpass, verifies exact same-brand relation keys and generated inventory
placement, and retains the complete rows in the sole non-Clone
`ParserBoxSourceSealV1`. Malformed, orphan, duplicate, foreign, or
provenance-invalid coverage rejects the unpublished product. This does not
open typed `CallableContract` parser carriage, resolver publication, Hako
parity, or body conformance. D-I0 is closed for this bounded path. The public
AST-only `NyashParser` paths `parse`,
`parse_from_string_with_fuel_and_build_config`, and
`parse_from_string_with_fuel_and_build_config_and_explain_report` remain
compatibility nonclaims because the rich finalizer is ordinary-Box-only. Their
total postpass envelope is the separate
`PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0/I0` design row; no second seal or
catch-and-fallback is allowed.

The broad AST cutover design uses one total private postpass result rather
than pretending that every parsed program is resolver-source sealed:

```text
CompletedParserPostpassV1
  = AST + ParserMetadata + optional BuildGateExplainReport
    + typed per-Box coverage

coverage row
  = SourceSealedOrdinary(ParserBoxSourceSealV1)
  | AstOnlyCompatibility(typed interface/static/record/mixed cohort)
```

Only `SourceSealedOrdinary` is a source-authority input. Compatibility rows
remain valid AST/metadata/explain projections and never become an empty seal,
resolver target, Recipe input, or fallback result. The postpass coordinator
consumes one parser invocation and one source session; wrappers do not reparse,
rescan AST names, or reconstruct identity from inventory ordinals. Fuel is set
once during parser construction and the same typed envelope owns metadata.

Because explain traversal covers nested BuildGate nodes while the source gate
ledger is top-level scoped, a shared private full BuildGate decision set must
feed prune, explain projection, and top-level ordinary source-path rebase.
Explain cutover remains parked until that decision-set parity is proven.

The shared projection keeps this scope distinction explicit: a structural
BuildGate inside a method/function body is decision-covered but is not a
top-level source-ledger entry, so it emits no resolver-grade source receipt.
Only top-level-scoped observations may produce selection receipts. The
post-closeout BuildCfg regression gate is green; member-level gate semantics
and grammar-evidence remain separate contracts.

S0 implementation receipt (2026-08-09): the parser now has a private total
postpass envelope and one explicit coordinator. Ordinary rows retain the sole
non-Clone source seal; interface/static/record/mixed/no-Box rows are typed
AST-only compatibility coverage. The existing rich ordinary entry reuses the
postpass-opening helper, while public AST/metadata/explain callers remain
unchanged until the ordered I0-A/B/C parity rows.

I0-A implementation receipt (2026-08-09): the selected
`parse_from_string_with_fuel_and_build_config` edge now uses one private
`string_postpass_entry` helper. It performs one tokenize, one parser
construction with fuel/config, one parse, one S0 postpass completion, and one
AST projection. The convenience wrappers inherit that edge; compatibility
cohorts are successful AST transport and the old delegate-only route is not a
fallback. Metadata, `NyashParser::parse`, explain/full-gate parity, resolver,
Recipe, Builder, MIR, and runtime remain parked for later rows. Parent
`72b3471e55` has one separate baseline-red nested member-gate source-path test
that fails before postpass opening because its branches have different public
signatures; it is parked as `PARSER-MEMBER-GATE-NESTED-SOURCE-PATH-D0` and is
not an I0-A regression.

I0-B receipt: `NyashParser::parse` and the metadata wrapper share one
`parse_postpass_s0` finalizer. `ParserMetadata` is moved exactly once through
`CompletedParserPostpassV1::into_ast_and_metadata()`; metadata is not rebuilt
from AST nodes. Explain/full BuildGate decision-set parity remains an I0-C
boundary.

I0-C design receipt (2026-08-09): the next parser slice is accepted as one
private `PreparedBuildGateDecisionSetV1` for postpass-visible AST gates. It
evaluates every structural predicate once, including inactive subtrees, and
feeds prune, top-level source-path survival, and explain projection. Explain
counter output remains the reachable-row v0 projection; inactive rows are
retained for coverage and diagnostics. Member-level parse-time gates and the
separate grammar-evidence demand remain outside this decision set. No
resolver/Recipe/Builder/MIR/runtime or fallback/reparse path opens here.

I0-C-S0 receipt (2026-08-09): the parser-private
`PreparedBuildGateDecisionSetV1` issuer is landed with seven focused I0-C
tests and the 12-case BuildCfg regression gate. This is an internal postpass
authority only; the projection receipt below records the shared consumer
switch.

I0-C projection I0 receipt (2026-08-09): one structural walker and one
postpass product now drive AST pruning, source-path receipts, and explain
capture. The decision set remains parser-private; no second callable or parser
semantic authority is added, and the shared path has no fallback or predicate
re-evaluation.

R6-S3B-A receipt (2026-08-08): the bounded rich parse path now carries one
non-Clone `OpenParserPostpassProductV1` across its existing prune/delegate
boundary. The product owns the AST, a `ParserSourceSessionV1` for prepared
parser source payloads, and
cloneable diagnostic metadata; `parse_from_string_with_source_seal_ast` is a
single AST projection of the finalized rich product for direct ordinary Rust
`box` only. Top-level gate structural paths and source-aware delegate
relations remain closed for S3B-B/C/D.

R6-S3B-B1 receipt (2026-08-08): parser-private top-level build-gate source
paths now carry an invocation brand, parser-issued gate id, branch, and child
ordinal through `SourceBoxDeclarationPathV1`. Box source transactions consume
that path directly, including distinct paths for multiple Boxes in one branch
and nested-gate paths. This is transport only; branch selection receipts,
prune/rebase, delegate relations, resolver contracts, and callable activation
remain closed for later S3B-B2/B3/C/D rows.

R6-S3B-B2 implementation receipt (2026-08-08): this parser slice owns a typed
parser-issued gate source ledger and one private
`BuildGateSelectionReceiptV1` per opened top-level gate. The AST remains free
of gate IDs; an original-AST walk validates the parser ledger by structural
cursor and evaluates each gate once. `ParserSourceSessionV1` transports the
ledger and atomically consumes/returns the selected-branch source seals while
preserving their original paths. Nested and empty top-level gates require
parser-source-preorder order and exact end-of-stream coverage. Gates inside Box
methods/bodies are outside this cohort and must not be registered through the
top-level Box path vocabulary. Missing, foreign, duplicate, or mismatched
evidence rejects the whole unpublished postpass product. Focused parser tests
and `frontend_parsed_box_source_seal_r6_s3b_b2_guard.sh` are the landed
receipt. Delegate relations, final seal expansion, Hako parity, resolver
contracts, Recipe, Builder, and MIR remain closed.

R6-S3B-B3-I0 implementation receipt (2026-08-08): the finalizer now carries a
parser-private `OpenParserPostpassProductV1::final_box_paths` list and issues
one private `FinalizerCoveragePlanV1`. It matches prepared source paths to final ordinary
AST Boxes by exact parser brand/path, rejects count, duplicate, foreign, or
missing coverage, and uses the resulting mapping instead of positional final
AST order. The generated delegate suffix remains descriptive AST compatibility
data outside the resolver-visible source seal until R6-S3B-C issues a
source-aware relation. Focused tests and
`frontend_parsed_box_source_seal_r6_s3b_b3_guard.sh` landed in the same slice;
no delegate transport, resolver, Recipe, Builder, or MIR implementation was
opened.

R6-S3B-C-D0 design receipt (2026-08-08): the next delegate slice is a
parser-private source-aware transaction, not a resolver or callable-contract
implementation. It records exactly one
`GeneratedDelegateSourceRelationV1` per source `expose` while parsing, then
preflights all hosts, target source relations, generated placements, names,
and collisions before one consume-return commit of AST rows, inventory
placement, and relation rows. A private target index may use names for lookup,
but parser-issued same-brand Box paths and existing target method relations
are the authority; inventory ordinals, generated suffixes, AST order, and
`HashMap` order are not source identity. The bounded cohort is ordinary
top-level Rust Boxes with direct explicit target methods. Generated-delegate
chains, compatibility-only delegates, interface/static/record/Hako/provider
cohorts remain outside the row. C does not widen the resolver-visible
`ParserBoxSourceSealV1`; R6-S3B-D owns complete relation coverage, final seal
issuance, and generated-suffix adapter retirement. Any failure discards the
whole unpublished postpass product; partial commit, retry, and name-based
fallback remain forbidden. No C implementation, resolver target, Recipe,
Builder, MIR, or provider integration is open from this receipt.

R6-S3B-C-S0 implementation receipt (2026-08-09): the parser now records one
private `DelegateSourceDeclarationV1` per explicit `expose`, rebases its
parser-issued source member path through selected member-gate merges, and
transports the rows only through `PreparedBoxSourceSealV1`. The final
resolver-visible source seal deliberately excludes these rows until C/D issues
complete generated-delegate relation coverage. Compatibility-only delegates
reject before source authority is acquired. Focused parser tests and the
S0 guard landed in the same implementation slice. Target lookup, generated
placement, resolver target issuance, Recipe/CallSlot, and final seal expansion
remain closed.
The S0 cardinality is one row per expose; generated inventory order is not
source identity.

R6-S3B-C-S1-D0 design receipt (2026-08-09): the next boundary is an accepted
parser-private borrowed `DelegateTargetIndexV1` target index, not a resolver target catalog. It uses
the host field's declared type and the expose's source method name only as
query selectors, then returns the exact same-brand target Box path plus one
existing explicit method source relation. Zero candidates are Unresolved,
ambiguous path candidates are Rejected, and fully observed generated-only,
delegate-chain, compatibility-only, or overload targets are Declined. No AST,
inventory, final seal, Recipe/CallSlot, or runtime state is mutated in C-S1;
the implementation was later closed by its focused guard. The borrowed
`TargetMethodRef` may be reused by multiple exposes and carries no generated
placement or resolver identity.

R6-S3B-C-S1 implementation receipt (2026-08-09): the parser now issues the
private borrowed target index from the unpublished postpass product. The index
aligns ordinary Box AST entries with same-brand exact source paths and prepared
seals, validates existing explicit method relations, and returns a reusable
`TargetMethodRefV1` only for one exact target path plus one explicit relation.
Focused tests cover a reusable positive candidate, missing-field unresolved,
missing-method rejection without fallback, and duplicate-target rejection.
The implementation does not mutate AST/inventory/seals, add generated
placement, extend the final seal, or connect resolver/Recipe/runtime routes.
C-I0 all-host/expose preflight and atomic generated-batch commit have an
accepted parser-private implementation receipt; final relation/seal coverage
remains unopened at the current clean stop.

R6-S3B-C-I0-D0 design receipt (2026-08-09): the accepted parser slice
owns one private `PreparedDelegatePostpassBatchV1` containing staged per-host
forwarder drafts, expected inventory placement receipts, and owned generated
delegate source-relation rows. It borrows the C-S1 target index only during
preflight and may borrow a descriptive target declaration/signature view for
forwarder construction; neither is a semantic or resolver authority. Every
host and expose is matched exactly once before any AST/inventory mutation.
Generated relation rows are carried through the prepared parser payload so D
can verify complete coverage without AST re-scan, but C-I0 does not extend
`ParserBoxSourceSealV1`. Placement is computed against staging inventory and
actual commit receipts must match. Zero-delegate ordinary programs are exact
no-ops; all failures drop the unpublished product with no partial commit,
retry, or fallback.

R6-S3B-C-I0 implementation receipt (2026-08-09): the batch is implemented in
`src/parser/delegate_batch.rs` and carries owned relation rows through
`ParsedProgramWithSourceV1`. Focused tests cover all-host preflight, zero
delegate no-op, generated-name collision, duplicate source rows,
staged-vs-actual placement mismatch, and persisted relation output. The
final non-Clone source seal remains unchanged; no resolver, Recipe/CallSlot,
Builder/MIR, provider/runtime, or production authority opens in C-I0.

R6-S3B-D-I0 implementation receipt (2026-08-09): the finalizer now owns the
single generated-delegate relation/placement coverage check. It validates
same-brand host and target paths, exact `(host path, delegate member source
site, expose ordinal)` relation keys, generated provenance/selection, and the
C-I0 placement receipt against the final inventory. Every generated suffix
entry must have exactly one relation and every relation must land on one
generated entry; duplicates, missing/orphan placements, foreign paths, and
non-delegate rows reject before the non-Clone seal is issued. The final seal
retains the generated relation rows, while AST/name/ordinal reconstruction,
resolver targets, Recipe/CallSlot, Builder/MIR, provider/runtime, fallback,
and retry remain closed. Focused positive/negative tests, the D-I0 guard, and
this reference update landed in the same implementation slice.

## Related references

- [Rune declaration metadata](runes.md)
- [Ownership and Home Flow](ownership.md)
- [Language status index](status-index.md)
