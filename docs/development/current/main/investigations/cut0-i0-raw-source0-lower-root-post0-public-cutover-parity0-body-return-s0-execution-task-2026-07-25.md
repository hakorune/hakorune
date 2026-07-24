# RAW public cutover PARITY0 Script body-return S0

Decision: `RAW-PUBLIC-CUTOVER-PARITY0-BODY-RETURN-D0`

Status: design-stop / queued. The current parity matrix is held at the green
empty-Script row until Script return ownership is selected.

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

## Audit refinement (design-stop remains active)

The worker authority audit narrows the preferred design to A′:

```text
recipe: route/exit policy only
Builder lowering: exact ValueId and type authority
BODY finalization: one owner emits Return(Value), updates signature, and
                   emits the paired exit witness before cleanup
ROOTBATCH0: borrowed witness validation only
```

The original “recipe supplies exact return value/type before signature
creation” wording is not accepted. The implementation must first answer the
five owner questions in the design card, including the Legacy-App conflict:
Legacy App currently reaches the common last-value finalizer, while the Raw
recipe contract is `AppMain0Void`. Until that divergence is explicitly
selected, App scalar parity remains parked with the Script scalar rows.

No implementation is authorized by this card. In particular, do not add a
provisional Void-to-scalar adapter, a second root lane, or a fallback to the
Legacy builder.

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

## Next

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-BODY-RETURN-D0
```
