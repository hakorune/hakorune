---
Status: accepted design; implementation not opened
Date: 2026-08-09
Decision: accepted bounded postpass-visible BuildGate decision-set design
Parent: `parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md`
---

# PARSER-PUBLIC-AST-POSTPASS-I0-C

## Decision

I0-C closes the duplicated predicate-evaluation boundary for the
postpass-visible AST `BuildGate` family. It does not redesign member-level
gate selection, which is performed during Box member parsing and does not
remain as an `ASTNode::BuildGate`.

```text
parse_program once
  -> parser-issued postpass BuildGate inventory
  -> one PreparedBuildGateDecisionSetV1
  -> prune / source-path survival / explain projections
  -> one total postpass completion
```

The current three evaluators are not independent authorities after I0-C:

```text
source_gate_prune.rs       -> decision-set projection
build_cfg/prune.rs         -> decision-set projection
build_cfg/predicate.rs     -> decision-set explain projection
```

`NyashParser::eval_build_predicate` is used by the decision-set issuer only.
No consumer re-evaluates a predicate, and no public caller catches a failed
projection to try another postpass arm.

## Scope boundary

Included:

```text
top-level AST BuildGate
nested top-level BuildGate
statement-level AST BuildGate inside active AST containers
Build / Feature / Target / Backend predicates
not / all / any combinators
top-level source-path survival/rebase relation
public explain-report projection
```

Explicitly excluded:

```text
member-level gate groups merged by BoxMemberState
member signature/layout policy
@rune Gate member sugar
parse_grammar_evidence_from_string_with_build_config
resolver source publication
Recipe/Builder/MIR/runtime
provider/runtime dispatch
```

Member-level gates remain governed by their existing same-public-signature
rule. The known nested member-gate baseline red remains under
`PARSER-MEMBER-GATE-NESTED-SOURCE-PATH-D0`; I0-C must not weaken that rule.
Grammar-evidence parsing remains a separate token/grammar projection until a
later demand is explicitly designed; it must not silently enter delegate or
explain lowering.

## Authority and types

The parser issues a complete, parser-private structural inventory while it
parses each AST `BuildGate`. The inventory is moved into the postpass product;
it is not reconstructed from AST names or post-prune ordinals.

```text
ParserBuildGateObservationSetV1       // parser-owned, all AST gate rows
  - invocation brand
  - parser-issued gate id
  - predicate and source span
  - parser scope/context
  - optional top-level SourceBuildGatePathV1 relation

PreparedBuildGateDecisionSetV1        // private, non-Clone postpass owner
  - complete observation coverage
  - one evaluated row per postpass-visible AST BuildGate
  - selected branch: Then / Else / NoElse
  - reachability: Reachable / InactiveSubtree
  - exact predicate/source relation
  - top-level source relation receipt when available
```

`PreparedBuildGateDecisionSetV1` is parser postpass authority only. It is not
a resolver `Verified*` product, a source seal, a Recipe, or a physical plan.
Its canonical issuer consumes the same parser invocation brand, AST gate
observations, top-level source ledger, and immutable `ParserBuildConfig` once.

The structural decision coordinate is a private AST projection coordinate. It
is not declaration identity, resolver identity, inventory ordinal, or final
AST placement. Source identity remains `SourceBuildGatePathV1` and the
parser-issued source relation. Member-level parse-time gate state is not
inserted into this inventory.

`BuildGateSelectionReceiptV1` must carry the exact predicate (or an exact
predicate digest) in addition to brand, gate id, path, and selected branch.
Receipt validation rechecks the full one-to-one relation; it may not rely only
on the old cursor's pre-issuance comparison.

## Evaluation semantics

The issuer evaluates every postpass-visible AST predicate exactly once,
including predicates inside structurally inactive subtrees. This makes
unknown features and unsupported predicate keys deterministic compile errors
instead of path-dependent behavior. Syntax errors retain the existing parser
diagnostic family.

