---
Status: accepted execution plan; behavior change 0
Date: 2026-07-26
Decision: OWNERSHIP-SPARSE-RESUME-D0-GUARD-REFRESH0
Classification: BoxShape receipt refresh
Prerequisite: MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
Next semantic row after closeout: OWN-GRAM-REJECT0
---

# Ownership sparse-resume D0: receipt refresh

## Decision

`OWNERSHIP-SPARSE-RESUME-D0` is closed as an inventory result. The ownership
language contract, SharedV1 fence, production ownership caller-zero claim, and
canonical-core prerequisite have not drifted. The existing proof harness cannot
currently prove every prerequisite because its receipt anchors predate the F1
draft-seal and normal-callable physical split.

The next row is therefore a behavior-neutral guard refresh, not ownership
syntax activation:

```text
OWNERSHIP-SPARSE-RESUME-D0-GUARD-REFRESH0
  -> OWN-GRAM-REJECT0
```

## D0 inventory

| Boundary | Result | Required response |
| --- | --- | --- |
| Canonical Script/Main/helper reference lane | green | retain as prerequisite |
| `BindingRef -> ValueId` | existing Binding SSA authority | retain one-owner guard |
| Ownership SSA | passive/verified receipts exist; canonical source caller remains zero | retain caller-zero proof |
| `: view T` / `: share T` | currently coalesces into type text | `OWN-GRAM-REJECT0` must reject before grammar activation |
| SharedV1 | named compatibility lane; sparse route fallback remains forbidden | retain fence |
| SSA/ownership receipt guards | stale paths, manifests, and lexical anchors | refresh structurally before activation |

No D0 evidence authorizes a new parser production, AST carrier, resolver,
Loan Flow, MIR opcode producer, runtime rule, backend capability, product
route, or default caller.

## Exact stale receipts

### Binding SSA receipts

The following inventory anchors moved during the accepted F1/draft-seal and
PHI extraction; they are not removed semantic responsibilities:

```text
phi.batch-prepend
phi.canonical-if-final
publication.function-draft-finalize
publication.function-session-commit
return.canonical-lower-arm
return.implicit-void-finalization
```

`SSA-I1-T` also has two new type-preparation files and now closes the function
through `ReadyFunctionDraftSealV1`; its old fixed manifest and old direct
finalization anchors are no longer authoritative.

### Callable catalog fence

The CAT0 guard currently forbids the substring `FunctionSyntaxViewV1`, which
now falsely matches the constrained `EmbeddedCallableFunctionSyntaxViewV1`
normal-source-plan seam. The refresh must replace that lexical prohibition with
an exact direct-import/use fence and a bounded allowlist for the one embedded
function forwarding seam. Body traversal remains exclusively in the existing
callable-resolution owner.

### Ownership receipts

```text
origin.receiver anchor:
  moved to capability/function_role_policy.rs; receiver rejection remains
  separately defended by capability product-shape checks

legacy ReleaseStrong ledger:
  37 newly indexed historical/test/check/tool paths, zero unclassified
  production source paths; regenerate deterministic counts only

trivial-owner profile:
  analyzer split is harmless, but its one historical entry became four named
  current profile entries. Preserve V1 as historical and issue a versioned
  current profile receipt; do not edit a one-entry claim into a false summary.
```

## Series

This is one BoxShape series with no accepted source shape. Each commit stays
buildable and all touched source/check files stay below 800 lines.

```text
1. OWNERSHIP-SSA-RECEIPT-ANCHOR0-S0
   Repoint six Binding SSA receipt anchors and the I1-T manifest/lifecycle
   receipt to their exact PHI/draft-seal owners.

2. OWNERSHIP-CALLABLE-FENCE0-S0
   Replace the CAT0 substring false-positive with an exact structural fence
   and its one named embedded-function seam.

3. OWNERSHIP-PROFILE-RECEIPT0-S0
   Refresh the receiver multi-anchor, deterministic legacy ledger, and
   versioned current trivial-owner profile receipt.

4. OWNERSHIP-SPARSE-RESUME-D0-G0
   Run the complete prerequisite receipts and either issue readiness or retain
   one exact inventory blocker. Only a green closeout unlocks OWN-GRAM-REJECT0.
```

## Structural law

```text
accepted grammar delta                 = 0
AST / resolver / MIR / runtime delta   = 0
production ownership caller             = 0
SharedV1 retry from sparse route        = 0
second BindingRef -> ValueId map        = 0
second ownership event authority        = 0
receipt anchor weakening                = 0
historical V1 claim rewritten as current= 0
all modified/new source/check files     < 800 lines
```

## Follow-up: OWN-GRAM-REJECT0

After the refresh is green, the first semantic slice remains deliberately
narrow:

```text
reject static/instance result lookalikes `: view T` and `: share T`
with [freeze:contract][parser/ownership_syntax_inactive]

preserve ordinary calls view(...)/share(...), local identifiers named
view/share, and literal type names where unambiguous
```

Rust and Hako parser paths close in two buildable commits. No ownership form
becomes parser-live in that row.
