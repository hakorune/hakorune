# Tests Organization

This directory contains organized test files for the hakorune-vm.

## Structure

### `unit/`
- Unit tests for individual components
- Core operations, handlers, and utilities

### `integration/`
- Integration tests across multiple components
- End-to-end functionality verification

### `regression/`
- Regression tests for known bugs
- Prevent reintroduction of fixed issues

### `performance/`
- Performance benchmarks and validation
- Memory usage, execution time tests

## Migration Notes

Previously, all tests were in this directory. They have been organized as:

- **Unit Tests**: Core operation tests (const, binop, compare, etc.)
- **Integration Tests**: Complex scenarios (mircall phases, boxcall, etc.)
- **Regression Tests**: Bug-specific tests (compare_bug, mapbox_fix, etc.)
- **Performance Tests**: Performance-critical scenarios
