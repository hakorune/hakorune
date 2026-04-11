#include <stdint.h>
#include <stdio.h>

typedef struct {
    int64_t value;
} Counter;

static inline int64_t counter_step(const Counter* counter) {
    return counter->value + 2;
}

static inline int64_t counter_step_chain(const Counter* counter) {
    return counter_step(counter);
}

int main(void) {
    const int64_t ops = 2000000;
    Counter counter = {.value = 41};
    int64_t acc = 0;

    for (int64_t i = 0; i < ops; ++i) {
        acc += counter_step_chain(&counter);
    }

    printf("%lld\n", (long long)(acc + counter_step_chain(&counter)));
    return 0;
}
