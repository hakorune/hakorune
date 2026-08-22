Status: if_control owner-split R0 complete; selected dynamic_full_body_recipe direction D0
Date: 2026-08-23
Parent: `CURRENT_STATE.toml` and `mirbuilder-post-audit-follow-up-queue-2026-08-21.md`
Current row: `MIRBUILDER-DYNAMIC-FULL-BODY-RECIPE-DIRECTION-D0`
---

# MirBuilder structure refactor queue D0

This card records the top-down structure review as bounded cleanup work. It
does not select a new lane, activate a production route, retire a semantic
owner, or authorize a broad directory move. The current pointer remains the
sole task selector.

## Decision

Accept only behavior-neutral `BoxShape` cleanup after the current Callable Loop
root Ready I0 has its required evidence. Keep the existing source, Facts,
Recipe, physical writer, and publication authorities unchanged. Each move must
have a complete caller/visibility/dependency census before the first rename.

The review contains three different kinds of proposal:

```text
already true / no-op
  loop physicalizer is already cfg(test)-only and caller-zero
  dynamic_v2_aot_activation has no tracked files

bounded BoxShape cleanup
  focused #[path] child-module grouping
  test-module extraction from near-limit files
  live README navigation correction

design-stop / later architecture
  if_control ownership split
  dynamic_full_body_recipe vocabulary relocation
  Script direct-static and normal_*/module_* shelving
  raw-root boundary unification, logger injection, AST view migration
```

## Six-line brief

Decision: Run a verified structural cleanup series, not a repository-wide
reorganization. Start with a census and one behavior-neutral child-module
move; keep semantic and production cutover rows closed.

Source authority + canonical issuer: None is introduced. Existing module
owners, source/Facts issuers, Recipe producers, physical writers, and
publication collectors remain the sole authorities.

Non-authority: directory names, line counts, `#[path]` syntax, `#[allow(dead_code)]`,
test-only canaries, empty directories, and README claims do not authorize a
route, semantic product, fallback, or retirement.

Fail-fast boundary: Before each move, prove exact callers, parent `cfg` scope,
visibility, re-export edges, test ownership, and dependency direction. Any
new public API, semantic reorder, second dispatcher, fallback, or owner drift
returns the row to design stop.

Smallest next slice: `MIRBUILDER-DYNAMIC-FULL-BODY-RECIPE-DIRECTION-D0` — map
the existing compiler/builder dependency edges and choose one-way vocabulary
ownership before any relocation; no code move is authorized by this D0.

Non-claims: no I9 transaction completion, no pure symbolic CorePlan, no
ordinary Outside consumer, no production switch, no legacy semantic retirement,
no compile-speed claim, and no blanket `pass` fusion.

## Local verification of the review

The following facts were checked in the current working tree on 2026-08-23:

| Review claim | Current fact | Disposition |
| --- | --- | --- |
| `loop_recipe_physicalizer/` has 24 files that should be isolated | The directory already has 24 focused files, and its parent declaration is `#[cfg(test)]`; the caller-zero card records no non-test issuer caller | Do not move now; retain caller-zero census and open a separate test-home BoxShape card only if a real path benefit remains |
| `common_v2` is flat and needs a directory | 27 `common_v2*` files are present, and `common_v2_session.rs` already groups child files with `#[path]` declarations | Full directory move needs D0; first map the session cluster, `super` imports, visibility, and re-exports |
| `dynamic_v2_aot_activation/` is empty noise | The directory has zero entries and zero tracked files | No repository task; do not invent a delete commit |
| `builder.rs` needs test extraction | 831 lines in the current worktree, with many `#[cfg(test)]` registrations | Valid near-limit BoxShape; split tests only, preserve production barrel and module visibility |
| `loop_physical_prepare.rs` needs a home before growth | 795 lines and test-only parent registration | Valid prerequisite BoxShape; no semantic changes or production activation |
| `compiler/tests.rs` is over the hard limit | 849 lines and a test-only owner | Needs a `super`/fixture/filter owner map before the later test-only split |
| `function/metadata.rs` is over 800 lines | 804 lines and a flat catalog with broad field ownership | Exclude from this queue; line count alone is not a split authority |
| live README points to `stmts.rs`/`exprs.rs` | `src/mir/builder/README.md` contains stale live-path references; `stmts/` exists, but there is no single `exprs.rs` or `exprs/` owner (the live split uses `raw_expression_dispatch/` and expression siblings) | Narrow documentation fix with an owner census; do not rewrite archive/history/reference claims globally |
| `normal_*` / `module_*` counts are 95 / 48 | The broad current filename census is 103 / 63; it is not an ownership census | Do not adopt the supplied counts; open a separate owner map before shelving |
| raw-root boundary is a simple 28 / 9 split | The broad current filename census is 38 files; compiler/builder ownership still needs route classification | D0 only; no directory merge from counts |
| ring0 has 440 reverse dependencies | A broad current `ring0` filename scan finds 223 files, not dependency edges | Measure actual import edges before injection design |
| AST direct imports are 617 | A broad current import scan finds 893 files; it includes tests and non-authority uses | Reject the number as a claim; stage a source-view census and isolate the synthetic-AST correctness case |

