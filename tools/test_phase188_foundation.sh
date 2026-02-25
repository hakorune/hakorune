#!/bin/bash
# Phase 188 Foundation Verification Script
#
# This script verifies that the Phase 188 implementation foundation is ready.
# It checks:
# 1. Build succeeds
# 2. Pattern detection module exists
# 3. Lowering functions module exists
# 4. Router integration is in place

set -e

echo "=== Phase 188 Foundation Tests ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track overall status
ALL_PASS=true

# Test 1: Build verification
echo "1. Build verification..."
if cargo build --release 2>&1 | tail -5 | grep -q "Finished"; then
  echo -e "${GREEN}✅ Build succeeded${NC}"
else
  echo -e "${RED}❌ Build failed${NC}"
  ALL_PASS=false
fi

echo ""

# Test 2: Pattern detection module check
echo "2. Pattern detection module check..."
if rg "is_simple_while_pattern" src/mir/loop_pattern_detection.rs > /dev/null 2>&1; then
  echo -e "${GREEN}✅ Pattern detection module exists${NC}"

  # Count functions
  PATTERN_FUNCS=$(rg "^pub fn is_.*_pattern\(" src/mir/loop_pattern_detection.rs | wc -l)
  echo "   Found $PATTERN_FUNCS pattern detection functions"

  if [ "$PATTERN_FUNCS" -ge 3 ]; then
    echo -e "${GREEN}   ✅ All 3 pattern detectors present${NC}"
  else
    echo -e "${YELLOW}   ⚠️ Expected 3 pattern detectors, found $PATTERN_FUNCS${NC}"
  fi
else
  echo -e "${RED}❌ Pattern detection module not found${NC}"
  ALL_PASS=false
fi

echo ""

# Test 3: Lowering functions check
echo "3. Lowering functions check..."
if rg "lower_simple_while_to_joinir" src/mir/join_ir/lowering/loop_patterns.rs > /dev/null 2>&1; then
  echo -e "${GREEN}✅ Lowering functions scaffolding exists${NC}"

  # Count lowering functions
  LOWERING_FUNCS=$(rg "^pub fn lower_.*_to_joinir\(" src/mir/join_ir/lowering/loop_patterns.rs | wc -l)
  echo "   Found $LOWERING_FUNCS lowering functions"

  if [ "$LOWERING_FUNCS" -ge 3 ]; then
    echo -e "${GREEN}   ✅ All 3 lowering functions present${NC}"
  else
    echo -e "${YELLOW}   ⚠️ Expected 3 lowering functions, found $LOWERING_FUNCS${NC}"
  fi
else
  echo -e "${RED}❌ Lowering functions not found${NC}"
  ALL_PASS=false
fi

echo ""

# Test 4: Router integration check
echo "4. Router integration check..."
if rg "try_lower_loop_pattern_to_joinir" src/mir/join_ir/lowering/mod.rs > /dev/null 2>&1; then
  echo -e "${GREEN}✅ Router integration point exists${NC}"

  # Check if function is public
  if rg "^pub fn try_lower_loop_pattern_to_joinir" src/mir/join_ir/lowering/mod.rs > /dev/null 2>&1; then
    echo -e "${GREEN}   ✅ Router function is public${NC}"
  else
    echo -e "${YELLOW}   ⚠️ Router function may not be public${NC}"
  fi
else
  echo -e "${RED}❌ Router integration not found${NC}"
  ALL_PASS=false
fi

echo ""

# Test 5: Module imports check
echo "5. Module imports check..."
MODULE_IMPORTS_OK=true

if rg "pub mod loop_pattern_detection" src/mir/mod.rs > /dev/null 2>&1; then
  echo -e "${GREEN}   ✅ loop_pattern_detection imported in src/mir/mod.rs${NC}"
else
  echo -e "${RED}   ❌ loop_pattern_detection NOT imported in src/mir/mod.rs${NC}"
  MODULE_IMPORTS_OK=false
  ALL_PASS=false
fi

if rg "pub mod loop_patterns" src/mir/join_ir/lowering/mod.rs > /dev/null 2>&1; then
  echo -e "${GREEN}   ✅ loop_patterns imported in src/mir/join_ir/lowering/mod.rs${NC}"
else
  echo -e "${RED}   ❌ loop_patterns NOT imported in src/mir/join_ir/lowering/mod.rs${NC}"
  MODULE_IMPORTS_OK=false
  ALL_PASS=false
fi

if [ "$MODULE_IMPORTS_OK" = true ]; then
  echo -e "${GREEN}✅ All module imports configured correctly${NC}"
else
  echo -e "${RED}❌ Some module imports missing${NC}"
fi

echo ""

# Test 6: Implementation roadmap check
echo "6. Implementation roadmap check..."
ROADMAP_PATH="docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/IMPLEMENTATION_ROADMAP.md"
if [ -f "$ROADMAP_PATH" ]; then
  echo -e "${GREEN}✅ Implementation roadmap exists${NC}"

  # Check roadmap completeness
  ROADMAP_SECTIONS=$(grep "^###" "$ROADMAP_PATH" | wc -l)
  echo "   Found $ROADMAP_SECTIONS sections in roadmap"

  if [ "$ROADMAP_SECTIONS" -ge 10 ]; then
    echo -e "${GREEN}   ✅ Roadmap is comprehensive${NC}"
  else
    echo -e "${YELLOW}   ⚠️ Roadmap may be incomplete (expected 10+ sections)${NC}"
  fi
else
  echo -e "${RED}❌ Implementation roadmap not found${NC}"
  ALL_PASS=false
fi

echo ""

# Test 7: Design document check
echo "7. Design document check..."
DESIGN_PATH="docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/design.md"
if [ -f "$DESIGN_PATH" ]; then
  echo -e "${GREEN}✅ Design document exists${NC}"

  # Check design document size
  DESIGN_LINES=$(wc -l < "$DESIGN_PATH")
  echo "   Design document: $DESIGN_LINES lines"

  if [ "$DESIGN_LINES" -ge 2000 ]; then
    echo -e "${GREEN}   ✅ Design document is comprehensive${NC}"
  else
    echo -e "${YELLOW}   ⚠️ Design document may be incomplete (expected 2000+ lines)${NC}"
  fi
else
  echo -e "${RED}❌ Design document not found${NC}"
  ALL_PASS=false
fi

echo ""

# Test 8: TODO markers check
echo "8. TODO markers check..."
TODO_COUNT=$(rg "TODO:" src/mir/loop_pattern_detection.rs src/mir/join_ir/lowering/loop_patterns.rs | wc -l)
echo "   Found $TODO_COUNT TODO markers in implementation files"

if [ "$TODO_COUNT" -ge 10 ]; then
  echo -e "${GREEN}✅ Clear TODO markers present for implementation${NC}"
else
  echo -e "${YELLOW}⚠️ May need more TODO markers (expected 10+)${NC}"
fi

echo ""
echo "=========================================="
echo ""

# Final summary
if [ "$ALL_PASS" = true ]; then
  echo -e "${GREEN}=== ✅ Foundation Ready for Implementation ===${NC}"
  echo ""
  echo "Next steps:"
  echo "1. Open: src/mir/loop_pattern_detection.rs"
  echo "2. Start: is_simple_while_pattern() implementation"
  echo "3. Reference: docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/design.md"
  echo "4. Timeline: 6-8h for Pattern 1, 18-28h total"
  echo ""
  exit 0
else
  echo -e "${RED}=== ❌ Foundation Has Issues ===${NC}"
  echo ""
  echo "Please fix the issues above before proceeding."
  echo ""
  exit 1
fi
