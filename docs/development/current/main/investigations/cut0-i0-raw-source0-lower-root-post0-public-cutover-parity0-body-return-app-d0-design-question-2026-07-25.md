# RAW public cutover PARITY0 App body-return design question

Decision: `RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-BODY-RETURN-APP-D0`

Status: design-stop / parked. `RAW-BODY-RETURN-prime-r1` closes Script
last-value parity and the Raw App FixedVoid contract, but Legacy App currently
uses the common last-value finalizer. The normal-entry App cutover must not
silently choose one ABI.

## Known facts

```text
Raw App first slice:
  main/0 signature = Void
  lowered tail     = retained as discarded evidence
  physical Return  = synthetic Void
  completion       = NoValue

Legacy App:
  common finalizer consumes the last ValueId
  signature/Return may follow the lowered value
```

## Consultation boundary

Choose exactly one before widening the public App route:

```text
A: promote App to Legacy-compatible last-value return
   (requires App route return-policy and downstream parity review)

B: keep App FixedVoid and explicitly exclude scalar App from NarrowV1
   (typed eligibility rejection; no fallback to Legacy)
```

Non-authorities:

```text
snapshot normalization
postprocess return inference
module symbol inspection
public adapter repair
legacy build_module fallback
```

Until this consultation closes, the accepted executable boundary is:

```text
Script scalar parity = enabled
App FixedVoid parity = enabled
App scalar parity    = not claimed
normal-entry cutover = not claimed
```