## Baseline census receipt — 2026-08-23

The baseline row is complete. The current tree confirms the following exact
owner facts before any move:

```text
loop physicalizer parent: resolved_lowering/mod.rs:145 #[cfg(test)]
loop physicalizer outside-tree consumers: test-only generic G0 session only
loop_physical_prepare parent: compiler/module_registry.in.rs:109 #[cfg(test)]
dynamic_v2_aot_activation tracked entries: 0
builder.rs: 831 lines
loop_physical_prepare.rs: 795 lines, test module begins at line 479
compiler/tests.rs: 849 lines, test-only owner
src/mir/function/metadata.rs: 804 lines, explicitly excluded
operation_emitter.rs: 491 lines, supplied 794-line claim is stale
if_control.rs: 798 lines, architecture D0, not a mechanical split
dynamic_full_body_recipe/mod.rs: 287 lines, 53 vocabulary references
pre-R0 snapshot: common_v2_session.rs had 12 #[path] child declarations
```

The live README stale references are limited to
`src/mir/builder/README.md:502-503,612`; the live tree has `stmts/` and
`raw_expression_dispatch/`, not a flat `exprs.rs` owner. Broad filename/import
counts remain informational until an owner/edge census exists.

The first implementation cell was deliberately the test-only
`binding_id_tests` module at `src/mir/builder.rs:736-831`. It preserves the
logical module name through an external `#[path]` child, changes no production
registration or authority, and has a measurable exit: `builder.rs < 760` with
the pre/post test symbol set unchanged.

## Builder test-home R0 receipt — 2026-08-23

`MIRBUILDER-BUILDER-TEST-HOME-R0` is complete. The four existing tests remain
under the logical `mir::builder::binding_id_tests` module, while their bodies
now live in `src/mir/builder/builder_binding_id_tests.rs` behind the parent
`#[cfg(test)]` path declaration. No production module registration,
re-export, semantic owner, fallback, or runtime route changed.

Evidence:

```text
builder.rs = 738 lines
builder_binding_id_tests.rs = 99 lines
focused binding_id_tests = 4 passed
cargo check --profile quick = passed (existing warning baseline retained)
builder test-home R0 guard = passed
focused rustfmt for new file = passed
git diff --check = passed
```

The full-workspace formatter still reports unrelated pre-existing formatting
drift outside this slice; it is not used as a current-change failure. The
following loop physical prepare test-home receipt records the next cell that
was selected at that time. `compiler/tests.rs`, README correction, common_v2
relocation, and all live semantic architecture rows remain separate.

## Loop physical prepare test-home R0 receipt — 2026-08-23

`MIRBUILDER-LOOP-PHYSICAL-PREPARE-HOME-R0` is complete. The six existing tests
remain under the logical `mir::compiler::loop_physical_prepare::tests` module,
while their bodies now live in
`src/mir/compiler/loop_physical_prepare_tests.rs` behind the parent
`#[cfg(test)]` path declaration. The owner remains test-only/caller-zero; no
production registration, semantic owner, fallback, publication, or runtime
route changed.

Evidence:

```text
loop_physical_prepare.rs = 481 lines
loop_physical_prepare_tests.rs = 319 lines
focused loop_physical_prepare = 6 passed
cargo check --profile quick = passed (existing warning baseline retained)
loop physical prepare home R0 guard = passed
git diff --check = passed
```

