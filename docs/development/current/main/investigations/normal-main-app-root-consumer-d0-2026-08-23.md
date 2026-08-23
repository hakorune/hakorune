# Normal root consumer replacement series

Status: S0 implementation complete; I0 selected as mandatory successor
Date: 2026-08-23
Owner: parser root authority -> normal/default root lifecycle
Work mode: fast

## Execution brief

Decision:
  Use one parser-owned HRTB App/Script root loan. App preserves Program order
  with an opaque RootMain item; Script exposes one paired statement cursor.
Source authority + canonical issuer:
  ParserNormalRootPreservationV1 plus the complete same-invocation Program
  authority; with_parser_normal_root_consumer_loan is the sole view issuer.
Non-authority:
  VerifiedRawRootExpansionV1, source_ast scans, names, ordinals, pointers,
  root_is_app_mode, Script-A rows, Builder state, Recipe, and MIR.
Fail-fast boundary:
  The source-backed lifecycle consumes the root loan before declaration facts,
  resolver creation, target installation, module preparation, or Builder work.
Smallest next slice:
  S0 parser loan, immediately followed by I0 lifecycle consumption, then R0
  retirement of the remaining source-backed raw observer and bool selection.
Non-claims:
  No compatibility policy change, new App shape, Script-A meaning, Recipe,
  physical lowering, fallback, publication, performance work, or barrel move.

## Closed prerequisites

- NORMAL-MAIN-ROOT-PRESERVATION-A-I0 (a906f4aec2) carries one non-Clone
  final-transform root token.
- NORMAL-CALLABLE-SOURCE-TRANSFORM-DISPOSITION-I0 (198560b0e0) permits only
  unchanged source to reach parser finalization. TestHarnessGeneratedTail
  enters typed compatibility before root-token issuance.
- NORMAL-MAIN-ROOT-EXACT-COHORT-I0 (0b9d4eb43d) preserves the complete
  Program body exactly.
- NORMAL-MAIN-APP-ROOT-RELATION-I0 (b96d3f17b3) co-seals the existing App
  admission, opaque callable identity, paired final slot, same parser witness,
  Main-is-root, and no-static-children relation.

## Selected authority chain

~~~text
ParserNormalRootSourceDispositionV1
  -> exact final-transform preservation
  -> ParserNormalRootPreservationV1
  -> scoped ParserNormalRootConsumerLoanV1
       App(root body + ordered RootMain/sibling cursor)
       Script(paired statement cursor)
  -> move-only lifecycle AfterLoan owner
  -> existing semantic package/work-plan path
  -> R0 removes the remaining raw source-backed observer
~~~

The parser decides App versus Script once. The lifecycle may project that
already-admitted role into a temporary execution mode, but cannot classify the
AST again. Compatibility keeps its existing raw owner and never receives a
synthetic parser root receipt.

## Root-loan contract

~~~rust
impl VerifiedFinalCallableProgramSourceV1 {
    pub(crate) fn with_normal_root_consumer_loan<R>(
        &self,
        consume: impl for<'src> FnOnce(
            ParserNormalRootConsumerLoanV1<'src>,
        ) -> R,
    ) -> Result<R, ParserNormalRootConsumerLoanRejectV1>;
}
~~~

App exposes only:

- root body, uses, attrs;
- closed Implicit | Explicit(&str) result syntax;
- one Program-order cursor;
- RootMain at the admitted Main statement position;
- {kind, statement} for every sibling.

App does not expose the raw Main declaration, name, parameters, parser anchor,
source site, final-slot ordinal, or pointer. Script does not expose parallel
row and AST slices. The HRTB callback cannot return any borrowed view.

Terminal states are exhaustive and typed:

~~~text
Outside(reason)
ScriptTerminal(reason)
SourceAuthorityUnavailable(reason)
Incomplete(reason)
IntegrityInvalid(reason)
DiscardedBeforeA
~~~

Every terminal rejects before the callback and before Builder effects. There
is no wildcard/default mapping, repair, fallback, or retry.

## Replacement series

### S0 — NORMAL-ROOT-CONSUMER-LOAN-S0

Change:
  Add only the parser consumer-loan module, final-source facade, focused tests,
  parser README receipt, and the existing reusable guard extension.
