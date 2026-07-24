# RAW public cutover PARITY0 Script body-return S0

Decision: `RAW-BODY-RETURN-prime-r1`

Status: closed. The selected A′ owner chain is implemented and its focused
Script/App/ROOTBATCH gates are green. App scalar parity remains parked in a
separate design row.

## Decision

```text
recipe = route + exit policy authority only
Builder lowering = exact ValueId + MirType authority
BODY = sole finalizer for signature, physical Return, completion, and
       RawRootBodyExitWitnessV1
ROOTBATCH0 = borrowed witness validation and retention only

Script = ScriptLastValueOrVoid; provisional Unknown skeleton -> last-value-or-Void finalization
App    = AppFixedVoid; fixed Void skeleton and Void completion (discarded tail retained)
failure = typed discard-only owner; no retry, fallback, or postprocess repair
```

## Evidence

```text
Legacy scalar main: typed return + last expression ValueId
Raw scalar main: Void signature + Const(Void) return
Raw recipe: RootBodyResultV1::Value exists, but is not reflected in signature
```

## Required consultation outputs

```text
Script vs App return contract authority
Main slot/ledger/collector compatibility
signature creation timing relative to recipe result
last-value materialization and finalizer ownership
failure retention before root batch publication
```

## Audit refinement (decision is closed)

The worker authority audit narrows the preferred design to A′:

```text
recipe: route/exit policy only
Builder lowering: exact ValueId and type authority
BODY finalization: one owner emits Return(Value), updates signature, and
                   emits the paired exit witness before cleanup
ROOTBATCH0: borrowed witness validation only
```

The implementation does not put `MirType` or `ValueId` into the recipe. A
single BODY exit plan is prepared by borrowing the open Builder function and
active tracker, then one private infallible commit emits all three physical
facts and the paired witness. The old split `finish_raw_root_function_v1`
terminal is absent. The Legacy-App last-value difference is explicit: Raw App
uses FixedVoid for this row, and App scalar parity remains parked at
`RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-BODY-RETURN-APP-D0`.
The disconnected legacy completion fixture may carry a clearly marked
`legacy_unverified` witness bridge; it is not a BODY0 producer and is not
accepted by the new ROOTBATCH witness validator.

## Forbidden until decision

```text
hide return fields in parity snapshot
convert Raw Void to scalar in the adapter
reject scalar rows without changing NarrowV1 contract
add a second root function/publication lane
fallback to Legacy build_module
```

## Acceptance after design lock

```text
empty Script parity remains green
Script scalar literal/binary parity green
App Main remains Void
RootBatch Main/condition receipts remain exact
source/check files remain below 800 lines
```

## Closeout evidence

```text
Script empty/literal/string/binary parity = green
App empty and non-empty FixedVoid routes = green
ROOTBATCH Main/condition identity        = unchanged
exit witness                             = retained through root witness
ROOTBATCH witness validation              = before collector/ledger mutation
old split root finalizer                  = zero production definitions/callers
production consumers remain zero          = no new public cutover/adapter
all modified source/check files           = below 800 lines
```

## Next

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-S0
```
