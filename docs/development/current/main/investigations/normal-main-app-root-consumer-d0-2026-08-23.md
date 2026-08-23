Status: G0-H1 complete — affine HRTB root-loan S0 selected
Date: 2026-08-23
Decision: NORMAL-MAIN-APP-ROOT-CONSUMER-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-root-source-disposition-d0-2026-08-23.md
ProductionCaller: root consumer 0; S0 may remain caller-zero for one commit only, and its immediately reserved successor connects the sole lifecycle caller
ProductionEdit: only the parser-owned loan substrate is authorized in S0; no Builder effect, lowering, fallback, or production switch is part of that commit
CeremonyTier: accepted T2 Decision -> one-commit S0, then mandatory production I0
---

# NORMAL-MAIN-APP-ROOT-CONSUMER-D0

## Six-line brief

```text
Decision:
  accept one parser-owned HRTB root view and one move-consuming lifecycle
  facade; App uses a typed Program cursor with a hidden RootMain item rather
  than a raw Program or root-body-only projection.
Source authority + canonical issuer:
  the H1 private App relation and ParserNormalProgramSourceAuthorityV1 remain
  the only owners; VerifiedFinalCallableProgramSourceV1 is the sole scoped
  view issuer, and PreparedNormalDefaultProgramRootV1 owns production affinity.
Non-authority:
  raw Program access, names, ordinals, pointers, AST role rescans, Script-A
  rows, VerifiedRawRootExpansionV1, semantic package presence, Builder, and MIR.
Fail-fast boundary:
  the sole source-backed consume starts at the lifecycle entry before raw
  preflight, declaration/resolver work, target install, module, catalog, or
  work-plan effects; every terminal and loan-integrity error stops there.
Smallest next slice:
  NORMAL-ROOT-CONSUMER-LOAN-S0 adds only the parser loan, typed states, tests,
  guard, and README receipt; it is one caller-zero commit whose immediate next
  commit must be NORMAL-ROOT-CONSUMER-I0 or the S0 is reverted.
Non-claims:
  root lowering, Recipe/work-plan changes, raw observer retirement, ABI/result
  semantics, static-child expansion, MIR/publication, fallback, and performance.
```

## Current evidence

The previous transport I0 is landed as `eeac2da553`, with closeout pointer
`ef88ba6c53`. The source-root disposition now moves through the parser source
chain as one required field:

```text
ParsedProgramWithCallableParameterSourceV1
  -> PreparedNormalCallableProgramSourceV1
  -> VerifiedFinalCallableProgramSourceV1
  -> PreparedNormalDefaultProgramRootV1
```

`CanonicalScriptSourceRowsV1` remains the separate Script-A handoff product.
The reference A route explicitly discards the root disposition and rejects
`AppReady`; no root consumer exists yet.

The current root lifecycle still does this twice:

```text
PreparedNormalDefaultProgramRootV1
  -> VerifiedRawRootExpansionV1::from_program(source_ast)
  -> is_app_mode: bool
  -> Builder/root work plan
```

The second scan occurs after catalog installation, before lowerer invocation.
This is a second root observer and is not allowed to become the canonical
consumer of the parser disposition.

## Fresh top-down worker audit — `NoSafeSlice` confirmed

The requested read-only top-down audit confirms `NoSafeSlice` for consumer
implementation.  The parser-side App admission is a Keeper, and the normal
Program-body loan is the correct candidate authority for Script.  Neither
currently proves the final transformed root structure required by the existing
App/Script lowerer:

```text
AppReady(P1) + final root AST(P2)
  -> exact root/static-child relation is not yet source-sealed
```

In particular, `validate_parser_normal_program_source_transform_v1` preserves
the covered body prefix and composite relation but currently permits additional
transformed root statements.  It therefore cannot yet be used as the root
cohort-preservation proof.  `VerifiedMainExpansionV1::from_program` remains a
second AST/name/ordinal classifier and is not an acceptable repair.

The current issuer chain is only an admission chain:

```text
ParserStaticBoxParentSourceAuthorityIssuerV1::issue_once
  -> issue_parser_main_app_entry_v1
  -> issue_parser_normal_root_source_v1
```

