# Array runtime owner

`ArrayStateCell` in `mod.rs` owns storage and the optional element contract under
one `RwLock`. Shared aliases retain this state; clone/slice create fresh state
according to their existing contracts. No handle-keyed contract table is needed.

`runtime_contract.rs` owns claim/adoption and exact numeric element validation.
A claim audits existing elements before installation, accepts the same contract
idempotently and rejects a conflicting contract without replacing it.

`ops/store.rs::slot_store_{i64,bool,f64}_result` is the sole primitive store
implementation. Each operation preserves index, unsupported-storage and element
contract errors, including the existing numeric rejection reason. Storage and
contract checks precede mutation under the same write lock. Negative indices
keep the existing OOB observation. Append at the exact end, overwrite and
storage conversion retain their existing behavior.

The public `slot_store_*_raw` boolean methods delegate to these crate-visible
Result methods. Compatibility projection alone discards the error. Boxed/text
and read-modify-write operations retain their own existing owners; this slice
adds neither a second mutation implementation nor checked kernel exports.

Returned validation rejection leaves storage and length unchanged. Allocation
failure in Arc/Box/Vec or registry growth is not a returned error contract yet.
The checked runtime/C prerequisites remain in the
[collection construction SSOT](../../../docs/development/current/main/design/collection-literal-construction-ssot.md#checked-array-runtime-premise-audit-and-task-order).
