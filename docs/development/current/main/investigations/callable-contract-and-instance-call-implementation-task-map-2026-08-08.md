---
Status: accepted revised task map; H1/R6-S0/R6-S1/R6-S2a/R6-S2/R6-S3A/R6-S3B-D0/R6-S3B-A/R6-S3B-B0/R6-S3B-B1/R6-S3B-B2/R6-S3B-B3-D0/R6-S3B-B3-I0/R6-S3B-C-D0/R6-S3B-C-S0/R6-S3B-C-S1/C-I0-D0/C-I0 implementation/R6-S3B-D0/D-I0/PARSER-PUBLIC-AST-POSTPASS-I0-A/PARSER-PUBLIC-AST-POSTPASS-I0-B closed; broad AST postpass I0-C remains parked
Date: 2026-08-08
Decision: current Hakorune authority wins over the external type-profile proposal
Reference: `docs/reference/language/callable-contracts.md`
---

# Callable contract and instance-call implementation task map

## Final design

```text
cloneable ordered Box method inventory
  + explicit non-Clone parser-owned source seal
  -> resolver declaration + semantic signature
  -> typed declared Query behavior
  + same-declaration VerifiedHomeAbi (sole Home authority)
  -> sealed declared callable catalog
  -> reusable instance target
  -> exact source-bound call relation
  -> Recipe CallSlot
  -> complete body conformance set
  -> publishable callable catalog
  -> physical ABI projection
  -> Lower

module publication
  consumes only the publishable catalog
  and does not decide callable semantics
```

## External-review closure checklist (2026-08-08)

The external review is reconciled into this task map. These are design
invariants, not permission to open the later resolver rows early:

```text
BoxMethodInventoryV1
  = cloneable selected/generated placement carrier

ParserBoxSourceSealV1
  = non-Clone parser-owned source seal
  = final rich parse product only
  (the rich handoff product is ParsedProgramWithSourceV1)

SourceBoxMethodSiteV1
  = as-written source/member identity and selected-gate path

BoxMethodInventoryOrdinalV1
  = placement only; never resolver identity

CallableContractSyntaxV1
  = typed syntax normalization before resolver

VerifiedHomeAbi
  = sole receiver/parameter/result Home authority
  = callable contract may reference, but never restates, its demands

VerifiedConformantCallableCatalogV1
  = complete same-brand declared-contract + body-conformance co-seal
```

Required ordering and stop lines:

1. Finish the parser-owned source seal and source-site/placement split before
   resolver declaration issuance.
2. Normalize raw rune attributes into typed syntax before the semantic issuer;
   resolver string matching is forbidden.
3. Retire or prove non-test caller-zero for
   `source_instance_result_contract` before opening the declaration-first
   instance target. Two target authorities may not coexist.
4. Build conformance as a complete Verify product. Lower and publication
   consume only the conformant catalog; publication does not re-decide meaning.
5. Keep semantic `I64` separate from `ExactTrivial*Abi`, `MirType`, and target
   ABI projections.

All five rows require focused positive/negative tests, owner README updates,
and the affected `docs/reference/**` receipt in the same implementation
commit. A missing issuer is `NoSafeSlice` development state, not a source
disposition.

## External authority audit reconciliation — 2026-08-09

The latest outside review was checked against the landed R6-S3B parser work
and this task map. It does not require a new architecture or an early resolver
implementation. The recommended corrections are already assigned to the
following existing owners and remain ordered, not silently folded into C-I0:

```text
parser-owned source seal between AST inventory and resolver
  -> R6-D0/S0-S3 + the closed R6-S3B-C-I0 parser-private receipt

as-written SourceBoxMethodSite != selected/generated inventory placement
  -> SourceBoxMethodSiteV1 / BoxMethodInventoryOrdinalV1 boundary

CallableContract(query) references, but never duplicates, VerifiedHomeAbi
  -> LANGUAGE-TYPED-CALLABLE-PROFILE-D0 / OWN-HOME-ABI0-S0/query

raw rune strings become typed syntax before the resolver issuer
  -> typed CallableContractSyntaxV1 in the language/parser row

old body-inferred source_instance_result_contract is dispositioned before
the declaration-first instance target opens
  -> SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-R0

declared contract and body conformance are separate; the complete conformant
catalog is the Verify owner
  -> CALLABLE-CONTRACT-CONFORMANCE-D0/I0 and publishable catalog co-seal

semantic I64 projects one-way to physical ABI; MirType/ExactTrivial*Abi do not
issue source meaning
  -> semantic-signature row followed by physical ABI projection
```