`ParserMainAppEntrySealV1` proves the Main declaration/method admission, but it
does not yet co-seal the root body, static-child relation, and final transformed
AST.  That missing co-seal is the actual blocker; adding a Builder-side
projector before it exists would create a second authority.

For Script, `ParserNormalProgramSourceAuthorityV1` and its HRTB loan are the
right source owner, but the current loan still exposes `program()` and the
transform guard permits an appended root suffix.  The future root boundary
must therefore use a narrower root-only view and must not hand raw AST access
to the Builder consumer.

The ordered prerequisite series is fixed below. G0-H1 are complete and the
HRTB loan Decision alone is open; until J0
lands, the production root consumer count remains zero.

### Second top-down worker audit — the transform boundary is the real predecessor

The App admission data is stronger than the first audit needed to assume:
`ParserMainAppEntrySealV1` already proves exact static `Main`, exactly one
direct member, `main/0`, and one callable identity under the parser invocation.
The remaining unsafe input is the final transform itself:

```text
parser-owned source prefix
  -> ParserNormalCallableTransformSessionV1::finish(FnOnce -> ASTNode)
  -> maybe_expand_and_dump
  -> test harness may append top-level setup/call rows
  -> prefix-only root preservation currently accepts the unowned suffix
```

The direct counterexample is a Script-ready top-level `test_*` function.  The
test harness can append its call after all parser-owned rows.  The appended row
has no parser invocation relation, yet the current prefix check can still issue
a Ready root token.  Conversely, changing only the check to exact length would
silently reject the existing test-harness feature.

The accepted correction is an upstream transform disposition:

```text
normal callable macro owner
  ├─ ExactUnchanged
  │    -> ParserNormalCallableTransformSessionV1::finish_exact(self)
  │    -> exact final source/root preservation
  │
  └─ TestHarnessGeneratedTail(transformed AST)
       -> explicit Compatibility(TestHarnessGeneratedTail)
       -> parser final source/root token count = 0
```

This is not a retry after canonical rejection.  The macro owner issues the
disposition while performing the transform, before the parser final-source
attempt starts.  The parser does not reread `NYASH_TEST_RUN`, infer suffix
origin from AST shape, or compare a foreign AST to recover authority.

If composite source preservation is Ready, an actual generated tail remains a
typed `CompatibilityLoss` rejection, matching the existing registered-macro
and default-derive rule.  It may not discard that stronger source authority to
enter compatibility.  If the macro engine mutates the AST despite no named
registered/default transform, the result is a typed unclassified-mutation
reject, not `ExactUnchanged` and not an anonymous compatibility row.

Canonicalizing generated test rows in the root token is rejected for this
slice.  That would require a second macro-generated source authority, complete
generated rows, and Recipe coverage for those rows.  The bounded root lane
instead contains parser source only.

The following are not valid repairs: using the current raw expansion,
comparing names or ordinals in MIR, treating appended rows as default
Script/App state, or exposing `ParserMainAppEntrySealV1` to Builder.

## Recommended design — A-prime behind one exact-transform gate

Choose a narrowed A-prime after the accepted transform prerequisite:

```text
one parser-owned source product
  -> one macro-owned ExactUnchanged disposition
  -> one parser finish_exact move
  -> one opaque, non-Clone root-preservation token
  -> one move-only normal-root consumer
```

The token is issued inside the existing parser root-source authority family;
it is not a Builder classifier and it exposes no parser anchor, name, ordinal,
digest, pointer, or Script-A row.  Its private proof must establish:

```text
AppReady:
  exact parser Main admission + exact root/static-child relation + unchanged
  final-transform root cohort + no App role drift
ScriptReady:
  exact normal Program-body root relation + unchanged final-transform root
  cohort + no new Main role
both:
  same source product, same parser witness, exact root shape, typed drift errors
```

App/Script structural extraction in Builder may then be a projection under an
already-admitted role.  It must not decide the role, reissue source facts, or
scan a second AST to override the token.  An actual generated test tail never
enters this bounded root lane: it is classified by its generator before parser
finalization.  Unknown additions, removals, reorderings, or replacements remain
typed rejects.

