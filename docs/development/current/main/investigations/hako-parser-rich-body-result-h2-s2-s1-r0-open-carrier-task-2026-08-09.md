---
Status: closed — Typed product now retains its exact open carrier
Date: 2026-08-09
Row: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R0`
Parent: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1`
Mode: BoxShape / behavior-neutral temporal ownership repair
---

# HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R0

## Goal

Make a Typed `ParserNodeProductV1` retain the exact open
`SourceCarrierBuilderV1` that issued its node ref, plus its compatibility
fragment. This is the temporal handoff required for the later SourceBody owner
to complete the same tree.

```text
open SourceCarrierBuilderV1
  -> child/statement node ref
  -> ParserNodeProductV1::Typed(
       same open carrier,
       exact node ref,
       compatibility fragment,
       next position
     )
  -> later body owner continues the same carrier
```

## Boundary

`SourceNodeRefV1` is an arena-branded index inside one builder allocation. A
Typed product without that builder is partial truth. R0 repairs this relation
and makes builder mutation reject same-index refs issued by another arena.

```text
Typed:
  carrier != null
  node_ref != null
  compatibility fragment present

CompatOnly:
  carrier = null
  node_ref = null
  compatibility fragment present

ParseError:
  carrier = null
  node_ref = null
  issue present
```

Do not add a second node lookup, clone/copy API, from-ref constructor, sealed
tree, root, list, statement parser, expression parser, or parser connection.
The product remains parser-private and open; only the later body row may seal
the carrier.

## Implementation

1. extend `ParserNodeProductV1` with its exact carrier;
2. require `typed(carrier, node_ref, compat_fragment, next_pos)`;
3. keep CompatOnly and ParseError carrier-free;
4. update the existing P0 fixture to prove the Typed carrier remains Open and
   is the same mutable owner that issued the node;
5. brand node/list refs with the private builder arena and reject foreign refs;
6. update the P0 guard/README contract without weakening parser nonconnection;
7. keep all Hako sources below 800 lines.

## Acceptance

```text
positive:
  one builder issues LiteralInt ref
  Typed product retains builder/ref/compat/next
  retained builder remains Open
  later mutation through retained carrier affects the same owner
  same-index foreign node/list refs poison the receiving builder

negative/structural:
  Typed factory without carrier no longer exists
  CompatOnly/ParseError expose no carrier
  parser branch connection remains 0
  tree publication remains 0
  arbitrary node-ref reconstruction remains forbidden
```

Run:

```bash
bash tools/checks/hako_parser_source_carrier_p0_guard.sh
bash tools/checks/hako_parser_rich_body_h2_s2_s0_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/naming_charter_guard.sh
```

## Nonclaims

```text
expression rich product
precedence refactor
Return arm connection
SourceBody/list/root seal
method/H3 connection
grammar or compatibility behavior change
Home, resolver, Recipe, MIR, runtime
```

## Closeout

Implementation, P0 fixture/guard, source-carrier README, current pointers,
commit, and push close together. Next is `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R1`.

## Closeout receipt

`ParserNodeProductV1::Typed` now requires and retains the exact open builder,
node ref, compatibility fragment, and next position. The P0 fixture proves the
retained builder is still Open and that a later parent node is appended through
the same owner. Arena branding rejects same-index node/list refs from a foreign
builder. CompatOnly and ParseError retain neither builder nor node ref.
The old node-ref-only Typed factory is gone; parser branch connection and tree
publication remain zero.
