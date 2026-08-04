---
Status: Superseded historical decision and task order
Date: 2026-07-26
Decision: LANGUAGE-TRYLESS-POSTFIX-CATCH-prime-r1 superseded on 2026-08-05
Superseded-by: LANGUAGE-RESULT-EXIT-C-PRIME0-D0
Supersedes: LANGUAGE-DOCS-TRY-CATCH-D1 unresolved consultation
Authority-prose closeout: LANGUAGE-DOCS-POSTFIX-CATCH-D1-CLOSEOUT closed 2026-07-26; implementation authority remains parked
Scope: remove source `try`, give postfix `catch` one protected-region owner, and keep terminal Fault non-catchable
ceremony_tier: T2 semantic target and grammar/profile boundary; no Outcome implementation
sunset_id: none for the durable decision; compatibility surfaces use the three stable sunset IDs below
proof_inventory_before: unresolved status-index rows + five-Outcome semantic kernel + Compat2025 source-try parser + gated TryCatch-shaped transport
new_proofs: one accepted target matrix; no executable semantic or implementation proof
retired_or_merged_proofs: the unresolved D1 consultation is superseded; physical conflict sentinels remain until closeout
net_proof_delta: 0 executable proofs
sunset_budget: 0; no new per-row shell guard
sunset_row: LANGUAGE-DOCS-POSTFIX-CATCH-D1-CLOSEOUT closed the D1 conflict-scaffolding prose
retire_when: accepted target matrix synchronized without claiming parser/runtime/backend activation (closed 2026-07-26)
budget_repayment_evidence: current-state pointer guard + existing docs/status and grammar/profile guards
Related:
  - docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md
  - docs/development/current/main/investigations/language-docs-try-catch-d1-consultation-2026-07-25.md
  - docs/reference/language/status-index.md
  - docs/reference/language/semantic-kernel.md
  - docs/reference/language/failure-outcome-relations.md
  - docs/reference/language/scope-exit-semantics.md
  - docs/reference/language/function-exit-and-entry-result.md
  - docs/reference/language/lifecycle.md
  - docs/reference/language/grammar-contract.md
  - docs/reference/language/EBNF.md
Resume-after-closeout:
  - NORMAL-FILE-VM0-FRONTDOOR-FORGE0-S0
---

# Tryless postfix catch task order

> Historical notice: this protected-region target is no longer executable
> authority. The accepted v1 target is Result-only recoverable failure,
> typed postfix `?` for unchanged propagation, no source catch or
> `RecoverableFailure` Outcome, and one standalone `cleanup {}` surface. See
> `language-result-propagation-and-exit-transaction-ssot.md`. The queue below
> remains provenance for migration census and old-edge retirement only.

## Correction being sealed

Hakorune does not use a source `try` statement. A postfix `catch` marks the
immediately preceding expression, block, or member body as the protected
region:

The examples below are accepted target spelling, not a claim about the current
parser/profile implementation:

```hako
work() catch (error) {
    recover(error)
}

{
    step1()
    step2()
} catch (error) {
    recover(error)
}
```

This keeps the protected body at the current indentation level. Adding a
compatibility language profile that accepts `try { ... }` would restore the
extra nesting that the syntax deliberately removed. It is therefore not an
accepted compatibility spelling.

The earlier pasted candidate `LANGUAGE-TRY-CATCH-CLEANUP-prime-r1` is
superseded before implementation. Its statement-`try` alias/transport choice
and compatibility-only postfix-catch choice are not executable authority.

## Accepted target vocabulary

The exact target semantic variant name is:

```text
OutcomeV1::RecoverableFailure(RecoverableFailureV1)
```

It is distinct from all of:

```text
Result::Err(E)       = an ordinary source value
CompatFailureV1     = historical compatibility carrier
Fault(FaultReason)  = terminal and non-catchable
```

The target protected-region transition is:

```text
protected Normal(value)              -> Normal(value)
protected Return(value_or_unit)      -> Return(value_or_unit)
protected Break / Continue           -> unchanged
protected RecoverableFailure(reason) -> run the postfix catch handler once
protected Fault(reason)              -> bypass catch; Fault remains terminal
```

The exact handler-result and outer-protected-region law is intentionally not
invented here; it is a required output of `LANGUAGE-RECOVERABLE-FAILURE-D0`.
At minimum, no implementation may make a handler recursively catch its own
failure or silently convert it to `Result::Err`, Unit, or terminal `Fault`.

Cleanup is a separate pending-outcome channel. A terminal Fault bypasses catch
but still drains cleanup. The current cleanup kernel law remains: all cleanup
rows run in LIFO order, and the first cleanup Fault replaces the pending
protected/body Fault. Whether cleanup may produce `RecoverableFailure` is
unresolved and must be fixed by D0; cleanup never catches itself.

## Accepted surface matrix