This audit is therefore recorded as **accepted architecture / parked
implementation**. No `Verified*` placeholder, test-only issuer, target
fallback, or second source authority is opened by the audit itself. Each
future implementation row must update its focused tests, owner README, and
language/reference receipt in the same commit.

The external review's architecture is accepted with these mandatory Hakorune
corrections:

```text
accepted source: CallableContract(query)
rejected source: CallableContract(exact_trivial_i64)

signature owns: arity and semantic parameter/result types
query owns: exact receiver direct-state reads and bounded no-effect behavior
Pure owns: no receiver/heap/global read
physical verifier owns: MIR representation and target ABI
```

Declaration and conformance are separate. An annotation declares an
obligation; it does not prove the body. Production publication requires both.

The Hako parser D0 audit adds four mandatory frontend corrections. First, reuse
the source-carrier lifecycle/sealer but add parser-invocation-branded
declaration refs/sites. Second, separate exact as-written method source sites
from selected/generated inventory placement. Raw `BoxMethodInventoryV1` stays
Clone-capable data; only the non-Clone parser seal may cross into resolver
semantics. Third, the seal must be issued only from one rich parse-output path
after build-gate prune/rebase and delegate lowering have completed; AST-only
APIs are projections that discard the seal. H1 is now closed as a disconnected
substrate; `CURRENT_STATE.toml` remains on the Rust R6 source-seal/final-parse
product correction. D-I0 is closed for the bounded ordinary-Box rich path;
broad public AST-only caller convergence is a separate BoxShape design stop
because the current rich finalizer rejects interface/static/record/mixed
cohorts and must preserve fuel, metadata, and explain-report contracts.
Resolver semantic publication is still closed. Fourth, the callable contract may reference
`VerifiedHomeAbi` but must not duplicate its receiver/parameter/result Home
demand; typed syntax normalization must happen before the resolver issuer.

## Single authority table

| Meaning | Sole owner | Forbidden reconstruction |
| --- | --- | --- |
| selected inventory order and lookup | AST-owned `BoxMethodInventoryV1` | source identity, resolver authority |
| source order, duplicate, exact method site | non-Clone parser-owned Box source seal | inventory ordinal, JSON, `HashMap`, resolver rescan |
| nominal Box/method identity and signature | resolver declaration inventory | method/Box name, Builder catalog |
| semantic parameter/result types | resolver semantic signature | `ExactTrivial*Abi`, `MirType`, source-string physical classifier |
| declared query behavior | typed Query behavior issuer | raw rune strings, body inference, `EffectMask` |
| receiver/parameter/result Home relation | `VerifiedHomeAbi` issuer | Query aggregate, lexical receiver presence, empty ABI |
| implementation compliance | body conformance verifier | annotation presence alone |
| complete declared/conformance coverage | publishable catalog co-seal | publication-time semantic recheck |
| semantic -> target ABI | physical ABI verifier | reverse `MirType` inference |
| reusable declaration target | resolver target catalog | call-site text, runtime registry |
| caller/receiver/arguments/result relation | source-bound call relation | Recipe or Lower re-resolution |
| logical call operation | verified Recipe `CallSlot` | provider/runtime fallback |

## Additional finite task slices from the review

These rows are deliberately placed before resolver target implementation and
do not change the active R6 implementation lane:

