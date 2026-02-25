// APP-PERF-01: CHIP-8 kernel benchmark (small) - C reference
// Deterministic emulator-style workload for cross-language comparison
// 400,000 iterations with branch-heavy logic

#include <stdio.h>
#include <stdint.h>

int main(void) {
    volatile int32_t n = 400000;  // Prevent over-optimization
    int32_t i = 0;
    int32_t pc = 0;
    int32_t op = 7;
    int32_t acc = 1;
    int32_t sum = 0;
    const int32_t MOD = 1000000007;
    int32_t branch = 0;
    int64_t result = 0;

    int32_t n_val = n;  // Read from volatile

    while (i < n_val) {
        op = (op * 73 + 19) % 65536;
        branch = op % 16;

        if (branch < 4) {
            acc = (acc + (op % 251) + pc) % MOD;
        } else if (branch < 8) {
            acc = (acc + (op % 127) + 3) % MOD;
        } else if (branch < 12) {
            acc = (acc + (branch * branch) + (pc % 17)) % MOD;
        } else {
            acc = (acc + (op % 97) + (pc % 13) + 11) % MOD;
        }

        pc = (pc + 2 + (op % 3)) % 4096;
        sum = (sum + acc + pc + branch) % MOD;
        i = i + 1;
    }

    result = (sum + acc + pc) % MOD;
    return (int)(result & 0xFF);
}
