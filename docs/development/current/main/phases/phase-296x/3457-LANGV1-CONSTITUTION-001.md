# 3457 - LANGV1-CONSTITUTION-001

## Decision

Language coherence is now the active prerequisite for selfhost migration.
MirBuilder 3456 is parked without discarding its scope or evidence.

## Scope

This is one docs-only durable-policy row. It creates the normative Hakorune
language constitution and wires existing language owners to it.

Required delta:

1. Add `docs/reference/language/semantic-contract-charter.md`.
2. Record exactly the seven laws named by the language-v1 workstream.
3. Define normative precedence among the charter, semantic kernel, EBNF,
   topic semantics, generated support views, and implementation notes.
4. Define the accepted-Decision -> fixture -> implementation -> conformance
   change protocol.
5. Keep compatibility explicit and unsupported behavior fail-fast before
   effects.

## Acceptance

```text
language_constitution_owner_count = 1
constitutional_law_count = 7
normative_precedence_defined = 1
language_change_protocol_defined = 1
compatibility_requires_explicit_profile = 1
unsupported_fails_before_effect = 1
parser_behavior_changed = 0
runtime_behavior_changed = 0
backend_behavior_changed = 0
selfhost_claim = 0
```

## Stop Lines

```text
no parser implementation
no semantic-kernel implementation
no type-contract activation
no null migration
no lifecycle behavior change
no capability verifier activation
no selfhost migration
```

## Next

After this constitution is accepted and linked from the language index and v1
freeze SSOT, advance the same workstream to:

```text
LANGV1-SEMANTIC-KERNEL-001
```

Do not create inventory, rerun, or consultation cards between these rows.
