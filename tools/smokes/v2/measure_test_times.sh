#!/bin/bash
# measure_test_times.sh - Measure execution time for each smoke test

set -e

PROFILE="${1:-quick}"
OUTPUT="${2:-/tmp/smoke_test_times_${PROFILE}.txt}"

echo "Measuring test times for profile: $PROFILE"
echo "Output: $OUTPUT"

# Get list of test files
cd "$(dirname "$0")"
PROFILE_DIR="profiles/$PROFILE"

if [ ! -d "$PROFILE_DIR" ]; then
    echo "Error: Profile directory not found: $PROFILE_DIR"
    exit 1
fi

# Check for bc command
if ! command -v bc &> /dev/null; then
    echo "Warning: bc command not found. Time calculations may be less precise."
    echo "Install with: sudo apt-get install bc"
    USE_BC=0
else
    USE_BC=1
fi

# Find all .sh test files
TEST_FILES=$(find "$PROFILE_DIR" -name "*.sh" -type f | sort)

# Count total tests
TOTAL=$(echo "$TEST_FILES" | wc -l)
echo "Found $TOTAL test files"

# Initialize output file
> "$OUTPUT"

# Measure each test
COUNT=0
for TEST_FILE in $TEST_FILES; do
    COUNT=$((COUNT + 1))
    TEST_NAME=$(basename "$TEST_FILE" .sh)

    echo -n "[$COUNT/$TOTAL] Measuring $TEST_NAME... "

    # Run test and measure time (suppress output)
    START=$(date +%s.%N)
    if bash "$TEST_FILE" > /dev/null 2>&1; then
        STATUS="PASS"
    else
        STATUS="FAIL"
    fi
    END=$(date +%s.%N)

    # Calculate duration
    if [ "$USE_BC" -eq 1 ]; then
        DURATION=$(echo "$END - $START" | bc)
    else
        # Fallback: use awk for calculation
        DURATION=$(awk "BEGIN {print $END - $START}")
    fi

    # Write to output
    echo "$DURATION $TEST_NAME $STATUS" >> "$OUTPUT"

    echo "${DURATION}s ($STATUS)"
done

# Sort by time (descending)
SORTED_OUTPUT="${OUTPUT}.sorted"
sort -rn "$OUTPUT" > "$SORTED_OUTPUT"

echo ""
echo "Results written to: $SORTED_OUTPUT"
echo ""
echo "Top 20 slowest tests:"
head -20 "$SORTED_OUTPUT"

echo ""
echo "Total time:"
awk '{sum+=$1} END {print sum " seconds"}' "$OUTPUT"

echo ""
echo "Statistics:"
echo "  Total tests: $TOTAL"
echo "  PASS: $(grep PASS "$OUTPUT" | wc -l)"
echo "  FAIL: $(grep FAIL "$OUTPUT" | wc -l)"

# Category analysis
echo ""
echo "Category breakdown:"
echo "  LLVM-related (*_llvm.sh):"
grep "_llvm " "$OUTPUT" | wc -l
echo "  Selfhost-related (selfhost_*):"
grep "^[0-9.]* selfhost_" "$OUTPUT" | wc -l
echo "  S3 backend (s3_backend_*):"
grep "^[0-9.]* s3_backend_" "$OUTPUT" | wc -l

# Find tests over 1 second
echo ""
echo "Tests over 1 second:"
awk '$1 > 1 {print}' "$SORTED_OUTPUT" | wc -l
echo ""
echo "Top tests over 1 second:"
awk '$1 > 1 {print}' "$SORTED_OUTPUT" | head -50
