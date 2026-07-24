# RAW-SOURCE0 LOWER ROOT0 — ROOTBATCH0-S0 execution task

Status: **in progress — disconnected Raw root batch owner; production consumers remain zero**  
Date: 2026-07-24  
Decision: **ROOTBATCH-prime-r1**

The design consultation is closed.  This card is the first executable
ROOTBATCH0 slice after BODY0.  It may consume the completed BODY owner and
produce an unpublished root-batch proof, but it must not wire public ingress,
drain, finalization, postprocess, external commit, or CUT0 activation.

## Decision lock

```text
Q1  RawRootBodyCompleteInvocationV1::prepare_root_batch(self) is the sole
    compiler-visible entry.  A Builder sibling consumes the paired physical
    owner; compiler-side loose tuples and re-pairing are forbidden.

Q2  RawRequiredConditionDraftV1::build() is the sole condition producer.
    It fixes condition_fn/1, one Integer parameter, Integer return, PURE,
    one-block const-1/return body, and inserted-only publication.

Q3  RawRootBatchSlotV1 is the identity SSOT.  Main is symbol "main", arity 0;
    RequiredCondition is symbol "condition_fn", arity 1.  "main/0" and
    "condition_fn/1" are diagnostics only; signature names never contain /N.

Q4  Preparation is fully mutation-free.  It never calls ledger.reserve().
    A prepared two-slot ledger plan and collector replacement plan are
    co-sealed before private infallible commit.

Q5  Raw requires Main + condition_fn.  Main uses whole-pair replacement;
    condition_fn is CanonicalRejectDuplicate.  Optional/Forbidden and caller
    policy selection do not enter this terminal.

Q6  Every failure retains the complete BODY0 owner and exact nested cause.
    Rejection exposes inspection and discard(self) only.  No retry, fallback,
    downgrade, into_parts, or typed panic claim is added.

Q7  Success returns RawRootBatchCompleteInvocationV1::{Script, App}.  It owns
    the session, post-batch physical carrier, body witness, helper receipts,
    callable-Main outcome, Main/condition receipts, and sealed Raw ledger.
    Its only future terminal is DRAIN0.
```

## S0a — identity contract

Add one Raw root slot contract and make BODY0's root skeleton consume it.

```text
Main:
  key      = FunctionDraftKeyV1::Main
  symbol   = "main"
  arity    = 0
  policy   = LegacyReplaceWholePair
  role     = RootMain

RequiredCondition:
  key      = FunctionDraftKeyV1::SyntheticConditionFn
  symbol   = "condition_fn"
  arity    = 1
  policy   = CanonicalRejectDuplicate
  role     = SyntheticConditionFn
```

The BODY0 producer must emit `MirFunction.signature.name == "main"`, not
`"main/0"`.  `Main.main/0` remains the separate source callable identity.
Post-hoc string rewriting is forbidden.

## S0b — typed condition factory

Add `RawRequiredConditionDraftV1::build()` in a Builder sibling module.  No
caller supplies a `MirFunction`, body, policy, or condition option.

The factory must prove:

```text
signature.name       = "condition_fn"
parameters           = exactly one Integer
return type          = Integer
effects              = PURE
basic blocks          = exactly one entry block
body                  = one Integer constant 1 followed by Return(const 1)
calls / source AST    = zero
entry metadata        = false
```

Implementation note (2026-07-24): S0a and S0b are now landed as disconnected
Builder vocabulary, with the borrow-only two-slot ledger plan vocabulary
prepared for S0c.  The focused identity/body/factory tests and `cargo check
--lib` are green.  No compiler or production consumer is connected; S0c
remains the next active implementation gate.

## S0c — mutation-free prepare

The Builder terminal borrows the complete physical owner and constructs all
plans before consuming anything:

```text
brand/family/session/physical agreement
Main draft identity and condition factory contract
shell has no published root function
collector index and existing helper/callable receipts
ledger is clean, unpoisoned, and has no open reservation
next ordinal can represent two reservations
callable-Main event/disposition is sealable
collector Main replacement == ledger Main replacement
condition key/symbol is absent
```

`RawExpansionReceiptLedgerV1::reserve()` is forbidden during prepare.  A
prepared pair owns both future ordinals and the post-pair ordinal.  The
collector replacement plan and ledger replacement disposition are co-sealed.

## S0d — private commit and consuming handoff

After preparation succeeds, a private infallible commit performs the only
mutation:

```text
materialize the two reserved slots
collector commit for Main + condition_fn
obtain branded receipts
validate both receipts together
record both ledger events together
seal the ledger
issue RawInvocationRootWitnessV1
return route-specific RawRootBatchCompleteInvocationV1
```

No `Result` branch, callback, retry, fallback, or observer is allowed after
the prepared product is issued.

## Failure owner

```rust
RejectedRawRootBatchInvocationV1 {
    complete_body_owner,
    stage,
    nested_error,
    prepared_facts,
}
```

Preflight failure retains the unchanged BODY0 owner.  Any later failure must
retain the unpublished session/physical carrier, root draft, completed body,
condition draft, helper receipts, callable-Main evidence, and exact cause.
Only `stage(&self)`, `error(&self)`, and `discard(self)` are public.

## Required fixtures

```text
identity: BODY0 main signature is "main" and source Main.main/0 remains distinct
condition: exact factory shape and no caller-provided MirFunction
success: empty Script, scalar Script, App Main.main/0
success: helper receipts plus callable-Main Selected/NotSelected evidence
replacement: absent Main, exact whole-pair Main replacement
failure: main/0 drift, foreign brand/family, dirty shell, dirty collector index
failure: existing condition key/symbol, poisoned/open ledger, ordinal overflow
failure: collector/ledger Main disposition mismatch
atomicity: every prepare failure leaves collector and ledger unchanged
one-shot: retry/fallback/second entry/loose parts = zero
handoff: shell published root count = 0; DRAIN0 consumer = 0
```

## Non-claims

```text
BODY0 recipe/lowering semantics       = 0
legacy MainPending widening           = 0
drain/finalization/postprocess        = 0
external commit/public ingress        = 0
JSON behavior/CUT0 activation         = 0
production consumer                    = 0
```

## Internal order

```text
ROOTIDENTITY0 -> CONDITION0 -> PREPARE0 -> I0 -> G0
```

All modified source/check files must remain below 800 lines.  The first
production consumer remains forbidden until a later atomic cutover decision.
