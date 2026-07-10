# 3464 - LANGV1-RUST-GRAMMAR-PEEK-COMPAT-ALIAS-001

## Status

Active implementation card. Migrate Rust `peek` under the typed grammar
profile without changing `from`, Hako, runtime, backend, or selfhost behavior.

## Inventory

```text
Canonical match = live Rust TokenType::MATCH plus expr_parse_match
Rust peek token/parser = absent
Hako peek parser = live but outside this card
Compat2025 contract = lossless closed subset aliases immediately to Match
```

The Rust Match parser already owns the target normalized shape. The narrow
implementation adds a `PEEK` token and a profile gate before invoking that
same parser. It does not add a second pattern parser.

## Ordered Work

1. Tokenize `peek` as a distinct `PEEK` token.
2. Canonical `peek` rejects with `parser/peek_legacy_replaced_by_match`.
3. Explicit Compat2025 `peek` invokes the existing Match parser directly.
4. Successful Compat2025 `peek` returns the same normalized AST shape as the
   equivalent `match` source.
5. A `peek` form rejected by the Match parser fails with
   `parser/peek_compat_not_normalizable`; do not retry another parser.
6. Extend the existing Rust grammar-profile guard and shared corpus fixtures.
7. Keep both `from` forms unchanged; their transport-only boundary requires a
   separate design decision.

## Acceptance

```text
rust_peek_profile_seam_implemented = 1
canonical_peek_rejected = 1
compat2025_peek_normalized_to_match = 1
peek_parser_implementation_count = 0
implicit_compat_retry = 0
rust_from_behavior_changed = 0
hako_parser_behavior_changed = 0
```

## Non-Claims

```text
rust_from_migrated = 0
from_compat_transport_implemented = 0
hako_peek_migrated = 0
live_parse_witness_conformance = 0
parser_sharing = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Verification

```bash
bash tools/checks/language_v1_rust_grammar_profile_guard.sh
cargo check
bash tools/checks/language_v1_grammar_contract_substrate_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Next

After green, stop at the Rust `from` transport-only design boundary. Do not
reuse the live semantic `FromCall`/inheritance AST as compatibility transport.
