# Parser-private Source Carrier V1

This directory owns the construction substrate for the Hako parser's sealed,
source-preserving tree.

P0 rules:

- authoritative parser branches do not import this directory yet;
- records are constructed through factories/builder only;
- raw MapBox, JSON, token text, ProgramV0, MIR, planner, route, backend, and
  runtime meaning are forbidden;
- children are built before parents (`child_id < parent_id`);
- a builder publishes zero or one defensively reconstructed tree;
- `CompatOnly` is a parser product for future ProgramV0 migration and is never
  a source-tree node;
- every source file stays below 800 lines.

Mutation and publication are physically separate: `source_carrier_builder_v1`
owns drafts/state, while `source_carrier_sealer_v1` owns reachability,
bottom-up defensive validation, and immutable reconstruction.

The language cannot enforce module-private constructors. Repository guards
therefore enforce factory-only construction and parser nonconnection during
P0.
