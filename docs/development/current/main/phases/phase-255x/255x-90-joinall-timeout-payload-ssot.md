Status: SSOT
Phase: 255x

# JoinAll Timeout Payload SSOT

## Decision

- `TaskGroupBox.joinAll(timeout_ms)` gets a dedicated timeout payload now.
- explicit scope exit does not get the same surface in this cut.

## Current Contract

1. `joinAll(timeout_ms)` performs the same bounded join as before.
2. If a first failure is latched, return `ResultBox::Err(first_failure_payload)`.
3. Otherwise, if the bounded join deadline is hit, return:
   - `ResultBox::Err(ErrorBox("TaskJoinTimeout", "timed out after <ms>ms"))`
4. Otherwise return `ResultBox::Ok(void)`.

## Precedence

1. first failure
2. timeout
3. success

## Non-Goals

- no aggregate-on-exit surface
- no implicit-root timeout surface
- no plugin/runtime `env.future.await` timeout change
- no VM-side `Await` timeout contract
