# Array runtime owner

`ArrayStateCell` in `mod.rs` owns storage and the optional element contract under
one `RwLock`. Shared aliases retain this state; clone/slice create fresh state
according to their existing contracts. No handle-keyed contract table is needed.

`runtime_contract.rs` owns claim/adoption and exact numeric element validation.
A claim audits existing elements before installation, accepts the same contract
idempotently and rejects a conflicting contract without replacing it.

`ops/store.rs::store_{i64,bool,f64}_locked` owns primitive mutation.
Indexed Result stores and Result appends acquire one state lock and reuse it.
Each operation preserves index, unsupported-storage and element contract errors, including the existing numeric rejection reason. Storage and
contract checks precede mutation under the same write lock. Negative indices
keep the existing OOB observation. Append at the exact end, overwrite and
storage conversion retain their existing behavior. Dedicated append determines the
end position, validates, writes and returns the committed length under that lock;
concurrent shared aliases cannot overwrite another append's committed element.

The public `slot_store_*_raw` boolean methods delegate to crate-visible indexed
Result methods; `slot_append_*_raw` projects the committed length or zero.
The existing claim and primitive Result append methods plus their two error
enums are the public runtime surface used by checked kernel exports. The
implementation module remains crate-private; no second mutation wrapper or
compiler receipt is introduced.
Compatibility projection alone discards the error. Boxed/text
and read-modify-write operations retain their own existing owners; this slice
adds no second mutation implementation.

Returned validation rejection leaves storage and length unchanged. Allocation
failure in Arc/Box/Vec or registry growth follows the selected native fatal-OOM
policy, separately from returned contract errors. Claim adoption of noninteger
storage (including InlineRecord) retains ExistingElementMismatch; primitive
append's UnsupportedStorage is a distinct mutation capability error.
The checked runtime/C prerequisites remain in the
[collection construction SSOT](../../../docs/development/current/main/design/collection-literal-construction-ssot.md#checked-array-runtime-premise-audit-and-task-order).
