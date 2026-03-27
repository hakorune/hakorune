#include <stdint.h>
#include <string.h>

int main(void) {
  const int rows = 64;
  const int64_t ops = 600000;
  const char *line_seed = "line-seed";
  const char *none_seed = "none-seed";
  const char *lines[64];

  for (int i = 0; i < rows; i++) {
    lines[i] = (i % 2 == 0) ? line_seed : none_seed;
  }

  volatile int64_t acc = 0;
  for (int64_t i = 0; i < ops; i++) {
    int row = (int)(i % rows);
    acc += (int64_t)strlen(lines[row]);
  }

  return (int)(acc & 0xFF);
}