Contract:
  App uses exact [Sibling..., RootMain, Sibling...] order; Script uses one
  paired cursor; all loans/cursors are non-Clone.
Done:
  App-only, App+sibling, empty Script, nonempty Script, nonzero Main arity, and
  Main-helper terminal evidence pass. Production caller count is zero.
Stop:
  Caller-zero lifetime is one landed commit. The immediate next semantic
  commit must be I0 or S0 is reverted.

### I0 — NORMAL-ROOT-CONSUMER-I0

Change:
  Add one private, move-only
  PreparedNormalDefaultProgramRootAfterLoanV1 and consume the parser loan at
  the first source-backed lifecycle operation.
Contract:
  Parser-backed App/Script role comes only from the loan. Compatibility alone
  may retain the existing raw preflight. AfterLoan has no reloan, generic
  parts escape, Clone, or public constructor.
Done:
  Source-backed root-loan production caller = 1; source-backed raw preflight
  classifier = 0; every loan terminal has Builder effect count 0; existing
  positive App/Script lifecycle tests remain green; fallback/retry = 0.
Stop:
  The retained-source raw expansion may remain only as an integrity observer
  reserved to R0. It may not override the parser-issued role.

### R0 — NORMAL-ROOT-CONSUMER-RAW-RETIREMENT-R0

Change:
  Replace the retained source-backed raw expansion/work-plan bool input with
  the admitted App/Script structural projection.
Contract:
  VerifiedRawRootExpansionV1 may remain only for explicit compatibility or
  test ownership; it is not selected-normal route authority.
Done:
  Source-backed VerifiedRawRootExpansionV1::from_program callers = 0;
  source-backed App/Script bool selector = 0; stale “only route authority”
  comment is removed; root role in work-plan/Builder is typed and
  source-admitted; old bypass/fallback = 0.
Stop:
  Do not broaden App shapes or change compatibility semantics.

## Acceptance guard

The existing
tools/checks/parser_normal_root_preservation_a_i0_guard.sh owns this series.
It must prove:

~~~text
root-loan issuer definition                       = 1
root-loan production caller                       = 0 in S0, 1 in I0/R0
raw Main/program/source-row/identity accessor      = 0
loan/cursor/AfterLoan Clone                        = 0
HRTB callback signature                            = 1
source-backed old raw preflight caller             = 0 after I0
source-backed all raw classifier callers           = 0 after R0
canonical terminal -> Builder effect               = 0
fallback/retry                                     = 0
production source                                  < 760 lines
~~~

Focused Cargo evidence:

~~~text
parser normal_root_preservation_tests
normal/default root lifecycle tests
cargo check
current_state_pointer_guard
git diff --check
~~~

## NoSafeSlice

Return to NoSafeSlice if any of these becomes necessary:

- raw Program or raw Main access escapes the loan;
- App siblings cannot retain exact Program order;
- names, ordinals, pointers, or AST equality decide App/Script downstream;
- Script rows and statements must be separated and re-paired;
- a callback can return a borrow;
- AfterLoan can reloan, Clone, or expose generic parts;
- a source terminal reaches declaration/resolver/target/module/Builder effects;
- compatibility must receive a fabricated parser receipt;
- I0 cannot immediately consume S0;
- source-backed raw classification remains authoritative after R0.

## Follow-up taskization

These are ordered after the root S0/I0/R0 series and must not be mixed into
its semantic commits:

1. CURRENT-STATE-COMPACT-POINTER-P0
   - keep only current lane, blocker, authorized next step, prohibitions, and
     the latest 2-3 landed entries;
   - move accumulated history to git/history owners; create no second pointer.
2. MIRBUILDER-README-STABLE-CONTRACT-R0
   - retain navigation, north star, responsibility map, replacement law, and
     stable compatibility owners;
   - retire dated Current frontier and closed-row journals from the README.
3. MIRBUILDER-BARREL-OWNER-CENSUS-D0
   - classify registrations as production, compatibility, caller-zero,
     test-only, or facade/re-export;
   - retire caller-zero first, then move only owner-safe atoms;
   - no all-at-once production/experimental/compatibility reshuffle.

src/mir/README.md navigation, parser witness/final preservation, HRTB loans,
ModuleBuilderInvocationSession outer transaction, thin CURRENT_TASK.md, and
owner-specific direct-static semantic shelves are Keepers.