The new guard preserves the parent `#![cfg(test)]`, registry `#[cfg(test)]`,
logical module path, and all six test symbols. The following README receipt
records the next cell that was selected at that time; `compiler/tests.rs`,
common_v2 relocation, and all live semantic architecture rows remain separate.

## Live README path receipt P0 — 2026-08-23

`MIRBUILDER-LIVE-README-PATH-RECEIPT-P0` is complete. Only the current
`src/mir/builder/README.md` navigation claims were corrected:
`stmts/mod.rs` is the statement facade and
`raw_expression_dispatch/mod.rs` is the legacy raw-expression dispatcher.
No Rust module registration, semantic owner, fallback, publication, or
runtime route changed.

Evidence:

```text
two live target paths exist
stale flat stmts.rs/exprs.rs references = 0
git diff --check = passed
```

The next row is the compiler test-home D0. It is an owner/dependency census,
not permission to split or alter the production compiler module graph.

## Ordered task queue

### P0 — baseline and no-op closure

#### `MIRBUILDER-STRUCTURE-BASELINE-CENSUS-P0` — complete

Record the exact file counts, parent module declarations, caller classes,
re-exports, and line-budget values in a reusable guard or receipt. The guard
must distinguish production, test-only, compatibility, and disconnected
modules. A count-only guard is insufficient for a move.

Acceptance:

```text
loop_recipe_physicalizer parent cfg(test) = 1
loop physicalizer non-test issuer callers = 0
loop_physical_prepare parent cfg(test) = 1
dynamic_v2_aot_activation tracked files = 0
builder.rs / loop_physical_prepare.rs / compiler/tests.rs line counts recorded
metadata.rs is explicitly excluded with its owner rationale
live README stale references are enumerated separately from archive references
```

#### `MIRBUILDER-LIVE-README-PATH-RECEIPT-P0` — complete

Correct only current `src/mir/builder/README.md` references that claim
nonexistent flat `stmts.rs`/`exprs.rs` files. Point to the existing directory
facades or the current owning module. Do not change archived reports or
reference text without a separate authority review.

Acceptance: README links resolve; no Rust code, module path, semantic product,
or production caller changes; `git diff --check` is clean.

Evidence: both live paths resolve, the stale flat-path count is zero, and the
README-only diff passes `git diff --check`.

#### `MIRBUILDER-LOOP-CALLERZERO-NAVIGATION-P0`

Add a short navigation receipt to `src/mir/README.md` for the test-only Loop
physicalizer and its caller-zero card. This is documentation only; it must not
make the canary look production-reachable or authorize deletion.

Acceptance: the README points to the caller census and names the `cfg(test)` /
caller-zero status; no Rust module graph changes.

### R0 — behavior-neutral BoxShape series

Each row is a separate 2–5 commit refactor series. Do not combine file moves
with I9, source Facts, Recipe, or physical writer changes.

#### `MIRBUILDER-COMMON-V2-SESSION-HOME-D0` — complete

Audit the child implementation files currently attached to
`resolved_lowering/common_v2_session.rs` into a `common_v2_session/` directory
with an explicit `mod.rs` facade. Preserve the logical module names, visibility,
test paths, and every existing re-export. Start with the session cluster only;
do not sweep `loop_recipe_contract`, `compiler`, or unrelated `common_v2`
families into the same commit.

Only after that audit may a separate R0 move be opened. Stop if the move needs
a public visibility widening, a new route classifier, a second physical/session
owner, or a semantic reorder.

D0 census receipt:

```text
scope: src/mir/builder/resolved_lowering/common_v2_session.rs
parent: 545 lines; registered once as resolved_lowering::common_v2_session
private path children: 12
children: length_call, initial_index_seed, return_read, condition_bool,
  s6c_operand_issuer, s6c_text_eq_occurrence, s6c_substring_v9_issuer,
  s6c_substring_callout_materializer, session_length, session_segments,
  s6c_scalar_equality_leaf, s6c_cursor_cfg
largest child: s6c_cursor_cfg = 687 lines; all scoped files < 800
external logical consumers: resolved_lowering/mod.rs re-exports, physical_entry_session,
  physical_entry_draftseal, source/test bridges, and existing test-only modules
visibility: existing pub(in crate::mir*) scopes; no widening required
second session owner: none; CommonV2CanonicalSessionRefV1 remains the sole owner
outside scope: sibling common_v2 modules, test files, compiler admission files,
  pinned-text/residence owners, and unrelated common_v2 families
path-dependent evidence: common_v2_s6c_structure_guard.sh and README child references
```

