# Box Member Parser Boundary

This directory parses Box member syntax. Keep syntax recognition separate from
synthetic method body construction.

- `fields.rs`: stored fields, weak-field delegation, visibility sugar, and
  header-first computed/get parsing.
- `properties.rs`: once/birth_once and block-first unified member parsing.
- `property_emit.rs`: the only owner for synthetic property method AST bodies,
  naming (`__get_*`, `__get_once_*`, `__get_birth_*`,
  `__compute_once_*`, `__compute_birth_*`), stored field initializer
  prologues, and `birth_once` constructor prologue statements.
- `postfix.rs`: the only owner for Box member postfix `catch/cleanup` parsing
  and `TryCatch` wrapping, including the member postfix syntax gate.
- `pending_method.rs`: unpublished explicit method transaction. Postfix
  syntax may mutate this value; the ordered inventory receives it exactly
  once only after postfix parsing is complete.
- `property_batch.rs`: prepares every helper emitted by one property member
  and commits the complete generated-method batch only after all collision and
  provenance checks pass.

Rules:

- Do not reserve `get` in the tokenizer. It is contextual at Box member head.
- Do not add generic runtime property lookup here.
- Do not duplicate synthetic property method bodies in parser entry modules.
- Do not duplicate stored field initializer or `birth_once` eager initializer
  AST construction outside `property_emit.rs`.
- Do not duplicate Box member postfix `catch/cleanup` parsing outside
  `postfix.rs`.
- Do not mutate a published method inventory entry for postfix syntax. Keep
  the method pending and commit it once at the next member or Box end.
- Do not insert generated property/delegate rows one at a time. Construct one
  complete `PreparedGeneratedBoxMethodBatchV1` and commit it atomically.
- Selected branches stage through the enclosing
  `OpenBoxMethodSourceTransactionV1`; the branch relation table owns exact
  source sites while the AST inventory receives one prepared append. No
  parallel ordinal slice or length delta may cross this boundary. The sole
  unpublished method/source owner is the transaction; the AST inventory is
  only its ordered placement carrier.
- A parsed delegate carries its exact source selection. Compatibility-only
  delegates must not generate fresh source-authoritative method rows.
- Do not bypass the member postfix gate for method or constructor postfix
  handlers.
- Keep `weak` on the stored-field path only. Do not route weak fields through
  computed/once/birth_once property parsing.
- Keep AST/JSON/MIR shape stable unless a separate language decision changes it.
