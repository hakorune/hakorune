# Syntax-3 Handler Compatibility Guide

Status: Historical/compatibility implementation inventory; no current
language-authority role.

Current target guide:

- `docs/guides/exception-handling.md`
- `docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md`

## Why this compatibility lane still exists

Existing Rust/Hako parser fixtures and MIR bridges may still carry:

```text
statement try
postfix catch / cleanup
method handler tails
TryCatch / CatchClause
syntax-3 environment gates
```

These shapes preserve migration and regression evidence only. They do not
authorize new source use, typed exception dispatch, `RecoverableFailure`, an
exception ABI, or JoinIR `Invoke` lowering.

## Accepted C′ boundary

```text
try / throw / catch          = rejected target
recoverable failure          = Result<T,E>
unchanged propagation        = exact Result-only postfix ?
local recovery/conversion    = guard let / match
lexical cleanup              = standalone cleanup { ... }
Box finalization             = non-callable terminal-Home fini { ... }
Fault                        = terminal and non-catchable
```

Parser acceptance under `NYASH_PARSER_STAGE3`, `NYASH_BLOCK_CATCH`, or
`NYASH_METHOD_CATCH` is implementation evidence, not a semantic profile. No
environment variable may become language authority.

## Migration stop lines

- Do not add new postfix catch/cleanup examples.
- Do not implement typed catch dispatch or exception unwinding.
- Do not lower an unsupported handler to a no-op or retry another profile.
- Preserve current bridge behavior only until its atomic retirement row.
- New code uses `Result`, `?`, `guard let`/`match`, and standalone `cleanup`.

Retirement belongs to `LANGUAGE-RESULT-EXIT-C-PRIME0-R0`. Only after I0/R0 and
backend parity may `LANGUAGE-RESULT-EXIT-C-PRIME0-DOC0` rewrite the
implementation-backed grammar and remove this compatibility guide or redirect
it permanently.
