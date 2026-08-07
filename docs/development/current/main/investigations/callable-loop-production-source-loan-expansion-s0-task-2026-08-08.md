# Callable Loop Production Source-Loan Expansion S0

Status: closed implementation row
`CALLABLE-LOOP-PRODUCTION-PREPARED-INGRESS-D0` (2026-08-08).

## Change

Extend the existing normal callable semantic loan so one bounded, move-only
source receipt can carry the exact resolver-backed function input and ledger
view to the future logical/prepared ingress. Preserve the existing raw host
behavior; this row does not activate a physicalizer or switch a production
caller.

## Contract

```text
VerifiedNormalCallableSemanticLoanV1
  -> one source-loan expansion receipt
       CallableSemanticSourceLedgerView
       ResolvedFunctionLoweringInputV1
       owner / source / function / forest / projection identity
       frame / scope identity checked through existing ledger evidence
```

Use the existing forest, projection, function site, and resolver ledger. Issue
`ResolvedFunctionLoweringInputV1` exactly once from those owners. Do not AST
re-walk, resolve by name/arity, synthesize a header, create a new semantic
owner, import Builder/physicalizer state, or change raw lowering behavior.

`VerifiedCallableFunctionLoweringInputV1` in
`loop_physical_prepare.rs` remains a `cfg(test)` static-header canary helper;
it is not promoted. Callable index/header and target ABI are optional companion
receipts for profiles that require them, not common source-receipt fields.

## Done

- one non-Clone receipt is issued from the existing loan owner;
- input/ledger owner, source, function, forest, and projection identities are
  checked before any Builder effect;
- common profile succeeds without an index/header companion;
- missing/foreign loan selection, duplicate/unconsumed loan accounting, and
  fresh-request reuse remain fail-fast at the existing source/loan owners;
- raw selected/legacy parity is unchanged;
- focused tests, rustfmt/check, current-state pointer guard, and diff hygiene
  are green;
- implementation closeout updates the relevant `docs/reference/**` contract
  only if a reference claim changes, plus diagnostics, migration note, guards,
  current mirrors, and task pointers in the same commit.

## Closeout

`CALLABLE-LOOP-PRODUCTION-SOURCE-LOAN-EXPANSION-S0` is closed by the source
receipt implementation and focused resolver-site test. The receipt is
non-`Clone`, move-only, and borrows the existing forest/projection owners with
the program and row lifetimes kept separate. The raw host still consumes the
old `(lineage, lowering_state)` pair and therefore has no behavior or parity
change. The next bounded row is prepared-ingress assembly; index/header
companions remain profile-specific and unimplemented here.

## Stop

Do not issue `PreparedCallableLoopPhysicalizationV1`, emit CFG/SSA/PHI/MIR,
open a physical session, select a route, switch a production caller, enable
Generic G0, add retry/fallback, or delete legacy edges. If an exact input or
identity cannot be issued from existing owners, return `NoSafeSlice` and
reopen design; do not add a resolver or AST adapter.
