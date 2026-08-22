# Host-handle runtime owner

`host_handles.rs` owns the process-wide slot table. Child modules project
bounded responsibilities without creating another registry.

## Text formal call lifetime

`call_lifetime.rs` is the sole mechanical owner of invocation-wide Text
formal pins and pin-aware slot retirement.

```text
published {slot, generation} pairs
  -> atomic exact-Text/generation/retirement preflight
  -> grouped per-occurrence pin commit
  -> opaque move-only lease-set token
  -> explicit whole-set finish
```

One pin means one callee invocation times one ExactText formal occurrence.
Passing the same pair in two formal positions adds two pins. A nested callee
entry acquires its own set; merely forwarding the pair adds no pin.

The raw drop path and generation-matched Dynamic lease drop both use the same
retirement terminal. An unpinned slot is removed immediately. A pinned slot
becomes `Pending`, retains its payload and generation, and is removed exactly
once when the last pin finishes. Only actual removal recycles the slot and
increments `DROP_EPOCH`.

`text_formal_call_lease.rs` is the caller-zero Rust façade. Its token is
non-Clone, has no public constructor, and finishes only through an explicit
consuming method. Empty lease sets are rejected; a no-Text signature uses a
separate future no-lease disposition.

## ExactText residence

`text_formal_residence.rs` consumes already-published physical pairs and
reuses the same call-lifetime transaction to produce one move-only residence
owner. The owner keeps occurrence order, projects only exact concrete Text
payloads (`StableText` or the built-in `StringBox`), and exposes a root
descriptor through a scoped closure; the runtime token and raw pointer are
never independently returned. The published-lane adapter validates the
entry `{slot,generation}` view once and immediately hands it to this same
all-pairs transaction; it is not a second pin owner.

```text
published {slot,generation} pairs
  -> all-or-nothing exact-Text validation + pin
  -> private frame header + one root row per formal occurrence
  -> explicit consuming finish
```

This caller-zero substrate intentionally rejects `StableBox`/virtual
`as_str_fast`, spoofed runtime names, frame overflow, stale or retiring
pairs, partial output, and any compiler/session/loop consumer. It is not a
callable actualizer, MIR residence, TextEq route, or production ABI.

## Authority boundary

This runtime owner validates slot liveness, generation, exact Text class,
pin multiplicity, and retirement. It does not own:

- source bindings or actual-origin proof;
- logical or physical callable signatures;
- MIR lanes, `ValueId`, CFG, SSA, PHI, or Completion;
- canonical Trap translation;
- TextEq, Substring, route choice, fallback, or retry.

Those compiler/session connections remain closed until their own authority
rows issue exact, same-cohort products. The current runtime API has no C or
production caller.
