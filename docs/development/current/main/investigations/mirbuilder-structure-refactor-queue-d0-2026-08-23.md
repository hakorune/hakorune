Status: parked BoxShape/design queue; not the current execution pointer
Date: 2026-08-23
Parent: `CURRENT_STATE.toml` and `mirbuilder-post-audit-follow-up-queue-2026-08-21.md`
Current row: `MIR-CALLABLE-LOOP-ROOT-UNPUBLISHED-SCOPE-I0` remains active
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

Smallest next slice: `MIRBUILDER-STRUCTURE-BASELINE-CENSUS-P0` — record the
validated facts below, add only a narrow navigation/line-budget guard, and
select one child-module/test extraction for a later behavior-neutral commit.

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

## Ordered task queue

### P0 — baseline and no-op closure

#### `MIRBUILDER-STRUCTURE-BASELINE-CENSUS-P0`

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

#### `MIRBUILDER-LIVE-README-PATH-RECEIPT-P0`

Correct only current `src/mir/builder/README.md` references that claim
nonexistent flat `stmts.rs`/`exprs.rs` files. Point to the existing directory
facades or the current owning module. Do not change archived reports or
reference text without a separate authority review.

Acceptance: README links resolve; no Rust code, module path, semantic product,
or production caller changes; `git diff --check` is clean.

#### `MIRBUILDER-LOOP-CALLERZERO-NAVIGATION-P0`

Add a short navigation receipt to `src/mir/README.md` for the test-only Loop
physicalizer and its caller-zero card. This is documentation only; it must not
make the canary look production-reachable or authorize deletion.

Acceptance: the README points to the caller census and names the `cfg(test)` /
caller-zero status; no Rust module graph changes.

### R0 — behavior-neutral BoxShape series

Each row is a separate 2–5 commit refactor series. Do not combine file moves
with I9, source Facts, Recipe, or physical writer changes.

#### `MIRBUILDER-COMMON-V2-SESSION-HOME-D0`

Audit the child implementation files currently attached to
`resolved_lowering/common_v2_session.rs` into a `common_v2_session/` directory
with an explicit `mod.rs` facade. Preserve the logical module names, visibility,
test paths, and every existing re-export. Start with the session cluster only;
do not sweep `loop_recipe_contract`, `compiler`, or unrelated `common_v2`
families into the same commit.

Only after that audit may a separate R0 move be opened. Stop if the move needs
a public visibility widening, a new route classifier, a second physical/session
owner, or a semantic reorder.

#### `MIRBUILDER-BUILDER-TEST-HOME-R0`

Extract one coherent `#[cfg(test)]` cluster from `src/mir/builder.rs` while
keeping `builder.rs` as the production barrel. Preserve test module paths where
guards or imports rely on them, and add a focused module README/guard only if
the ownership boundary changes.

Acceptance: production `builder.rs < 760` lines after the extraction, no
production module registration changes, focused builder tests green, and the
pre/post test symbol census is equal.

#### `MIRBUILDER-LOOP-PHYSICAL-PREPARE-HOME-R0`

Split `src/mir/compiler/loop_physical_prepare.rs` before semantic growth. Keep
the existing test-only/caller-zero contract and assign each moved function to
one owner. This row is not permission to activate the Loop physicalizer.

Acceptance: no file reaches 800 lines; the caller-zero guard, focused canaries,
and fresh-session discard evidence remain unchanged.

#### `MIRBUILDER-COMPILER-TESTS-HOME-D0`

First map `super` imports, test filters, fixture ownership, and parent
`#[cfg(test)]` scope for `src/mir/compiler/tests.rs`. Only then split it into
coherent test child modules in a later R0. Keep the
production compiler module graph unchanged and preserve all test names,
fixtures, and imports.

Acceptance for D0: the owner map identifies every moved test group and no
production module dependency. A later R0 must make `compiler/tests.rs < 760`
or document a justified remaining-owner exception; focused compiler tests and
diff/size guards must be green.

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
| `MIRBUILDER-AST-VIEW-MIGRATION-D0` | the broad current scan finds 893 AST-importing files, not 617; staged source-view authority is needed, and the synthetic AST construction in `recipe_tree/matcher` is a separate correctness review |

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
evidence is not completion. The queue remains parked until the current
Callable Loop root Ready I0 is closed and `CURRENT_STATE.toml` selects a
cleanup row.