```text
R6-S3-PARSER-SOURCE-SEAL-D0
  final rich parse product and one non-Clone seal issuer
  build-gate prune/rebase + delegate postpass transport
  AST-only projection/discard path

R6-S3A-PARSER-RICH-PRODUCT-I0
  bounded direct top-level ordinary Rust Box path
  finalizer after existing prune/delegate AST postpass
  exact prepared-inventory prefix + generated delegate suffix placement probe
  no resolver-grade GeneratedDelegate relation claim in this row
  no partial seal for top-level gate/interface/static/record cohorts

R6-S3B-PARSER-SOURCE-AWARE-POSTPASS-D0/I0
  AST-only projection through the rich path
  typed top-level gate rebase and source-aware delegate transport

R6-S3B-D0
  one move-only ParserPostpassProductV1 handoff
  parser-issued structural source paths and sole postpass owner
  NoSafeSlice/Rejected/Unresolved/Declined/Candidate boundaries

R6-S3B-A
  canonical rich parse product for the ordinary direct-Box cohort
  AST-only APIs project from that product exactly once
  no resolver-grade generated-delegate suffix claim

  implementation receipt:
    OpenParserPostpassProductV1 owns AST + ParserSourceSessionV1 + diagnostic
    ParserMetadata; rich finalization and bounded AST projection
    consume the same product; gate structural paths and delegate relations
    remain closed for later S3B-B/C/D.

R6-S3B-B
  typed top-level build-gate path/cursor
  atomic branch selection, prune, and source-path preservation
  no post-prune ordinal reconstruction

R6-S3B-C-D0
  accepted design stop for one parser-private source-aware relation per
  expose, parser-time source transport, path-based private target lookup,
  all-host/expose preflight, and consume-return atomic AST/inventory/relation
  commit. Direct ordinary Rust Box targets only; generated-delegate chains,
  compatibility-only, interface/static/record/Hako/provider cohorts remain
  outside the bounded row. C does not widen the resolver-visible final seal;
  R6-S3B-D owns complete relation coverage and suffix-adapter retirement.

  Reference:
  `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-d0-design-task-2026-08-08.md`

R6-S3B-C
  source-aware delegate transaction
  GeneratedDelegateSourceRelation and generated-batch coverage; open only
  after C-D0 design guard is green

R6-S3B-C-S0 (closed)
  parser-time `DelegateSourceDeclarationV1` transport, one row per expose,
  selected member-gate source-path rebase, prepared postpass carriage, and
  compatibility-only rejection. No target lookup, generated placement,
  final-seal expansion, or resolver connection.
  Reference:
  `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-s0-implementation-task-2026-08-09.md`

R6-S3B-C-S1-D0 (accepted design; closed)
  private borrowed target index keyed by exact same-brand Box paths, with
  existing explicit method source relations as the only target authority.
  Field/type and method names are query selectors only; zero candidates are
  Unresolved, ambiguous paths are Rejected, and outside-cohort generated or
  chained targets are Declined. The C-S0 prepared transport remains the only
  landed C implementation; C-I0 batch commit stays closed.
  Reference:
  `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-s1-d0-design-task-2026-08-09.md`
  Planned implementation:
  `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-s1-implementation-task-2026-08-09.md`

R6-S3B-C-S1 (closed)
  private `DelegateTargetIndexV1<'product>` borrowing the unpublished
  postpass product; exact same-brand Box paths plus one explicit target method
  relation. No AST/inventory/final-seal mutation, generated placement, batch
  commit, resolver target, or runtime route.
  Task:
  `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-s1-implementation-task-2026-08-09.md`

R6-S3B-C-I0-D0 (accepted design; implementation closed)
  defines the private `PreparedDelegatePostpassBatchV1` owner, borrowed target
  signature view, all-host/expose exact preflight, staged placement receipts,
  relation persistence through finalization, typed failure/discard matrix, and
  one consume-return commit. It does not extend the final seal or issue a
  resolver target.
  Design:
  `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-d0-design-task-2026-08-09.md`

R6-S3B-C-I0 (closed implementation receipt)
  implements only the accepted parser-private atomic generated batch. The
  bounded batch performs all-host/expose preflight, stages inventory placement,
  persists parser-private relation rows through the prepared source payload,
  and applies one consume-return commit. Focused tests cover zero-delegate
  no-op, later-host failure without AST mutation, generated-name collision,
  staged-vs-actual placement mismatch, and duplicate source rows. No
  R6-S3B-D authority opened.
  Task:
  `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-implementation-task-2026-08-09.md`

R6-S3B-D (R6-S3B-D-D0 and D-I0 closed; broad AST cutover design active)
  final complete relation coverage and one final non-Clone seal
  sole `ParserBoxSourceSealV1` extension after complete relation coverage
  retire the bounded S3A generated-suffix adapter
  keep broad AST-only projection as an explicit compatibility nonclaim until
  the total postpass envelope is designed
  D0 design receipt:
  `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-d-d0-design-task-2026-08-09.md`
  implementation receipt:
  `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-d-i0-implementation-task-2026-08-09.md`
  D-I0 retains generated relation rows in the sole final seal after same-brand
  relation-key, provenance, and placement-receipt coverage; focused
  positive/negative tests and the D-I0 guard are landed. The three broad
  public AST-only callers remain compatibility nonclaims because the rich
  finalizer is ordinary-Box-only. Their total postpass envelope and caller
  cutover are the separate `PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0/I0` row.
  Resolver/runtime work remains closed.

