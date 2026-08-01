# JoinIR-to-MIR conversion

This module owns exactly one transformation:

```text
Structured JoinModule -> MirModule
```

It does not execute MIR, choose VM routes, read bridge environment flags, or
own process exit behavior. `NormalizationExecuteBox` is its current production
consumer; the ordinary VM runner remains the sole MIR execution owner.