The audit closes because every child is nested under the same logical parent,
all external consumers name the parent module rather than a physical child
file, and the move can preserve the current `pub(in crate::mir*)` scopes. The
existing compiler↔builder type dependency is pre-existing and is not changed
by this BoxShape move. The supplied broad `common_v2` file count is therefore
not a move scope.

#### `MIRBUILDER-COMMON-V2-SESSION-HOME-R0` — complete

Create `src/mir/builder/resolved_lowering/common_v2_session/mod.rs` and move
the audited parent plus its 12 children into that directory. Replace the
parent's path glue with ordinary child module declarations, retaining the
logical module name `resolved_lowering::common_v2_session` and every existing
re-export. Update only the structure guard and live README paths that refer to
the moved physical files.

Acceptance:

```text
common_v2_session/mod.rs remains < 760 lines and every moved child < 800
resolved_lowering::common_v2_session logical imports compile unchanged
all existing pub(in crate::mir*) visibility scopes remain unchanged
parent re-export set and test module paths are unchanged
common_v2_s6c_structure_guard.sh is updated and green
focused common_v2 session tests and cargo check --profile quick pass
git diff --check is clean
no non-session common_v2 file moves, semantic edits, new authority, route,
  fallback, physical effect, or production switch
```

Evidence:

```text
common_v2_session/mod.rs = 533 lines; 12 moved children, all < 800
largest moved child: s6c_cursor_cfg.rs = 687 lines
flat legacy session/child paths = 0; facade path attributes = 0
logical module/re-export/visibility/owner boundary unchanged
one moved fixture include_str path adjusted for the new physical depth
focused common_v2 suite = 63 passed, 0 failed, 0 ignored
common_v2_s6c_structure_guard.sh = passed
cargo check --profile quick = passed (existing warning baseline retained)
focused rustfmt, git diff --check, and current-state pointer guard = passed
no sibling common_v2 move, semantic edit, route, fallback, or production switch
```

#### `MIRBUILDER-BUILDER-TEST-HOME-R0` — complete

Extract one coherent `#[cfg(test)]` cluster from `src/mir/builder.rs` while
keeping `builder.rs` as the production barrel. Preserve test module paths where
guards or imports rely on them, and add a focused module README/guard only if
the ownership boundary changes.

Acceptance: production `builder.rs < 760` lines after the extraction, no
production module registration changes, focused builder tests green, and the
pre/post test symbol census is equal.

#### `MIRBUILDER-LOOP-PHYSICAL-PREPARE-HOME-R0` — complete

Split `src/mir/compiler/loop_physical_prepare.rs` before semantic growth. Keep
the existing test-only/caller-zero contract and assign each moved function to
one owner. This row is not permission to activate the Loop physicalizer.

Acceptance: no file reaches 800 lines; the caller-zero guard, focused canaries,
and fresh-session discard evidence remain unchanged.

Evidence: the six focused tests, `cargo check --profile quick`, the dedicated
home guard, and `git diff --check` passed. The parent is 481 lines and the
test child is 319 lines; the original logical module and `cfg(test)` scope are
unchanged.

#### `MIRBUILDER-COMPILER-TESTS-HOME-D0` — complete

First map `super` imports, test filters, fixture ownership, and parent
`#[cfg(test)]` scope for `src/mir/compiler/tests.rs`. Only then split it into
coherent test child modules in a later R0. Keep the
production compiler module graph unchanged and preserve all test names,
fixtures, and imports.

Acceptance for D0: the owner map identifies every moved test group and no
production module dependency. A later R0 must make `compiler/tests.rs < 760`
or document a justified remaining-owner exception; focused compiler tests and
diff/size guards must be green.

Current design-stop brief:

```text
Decision: map ownership before moving any compiler test group.
Source authority + canonical issuer: none; tests observe existing production owners.
Non-authority: filename counts, test names, filters, and line count alone.
Fail-fast boundary: stop before code movement if a group has unresolved super/fixture/visibility edges.
Smallest next slice: one owner map for all groups and parent #[cfg(test)] scope.
Non-claims: no split, production registration change, semantic change, fallback, or new receipt.
```

