# Phase 85: Ring0 Runtime System

Ring0 is the OS API abstraction layer in Hakorune/Nyash, providing a clean interface to system resources without knowing about Boxes or Nyash-specific concepts.

## Architecture

Ring0Context bundles multiple API traits:
- **MemApi**: Memory allocation (malloc/free)
- **IoApi**: Stdin/Stdout/Stderr operations
- **TimeApi**: System time and monotonic clock
- **LogApi**: Structured logging
- **FsApi**: File system operations (Phase 90-A)
- **ThreadApi**: Thread operations (Phase 90-D)

## Implementations

### Phase 88: Initial std-based implementations
- NoopMem: Stub memory API (returns null)
- StdIo: std::io based I/O
- StdTime: std::time based time
- StdLog: eprintln!/println! based logging
- StdFs: std::fs based filesystem
- StdThread: std::thread based threading

### Phase 102: MemApi Bridge Skeleton

**Completed Tasks**:
- StdMem implementation (stdlib alloc/free based)
- default_ring0() unified to use StdMem instead of NoopMem
- Unit tests for StdMem allocation and statistics
- NoopMem retained for compatibility

#### Implementation Status
- **StdMem**: Implemented (stdlib alloc/free based)
- **Statistics Management**: allocated/freed/current counters
- **Compatibility**: NoopMem retained for testing

#### Design Notes
- The free() method counts freed operations but not freed bytes (no size info)
- Full statistics tracking will be added in Phase 102B (hakmem integration)

#### Usage Example
```rust
let mem = StdMem::new();
let ptr = mem.alloc(1024);
let stats = mem.stats();
println!("allocated: {}, current: {}", stats.allocated, stats.current);
mem.free(ptr);
```

#### Next Steps
Phase 102B will add:
- Size-pairing dictionary (record alloc sizes)
- Accurate freed size statistics
- hakmem API bridge integration

## Global Ring0Context

Ring0 provides a global singleton via:
- `init_global_ring0(ctx)`: Initialize global context
- `get_global_ring0()`: Get global context

## Testing

All Ring0 components have comprehensive unit tests. Run with:
```bash
cargo test --release --lib ring0
```

## Files

- `src/runtime/ring0/mod.rs`: Main module and Ring0Context struct
- `src/runtime/ring0/traits.rs`: API trait definitions
- `src/runtime/ring0/std_impls.rs`: Standard library implementations
- `src/runtime/ring0/errors.rs`: Error types
