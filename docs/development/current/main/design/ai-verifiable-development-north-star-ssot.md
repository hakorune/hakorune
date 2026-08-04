---
Status: SSOT
Decision: AI-VERIFIABLE-DEVELOPMENT-NORTH-STAR0-D0 accepted
Date: 2026-08-05
Scope: Long-term product and compiler direction for converging on correct high-performance Hakorune programs with minimal verified iteration.
Related:
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/development/current/main/design/language-minimal-surface-ssot.md
  - docs/development/current/main/design/ownership-home-model-ssot.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# AI-verifiable development North Star

## Decision

Hakorune does not optimize primarily for the shortest grammar or the largest
set of expressible programs. Its long-term product goal is:

> Minimize the number and cost of verified iterations required to converge on
> correct, high-performance code.

The compiler is the semantic partner in that loop. Source generation,
interactive editing, human review, and automated agents must be able to ask
the compiler for typed facts, receive stable rejection reasons, apply only
capability-backed repairs, and verify the result again.

This is a durable design-selection policy. It does not activate a language
feature, tool API, current work lane, or implementation task by itself.

## Product hypothesis

Rust remains the practical default for existing systems while its ecosystem,
libraries, IDEs, and compiler feedback are already mature. Hakorune does not
need to win that installed-base comparison. Its long-term opportunity is new
or closed systems where a smaller canonical surface, explicit ownership and
failure boundaries, stable diagnostics, and fast compiler queries reduce the
number of repair iterations needed by a human or an AI. This is a hypothesis
to measure after the compiler, standard library, tooling, and selfhost corpus
are real; it is not a claim that an AI will always prefer Hakorune or that
grammar simplicity alone is sufficient.

## Success model

```text
source intent
-> canonical source surface
-> Resolve
-> Observe
-> Facts
-> Recipe
-> Verify
-> Lower
-> stable diagnostic or verified product
-> bounded repair
-> verify again
```

The winning design is the one that reduces ambiguity and diagnostic distance
across this loop while preserving performance and explicit semantics. A short
spelling that creates hidden control, fallback, ownership, cost, or authority
is a regression against this goal.

## Constitutional laws

### 1. One concept, one Canonical surface

- one source spelling owns one semantic responsibility;
- compatibility and historical inputs never become silent retry routes;
- equivalent convenience spellings require measured convergence value and one
  early verified expansion owner;
- ordinary data access, calls, ownership transfer, failure propagation, and
  lifecycle hooks remain visibly distinct.

### 2. One directional compiler authority

`Resolve -> Observe -> Facts -> Recipe -> Verify -> Lower` is the semantic
direction. Lowering does not rediscover policy, and runtime/backend code does
not infer source meaning from names, dynamic tags, or fallback behavior.

Every accepted plan is branded and non-forgeable. Every rejection names the
earliest owner that has enough information to decide it.

### 3. Stable machine-readable diagnostics

A diagnostic is a product, not only prose. Its stable core should be able to
carry:

```text
reason code
source site
semantic owner
relevant binding/type/Home/callable identities
first invalid edge
required capability
available repair classes
support/profile boundary
```

Human text may improve without changing the reason identity. A diagnostic may
suggest a repair only when the compiler has proved that the required syntax
and capability exist. Guessing, by-name repair, and hidden `share` or fallback
are forbidden.

### 4. Read-only semantic query surface

The long-term tool surface should answer bounded structural questions such as:

```text
What is this expression's sealed type?
Which binding and Home root does this use?
What Home ABI does this call require and return?
Which fact or Recipe admitted/rejected this construct?
Where was this Home consumed?
Which backend capability blocks this plan?
Which verified repair classes are available?
```

Queries are projections of sealed compiler products. They cannot become a
second resolver, verifier, policy engine, or mutable back door into compiler
state. Unknown facts produce a typed unknown/rejection, never a plausible
guess.

### 5. Repairs are verified proposals

A repair candidate contains its preconditions, affected source sites, semantic
owner, and expected postcondition. Applying it does not make it correct; the
normal compiler pipeline must verify the changed program again.

```text
diagnostic
-> capability-filtered repair proposal
-> explicit source edit
-> fresh Resolve/Facts/Recipe/Verify
-> success or a new exact rejection
```

Automatic textual rewrites, AST-equivalence guesses, and retry-until-green
loops are outside the contract.

### 6. Feedback latency is part of correctness usability

