---
Status: accepted revised task map; H1/R6-S0/R6-S1/R6-S2a/R6-S2/R6-S3A closed, R6-S3B design stop
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

VerifiedParsedBoxDeclarationV1
  = non-Clone parser-owned source seal
  = final rich parse product only

SourceBoxMethodSiteV1
  = as-written source/member identity and selected-gate path

BoxMethodInventoryOrdinalV1
  = placement only; never resolver identity

CallableContractSyntaxViewV1
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
substrate; `CURRENT_STATE.toml` is deliberately design-stopped at the Rust R6
source-seal/final-parse-product correction before any parser connection or
resolver semantic publication. Fourth, the callable contract may reference
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
  exact prepared-inventory prefix + generated delegate suffix validation
  no partial seal for top-level gate/interface/static/record cohorts

R6-S3B-PARSER-SOURCE-AWARE-POSTPASS-D0/I0
  AST-only projection through the rich path
  typed top-level gate rebase and source-aware delegate transport

R6-S3-SOURCE-SITE-PLACEMENT-I0
  SourceBoxMethodSiteV1 separate from selected/generated inventory ordinal
  remove parser sidecar/member-ordinal reconstruction

CALLABLE-CONTRACT-TYPED-SYNTAX-D0/I0
  RuneAttr -> CallableContractSyntaxViewV1::Query
  parser validates placement/duplicates; resolver consumes typed syntax only

CALLABLE-HOME-ABI-REFERENCE-COSEAL-D0/I0
  VerifiedHomeAbi remains the sole Home authority
  declared callable contract stores a same-declaration relation, not a second
  receiver/parameter/result demand

SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-R0
  non-test caller-zero or retirement of the old body-inferred target/result
  family before declaration-first instance target issuance

CALLABLE-CONFORMANCE-CATALOG-COSEAL-D0/I0
  complete same-brand declared-contract + body-conformance set
  Lower/publication consumes only VerifiedConformantCallableCatalogV1

CALLABLE-SEMANTIC-PHYSICAL-TYPE-SPLIT-D0
  semantic I64 -> one-way physical ABI projection
  no ExactTrivial*Abi/MirType reverse inference
```

The parser rows are R6-S3 design/implementation boundaries; R6-S3A is closed
as the bounded rich-product slice and R6-S3B is the current design stop. The
latter five remain resolver/callable rows. Do not create a parallel
implementation lane for them.

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
   accepted; S0, S1, S2a, and S2 closed; S3A is the current bounded slice
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
