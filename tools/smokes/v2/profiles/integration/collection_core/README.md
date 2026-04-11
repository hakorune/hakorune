# collection_core smoke bridge family

Historical bridge family for the `mapbox -> collection-core` re-home.

All live MapBox owner rows are retired from `collection-core.txt` now.
The dedicated non-vm_hako owners live under
`tools/smokes/v2/profiles/integration/phase29y/hako/emit_mir/`, and the old
bridge scripts are archived under
`tools/smokes/v2/profiles/archive/collection_core/`.

Retirement order:

- bridge rows moved out of `collection-core.txt`
- all 7 bridge scripts are archived
- runtime collection ownership now lives in non-vm_hako emit+exec families plus
  the broader ring1/runtime-data suites
