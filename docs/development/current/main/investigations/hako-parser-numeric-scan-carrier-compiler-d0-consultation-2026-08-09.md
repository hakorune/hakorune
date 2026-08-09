---
Status: closed — no remaining scanner carrier blocker
Date: 2026-08-09
Row: `HAKO-PARSER-NUMERIC-SCAN-CARRIER-COMPILER-D0`
Parent: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S0`
Mode: BoxShape / compiler acceptance census
---

# HAKO-PARSER-NUMERIC-SCAN-CARRIER-COMPILER-D0

## Goal

Re-run the unmodified `ParserNumberScanBox.scan_int("42}", 0)` dependency
canary now that the earlier `StringHelpers.int_to_str/1 -> to_i64/1` result
publication gap is closed. Name the exact first remaining carrier producer and
its missing compiler-side authority before reopening lexical-parts work.

Valid library source is not changed merely because the current compiler route
is narrow. A source signature annotation is permitted only if the language
contract itself requires one, never as a MIR/GenericLoop acceptance repair.

## Census

Trace exactly:

```text
unmodified scanner source
  -> exact first failing callable and Loop
  -> exact carrier BindingRef / ValueId
  -> exact initializer producer kind
  -> existing semantic/type evidence
  -> existing publication owner or proven missing owner
  -> GenericLoop entry observation
```

Distinguish at least:

```text
formal parameter
local Copy from parameter
literal / binary / call result
unknown or foreign source relation
```

Do not infer the producer from the final `MissingTransientType` alone.

## Output

Close with one of:

```text
accepted compiler BoxShape row:
  one existing semantic fact
  -> one existing or uniquely selected publisher
  -> one exact carrier

NoSafeSlice:
  the required semantic issuer is absent
  -> stop and write the missing authority contract
```

The output card must name the minimal executable slice, fail-fast boundary,
focused fixture/gate, reference/README updates, and all nonclaims.

## Forbidden repairs

```text
annotate scan_int solely to satisfy GenericLoop
default numeric-looking carriers to Integer
infer from parameter name i/j or method name scan_int
post-hoc mutate ValueId/type_ctx at Loop entry
source rewrite or JSON rescan
retry/fallback or alternate route
mix lexical-parts S0 into the carrier repair
```

## Acceptance

```text
actual StringHelpers dependency remains green
unmodified scanner fixture reaches a single reproducible first blocker
producer kind and exact source site are proven from code/evidence
source authority and physical publisher are not conflated
one compiler-side next row or honest NoSafeSlice is selected
H2-S2-S0 remains parked until that executable prerequisite is green
```

## Closeout receipt

After `GENERAL-STATIC-CALL-RESULT-PUBLICATION-I0`, the exact unmodified probe:

```hako
using lang.compiler.parser.scan.parser_number_scan_box as ParserNumberScanBox

static box Main {
  main() {
    local result = ParserNumberScanBox.scan_int("42}", 0)
    print(result)
    return 0
  }
}
```

completed successfully and printed:

```text
{"type":"Int","value":42}@2
RC: 0
```

The earlier `MissingTransientType` was fully preempted by the dependency
`StringHelpers.int_to_str/1 -> to_i64/1` publication gap. No scanner-specific
carrier repair, parameter annotation, GenericLoop default, or new publisher
is required. `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S0` is unblocked and may
resume from its clean/stashed lexical-parts implementation boundary.
