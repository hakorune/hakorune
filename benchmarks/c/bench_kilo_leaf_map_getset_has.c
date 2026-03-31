#include <stdint.h>

int main(void) {
  const int64_t ops = 2000000;
  volatile int64_t key = -1;
  int64_t value = 0;
  int64_t sum = 0;

  value = 1;
  for (int64_t i = 0; i < ops; i++) {
    if (key != 0) {
      sum += value;
    }
  }

  return (int)((sum + value) & 0xFF);
}
