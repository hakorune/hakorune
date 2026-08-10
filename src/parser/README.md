# Parser layer boundary

The parser owns source syntax, ordered source coordinates, and parser-private
transport products. It does not resolve callable targets, issue semantic
contracts, build Recipe/CallSlot products, or emit MIR/runtime routes.

## Shared Box-member source cursor

`source_member_cursor.rs` is the parser-private owner of one Box declaration's
parser invocation brand, exact source path, and next source-member ordinal.
Both ordinary and static Box parsers advance this cursor exactly once after a
source member is successfully parsed. Method inventory ordinals, names, and
arity are never source identity.

The cursor is unpublished transaction staging. It owns no method inventory,
parameter row, source seal, resolver meaning, Take/Home authority, Recipe, or
MIR fact. Static Boxes remain outside `ParserBoxSourceSealV1`; the shared
cursor only gives the later callable-parameter source issuer an exact source
coordinate without reconstructing it from generated/selected inventory.

## Home contextual syntax (`release` source I0 landed)

The language target has exactly three ownership-changing source forms:

```text
declaration  take node: Node
expression   share <non-group postfix expression>
statement    release node
```

`take`, `share`, and `release` remain ordinary `IDENT` spellings; the lexer
does not add global keyword tokens. Contextual recognition is same-line.
`share(...)` and `share (expr)` remain ordinary calls permanently, while
`adopt(share node)` is ordinary-call composition. `take` belongs to the
declaration handoff; `share` and `release` carriers belong to the rich
body transaction below. Parser acceptance proves syntax and source identity
only, never Home capability, demand, availability, sharing representation, or
terminality.

The bounded `release IDENT` parser/source row is live in both parsers. Rust
uses a dedicated `ASTNode::Release`; the one-shot body transaction issues a
non-`Clone` catalog keyed by exact method source site and direct body ordinal.
Nested Release is rejected in this cohort, and all semantic/physical paths
remain unsupported. `take` and `share` parser rows remain inactive. See
`OWN-HOME-SYNTAX-D0` and `OWN-HOME-RELEASE-SOURCE-I0` in the current task map.

## Typed `CallableContract` syntax carriage (I0)

The parser accepts the declaration-local spelling
`@rune CallableContract(query)` only on instance methods. `runes.rs` owns
unknown-value, arity, duplicate, and placement rejection. The small
`callable_contract_syntax.rs` module normalizes the validated attribute into
`CallableContractSyntaxV1::Query`; the explicit method source relation carries
that row together with the non-Clone `SourceBoxMethodSiteV1` and the
declaration-local rune ordinal.

This is syntax carriage, not semantic issuance. The parser does not decide
Home, signature, effect, suspension/control, ABI, body conformance, resolver
targets, Recipe/CallSlot, or MIR. Generated/property/delegate rows receive no
explicit callable-contract row, and inventory ordinals never become source
identity. Conflicts with legacy `Contract`/`Profile` metadata remain a later
semantic issuer responsibility.

## Parser→resolver source handoff (I0)

`source_resolver_handoff.rs` consumes one finalized non-Clone
`ParserBoxSourceSealV1` and returns the AST separately from one
`ParserBoxResolverSourceHandoffV1`. The handoff is AST-free and retains only
the bounded ordinary top-level Rust Box source name, direct method site,
ordered header syntax, typed Query carriage, and diagnostic inventory
placement. It skips generated rows and rejects generated-only or unsupported
cohorts; it never rebuilds identity from `HashMap`, JSON, names, or selected
ordinals. The handoff is non-Clone and cannot be issued twice by ownership.

This is parser transport, not resolver semantic issuance. Semantic types,
nominal Box identity, Home ABI, targets, Recipe/CallSlot, body conformance,
Builder/MIR, and runtime/provider routes remain later owners.

## Body-source transaction boundary (general I0 landed)

The body-source path must not pair an AST with the declaration handoff after
the parser transaction has ended. A parser-private non-`Clone`
`ParserResolverBodyTransactionV1` consumes the rich parse product exactly once
and exposes only:

```text
ParserBoxResolverSourceHandoffV1
ParserBoxBodySourceEnvelopeV1
```

The envelope carries AST-free branded method sites, body-root/item coverage,
and parser provenance. It does not expose an AST, method-name lookup, bare
inventory ordinal, or resolver brand. The general resolver body-source issuer
validates every supported direct declaration; it does not select Query
behavior. A separate Query body-source projection borrows the already sealed
selected Query view, preserves sparse source order, and emits no default row
for non-Query methods. FunctionOwner binding, body facts, conformance,
targets, Recipe/CallSlot, and MIR remain later owners.

## Resolver instance-method syntax lease (carrier I0 — landed)

The accepted next boundary is transaction-scoped, not a second AST API:

```text
ParserResolverBodyTransactionV1::with_direct_method_syntax(self, callback)
  -> handoff + body envelope + borrowed private syntax lease
  -> callback returns AST-free resolver carrier/catalog only
```

