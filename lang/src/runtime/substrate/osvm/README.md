# lang/src/runtime/substrate/osvm — OS VM Capability Staging

Responsibilities:
- Current home for the first truthful `hako.osvm` rows, which are already landed.
- Live rows:
  - page reserve
  - page commit
  - page decommit
- Future home for:
  - page_size vocabulary
  - OS VM capability-facing vocabulary

Rules:
- `osvm` is capability substrate, not semantic owner.
- Keep this directory focused on the narrow OS virtual-memory seam.

Current live subset:
- `osvm_core_box.hako`
  - `reserve_bytes_i64(len_bytes)`
  - `commit_bytes_i64(base, len_bytes)`
  - `decommit_bytes_i64(base, len_bytes)`
  - already-landed rows over `hako_osvm_reserve_bytes_i64` / `hako_osvm_commit_bytes_i64` / `hako_osvm_decommit_bytes_i64`

Non-goals:
- No `page_size` API in this wave.
- No allocator policy here.
- No final OS VM rewrite here.
- The reserve/commit/decommit rows are the complete current pilot; `page_size` stays parked.