Fast checking matters because a precise answer that arrives too late still
increases convergence cost. Performance work must measure the exact active
front and keep correctness products reusable without making stale facts
authoritative.

Incremental or cached answers must carry source/profile/schema/dependency
fingerprints. A mismatch invalidates the answer; it never falls back to a
weaker semantic route.

### 7. Corpus and tools complete the language

The grammar alone is not an AI-verifiable development environment. The product
requires, in bounded order:

- a large selfhost corpus and representative applications;
- positive and negative fixtures for every admitted boundary;
- stable formatting and language-server navigation;
- standard libraries with explicit capability and ownership contracts;
- queryable Facts/Recipe/diagnostic products;
- fast check/build paths and backend parity receipts.

Missing ecosystem support remains an explicit product gap. It must not be
hidden by claiming that a small grammar alone makes the language easier.

## Measurement contract

No single metric selects the design. Establish a baseline before setting a
numeric target, then track at least:

```text
verified edit/check iterations to green
time to first owning diagnostic
stable reason-code rate across equivalent failures
diagnostic unknown/ambiguous rate
semantic-query coverage of sealed compiler products
repair proposals rejected for missing capability
repair proposal fresh-verification success rate
check latency at the exact active front
silent fallback / semantic retry count = 0
```

Benchmark fixtures must include successful programs and intentionally invalid
programs. Measuring only compile throughput does not measure convergence.

## Design review filter

Every substantial language/compiler/tool proposal should answer:

```text
Does this reduce or increase the number of semantic authorities?
Can the compiler identify the exact owning fact and rejection edge?
Can a tool query the result without reimplementing policy?
Are repair suggestions capability-backed and freshly reverified?
Does the change preserve explicit cost, ownership, and control flow?
What exact-front latency or convergence metric can regress?
Which compatibility/fallback authority is deleted?
```

If these answers are unknown, the proposal remains a design stop rather than
entering implementation.

## Capability ladder, not a current task queue

Future work should be selected in dependency order, without opening a parallel
lane from this document:

```text
AI-DEVLOOP-BASELINE0-P0
-> AI-DIAGNOSTIC-PRODUCT0-S0
-> AI-SEMANTIC-QUERY0-D0-S0
-> AI-VERIFIED-REPAIR0-D0-S0
-> AI-FAST-CHECK0-G0
-> AI-DEVLOOP-CORPUS-TOOLING0-G0
-> AI-DEVLOOP-REFERENCE-CLOSEOUT0-DOC0
```

- `BASELINE0-P0` measures current diagnostic distance, iteration count, and
  latency without adding production behavior.
- `DIAGNOSTIC-PRODUCT0-S0` seals stable reason/source/owner/capability data
  before changing human presentation.
- `SEMANTIC-QUERY0-D0-S0` selects a read-only schema and proves that every
  answer projects an existing sealed product.
- `VERIFIED-REPAIR0-D0-S0` admits only repair classes backed by an implemented
  capability and fresh compiler verification.
- `FAST-CHECK0-G0` establishes exact-front latency and cache-fingerprint gates.
- `CORPUS-TOOLING0-G0` closes formatter, navigation, fixtures, library
  contracts, and representative-project evidence.

Each row requires explicit selection by `CURRENT_STATE.toml`. None is active
while the current MirBuilder design/execution stop owns the lane.

## Mandatory implementation-after reference closeout

`AI-DEVLOOP-REFERENCE-CLOSEOUT0-DOC0` is mandatory only after selected APIs,
diagnostic products, tools, and gates have landed. It updates implementation-
backed reference pages, schemas, examples, and tool documentation from actual
behavior. This North Star cannot satisfy that receipt.

Before that closeout, reference documentation must distinguish:

```text
accepted long-term policy
implemented compiler product
supported tool/query surface
measured convergence result
```

No target-only semantic query, repair, latency, or ecosystem capability may be
presented as live support.

## Non-goals and non-claims

- no claim that Hakorune replaces Rust for existing ecosystems;
- no claim that an AI will always prefer Hakorune;
- no language-feature activation from projected AI convenience alone;
- no hidden control, ownership, allocation, retry, or backend fallback;
- no second compiler policy implementation inside an LSP or agent service;
- no tracing-GC, concurrency, or cross-backend capability claim not proved by
  its owning Decision and implementation receipt;
- no update to the current active lane.

The durable competitive goal is not “write everything Rust can write.” It is:

> Make the correct high-performance solution narrow, observable, and
> mechanically verifiable with the fewest semantic detours.