The lease is non-`Clone`, cannot escape the callback lifetime, and is keyed by
the parser-issued direct source site. The resolver constructs its canonical
`FunctionSyntaxViewV1` only inside this callback; after the callback returns,
no resolver product retains an AST or syntax pointer. Name, inventory ordinal,
legacy AST lookup, and caller-built function maps are forbidden.

The carrier I0 receipt is closed. The lease is used only by the existing
`FunctionSemanticResolverSessionV1` owner-forest issuer; the next semantic
boundary is the catalog-level owner binding D0, not another parser API.

## C-S1 delegate target index

`delegate_target_index.rs` is a borrowed lookup proof over one unpublished
`OpenParserPostpassProductV1`. Exact parser invocation brand, Box declaration
path, and existing explicit method source relation are the authority. Field
type and expose method names are query selectors only. The index and its
`TargetMethodRefV1` results never mutate AST, method inventory, prepared/final
seals, or generated delegate placement.

The R6-S3B-C-I0-D0 design is accepted and its bounded implementation is
closed. The private `PreparedDelegatePostpassBatchV1` owns the staged batch:
all host/expose rows are preflighted, target method declaration/signature views
are borrowed only for forwarding AST construction, placement receipts are
computed against a staging inventory, and generated relation rows are carried
through the prepared source payload. A single consume-return commit applies
AST, inventory, and relation changes; any failure drops the whole unpublished
postpass product. C-I0 does not extend `ParserBoxSourceSealV1`; R6-S3B-D alone
may issue complete resolver-visible generated relation coverage.

The C-I0 implementation receipt is implemented by `delegate_batch.rs` and
`delegate_source_relation.rs`. `ParsedProgramWithSourceV1` exposes the
parser-private generated relation rows for D without an AST/name rescan.
Focused tests cover zero-delegate no-op, later-host atomic failure, generated
name collision, duplicate source rows, staged-vs-actual placement mismatch,
and persisted relation output. Final-seal, resolver, Recipe, Builder, MIR,
provider, and runtime authority remain closed.

## R6-S3B-D-D0 / D-I0 final-seal boundary

The D0 design is accepted and D-I0 is closed for the bounded ordinary-Box
rich path.
It is the only boundary allowed to consume the C-I0 relation rows and extend
the non-Clone `ParserBoxSourceSealV1`. Its finalizer-owned coverage plan must
compare exact same-brand relation keys and generated inventory placement
receipts without rescanning AST names or rebuilding source identity from
ordinals. The implementation is limited to that final seal and retirement of
the bounded S3A generated-suffix adapter; resolver, CallableContract, Recipe,
Builder/MIR, provider, and runtime work remain outside the row.

The D-I0 finalizer now retains the complete generated relation rows in the
seal. It rejects duplicate or orphan relation keys, foreign host/target paths,
non-delegate placements, and provenance/selection mismatches before issuing
the seal. The focused final-seal tests and
`frontend_parsed_box_source_seal_r6_s3b_d_i0_guard.sh` are part of this same
implementation slice; resolver/runtime connection remains closed. The public
AST-only `parse`/fuel/explain entrypoints remain compatibility nonclaims
because the rich finalizer intentionally rejects interface/static/record/mixed
cohorts. Their total postpass envelope is tracked by
`PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0/I0`; no catch-and-fallback cutover is
allowed.

## Broad AST postpass design stop

The broad parser API family will converge on one private postpass owner, not
on a rich-path attempt followed by a legacy retry. The owner will consume one
parser invocation, AST, source session, metadata, and optional explain demand
and return a typed total result:

```text
CompletedParserPostpassV1
  ast + metadata + optional explain + per-Box coverage

coverage row
  SourceSealedOrdinary(ParserBoxSourceSealV1)
  | AstOnlyCompatibility(typed interface/static/record/mixed cohort)
```

Only `SourceSealedOrdinary` is resolver-visible. Compatibility rows are
successful AST projections and never carry an empty seal, target, Recipe
input, or fallback marker. Final AST placement is coverage metadata; parser
source paths/sites remain identity and inventory ordinals never become source
identity.

The same private `PreparedBuildGateDecisionSetV1` (implementation name may
change) must feed AST prune, explain-report projection, and top-level ordinary
source-path rebase. Fuel is configured once at parser construction; metadata
is taken once by the envelope; `NyashParser::parse` uses the same owner without
re-tokenizing. Explain cutover is parked until the shared all-gate decision
set has parity.

S0 now implements the private `CompletedParserPostpassV1` envelope, structural
cohort classifier, explicit compatibility delegate arm, and
`OpenParserPostpassProductV1::finish_total_s0`. The bounded rich ordinary path
uses the shared postpass-opening helper. Public AST, metadata, and explain
callers are still unchanged; I0-A/B/C own those switches and their parity
receipts.

## I0-A string/build-config edge — closed

