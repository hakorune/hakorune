# DYNAMIC-CARRIER-REBIND-TRANSACTION-D0

Status: parked design stop after operator-result lifecycle I0
Date: 2026-08-10

## Question

Select one atomic transaction for replacing the current B0 carrier with V17:

```text
evaluate I15 while old B0 remains current
Fault  -> V17 absent; B0 unchanged
Normal -> authorize I16
commit V17 as current B0 exactly once
return displaced B0 obligation for exact end
publish JoinSig Backedge(B0=V17)
```

The Decision must fix failure ordering, displaced-carrier end ordering,
primary-Fault preservation, and the boundary between semantic rebind and
physical cleanup. It must reason from B0 prior-current lineage, never from V15
last use.

## Hard stops

```text
no implementation before Decision accepted
no guessed last-use end
no V17 install on Fault
no duplicate old-B0 end
no Home inference from Dynamic
no physical CFG/PHI/ValueId authority
no retry/fallback
```