The first implementation slice is now bounded and accepted.  The root consumer
itself remains closed until exact-root co-seal and the narrow HRTB view land.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `issue_parser_main_app_entry_v1` | exact parser App `Main.main/0` relation | Builder route or MIR |
| `issue_canonical_script_cohort` / Script-row issuer | positive Script cohort and A rows | App selection or root effects |
| `issue_parser_normal_root_source_v1` | same-invocation App/Script disposition and typed terminals | structural lowering or Builder state |
| `ParserNormalProgramSourceAuthorityV1` | exact Program-body coverage and HRTB AST/source loan | App/Script selection, target, MIR |
| normal callable macro/test-harness transform issuer | whether this invocation actually generated a test tail and its transformed compatibility AST | parser/source authority or root admission |
| `ParserNormalCallableTransformSessionV1::finish_exact` | one-shot transition from unchanged parser source to final source issuance | arbitrary foreign AST acceptance or compatibility selection |
| `ParserNormalRootPreservationIssuerV1` | exact final App/Script preservation after the transform gate | macro-tail provenance or Builder projection |
| `PreparedNormalDefaultProgramRootV1` | move-only transport wrapper | source reclassification |
| `ModuleBuilderInvocationSessionV1` lifecycle | future one-time root consumer and unpublished session | parser truth or second root classifier |
| `VerifiedRawRootExpansionV1` | legacy structural projection candidate only | App/Script authority |
| `PreparedProgramRootWorkPlanV1` | already-admitted root work plan | root selection or source inference |

The key missing authority is not another App/Script classifier. It is an
exact, source-backed relation from `AppReady` to the structural data needed by
the existing App lowerer. The existing `VerifiedMainExpansionV1::from_program`
does not provide that relation: it independently searches for `Main`, accepts
shapes the parser admission rejects, and does not carry the parser witness.

## Candidate consumer and required API shape

The only suitable consumer boundary is the beginning of
`complete_normal_default_program_root_catalog_lifecycle_with_target`, before
`install_pinned_text_target_capability` and before
`prepare_normal_default_module`.

The consumer must be move-only and opaque across the parser/MIR boundary. A
plain getter is rejected because it permits an unconsumed disposition and a
second raw observer. The design must choose one of these equivalent shapes:

```text
PreparedNormalDefaultProgramRootV1
  -> consume_root_source(self)
  -> RootConsumerInputV1
  -> lifecycle
```

or an HRTB callback whose callback owns the complete unpublished root session
and cannot return parser anchors or a borrowed AST. The resulting type must
make a second consume impossible without adding a parallel `Option` state.

The consumer input may expose only closed `App` / `Script` root roles plus
typed terminal errors. It must not expose `ParserMainAppEntrySealV1`, source
paths, names, ordinals, AST pointers, or A rows to MIR.  The safest shape is a
parser-owned HRTB callback that lends a root-only view for the duration of the
named consumer; a self-referential struct containing an owned root AST and a
borrow into that same AST is not allowed.

The current `ParserNormalProgramSourceLoanV1::program()` is therefore not a
valid root-consumer boundary.  D0-I must either narrow it or introduce a
separate `with_normal_root_consumer_view(...)` API whose view has no raw
Program accessor.

## Structural projection decision still missing

For `ScriptReady`, the existing normal Program-body source authority is the
natural structural input. It does not need Script-A rows. The resulting Script
route must preserve the existing semantic package/window path without
rebuilding a Script label from AST absence.  The view must be paired and
HRTB-scoped; raw `program()` access is not part of the contract.

For `AppReady`, the existing `VerifiedMainExpansionV1` needs an admitted-source
entry point, not its current classifier entry point.  The parser-side root
issuer (the existing final-transform preservation issuer, or a parser-private
sub-issuer called only by it) must first co-seal the exact relation.  The
Builder entry point must then accept that proof rather than search the AST.
The relation must prove, under the same parser invocation and final-source
transform:

```text
AppReady seal
  ↔ exact static Main declaration
  ↔ exact main method source relation
  ↔ accepted root/static-child structural projection
```

