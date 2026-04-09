#include <stdint.h>

typedef struct {
  int64_t enabled;
} Flag;

int main(void) {
  const int64_t ops = 2000000;
  const int64_t flip_at = 1000000;
  Flag flag = {1};
  volatile int64_t acc = 0;

  for (int64_t i = 0; i < ops; i++) {
    acc += (flag.enabled != 0);
    flag.enabled = (i < flip_at) ? 1 : 0;
  }

  return (int)((acc + (flag.enabled != 0)) & 0xFF);
}
