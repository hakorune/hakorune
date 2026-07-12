# Decoded UTF-8 Byte Length Contract V0

Status: accepted design / implementation task order
Decision: B-led hybrid
Scope: RHako/HHako `BoundedBodyAnalysisSnapshotV0` text-budget parity

## Semantic authority

```text
DecodedUtf8ByteLenV0(value)
  = the number of octets produced by encoding the decoded Unicode scalar
    sequence in UTF-8, without normalization, BOM insertion, or C-string
    termination.
```

The decoded value is primary. A stored `byte_count` is only a checked derived
witness. Neither a Rust implementation, an Hako implementation, a carrier
field, nor a public method spelling owns the meaning.

Required laws:

```text
reads NYASH_STR_CP = 0
aliases length/len/size = 0
normalization = none
U+0000 byte length = 1
checked result >= 0
actual <= limit = accepted
actual > limit = Unsupported
```

RHako and HHako share only the operation ID, laws, limits, `TextClassV0`,
reason codes, and fixture expectations. They independently own ProgramV0
traversal, path construction, text-class selection, budget accumulation,
outcomes, builder lifecycle, and snapshot reconstruction.

## Accepted hybrid

```text
semantic operation:
  DecodedUtf8ByteLenV0

initial executable leaf:
  environment-independent analysis/internal capability

local carrier in each implementation:
  ValidatedTextV0(value, byte_count, text_class)

Rust normalized carrier -> HHako:
  replay-only/non-authority; forbidden in the direct parity proof
```

The first slice does not add a Stable public `String.len_bytes()` method.
Current reference documentation names `ByteCursorBox.len_bytes()`; its MVP
placeholder and all existing `String.length/len/size` routes are non-authority.

## Execution support boundary

```text
Rust reference VM / HHako parity lane:
  supported in the first slice

LLVM/object, PyVM, Wasm:
  capability preflight rejection before reader effects
```

There is no VM fallback. If a future parity gate executes on another backend,
that backend becomes a prerequisite and must receive an explicit byte-safe
implementation first. A future LLVM route uses a dedicated handle symbol such
as `nyash.string.len_bytes_h`; it must not reuse `nyash.any.length_h`,
`nyrt_string_length`, raw pointers, `CStr`, or `strlen`.

## Durable task order

### U0 — Contract and capability inventory (complete)

- freeze this semantic contract and operation ID;
- inventory the reference-VM callable seam and every backend preflight owner;
- classify current `String.length`, `ByteCursorBox.len_bytes`, length corridors,
  C-string routes, and PyVM behavior as non-authority;
- name the internal carrier-integrity failure outcome without mapping it to
  user `InvalidInput`.

U0 closeout inventory:

```text
RHako semantic leaf:
  src/analysis/bounded_body_snapshot_v0/decoded_utf8_byte_len_v0.rs
  crate-private DecodedUtf8ByteLenV0(&str) -> usize

future HHako internal call spelling:
  hako.analysis.decoded_utf8_byte_len_v0(text)
  Callee::Extern only; no String method, slot, alias, or public ABI row

reference execution route:
  Callee::Extern
  -> MirInterpreter::execute_extern_function
  -> analysis leaf

HHako preflight:
  reference VM extern allowlist only

product backend gate:
  MIR metadata -> decoded_utf8_byte_len_backend_capability
  -> shared BackendPreflight -> backend entry rejection
```

Non-authority inventory:

```text
StringBox.length / len / size
StringBox method slot 300
NYASH_STR_CP and string_codepoint_mode
Rust VM String fast/slow dispatch
ByteCursorBox.len_bytes MVP placeholder
nyash.string.len_h and nyash.any.length_h
nyrt_string_length / raw pointer / CStr / strlen
LLVM string corridors
generic HostFacade / hostbridge / hako.intrin
Rust normalized-carrier replay input
```

Existing fail-fast owners are `src/mir/backend_capability.rs` plus the LLVM,
PyVM, and Wasm backend entries. Capability failure is a backend/runner error
before reader effects and has no `PathV0`; it must never be translated to
reader `Unsupported` or `InvalidInput`. Carrier witness mismatch is separately
named `InternalCarrierContractViolation`.

### U1 — Environment-independent reference leaf (complete)

- implement the smallest analysis/internal executable leaf;
- count decoded Rust strings by `value.as_bytes().len()`;
- do not register a Stable public String surface or alias;
- prove no dependency on `NYASH_STR_CP`, codepoint mode, generic length routes,
  or C-string APIs.

U1 closeout:

- `DecodedUtf8ByteLenV0::count` is crate-private under the SnapshotV0 analysis
  module and has exactly one implementation: `value.as_bytes().len()`;
- RHako `ValidatedTextV0` construction and SnapshotV0 budget accounting both
  consume that leaf;
- contract fixtures cover ASCII, three- and four-byte scalars, combining versus
  precomposed text, embedded NUL, and decoded multibyte text;
- the focused suite runs with `NYASH_STR_CP` unset and with `NYASH_STR_CP=1`;
- a dedicated guard rejects mode/config, public string surface, generic length,
  C-string, and legacy helper dependencies.

### U2 — HHako internal capability and preflight (complete)

- expose only the internal `decoded_utf8_byte_len_v0` capability to the HHako
  parity lane;
- make unsupported backends fail before reader effects;
- keep capability failure distinct from reader `Unsupported`/`InvalidInput`.

U2 closeout:

- the only HHako-facing spelling is
  `hako.analysis.decoded_utf8_byte_len_v0` through `Callee::Extern`; it has no
  aliases, String method slot, or public ABI registration;
- the extern route publishes one `scalar_i64` result with `string_handle`
  demand, and the explicit-extern builder preserves that integer result type;
