#include <stdint.h>
#include <string.h>

int main(void) {
  const int rows = 64;
  const int64_t ops = 400000;
  const char *line_seed = "line-seed";
  const char *none_seed = "none-seed";
  const char *lines[64];

  for (int i = 0; i < rows; i++) {
    lines[i] = (i % 2 == 0) ? line_seed : none_seed;
  }

  volatile int64_t hits = 0;
  for (int64_t i = 0; i < ops; i++) {
    int row = (int)(i % rows);
    const char *cur = lines[row];
    int found = (strstr(cur, "line") != NULL);
    if (found) {
      hits += 1;
    }
    if ((i % 16) == 0) {
      lines[row] = found ? none_seed : line_seed;
    }
  }

  return (int)(hits & 0xFF);
}
