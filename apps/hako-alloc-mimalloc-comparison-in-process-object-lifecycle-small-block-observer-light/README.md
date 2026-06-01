# Object Lifecycle Small Block Observer-Light Comparison

This app is a comparison-only workload for the mimalloc current workstream.

It keeps the same representative small-block operation sequence as
`hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof`,
but replaces the public object-lifecycle facade with an app-local
observer-light facade.

Purpose:

- estimate the cost of facade/queue/result publication that C does not perform
- keep production `HakoAllocObjectLifecycleFacade` semantics unchanged
- keep `HakoAllocPageModel` page-local behavior unchanged
- avoid provider activation, allocator replacement, hooks, and global allocator

This app is not a product API and must not be used as the public allocator
facade.
