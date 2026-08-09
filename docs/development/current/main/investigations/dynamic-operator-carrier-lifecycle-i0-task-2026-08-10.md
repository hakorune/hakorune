# DYNAMIC-OPERATOR-CARRIER-LIFECYCLE-I0

Status: ordered after `DYNAMIC-OPERATOR-EXECUTION-CONTRACT-I0`
Date: 2026-08-10

## Change

Consume the complete existing Dynamic invocation-lifecycle program once and
co-seal exactly two private, non-Clone operator-result rows.

```text
V9:
  I5 DynamicAdd -> V9
  exact I6 argument ordinal 1
  BorrowedNoEscapeForInvocation
  end after the I6 Normal/Fault outcome

V17:
  I15 DynamicAdd -> V17
  exact I16 WriteBinding(B0,V17)
  exact JoinSig Backedge(B0=V17)
  forward at the later rebind commit
```

Only borrow-scoped views leave the wrapper. The issuer accepts no caller-
supplied item, value, binding, source site, lifecycle, operator contract, or
JoinSig.

## Acceptance

- V9 wrong consumer/ordinal, early end, false forward, and missing Fault-path
  cleanup authorization reject;
- V17 wrong binding/value/backedge, early end, Fault rebind, and application of
  the iteration-local `ch` rule reject;
- I15 Fault leaves B0 unchanged and publishes no V17;
- old B0 end/install order remains unclaimed;
- no Home, cleanup execution, CFG/MIR, Completion, retry, or fallback.
