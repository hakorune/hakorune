# Parser Program Owner

`ParserProgramBox` owns one top-level source-to-ProgramJSON orchestration loop.
It consumes parser state only through the `ParserBox` facade passed as `ctx`.

Responsibilities:

- recursion-depth and progress guards;
- statement/static-data sequencing;
- parser contract-error consumption;
- final ProgramJSON assembly.

Non-responsibilities:

- expression or statement grammar;
- enum/record semantic policy;
- MIR or runtime lowering;
- compatibility fallback.

`ParserBox.parse_program2` is the stable public entry and delegates here. Do
not add a second public ProgramJSON parser entry.
