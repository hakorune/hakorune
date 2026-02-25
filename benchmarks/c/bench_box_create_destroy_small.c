#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Mirrors benchmarks/bench_box_create_destroy_small.hako
// Loop 1e4 times: create a tiny string, take length, accumulate, destroy.
int main(void) {
    const int N = 10000; // 1e4 iterations
    long long total = 0;
    for (int i = 0; i < N; i++) {
        char *tmp = (char*)malloc(2);
        if (!tmp) return 2;
        tmp[0] = 'x';
        tmp[1] = '\0';
        total += (long long)strlen(tmp);
        free(tmp);
    }
    // Print total to avoid dead‑code elimination; expected 10000
    printf("%lld\n", total);
    return 0;
}

