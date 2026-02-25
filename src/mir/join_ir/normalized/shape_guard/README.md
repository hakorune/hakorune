# Shape Guard Module Structure

This directory contains the modularized shape detection system for JoinIR pattern recognition.

## Directory Structure

```
shape_guard/
├── mod.rs          # Main module - re-exports & public API (28KB, ~760 lines)
├── pattern2.rs     # Pattern 2 detectors (simple break-loop) (4KB, ~134 lines)
├── pattern3.rs     # Pattern 3 detectors (if-sum patterns) (2.3KB, ~68 lines)
├── pattern4.rs     # Pattern 4 detectors (continue patterns) (6.2KB, ~168 lines)
├── selfhost.rs     # Selfhost-specific P2/P3 detectors (8.3KB, ~231 lines)
└── utils.rs        # Shared utility functions (1.2KB, ~35 lines)
```

**Total**: ~1405 lines (previously 1401 lines in single file)

## Module Responsibilities

### `mod.rs`
- Public API and type definitions (`ShapeCapabilityKind`, `NormalizedDevShape`, etc.)
- Shape detection coordination (`detect_shapes`, `canonical_shapes`, etc.)
- Capability mapping (`capability_for_shape`)
- Re-exports all detector functions to maintain backward compatibility
- Comprehensive test suite

### `pattern2.rs`
Pattern 2 shape detectors (simple break-loop patterns):
- `is_pattern1_mini` / `is_pattern2_mini` - Core P2 patterns
- `is_jsonparser_skip_ws_mini` / `is_jsonparser_skip_ws_real` - Skip whitespace variants
- `is_jsonparser_atoi_mini` / `is_jsonparser_atoi_real` - ASCII to integer variants
- `is_jsonparser_parse_number_real` - Number parsing

### `pattern3.rs`
Pattern 3 shape detectors (if-sum patterns with conditional carrier updates):
- `is_pattern3_if_sum_minimal` - Core P3 detection logic
- `is_pattern3_if_sum_multi` - Multi-carrier variant
- `is_pattern3_if_sum_json` - JsonParser variant

### `pattern4.rs`
Pattern 4 shape detectors (continue patterns with loop-internal control flow):
- `is_pattern4_continue_minimal` - Core P4 detection (Phase 89 tightened)
- `is_jsonparser_parse_array_continue_skip_ws` - Array parsing with continue
- `is_jsonparser_parse_object_continue_skip_ws` - Object parsing with continue
- `is_pattern_continue_return_minimal` - Continue + Early Return (Phase 89)
- `is_parse_string_composite_minimal` - Variable step increment (Phase 90)

### `selfhost.rs`
Selfhost-specific shape detectors:
- P2 family:
  - `is_selfhost_token_scan_p2` / `is_selfhost_token_scan_p2_accum` - Token scanning
  - `is_selfhost_args_parse_p2` - Argument parsing (Phase 53)
  - `is_selfhost_verify_schema_p2` - Schema verification (Phase 54)
- P3 family:
  - `is_selfhost_if_sum_p3` / `is_selfhost_if_sum_p3_ext` - If-sum patterns
  - `is_selfhost_stmt_count_p3` - Statement counting (Phase 53)
  - `is_selfhost_detect_format_p3` - Format detection (Phase 54)
- Helper functions:
  - `is_selfhost_p2_core_family_candidate` - Structural signature for P2
  - `is_selfhost_p3_if_sum_family_candidate` - Structural signature for P3

### `utils.rs`
Shared utility functions:
- `name_guard_exact` - Name-based shape filtering
- `find_loop_step` - Find loop_step function in module
- `count_compare_ops` - Count Compare operations with specific operator

## Design Principles

### Single Responsibility
Each file handles one pattern family, making it easier to:
- Understand the detection logic for a specific pattern
- Modify or extend pattern detection
- Test individual patterns in isolation

### Backward Compatibility
All public APIs are maintained through re-exports in `mod.rs`:
- Existing code continues to work without changes
- All tests pass with no modifications

### Clear Separation
- **Pattern detection**: Pattern-specific files (pattern2-4, selfhost)
- **Shared logic**: utils.rs
- **API & coordination**: mod.rs

## Testing

All tests are in `mod.rs` to keep them close to the public API. Tests verify:
- Individual pattern detection
- Shape detection coordination
- Disambiguation between similar patterns
- Rejection of mismatched patterns

## Migration Notes

**Phase 195**: Modularized from single 1401-line file to 6 organized files.

**Changes**:
- File split into pattern-based modules
- All APIs re-exported to maintain compatibility
- Tests remain in mod.rs
- Build succeeds with no new warnings
- Total line count: 1405 (4 lines added for module declarations)

**Benefits**:
- Improved maintainability (each file < 250 lines)
- Clearer code organization
- Easier to add new patterns
- Better testability
