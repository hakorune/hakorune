# Environment Variable SSOT - Quick Start

## TL;DR

All smoke test environment variables are now managed in `tools/smokes/v2/lib/env.sh`.

## For Script Authors

### Existing Scripts
✅ **No changes required!** Scripts that source `test_runner.sh` or `llvm_exe_runner.sh` automatically get env.sh.

### New Scripts

```bash
#!/bin/bash
source "$(dirname "$0")/../lib/test_runner.sh"  # Auto-sources env.sh
require_env || exit 2

# Your test code here
test_pass "my_test: passed"
```

### Phase 131+ Scripts (Require JoinIR Dev)

```bash
#!/bin/bash
source "$(dirname "$0")/../lib/test_runner.sh"
source "$(dirname "$0")/../lib/llvm_exe_runner.sh"
require_env || exit 2

# Enable JoinIR dev mode (required for Phase 131+)
require_joinir_dev

# Your test code here
```

## Common Variables (Auto-Set by env.sh)

```bash
NYASH_JOINIR_DEV=1          # JoinIR dev features
HAKO_JOINIR_STRICT=1        # Strict validation
NYASH_LLVM_USE_HARNESS=1    # Python llvmlite backend
NYASH_FEATURES=stage3       # Stage 3 parser
NYASH_ENABLE_USING=1        # Using system enabled
```

## Override Variables

To override defaults, set BEFORE sourcing:

```bash
export NYASH_CLI_VERBOSE=1      # Enable verbose mode
export NYASH_DEBUG_FUEL=unlimited  # Unlimited debug fuel
source "$(dirname "$0")/../lib/test_runner.sh"
```

## Mode Presets

```bash
source "$(dirname "$0")/../lib/env.sh"

setup_smoke_env dev         # Verbose, unlimited fuel
setup_smoke_env integration # Moderate settings
setup_smoke_env quick       # Fast, minimal logging
```

## Debugging

```bash
# Show current configuration
source "$(dirname "$0")/../lib/env.sh"
show_smoke_env

# Validate configuration
source "$(dirname "$0")/../lib/env.sh"
validate_env_setup
```

## Full Documentation

See `tools/smokes/v2/lib/ENV_README.md` for complete reference.