If this relation cannot be supplied without exposing parser anchors or adding a
second source issuer, the result is `NoSafeSlice`; do not weaken the check to
name, ordinal, digest, pointer, or AST shape equality.

## Accepted bounded task series after both worker audits

The series is ordered by authority. G0-H1 are complete; I0 implementation is
closed behind `NORMAL-ROOT-CONSUMER-LOAN-D0`. A later row does not become
executable merely because an earlier focused test is green.

```text
G0 / NORMAL-CALLABLE-SOURCE-TRANSFORM-DISPOSITION-I0  [complete: 198560b0e0]
      Make the macro/test-harness owner return Unchanged or actual
      GeneratedTail.  Only Unchanged may call a no-argument finish_exact;
      GeneratedTail enters the existing typed compatibility lane before root
      token issuance.  Remove the production FnOnce -> ASTNode finish API.

G1 / NORMAL-MAIN-ROOT-EXACT-COHORT-I0                 [complete: 0b9d4eb43d]
      Require source body count == initial count == final count and exact full
      statement preservation.  Addition, removal, reorder, or replacement is
      a typed reject.  Do not add logic to main_expansion.rs.

H1 / NORMAL-MAIN-APP-ROOT-RELATION-I0                 [complete: b96d3f17b3]
      In the existing final-source issuer, co-seal the private App admission,
      exactly one matching callable identity, its paired final slot, the same
      invocation, Main-only-member relation, and NoStaticChildren.  Do not
      expose parser sites, anchors, names, or ordinals.

S0 / NORMAL-ROOT-CONSUMER-LOAN-S0                     [selected fast cell]
      Add one parser-owned HRTB root view. App lends a typed root body plus an
      exact Program-order cursor whose Main row is an opaque RootMain marker;
      Script lends a paired statement cursor. No raw Program getter, parser
      identity, placement ordinal, or Script-A row escapes. Caller-zero is
      permitted for this one commit only.

I0 / NORMAL-ROOT-CONSUMER-I0                          [reserved immediately after S0]
      Consume PreparedNormalDefaultProgramRootV1 by move at the beginning of
      the named lifecycle. Source-backed root role/terminal authority comes
      only from the loan; the old callable raw preflight edge becomes zero.
      The post-loan wrapper exposes no reloan or generic parts escape.

R0 / NORMAL-RAW-ROOT-OBSERVER-R0                      [closed behind I0]
      Replace the remaining source-backed callable-catalog and post-catalog
      raw observers with admitted projections, remove bool-based selection,
      and prove fallback/retry zero. Compatibility and test-owned raw observers
      remain under their existing owner.
```

The deterministic source sentence is:

```text
parser exact App/Script source
  -> macro owner ExactUnchanged
  -> parser exact-root relation
  -> narrow HRTB root view
  -> one pre-effect lifecycle consumer
  -> existing root Recipe/work plan
```

Generated test tails leave this sentence before parser final-source issuance.

## Finite state table

| Root state | Consumer result | Effects before terminal | Allowed next step | Fallback |
| --- | --- | ---: | --- | --- |
| `AppReady` | admitted App root input, only after exact structural relation | 0 | root lifecycle | none |
| `ScriptReady` | admitted Script root input from normal body authority | 0 | existing Script semantic path | none |
| `Outside` | typed root terminal | 0 | discard session | none |
| `ScriptTerminal(_)` | typed Script-source terminal | 0 | discard session | none |
| `SourceAuthorityUnavailable(_)` | typed source terminal | 0 | discard session | none |
| `Incomplete(_)` | typed incomplete terminal | 0 | discard session | none |
| `IntegrityInvalid(_)` | typed integrity terminal | 0 | discard session | none |
| `DiscardedBeforeA` | route mismatch on normal root path | 0 | typed reject | none |
| compatibility outer lane | existing compatibility owner | existing compatibility policy | compatibility route only | no synthetic root state |

`AppReady` and `ScriptReady` are not candidate/noncandidate dispositions. They
are source-root admission states. The root consumer must not turn a terminal
state into Script, App, compatibility, or a legacy retry.

## Counterexamples that must remain rejected