R6-S3B-B0
  accepted design receipt for parser-issued gate paths and selection receipts (closed)

R6-S3B-B1
  parser gate-id/branch/child cursor and SourceBoxDeclarationPath transport (closed)

R6-S3B-B2
  parser-issued gate-ledger transport with explicit top-level scope and a
  distinct gate-path type, one typed selection receipt per gate, and atomic
  consume-return ParserSourceSession prune/rebase (closed)

R6-S3B-B3-D0
  accepted design stop for finalizer AST/source exact coverage. The next bounded
  implementation must preserve explicit/property seals, keep a valid
  generated-delegate suffix outside the resolver-visible source seal, and
  reject malformed/provenance-invalid suffixes. R6-S3B-C later issues
  GeneratedDelegateSourceRelation; no implementation is opened here.

R6-S3B-B3-I0 (closed)
  private FinalizerCoveragePlanV1 and source-path one-to-one finalizer
  alignment for the bounded ordinary Rust Box cohort; preserve the valid
  generated-delegate placement canary outside the final source seal; reject
  malformed/foreign/missing/duplicate coverage; same-slice focused tests,
  guard, reference, and parser-module owner documentation landed. The next
  boundary is R6-S3B-C design for a source-aware generated delegate relation.

R6-S3-SOURCE-SITE-PLACEMENT-I0
  SourceBoxMethodSiteV1 separate from selected/generated inventory ordinal
  remove parser sidecar/member-ordinal reconstruction

CALLABLE-CONTRACT-TYPED-SYNTAX-D0/I0
  RuneAttr -> CallableContractSyntaxV1::Query
  parser validates placement/duplicates; resolver consumes typed syntax only

CALLABLE-HOME-ABI-REFERENCE-COSEAL-D0/I0
  VerifiedHomeAbi remains the sole Home authority
  declared callable contract stores a same-declaration relation, not a second
  receiver/parameter/result demand

SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-R0
  non-test caller-zero or retirement of the old body-inferred target/result
  family before declaration-first instance target issuance
  Task:
  `docs/development/current/main/investigations/source-instance-result-contract-retire0-r0-task-2026-08-09.md`

CALLABLE-CONFORMANCE-CATALOG-COSEAL-D0/I0
  complete same-brand declared-contract + body-conformance set
  Lower/publication consumes only VerifiedConformantCallableCatalogV1

CALLABLE-SEMANTIC-PHYSICAL-TYPE-SPLIT-D0
  semantic I64 -> one-way physical ABI projection
  no ExactTrivial*Abi/MirType reverse inference

PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0/I0 (D0/S0/I0-A/I0-B closed; I0-C parked)
  one total typed postpass owner for `parse`, fuel/build-config parsing,
  metadata, and explain-report projections. The private result carries AST,
  metadata, optional explain, and typed per-Box coverage:
  `SourceSealedOrdinary` or `AstOnlyCompatibility`. Only the ordinary row may
  become resolver source authority; interface/static/record/mixed rows are
  successful AST-only compatibility and never fake seals. A shared full
  BuildGate decision set must feed prune, explain, and top-level source-path
  rebase. Preserve fuel, metadata, and explain behavior; no reparse, AST/name
  rescan, catch-and-fallback, retry, ordinal identity reconstruction, or old
  whole-root helper hidden behind the new owner.

  Ordered rows:
    PARSER-PUBLIC-AST-POSTPASS-S0
      private envelope/cohort admission/caller census; no public switch
    PARSER-PUBLIC-AST-POSTPASS-I0-A
      string/build-config wrapper family with fuel/AST/diagnostic parity
    PARSER-PUBLIC-AST-POSTPASS-I0-B
      `NyashParser::parse` and metadata projection, no re-tokenization
    PARSER-PUBLIC-AST-POSTPASS-I0-C
      full BuildGate decision set and explain parity
    PARSER-PUBLIC-AST-POSTPASS-FINAL
      old whole-root delegate caller-zero and compatibility quarantine

  Task:
  `docs/development/current/main/investigations/parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md`
  S0 implementation receipt:
  `docs/development/current/main/investigations/parser-public-ast-postpass-s0-implementation-task-2026-08-09.md`
  S0 receipt:
  private total envelope/cohort coordinator is landed; public callers remain
  unchanged before I0-A. I0-A receipt: the string/build-config edge now
  enters `string_postpass_entry` once, preserves fuel/AST/diagnostic behavior,
  and retires its delegate-only production edge. I0-B receipt:
  `NyashParser::parse` and the metadata wrapper share one parser-private
  postpass finalizer and one consuming `into_ast_and_metadata()` projection;
  metadata is moved from the completed product exactly once. I0-C remains
  parked for the full BuildGate decision set and explain parity.

