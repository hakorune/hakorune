# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D2

Status: closed__2026-09-25__NoSafeSlice
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-EXACT-USIZE-S0 (landed)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  post-S0 terminal map on the exact-usize S0 card.

## Slice

Select ONE bounded failure class from the remaining 7 red entries of
`real-apps-exe-boundary` (post-usize-S0 receipt, 4 pass / 7 fail)
with a complete authority/caller/delete/evidence tuple, and emit one
bounded execution card. No implementation in this mode.

## Fresh class map (post-exact-usize-S0)

| candidate family | entries | known owner hints |
|---|---|---|
| `main-import-view/selected-header-missing` | json_stream_aggregator, boxtorrent_mini | qualified main-import ABI; json_stream needs non-i64 result — boxtorrent terminal newly reached here after usize admission; likely same owner edge |
| `ordinary-new/birth-global-legacy-stopped` | binary_trees, mimalloc_lite | ordinary-new claim authority for `Class.birth/N`; binary_trees also has return-position new + null-arg gaps (per D1 audit); mimalloc newly reached here after usize admission |
| `NamedArray(TextSourceMissing)` | allocator_stress | computed-handle vs admitted text-source; semantic boundary decision |
| `return_type_strategy` panic rc=101 | untyped_field_min | untyped/inferred storage — ParkedSealed per MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0; sentinel only |
| `no_lowering_variant` + stale `opt` | newbox_min | env/toolchain debt (opt=LLVM14, opt-18 exists); not semantic work |

## Questions to answer

1. Which single class has a complete tuple (source authority +
   canonical issuer + fail-fast boundary + smallest slice +
   non-claims)?
2. For the `main-import-view` pair: do json_stream and boxtorrent
   share one owner edge (qualified-import selected-header/ABI), or
   does boxtorrent stop for a different reason under the same label?
3. For the `birth-global-legacy-stopped` pair: is mimalloc's
   `Class.birth/N` shape identical to binary_trees' (return-position
   new + null args), or a narrower subset that admits a bounded
   slice alone?
4. Do any of the moved terminals hide a *different* family behind
   the same named stop (masking)?

## Boundary

- Includes: class inventory re-verification, owner audit, tuple
  assembly, next S-card emission.
- Excludes: implementation, grammar/ABI changes, fallback.

## Exit

- [ ] One accepted Decision with complete tuple.
- [ ] One bounded S-card emitted; pointers synced.

## Decision (closed 2026-09-25): NoSafeSlice — open the main-import-view result-ABI D0

Worker `63b3169c` census + main-investigator spot-checks
(`physical_header.rs:113-115`, `source_call_publication.rs:14-17`,
app call sites verified):

```text
Decision: NoSafeSlice — no remaining class satisfies the full
          (authority, issuer, caller, boundary, bounded slice) tuple.
          Open a design card for the highest-information family:
          main-import-view non-i64 (ExactString) qualified-result
          authority, covering json_stream + boxtorrent on one edge.
Source authority + canonical issuer: TBD — candidate exists
          (VerifiedSameModuleCallableResultCatalogV1 ExactString/
          ExactBool dispositions + static_call_result_publication_owner
          already select those handoffs); the D-card must NAME it.
Non-authority: MIR-shape guessing, receiver-name heuristics,
          reachability narrowing.
Fail-fast boundary: selected-header-missing / not-exact-i64 /
          static-publication-representation stay named.
Smallest next slice: the D0 card itself, then one bounded S-card.
Non-claims: no EXE green claim; downstream store-RHS/argument-shape
          stops are out of scope.
```

### Census answers

1. **Complete-tuple class**: none. Every remaining family needs a
   named authority expansion (BoxCount) before an S-card.
2. **main-import-view pair = same edge**: both apps call
   `Main.<helper>/0` (unannotated -> String result) via
   QualifiedUnbound `Main.` receiver -> DirectCanonicalOwner.
   json_stream `main.hako:172` `agg.ingest(Main.sampleStream())`;
   boxtorrent `main.hako:234` `local source = Main.samplePayload()`.
   Both die at model.rs:312-318 second arm because
   `physical_header.rs:113-115` skips contracts whose result is
   `None` (unannotated/non-i64). Downstream gates that would also
   need widening: `model.rs:331` `not-exact-i64`, and
   `source_call_publication.rs:14-17,55-59` (ExactI64|ExactBool
   only).
3. **birth-global-legacy-stopped pair = NOT identical**:
   binary_trees first stop is `me.builder = new BinaryTreeBuilder()`
   (store-RHS inside birth) then return-position `new` + null args.
   mimalloc_lite first stop is `new HakoAllocHeap()` inside
   `MiWorkload.run` — imported box whose stored-field initializers
   (`page_heap_box.hako:185-194`) desugar into birth-prologue
   store-RHS `new` with qualified-call args, plus
   `FieldContractUnsupported` construction. Shared structural gap:
   claims cover only `[Body, Initializer]` local-destination sites
   (`coseal_issue.rs:104-117`) with `Integer|Bool|Local` args
   (`ordinary_new_arguments.rs:9-13`) — destination/argument-kind
   expansion is BoxCount.
4. **Masking check**: `NamedArray(TextSourceMissing)` is a separate
   text-append-only contract lane (`TextRetainedByReceiver`);
   allocator_stress pushes a computed handle inside a loop — needs
   a new relation kind + ABI variant. `newbox_min` = toolchain debt
   (opt=LLVM14 before opt-18 in `physical_options.inc:156-157`,
   `NYASH_NY_LLVM_OPT_TOOL` override exists) + residual
   `no_lowering_variant` pure-lane gap. `untyped_field` stays
   ParkedSealed sentinel.

### Ranking

| family | entries | verdict |
|---|---|---|
| main-import-view String result | 2 | NeedsDesign — one edge, downstream result-representation authority exists; D-card names it |
| ordinary-new claim destination/args | 2 | NeedsDesign — claim model requires local BindingRefV1 destination + 3 arg kinds |
| NamedArray handle push | 1 | NeedsDesign — new relation + ABI variant |
| newbox toolchain | 1 | EnvironmentDebt (opt-18 pin first) |
| untyped_field panic | 1 | ParkedSealed sentinel |

Next card: MIRBUILDER-EXE-ACCEPTANCE-MAIN-IMPORT-STRING-RESULT-D0.

## Exit

- [x] One accepted Decision — NoSafeSlice with ranked family map.
- [x] Next design card emitted; pointers synced.