D0 census receipt:

```text
parent: src/mir/compiler/mod.rs:710 #[cfg(test)] mod tests;
production registration: none; module_registry.in.rs is unchanged
test count: 25 total, 6 ignored, 19 non-ignored
shared surface: one super import block; no local helper module or test fixture file
crate surface: AST/MIR/parser/runtime/config/test-support reads only
groups: finish-schedule/discard (6), exact-numeric contracts (5), basic/legacy lowering (5), string corridor (2), method-id (1), await (3), throw/loop/try-catch (3)
external state: ring0 initialization in numeric/try paths; env mutation only in ignored await rewrite
R0 shape: keep compiler::tests as the parent facade; add test-only child files under compiler/tests/ and retain all function names/attributes
```

The D0 is complete because every test belongs to one bounded group, the only
parent-private surface is the existing compiler module import block, and no
production module dependency was found. The later R0 must preserve the
`#[cfg(test)]` parent registration and must not alter fixtures, semantics, or
runtime/compiler ownership.

#### `MIRBUILDER-COMPILER-TESTS-HOME-R0` — complete

Move the seven mapped test groups behind a test-only `compiler::tests` facade.
This is a behavior-neutral BoxShape refactor, not a test repair or compiler
production change.

Acceptance: `compiler/tests.rs < 760`, all 25 test names and six ignore
attributes remain, the parent `#[cfg(test)] mod tests;` remains the only
production registration, focused compiler tests retain their current result,
and one reusable size/module guard proves the split.

Evidence:

```text
compiler/tests.rs facade = 30 lines
child groups = 7; every child < 800 lines
test attributes = 25; ignored tests = 6; names preserved exactly once
focused compiler suite = 5 passed, 14 failed, 6 ignored
parent 93c5fc3e0a = 5 passed, 14 failed, 6 ignored with the same baseline errors
cargo check --profile quick = passed (existing warning baseline retained)
compiler tests home R0 guard = passed
focused rustfmt check = passed
git diff --check = passed
```

The 14 failing tests are recorded as existing baseline debt: normal-program
admission, instance-constructor source cohort, and script-neutral-window
contract failures. The split changed only test ownership and preserves the
test-gated compiler production module graph.

### D0 — deferred architecture decisions

The following are not mechanical moves. They require a separate authority
brief before implementation:

| Row | Reason for design stop |
| --- | --- |
| `MIRBUILDER-IF-CONTROL-OWNER-SPLIT-D0` | analyzer, use-ledger, and product ownership must be separated without creating a second control authority |
| `MIRBUILDER-DYNAMIC-FULL-BODY-RECIPE-DIRECTION-D0` | the current broad token scan finds 53 occurrences; moving compiler vocabulary across the dependency edge can either resolve or deepen the cycle; prove the desired one-way ownership first |
| `MIRBUILDER-SCRIPT-DIRECT-STATIC-SHELF-D0` | consolidation must follow the active Script cutover and old-route retirement, not precede it |
| `MIRBUILDER-NORMAL-MODULE-SHELF-D0` | broad current filename counts are 103 `normal_*` and 63 `module_*`; an ownership map is needed, not a directory-only rename |
| `MIRBUILDER-RAW-ROOT-BOUNDARY-D0` | compiler/builder split affects source, physical, and publication edges |
| `MIRBUILDER-RING0-LOGGER-INJECTION-D0` | a broad scan finds 223 ring0-containing files, not dependency edges; measure the actual graph and forbid a hidden global observer |
| `MIRBUILDER-AST-VIEW-MIGRATION-D0` | the broad current scan finds 893 AST-importing files, not 617; staged source-view authority is needed, and any synthetic AST construction is a separate correctness review |

#### `MIRBUILDER-IF-CONTROL-OWNER-SPLIT-D0` — complete

This is the selected next row. It is a BoxShape audit, not permission to edit
`if_control.rs`. The file is at the 760-line split trigger and just below the
800-line hard stop, but line count alone does not define the new module
boundaries.

Six-line brief:

