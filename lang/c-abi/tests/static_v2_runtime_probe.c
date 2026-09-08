/* Observe real kernel storage and inject only the returned status for trap tests. */
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
extern int64_t ny_main(void);
extern uint32_t real_store(int64_t, int64_t, uint32_t, uint64_t)
    __asm__("__real_nyash.map.literal_store_v1");
extern int64_t string_handle(const unsigned char*, uint64_t)
    __asm__("nyash.box.from_i8_string_const_len_v1");
extern int64_t map_get(int64_t, int64_t) __asm__("nyash.map.get_hh");
extern int64_t map_size(int64_t) __asm__("nyash.map.size_h");
extern int64_t float_bits(int64_t) __asm__("nyash.float.get_bits_h");
extern int64_t is_type(int64_t, const char*) __asm__("nyash.any.is_type_h");
extern int64_t string_eq(int64_t, int64_t) __asm__("nyash.string.eq_hh");
extern int64_t type_id(int64_t) __asm__("nyash.object.type_id_h");
extern int64_t typed_birth(int64_t, int64_t) __asm__("nyash.object.new_typed_hi");
extern int64_t direct_array_birth(void) __asm__("nyash.array.direct_i64.birth_h");
extern int64_t map_birth(void) __asm__("nyash.map.birth_h");
extern int64_t real_string(const unsigned char*, uint64_t)
    __asm__("__real_nyash.box.from_i8_string_const_len_v1");
int64_t checked_string(const unsigned char*, uint64_t)
    __asm__("__wrap_nyash.box.from_i8_string_const_len_v1");
int64_t checked_string(const unsigned char* bytes, uint64_t len) {
  return getenv("FORCE_STRING_ZERO") ? 0 : real_string(bytes, len);
}
static int64_t observed_map;
static unsigned writes;
uint32_t observed_store(int64_t, int64_t, uint32_t, uint64_t)
    __asm__("__wrap_nyash.map.literal_store_v1");
uint32_t observed_store(int64_t map, int64_t key, uint32_t kind, uint64_t payload) {
  assert(kind == strtoul(getenv("EXPECT_KIND"), NULL, 0));
  if (kind == 5 && getenv("EXPECT_TYPE_ID")) assert(type_id((int64_t)payload) == strtoll(getenv("EXPECT_TYPE_ID"), NULL, 0));
  else if (kind == 5) assert(is_type((int64_t)payload, getenv("EXPECT_HANDLE_TYPE")));
  else assert(payload == strtoull(getenv("EXPECT_BITS"), NULL, 0));
  assert(!getenv("FORCE_STRING_ZERO"));
  observed_map = map;
  writes++;
  const char* forced = getenv("FORCE_STATUS");
  return forced ? (uint32_t)strtoul(forced, NULL, 0) : real_store(map, key, kind, payload);
}
int main(void) {
  if (getenv("CHECK_NON_HOST_CONTRACT")) {
    int64_t map = map_birth(), key = string_handle((const unsigned char*)"k", 1);
    int64_t typed = typed_birth(11, 0), direct = direct_array_birth();
    assert(type_id(typed) == 11 && direct != 0);
    assert(real_store(map, key, 5, (uint64_t)typed) == 2);
    assert(real_store(map, key, 5, (uint64_t)direct) == 2);
    assert(map_size(map) == 0);
    assert(real_store(map, key, 5, (uint64_t)key) == 0);
    puts("non-host-contract-ok");
    return 0;
  }
  assert(ny_main() == 30);
  fprintf(stderr, "after-entry\n");
  assert(writes == strtoul(getenv("EXPECT_WRITES"), NULL, 0));
  assert(map_size(observed_map) == 1);
  unsigned char key[64];
  const char* hex = getenv("EXPECT_KEY_HEX");
  size_t length = strlen(hex) / 2;
  assert(length <= sizeof(key));
  for (size_t i = 0; i < length; i++) {
    char byte[3] = {hex[i * 2], hex[i * 2 + 1], 0};
    key[i] = (unsigned char)strtoul(byte, NULL, 16);
  }
  int64_t result = map_get(observed_map, string_handle(key, length));
  uint64_t expected = strtoull(getenv("EXPECT_BITS"), NULL, 0);
  if (strtoul(getenv("EXPECT_KIND"), NULL, 0) == 3)
    assert((uint64_t)float_bits(result) == expected);
  else if (strtoul(getenv("EXPECT_KIND"), NULL, 0) == 4) assert(is_type(result, "VoidBox"));
  else if (strtoul(getenv("EXPECT_KIND"), NULL, 0) == 5) {
    const char* type = getenv("EXPECT_HANDLE_TYPE");
    if (getenv("EXPECT_TYPE_ID")) assert(type_id(result) == strtoll(getenv("EXPECT_TYPE_ID"), NULL, 0));
    else assert(is_type(result, type));
    if (type && !strcmp(type, "StringBox")) {
      const char* text = getenv("EXPECT_TEXT");
      assert(string_eq(result, string_handle((const unsigned char*)text, strlen(text))));
    } else if (type && !strcmp(type, "MapBox")) assert(map_size(result) == 0);
  } else assert((uint64_t)result == expected);
  puts("kernel-readback-ok");
  return 0;
}
