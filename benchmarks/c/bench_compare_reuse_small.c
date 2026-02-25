#include <stdio.h>

// Mirrors benchmarks/bench_compare_reuse_small.hako
int main(void) {
    const int N = 120000;
    long long total = 0;
    for (int i = 0; i < N; i++) {
        int m = i % 31;
        if (m < 20) {
            total += 1;
        }
        if (m < 20) {
            total += 2;
        }
    }
    printf("%lld\n", total);
    return 0;
}
