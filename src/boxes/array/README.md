# Array runtime owner

Selected append-only owners use `owned_slots.rs` in the SAME `ArrayStateCell`
and the exclusive `ArrayStorage::OwnedSlots` representation. It carries disjoint
typed profiles: indexed object Homes use `ArrayOwnedSlotDescriptor`; opaque
Dynamic carriers use `ArrayCarrierSlotDescriptor`. The latter has no object End
thunk, Text-copy behavior, or read API. Both are physical profiles, not source
ownership authorization. Claims accept only an empty, uncontracted builtin
Array; primitive/boxed mutation and materialization cannot reinterpret these
slots. Generic `invoke_surface` rejects this storage through its existing error
owner. Representation diagnostics and length inspection do not lend a source
borrow or End capability.

Object append validates the live descriptor and unique non-null identity,
reserves before commit, and changes ownership only on Normal. Null occupies a
slot with no child Home. Read checks Live/SAME descriptor and lends the nullable
identity without clone, retain or a new Home. Object End changes Live to Ending
and detaches the successful slots under the state lock. The kernel caller leaves
the host registry closure before invoking reverse-order child End, continues
after returned Fault, then marks Ended and releases the Array residence.
Ending/Ended never revive.

Dynamic carrier append validates and atomically adopts a caller-owned opaque
token, reserves before commit, and stores it only on success. Its ArrayBox API
detaches the committed token sequence under the state lock and finishes the
profile after the checked kernel owner has attempted token End. Compiler source
and physical operation selection remain pending; this storage API does not
activate source or MIR use.
Reserve failure is returned before commit; other allocation/registry failures
retain their existing fatal policy. Compiler descriptor/final bindings remain
required before this preparation can authorize a production Array consumer.

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
