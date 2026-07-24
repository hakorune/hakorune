# RAW public cutover consultation

Decision stop: `RAW-PUBLIC-CUTOVER-CONSULT0`

`PUBLICATION-ADAPTER0-S0` and `PUBLIC-INGRESS0-S0` are implemented and
focused-verified. `compile_raw_with_source` is now an explicit NarrowV1 entry;
`compile_with_source`, JSON bridges, the executor, and old Raw bridges remain
unchanged. The next decision must choose how, or whether, the normal entry is
cut over and when the old chain is retired.

## Evidence inventory

```text
new explicit Raw ingress producer                 = 1
normal compile_with_source legacy route           = unchanged
old MirBuilder::build_module production route    = present
old Raw finalizer/run_raw sources                 = present
JSON/Program(JSON v0) routes                      = unchanged
Raw focused chain (single-thread)                 = 240 passed
adapter/ingress guards                            = green
```

## Questions

### Q1 — normal-entry authority

Choose one:

```text
A. keep compile_with_source legacy and require explicit Raw ingress
B. switch compile_with_source to Raw NarrowV1 for admitted grammar only
C. add a separate capability-selected normal entry before any switch
```

The current recommendation is A until parity and caller census are complete.

### Q2 — admitted-grammar parity

Which evidence is sufficient before a switch?

```text
A. bounded success/failure parity for the locked Raw grammar and compiler reuse
B. full library-suite parity including unsupported legacy shapes
C. runtime-only parity without MIR/verification relation checks
```

Unsupported Raw shapes must remain typed rejection; they must not fallback.

### Q3 — failure and Builder reuse

Choose the required gate:

```text
A. every Raw failure leaves live Builder unchanged; repeated compiler reuse is tested
B. compare only public error strings
C. allow legacy retry for failures outside the first slice
```

### Q4 — old-chain retirement

Choose the retirement trigger:

```text
A. old Raw finalizer/run_raw/ledger-root evidence non-test callers = 0,
   then delete in a dedicated row
B. delete old sources immediately after explicit ingress lands
C. keep both chains permanently
```

### Q5 — JSON and production activation

Choose the boundary:

```text
A. JSON routes stay unchanged; executor/CUT0 activation is a later row
B. route JSON through compile_raw_with_source now
C. switch executor first and infer route from module symbols
```

### Q6 — sunset measurement

Choose the census scope:

```text
A. explicit production caller census by route plus RAW-PUBLICATION-SUNSET-001
B. repository-wide lexical zero for every old symbol
C. focused test caller census only
```

No implementation should begin until Q1-Q6 and the normal-entry stop line are
recorded. This consultation deliberately claims no normal-entry cutover,
legacy deletion, executor wiring, JSON change, or CUT0 activation.
