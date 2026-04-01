# lang/src/runtime/substrate/osvm — OS VM Capability Staging

Responsibilities:
- Current home for the reserve-only first truthful `hako.osvm` row, which is already landed.
- Future home for:
  - page reserve
  - page commit
  - page decommit
  - OS VM capability-facing vocabulary

Rules:
- `osvm` is capability substrate, not semantic owner.
- Keep this directory focused on the narrow OS virtual-memory seam.

Current live subset:
- `osvm_core_box.hako`
  - `reserve_bytes_i64(len_bytes)`
  - already-landed reserve-only row over `hako_osvm_reserve_bytes_i64`

Non-goals:
- No `commit/decommit` API in this wave.
- No allocator policy here.
- No final OS VM rewrite here.
- The reserve-only row is the complete current pilot; `commit/decommit/page_size` stay parked.
