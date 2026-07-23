# CUT0-I0 ROOT0-DRAIN0-PHYSICAL0 境界相談

Status: **Closed — Q1-1/Q2-1/Q3-1/Q4-1 selected; BRIDGE0 is next**

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

## Decision closeout — 2026-07-23

Q1–Q4 are closed as follows:

```text
Q1 physical row vocabulary = neutral src/mir contract
Q2 source handoff          = consuming CanonicalDrainManifestV1::into_physical(self)
Q3 collector extraction    = keyed Builder child terminal in module_draft_collector/drain.rs
Q4 wrapper unpack          = narrow consuming into_parts plus Builder sibling module
```

The neutral contract carries only invocation brand, invocation family,
canonical semantic identity, physical symbol, arity, and a type-sealed
canonical inserted disposition. It does not carry `FunctionDraftKeyV1`,
generic publication policy, `ModuleInvocationPolicyV1`, source headers, or
callable-catalog references. The compiler manifest remains compiler-private;
only its consuming conversion emits the neutral product.

Preparation and drain are separate one-shot phases. Preparation is
mutation-free and validates all brand, family, shell-empty, keyed collector,
receipt, cardinality, identity, symbol, arity, policy, and replacement
invariants. Only a prepared product may perform the infallible consuming
drain. The old symbol-only drain owner is not widened.

Before PHYSICAL0 wiring, capability mismatches remain distinguishable from
foreign-brand mismatches: brand disagreement reports `ForeignBrand`, while
capability brand/family disagreement reports `CapabilityMismatch`.

The executable rows are intentionally split:

```text
PHYSICAL0-BRIDGE0  neutral physical manifest + compiler-private conversion
PHYSICAL0-COLLECT0 keyed collector prepare/drain + wrapper into_parts
PHYSICAL0-PREP0    mutation-free physical preflight + rejected-owner proof
PHYSICAL0-I0       completion-owned prepare_drain + infallible drain
PHYSICAL0-P0/G0    four-route fixtures, failure matrix, census guard
```

No production drain, finalizer, external commit, fallback, retry, Raw
convergence, or atomic CUT0 activation is authorized by this closeout.
