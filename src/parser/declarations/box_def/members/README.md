# Box Member Parser Boundary

This directory parses Box member syntax. Keep syntax recognition separate from
synthetic method body construction.

- `fields.rs`: stored fields, weak-field delegation, visibility sugar, and
  header-first computed/get parsing.
- `properties.rs`: once/birth_once and block-first unified member parsing.
- `property_emit.rs`: the only owner for synthetic property method AST bodies,
  naming (`__get_*`, `__get_once_*`, `__get_birth_*`,
  `__compute_once_*`, `__compute_birth_*`), and `birth_once` constructor
  prologue statements.
- `postfix.rs`: the only owner for Box member postfix `catch/cleanup` parsing
  and `TryCatch` wrapping, including the member postfix syntax gate.

Rules:

- Do not reserve `get` in the tokenizer. It is contextual at Box member head.
- Do not add generic runtime property lookup here.
- Do not duplicate synthetic property method bodies in parser entry modules.
- Do not duplicate `birth_once` eager initializer AST construction outside
  `property_emit.rs`.
- Do not duplicate Box member postfix `catch/cleanup` parsing outside
  `postfix.rs`.
- Do not bypass the member postfix gate for method or constructor postfix
  handlers.
- Keep `weak` on the stored-field path only. Do not route weak fields through
  computed/once/birth_once property parsing.
- Keep AST/JSON/MIR shape stable unless a separate language decision changes it.
