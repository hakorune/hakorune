# Callable Loop Production Source-Loan Expansion S0

Status: next implementation row after accepted
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
- requiring-profile companion mismatch returns typed `NoSafeSlice`;
- missing, foreign, duplicate, borrowed/unconsumed, and fresh-request reuse
  cases are covered;
- raw selected/legacy parity is unchanged;
- focused tests, rustfmt/check, current-state guard, and replacement guard are
  green;
- implementation closeout updates the relevant `docs/reference/**` contract
  only if a reference claim changes, plus diagnostics, migration note, guards,
  current mirrors, and task pointers in the same commit.

## Stop

Do not issue `PreparedCallableLoopPhysicalizationV1`, emit CFG/SSA/PHI/MIR,
open a physical session, select a route, switch a production caller, enable
Generic G0, add retry/fallback, or delete legacy edges. If an exact input or
identity cannot be issued from existing owners, return `NoSafeSlice` and
reopen design; do not add a resolver or AST adapter.