| Surface | Grammar target | Availability target | Semantic owner | Current implementation claim |
| --- | --- | --- | --- | --- |
| statement `try` | rejected in Canonical and Compat2025 | prohibited | grammar contract only | current Compat parser acceptance is legacy drift |
| postfix `catch` | canonical in Canonical and Compat2025 | pending until the Outcome/producer/runtime rows close | `ProtectedRegionOutcomeV1` over `RecoverableFailure` | current `TryCatch` AST/MIR is transport evidence only |
| `cleanup` standalone/local/postfix | canonical | pending/guarded by exact backend capability | semantic kernel cleanup law + scope-exit owner | no broad parser/backend activation claim |
| scope `fini {}` / `local ... fini` | Canonical rejected; Compat2025 alias only | transport, later guarded alias | normalize once to canonical cleanup | current CatchClause marker is legacy representation |
| `box.fini()` | canonical | lifecycle capability guarded | object lifecycle SSOT | never normalized to scope cleanup |
| source `throw` | rejected in Canonical and Compat2025 | prohibited | grammar contract only | generated/internal Throw shapes are not source permission |

Stable source rejection tags remain:

```text
[freeze:contract][parser/try_reserved]
[freeze:contract][parser/throw_reserved]
```

No parser may retry Canonical input as Compat2025. A migration tool may read
historical `try` spelling into a non-semantic migration record, but that tool
is outside both language grammar profiles and may not enter canonical AST,
MirBuilder, MIR, VM, LLVM, or process-result routes.

## Required semantic stop before implementation

The accepted surface requires a catchable Outcome, but the current repository
has no canonical producer or callable/entry ABI for it. Do not infer one from
`Result::Err`, `ASTNode::TryCatch`, `MirInstruction::Catch`, JSON Result mode,
generated once-property poison handling, or Builder fallback gates.

Before registry, parser, lowering, or runtime activation, close:

```text
LANGUAGE-RECOVERABLE-FAILURE-D0
```

That one design row must select:

```text
1. exact source/runtime operations allowed to produce RecoverableFailureV1
2. payload identity, ownership, and diagnostic projection
3. propagation across call, method, Script, source-entry, and process boundaries
4. unhandled-boundary behavior; silent Unit/Result/Fault conversion is forbidden
5. handler parameter binding, legal control outcomes, handler result relation,
   outer protected-region propagation, and self-recursion prevention
6. cleanup ordering for protected, handler, and outward-propagated outcomes,
   including whether cleanup may issue RecoverableFailure
7. exact supported backend set and pre-effect rejection boundary
```

Until that row closes:

```text
canonical RecoverableFailure producer = 0
canonical catch runtime consumer       = 0
postfix catch availability             = pending
unsupported route                      = typed pre-effect rejection
```

## Docs-only closeout record

```text
LANGUAGE-DOCS-POSTFIX-CATCH-D1-CLOSEOUT
```

This was a short preemption of the accepted NormalFile Forge row. It recorded
the accepted target without pretending the physical registry/parser/runtime
already matches it.

Internal order:

```text
CLOSEOUT-DECISION0
  accepted matrix, supersession, and no-source-try rationale

CLOSEOUT-OUTCOME0
  semantic-kernel / failure-outcome-relations
  name RecoverableFailure and ProtectedRegion target
  keep producer and cross-boundary activation pending
  function-exit-and-entry-result receives only a pending decision pointer;
  it does not project RecoverableFailure before D0 selects the boundary ABI

CLOSEOUT-SCOPE0
  scope-exit / lifecycle
  cleanup independent, scope fini alias, box.fini separate

CLOSEOUT-GRAMMAR0
  grammar-contract / EBNF prose
  target profiles, current drift, no backend no-op wording
  registry/parser behavior delta = 0

CLOSEOUT-ENTRY0
  status-index / language README / quick-reference / stage-profiles /
  20-Decisions
  target status and current availability remain separate

CLOSEOUT-G0
  current-state pointer guard
  source/registry/parser/runtime/backend diff = 0
  no new one-off shell guard
```

The status index uses an explicit `authority_sync_pending` note and retains
the conflict sentinel with this decision link until physical registry and
parser rows are synchronized. It must not label the feature live merely
because the target decision is accepted.

Closeout result: completed on 2026-07-26 with authority/docs changes only;
registry, parser, AST, MIR, runtime, backend, JSON, and environment-gate
behavior remain unchanged. The active execution row now resumes exactly:

```text
NORMAL-FILE-VM0-FRONTDOOR-FORGE0-S0
```

The Forge task remains accepted and unmodified in scope. This language
closeout neither completes nor cancels it.

## Parked implementation order

The language implementation queue is recorded now but is not the current
execution authority:

```text
LANGUAGE-EXCEPTION-CLEANUP-SURFACE-CENSUS0-P0
  classify Rust/Hako parser, AST, macro, JSON, MIR, generated poison, gates,
  fixtures, and historical consumers by canonical/compat/tool/test status;
  this read-only product is the mandatory D0 input

-> LANGUAGE-RECOVERABLE-FAILURE-D0
  exact producer, payload, propagation, top-level boundary, cleanup relation

-> LANGUAGE-CLEANUP-AST-SPLIT0-D0
-> LANGUAGE-CLEANUP-AST-SPLIT0-S0
  canonical cleanup/protected-region shapes
  historical exception migration transport
  no cleanup/fini encoding through CatchClause

-> LANGUAGE-GRAMMAR-PROFILE-EXPLICIT0-I0
  explicit GrammarProfileV1 at every parser entrance
  ambient semantic profile owner zero
  Canonical-to-Compat retry zero

-> LANGUAGE-EXCEPTION-CLEANUP-REGISTRY0-S0
  source try reject in both profiles
  postfix catch + exact cleanup rows
  scope fini Compat alias
  throw reject row
  registry/corpus only; runtime delta zero

-> LANGUAGE-TRY-SYNTAX-MIGRATION-TRANSPORT0-I0
  external migration tool only
  language parser / AST / MIR / runtime consumers zero

-> LANGUAGE-LEGACY-EXCEPTION-FENCE0-S0
  body-only try fallback, Throw-to-trace fallback, JSON catch-ignore,
  canonical MIR Catch/Throw producers zero
  must close before a new ProtectedRegion runtime consumer

-> LANGUAGE-PROTECTED-REGION-OUTCOME0-S0
  postfix catch over RecoverableFailure
  Fault bypass
  disconnected semantic/lowering proof first

-> LANGUAGE-SCOPE-CLEANUP-OUTCOME0-D0
-> LANGUAGE-SCOPE-CLEANUP-VMREF0-S0
  pending Outcome storage, LIFO cleanup, Fault precedence, VM-reference slice

-> LANGUAGE-EXCEPTION-CLEANUP-BACKEND-PARITY0
  required backend set or explicit fail-fast

-> LANGUAGE-TRY-COMPAT-PARSER-RETIRE0-S0
-> LANGUAGE-FINI-ALIAS-RETIRE0-S0
-> LANGUAGE-STAGE3-ENV-GATES-RETIRE0-S0
-> LANGUAGE-EXCEPTION-CLEANUP-G0
```

No parser/registry/runtime task above may start merely because this queue
exists. `CURRENT_STATE.toml` must select it explicitly after the Forge or a
later priority decision.

## Compatibility sunsets

```text
sunset_id =
  LANGUAGE-TRY-COMPAT-PARSER-SUNSET-001

owner =
  LANGUAGE-TRY-COMPAT-PARSER-RETIRE0

retire_when =
  Canonical and Compat language parser source-try producers zero
  + registry positive source-try fixtures zero
  + generated grammar projections agree

external migration tool =
  durable non-language lane; not counted as a language parser consumer

sunset_id =
  LANGUAGE-FINI-SCOPE-ALIAS-SUNSET-001

owner =
  LANGUAGE-FINI-ALIAS-RETIRE0

retire_when =
  canonical cleanup parser/runtime capability green for the exact supported
  backend/route set named by the activation row
  + generated/current source fini-alias producer zero
  + production fini-alias consumer zero

promotion =
  forbidden; this is a migration alias with a bounded retention period, not a
  second durable canonical spelling

sunset_id =
  LANGUAGE-STAGE3-ENV-GATES-SUNSET-001

owner =
  LANGUAGE-STAGE3-ENV-GATES-RETIRE0

retire_when =
  every parser entrance owns explicit GrammarProfileV1
  + ambient environment semantic selection zero
  + Rust/Hako profile witness parity green
```

These sunsets do not include `box.fini()`.

## Structural laws for the eventual implementation

The counts in this section are eventual target invariants, not current
implementation evidence:

```text
source try producer                              = 0
source throw producer                            = 0
canonical postfix catch syntax producer target   = 1

RecoverableFailure semantic owner target         = 1
RecoverableFailure current producer              = 0
canonical catch current runtime consumer          = 0
Result::Err -> RecoverableFailure implicit lift  = 0
Fault catchability                               = 0
handler self-recursion                           = 0

canonical cleanup owner                          = 1
scope fini independent semantic owner            = 0
box.fini -> scope cleanup normalization           = 0

cleanup/fini encoded through CatchClause          = 0
canonical semantic TryCatch owner                 = 0

unsupported backend no-op                        = 0
Builder body-only try fallback                    = 0
Throw-to-trace semantic fallback                  = 0
JSON catch-ignore canonical path                  = 0

explicit grammar profile owner                    = 1
ambient environment semantic owner                = 0
fallback/retry                                    = 0
```

## Non-claims

```text
RecoverableFailure producer selected
cross-call or top-level failure ABI selected
parser/registry/AST/MIR/runtime/backend activation
source try compatibility acceptance
throw activation
cleanup runtime activation
box.fini backend promotion
normal-file Forge completion
normal/default entry cutover
JSON / Program(JSON v0) widening
REPL / executor / selfhost / fastmem
LLVM/native change
CUT0
```
