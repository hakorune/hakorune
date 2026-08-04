---
Status: Durable language design SSOT; accepted target, production activation 0
Decision: LANGUAGE-RESULT-EXIT-C-PRIME0-D0 accepted on 2026-08-05
Scope: Result-only recoverable failure, postfix propagation, lexical cleanup, and one verified exit transaction.
Related:
  - docs/reference/language/failure-outcome-relations.md
  - docs/reference/language/semantic-kernel.md
  - docs/reference/language/scope-exit-semantics.md
  - docs/reference/language/function-exit-and-entry-result.md
  - docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md
  - docs/development/current/main/workstreams/language-v1-convergence-current.md
---

# Result Propagation and Exit Transaction SSOT

## Decision

Hakorune v1 uses one value-level recoverable-failure family and one explicit
unchanged-propagation marker.

```text
recoverable failure = Result<T, E>
unchanged propagation = postfix expr?
absence = Option<T> + guard let / match
local recovery or error conversion = guard let / match
lexical exit action = standalone cleanup { ... }
terminal failure = Fault
```

The following are not Canonical v1 language authorities:

```text
source try
source throw
source catch
RecoverableFailure Outcome
Option ?
implicit error conversion
user-defined Try / residual protocol
local or postfix cleanup sugar
scope fini cleanup alias
```

This Decision explicitly supersedes
`LANGUAGE-TRYLESS-POSTFIX-CATCH-prime-r1` as a target language decision. That
document remains historical evidence for the rejected protected-region route.
It does not authorize implementation retirement while this workstream remains
parked.

## Source surface

```hako
load(path): Result<Data, IoError> {
    local file = File.open(path)?
    local data = file.read()?
    return Result::Ok(data)
}
```

`?` is a typed postfix control operator, not parser text substitution and not
dynamic method dispatch. Its v1 admission rule is exact:

```text
operand type = Result<T, E>
enclosing callable result = Result<U, E>
error identity = exact E
```

On `Ok(T)`, the selected payload becomes the expression result. On `Err(E)`,
the exact error payload is forwarded into `Result::Err(E)` for the enclosing
callable and a pending Return is created. The operand is evaluated exactly
once. No clone, hidden `share`, fallback, or implicit conversion is permitted.

`Option<T>` does not use `?` in v1. `None` is absence rather than failure, so
its early-exit policy stays visible through `guard let` or `match`.

The canonical grammar gives postfix `?` one token authority in the postfix
chain, including shapes such as `open(path)?.read()?`. C-style ternary
`cond ? a : b` is not a Canonical v1 spelling; its real callers must be
censused and migrated to the selected `if`/`match` form before cutover.

## Verified propagation plan

The bootstrap parser may preserve the source node and exact source site. The
selfhost semantic layer owns all meaning and must seal a passive product
before Builder effects:

```text
VerifiedResultPropagationPlanV1
  source site
  operand Result<T,E> contract
  enclosing Result<U,E> contract
  Ok payload Home relation
  Err payload Home-forward relation
  protected exit-carrier receipt
  exact cleanup/exit transaction receipt
```

The sole physical consumer lowers this plan to ordinary enum projection,
branching, and the common pending-Return path. A QMark-specific MIR opcode,
runtime registry, dynamic `isOk/getValue` protocol, or direct lowerer-owned
`Return` is forbidden.

## One exit transaction

All normal and early exits use one verified owner:

```text
1. evaluate the exit expression or ? operand exactly once
2. move the outgoing value/Home into a protected pending carrier
3. cross lexical scopes from inner to outer
   a. run registered cleanup bodies in LIFO order
   b. release non-forwarded local Homes in reverse declaration order
4. let each Home release enter the object lifecycle DropPlan when terminal
5. if no Fault replaced the pending outcome, publish it once
6. otherwise release the unpublished pending value and publish Fault once
```

The protected return Home supports handles used by cleanup but cannot be
re-consumed, rebound, or destroyed by cleanup. The exit transaction emits Home
release requests; it never duplicates field-order or user-`fini` policy owned
by the lifecycle descriptor.

Fault ordering is causal:

```text
first Fault in time = primary
later cleanup/finalization Faults = suppressed diagnostics
ordinary pending Return or Result::Err + teardown Fault = terminal Fault
```

This precedence supersedes any earlier target where a later cleanup Fault
blindly replaced an already-pending body Fault. Teardown still continues on a
best-effort basis after the primary Fault is sealed.

## Cleanup surface

The sole canonical registration spelling is:

```hako
cleanup {
    transaction.rollbackUnlessCommitted()
}
```

It registers on reaching the statement, runs exactly once for every exit that
crosses the lexical scope, and is LIFO within that scope. A cleanup body may
not issue `return`, `break`, `continue`, `?`, suspension, or another outward
control outcome. A fallible domain operation must be handled explicitly with
`match` or replaced by an explicitly best-effort ordinary method.

The following must retire atomically with their selected carriers:

```text
local x = e cleanup { ... }
expr cleanup { ... }
fini { ... }                 # historical scope alias
local x = e fini { ... }
handler_tail catch/cleanup
TryCatch/CatchClause cleanup encoding
ambient exception/cleanup environment gates
```

Box-member `fini { ... }` is a separate terminal-Home hook owned by the C′
lifecycle SSOT. It is not a scope handler.

## Current implementation boundary

The accepted target is not live today:

```text
typed Result ? production consumer = 0
verified exit transaction production consumer = 0
canonical standalone-cleanup physical owner = 0
Home production activation = 0
```

The current dynamic QMark route, arbitrary-object QMark fixture, TryCatch-based
cleanup transport, postfix catch target, and compatibility gates are migration
evidence only. Unsupported target syntax must reject before Builder effects;
there is no Canonical-to-Compat retry.