```text
static box Main { main(argument) { return argument } }
  parser: Outside(NonZeroMainArity)
  raw AST classifier: may choose App
  required result: typed terminal, no Builder effect

static box Main { main() {} helper() {} }
  parser: Outside(StaticParent(DirectMethodCohort))
  raw expansion: may accept extra static children
  required result: typed terminal, no Builder effect

parser App/Script witness from invocation P1 + final AST/source from P2
  required result: ParserWitnessMismatch / integrity terminal

AppReady on Script-A frontdoor
  required result: existing typed reject, never Script discard

top-level test_* function with an actual generated harness call
  required result: Compatibility(TestHarnessGeneratedTail), root token count 0

composite-ready source with an actual generated harness tail
  required result: CompatibilityLoss, no compatibility origin and no root token

P1 source session + a structurally identical raw AST produced independently
  required result: impossible on the production exact API; no raw callback
```

## NoSafeSlice conditions

Remain at `design_stop` if any one holds:

```text
no single named root consumer can be connected before Builder effects
App structural projection has no source-owned exact relation
actual generated tail cannot be distinguished by the generator itself
the parser must reread an env flag or infer suffix origin from AST structure
the production final-source API still accepts FnOnce(&ASTNode) -> ASTNode
an unclassified macro mutation can enter ExactUnchanged or compatibility
composite Ready can be discarded for generated-tail compatibility
VerifiedRawRootExpansionV1::from_program must remain the App/Script classifier
root disposition must be exposed through parser anchors, names, or ordinals
root consumer needs Script-A rows or clones them
consume requires a parallel Option/default/compatibility state
non-ready states cannot terminate before target/module/catalog effects
root lifecycle must retain a second AST scan as an authority
root bool or semantic-package presence remains the selection input
the source-to-Recipe sentence cannot be stated in one deterministic line
the bounded slice needs root lowering, ABI, publication, fallback, or switch
```

## G0 acceptance and execution gate

The main-thread source audit and the independent top-down worker audit agree on
the same finite G0 mapping.  The following evidence is required for the bounded
implementation:

```text
macro disabled or actual no-op
  -> SourceBacked via finish_exact; transformed AST argument count = 0

actual nonempty test-harness tail, composite source not Ready
  -> Compatibility(TestHarnessGeneratedTail); root token count = 0

actual nonempty test-harness tail, composite source Ready
  -> typed CompatibilityLoss; compatibility origin count = 0

registered macro/default derive
  -> existing disposition unchanged

unexpected macro-engine mutation outside named dispositions
  -> typed reject; no source-backed/compatibility fallback
```

Structural guards must prove one production `finish_exact` caller, zero
production raw transform callbacks, one GeneratedTail issuer, zero parser env
reads, zero post-reject retry, and zero root token issuance on generated-tail
paths.  Focused positive/negative tests, `cargo check`, pointer guard, and
`git diff --check` close G0.  Production source remains below the 760-line split
trigger and 800-line hard stop; `main_expansion.rs` (743 lines) and
`program_root_work_plan.rs` (721 lines) are forbidden edit targets in G0-H1.

G0 changes one bounded transform disposition and is therefore the selected
BoxCount.  It does not authorize G1, H1, I0, J0, or K0 in the same change.

### G0 closeout evidence

G0 is implemented and pushed on `main` as `198560b0e0`:

```text
actual no-op
  -> one production finish_exact caller

actual generated tail
  -> one macro-owned GeneratedTail issuer
  -> typed TestHarnessGeneratedTail compatibility before parser finalization

composite Ready + generated tail
  -> typed CompatibilityLoss

unknown macro mutation
  -> typed UnclassifiedSourceMutation
```

The production raw-AST callback and the retired AST-only test-harness wrapper
are both caller-zero and removed. The parser callback survives only behind
`#[cfg(test)]` for the existing preservation drift matrix. No root consumer,
root lowering, fallback, or production switch was added.

Acceptance evidence:

```text
cargo test normal_callable_transform --lib -- --nocapture
  -> 7 passed

cargo test parser::normal_callable_program_source::tests --lib -- --nocapture
  -> 28 passed

cargo check
  -> passed (existing warning baseline only)

parser_normal_root_preservation_a_i0_guard.sh
current_state_pointer_guard.sh
git diff --check
  -> passed

production source maxima in the touched owners
  -> test_harness.rs 523 lines
  -> normal_callable_program_source/model.rs 482 lines
  -> all touched source below 760
```

The repository-wide `cargo fmt --all -- --check` still reports the existing
format baseline in 79 untouched files; its intersection with this change's
file set is zero. All touched Rust files were formatted directly. This is
classified as known baseline debt, not a G0 failure.

### G1 closeout evidence

G1 is implemented and pushed on `main` as `0b9d4eb43d`. The existing sole
`ParserNormalRootPreservationIssuerV1` now requires one exact root cohort:

```text
parser-owned body-row count
  == initial Program statement count
  == final Program statement count

initial statement[position]
  == final statement[position]
```

The old prefix/suffix model, static-`Main` name scan, and non-callable-tail
acceptance are removed. Addition is rejected by the root cardinality check;
removal is rejected even earlier by the existing parser program-source owner;
reorder and replacement return the exact changed position. Existing callable,
constructor, and composite drift validators retain their earlier typed errors
before the final root co-seal.

Acceptance evidence:

```text
cargo test parser::normal_callable_program_source::tests --lib -- --nocapture
  -> 29 passed

cargo test normal_callable_transform --lib -- --nocapture
  -> 7 passed

cargo check
parser_normal_root_preservation_a_i0_guard.sh
current_state_pointer_guard.sh
targeted rustfmt --check
git diff --check
  -> passed

normal_root_preservation.rs 165 lines
normal_callable_program_source/transform.rs 90 lines
normal_callable_program_source/tests.rs 652 lines
  -> all below 760
```

No App relation, HRTB loan, root consumer, Builder effect, fallback, or raw
observer retirement was added in G1.

### H1 closeout evidence

H1 is implemented and pushed on `main` as `b96d3f17b3`. The existing sole
`ParserNormalRootPreservationIssuerV1` now turns its ready state into one closed
private relation:

```text
App admission seal
  + exactly one opaque-identity-matching callable row
  + that row's already-paired final slot
  + same parser invocation
  + exact unchanged root cohort
  -> private App root relation
```

The relation moves the original non-`Clone` App admission seal and privately
retains the paired BoxMethod slot, `CallableMainIsRoot`, and
`NoStaticChildren`. Identity is the primary pairing authority; source path and
placement are private redundant integrity checks. No parser site, anchor, name,
or ordinal is exposed, and neither `main_expansion.rs` nor Builder was changed.

Focused counterexamples prove that structurally identical rows from another
parse do not pair, a foreign parser witness rejects before pairing, a TopLevel
slot cannot replace the paired BoxMethod slot, callable/slot cardinality drift
rejects, and a Main helper remains terminal before relation issuance. A
top-level callable sibling remains accepted because `NoStaticChildren` applies
to the admitted Main parent rather than the whole Program callable set.

Acceptance evidence:

```text
cargo test parser::normal_callable_program_source --lib -- --nocapture
  -> 34 passed

cargo test normal_callable_transform --lib -- --nocapture
  -> 7 passed

cargo check
parser_normal_root_preservation_a_i0_guard.sh
current_state_pointer_guard.sh
targeted rustfmt --check
git diff --check
  -> passed

normal_root_preservation.rs 299 lines
normal_root_preservation_tests.rs 136 lines
normal_callable_program_source/transform.rs 92 lines
  -> all below 760
```

No HRTB loan, root body exposure, lifecycle consumer, Builder effect, fallback,
production switch, or raw observer retirement was added in H1.

## Accepted D0 — affine HRTB root-consumer loan

The main-thread source audit and the independent read-only top-down audit agree
on a conditional Accept. The earlier root-body-only App sketch was incomplete:
H1 forbids static children inside `Main`, but an App may still contain a
top-level callable or another top-level declaration. Dropping those rows would
change the Program work-plan cohort.

The corrected App view therefore preserves exact Program order while hiding
the admitted Main declaration itself:

```text
parser body row + exact final statement
  -> RootMain                         when it is the H1 App root statement
  -> Sibling { kind, statement }      for every other exact Program row
```

