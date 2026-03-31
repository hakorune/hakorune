#include <stdint.h>

int main(void) {
  const int64_t ops = 2000000;
  volatile int64_t key = 0;
  int64_t sum = 0;

  for (int64_t i = 0; i < ops; i++) {
    if (key == 0) {
      sum += 1;
    }
  }

  return (int)(sum & 0xFF);
}
