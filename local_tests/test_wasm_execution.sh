#!/bin/bash
# WASM execution test script

echo "🎯 Testing WASM compilation and execution with host functions"

# First compile to WAT
echo "📝 Compiling test_mir_simple.hako to WASM..."
../target/release/nyash --compile-wasm test_mir_simple.hako

# Check if WAT was generated
if [ -f "test_mir_simple.wat" ]; then
    echo "✅ WAT file generated successfully"
    echo "📄 WAT content preview:"
    head -20 test_mir_simple.wat
    echo "..."
    
    # Now we need a custom WASM runner that provides host functions
    echo ""
    echo "🚀 To execute WASM with host functions, we need to build a custom runner"
    echo "   that provides the required imports (env::print, etc.)"
else
    echo "❌ WAT file generation failed"
    exit 1
fi