PARSER-MEMBER-GATE-NESTED-SOURCE-PATH-D0 (parked baseline debt)
  The existing nested selected-else source-path fixture fails on parent
  `72b3471e55` before postpass opening because its member-level gate branches
  have different public signatures. This is not an I0-A regression. Decide
  fixture repair versus a language-rule change in a separate design row;
  do not weaken the signature validator or add postpass fallback.
  Task:
  `docs/development/current/main/investigations/parser-member-gate-nested-source-path-d0-task-2026-08-09.md`

POST-CUTOVER-COMPAT-API-QUARANTINE-D0 (parked)
  after parser/Builder caller-zero and legacy retirement, move remaining
  inventory compatibility conversions and historical name-order views behind
  an explicit compat module; canonical `BoxMethodInventoryV1` must not expose
  compatibility constructors as ordinary authority. Keep JSON v1 and other
  bootstrap adapters descriptive-only, with a named removal condition for
  each retained consumer.

POST-CUTOVER-DOC-CURRENT-HISTORY-CLEANUP-D0 (parked)
  keep one live status line in each owner README and move landed R1--R6
  receipts into clearly labeled historical sections; reconcile
  `CURRENT_TASK.md`, `CURRENT_STATE.toml`, task-map status, and reference
  receipts at each row close. This is documentation hygiene only and must not
  open a second semantic or implementation authority.
```

The parser rows are R6-S3 design/implementation boundaries; R6-S3A,
R6-S3B-D0, S3B-A, S3B-B0, S3B-B1, and S3B-B2 are closed. S3B-B3-D0 is the
current design stop after parser gate-ledger transport, scope/path distinction,
selection receipts, and atomic prune/rebase; no implementation is open there. The
latter five callable rows remain resolver/callable rows. Do not create a parallel
implementation lane for them.

External review reconciliation (2026-08-08): no new parallel task is needed.
The parser source seal, source-site/placement split, typed callable syntax,
`VerifiedHomeAbi` sole authority, old instance-result retirement,
conformance-catalog plan, and semantic-I64/physical-ABI split remain ordered
in this task map. R6-S3B-B3-I0 is closed; delegate relation work remains the
separate R6-S3B-C design boundary.

`NoSafeSlice` means a required issuer is not implemented. It is not a source
disposition. After an issuer exists, disposition is:

```text
Rejected > Unresolved > Declined > Candidate
```

## Ordered finite ladder

### A. Landed frontend inventory

1. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R1` — closed
   - replace the AST `HashMap` field;
   - compile compatibility consumers through explicit `CompatibilityOnly`;
   - no source-authority claim.
2. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R2` — closed
   - shared pending/direct issuance substrate;
   - interface/static parser issuance and duplicate/site proof;
   - build_cfg transforms declarations without losing metadata;
   - ordinary source authority remains zero.
3. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R3` — closed
   - ordinary Box sole-inventory cutover;
   - selected build-gate, generated property, and delegate atomic batches;
   - generated rows stay non-source provenance.
4. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R4` — closed
   - ordered JSON v2;
   - legacy JSON v1 imports only `CompatibilityOnly`;
   - strict recursive mode rejects malformed nested v2 without fallback.
5. `FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R5` — closed
   - R5-S1 closed: deferred non-Main static-Box Program edge now consumes the
     ordered inventory directly and retains only an explicit compatibility
     name-order projection;
   - R5-S2 closed: connected static-`Main` compatibility child ports now carry
     the ordered inventory directly; the compatibility leaf retains only its
     explicit historical name-order projection and nested-`Main` rejection;
   - R5-S3 closed: production Builder map caller-zero was proven; retained
     name-order owners are explicit compatibility views, while runtime and
     legacy JSON projections remain outside R5.

### B. Exact parser source authority and parity

6. `HAKO-PARSER-BOX-DECLARATION-CARRIER-H1` — closed
   - prove only the disconnected branded refs/sites, exact member paths,
     separate inventory ordinals, ordered drafts, duplicate-without-mutation,
     one-Box seal, foreign-brand/site negatives, and double-finish rejection;
   - focused H1 guard and the existing P0 guard are green; all touched Hako
     sources remain below 800 lines;
   - no parser connection, build-gate, delegate postpass, scanner, resolver,
     semantic publication, or compatibility projection.
7. `FRONTEND-PARSED-BOX-SOURCE-SEAL-R6-D0/S0/S1/S2A/S2/S3A` — R6-D0
   accepted; S0, S1, S2a, and S2 closed; S3A was the bounded predecessor and
   D-I0 is the closed final-seal slice; broad public AST postpass design is next
   - give explicit methods an exact branded source site independent of the
     all-row inventory ordinal;
   - generated property/delegate rows retain generated origin only and never
     receive an explicit method source site;
   - define one parser invocation/source-authority session owning the fresh
     brand, source sites, unpublished inventory, and prepared seal payload;
   - carry that payload through the bounded finalizer after the existing
     prune/delegate AST postpass, then issue one non-Clone parser source seal
     only for the final ordinary-Box AST/inventory product;
   - later S3B will make AST-only parser APIs project the same rich parse path
     and discard the seal; no second scanner/rescan and no `ParserMetadata`
     authority;
   - only after the above contract is accepted, atomically publish the
     inventory plus seal and delete `method_source_member_ordinals`,
     length-delta reconstruction, the parallel gate-merge slice, and raw
     delegate ordinal sidecars in one transaction cutover series;
   - R4 JSON remains descriptive and R5 Builder receives inventory only.
   - S0 closed with the descriptive `BoxMethodInventoryOrdinalV1` rename and
     preserved JSON wire spelling; S1 closed with parser-private
     brand/source-site/transaction/prepared-seal types and a non-Clone final
     seal type with no constructor; S2a closed with parser-session ingress:
     one invocation brand and exact top-level statement cursor. S2 now owns
     the ordinary direct/property/member-gate producer cutover and method
     sidecar retirement; S3A now validates the AST-only delegate suffix after
     the postpass, while source-aware delegate relation transport remains an
     explicit S3B nonclaim.

   R6-S2 was a behavior-neutral Refactor Series with three bounded cells; all
   three are now landed:

   ```text
   R6-S2b-AST-receipts
     generated batch commit returns placement receipts;
     selected-gate merge consumes transaction-owned relation lookup/rebase;
     replace the AST parallel &[u32] merge with a typed prepared append/rebase
     boundary before claiming sidecar retirement

   R6-S2-transaction-cutover
     BoxMemberState's unpublished inventory, source relations, and member
     cursor have one transaction owner; direct/property/once/birth_once and
     member-gate producers use that owner

   R6-S2-sidecar-retirement
     delete method_source_member_ordinals, record_new_methods_since, and
     length-delta reconstruction after parser caller-zero tests are green
   ```

   The typed AST bridge is a prerequisite, not an optional cleanup:

   ```text
   source transaction:
     consume branch inventory and its typed method-row relations
     prepend the selected-gate path and rebase source rows
     prepare one complete append

   AST inventory:
     validate duplicate/name/declaration identity and contiguous placement
     commit the prepared append atomically
   ```

   A suitable shape is
   `PreparedBoxMethodInventoryAppendV1` with
   `BoxMethodInventoryV1::prepare_append` / `commit_prepared_append`.
   The AST crate must not know parser brands or source-site authority. The
   former `try_merge_selected_gate(selected, &[u32], gate_site)` was removed,
   not renamed into a new sidecar. Until this bridge was fixed, the S2 blocker
   was `NoSafeSlice`; it never authorized another parallel ordinal ledger.

   The transaction-side owner surface is bounded to
   `open_for_box`, `branch`, current member/gate site, explicit commit,
   generated-property batch commit, prepared gate merge, and final `finish`.
   `BoxMemberState` keeps non-method metadata and delegates, while one
   `source_tx` owns the unpublished inventory, member cursor, and typed source
   relations. No parser producer receives `&mut BoxMethodInventoryV1` after
   the cutover.

   R6-S2b AST receipt support and the parser transaction cutover are landed in
   the frontend AST/parser crates. The prepared append validates the complete
   unpublished batch before mutation, returns placement receipts, and exposes
   gate-path rebasing without making the AST crate a parser source authority.
   The former parallel gate merge and all ordinary-parser sidecars are
   removed; the live path accepts only the typed transaction-owned relation
   and append products.

   The latest external review is reconciled into this row, not opened as a
   second authority. `BoxMethodInventoryV1` remains cloneable selected/
   generated placement data; only the non-Clone parser source seal is
   resolver-grade. Inventory ordinals may index a transaction-private
   relation table, but never become declaration identity. Generated rows have
   generated origin and placement receipts only; explicit source sites are
   independent of all-row placement.

   These are explicit R6-S3B or later nonclaims: AST-only projection cutover,
   top-level build-gate rebase, source-aware delegate relation transport, raw
   `DelegateDecl` ordinal retirement, typed `CallableContractSyntaxV1` parser
   acceptance, resolver declaration/target issuance, Recipe/Builder/MIR/
   provider/runtime connection. S3A's final parser seal is issued only by its
   bounded rich product after the existing prune/delegate postpass. If current
   parser inputs cannot issue a required relation, stop at `NoSafeSlice`; do
   not infer source identity from names, ordinals, JSON, or sidecar deltas.

8. `HAKO-PARSER-BOX-DECLARATION-CARRIER-H2/H3`
   - issue the same ordered inventory and non-Clone parser seal while
     parsing each Box and body once; no body slice or scanner rescan;
   - consume the R6 rich parse product; whole-program delegate expansion must
     happen before final seal or consume and return the branded product;
     detached sealed inventory mutation is forbidden;
   - if one-pass body `ParserNodeProductV1` is unavailable, stop at
     `NoSafeSlice` rather than adding a fallback.
9. `HAKO-PARSER-BOX-DECLARATION-CARRIER-H4`
   - own the sole selected build-gate transaction: parse branch states,
     compare surface signatures, select once, rebase ordinals, then commit;
   - if canonical Hako build-config evaluation is unavailable, stop at
     `NoSafeSlice` rather than inventing one in the parser.
10. `HAKO-PARSER-BOX-DECLARATION-CARRIER-H5/H6`
   - H5 normalized Rust/Hako parity is test evidence, never semantic
     transport;
   - H6 issues typed `CallableContractSyntaxV1::Query` carriage with exact
     site/provenance; resolver raw-string matching is zero.

### C. Resolver declaration and declared contract

11. `RESOLVER-INSTANCE-METHOD-DECLARATION-AND-SEMANTIC-SIGNATURE-I0`
   - consume only the parser-owned source seal;
   - exact nominal Box/method identity, catalog brand, and resolved semantic
     parameter/result types;
   - semantic `I64` is not `ExactTrivial*Abi`, `MirType`, or a source-string
     physical classifier;
   - no behavioral contract, Home ABI, physical ABI, or target.
12. `OWN-HOME-CALLABLE-ABI-D0` -> `OWN-HOME-RELATION0-S0` ->
   `OWN-HOME-ABI0-S0/query`
   - existing Home taskboard remains the sole owner;
   - issue one same-declaration `VerifiedHomeAbi` with receiver `Handle`, zero
     parameter demands, and Trivial result for the bounded query cohort;
   - no query aggregate may restate or fabricate this axis.
13. `RESOLVER-DECLARED-QUERY-BEHAVIOR-I0`
    - consume only typed `CallableContractSyntaxV1::Query`;
    - issue the behavioral read/effect/control obligation;
    - no Home or physical ABI issuance.
14. `RESOLVER-DECLARED-QUERY-INSTANCE-CONTRACT-I0`
    - co-seal declaration, semantic signature, Query behavior, and the exact
      same-declaration `VerifiedHomeAbi`;
    - publish one sealed declared callable catalog usable for recursive
      target resolution and body verification;
    - bounded positive fixture `length(): i64`; neither name nor physical ABI
      is semantic authority.

### D. Target and source-bound logical call

15. `SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-R0`
    - delete the caller-zero body-inferred result/target/rebind/preloop family
      before adding the new declaration-first target;
    - preserve only general `source_call_target` source-site primitives and
      unrelated production result-representation owners;
    - require non-test caller zero and same-slice module/README/ledger guard.
    - Task:
      `docs/development/current/main/investigations/source-instance-result-contract-retire0-r0-task-2026-08-09.md`
16. `LOOP-RESOLVER-INSTANCE-CALL-TARGET-I0`
    - catalog-owned reusable opaque target reference;
    - existing FreeStatic index unchanged;
    - no call-site or Recipe facts.
17. `LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-D0/I0`
    - exact caller/receiver/argument/result sites and exact target;
    - caller-zero logical product;
    - no Builder or physical call.
18. `LOOP-RECIPE-CALLSLOT-COSEAL-I0`
    - deterministic source relation to existing typed Recipe `CallSlot`;
    - full source evidence and verifier coverage;
    - no provider selection or fallback.

### E. Body conformance and activation

19. `CALLABLE-CONTRACT-CONFORMANCE-D0/I0`
    - verify direct receiver-read footprint, no writes/Home escape/allocation/
      IO/FFI/failure escape/suspension/non-local control;
    - publish one complete same-brand conformance set;
    - never infer a replacement public contract from the body.
20. `CALLABLE-PUBLISHABLE-CATALOG-COSEAL-I0`
    - co-seal the declared catalog with exactly one accepted conformance per
      body-bearing declaration;
    - reject missing, duplicate, foreign, or rejected conformance;
    - issue one publishable catalog; publication performs no semantic recheck.
21. `CALLABLE-PHYSICAL-ABI-PROJECTION-D0/I0`
    - project semantic signature plus target capability one way into physical
      ABI and `FunctionSignature`;
    - no `MirType`/physical ABI reverse inference into resolver semantics.
22. Named production activation
    - one selected caller switches to the verified route;
    - delete that caller's old lookup/retry/fallback in the same commit.

## Legacy retirement ledger

| Legacy surface | Keep through | Delete when |
| --- | --- | --- |
| AST compatibility method map | R1-R4 | R5 caller zero and JSON v2 parity |
| name-sorted compatibility iteration | legacy JSON/Builder consumers | those named consumers migrate |
| parallel `method_source_member_ordinals` sidecar | R6-S2 ordinary transaction cutover | deleted from the ordinary parser; bounded final rich source seal is R6-S3A |
| inventory `selected_method_ordinal` as source identity | R6 | retain only as selected inventory placement |
| Builder same-module name/arity catalog | current production compatibility | resolver target is selected and its caller cut over |
| `source_instance_result_contract` body-inferred target/result family | current caller-zero tests | `SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-R0`, before new target I0 |
| FreeStatic resolved index | indefinitely for FreeStatic only | never reused for instance methods |
| `Contract(pure|readonly)` metadata | existing metadata lane | a separate Decision explicitly migrates it |
| test-only normalized Rust/Hako comparison | parity tests | may remain as evidence; never runtime authority |

No compatibility row may be promoted to `ExplicitSource`; no resolver may
recover source order/site from a compatibility map.

## Mandatory tests and documentation

Every implementation row closes in one implementation-coupled commit with:

```text
implementation
focused positive/negative tests
owner module README
affected docs/reference receipt
active card closeout and next pointer
all touched source files < 800 lines
```

Required test families are:

```text
frontend:
  order, direct/selected duplicate, exact source site vs inventory ordinal,
  generated rows without method source sites, parser-seal non-forgeability,
  provenance, JSON v1/v2 descriptive-only boundary