`parse_from_string_with_fuel_and_build_config` now enters the private
`string_postpass_entry` owner exactly once: tokenize, configure fuel/build
profile, parse, open the total postpass product, and project the AST. Its
convenience wrappers inherit the same edge. Compatibility cohorts remain
successful AST transport; they never become source seals or fallback markers.
The prior delegate-only production call was removed from this edge, while
grammar-evidence, metadata, `NyashParser::parse`, and explain callers remain
parked for I0-B/I0-C. Focused and parser regression tests preserve AST shape,
fuel, diagnostic-family, gate, cohort, and delegate behavior. No resolver,
Recipe, Builder, MIR, runtime, or remaining caller retirement is claimed. The
parent `72b3471e55` has one separate baseline-red nested member-gate
source-path test that fails before postpass opening because its branches have
different public signatures; it is parked as
`PARSER-MEMBER-GATE-NESTED-SOURCE-PATH-D0` and is not an I0-A regression.

## I0-B parse/metadata projections — closed

`NyashParser::parse` and `parse_from_string_with_fuel_and_metadata` now share
the parser-private `parse_postpass_s0` finalizer. The completed postpass owns
metadata until one consuming `into_ast_and_metadata()` projection; AST-only
callers use `into_ast()`. No caller reparses, retakes metadata, or reconstructs
it from AST nodes. Explain/full BuildGate decision-set parity remains parked
for I0-C.

## I0-C BuildGate decision-set and projection — closed

The postpass-visible AST `BuildGate` family will use one parser-private
`PreparedBuildGateDecisionSetV1`. Parser-issued observations cover every AST
gate, including nested statement gates; the set evaluates each predicate once
and feeds prune, top-level source-path survival, and explain projection.
Consumers never call `eval_build_predicate` again. The decision coordinate is a
private structural projection, not source/resolver identity or final AST
ordinal, and the top-level source ledger remains the narrower source-path
authority.

All structural predicates, including inactive subtrees, are validated once.
Explain counters retain the v0 reachable-row projection, while inactive rows
remain in the decision set for coverage and diagnostics. Member-level gates
remain the separate parse-time BoxMemberState/signature contract; grammar
evidence remains a separate demand. I0-C opens no resolver, Recipe, Builder,
MIR, runtime, fallback, retry, or reparse path.

S0 receipt (2026-08-09): `src/parser/build_cfg/decision_set.rs` now issues
the non-Clone `PreparedBuildGateDecisionSetV1` from parser-owned observations.
It aligns every postpass-visible AST gate, validates nested predicate
configuration even in inactive subtrees, evaluates each top-level predicate
once, and records selected branch plus reachability. Focused I0-C tests and
the existing 12-case BuildCfg regression gate are green.

Projection I0 receipt (2026-08-09): `build_cfg/projection.rs` is now the one
shared structural walker. It borrows the decision set, traverses selected and
inactive branches for complete coverage, emits only the selected AST, and
produces validated source receipts plus reachable-row v0 explain output.
`source_seal.rs` consumes this aggregate and derives retained Box paths from
prepared source seals; the old evaluator/generic-prune path is not used by the
shared postpass. Public explain uses the same completed postpass product.

The post-closeout structural-gate correction is also part of this boundary:
BuildGates inside method/function bodies remain decision-covered, but receive
no top-level source-ledger identity. Only observations issued while
`SourceBuildGateScopeV1::TopLevelItem` is open may produce source selection
receipts. The 12-case BuildCfg gate is green after this correction.

The next `PARSER-PUBLIC-AST-POSTPASS-FINAL` row is a retirement-proof design
stop. The broad public callers are already caller-zero on the old edge, so the
row must quarantine grammar-evidence and the explicit compatibility arm,
retire only caller-zero helpers, and never invent a production switch. A
top-level no-else source receipt must receive an explicit typed representation
before helper retirement; mapping it to `Else` is forbidden.

The accepted D0 representation is intentionally split by authority:

```text
BuildGateSelectionOutcomeV1::{Then, Else, NoElse}
  = semantic decision outcome carried by the selection receipt

SourceBuildGateBranchV1::{Then, Else}
  = source/Box path segment only
```

Every top-level gate keeps one source record and one receipt. `NoElse` creates
no child path and cannot authorize a descendant; final-seal survival matches
only Then/Then or Else/Else. NoElse-to-Else mapping and missing receipts are
not compatibility behavior.

FINAL-RETIRE-S0 is closed: the caller-zero `source_gate_prune.rs` owner and
old `explain_build_gate_program` helper were removed. Grammar-evidence
selection, resolver source-seal transport, and the explicit compatibility arm
remain separate owners; this retirement did not change receipt or path
semantics.

FINAL-GUARD-CLEANUP-S0 is the bounded closeout row: active B2/B3/D-I0 guards
now validate `build_cfg/prune.rs`, `source_seal.rs`, and the private finalizer
owners rather than the retired helper. The retired filename remains only in
historical retirement evidence; it is not an active parser authority.
