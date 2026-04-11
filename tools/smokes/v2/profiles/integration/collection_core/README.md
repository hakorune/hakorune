# collection_core smoke bridge family

Bridge witnesses owned by `collection-core.txt` while `vm_hako_caps/mapbox/*`
is being retired from the vm-hako capability bucket.

These wrappers intentionally keep the current vm-hako implementation alive
without letting `collection-core.txt` point at `vm_hako_caps/` directly.

Retirement order:

- first, keep `collection-core.txt` pointed at this family
- next, move the real mapbox rows here
- then archive the non-live vm_hako mapbox rows
- finally, retire this bridge after LLVM-side collection/runtime-data coverage
  replaces it