resolver contract:
  typed Query row, semantic signature, VerifiedHomeAbi same-brand co-seal,
  Candidate/Declined/Unresolved/Rejected, foreign brand, conflict, partial
  aggregate unconstructible

target/relation:
  reusable target, exact receiver/arity/types, foreign caller/site,
  no FreeStatic/name fallback

conformance/publication:
  direct receiver read accepted for query, writes/effects/control rejected,
  complete declared/conformance coverage, declaration without conformance
  cannot issue the publishable catalog
```

Reference updates are not deferred to a final cleanup row. The exact landed
surface updates `docs/reference/language/callable-contracts.md` and its owning
module README in the same commit. Future rows remain described as future until
their issuer and negative matrix land.

## Global stop lines

```text
no exact_trivial_i64 source profile
no receiver-read Pure widening
no source order/site recovery from HashMap or names
no source method identity from selected/generated inventory ordinal
no parser source authority from JSON or Cloneable AST data alone
no resolver CallableContract parsing from raw rune strings
no Query-owned duplicate receiver Home axis
no resolver semantic type from ExactTrivial ABI or MirType
no public partial semantic-receipt constructors
no declaration annotation treated as body proof
no target from method/Box name
no instance fallback to FreeStatic
no old body-inferred instance target beside the new declaration target
no Recipe before the exact source-bound call relation
no production publication before publishable-catalog co-seal
no Builder/provider/runtime retry or fallback
```
