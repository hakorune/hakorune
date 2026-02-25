# Smoke Test Environment Configuration (SSOT)

## Overview

`tools/smokes/v2/lib/env.sh` provides centralized environment variable configuration for all smoke tests. This prevents "sparrow bugs" (scattered duplicate configurations) and provides a single source of truth (SSOT) for smoke test environment.

## Usage

### Basic Usage

Source `env.sh` in your smoke script (already done in test_runner.sh and llvm_exe_runner.sh):

```bash
source "$(dirname "$0")/../lib/env.sh"
```

### Mode-Specific Configuration

Use `setup_smoke_env` to configure for different test profiles:

```bash
source "$(dirname "$0")/../lib/env.sh"

# Dev mode: verbose logging, unlimited fuel, JoinIR dev enabled
setup_smoke_env dev

# Integration mode: JoinIR dev enabled, moderate verbosity
setup_smoke_env integration

# Quick mode: minimal logging, fast execution
setup_smoke_env quick
```

### Display Current Configuration

```bash
source "$(dirname "$0")/../lib/env.sh"
show_smoke_env
```

Output:
```
[INFO] Smoke Environment Configuration (SSOT: lib/env.sh)
  JoinIR Dev:          NYASH_JOINIR_DEV=1
  JoinIR Strict:       HAKO_JOINIR_STRICT=1
  LLVM Harness:        NYASH_LLVM_USE_HARNESS=1
  LLVM Backend:        NYASH_LLVM_BACKEND=crate
  Plugins:             NYASH_DISABLE_PLUGINS=0
  Debug Fuel:          NYASH_DEBUG_FUEL=10000
  Verbose:             NYASH_CLI_VERBOSE=0
```

## Environment Variables (SSOT)

### JoinIR Development Mode (Phase 131+)

Required for dev-only fixtures using normalized shadow control flow.

- `NYASH_JOINIR_DEV`: Enable dev-only JoinIR features (default: 1)
- `HAKO_JOINIR_STRICT`: Enable strict JoinIR validation (default: 1)

### LLVM Features

- `NYASH_LLVM_USE_HARNESS`: Use Python llvmlite backend (default: 1)
- `NYASH_LLVM_BACKEND`: Backend selection - crate|python|auto (default: crate)
- `NYASH_LLVM_VERIFY`: Enable LLVM verification (default: 0)
- `NYASH_LLVM_VERIFY_IR`: Enable IR verification (default: 0)

### Tmpdir EXDEV Mitigation

Prevents "Invalid cross-device link" errors when /tmp is on different filesystem.

- `TARGET_TMPDIR`: Override tmpdir location (default: current directory)

### Plugin Loader Strategy

- `NYASH_LOAD_NY_PLUGINS`: Load .hako-based plugins from nyash.toml (default: 0)
- `NYASH_DISABLE_PLUGINS`: Disable all plugins except core builtins (default: 0)

### Parser Features

- `NYASH_FEATURES`: Parser feature level (default: stage3)
- `NYASH_ENABLE_USING`: Enable using system (default: 1)
- `HAKO_ENABLE_USING`: Enable using system (default: 1)
- `NYASH_ALLOW_USING_FILE`: Allow file imports (default: 1)
- `HAKO_ALLOW_USING_FILE`: Allow file imports (default: 1)
- `NYASH_USING_AST`: Use AST mode for using (default: 1)

### Debug Features

Controlled by `SMOKE_DEBUG=1` environment variable:

- `NYASH_CLI_VERBOSE`: Verbose CLI output (default: 0, dev mode: 1)
- `HAKO_TRACE_EXECUTION`: Trace execution (default: 0, debug mode: 1)
- `HAKO_VERIFY_SHOW_LOGS`: Show verification logs (default: 0, debug mode: 1)
- `NYASH_DEBUG_FUEL`: Debug fuel limit (default: 10000, dev mode: unlimited)

### Other Settings

- `HAKO_SILENT_TAGS`: Reduce log noise (default: 1)

## Validation

### Manual Validation

```bash
source "$(dirname "$0")/../lib/env.sh"
validate_env_setup
```

### Auto-Validation

Set `SMOKE_ENV_VALIDATE=1` to auto-validate on source:

```bash
export SMOKE_ENV_VALIDATE=1
source "$(dirname "$0")/../lib/env.sh"
# Validation runs automatically
```

## Mode Presets

### Dev Mode

```bash
setup_smoke_env dev
```

- Verbose: ON (NYASH_CLI_VERBOSE=1)
- Debug fuel: unlimited
- JoinIR dev: ON

### Integration Mode

