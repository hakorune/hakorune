# 296x-1307 STRING-CORRIDOR-STABLE-LENGTH-HINT-FALLBACK-RETIRE-CLOSEOUT-001

Status: closed  
Date: 2026-06-19  
Output contract: `string-corridor-stable-length-hint-fallback-retire-closeout-v0`

## Decision

`STRING-CORRIDOR-STABLE-LENGTH-HINT-FALLBACK-RETIRE-001` is closed.

String-corridor planning now reads typed `StringCorridorRelation` stable-length
evidence only. The previous fallback that parsed
`optimization_hints` strings as correctness evidence was retired.

The sink pass still emits the existing stable-length optimization hint for
diagnostics, but it also emits the typed stable-length relation directly. The
relation refresh keeps typed stable-length relations only when their base and
witness values still exist.

## Evidence

```bash
cargo test -q string_corridor_relation
cargo test -q string_corridor_sink
cargo test -q string_kernel_plan
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Additional grep evidence:

```text
stable_length_relation_from_hint=0
stable_length_value_for_source_from_hints=0
strip_prefix("string_corridor_sink:stable_length_scalar")=0
```

## Next

Return to compiler route debt cleanup:

```text
COREPLAN-LOOP-ACTUAL-SELECTION-TRACE-001
```

Scope:

- record the actual legacy loop route whose handler returned success.
- keep B-lite / resolver shadow read-only.
- do not make resolver a route-selection owner.
- do not delete suppression branches in this row.

## Stop Line

- do not retire named loop routes before actual-selection trace exists.
- do not use raw candidate lists as selected-route proof.
- do not feed legacy observation back into resolver decisions.
- do not reopen fastpath optimization without a fresh measured owner.

summary=ok
