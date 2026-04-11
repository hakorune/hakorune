# collection_core smoke bridge family

Collection-owned MapBox witnesses while `vm_hako_caps/mapbox/*` is being
retired from the vm-hako capability bucket.

The 4 remaining live rows now execute from this directory directly. They still reuse
`vm_hako_caps_common.sh` from `vm_hako_caps/lib/` so the runtime route and
contract checks stay identical while ownership is moved.

`MapBox.clear`, `MapBox.delete`, and `MapBox.keys` now have non-vm_hako
emit+exec owners under
`tools/smokes/v2/profiles/integration/phase29y/hako/emit_mir/` and no longer
live in `collection-core.txt`.

Retirement order:

- first, keep `collection-core.txt` pointed at this family
- next, keep the remaining 4 live rows here as the owner home
- then archive the 6 non-live vm_hako mapbox rows
- `clear` / `delete` / `keys` already moved to non-vm_hako emit+exec owners
- finally, retire this bridge after LLVM-side collection/runtime-data coverage
  replaces `set` / `get` / `has` / `size`
