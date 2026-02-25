#include <stdio.h>
#include <string.h>

// Mirrors benchmarks/bench_method_call_only_small.hako
// Loop 5e3 times: take length of preallocated string and accumulate.
int main(void) {
    const int N = 5000; // 5e3 iterations
    const char *s = "nyash";
    long long total = 0;
    for (int i = 0; i < N; i++) {
        total += (long long)strlen(s);
    }
    // Print total to avoid dead‑code elimination; expected 25000
    printf("%lld\n", total);
    return 0;
}

