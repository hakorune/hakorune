---
Status: design_stop; one bounded semantic-source row is missing at callable lowering
Task: MIR-CALLABLE-SEMANTIC-NESTED-IF-SOURCE-ROW-D0
Date: 2026-08-22
Priority: carry one parser-backed nested If variable site into the existing callable semantic state
Parent: SCRIPT-CATALOGED-BOX-ROOT-PARTITION-I0
PreviousCard: script-cataloged-box-root-partition-i0-2026-08-22
NextCard: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
---

# Callable semantic nested-If source row D0

## Six-line brief

```text
Decision: keep the parser/resolver semantic source as the sole owner and carry exactly one existing semantic variable-reference row for the nested IfCondition site into the current CallableSemanticLoweringState; do not extend W6 or add a second physical route.
Source authority + canonical issuer: `VerifiedNormalCallableSemanticSourceV1::seal` and its exact `loan` use the same parser invocation, `VerifiedSourceProjectionV1`, owner forest, and callable source ledger; `CallableSemanticLoweringState` consumes the resulting `(SourceNodeSiteV1, BindingRefV1)` relation.
Non-authority: selected-Dynamic W6/APrime rows, `body_state_bridge::observe_reads`'s hard-coded row list, Builder/AST reread, variable names, ordinal/path reconstruction, parameter defaults, raw ValueId, and any new physical writer.
Fail-fast boundary: before `read_callable_variable_v1` enters physical lowering; missing, foreign, duplicate, stale, or path-drifted source row rejects the unpublished callable session and never publishes semantic/local state.
Smallest next slice: census the exact parser-scan nested `IfCondition/Lhs/Lhs` row, prove its source binding and current lowerer site are the same authority relation, then add one bounded source-row consumer to the existing state/bridge seam.
Non-claims: no general nested-If support, all-method body lowering, W6 schedule expansion, A/C/Recipe/Join, publication, fallback, compatibility/raw retirement, backend, or performance.
```

## Observed blocker

After `SCRIPT-CATALOGED-BOX-ROOT-PARTITION-I0`, the unchanged
`parser_scan_loop_box.hako` fixture no longer leaks its top-level
`BoxDeclaration` into Script root lookup. Its lifecycle reaches `RootLower`
and stops at:

```text
[freeze:contract][callable-semantic-lowering/missing-variable-site]
site=[ProgramBodyRoot, ProgramBody(0), IfCondition, Lhs, Lhs]
```

This is a source-observation boundary failure, not permission to infer a
binding from the variable name or to make the physical lowerer rescan the AST.
The current selected-Dynamic body/state bridge observes its bounded existing
rows, while `CallableSemanticLoweringState::read_variable` requires every
callable variable site to be present in its parser-backed `variables` map.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `VerifiedNormalCallableSemanticSourceV1::seal` | same-parser callable semantic rows, owner forest, `VerifiedSourceProjectionV1` | physical values, MIR blocks, W6 target selection |
| `VerifiedNormalCallableSemanticSourceV1::loan` | one exact callable owner/input/ledger relation for the selected source row | source reparse, name/ordinal pairing, physical emission |
| `VerifiedSourceProjectionV1` | exact source-site navigation and semantic-site validation | semantic meaning inferred after lowering |
| `CallableSemanticLoweringState::from_exact_source` | state-local source binding map and owner/value ledger | issuing a second source authority |
| `CallableSemanticLoweringState::read_variable` / existing observation methods | one exact source-site consumption and current binding/value check | variable-name lookup, AST traversal, physical writer |
| selected-Dynamic W6/APrime relation | bounded `skip_while` physical rows already co-sealed for its dynamic lane | general callable variable coverage |
| `body_state_bridge::observe` | one-shot consumption of already-issued W6 rows and existing state seam | creating a new row from AST, expanding W6, publication |
| sole physical lowerer | existing MIR effects after source/state checks | semantic source repair or fallback |

The parser source and semantic projection remain one authority chain:

```text
same parser invocation
  -> VerifiedNormalCallableSemanticSourceV1::seal
  -> exact source loan / VerifiedSourceProjectionV1
  -> ResolvedFunctionLoweringInputV1
  -> CallableSemanticLoweringState::from_exact_source
  -> current source-site read
  -> existing physical lowerer
```

The bridge may consume the row, but it does not issue it. If the exact row is
not present in the sealed semantic source, this D0 is `NoSafeSlice`; adding a
literal `observe_reads` tuple is not a source fix.

## Exact bounded cohort

Only one row is in scope:

```text
source fixture: parser_scan_loop_box.hako
callable lane: the first selected body that reaches the existing bridge
site: ProgramBody(0) -> IfCondition -> Lhs -> Lhs
role: one variable reference consumed by the existing callable lowerer
```

Before implementation, the census must record:

