# RAW-SCRIPT-DEMAND-WINDOW-BOUNDARY2-D0 — design consultation (closed)

```text
Decision:
  Accept-corrected.

Execution:
  RAW-SCRIPT-ROOT-NEUTRAL-LEXICAL-SHADOW-CUTOVER0-I0-R0

Resolution:
  Issue one total Program-item demand window from the existing work-plan
  partition. Every original ordinal gets a typed Resolved, Transferred,
  Diagnostic, or Transparent disposition plus its retained-runtime status.
  Only the existing lexical closure and the already typed StaticConst,
  selected-diagnostic, and top-level-callable boundaries may select Complete.
  All other responsibilities select Deferred before child descent. Complete
  alone reaches the shared root-neutral traversal, canonicalizer, forest, and
  Program projection; Deferred owns none and lowers once through the existing
  terminal. The full manual selected-Script resolver chain is deleted in the
  same commit.
```

```text
Parent:
  RAW-SCRIPT-ROOT-NEUTRAL-LEXICAL-SHADOW-CUTOVER0-I0-R0

Reason:
  the shared shadow/canonical core is viable, but the existing work-plan does
  not yet issue one total, typed Program-item demand window.  Continuing from
  runtime rows alone would silently omit ImmediateOnly callable boundaries and
  turn the Script route into a partial semantic owner.
```

## Current exact state

The following are real and must be reused rather than replaced:

```text
ScriptSyntaxViewV1 / SemanticOwnerRootProfileV1::Script
ProgramBodyRoot / ProgramBody(original ordinal)
shared shadow statement/expression traversal
shared canonicalize_draft and Script forest/projection payload
PreparedProgramRootWorkPlanV1 partition:
  ImmediateAndRuntime | ImmediateOnly | DeferredAndRuntime | RuntimeOnly
```

The old selected-Script authority is still:

```text
runtime semantic_closure_admission
-> admit_runtime_script_lexical_v1
-> visible-name map + recursive AST walk
-> manual Local/Variable facts and Script draft construction
```

It must disappear in the atomic cutover; wrapping it is not an answer.

## Missing decision

The prior consultation correctly selected a sparse root-neutral traversal, but
did not pin down the *total Program-item product* that precedes it.  A runtime
row list is insufficient:

```text
FunctionDeclaration: ImmediateOnly / transferred callable boundary
instance Box:        ImmediateAndRuntime
static Box:          DeferredAndRuntime or RuntimeOnly by mode
StaticConst:         metadata transfer + retained runtime completion
selected unsupported: diagnostic boundary + retained runtime terminal
ordinary Script item: resolved or Deferred responsibility
```

The next design must decide this product before another implementation attempt.

## Question

Design one source-only `VerifiedScriptRootDemandWindowV1` (name may be
corrected) issued exactly once from the existing Program work-plan partition.
It must cover every original Program ordinal exactly once and let the selected
Script route choose exactly one terminal:

```text
Complete
  -> sparse shared shadow traversal for only Resolved entries
  -> one shared Script canonicalization / forest / projection
  -> existing RootLower once

Deferred
  -> no owner / forest / projection
  -> ExistingRootLower once
```

For every Program item, the window must issue one typed disposition, not an
untyped skip:

```text
Resolved
TransferredCallable
TransferredStaticMetadataWithRuntimeCompletion
ExistingDiagnosticBoundary
RetainedRuntimeCompatibility
```

You may rename or refine this vocabulary, but it must make child demand and
terminal owner explicit.

## Exact questions to answer

1. What are the fields and sealed invariants of the total demand window?
   In particular, how does it retain original ordinal without cloning or
   rereading the Program AST?
2. Which existing work-plan point issues it without making
   `program_root_work_plan.rs` (currently 799 lines) exceed the limit?
3. Give the exact disposition for all four work-plan variants and for
   StaticConst, selected unsupported, top-level FunctionDeclaration, Using,
   instance Box, static Box, and ordinary lexical candidates.
4. Which entries may be traversed under `ScriptLexicalCoreV1` now, and which
   must select Deferred *before child descent*?  Preserve Weak/Call/If error
   precedence.
5. How does the Complete verifier prove:

```text
every Program ordinal = exactly one disposition
every Resolved ordinal = exactly one sparse traversal demand
no Script facts inside transferred/diagnostic prefixes
no partial forest/projection
```

6. What is the smallest atomic I0/R0 after this D0?  Name its production
caller, new owner, exact old-edge deletion, focused fixture set, and
hard-stop conditions.
7. State whether the existing ten Complete fixture identities can remain
Complete in that first row.  If not, return `NoSafeSlice`; do not narrow them
silently.

## Non-claims and hard constraints

```text
Do not add a second resolver, Script-only canonicalizer, forest, or projection.
Do not widen FunctionSyntaxViewV1 or FunctionSourceViewV1 to Program.
Do not synthesize FunctionDeclaration from Program.
Do not classify only runtime rows and call it complete coverage.
Do not resolve children of a responsibility-disabled entry.
Do not move existing user diagnostics into ScriptSemanticSeal.
Do not downgrade a failed Complete seal into Deferred.
Do not retry/fallback/reselect after the terminal choice.
Do not clone/reparse/reread source AST.
Do not add a production-caller-zero proof owner or a per-row guard.
Do not exceed 800 lines in any source/check file.
```

## Required answer shape

```text
Decision: Accept / Accept-corrected / NoSafeSlice
total window product and issuer
Complete / Deferred / rejection terminals
Program-item disposition matrix
production graph and same-commit old-edge deletion
focused evidence and fixture-identity ratchet
800-line-safe file split
hard stops
```
