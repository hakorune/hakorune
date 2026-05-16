# mimalloc-facade-known-page-loop-proof

Proof app for MIMAP-038A.

It verifies that `HakoAllocObjectLifecycleFacade.objectLifecycleKnownPageIndexById`
walks all known pages instead of the old fixed three-page shape. The app adds
four pages, finds the fourth page by id, and releases a block through the
facade using that fourth-page lookup.

Stop lines:

- no page-source / OSVM calls
- no provider activation, hooks, or host allocator replacement
- no backend matcher shortcut
