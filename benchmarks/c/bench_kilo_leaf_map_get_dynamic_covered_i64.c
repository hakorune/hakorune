#include <stdint.h>

#if defined(__GNUC__) || defined(__clang__)
#define HAKO_NOINLINE __attribute__((noinline))
#else
#define HAKO_NOINLINE
#endif

enum { I64_MAP_CAP = 8 };

typedef struct {
  uint8_t used[I64_MAP_CAP];
  int64_t keys[I64_MAP_CAP];
  int64_t values[I64_MAP_CAP];
} I64Map;

static uint64_t hash_i64(int64_t key) {
  uint64_t x = (uint64_t)key;
  x ^= x >> 33;
  x *= UINT64_C(0xff51afd7ed558ccd);
  x ^= x >> 33;
  return x;
}

static HAKO_NOINLINE void map_set_i64(I64Map* map, int64_t key, int64_t value) {
  uint64_t hash = hash_i64(key);
  for (int64_t probe = 0; probe < I64_MAP_CAP; ++probe) {
    int64_t idx = (int64_t)((hash + (uint64_t)probe) & (I64_MAP_CAP - 1));
    if (!map->used[idx] || map->keys[idx] == key) {
      map->used[idx] = 1;
      map->keys[idx] = key;
      map->values[idx] = value;
      return;
    }
  }
}

static HAKO_NOINLINE int64_t map_get_i64(const I64Map* map, int64_t key) {
  uint64_t hash = hash_i64(key);
  for (int64_t probe = 0; probe < I64_MAP_CAP; ++probe) {
    int64_t idx = (int64_t)((hash + (uint64_t)probe) & (I64_MAP_CAP - 1));
    if (!map->used[idx]) {
      return 0;
    }
    if (map->keys[idx] == key) {
      return map->values[idx];
    }
  }
  return 0;
}

int main(void) {
  const int64_t ops = 2000000;
  I64Map map = {0};
  int64_t sum = 0;

  map_set_i64(&map, 0, 1);
  map_set_i64(&map, 1, 2);
  map_set_i64(&map, 2, 3);

  for (int64_t i = 0; i < ops; ++i) {
    int64_t key = i % 3;
    int64_t value = map_get_i64(&map, key);
    sum += value;
  }

  return (int)((sum + map_get_i64(&map, 1)) & 0xFF);
}
