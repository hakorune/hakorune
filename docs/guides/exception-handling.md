# Failure Handling — Result, `?`, and `cleanup`

Status: Accepted C′ target guide; production activation 0.

Normative owners:

- `docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md`
- `docs/reference/language/failure-outcome-relations.md`
- `docs/reference/language/scope-exit-semantics.md`

## The four rules

```text
recoverable failure       = Result<T,E>
unchanged propagation     = Result expression followed by ?
local handling/conversion = guard let or match
always-run lexical action = standalone cleanup { ... }
terminal contract failure = Fault
```

Canonical v1 has no source `try`, `throw`, `catch`, or
`RecoverableFailure`. `Option<T>` represents absence and does not support `?`
in v1.

## Propagate an unchanged error

```hako
load(path): Result<Data, IoError> {
    local file = File.open(path)?

    cleanup {
        file.closeBestEffort()
    }

    local data = file.read()?
    return Result::Ok(data)
}
```

`expr?` is accepted only when the operand is `Result<T,E>` and the enclosing
callable returns `Result<U,E>` with the exact same `E`. The operand runs once.
No implicit error conversion, hidden `share`, dynamic `isOk/getValue`, or
user-defined Try protocol is permitted.

## Handle or convert locally

```hako
match File.open(path) {
    Result::Ok(file) => use(file)
    Result::Err(error) => report(error)
}
```

If the error type changes, construct the new error explicitly in a `match`.

## Cleanup is not catch or object finalization

`cleanup {}` registers one lexical exit action after execution reaches the
statement. Registrations are LIFO. A cleanup body cannot issue `return`,
`break`, `continue`, `?`, `await`, or `yield`.

Box-member `fini {}` is different: it is a non-callable hook invoked only by
the terminal Home DropPlan. An ordinary `close()`/`shutdown()` method may
return `Result` when exact shutdown timing and error handling matter.

## Faults

`Fault` is terminal and non-catchable. During exit, the first Fault in time is
primary; later cleanup/finalization Faults are suppressed while remaining
teardown continues best effort.

## Current implementation boundary

The repository still contains syntax-3 handler syntax, TryCatch carriers,
environment gates, and a dynamic QMark route. They are migration evidence,
not target-language permission. Until `LANGUAGE-RESULT-EXIT-C-PRIME0-I0/R0`
lands, unsupported C′ shapes must reject before Builder effects.

After implementation and backend parity, the mandatory
`LANGUAGE-RESULT-EXIT-C-PRIME0-DOC0` receipts update EBNF, registry, both
parsers, reference pages, examples, and migration guides from actual landed
behavior.
