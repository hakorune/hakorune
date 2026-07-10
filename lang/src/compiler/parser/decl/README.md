# Declaration Evidence Parser

This directory parses declaration grammar evidence for Language v1.

- Output belongs only to `ProgramJSON.parser_evidence.declarations`.
- Declaration evidence must never enter `ProgramJSON.body`, MIR, runtime, or backend input.
- `ParserDeclarationBox` owns dispatch and registry-derived profile gates.
- `ParserDelegateExposesBox` observes canonical delegate declarations without
  publishing forwarding methods.
- Subparsers own syntax only and must not read environment-selected profiles.
- Source slicing, reparsing, fixture-specific branches, and semantic fallback are forbidden.

The grammar/profile authority is the generated projection from
`grammar/language-v1-registry.toml`.
