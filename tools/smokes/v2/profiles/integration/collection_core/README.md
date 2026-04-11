# collection_core smoke bridge family

Collection-owned MapBox witnesses while `vm_hako_caps/mapbox/*` is being
retired from the vm-hako capability bucket.

The 7 live rows now execute from this directory directly. They still reuse
`vm_hako_caps_common.sh` from `vm_hako_caps/lib/` so the runtime route and
contract checks stay identical while ownership is moved.

Retirement order:

- first, keep `collection-core.txt` pointed at this family
- next, keep the 7 live rows here as the owner home
- then archive the 6 non-live vm_hako mapbox rows
- finally, retire this bridge after LLVM-side collection/runtime-data coverage
  replaces it
