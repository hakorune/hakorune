# CUT0-I0 ROOT0-DRAIN0-PHYSICAL0 境界相談

Status: **DESIGN-STOP — neutral physical-manifest seam must be selected before implementation**

Related:

- `cut0-i0-root0-drain0-execution-task-2026-07-23.md`
- `cut0-i0-root0-drain0-design-question-2026-07-23.md`
- `CURRENT_STATE.toml`

## Context

`ROOT0-DRAIN0-MANIFEST0` is closed. The compiler now owns a source-derived
`CanonicalDrainManifestV1` containing exact semantic identity, symbol, arity,
and a canonical inserted seal. `PHYSICAL0` must validate those rows against
the Builder collector, receipts, and empty shell before any mutation.

The manifest currently lives under `src/mir/compiler`. Builder already owns
the neutral `src/mir` vocabulary, while compiler depends on Builder. A
physical terminal importing the compiler manifest would invert that layer
boundary and create a compiler/Builder ownership cycle. The old symbol-only
`ModuleLoweringShellDrainInventoryV1` and `ModuleLoweringInvocationDrainOwnerV1`
are explicitly not allowed to become the bridge.

The physical terminal also needs keyed collector extraction. The source
manifest is ordered by canonical callable key, while collector maps and
receipt admissions have their own order. Ordinal pairing is not a proof.

## Questions to decide

### Q1 — Where is the physical row vocabulary owned?

1. **Neutral shared physical contract (recommended)**

   Add a small `src/mir` product for the physical handoff. The compiler
   consumes its source manifest and emits this neutral product; Builder
   consumes only that product. It carries brand, family, semantic identity
   (`FunctionOwnerIdV1` or `CanonicalCallableKeyV1`), symbol, arity, and a
   type-sealed canonical inserted disposition.

2. Builder-owned adapter. This preserves dependency direction but risks
   exposing `FunctionDraftKeyV1` as a cross-layer authority.

3. Compiler manifest imported by Builder. **Reject:** wrong dependency
   direction and source authority leakage.

### Q2 — How does the source manifest cross the boundary?

1. **Compiler-owned consuming conversion terminal (recommended)**

   `CanonicalDrainManifestV1::into_physical(self)` consumes the source product
   and produces the neutral physical product. The conversion maps canonical
   identities without exposing generic caller-authored keys, symbols, arities,
   or policies.

2. A generic row enum added to the old collector. **Reject:** widens the
   legacy collector API and mixes source authority with admission policy.

3. A string-only symbol list. **Reject:** loses semantic identity and cannot
   prove key/receipt parity.

### Q3 — What is the collector extraction terminal?

1. **A new Builder child terminal keyed by the neutral manifest (recommended)**

   `module_draft_collector/drain.rs` preflights each row by semantic key,
   symbol, arity, policy, replacement, and collector brand, then consumes
   drafts in manifest order. The near-limit parent collector stays below 800
   lines.

2. Extending `into_draft_functions()`. **Reject:** its BTreeMap-value output
   does not encode the manifest-key proof.

3. Reusing the old module drain owner. **Reject:** it accepts caller-authored
   inventory and legacy condition/Main policy.

### Q4 — How are physical wrappers unpacked?

1. **Narrow consuming `into_parts` methods plus a new sibling physical module**
   (recommended). Collected wrappers keep fields private while one
   Builder-internal terminal receives shell, collector, and exact receipt by
   value.

2. Public fields on `module_invocation_brand0.rs`. **Reject:** multiple
   physical unpack authorities.

3. Expanding the near-limit completion file. **Reject:** keep preflight/drain
   vocabulary in a small module.

### Q5 — Required mutation-free checks

Before preparing any shell drain, the terminal must prove:

```text
neutral manifest brand == shell/collector/receipt brand
collector payload receipt_brand == Some(brand)
single receipt collector_brand == Some(brand)
every callable admission collector_brand == Some(brand)
shell function map is empty
manifest cardinality == collector cardinality == receipt cardinality
semantic identity, symbol, arity, CanonicalRejectDuplicate, and Inserted exact
missing/surplus/duplicate row = 0
```

Only then may a prepared physical product be issued. Its sole consuming
`drain(self)` terminal may move keyed drafts into the shell and return an
opaque branded drained module plus receipt/inventory evidence.

## Stop line

Do not implement PHYSICAL0, add compiler-to-Builder imports, widen the old
shell drain inventory, or connect production consumers until Q1–Q4 are
selected. Raw, finalization, external commit, retry, fallback, and atomic
CUT0 remain out of scope.
