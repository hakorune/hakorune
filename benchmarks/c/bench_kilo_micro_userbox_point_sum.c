#include <stdint.h>
#include <stdio.h>

typedef struct {
    int64_t x;
    int64_t y;
} Point;

static inline int64_t point_sum(const Point* point) {
    return point->x + point->y;
}

int main(void) {
    const int64_t ops = 2000000;
    Point point = {.x = 1, .y = 2};
    int64_t acc = 0;

    for (int64_t i = 0; i < ops; ++i) {
        acc += point_sum(&point);
    }

    printf("%lld\n", (long long)(acc + point_sum(&point)));
    return 0;
}
