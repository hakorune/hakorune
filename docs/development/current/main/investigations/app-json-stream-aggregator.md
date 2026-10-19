# APP-json-stream-aggregator

Status: common untyped-storage design decision first; JSON remains the selected app.
Date: 2026-10-19
Scope: unchanged JSON app, existing Loop/Text owners, canonical physical EXE.
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md),
[final pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Current measurement

- Fixed `real-apps-exe-boundary`: 7PASS/4FAIL, actual EXE6 + empty marker1;
  mimalloc and allocator PASS, no SKIP/timeout, duration783.522s.
  Measured literal-source admission `23ff510a8b` with its matching quick CLI,
  explicitly selected lifecycle runtime and production Bool-equality C FFI.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900`, build/run limits300s.
- JSON first stop: `[freeze:contract][source-loop/v2/source Loop has an unavailable path]`.
  MIR compilation fails before artifact generation; no link or runtime evidence.
  Original owner is `ingest`, Loop Body(2), source line145; first uncovered statement
  is line151 `me.ingestLine(stream.substring(start, end))`. No artifact is generated.
  On the current source-admission tree, incoming and Mul gates are cleared; debug
  reports `source expression has no checked execution root`, path=None, in this Loop.
  The scanner also lacks a Call action for line146 `local end = JsonLine.find(...)`;
  retain its real I64 result as well as the later discarded Bool result.
- Other reds remain explicit: untyped-field phase84 inference panic,
  binary-trees borrowed-result/source-not-i64, boxtorrent stored-child/receiver-unavailable.

## Current priority and measurement interpretation

- [Common storage D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md) is now the
  immediate semantic decision for three real apps plus the untyped sentinel.
  Tagged dynamic preserves existing EBNF; initial state/ownership/replacement/reads still
  need a complete accepted contract. No dynamic storage implementation is authorized yet.
- `mimalloc-lite` suite PASS includes a harness correction: an obsolete generic JSON
  all-Invoke refusal precheck was removed; production compiler/app behavior was unchanged.
  The canonical EXE already worked. Route/toolchain/artifact/no-replay/output/exit checks remain.
- Existing publication tests cover generic profile rejection of four object-lifecycle
  programs and positive Static I64 Invoke transport. They do not reproduce the same
  mimalloc import closure or assert every unsupported Invoke subtype. Do not describe
  this as a new runtime capability or an identical negative test transfer.
- Own unverified Loop Call source edits are saved in Git stash under
  `JSON Loop Call source connection pending common storage decision`; not built/tested.

## Next source connection

- Read-only worker recommends JSON over binary-trees: binary's `itemCheck` immediately
  reads untyped left/right/value; inferring their storage from birth writes would cross D0.
- The first uncovered path is the Bool-result statement call above, not the earlier
  `find`/`intField` loops. `ingestLine` returns original Bool literals false/false/true.
- Existing debug mode now retains owner/function/site and scalar incoming context.
  Refusal selection and the default-off debug setting are unchanged.
- Instance actual preparation and independent revalidation are separated behind existing
  APIs; Static/Instance Core argument checks share one source owner.
- Loop physical effects now borrow the SAME existing child group/Append packet from the
  exact parent, preserving source/continuation/finished checks without a separate Call map.
  `source_loop`13PASS/1FAIL matches the saved pre-change binary; source walk1 and Array
  lifetime5 PASS. The existing `count_at` source-Loop failure stays open.
- Uniform original I64/Bool literal Return census now survives parameterized source
  qualification and independent final result checking. Existing borrowed-use family is
  preserved; no input, Completion or Invoke ABI permission is inferred from that census.
- Core Text actuals reuse the SAME argument receipt/frame through Instance preparation,
  reconstruction and final validation. Complete literal Returns prove no Mul product;
  missing use drafts alone still cannot prove that absence.
- Focused Static22/Instance4/pending7 PASS; composition9PASS/2FAIL and result15PASS/2FAIL
  match saved pre-change failures. The existing Unit-value diagnostic expectation also
  fails with the pre-admission CLI; its refusal remains terminal, not a successful value.
- Existing case adds I64/Bool Return-census loss checks and substring actuals because
  empty field-plan coverage cannot detect a lost literal Return. Test functions remain9263;
  no new test file/hook/checker/guard/token. Read-only worker reviewed same-source boundaries.
- Reuse `home_prefix_source_loop*` source header/control/expression inventory,
  `produce_source_loop_recipe_v2`, and existing execution/schedule and expression writers.
  Landed Text, Bool, BranchNormal and argument owners supply existing contracts.
- Next connect both Local and Discard as Call actions in the existing source Loop tree,
  schedule, physical Static/Instance writers and final child-packet verification. Its substring
  argument belongs inside that SAME Call packet; do not turn it into an expression root.
  Preserve Normal/Fault argument cleanup and original Homes; no backend inference or app edit.
- JSON's later untyped UserStats/aggregator slots and Map remain explicit dependencies.
  This selection does not accept dynamic-slot semantics or promise a D0-free whole-app PASS.

## Retained verification obligations

- Allocator and mimalloc physical EXEs pass unchanged. Allocator's Bool Eq/Ne C admission
  retains mixed Bool/I64 and Bool-order refusals; existing matrix tail is still blocked
  by the pre-existing tagged-add negative. Earlier SSA/handoff/Array teardown/synthetic
  callee reds remain open; implementation details are retained in this file's Git history.
- No app workaround, silent retry, new checker/test file or runtime adapter was introduced.

## Completion obligations

- Existing JSON smoke must satisfy every artifact/run/output/negative contract unchanged.
- Common untyped-slot D0 is ReopenedDesign; implementation_permission=false until full acceptance.
- Retire replaced exclusive edges when caller/validation closure holds; retain shared owners.
- Fixed acceptance, final pipeline switching/retirement and the full goal remain incomplete.
