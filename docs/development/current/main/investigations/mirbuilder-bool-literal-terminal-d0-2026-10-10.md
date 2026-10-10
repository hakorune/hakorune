# Bool literal terminal result D0

Status: decision accepted; BOOL-LITERAL-TERMINAL-S0 selected
Date: 2026-10-10
Scope: MIRBUILDER-BOOL-LITERAL-TERMINAL-D0 decision and selected MIRBUILDER-BOOL-LITERAL-TERMINAL-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-i64-eq-s0-2026-10-10.md
  - src/mir/resolved_semantics/home_new_prefix_terminal.rs
  - src/mir/normal_callable_semantic_package/result_contract.rs

## Trigger and boundary

The unchanged `SizeClassBox.accepts(size)` has two Bool literal exits:
`return false` inside the If and `return true` after it. A focused
published-view probe with the original `size_class_box.hako` and a trivial
Main reached `SizeClassBox.accepts/1` and stopped at
`[ordinary-new/local-commit/root-exit-source-missing]`. A temporary
diagnostic identified the function and was removed. The test is uncommitted
and red; Eq S0 remains unfinished.

The Home terminal scanner recognizes Bool literals as trivial, but its
`Some(_) => true` arm retains no `TerminalRelationV1`. The child callable
retention and `normal_exit_projection_v1` therefore have no exact exit row.
`root_home` correctly refuses missing source.

The existing `callable_result_representation` solver already issues
`VerifiedCallableResultDispositionV1::ExactBool` for uniform Bool exits and
rejects mixed representations; its `bool_results` tests exercise that rule.
The ordinary package retains those solver rows through
`QualifiedStaticCallClaimIndexV1::result_for_key`. This is the callable result
meaning authority, not a physical ABI. `ExactTrivialScalarAbiV1` is currently
the exact *declared* i64 spelling substrate; extending its declaration
classifier to call an unannotated Bool result declared would manufacture
authority. The physical ordinary role and selected-C call reader currently
admit i64/map/handle/nullable-handle, not Bool. Also, the original
`accepts_usize` returns `me.accepts(size)`, so Bool call transport is a
distinct later frontier. A relation-only patch cannot claim whole-source
physical publication.

## Decision

```text
Decision: issue the two exact Bool-literal terminal relations before any
  physical Bool result/call admission. Keep result class, terminal site and
  physical representation as distinct evidence joined at their owners.
Source authority + canonical issuer: resolved Bool literal at the exact
  Return value site and verified Completion/Home exit flow;
  home_new_prefix_terminal issues TerminalRelationV1::BoolLiteral, the existing
  child retention and normal_exit_projection_v1 consume it. The existing
  callable_result_representation solver remains the sole ExactBool class
  issuer; a later physical-header projection must corroborate it by key.
Non-authority: AST spelling at the physical boundary, emitted ValueId kind,
  the absence of a result annotation, ExactTrivialScalarAbiV1::I64, and all
  caller observations.
Fail-fast boundary: missing, wrong-owner/site, duplicate or non-Bool source
  relation never supplies a Normal exit or physical Bool result. A Bool
  source relation alone cannot select an ordinary physical role or invoke.
Smallest next slice: BOOL-LITERAL-TERMINAL-S0, issuing/retaining exact Bool
  source exits for unchanged SizeClassBox.accepts/1, replacing the current
  relation-less Some(_) acceptance. Then design/execute Bool result physical
  header and ordinary return; treat accepts_usize Bool call separately.
Non-claims: no Bool ABI, ordinary_bool role, selected-C Bool call/return,
  Eq physical acceptance, whole-source publication or mimalloc-lite EXE PASS.
```

S0 acceptance: the two original `accepts` exits retain distinct values,
owner, Return/value sites and Normal/Fault Home obligations; a missing or
wrong-site relation fails closed at the existing root exit. Reuse the existing
Home/terminal tests and the unchanged-source published-view probe, adding
only the independent Bool case. The probe may advance to a named physical
Bool boundary but must not be reported as PASS. The uncommitted Eq changes
remain protected and excluded from this S0 commit.

The following Bool physical slice must borrow the existing `ExactBool` solver
row and the exact terminal/Completion product, then issue a Bool result header
and ordinary physical role with final MIR/backend checks. It must not call the
i64 role Bool merely because both use an i64 payload. A separate Bool-call
slice is required for `accepts_usize` if that source becomes selected. The Eq
S0 card retains its uncompleted ordered Static packet/Compare/Branch
acceptance and resumes only after the required Bool source/physical boundary
is verified. The whole mimalloc-lite app's recorded first stop remains
Heap-to-Page.

## S0 implementation evidence (source boundary)

The Home verified walk now issues `BoolLiteral` from the resolver's exact
literal site and retains it for ordinary child completion. Root-only final
handoff refuses Bool physical publication explicitly. This does not change
the plain-completion path: the focused test uses a real `new Page()` Home so
both Bool exits traverse the selected verified walk and carry a live Home
cleanup obligation.

At the pre-commit source revision, `cargo test --profile quick -p nyash-rust
--lib bool_literal_exits_keep_distinct_source_relations_and_reject_missing_exit`
passed 1/1 (`CARGO_BUILD_JOBS=4`). The test pins false/true at separate
Return sites, the same Completion's Normal and Fault Home sets, missing-site
rejection, and sibling-site substitution rejection. Reusing the same built
binary, Normal projection family passed 9/9 and existing Bool result solver
tests passed 2/2. `git diff --check` passed. The first red iterations were
test-input selection (`plain completion`) and then test-only Rust borrow/name
errors; none remains in the final run. Unchanged-source Eq-integrated probe
is still pending because its three Eq files are protected uncommitted work
in the previous checkout. Until that probe identifies the next exact stop,
S0 does not claim original-source physical progress.
