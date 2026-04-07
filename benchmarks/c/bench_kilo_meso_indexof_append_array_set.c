#include <stdint.h>
#include <string.h>

int main(void) {
  const int rows = 128;
  const int64_t ops = 320000;
  const char seed[] = "line-seed-abcdef";
  const int len = (int)(sizeof(seed) - 1);
  char lines[128][96];
  int lens[128];
  volatile int64_t total = 0;

  for (int i = 0; i < rows; i++) {
    memcpy(lines[i], seed, (size_t)len + 1);
    lens[i] = len;
  }

  for (int64_t i = 0; i < ops; i++) {
    const int row = (int)(i % rows);
    if (strstr(lines[row], "line") != NULL) {
      lines[row][lens[row]] = 'l';
      lines[row][lens[row] + 1] = 'n';
      lens[row] += 2;
      lines[row][lens[row]] = '\0';
      total += (int64_t)lens[row];
    }
  }

  return (int)((total + rows) & 0xFF);
}