- the Rust reference execution lane dispatches directly to
  `DecodedUtf8ByteLenV0::count`, including embedded NUL, without invoking a
  String length route or hostbridge;
- `BackendPreflight` rebuilds the declarative extern-route metadata before the
  shared capability gate reads it; LLVM/object, PyVM, and Wasm reject the
  capability before reader effects, while `mir-interpreter` remains the sole
  supported first-slice consumer;
- the HHako wrapper and fixture are green with `NYASH_STR_CP` unset and set to
  `1`; the runtime-direct fixture covers ASCII, multibyte scalars, combining
  versus precomposed text, and embedded NUL;
- the stable guard is
  `tools/checks/rust_lifecycle_mirbuilder_decoded_utf8_byte_len_v0_capability_guard.sh`.

### U3 — Independent local `ValidatedTextV0` (complete)

- RHako constructs and seals its local witness from the decoded Rust string;
- HHako normal factory invokes the internal leaf on already-decoded scalar
  string data; U4 later wires that factory into structured traversal;
- any replay-provided count is recomputed; mismatch becomes
  `InternalCarrierContractViolation`, never user `InvalidInput`;
- no path-keyed sidecar, raw input alias, or ProgramV0 schema widening.

U3 closeout:

- RHako `ValidatedTextV0` has private fields and one crate-private
  `from_decoded(value, class)` constructor; byte count is derived there and
  consumers use read-only accessors only;
- HHako `ValidatedTextV0Box.atom/literal` scalar-normalizes an already-decoded
  String, calls `DecodedUtf8ByteLenV0Box.count`, applies the local budget, and
  retains only value/count/class. It stores no `MapBox`, `ArrayBox`, `PathV0`,
  raw node, or caller-provided count;
- the declarative Hako schema owns the closed `Atom`/`Literal` class test.
  The Hako budget now keeps the literal and atom per-item limits separate, so
  a 1025-byte literal is accepted while a 1025-byte atom is Unsupported;
- `replay_only` always recomputes count. A mismatch or unknown class returns
  `InternalCarrierContractViolation` and leaves the budget unchanged. The
  repository guard forbids normal Hako sources from constructing the carrier
  or consuming `replay_only` directly;
- Hako has no runtime-private field or true immutable-object seal. U3 claims
  scalar containment plus a repository guard, not a runtime tamper-proof
  carrier or a physical String-copy proof;
- no Hako structured reader/traversal is claimed yet. U4 is the first slice
  that may call this normal factory from structured ingress;
- stable evidence remains
  `rust_lifecycle_mirbuilder_decoded_utf8_byte_len_v0_guard.sh` and
  `rust_lifecycle_mirbuilder_hako_bounded_body_snapshot_model_v0_guard.sh`.

### U4 — Resume S3 B2/B3 (active)

- rewrite the parked prototype around the selected operation;
- use a closed declarative field schema, not `split`, substring, or `indexOf`;
- cover all eight accepted statement and eleven accepted expression kinds;
- publish only deep-normalized typed carrier and one-node observations;
- keep every source file below 800 lines.

### U5 — Direct RHako/HHako parity gates

- feed the same ProgramV0 JSON independently to RHako and HHako;
- compare outcome, failure path/reason, flat preorder nodes, ordered atoms,
  ordered children, and all text budgets;
- direct parity must not depend on the Rust snapshot witness, Rust normalized
  carrier, Rust-computed count sidecar, or SnapshotBuilder;
- replay-only gates may supplement but never replace direct parity.

### U6 — Resume S3 B4-B6

- explicit preorder coordinator;
- `Open / Poisoned / Sealed` builder state machine;
- defensive reconstruction on the only successful `finish`;
- no partial snapshot on any failure path.

### U7 — Separate future API/backend decision

- decide `ByteCursorBox.len_bytes()` versus a convenience
  `String.len_bytes()` facade only when a real non-Snapshot consumer exists;
- inventory and gate each claimed product backend before Stable activation;
- delete the internal capability with ProgramV0 adapters if no consumer
  remains.

## Required fixture matrix

```text
decoded value   expected bytes
"abc"           3
"猫"            3
"😸"            4
"猫😸"          7
"e\u0301"       3
"é"             2
"\u0000"        1
"a\u0000b"      3
```

JSON escapes are measured after strict decoding. Lone surrogates, invalid
UTF-8, and malformed escapes remain ingress `InvalidInput`.

Run the exact matrix with `NYASH_STR_CP` unset and set to `1`. Add
`limit-1 / limit / limit+1` fixtures for 1024 atom bytes, 65536 literal bytes,
and 4194304 total text bytes using three- and four-byte scalars as well as
ASCII.

## Stop conditions

Stop before implementation claims if any of these occurs:

- HHako calls `length`, `len`, or `size` for the byte witness;
- the operation shares an existing length method ID, slot, alias, or lowering;
- `NYASH_STR_CP` can reach the result;
- a raw-pointer/C-string/`strlen` route is required;
- Rust-provided counts replace HHako direct computation;
- carrier mismatch is reported as user `InvalidInput`;
- unsupported backends silently use existing length or fall back to VM;
- count is taken from the escaped JSON lexeme or after Unicode normalization;
- a Stable public surface is registered without full backend inventory;
- ProgramV0 gains a count/provenance field or a path-keyed sidecar;
- caller or environment can override the schema count mode or limits;
- replay parity replaces direct independent RHako/HHako parity.

## Allowed claims after U5/U6

Only the declared accepted V0 subset may claim environment-independent
decoded UTF-8 byte budgets and exact RHako/HHako snapshot/outcome parity on
the supported execution lane. This does not claim a Stable public byte API,
all-backend support, independent HHako UTF-8 encoding, source syntax/kind
parity, planner/runtime authority transfer, or ProgramV0 permanence.