```text
parser invocation witness
callable declaration/source site
FunctionOwnerIdV1
SourceNodeSiteV1 / SourceExprSiteV1 correspondence
ResolvedLexicalRefV1 and BindingRefV1
whether the row is already in `owner.variable_refs()`
whether the lowerer's current site is exactly the same canonical path
```

No source name, parser ordinal, AST address, digest, or independently rebuilt
path may be used as the join key. A private path locator may navigate the
already-sealed projection, but it may not become a second identity authority.

## Candidate boundary

The accepted design candidate is a narrow extension of the existing semantic
source-to-state seam:

```text
existing semantic source row
  -> exact loan/projection observation
  -> existing state `variables` binding map
  -> current `read_callable_variable_v1`
  -> existing lowerer
```

The implementation may add a private helper/accessor to the existing source
loan or state owner if the census proves the current accessor cannot expose
the exact row. It must not add a parallel `Verified*`/`Prepared*` semantic
product, a W6 row, or a Builder adapter merely to transport the site.

The current `body_state_bridge::observe_reads` array remains a consumer of
co-sealed W6 rows. It may call the existing state observation method for the
new row only when the source relation proves that the row belongs to the
selected callable owner and has not already been consumed. It must not list a
variable by hand as a substitute for source issuance.

## Finite state table

| State | Meaning | Effect | Next |
| --- | --- | ---: | --- |
| `SourceRowUnobserved` | exact parser-backed row has not yet been checked at the lowering seam | none | locate through existing loan/projection |
| `SourceRowReady` | owner, site, lexical binding, and current state binding cohere | none | one read consumption |
| `SourceRowConsumed` | existing `read_variable`/observation accepted it once | none | existing lowerer |
| `SourceRowMissing` | sealed source has no exact row | none | typed reject + discard |
| `SourceRowForeign` | owner or invocation differs | none | typed reject + discard |
| `SourceRowDrifted` | projection/current lowerer site differs | none | typed reject + discard |
| `SourceRowDuplicate` | same source site would be consumed twice | none | typed reject + discard |
| `NoSafeSlice` | row cannot be issued by the existing source authority | none | remain in design_stop |

There is no `default`, empty row, name lookup, or fallback transition. A
rejection occurs before physical lowering effects and before local semantic
publication. The outer unpublished callable/function session remains the only
discard authority.

## Fail-fast and failure atomicity

The implementation boundary is:

```text
current source context
  -> exact semantic row/binding check
  -> existing CallableSemanticLoweringState read/observation
  -> physical lowerer
```

The following must be checked before the lowerer consumes the variable:

```text
same parser invocation
same FunctionOwnerIdV1
same SourceNodeSiteV1 after projection
ResolvedLexicalRefV1 is a local BindingRefV1
state binding matches the source binding
site has not already been consumed
current physical value exists for the binding
```

If any check fails, the result is a typed semantic-lowering rejection and the
outer unpublished session is discarded. No MIR instruction, local map
publication, DraftSeal, ModuleDrain, or ExternalCommit may occur.

## Acceptance evidence for the future I0

Positive:

```text
exact parser-scan nested IfCondition variable row is present
same owner/binding/site reaches `CallableSemanticLoweringState`
lowerer consumes it once and reaches the next existing blocker or completes
no W6 row count or physical writer count increases
```

Negative:

```text
foreign parser invocation -> typed reject
foreign owner/binding -> typed reject
missing exact source row -> typed reject
path drift at IfCondition/Lhs/Lhs -> typed reject
duplicate observation -> typed reject
missing current value -> typed reject
each reject leaves instruction count, local publication, and DraftSeal unchanged
```

Structural guards:

```text
one semantic source issuer remains
body_state_bridge does not construct a source row from name/ordinal/AST
no W6/APrime schedule expansion
no second physical writer or lowerer retry
no fallback/compatibility/raw edge
touched production source remains below 760 lines and 800 hard stop
```

## NoSafeSlice conditions

Keep `design_stop` if any of these is true:

```text
`VerifiedNormalCallableSemanticSourceV1` cannot expose the exact row
`VerifiedSourceProjectionV1` cannot locate the same transformed source site
the row is absent from the same-parser semantic forest and would need AST inference
the lowerer uses a different site identity that requires ordinal/name re-pairing
the selected-Dynamic bridge must expand its W6 relation to make the row exist
the only available fix is a second physical writer or a compatibility retry
the failure happens after physical effects with no outer discard boundary
```

If the exact row is absent, stop and design the missing resolver/source
projection relation first. Do not convert the blocker to `NonCandidate`,
`Deferred` with a guessed cause, or a runtime fallback.

## Ordered task slice

This D0 has one successor only:

```text
MIR-CALLABLE-SEMANTIC-NESTED-IF-SOURCE-ROW-I0
```

The successor may implement only the accepted exact row and its focused
positive/negative evidence. After its caller/edge census is closed, the next
publication design may resume at
`MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0`.

No code, fixture, fallback, production switch, or new semantic receipt is
authorized by this D0.