```bash
setup_smoke_env integration
```

- Verbose: OFF (unless overridden)
- Debug fuel: 10000
- JoinIR dev: ON

### Quick Mode

```bash
setup_smoke_env quick
```

- Verbose: OFF
- Debug fuel: 10000
- Silent tags: ON (reduce noise)

## Migration Notes

### For Existing Scripts

Most scripts will automatically use env.sh via test_runner.sh or llvm_exe_runner.sh sourcing it. No changes required.

### For New Scripts

1. Source test_runner.sh (which sources env.sh):
   ```bash
   source "$(dirname "$0")/../lib/test_runner.sh"
   ```

2. Optionally set mode if needed:
   ```bash
   setup_smoke_env integration
   ```

3. Use require_joinir_dev for Phase 131+ fixtures:
   ```bash
   require_joinir_dev
   ```

## Examples

### Basic Smoke Script (VM)

```bash
#!/bin/bash
source "$(dirname "$0")/../lib/test_runner.sh"
require_env || exit 2

# Test implementation
test_pass "my_test: passed"
```

### LLVM EXE Smoke Script

```bash
#!/bin/bash
source "$(dirname "$0")/../lib/test_runner.sh"
source "$(dirname "$0")/../lib/llvm_exe_runner.sh"

require_env || exit 2
llvm_exe_preflight_or_skip || exit 0

# Phase 131 requires dev mode
require_joinir_dev

# Test implementation
```

### Custom Mode Configuration

```bash
#!/bin/bash
source "$(dirname "$0")/../lib/test_runner.sh"

# Override specific variables after env.sh defaults
export NYASH_CLI_VERBOSE=1
export NYASH_DEBUG_FUEL="unlimited"

# Test implementation
```

## Troubleshooting

### Variables Not Set

Ensure env.sh is sourced before accessing variables:

```bash
# ✅ Correct
source "$(dirname "$0")/../lib/test_runner.sh"
echo "JoinIR dev: $NYASH_JOINIR_DEV"

# ❌ Wrong
echo "JoinIR dev: $NYASH_JOINIR_DEV"  # Not set yet!
source "$(dirname "$0")/../lib/test_runner.sh"
```

### Mode Not Applied

Call setup_smoke_env AFTER sourcing env.sh:

```bash
# ✅ Correct
source "$(dirname "$0")/../lib/env.sh"
setup_smoke_env dev

# ❌ Wrong
setup_smoke_env dev  # Function not defined yet!
source "$(dirname "$0")/../lib/env.sh"
```

### Defaults Overridden

env.sh uses `${VAR:-default}` pattern, so pre-set variables take precedence:

```bash
# Override before sourcing env.sh
export NYASH_JOINIR_DEV=0  # Disable dev mode
source "$(dirname "$0")/../lib/env.sh"
# NYASH_JOINIR_DEV will remain 0
```

## Design Principles

1. **SSOT (Single Source of Truth)**: All environment variable defaults in one place
2. **Fallback-Safe**: Uses `${VAR:-default}` pattern to allow overrides
3. **Mode-Based**: Pre-configured modes for common test scenarios
4. **Validation**: Built-in validation helpers to catch configuration issues
5. **Backward Compatible**: Existing scripts work without changes

## Related Files

- `tools/smokes/v2/lib/env.sh`: SSOT environment configuration
- `tools/smokes/v2/lib/test_runner.sh`: Core test runner (sources env.sh)
- `tools/smokes/v2/lib/llvm_exe_runner.sh`: LLVM EXE helpers (sources env.sh)
- `tools/build_llvm.sh`: LLVM build script (respects TARGET_TMPDIR from env.sh)

## Phase 131 Task 5 Implementation

This SSOT system was implemented as part of Phase 131 Task 5 to:
- Centralize environment variable configuration
- Prevent "sparrow bugs" (duplicate scattered settings)
- Enable easy modification of defaults across all tests
- Provide clear documentation of available settings

### Implementation Summary

- **New file**: `tools/smokes/v2/lib/env.sh` (180 lines)
- **Modified files**:
  - `tools/smokes/v2/lib/llvm_exe_runner.sh`: Source env.sh, update require_joinir_dev
  - `tools/smokes/v2/lib/test_runner.sh`: Source env.sh, update helper functions
  - `tools/build_llvm.sh`: Support TARGET_TMPDIR from env.sh

### Testing

All existing smoke tests pass with the new SSOT system:
- ✅ Phase 131 VM smoke test
- ✅ Phase 131 LLVM EXE smoke test
- ✅ Environment variable validation
- ✅ Mode-specific configurations