`RootMain` occupies the original cursor position. A consumer may enumerate the
cursor without shifting later source indices, but it cannot inspect the raw
Main Box, recover its source locator, or classify it again.

### Ownership and API shape

The parser view issuer is read-only source observation. Production affinity is
owned separately by the move-only prepared-root facade:

```rust
impl VerifiedFinalCallableProgramSourceV1 {
    pub(crate) fn with_normal_root_consumer_loan<R>(
        &self,
        consume: impl for<'src> FnOnce(
            ParserNormalRootConsumerLoanV1<'src>,
        ) -> R,
    ) -> Result<R, ParserNormalRootConsumerLoanRejectV1>;
}

impl PreparedNormalDefaultProgramRootV1 {
    fn consume_callable_root_loan<R>(
        self,
        consume: impl for<'src> FnOnce(
            ParserNormalRootConsumerLoanV1<'src>,
        ) -> R,
    ) -> Result<
        (PreparedNormalDefaultProgramRootAfterLoanV1, R),
        NormalRootConsumerRejectV1,
    >;
}
```

`PreparedNormalDefaultProgramRootAfterLoanV1` is lifecycle-private, non-Clone,
and has no root-loan method, raw-parts escape, or generic inner-source getter.
It may only continue into the existing semantic-package issuance. Thus the
parser remains the source-view issuer while the named lifecycle owns exactly
one production consume. No self-referential owner-plus-borrow product exists.

### Borrowed view

```rust
enum ParserNormalRootConsumerLoanV1<'src> {
    App(ParserNormalAppRootLoanV1<'src>),
    Script(ParserNormalScriptRootLoanV1<'src>),
}

struct ParserNormalAppRootLoanV1<'src> {
    root: ParserNormalAppRootBodyLoanV1<'src>,
    program: ParserNormalAppProgramCursorV1<'src>,
    callable_relation: ParserNormalAppRootCallableRelationRefV1<'src>,
    // private CallableMainIsRoot / NoStaticChildren proofs
}

enum ParserNormalAppProgramItemLoanV1<'src> {
    RootMain,
    Sibling {
        kind: ParserNormalProgramBodySyntaxKindV1,
        statement: &'src ASTNode,
    },
}

struct ParserNormalScriptStatementLoanV1<'src> {
    kind: ParserNormalProgramBodySyntaxKindV1,
    statement: &'src ASTNode,
}
```

The App root body exposes only `body`, `uses`, `attrs`, and a closed
`Implicit | Explicit(&str)` result-syntax view. Static Main, zero arity,
Main-is-root, and no-static-children are represented by the admitted App loan,
not by names or empty defaults. `params`, `param_decls`, the raw function node,
parser anchors, source sites, and final-slot ordinals do not escape.

The opaque callable relation may answer whether an already paired final
callable row is this root. It returns no identity, name, pointer, or locator.
Script exposes only one non-Clone paired cursor; source rows and AST statements
are never returned as parallel slices. The existing general Program loan and
its `program()` accessor remain valid for their existing Script authorities,
but are not reachable through this root-consumer view.

The HRTB makes `R` independent of `'src`, so a callback cannot return an AST
reference, cursor item, root body, or callable relation. An owned lowering or
semantic product may be returned only after consuming the scoped view.

### Finite state table

| Input state | Loan result | Pre-terminal effect | Next | Fallback |
| --- | --- | ---: | --- | --- |
| `Ready(App)` | exact App body + RootMain Program cursor + opaque callable relation | 0 | callback once | none |
| `Ready(Script)` | exact paired Script statement cursor | 0 | callback once | none |
| `Outside(reason)` | typed terminal retaining reason | 0 | discard | none |
| `ScriptTerminal(reason)` | typed terminal retaining the exact existing subreason | 0 | discard | none |
| `SourceAuthorityUnavailable(reason)` | typed terminal | 0 | discard | none |
| `Incomplete(reason)` | typed terminal | 0 | discard | none |
| `IntegrityInvalid(reason)` | typed terminal | 0 | discard | none |
| `DiscardedBeforeA` | typed route-mismatch terminal | 0 | discard | none |
| ready relation / final syntax contradiction | typed loan integrity reject | 0 | discard | repair forbidden |
| typed/raw compatibility | outside this parser loan | existing compatibility policy | compatibility only | no synthetic root state |