## Task family

Use one responsibility family rather than a document per constructor:

```text
LANGUAGE-RESULT-EXIT-C-PRIME0-D0   # this accepted Decision
-> LANGUAGE-RESULT-EXIT-C-PRIME0-P0
-> LANGUAGE-RESULT-EXIT-C-PRIME0-I0  # Trivial payload/local profile
-> LANGUAGE-RESULT-EXIT-C-PRIME0-R0
-> LANGUAGE-RESULT-EXIT-C-PRIME0-HOME0-I0    # Unique/owning-field profile
-> LANGUAGE-RESULT-EXIT-C-PRIME0-HOME0-I0/S  # Shared profile
-> LANGUAGE-RESULT-EXIT-C-PRIME0-DOC0
```

`P0` is a bounded read-only deletion census. It classifies every producer,
carrier, consumer, gate, fixture, and real source caller across parser, AST,
Program JSON, MIR, VM/EXE/AOT, runtime, environment configuration, grammar,
and docs for:

```text
dynamic/arbitrary QMark and C-style ternary ? :
try / throw / catch / RecoverableFailure
handler tails and TryCatch/CatchClause carriers
standalone/local/postfix cleanup and scope-fini aliases
compatibility retry and ambient semantic gates
```

Every row receives exactly one migration disposition: keep as the selected C′
owner, migrate to Result/guard/match/standalone cleanup, delete, or reject
before Builder effects. Unknown is a blocker; a raw token count is not closure.

`I0` orders cleanup-specific AST/plan extraction, passive
`VerifiedExitTransactionV1`, passive `VerifiedResultPropagationPlanV1`, one
exact typed Result consumer, and VM/EXE/AOT parity or pre-effect rejection.
Its first physical profile admits only Trivial Result/error payloads and
Trivial locals. Any Home-bearing payload, pending carrier, or local rejects
before effects; this cell may not claim Home release/finalization parity.

`R0` removes the dynamic ResultBox QMark route, arbitrary-object QMark,
try/throw/catch/RecoverableFailure producers and consumers, handler-tail and
TryCatch cleanup encoding, non-standalone cleanup spellings, scope-fini aliases,
ambient gates, implicit compatibility retry, and any ternary `? :` producer or
parser ambiguity left by the census.

`I0` must also prove the Trivial exit chronology matrix, not only the happy
Result branch:

```text
body Fault -> later cleanup Fault = body Fault primary
pending Return/Result::Err -> cleanup Fault = Fault primary
later teardown Fault = suppressed diagnostic
remaining cleanup = attempted best effort
published pending value after Fault = 0
```

`LANGUAGE-RESULT-EXIT-C-PRIME0-HOME0-I0` is the later **Unique and
owning-field-only** Home integration cell. It may start only after:

```text
OWN-HOME-RELATION0-S0
OWN-HOME-ABI0-S0
OWN-HOME-FLOW-CFG0-S0
OWN-TERMINAL-HOME-DROP-PLAN0-S0
OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/U
OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/F
```

It activates protected outgoing Unique Home carriers, reverse non-forwarded
local Home release, verified owning-field teardown, terminal lifecycle
handoff, and body/cleanup/fini Fault chronology. It proves Unique payload/error
Home transfer exactly once, pending value destruction on Fault, later
teardown-Fault suppression, and remaining local/field/native teardown best
effort. Shared Home payloads, errors, pending carriers, and release routes still
reject before effects; this cell must not claim Shared parity.

`LANGUAGE-RESULT-EXIT-C-PRIME0-HOME0-I0/S` is the separate Shared integration
cell. It starts only after the Unique/field cell and all of:

```text
OWN-HOME-SHARE0-I0
OWN-TERMINAL-HOME-DROP-PLAN0-S0/S
OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/S
```

It activates Shared Home payload/error propagation and Shared owner release in
the same verified exit transaction, including non-last release with zero hook
dispatch and terminal-winner hook dispatch exactly once. Until the applicable
Home cell lands, that Home-bearing `?`/cleanup route remains a typed pre-effect
rejection rather than a fallback.

## Mandatory implementation-after reference closeout

`LANGUAGE-RESULT-EXIT-C-PRIME0-DOC0` is mandatory after `P0`, `I0`, `R0`,
`HOME0-I0`, `HOME0-I0/S`, and backend parity. It must update the
implementation-backed reference truth, not merely repeat this target:

```text
required receipt: LANGUAGE-FAILURE-REFERENCE-CLOSEOUT0-DOC0
required receipt: LANGUAGE-CLEANUP-REFERENCE-CLOSEOUT0-DOC0
```

```text
docs/reference/language/EBNF.md
grammar/language-v1-registry.toml
grammar/language-v1-grammar-contract-corpus/**
docs/reference/language/grammar-contract.md
docs/reference/language/status-index.md
language profile/support matrix reference
docs/reference/language/option.md
docs/reference/language/failure-outcome-relations.md
docs/reference/language/semantic-kernel.md
docs/reference/language/scope-exit-semantics.md
docs/reference/language/function-exit-and-entry-result.md
docs/reference/language/LANGUAGE_REFERENCE_2025.md
accepted/rejected examples and migration redirects
```

Completion requires registry, Rust parser, Hako parser, EBNF, verified
products, backend capability, diagnostics, and examples to agree. Reference
pages must not claim the target live before those witnesses land.

## Non-claims

- no parser, AST, MIR, runtime, backend, or source migration change;
- no Option propagation sugar;
- no implicit error conversion;
- no catchable Fault;
- no current-lane change from MirBuilder;
- no permission to activate Home before its own readiness gates.