```text
Decision: preserve one resolved If-control authority; first map a safe split of analyzer, use-ledger, and product surfaces.
Source authority + canonical issuer: Resolved source/completion input is consumed by the existing IfControlAnalyzerV1; the final VerifiedResolvedFunctionIfControlV1 remains the sole product owner until the audit accepts a split.
Non-authority: filenames, re-export barrels, compiler projections, loop_owned_if helpers, test fixtures, and line counts cannot issue a second If-control product.
Fail-fast boundary: stop before any move if private visibility, cfg(test) scope, super imports, or compiler↔builder direction cannot be preserved without duplicate types or a new dispatcher.
Smallest next slice: produce the symbol/visibility/caller/dependency census and a candidate module graph for analyzer, use-ledger, and product; no code movement.
Non-claims: no semantic change, new receipt, route classifier, fallback, production switch, I9 work, or broad builder shelving.
```

Current census receipt:

```text
source: src/mir/resolved_control_flow/if_control.rs = 798 lines
registration: resolved_control_flow/mod.rs registers if_control once; tests are cfg(test)
current logical surfaces: source analyzer + completion/coverage validation,
  VerifiedResolvedFunctionIfControlV1 / row materialization product,
  FunctionIfControlUseLedgerV1 + IfControlCoverageUseV1
production import files: 20 (31 including test-only files)
adjacent helper: loop_owned_if.rs consumes the existing product and is not a second issuer
known public surface: 2 verifier entrypoints plus crate-visible product/ledger types
```

Required D0 output:

```text
1. exact symbol map: analyzer, coverage validator, product, row materializer,
   use-ledger, error vocabulary, and test-only helpers
2. caller map for both verifier entrypoints and every crate-visible type
3. visibility/re-export/cfg map proving no widening and no duplicate product type
4. dependency graph proving the split does not deepen compiler↔builder cycles
5. candidate module graph and one owner for each moved symbol
6. focused-test and reusable-guard plan; implementation remains forbidden until
   this Decision is accepted
```

Decision receipt:

```text
symbol map complete: analyzer/source navigation/seal; product/row materialization;
  use-ledger/coverage consumption; error vocabulary; test-only callers
caller map complete: verifier entrypoints have one production caller in
  compiler/capability.rs; remaining verifier calls are tests; product/ledger
  consumers are mapped to canonical_ssa and trivial_ssa lowering
visibility map complete: resolved_control_flow/mod.rs registers if_control once;
  no public re-export widening is required; tests remain cfg(test)
dependency map complete: the split remains inside resolved_control_flow; the
  existing compiler -> resolved_control_flow -> builder-consumer direction is
  unchanged and no new compiler↔builder edge is introduced
accepted module graph: if_control/mod.rs facade -> product.rs, use_ledger.rs,
  analyzer.rs; source_coverage.rs/function_control.rs/loop_owned_if.rs remain
  sibling owners; no duplicate product or verifier issuer
```

The audit closes because the candidate is a pure relocation of existing
symbols. The product remains `VerifiedResolvedFunctionIfControlV1`, the
analyzer remains the sole verifier issuer, and use-ledger movement changes no
consumer contract. The following R0 is the only authorized implementation
slice; no new semantic type, route, fallback, or production edge is opened.

#### `MIRBUILDER-IF-CONTROL-OWNER-SPLIT-R0` — complete

Create `src/mir/resolved_control_flow/if_control/mod.rs` and move the mapped
symbols into three private children:

```text
product.rs
  ResolvedIf* ports, VerifiedLocatedIfControlV1,
  ResolvedIfControlMaterializationV1,
  VerifiedResolvedFunctionIfControlV1, coverage partition row
use_ledger.rs
  FunctionIfControlUseLedgerV1, IfControlCoverageUseV1,
  their existing error types
analyzer.rs
  IfControlAnalyzerV1, row draft, policy, verifier entrypoints,
  source navigation and existing analyzer errors
```

Keep `resolved_control_flow::if_control` as the logical module and re-export
the existing crate/super-visible symbols from `mod.rs`. `source_coverage.rs`,
`function_control.rs`, and `loop_owned_if.rs` stay in their current homes.

Acceptance:

```text
if_control/mod.rs is the only logical facade
product.rs owns the existing If rows/materialization/product types
use_ledger.rs owns the existing coverage ledgers and errors
analyzer.rs owns the existing source navigation/verifier entrypoints
resolved_control_flow/mod.rs still registers if_control exactly once
no visibility widening, semantic reorder, route, fallback, or production switch
```

