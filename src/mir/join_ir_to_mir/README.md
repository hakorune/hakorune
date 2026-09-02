# JoinIR-to-MIR conversion

This module owns exactly one transformation:

```text
Structured JoinModule -> MirModule
```

It does not execute MIR, choose VM routes, read bridge environment flags, or
own process exit behavior. No production consumer remains in this bridge; its
current callers are test-only. The source pipeline and VM runner are separate
owners, and this compatibility boundary does not re-enter canonical publication.
