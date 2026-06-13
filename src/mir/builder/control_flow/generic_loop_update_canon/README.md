# Generic Loop Update Canon

This box owns generic-loop update-canon helpers.

## Boundary

```text
literal_match.rs:
  matches loop-variable update expressions against literal binary operations
  analysis-only
  no route selection
  no lowering

literal_step.rs:
  converts matched literal updates into UpdateCanon
  keeps literal-step construction separate from shape matching
```

## Compatibility

The older deep path
`facts/canon/generic_loop/update.rs` remains as a facade while imports migrate.

Do not merge update matching with loop-step placement or plan policy. This box
only observes update expression shape and builds facts-side `UpdateCanon`.
