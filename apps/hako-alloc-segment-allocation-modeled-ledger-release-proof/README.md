# hako-alloc-segment-allocation-modeled-ledger-release-proof

MIMAP-097A proof app for the scalar modeled segment allocation ledger release
route.

The app proves:

- live modeled allocation tokens can be marked released in the ledger;
- duplicate release, missing token, invalid token, and unsupported substrate
  reasons remain distinct;
- live-token lookup no longer returns released rows;
- real segment free, raw pointer, segment-map, arena, atomic bitmap,
  OSVM/page-source, thread, provider, and backend matcher seams stay inactive.
