#include <stdint.h>

typedef struct {
  int64_t x;
  int64_t y;
} Point;

int main(void) {
  const int64_t ops = 2000000;
  Point p = {1, 2};
  volatile int64_t acc = 0;

  for (int64_t i = 0; i < ops; i++) {
    const int64_t sum = p.x + p.y;
    p.x += 1;
    p.y += 2;
    acc += sum;
  }

  return (int)((acc + p.x + p.y) & 0xFF);
}
