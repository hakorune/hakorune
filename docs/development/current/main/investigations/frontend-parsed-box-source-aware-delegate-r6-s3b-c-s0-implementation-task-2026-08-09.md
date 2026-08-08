---
Status: closed bounded implementation
Date: 2026-08-09
Decision: implement parser-time delegate source transport only; do not open target lookup or final seal coverage
Parent: `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-d0-design-task-2026-08-08.md`
Reference: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-C-S0

## Scope

This row implements the first bounded slice after C-D0:

```text
parser reads explicit delegate
  -> one parser-private source row per expose
  -> source transaction owns the rows
  -> selected member-gate merge rebases the source path
  -> prepared postpass payload carries the rows
```

The rows are parser evidence, not resolver-visible semantic products. The
existing AST-only delegate lowering remains unchanged in this row. Generated
delegate inventory suffixes remain outside the final `ParserBoxSourceSealV1`.

## Source authority and owner

```text
OpenBoxMethodSourceTransactionV1
  owns `DelegateSourceDeclarationV1[]`

DelegateSourceDeclarationV1
  host delegate member site
  expose ordinal
  field/source/exposed names

PreparedBoxSourceSealV1
  transports the rows across prune/finalization
  but the final ParserBoxSourceSealV1 does not expose them yet
```

The parser records the row immediately after parsing the explicit delegate
declaration. It does not reconstruct rows from final AST, inventory ordinal,
delegate provenance, names, or a `HashMap`. Compatibility-only delegates are
rejected by the source transport and cannot acquire source authority.

## Required behavior

```text
one expose -> one row
expose ordinals are parser-local source order starting at zero
host member path is the current parser-issued source site
selected gate merge preserves the branch member and prepends the gate path
branch transaction rows merge atomically with its inventory relations
failure leaves the source transaction without partial delegate rows
```

No target Box/method lookup occurs in S0. No generated method placement is
attached to a relation in S0; that belongs to C-I0 after S1 target lookup.

## Acceptance tests

```text
transaction_records_one_delegate_source_row_per_expose
selected_gate_rebases_delegate_source_member_path
compatibility_delegate_cannot_enter_source_transport
ordinary parser delegate fixture remains parseable
existing final source seal still excludes generated delegate suffix rows
```

The source module and source-seal module remain below 800 lines. Focused Rust
tests, formatting, current pointer guard, the B3 parser guard, and this task's
source-transport guard are required before closeout. The implementation and
reference receipt must land in the same commit.

## Implementation receipt (2026-08-09)

Closed with the parser-private transport implemented in one slice:

```text
DelegateSourceDeclarationV1
  one row per expose (explicit delegate source only)
  parser-issued host source site + expose ordinal + names

OpenBoxMethodSourceTransactionV1
  records rows at parse time
  rebases selected member-gate paths during consume-return merge

PreparedBoxSourceSealV1
  carries rows across the parser postpass

ParserBoxSourceSealV1
  deliberately drops rows until C-D extends final seal coverage
```

Compatibility-only delegates reject before acquiring source authority. The
existing descriptive AST/generated delegate inventory remains unchanged, and
no target lookup, generated placement relation, resolver-visible seal
extension, or postpass batch commit was opened.

Verification:

```text
cargo test -q --lib parser::source_authority -- --nocapture
cargo test -q --lib parser::source_seal::source_seal_delegate_tests -- --nocapture
cargo test -q --lib parser_delegate_surface -- --nocapture
cargo fmt --all -- --check
git diff --check
bash tools/checks/frontend_parsed_box_source_seal_r6_s3b_b3_guard.sh
bash tools/checks/frontend_parsed_box_source_seal_r6_s3b_c_s0_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Nonclaims

```text
no path-based target index
no ExistingTargetMethodSourceRef resolution
no GeneratedDelegateSourceRelation with generated placement
no all-host/expose batch preflight
no atomic AST/generated-batch postpass transaction
no resolver-visible seal extension
no CallableContract, Home, ABI, Recipe, Builder, MIR, provider, or runtime
no Hako parity, generated-delegate chain, fallback, or retry
```

## Next slice

This row is closed. Its child `R6-S3B-C-S1-D0` design and bounded C-S1 private
target-index implementation are also closed. The C-I0 design is accepted and
its implementation remains unopened at the clean stop for all-host/expose
preflight and one atomic generated batch. If the parser cannot issue an exact source row, stop at
`NoSafeSlice`; do not add a test constructor or a name-based shortcut.