Evidence:

```text
if_control/mod.rs = 30 lines
product.rs = 247 lines; use_ledger.rs = 103 lines; analyzer.rs = 468 lines
flat legacy if_control.rs = 0; all four owners remain below 800 lines
product/verifier authority definitions = one each
resolved_control_flow focused suite = 33 passed, 0 failed
resolved_value_profile focused suite = 46 passed, 0 failed
resolved_if_control_structure_r0_guard.sh = passed
focused rustfmt, cargo check --profile quick, and git diff --check = passed
logical imports/re-exports, visibility, test paths, and behavior unchanged
```

Commit: `7c9ea5944f` (`refactor: split resolved if control ownership`).
The R0 is a BoxShape-only cleanup; it does not activate the disconnected
If-control analyzer or create a second production authority.

#### `MIRBUILDER-DYNAMIC-FULL-BODY-RECIPE-DIRECTION-D0` — selected next

The review proposal to move `dynamic_full_body_recipe` vocabulary toward the
Builder is not yet an implementation task. The current broad scan records 53
references, but a token count cannot distinguish compiler semantic ownership,
Builder physical consumption, compatibility glue, and test-only evidence. A
blind move could deepen the existing compiler↔Builder cycle instead of making
the dependency one-way.

Six-line design brief:

```text
Decision: audit dependency direction and vocabulary ownership before any move;
  do not relocate compiler symbols from the proposal alone.
Source authority + canonical issuer: existing compiler-owned dynamic semantic
  program/Facts/Recipe issuers remain sole owners; no issuer is added by D0.
Non-authority: directory names, token counts, re-exports, physical adapters,
  Builder state, compatibility shells, and test fixtures.
Fail-fast boundary: stop before editing if a proposed edge adds a compiler↔
  Builder cycle, widens visibility, mixes semantic and physical meanings, or
  requires a second Recipe/Join/physical authority.
Smallest next slice: classify all 53 references by definition/caller/owner,
  draw the current and proposed dependency graph, and select one bounded
  facade or vocabulary move only if it has a one-way proof.
Non-claims: no directory move, rename, re-export sweep, semantic reorder,
  pure-plan conversion, fallback, production switch, or performance claim.
```

Required D0 tasks:

```text
1. Census every dynamic_full_body_recipe reference and classify it as
   compiler semantic definition, compiler consumer, Builder physical consumer,
   compatibility/migration, or test-only evidence.
2. Separate names that describe source/Facts/Recipe/Join meaning from names
   that describe MIR/ValueId/BasicBlock/physical publication.
3. Map reverse imports and re-exports in both directions; record the exact
   edges that would be removed, preserved, or newly introduced.
4. Produce two candidate graphs: keep compiler vocabulary with a thin Builder
   adapter, or move only a dependency-free vocabulary leaf behind a facade.
5. Reject both candidates if neither gives one-way ownership without a new
   semantic receipt, visibility widening, or duplicated issuer.
6. Select at most one behavior-neutral next BoxShape cell with caller,
   visibility, line-budget, and focused-gate acceptance; otherwise keep D0.
```

The next implementation cell is forbidden until this Decision is accepted.
This row is intentionally separate from Script direct-static shelving and
from Dynamic I9 transaction hardening.

Acceptance:

```text
if_control/mod.rs remains < 760 lines; every child remains < 800
all existing logical imports and test paths compile unchanged
all current visibility scopes remain externally equivalent
VerifiedResolvedFunctionIfControlV1 and verifier entrypoints remain unique
focused resolved_control_flow and resolved_value_profile tests retain results
no new semantic type, issuer, route, fallback, effect, or production switch
reusable guard rejects flat child paths, duplicate verifier definitions, and
  any child over the 800-line boundary
```

## Guard and closeout contract

Every R0 commit must run:

```text
cargo check --profile quick
focused tests for the moved owner
existing caller-zero/source-Facts guards
new module/line-budget guard
git diff --check
```

Red results are classified as current-change failure, known baseline debt, or
informational census. A line-count reduction without symbol/dependency
evidence is not completion. The R0 and architecture rows remain parked until
the selected baseline census is complete and `CURRENT_STATE.toml` explicitly
selects one bounded cleanup row.