The decision set records reachability separately from evaluation. Explain
report counters preserve the existing `hakorune-build-cfg-explain-v0`
projection contract: only reachable gate rows contribute to
`conditional_group_count`, `active_branch_count`, and
`inactive_branch_count`; inactive nested rows are still evaluated once and
can fail with an unknown/malformed predicate diagnostic.

```text
all structural rows: evaluated once and retained in the decision set
reachable rows:      projected into legacy explain counters
inactive rows:       excluded from counters, never re-evaluated
```

This is an explicit language decision. It preserves existing report shape
while making predicate validation uniform. The language reference must include
the inactive-unknown diagnostic rule and its negative fixture.

Fuel is configured once before `parse_program`; decision evaluation consumes
no parser fuel and does not tokenize or parse again. Metadata remains owned by
the completed postpass envelope.

## Projection order and failure owner

The sole coordinator performs this sequence:

```text
open postpass product
  -> issue full decision set (one predicate evaluation per row)
  -> project AST prune using decision rows only
  -> project top-level source-path survival/selection receipts
  -> commit the source-session prune transaction
  -> ordinary delegate/finalizer or explicit compatibility arm
  -> project explain report from decision rows when demanded
  -> CompletedParserPostpassV1
```

The exact implementation may produce the explain report immediately after
decision issuance, but it must use the same rows and never walk AST semantics
again. The old generic recursive prune is removed from this postpass path;
the grammar-evidence nonclaim may retain its separate contract until it gets a
dedicated demand.

The following reject the whole unpublished postpass product:

```text
missing / duplicate / foreign observation row
AST/inventory predicate or structural-coordinate mismatch
missing or foreign top-level source relation
source-path survival/rebase mismatch
unknown feature or unsupported predicate key
incomplete decision coverage
predicate/selection receipt mismatch
```

All failures return the existing `ParseError` family (or one explicit typed
postpass BuildCfg diagnostic). There is no partial seal, fallback, retry,
reparse, AST/name rescan, or old whole-root helper hidden behind the owner.

## Acceptance matrix

The implementation row must cover, at minimum:

```text
positive:
  top-level selected and else gates
  nested top-level gates
  statement-level gates in active branches
  no-else gates
  Build / Feature / Target / Backend
  not / all / any

negative:
  unknown feature in an active and inactive subtree
  unsupported predicate key
  missing/duplicate/foreign gate observation
  predicate or source-path mismatch
  incomplete coverage
  exact/zero/exhausted parser fuel
  malformed source and ParseError preservation
```

For each positive case compare AST shape, explain `to_kv_lines`, metadata,
fuel behavior, and diagnostics against the existing public contract. Do not
compare source-seal internals for compatibility cohorts. The matrix must
also include direct `NyashParser::parse`, string/build-config, metadata, and
explain wrappers; grammar evidence remains an explicit nonclaim.

## Ordered implementation ladder

```text
I0-C-D0  this accepted design; no code or fixture switch
I0-C-S0  parser-private inventory + decision-set issuer and negative model tests
I0-C-S1  prune/source-path projections consume decisions; remove postpass re-eval
I0-C-S2  explain projection + public explain wrapper through shared coordinator
I0-C-S3  full parity matrix, line guard, reference/README/task closeout
```

No step may add a second public postpass owner. Keep new code in dedicated
parser modules (for example `src/parser/build_cfg/decision_set.rs` and
projection modules); `src/parser/mod.rs` and `src/parser/source_seal.rs` are
already beyond the 760-line split trigger and must not absorb the decision
set.

## Non-claims

```text
no member-level gate redesign
no grammar-evidence semantic delegate lowering
no resolver/source-seal expansion
no Recipe/Builder/MIR/runtime work
no source identity from AST names/ordinals/final placement
no fallback/retry/reparse
no production legacy retirement beyond the explain caller edge
```

## Closeout requirements

Before I0-C implementation opens, this card and the postpass SSOT must be
accepted. Each implementation slice updates, in the same commit, the parser
README, `docs/reference/language/build-conditional-gate.md`, the callable
contract reference when the public API changes, the task map, CURRENT_STATE,
focused parity tests, and the consolidated guard. All touched source files
remain below 760 lines (and never reach the 800-line hard boundary).