`ScriptTerminal` retains `NotApplicable`, `CompatibilitySource`, `Deferred`,
`AdmissionMissing`, `CohortUnresolved`, `ObservationIncomplete`,
`NonCandidate`, `DispositionTransported`, and `RowsNotHandoffReady` without a
wildcard, default, or early conversion to `String`.

The sole future production callpoint is the first source-backed operation in
`complete_normal_default_program_root_catalog_lifecycle_with_target`. It is
before the current raw preflight, declaration-facts collection, resolver
session, target installation, module preparation, catalog installation, and
work-plan creation. I0 removes the source-backed old raw-preflight selection
edge; compatibility retains its existing owner. Remaining source-backed raw
catalog/post-install observers belong to the later R0 and cannot override the
parser-issued role meanwhile.

### Selected execution series

```text
S0  parser consumer-loan types + sole issuer + focused tests + guard + README
    production callers = 0; maximum lifetime = one landed commit

I0  move-only prepared-root consume at lifecycle entry
    source-backed loan caller = 1
    old source-backed raw-preflight selector = 0
    terminal Builder effects = 0
    fallback/retry = 0

R0  replace the remaining source-backed raw catalog/post-install observers
    and remove bool selection; compatibility/test raw owners remain explicit
```

S0 and I0 are one reserved replacement series. No unrelated task, proof row,
or consultation may land between them. If I0 cannot immediately consume the
S0 surface, S0 is reverted or the lane returns to `NoSafeSlice`; caller-zero
S0 is never called a completed I0.

### S0 acceptance and guard

```text
positive:
  App-only
  App + top-level sibling with exact [Sibling, RootMain] order
  empty Script
  nonempty Script with exact paired kind/statement order

negative/terminal:
  nonzero-arity Main stays Outside
  Main helper stays terminal
  foreign parser/final relation and transform drift stay rejected
  every top-level terminal variant maps exhaustively without callback

structural:
  root-loan issuer definition = 1
  root-loan production caller = 0 in S0, reserved to become 1 in I0
  root view program() / raw Main AST accessor = 0
  parser anchor / source site / ordinal / pointer accessor = 0
  Script-A type reference = 0
  loan/cursor/AfterLoan Clone = 0
  HRTB callback signature = 1
  production source files < 760 lines; 800 hard stop
```

Likely S0 ownership is one new
`normal_root_preservation/consumer_loan.rs` sibling, a thin final-source facade,
a dedicated physical test file, parser re-exports, the parser README, and one
reusable guard. `main_expansion.rs`, `program_root_work_plan.rs`, lifecycle
code, fixtures outside the focused parser tests, and all physical owners are
forbidden S0 edit targets.

### NoSafeSlice conditions

Return to `NoSafeSlice` if App siblings require a raw Program getter, the Main
statement must escape instead of becoming `RootMain`, names/ordinals/pointers
are needed outside the private issuer, rows and statements must be split and
re-paired, the callback can return a borrow, `AfterLoanV1` can reloan or expose
generic parts, a terminal reaches any lifecycle effect, S0 cannot be followed
immediately by I0, or S0 requires semantic edits to `main_expansion.rs` or
`program_root_work_plan.rs`.

## Non-claims

This card does not authorize:

```text
production lifecycle root consumption during S0
VerifiedMainExpansionV1 API changes
production lifecycle edits during S0
canonical semantics for generated test-harness rows
root/static-child/body lowering
ProgramRootWorkPlan bool removal
root_is_app_mode removal
MIR/ValueId/CFG/publication changes
compatibility retirement
fallback/retry
production switch
performance work
```

The selected next work item is caller-zero substrate
`NORMAL-ROOT-CONSUMER-LOAN-S0`. The previous root transport and G0-H1 rows
remain complete. S0 has a one-commit lifetime and reserves
`NORMAL-ROOT-CONSUMER-I0` as its mandatory immediate successor.
